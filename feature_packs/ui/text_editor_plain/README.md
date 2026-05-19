# text_editor_plain

`text_editor_plain` is the bootstrap Text Editor feature crate for deterministic
plain UTF-8 document handling.

## Contract

- Exact text export is the default behavior.
- Selection behavior is explicit and policy-driven.
- Selected-or-full fallback is represented in metadata.
- Selection indices are clamped to document boundaries.
- Line endings are preserved exactly (no implicit normalization).

## Non-goals

- UI rendering
- system clipboard writes
- ropes / piece tables
- syntax highlighting
- text mutation or cleanup behavior

## Primary API

- `DocumentState` stores the source UTF-8 text.
- `SelectionRange` describes an optional selection window.
- `ExportPolicy` controls selection/full-document behavior.
- `export_text` returns `ExportedText` with fallback metadata.
