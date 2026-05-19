# Cloud Build Queue

Cloud Codex should pick exactly one `READY` item, build it, commit it, and push
the branch. Keep each slice narrow.

## READY: Bootstrap Text Editor Plain V1

Status: READY

Allowed paths:
- `feature_packs/ui/text_editor_plain/**`
- `docs/text_editor_system/**`
- `Cargo.toml`

Build intent:
- Create the first plain text editor feature crate.
- Provide deterministic document state helpers for plain UTF-8 text.
- Support exact export of full document and selected text.
- Keep selection-vs-full-document behavior explicit.

Requirements:
- No UI.
- No system clipboard writes.
- No rendering engine.
- No ropes or piece table.
- Normalize line endings only where the contract explicitly says so.
- Preserve exact text by default.

Contract tests:
- empty document export
- full document export
- selected text export
- selected-or-full fallback
- selection clamping
- line ending behavior
- feature metadata contract

Verification:
- `scripts/cloud_proof.sh`
- `cargo test -p text_editor_plain`

Done criteria:
- crate exists and passes tests
- workspace registers the crate
- README explains the contract and non-goals

## READY: Clipboard Payload V1

Status: READY

Allowed paths:
- `feature_packs/ui/text_editor_clipboard/**`
- `feature_packs/ui/text_editor_plain/**` only if a compile-facing dependency
  adjustment is unavoidable
- `docs/text_editor_system/**`
- `Cargo.toml`

Build intent:
- Build clipboard payload construction without writing the system clipboard.
- Represent payload kind, export policy, source, fallback flag, character count,
  line count, and cleanup receipt metadata.

Requirements:
- Exact text export remains default.
- System clipboard writes stay host/UI-only.
- Selected text vs full document behavior must be explicit.
- Preserve cleanup receipts.

Contract tests:
- exact payload from selected text
- exact payload from full document
- selected-or-full fallback metadata
- require-selection policy
- character and line counts
- receipt preservation

Verification:
- `scripts/cloud_proof.sh`
- `cargo test -p text_editor_clipboard`

Done criteria:
- crate exists and passes tests
- metadata is host-renderable without host side effects

## READY: Text Editor Actions V1

Status: READY

Allowed paths:
- `feature_packs/ui/text_editor_actions/**`
- `feature_packs/ui/text_editor_plain/**`
- `feature_packs/ui/text_editor_clipboard/**`
- `docs/text_editor_system/**`
- `Cargo.toml`

Build intent:
- Define stable Text Editor action IDs and action execution semantics.
- Route copy/export/cleanup actions through Rust contracts without owning host
  clipboard writes.

Requirements:
- Action IDs must be stable strings.
- Disabled reasons must be explicit.
- Results must carry payload metadata and receipts.
- No UI behavior.

Contract tests:
- action registry contains known actions
- stable action ID parsing
- disabled reasons for empty input or missing selection
- copy actions return payloads without mutation
- cleanup actions return receipts

Verification:
- `scripts/cloud_proof.sh`
- `cargo test -p text_editor_actions`

Done criteria:
- crate exists and passes tests
- action semantics are documented

## READY: Host Adapter V1

Status: READY

Allowed paths:
- `feature_packs/ui/text_editor_host_adapter/**`
- `feature_packs/ui/text_editor_actions/**`
- `feature_packs/ui/text_editor_clipboard/**`
- `feature_packs/ui/text_editor_plain/**`
- `docs/text_editor_system/**`
- `Cargo.toml`

Build intent:
- Expose host-facing action/result models for UI integration.
- Keep host models display-oriented and side-effect free.

Requirements:
- No system clipboard writes.
- Host result includes payload summary metadata.
- Host disabled results preserve disabled reason.
- Runner errors use stable JSON-friendly envelopes.

Contract tests:
- host action rendering preserves metadata
- payload results become host display summaries
- disabled outputs become host disabled results
- unknown action error envelope
- malformed input error envelope

Verification:
- `scripts/cloud_proof.sh`
- `cargo test -p text_editor_host_adapter`

Done criteria:
- crate exists and passes tests
- host adapter is documented as UI-display-only

## READY: Selection Policy Matrix V1

Status: READY

Allowed paths:
- `feature_packs/ui/text_editor_plain/**`
- `feature_packs/ui/text_editor_clipboard/**`
- `feature_packs/ui/text_editor_actions/**`
- `docs/text_editor_system/**`

Build intent:
- Centralize selected-text, full-document, selected-or-full fallback, and
  require-selection behavior into a tested matrix.

Requirements:
- Exact export remains default.
- Fallback to full document must be visible in metadata.
- Missing required selection must be a structured disabled/error state.

Contract tests:
- each policy branch
- empty selection
- empty document
- non-empty selection
- fallback metadata

Verification:
- `scripts/cloud_proof.sh`
- targeted package tests for touched crates

Done criteria:
- policy behavior is centralized and tested

## READY: Cleanup Receipt Classifier V1

Status: READY

Allowed paths:
- `feature_packs/ui/text_editor_clipboard/**`
- `feature_packs/ui/text_editor_actions/**`
- `docs/text_editor_system/**`

Build intent:
- Add stable cleanup receipt categories for host display.

Requirements:
- Categories include no-op, changed-lines, normalized-line-endings,
  stripped-ansi, and warning.
- Receipts remain deterministic and inspectable.

Contract tests:
- no-op cleanup
- changed line cleanup
- normalized line endings
- stripped ANSI
- warnings

Verification:
- `scripts/cloud_proof.sh`
- targeted package tests for touched crates

Done criteria:
- receipt categories are stable and documented

## BLOCKED

No blocked slices yet.

## DONE

No completed slices yet.
