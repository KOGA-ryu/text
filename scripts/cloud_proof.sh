#!/usr/bin/env bash
set -euo pipefail

if grep -Eq '^[[:space:]]*members[[:space:]]*=[[:space:]]*\[\][[:space:]]*$' Cargo.toml; then
  echo "No workspace members registered yet; skipping Rust package proof."
else
  cargo fmt --all --check
  cargo test --workspace
fi

git diff --check
