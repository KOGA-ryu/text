# Ownership Boundaries

This repo exists so Cloud Codex can build Text Editor feature logic without
colliding with local UI work.

## Cloud Codex Owns

- Text Editor Rust feature crates under `feature_packs/ui/text_editor_*/`
- contract tests
- fixtures
- feature metadata
- docs describing Text Editor feature behavior
- queue updates for completed cloud slices

## User And Spark Own

- Qt/binder UI layout
- button/dropdown/palette visual design
- screenshots and UI-tree proof artifacts
- local app integration in the broader `features` repo
- final import decisions

## Shared Files

`Cargo.toml` is shared only for workspace membership. Cloud Codex may edit it
when creating or removing a text-editor crate for an assigned queue item.

## Escalation Protocol

If a slice appears to require UI files, broader harness files, or unrelated
feature crates, stop and report:

- blocked file
- reason the change is needed
- minimal required change
- what was not edited

Do not make the shared or forbidden change silently.
