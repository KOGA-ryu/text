# Text Editor Build Queue

Cloud Dex should pick the first `READY` item, build it on a branch, commit it,
and push it back to this repo.

Do not work on Qt, binder UI, screenshots, rendering, syntax highlighting,
ropes, piece tables, paste expansion, or AI behavior from this repo.

## READY

1. Bootstrap Text Editor Plain V1
   - Create `feature_packs/ui/text_editor_plain`.
   - Add deterministic plain UTF-8 document helpers.
   - Support exact full-document export and selected-text export.
   - Make selected-or-full fallback explicit.
   - Prove with `cargo test -p text_editor_plain`.

2. Clipboard Payload V1
   - Create `feature_packs/ui/text_editor_clipboard`.
   - Build clipboard payloads without writing the system clipboard.
   - Include payload kind, export policy, source, fallback flag, character
     count, line count, and cleanup receipt metadata.
   - Prove with `cargo test -p text_editor_clipboard`.

3. Text Editor Actions V1
   - Create `feature_packs/ui/text_editor_actions`.
   - Define stable action IDs and action execution semantics.
   - Preserve disabled reasons, payload metadata, and receipts.
   - Prove with `cargo test -p text_editor_actions`.

4. Host Adapter V1
   - Create `feature_packs/ui/text_editor_host_adapter`.
   - Expose host-facing action/result models.
   - Keep host models display-oriented and side-effect free.
   - Prove with `cargo test -p text_editor_host_adapter`.

5. Selection Policy Matrix V1
   - Centralize selected-text, full-document, selected-or-full fallback, and
     require-selection behavior.
   - Make fallback and missing-selection states explicit.
   - Prove with targeted package tests plus `scripts/cloud_proof.sh`.

6. Cleanup Receipt Classifier V1
   - Add stable cleanup receipt categories for host display.
   - Include no-op, changed-lines, normalized-line-endings, stripped-ansi, and
     warning.
   - Prove with targeted package tests plus `scripts/cloud_proof.sh`.

## BLOCKED

No blocked slices yet.

## DONE

No completed slices yet.

## Cloud Dex Rules

- Work one item at a time.
- Branch format: `codex/<slice-name>`.
- Allowed paths are the assigned `feature_packs/ui/text_editor_*` crate,
  `docs/**`, and `Cargo.toml` for workspace membership.
- Run `scripts/cloud_proof.sh`.
- Commit and push when done.

Detailed slice contracts live in `docs/cloud_build_queue.md`.
