use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::PyDict;
use quick_xml::events::{Event, BytesStart, BytesEnd, BytesText, BytesDecl};
use quick_xml::Writer;
use regex::Regex;
use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::fs::File;

/// A fully normalized, high-fidelity provenance translation data model.
#[pyclass]
#[derive(Clone, Debug)]
pub struct ComprehensiveTranslationUnit {
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
    /// Holds open provenance identifiers (isbn, doi, url, loc, dewey, pdf, x-scientific_name, mp3_url)
    #[pyo3(get, set)]
    pub metadata: HashMap<String, String>,
    #[pyo3(get, set)]
    pub notes: Vec<String>,
}

#[pymethods]
impl ComprehensiveTranslationUnit {
    #[new]
    fn new(id: String, source_text: String, target_text: String, source_lang: String, target_lang: String) -> Self {
        ComprehensiveTranslationUnit {
            id,
            source_text,
            target_text,
            source_lang,
            target_lang,
            metadata: HashMap::new(),
            notes: Vec::new(),
        }
    }

    fn add_meta(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    fn add_note(&mut self, note: String) {
        self.notes.push(note);
    }
}

// ============================================================================
// SYSTEM STREAMING WRITERS (EXPORT PIPELINE GENERATION MATRIX)
// ============================================================================

fn write_meta_props<W: Write>(writer: &mut Writer<W>, metadata: &HashMap<String, String>) -> Result<(), quick_xml::Error> {
    for (key, val) in metadata {
        let mut prop = BytesStart::new("prop");
        prop.push_attribute(("type", key.as_str()));
        writer.write_event(Event::Start(prop))?;
        writer.write_event(Event::Text(BytesText::new(val)))?;
        writer.write_event(Event::End(BytesEnd::new("prop")))?;
    }
    Ok(())
}

fn write_notes<W: Write>(writer: &mut Writer<W>, notes: &[String]) -> Result<(), quick_xml::Error> {
    for note_text in notes {
        writer.write_event(Event::Start(BytesStart::new("note")))?;
        writer.write_event(Event::Text(BytesText::new(note_text)))?;
        writer.write_event(Event::End(BytesEnd::new("note")))?;
    }
    Ok(())
}

/// Generates valid, schema-compliant TMX documents using <prop> extensions.
pub fn stream_to_tmx<W: Write>(writer: &mut Writer<W>, unit: &ComprehensiveTranslationUnit) -> Result<(), quick_xml::Error> {
    let mut tu = BytesStart::new("tu");
    tu.push_attribute(("tuid", unit.id.as_str()));
    if let Some(sci) = unit.metadata.get("x-scientific_name") {
        tu.push_attribute(("x-scientific_name", sci.as_str()));
    }
    writer.write_event(Event::Start(tu))?;

    write_meta_props(writer, &unit.metadata)?;
    write_notes(writer, &unit.notes)?;

    let mut tuv_src = BytesStart::new("tuv");
    tuv_src.push_attribute(("xml:lang", unit.source_lang.as_str()));
    writer.write_event(Event::Start(tuv_src))?;
    writer.write_event(Event::Start(BytesStart::new("seg")))?;
    writer.write_event(Event::Text(BytesText::new(&unit.source_text)))?;
    writer.write_event(Event::End(BytesEnd::new("seg")))?;
    writer.write_event(Event::End(BytesEnd::new("tuv")))?;

    let mut tuv_trg = BytesStart::new("tuv");
    tuv_trg.push_attribute(("xml:lang", unit.target_lang.as_str()));
    writer.write_event(Event::Start(tuv_trg))?;
    writer.write_event(Event::Start(BytesStart::new("seg")))?;
    writer.write_event(Event::Text(BytesText::new(&unit.target_text)))?;
    writer.write_event(Event::End(BytesEnd::new("seg")))?;
    writer.write_event(Event::End(BytesEnd::new("tuv")))?;

    writer.write_event(Event::End(BytesEnd::new("tu")))?;
    Ok(())
}

/// Generates a valid ISO 30042 TBX Terminology Document using strict schema elements.
pub fn stream_to_tbx<W: Write>(writer: &mut Writer<W>, unit: &ComprehensiveTranslationUnit) -> Result<(), quick_xml::Error> {
    let mut entry = BytesStart::new("conceptEntry");
    entry.push_attribute(("id", unit.id.as_str()));
    writer.write_event(Event::Start(entry))?;

    // Global descriptions/provenance metadata at concept level
    for (key, val) in &unit.metadata {
        if key == "provenance" || key == "isbn" || key == "doi" || key == "library_of_congress" || key == "dewey" {
            let mut descrip = BytesStart::new("descrip");
            descrip.push_attribute(("type", key.as_str()));
            writer.write_event(Event::Start(descrip))?;
            writer.write_event(Event::Text(BytesText::new(val)))?;
            writer.write_event(Event::End(BytesEnd::new("descrip")))?;
        }
    }

    // Source Language Section
    let mut lang_src = BytesStart::new("langSec");
    lang_src.push_attribute(("xml:lang", unit.source_lang.as_str()));
    writer.write_event(Event::Start(lang_src))?;
    writer.write_event(Event::Start(BytesStart::new("termSec")))?;
    writer.write_event(Event::Start(BytesStart::new("term")))?;
    writer.write_event(Event::Text(BytesText::new(&unit.source_text)))?;
    writer.write_event(Event::End(BytesEnd::new("term")))?;
    writer.write_event(Event::End(BytesEnd::new("termSec")))?;
    writer.write_event(Event::End(BytesEnd::new("langSec")))?;

    // Target Language Section
    let mut lang_trg = BytesStart::new("langSec");
    lang_trg.push_attribute(("xml:lang", unit.target_lang.as_str()));
    writer.write_event(Event::Start(lang_trg))?;
    writer.write_event(Event::Start(BytesStart::new("termSec")))?;
    writer.write_event(Event::Start(BytesStart::new("term")))?;
    
    // Check for inline formatting requirement
    if let Some(sci_name) = unit.metadata.get("x-scientific_name") {
        if unit.target_lang == "la" || unit.target_text.contains(sci_name) {
            writer.write_event(Event::Text(BytesText::new(&unit.target_text)))?;
        } else {
            writer.write_event(Event::Text(BytesText::new(&unit.target_text)))?;
        }
    } else {
        writer.write_event(Event::Text(BytesText::new(&unit.target_text)))?;
    }
    writer.write_event(Event::End(BytesEnd::new("term")))?;

    // Append localized notes/term-specific descriptors directly to the target term definition boundary
    for (key, val) in &unit.metadata {
        if key == "mp3_url" || key == "pdf_anchor" || key.starts_with("x-") {
            let mut note = BytesStart::new("termNote");
            note.push_attribute(("type", key.as_str()));
            writer.write_event(Event::Start(note))?;
            writer.write_event(Event::Text(BytesText::new(val)))?;
            writer.write_event(Event::End(BytesEnd::new("termNote")))?;
        }
    }
    writer.write_event(Event::End(BytesEnd::new("termSec")))?;
    writer.write_event(Event::End(BytesEnd::new("langSec")))?;

    writer.write_event(Event::End(BytesEnd::new("conceptEntry")))?;
    Ok(())
}

/// Generates XLIFF 1.2 compliant <trans-unit> blocks.
pub fn stream_to_xliff12<W: Write>(writer: &mut Writer<W>, unit: &ComprehensiveTranslationUnit) -> Result<(), quick_xml::Error> {
    let mut tu = BytesStart::new("trans-unit");
    tu.push_attribute(("id", unit.id.as_str()));
    writer.write_event(Event::Start(tu))?;

    writer.write_event(Event::Start(BytesStart::new("source")))?;
    writer.write_event(Event::Text(BytesText::new(&unit.source_text)))?;
    writer.write_event(Event::End(BytesEnd::new("source")))?;

    writer.write_event(Event::Start(BytesStart::new("target")))?;
    writer.write_event(Event::Text(BytesText::new(&unit.target_text)))?;
    writer.write_event(Event::End(BytesEnd::new("target")))?;

    // Write extended properties and historical citations using standard XLIFF 1.2 extensions
    write_meta_props(writer, &unit.metadata)?;
    write_notes(writer, &unit.notes)?;

    writer.write_event(Event::End(BytesEnd::new("trans-unit")))?;
    Ok(())
}

/// Generates XLIFF 2.x compliant <unit> components leveraging formal <metadata> extensions.
pub fn stream_to_xliff20<W: Write>(writer: &mut Writer<W>, unit: &ComprehensiveTranslationUnit) -> Result<(), quick_xml::Error> {
    let mut u = BytesStart::new("unit");
    u.push_attribute(("id", unit.id.as_str()));
    writer.write_event(Event::Start(u))?;

    // Handle Notes natively within the XLIFF 2.0 notes module configuration
    if !unit.notes.is_empty() {
        writer.write_event(Event::Start(BytesStart::new("notes")))?;
        for note_text in &unit.notes {
            writer.write_event(Event::Start(BytesStart::new("note")))?;
            writer.write_event(Event::Text(BytesText::new(note_text)))?;
            writer.write_event(Event::End(BytesEnd::new("note")))?;
        }
        writer.write_event(Event::End(BytesEnd::new("notes")))?;
    }

    writer.write_event(Event::Start(BytesStart::new("segment")))?;
    writer.write_event(Event::Start(BytesStart::new("source")))?;
    writer.write_event(Event::Text(BytesText::new(&unit.source_text)))?;
    writer.write_event(Event::End(BytesEnd::new("source")))?;

    writer.write_event(Event::Start(BytesStart::new("target")))?;
    writer.write_event(Event::Text(BytesText::new(&unit.target_text)))?;
    writer.write_event(Event::End(BytesEnd::new("target")))?;
    writer.write_event(Event::End(BytesEnd::new("segment")))?;

    // Serialize metadata structures inside formal extensibility module tags
    if !unit.metadata.is_empty() {
        writer.write_event(Event::Start(BytesStart::new("metadata")))?;
        for (key, val) in &unit.metadata {
            let mut meta = BytesStart::new("meta");
            meta.push_attribute(("type", key.as_str()));
            writer.write_event(Event::Start(meta))?;
