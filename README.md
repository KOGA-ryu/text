# Text Editor Feature Cloud Repo

This repository is the cloud workbench for Text Editor feature slices.

It is intentionally not the full `features` repo. Cloud Codex should build
small Rust text-editor crates and docs here, push branches to GitHub, and leave
local UI integration to the user and Spark.

## Workflow

1. Read `BUILD_QUEUE.md`.
2. Verify the cloud remote with `scripts/cloud_setup_remote.sh`.
3. Pick one `READY` item.
4. Create a branch named `codex/<slice-name>`.
5. Edit only the allowed paths for that slice.
6. Run proof.
7. Commit and push.

## Build Queue

The visible Cloud Dex task list is here:

- [`BUILD_QUEUE.md`](BUILD_QUEUE.md)

Detailed slice contracts are kept in:

- [`docs/cloud_build_queue.md`](docs/cloud_build_queue.md)

## Repository Shape

```text
docs/
  cloud_build_queue.md
  ownership_boundaries.md
  slice_template.md
  text_editor_system/
feature_packs/ui/
  text_editor_*/
```

Feature crates are added under `feature_packs/ui/` as the queue advances.

## Default Proof

```bash
scripts/cloud_proof.sh
```

The workspace starts with no feature crates. The proof script skips Rust package
commands until the first workspace member is registered.
