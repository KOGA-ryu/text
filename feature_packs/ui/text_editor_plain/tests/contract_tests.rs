use text_editor_plain::{export_text, DocumentState, ExportPolicy, SelectionRange};

#[test]
fn empty_document_export() {
    let doc = DocumentState::new("");
    let exported = export_text(&doc, None, ExportPolicy::FullDocument);
    assert_eq!(exported.text, "");
    assert!(!exported.used_fallback_to_full_document);
}

#[test]
fn full_document_export() {
    let doc = DocumentState::new("hello\nworld");
    let exported = export_text(
        &doc,
        Some(SelectionRange::new(0, 5)),
        ExportPolicy::FullDocument,
    );
    assert_eq!(exported.text, "hello\nworld");
    assert!(!exported.used_fallback_to_full_document);
}

#[test]
fn selected_text_export() {
    let doc = DocumentState::new("hello world");
    let exported = export_text(
        &doc,
        Some(SelectionRange::new(6, 11)),
        ExportPolicy::SelectedText,
    );
    assert_eq!(exported.text, "world");
}

#[test]
fn selected_or_full_fallback_when_selection_missing_or_empty() {
    let doc = DocumentState::new("alpha beta");

    let missing = export_text(&doc, None, ExportPolicy::SelectedOrFullDocument);
    assert_eq!(missing.text, "alpha beta");
    assert!(missing.used_fallback_to_full_document);

    let empty = export_text(
        &doc,
        Some(SelectionRange::new(3, 3)),
        ExportPolicy::SelectedOrFullDocument,
    );
    assert_eq!(empty.text, "alpha beta");
    assert!(empty.used_fallback_to_full_document);
}

#[test]
fn selection_clamping() {
    let doc = DocumentState::new("abcdef");

    let clamped = export_text(
        &doc,
        Some(SelectionRange::new(2, 100)),
        ExportPolicy::SelectedText,
    );
    assert_eq!(clamped.text, "cdef");

    let swapped = export_text(
        &doc,
        Some(SelectionRange::new(5, 1)),
        ExportPolicy::SelectedText,
    );
    assert_eq!(swapped.text, "bcde");
}

#[test]
fn line_endings_preserved() {
    let text = "a\r\nb\nc\rd";
    let doc = DocumentState::new(text);
    let exported = export_text(&doc, None, ExportPolicy::FullDocument);
    assert_eq!(exported.text, text);
}

#[test]
fn feature_metadata_contract() {
    let metadata = include_str!("../feature.toml");
    for required_key in [
        "id =",
        "name =",
        "kind =",
        "status =",
        "summary =",
        "inputs =",
        "outputs =",
        "dependencies =",
        "compatible_features =",
        "tags =",
        "owner =",
        "created_at =",
        "updated_at =",
    ] {
        assert!(
            metadata.contains(required_key),
            "missing key: {required_key}"
        );
    }
}
