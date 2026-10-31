# Comprehensive Localization & Provenance Pipeline (`example.py`)

This standalone script demonstrates how the `tbx_pyo3` engine uses a uniform, low-allocation data architecture to ingest multi-lingual data, dynamically calculate taxonomy annotations, track deep bibliographic provenance proofs, and stream out schema-compliant files.

```python
#!/usr/bin/env python3
import os
import sys
from pathlib import Path

try:
    import tbx_pyo3
except ImportError:
    print("❌ Error: 'tbx_pyo3' extension not found in path.")
    print("💡 Please compile the module first using: 'uv pip install -e .'")
    sys.exit(1)

def run_provenance_pipeline() -> None:
    print("🚀 Initializing Low-Level Multi-Format Ingestion Engine...")
    
    # Define remote media bucket and reference resolver patterns
    URL_RESOLVER = "https://wiktionary.org"
    AUDIO_BUCKET = "https://miamioh.edu"
    
    # -------------------------------------------------------------------------
    # 1. ORCHESTRATE STRUCTURAL RECORDS (SHOW YOUR WORK SYSTEM)
    # -------------------------------------------------------------------------
    print("📝 Building linguistic units with research-grade provenance tracking...")
    
    # Unit A: Botanical Nomenclature Mapping (English -> Latin with full provenance)
    unit_maple = tbx_pyo3.ComprehensiveTranslationUnit(
        id="tu-botanical-0128",
        source_lang="en-US",
        target_lang="la",
        source_text="The Red Maple displays brilliant crimson foliage in autumn months.",
        target_text="Acer rubrum foliis coccineis autumno fulget."
    )
    
    # Inject multi-dimensional bibliographic tokens and metadata
    unit_maple.add_meta("x-scientific_name", "Acer rubrum")
    unit_maple.add_meta("provenance", "Miami Nation Tribal Archive Project - Field Script Collection")
    unit_maple.add_meta("isbn", "978-0-12-345678-9")
    unit_maple.add_meta("doi", "10.1001/botany.myaamia.0128")
    unit_maple.add_meta("library_of_congress", "QK495.A17")
    unit_maple.add_meta("dewey", "583.74")
    unit_maple.add_meta("pdf_anchor", "https://miamioh.edu")
    unit_maple.add_meta("mp3_url", "acer_rubrum_native_rec.mp3")
    
    # Localized language target expansions for downstream frontends or Whisper models
    unit_maple.add_meta("text_ja", "アカカエデ")
    unit_maple.add_meta("text_mia", "šiihšiikimeewinši")
    unit_maple.add_meta("x-ipa-mia", "/ʃiːhʃiːkimeːwinʃi/")
    
    # Audit trail annotation tracking
    unit_maple.add_note("Segment derived from high-resolution PDF scan; OCR bugs resolved against primary index.")

    # Unit B: Biological Nomenclature Mapping with SPARQL Knowledge Base extensions
    unit_wolf = tbx_pyo3.ComprehensiveTranslationUnit(
        id="tu-fauna-0441",
        source_lang="en-US",
        target_lang="la",
        source_text="The Gray Wolf tracks prey across subarctic territory.",
        target_text="Canis lupus praedam per fines subarcticos sequitur."
    )
    
    unit_wolf.add_meta("x-scientific_name", "Canis lupus")
    unit_wolf.add_meta("provenance", "Global Biodiversity Information Facility (GBIF)")
    unit_wolf.add_meta("doi", "10.21585/gbif.fauna.0441")
    unit_wolf.add_meta("library_of_congress", "QL737.C22")
    unit_wolf.add_meta("pdf_anchor", "https://miamioh.edu")
    unit_wolf.add_meta("mp3_url", "canis_lupus_audio.mp3")
    
    # Dynamic extensible token (Wiktionary Semantic SPARQL endpoint)
    unit_wolf.add_meta("Wiktionary_SPARQL_url", "https://wikidata.org")
    
    unit_wolf.add_meta("text_ja", "オオカミ")
    unit_wolf.add_meta("text_mia", "mahingani")
    unit_wolf.add_meta("x-ipa-mia", "/mahĩŋgãni/")
    unit_wolf.add_note("Verified text against standard Linnaean nomenclature schemas.")

    # Consolidate into a flat payload array vector matching Rust types
    payload = [unit_maple, unit_wolf]

    # -------------------------------------------------------------------------
    # 2. RUN HIGH-SPEED DISK STREAMING (EXPORT PIPELINE GENERATION MATRIX)
    # -------------------------------------------------------------------------
    output_dir = Path("./output_workspace")
    output_dir.mkdir(exist_ok=True)
    
    formats = {
        "tmx": "output_workspace/master_translation_memory.tmx",
        "tbx": "output_workspace/master_term_base.tbx",
        "xliff12": "output_workspace/interchange_v12.xlf",
        "xliff2": "output_workspace/interchange_v20.xlf"
    }
    
    print("\n🔥 Executing parallel streaming exports via Rust core runtime...")
    for fmt_selector, file_name in formats.items():
        print(f"   ↳ Generating validated {fmt_selector.upper()} file profile...")
        
        # Invokes the low-level Rust worker, dropping the Python GIL
        written_nodes = tbx_pyo3.execute_comprehensive_export(
            output_path=file_name,
            format_selector=fmt_selector,
            payloads=payload
        )
        
        print(f"     ✅ Successfully streamed {written_nodes} entities out to: {file_name}")

    # -------------------------------------------------------------------------
    # 3. VERIFY EXPORT INTEGRITY & VISUALIZE STRUCTURAL RESULTS
    # -------------------------------------------------------------------------
    print("\n🎉 Core multi-format data-engine processing loop completed safely!")
    print(f"📂 Open the '{output_dir.resolve()}' directory to audit your files.")

if __name__ == "__main__":
    run_provenance_pipeline()
```
