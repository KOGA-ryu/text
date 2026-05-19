#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentState {
    text: String,
}

impl DocumentState {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionRange {
    pub start: usize,
    pub end: usize,
}

impl SelectionRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    fn clamp(self, doc_len: usize) -> Self {
        let start = self.start.min(doc_len);
        let end = self.end.min(doc_len);

        if start <= end {
            Self { start, end }
        } else {
            Self {
                start: end,
                end: start,
            }
        }
    }

    fn is_empty(self) -> bool {
        self.start == self.end
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportPolicy {
    FullDocument,
    SelectedText,
    SelectedOrFullDocument,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportedText {
    pub text: String,
    pub policy: ExportPolicy,
    pub used_fallback_to_full_document: bool,
}

pub fn export_text(
    document: &DocumentState,
    selection: Option<SelectionRange>,
    policy: ExportPolicy,
) -> ExportedText {
    let full_text = document.text();
    let doc_len = full_text.len();
    let clamped_selection = selection.map(|range| range.clamp(doc_len));

    let (text, fallback) = match policy {
        ExportPolicy::FullDocument => (full_text.to_string(), false),
        ExportPolicy::SelectedText => {
            let selected = clamped_selection
                .and_then(|range| full_text.get(range.start..range.end))
                .unwrap_or("");
            (selected.to_string(), false)
        }
        ExportPolicy::SelectedOrFullDocument => {
            if let Some(range) = clamped_selection {
                if !range.is_empty() {
                    let selected = full_text.get(range.start..range.end).unwrap_or("");
                    (selected.to_string(), false)
                } else {
                    (full_text.to_string(), true)
                }
            } else {
                (full_text.to_string(), true)
            }
        }
    };

    ExportedText {
        text,
        policy,
        used_fallback_to_full_document: fallback,
    }
}
