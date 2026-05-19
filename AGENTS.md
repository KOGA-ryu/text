# AGENTS

This repo is the cloud build surface for the Text Editor feature system.

It is intentionally small. Do not mirror the full `/Users/kogaryu/dev/features`
repo into this repository.

## Mission

Build narrow, auditable Text Editor feature slices in Rust, then push the branch
back to GitHub for local import and UI integration.

Cloud Codex owns contract and feature logic here. The user and Spark own local
Qt/binder UI integration elsewhere.

## Start Here

Before editing anything, run:

```bash
pwd
git branch --show-current
git status --short
```

Then read:

1. `README.md`
2. `docs/cloud_build_queue.md`
3. `docs/ownership_boundaries.md`
4. `docs/slice_template.md`

## Work Rules

Pick exactly one `READY` queue item from `docs/cloud_build_queue.md`.

Allowed default writes:

- `docs/**`
- `feature_packs/ui/text_editor_*/**`
- `Cargo.toml` only to register or remove workspace members for the assigned
  text feature crates
- `.github/workflows/**` only when the task explicitly asks for CI maintenance

Forbidden default writes:

- `tools/**`
- `crates/**`
- unrelated `feature_packs/**`
- generated UI screenshots or proof artifacts
- local app/binder/Qt files

If a forbidden change seems necessary, stop and report:

- blocked file
- why it is needed
- minimal required change
- what you did not edit

## Feature Crate Contract

Each Rust feature crate must include:

- `Cargo.toml`
- `feature.toml`
- `README.md`
- `src/lib.rs`
- `tests/contract_tests.rs`
- `fixtures/`

Required `feature.toml` metadata:

- `id`
- `name`
- `kind`
- `status`
- `summary`
- `inputs`
- `outputs`
- `dependencies`
- `compatible_features`
- `tags`
- `owner`
- `created_at`
- `updated_at`

## Text Editor Boundaries

- Exact text export is the default.
- Rust crates must not write to the system clipboard.
- Rust crates may emit payloads, metadata, receipts, and host-renderable action
  results.
- Selection-vs-full-document behavior must be explicit.
- Cleanup receipts must be preserved.
- Do not build custom rendering, ropes, piece tables, syntax highlighting,
  paste expansion, AI prediction, or Qt/binder UI unless explicitly assigned.

## Verification

Run the strongest available proof for the slice.

Default proof:

```bash
scripts/cloud_proof.sh
```

The repo intentionally starts with an empty workspace. `scripts/cloud_proof.sh`
skips Rust package proof while `Cargo.toml` has no workspace members, then runs
the normal commands after the first feature crate is registered.

If a package exists, also run targeted package tests:

```bash
cargo test -p <package_name>
```

If a command cannot run in cloud, report the exact command and failure reason.

## Done

When the slice is complete:

1. Update the queue item status and notes.
2. Commit with a clear message.
3. Push the branch to GitHub.
4. Report branch, commit hash, changed files, and verification results.
