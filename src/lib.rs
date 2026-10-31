use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::PyDict;
use quick_xml::events::{Event, BytesStart, BytesEnd, BytesText};
use quick_xml::Writer;
use regex::Regex;
use std::collections::HashMap;
use std::io::Write;

/// A fully normalized cross-format Translation Unit.
#[pyclass]
#[derive(Clone, Debug)]
pub struct ExtendedTranslationUnit {
    #[pyo3(get, set)]
    pub id: String,
    #[pyo3(get, set)]
    pub source_text: String,
    #[pyo3(get, set)]
    pub target_text: String,
    #[pyo3(get, set)]
    pub source_lang: String,
    #[pyo3(get, set)]
    pub target_lang: String,
    #[pyo3(get, set)]
    pub metadata: HashMap<String, String>,
}

#[pymethods]
impl ExtendedTranslationUnit {
    #[new]
    fn new(id: String, source_text: String, target_text: String, source_lang: String, target_lang: String) -> Self {
        ExtendedTranslationUnit {
            id,
            source_text,
            target_text,
            source_lang,
            target_lang,
            metadata: HashMap::new(),
        }
    }

    fn add_meta(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}

/// Helper function that scans a string block for Latin Binomial configurations ($Genus $species)
fn detect_scientific_name(text: &str, regex: &Regex) -> Option<String> {
    if let Some(mat) = regex.find(text) {
        Some(mat.as_str().to_string())
    } else {
        None
    }
}

/// Dynamic XML Streaming Pull-Parser with automated Taxonomy & Property Mapping
#[pyfunction]
fn parse_and_enrich_stream(py: Python, file_path: String, url_prefix: String, audio_prefix: String) -> PyResult<Vec<PyObject>> {
    py.allow_threads(|| {
        let mut reader = quick_xml::reader::Reader::from_file(&file_path)
            .map_err(|e| PyValueError::new_err(format!("XML file open error: {}", e)))?;
        reader.config_mut().trim_text(true);

        // Pre-compile the high-performance regex filters
        let binomial_rx = Regex::new(r"\b[A-Z][a-z]+(?:\s+[a-z]+){1,2}\b").unwrap();
        let ipa_rx = Regex::new(r"^x-ipa-(?P<lang>[a-zA-Z\-]+)$").unwrap();

        let mut buf = Vec::new();
        let mut extracted_units = Vec::new();

        // SAX Parser tracking frames
        let mut current_id = String::new();
        let mut current_props: HashMap<String, String> = HashMap::new();
        let mut current_segments: Vec<(String, String)> = Vec::new(); // (Lang, Content)

        let mut inside_prop = false;
        let mut inside_seg = false;
        let mut inside_note = false;
        let mut current_prop_type = String::new();
        let mut current_lang_context = String::new();
        let mut notes_collection = Vec::new();

        while let Ok(event) = reader.read_event_into(&mut buf) {
            match event {
                // Hook top-level unit entity headers across TMX / TBX / XLIFF variants
                Event::Start(ref e) if e.name().as_ref() == b"tu" || e.name().as_ref() == b"termEntry" || e.name().as_ref() == b"trans-unit" || e.name().as_ref() == b"unit" => {
                    current_id = e.attributes().flatten()
                        .find(|a| a.key.as_ref() == b"id" || a.key.as_ref() == b"tuid")
                        .map(|a| String::from_utf8_lossy(&a.value).into_owned())
                        .unwrap_or_else(|| "unit-gen".to_string());
                    
                    current_props.clear();
                    current_segments.clear();
                    notes_collection.clear();

                    // Instantly capture inline custom attributes (e.g., <prop x-scientific_name="Panthera leo">)
                    if let Some(Ok(sci_attr)) = e.attributes().find(|a| a.as_ref().unwrap().key.as_ref() == b"x-scientific_name") {
                        let val = String::from_utf8_lossy(&sci_attr.value).into_owned();
                        if !val.is_empty() {
                            current_props.insert("scientific_name".to_string(), val);
                        }
                    }
                }

                // Ingest traditional metadata property strings
                Event::Start(ref e) if e.name().as_ref() == b"prop" || e.name().as_ref() == b"descrip" || e.name().as_ref() == b"termNote" => {
                    if let Some(Ok(attr)) = e.attributes().find(|a| a.as_ref().unwrap().key.as_ref() == b"type") {
                        current_prop_type = String::from_utf8_lossy(&attr.value).into_owned();
                        inside_prop = true;
                    }
                }

                // Handle localization note wrappers (<note> or <notes>)
                Event::Start(ref e) if e.name().as_ref() == b"note" || e.name().as_ref() == b"notes" => {
                    inside_note = true;
                }

                // Catch language variation context parameters
                Event::Start(ref e) if e.name().as_ref() == b"tuv" || e.name().as_ref() == b"langSec" || e.name().as_ref() == b"langSet" => {
                    current_lang_context = e.attributes().flatten()
                        .find(|a| a.key.as_ref() == b"xml:lang" || a.key.as_ref() == b"lang")
                        .map(|a| String::from_utf8_lossy(&a.value).into_owned())
                        .unwrap_or_default();
                }

                Event::Start(ref e) if e.name().as_ref() == b"seg" || e.name().as_ref() == b"term" || e.name().as_ref() == b"source" || e.name().as_ref() == b"target" => {
                    inside_seg = true;
                }

                // Capture underlying character data fragments securely
                Event::Text(ref e) => {
                    let clean_text = e.unescape().unwrap_or_default().into_owned();
                    
                    if inside_prop && !current_prop_type.is_empty() {
                        current_props.insert(current_prop_type.clone(), clean_text);
                    } else if inside_note {
                        notes_collection.push(clean_text);
                    } else if inside_seg {
                        // Extract language definition if missing (fallback schema layers)
                        let resolved_lang = if current_lang_context.is_empty() { "und".to_string() } else { current_lang_context.clone() };
                        current_segments.push((resolved_lang, clean_text));
                    }
                }

                Event::End(ref e) if e.name().as_ref() == b"prop" || e.name().as_ref() == b"descrip" || e.name().as_ref() == b"termNote" => { inside_prop = false; current_prop_type.clear(); }
                Event::End(ref e) if e.name().as_ref() == b"note" || e.name().as_ref() == b"notes" => { inside_note = false; }
                Event::End(ref e) if e.name().as_ref() == b"seg" || e.name().as_ref() == b"term" || e.name().as_ref() == b"source" || e.name().as_ref() == b"target" => { inside_seg = false; }

                // Process the entity on element closure closures
                Event::End(ref e) if e.name().as_ref() == b"tu" || e.name().as_ref() == b"termEntry" || e.name().as_ref() == b"trans-unit" || e.name().as_ref() == b"unit" => {
                    Python::with_gil(|py| {
                        let py_dict = PyDict::new(py);
                        py_dict.set_item("entry_id", &current_id).unwrap();

                        let py_meta = PyDict::new(py);
                        let mut resolved_scientific_name = current_props.get("scientific_name").cloned().unwrap_or_default();

                        // Loop text variants to scan for hidden binomial patterns
                        for (lang, text) in &current_segments {
                            if resolved_scientific_name.is_empty() {
                                if let Some(found_sci) = detect_scientific_name(text, &binomial_rx) {
                                    resolved_scientific_name = found_sci;
                                }
                            }
                            // Strip HTML formatting markers if processing plain text segments downstream
                            let flat_text = text.replace("<i>", "").replace("</i>", "");
                            py_meta.set_item(format!("text_{}", lang), flat_text).unwrap();
                        }

                        // Map standard attributes and execute rule conversions
                        for (key, val) in &current_props {
                            if let Some(caps) = ipa_rx.captures(key) {
                                py_meta.set_item(format!("ipa_{}", &caps["lang"]), val).unwrap();
                            } else if key.starts_with("x-mp3-") || key == "mp3_url" {
                                py_meta.set_item("resolved_audio_url", format!("{}/{}", audio_prefix, val)).unwrap();
                            } else if key.contains("url") {
                                py_meta.set_item("resolved_attribution_url", format!("{}/{}", url_prefix, val)).unwrap();
                            } else {
                                py_meta.set_item(key, val).unwrap();
                            }
                        }

                        py_meta.set_item("scientific_name", resolved_scientific_name).unwrap();
                        if !notes_collection.is_empty() {
                            py_meta.set_item("notes", notes_collection.clone()).unwrap();
                        }

                        py_dict.set_item("processed_metadata", py_meta).unwrap();
                        
                        let py_segs = PyDict::new(py);
                        for (l, t) in &current_segments {
                            py_segs.set_item(l, t).unwrap();
                        }
                        py_dict.set_item("bilingual_segments", py_segs).unwrap();

                        extracted_units.push(py_dict.to_object(py));
                    });
