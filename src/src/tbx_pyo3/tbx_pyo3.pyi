from typing import Dict, List

class ComprehensiveTranslationUnit:
    """
    A fully normalized, high-fidelity provenance translation data model.
    """
    id: str
    source_text: str
    target_text: str
    source_lang: str
    target_lang: str
    metadata: Dict[str, str]
    notes: List[str]

    def __init__(
        self, 
        id: str, 
        source_text: str, 
        target_text: str, 
        source_lang: str, 
        target_lang: str
    ) -> None: ...

    def add_meta(self, key: str, value: str) -> None: 
        """Dynamically tracks provenance keys (isbn, doi, pdf_anchor, x-scientific_name, etc.)"""
        ...

    def add_note(self, note: str) -> None: 
        """Appends a custom translation unit annotation node string."""
        ...

def execute_comprehensive_export(
    output_path: str,
    format_selector: str,  # Literal["tmx", "tbx", "xliff12", "xliff2"]
    payloads: List[ComprehensiveTranslationUnit]
) -> int:
    """
    Streams low-level translation elements out to validated target files losslessly.
    Returns the total number of successfully written structural nodes.
    """
    ...
