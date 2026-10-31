# 💎 tbx_pyo3

[![PyPI Version](https://shields.io)](https://pypi.org)
[![Build System](https://shields.io)](https://pypa.io)
[![Type Checking](https://shields.io)](https://readthedocs.io)
[![Backend](https://shields.io)](https://rust-lang.org)

A high-performance, low-allocation localization data engine built in **Rust via PyO3** and managed using **Hatchling**. Inspired by frameworks like **Lokit**, `tbx_pyo3` is built specifically to handle **Research-Grade Provenance Attributions** across industry-standard formats (**TBX, TMX, XLIFF 1.2, and XLIFF 2.x**).

Most localization libraries provide bare-minimum compliance, discarding custom properties, inline markers, or semantic context. This engine addresses that limitation, allowing you to track and preserve the rigorous attributions demanded by **field linguistics, indigenous documentation (e.g., ILDA Myaamia/Potawatomi), downstream AI workflows (MCP), and automated TTS training pipelines.**

---

## 🚀 Key Features

*   **Lossless Extended Metadata Layer (`<prop>`, `<termNote>`):** Native mapping of critical metadata parameters including **ISBN, DOI, PDF Page Anchors, Library of Congress numbers, and Dewey Decimal descriptors** seamlessly across variations of TMX, TBX, and XLIFF.
*   **Latin Binomial Scanning Engine (`$Genus $species`):** High-speed native compilation arrays automatically extract taxonomic markers (e.g., *Acer rubrum*, *Canis lupus*) out of generic notes or segments, standardizing them into strict property nodes.
*   **Professional IPA Notation Anchors:** Preserves and extracts International Phonetic Alphabet (IPA) strings (`x-ipa-[lang]`) directly within translation structures—essential for JavaScript frontend phonetic renderers, advanced speech synthesis (TTS), or speech-to-text alignments.
*   **AI Training & MCP Ready Data Loops:** Outputs structured arrays that convert cleanly into downstream metadata targets (such as `audio.metadata.json` templates for raw Whisper voice training files), complete with verifiable structural provenance proofs.
*   **"Show Your Work" Proof System:** Ensures total visibility for quality assurance. Translators and systems can trace segments back to their original OCR sources, PDF page offsets, or historical dictionary indices to keep your pipeline free of transcription defects.

---

## 🏗️ Supported Cross-Format Equivalents

| Format / Standard | Concept-Oriented Meta Component | Segment-Oriented Meta Component |
| :--- | :--- | :--- |
| **TBX (ISO 30042)** | `<descrip type="isbn">`, `<termNote>` | `<termNote type="x-ipa-mia">` |
| **TMX (1.4+)** | *Concept blocks map to flat lists* | `<prop type="x-scientific_name">` |
| **XLIFF 1.2** | `<trans-unit>` custom properties | `<prop type="doi">` properties |
| **XLIFF 2.x** | `<metadata>` modular namespace maps | `<meta type="pdf_anchor">` maps |

---

## 📦 Package Tree Architecture

```text
tbx_pyo3/
├── Cargo.toml               # Rust dependencies (quick-xml, regex)
├── pyproject.toml           # Hatchling + hatch-pyo3-plugin definition
├── README.md                # Component documentation
└── src/
    ├── lib.rs               # Zero-allocation streaming Rust core
    └── tbx_pyo3/
        ├── __init__.py      # Package initializations
        ├── py.typed         # PEP 561 static types verification marker
        └── tbx_pyo3.pyi     # Complete Mypy signature type stubs
```

---

## 🐍 Structural End-to-End Execution Profile

This example showcases the ingestion and alignment of multilingual biological nomenclature. It handles the structural conversion from an English vernacular string down through Latin taxonomy identifiers into Indigenous language systems, preserving phonetics and verifiable source citations.

```python
import tbx_pyo3

# 1. Instantiate the comprehensive translation structure tracking botanical properties
record = tbx_pyo3.ComprehensiveTranslationUnit(
    id="concept-acer-0128",
    source_lang="en-US",
    target_lang="la",
    source_text="The Red Maple displays vibrant crimson foliage in autumn months.",
    target_text="Acer rubrum foliis coccineis autumno fulget."
)

# 2. Append rich, high-fidelity research credentials and phonetic notation markers
record.add_meta("x-scientific_name", "Acer rubrum")
record.add_meta("provenance", "Miami Nation Tribal Archive Project - Field Script Collection")
record.add_meta("isbn", "978-0-12-345678-9")
record.add_meta("doi", "10.1002/botany.myaamia.0128")
record.add_meta("library_of_congress", "QK495.A17")
record.add_meta("dewey", "583.74")
record.add_meta("pdf_anchor", "https://miamioh.edu")

# Dynamic localized language extensions mapped to the root node
record.add_meta("text_ja", "アカカエデ")
record.add_meta("text_mia", "šiihšiikimeewinši")

# International Phonetic Alphabet mappings for linguistic tools or TTS audio pipelines
record.add_meta("x-ipa-mia", "/ʃiːhʃiːkimeːwinʃi/")

# Complete traceability loop: link back to raw files to "show your work"
record.add_note("Segment derived from high-resolution PDF scan; OCR errors resolved against primary index.")

# 3. Create the ingestion sequence list payload
dataset_payload = [record]

# 4. Execute streaming exports natively across your workspace pipeline file signatures
# Outputs clean <prop type="x-scientific_name"> properties inside a TMX data dump
tbx_pyo3.execute_comprehensive_export("output_workspace.tmx", "tmx", dataset_payload)

# Outputs strict, schema-compliant <conceptEntry> structures mapped to standard TBX Core parameters
tbx_pyo3.execute_comprehensive_export("output_workspace.tbx", "tbx", dataset_payload)

# Outputs fully structured metadata bundles inline matching standard XLIFF 2.x modules
tbx_pyo3.execute_comprehensive_export("output_workspace.xlf", "xliff2", dataset_payload)

print("💎 Localization provenance assets generated losslessly across all standard formats.")
```

---

## 🛠️ Build and Local Deployment

Compile and deploy your package locally using standard Python build orchestration tools:

```bash
# Install package dependencies and build extensions in editable mode
uv pip install -e .

# Validate type annotations pass strict Mypy verification loops
mypy --strict src/tbx_pyo3

# Package and build project distributions (wheels & sdist) via Hatchling
uv build
```

---

## 📜 License

This system configuration component is distributed under the conditions of the **MIT License**.
