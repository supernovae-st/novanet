# Source Panel Redesign Plan

**Version:** v0.17.3
**Date:** 2026-03-07
**Status:** ✅ COMPLETE

## Summary

Simplify the TUI Source panel by removing the confusing YAML/Data toggle and replacing it with context-aware content.

## Implementation Summary

| Phase | Commit | Description |
|-------|--------|-------------|
| 1 | `82fb3b61` | Remove SourceTab enum and toggle logic |
| 2-3 | `8d02fe41` | ContentPanelMode + context-aware rendering |
| 4 | `c9fc06a0` | Rename Focus::Yaml → Focus::Content |
| 5 | `4a0446a2` | Add ContentPanelMode and TreeItemData tests |
| 6 | `8f4b4c84` | Documentation updates |

**Results:**
- 1264 tests passing
- Zero clippy warnings
- All deprecated code removed

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Toggle | **Remove** | Adds complexity without value |
| Panel name | **SCHEMA** / **INFO** | Clear semantic meaning |
| Instance view | **Message pointing to PROPERTIES** | No duplication |
| Data location | **PROPERTIES only** | Single source of truth |

## Before/After

### Before (confusing)
```
┌─ SOURCE ▶YAML◀ [Data] ⊞286 │ org/semantic/entity-native.yaml ─┐
│ [Toggle between YAML file and instance data]                   │
│ [Duplicates PROPERTIES panel for instances]                    │
└────────────────────────────────────────────────────────────────┘
```

### After (simple)
```
# When Class/ArcClass selected:
┌─ SCHEMA ──────────────── EntityNative ─────────────────────────┐
│ # models/node-classes/org/semantic/entity-native.yaml          │
│ name: EntityNative                                              │
│ realm: org                                                      │
│ layer: semantic                                                 │
│ ...                                                             │
└─────────────────────────────────────────────────────────────────┘

# When Instance selected:
┌─ INFO ───────────────── barcode@en-US ─────────────────────────┐
│                                                                 │
│   Instance: barcode@en-US                                       │
│   Class: EntityNative (org/semantic)                            │
│                                                                 │
│   ─────────────────────────────────────────────────────────     │
│                                                                 │
│   Instances are stored in Neo4j, not YAML files.                │
│   → See PROPERTIES panel for instance data                      │
│   → Press Tab to navigate to PROPERTIES                         │
│                                                                 │
│   [s] View parent class schema                                  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘

# When Realm/Layer/Section selected:
┌─ INFO ───────────────── shared ────────────────────────────────┐
│                                                                 │
│   Realm: shared                                                 │
│   4 layers, 38 node classes                                     │
│                                                                 │
│   Select a Class to view its YAML schema.                       │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Implementation Phases

### Phase 1: Remove SourceTab enum and toggle logic

**Files:** `app.rs`

1. Delete `SourceTab` enum (lines ~209-230)
2. Delete `source_tab` field from `App` struct
3. Delete `source_tab_class_cursor` field from `App` struct
4. Delete `toggle_source_tab()` method
5. Delete `has_instances_for_current_class()` if only used by toggle
6. Remove 't' keybinding handler

**Commit:** `refactor(tui): remove SourceTab enum and toggle logic`

### Phase 2: Create content mode detection

**Files:** `app.rs`

1. Add method `fn content_panel_mode(&self) -> ContentPanelMode`
2. Create enum:
   ```rust
   pub enum ContentPanelMode {
       Schema { path: String, class_name: String },
       InstanceInfo { instance_key: String, class_name: String, realm: String, layer: String },
       SectionInfo { name: String, description: String },
   }
   ```
3. Derive mode from current tree selection

**Commit:** `feat(tui): add ContentPanelMode for context-aware panel`

### Phase 3: Update yaml_panel.rs rendering

**Files:** `yaml_panel.rs`

1. Rename `render_yaml_panel` to `render_content_panel`
2. Remove tab bar (`▶YAML◀ [Data]`)
3. Add `render_schema_content()` - shows YAML with syntax highlighting
4. Add `render_instance_info()` - shows helpful message
5. Add `render_section_info()` - shows section overview
6. Update title to show SCHEMA or INFO based on mode
7. Remove `generate_instance_lines()` function (no longer needed)

**Commit:** `refactor(tui): simplify content panel with context-aware rendering`

### Phase 4: Update panel registration

**Files:** `ui/mod.rs`, `app.rs`

1. Update `Panel` enum if needed (Yaml → Content?)
2. Update `Focus` enum if needed
3. Update `panel_rects` registration
4. Update keybinding hints in status bar

**Commit:** `refactor(tui): rename Yaml panel to Content panel`

### Phase 5: Update and add tests

**Files:** `app.rs` (tests module)

1. Remove tests for `toggle_source_tab`
2. Remove tests for `SourceTab`
3. Add tests for `ContentPanelMode` detection
4. Add tests for mode transitions

**Commit:** `test(tui): update tests for content panel redesign`

### Phase 6: Update documentation

**Files:** `KEYBINDINGS.md`, `CLAUDE.md`

1. Remove 't' keybinding documentation
2. Update panel descriptions
3. Document new behavior

**Commit:** `docs(tui): update documentation for content panel`

## Rollback Plan

If issues arise:
```bash
git revert HEAD~N  # Revert N commits
```

## Testing Checklist

- [x] Class selection shows YAML (SCHEMA mode)
- [x] Instance selection shows info message (INFO mode)
- [x] Realm/Layer selection shows section info
- [x] ArcClass selection shows YAML
- [x] No 't' keybinding (should do nothing)
- [x] Tab navigation still works between panels
- [x] Scroll works in SCHEMA mode
- [x] Focus highlighting works
- [x] All existing tests pass (1264)
- [x] No clippy warnings

## Completion Date

**Completed:** 2026-03-07
