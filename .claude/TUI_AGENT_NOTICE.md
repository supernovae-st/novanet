# NOTICE FOR TUI EXTRACTION AGENTS

**Date**: $(date)
**Priority**: HIGH

## v10.9.0 Schema Changes Committed

The schema has been refactored with new naming conventions.
Your TUI extraction (info.rs, tree.rs) needs to be updated.

**Read**: `tools/novanet/TUI_CONTEXT_v10.9.0.md` for full details.

### Quick Summary

1. **Node renames**: EntityL10n→EntityContent, BlockL10n→BlockGenerated, PageL10n→PageGenerated
2. **Arc renames**: HAS_L10N→HAS_CONTENT, HAS_OUTPUT→HAS_GENERATED
3. **Type names in data.rs**: Use `RealmInfo`, `LayerInfo`, `KindInfo`, `ArcFamilyInfo`, NOT `*Node`

### Your extracted files were removed

The files `info.rs` and `tree.rs` were temporarily removed because:
- They referenced non-existent types (`RealmNode`, `KindNode`, etc.)
- mod.rs had duplicate function definitions

**Action needed**: Re-extract with correct type references from current data.rs
