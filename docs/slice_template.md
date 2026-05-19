# Slice Template

Use this shape for new queue items.

```md
## <status>: <slice name>

Status: READY | IN_PROGRESS | BLOCKED | DONE

Branch:
Commit:

Allowed paths:
- `docs/**`
- `feature_packs/ui/text_editor_<name>/**`
- `Cargo.toml`

Forbidden paths:
- `tools/**`
- `crates/**`
- unrelated `feature_packs/**`
- generated UI proof artifacts

Build intent:
- 

Requirements:
- 

Contract tests:
- 

Verification:
- `scripts/cloud_proof.sh`

Done criteria:
- 

Notes:
- 
```
