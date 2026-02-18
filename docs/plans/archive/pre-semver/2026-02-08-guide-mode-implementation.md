# Guide Mode Implementation Plan

**Date**: 2026-02-08
**Design**: `2026-02-08-guide-mode-design.md`
**Method**: Subagent-Driven Development with TDD

---

## Overview

Implement the Guide mode TUI feature as specified in the design document.
This mode provides interactive educational views of the NovaNet taxonomy.

---

## Architecture

```
src/tui/
├── guide/                    # NEW MODULE
│   ├── mod.rs               # GuideState, GuideTab enum
│   ├── traits.rs            # Traits tab rendering
│   ├── layers.rs            # Layers tab rendering
│   ├── arcs.rs              # Arcs tab rendering
│   └── pipeline.rs          # Pipeline tab with animation
├── app.rs                   # Add NavMode::Guide, guide state
├── data.rs                  # Add trait/layer stats queries
├── ui/mod.rs                # Add render_guide() dispatch
└── mod.rs                   # Add guide data loading
```

---

## Tasks (Subagent-Driven)

### Batch 1: Foundation (Quick Wins)

| Task | File | Description |
|------|------|-------------|
| 1.1 | `app.rs` | Add `NavMode::Guide` variant |
| 1.2 | `app.rs` | Add `GuideState` field to `App` |
| 1.3 | `guide/mod.rs` | Create `GuideTab` enum (Traits, Layers, Arcs, Pipeline) |
| 1.4 | `guide/mod.rs` | Create `GuideState` struct |

**Acceptance**: `cargo test` passes, `cargo clippy` clean

### Batch 2: State Management

| Task | File | Description |
|------|------|-------------|
| 2.1 | `guide/mod.rs` | Implement `GuideState::new()`, `handle_key()` |
| 2.2 | `app.rs` | Wire Guide key handling in main `handle_key()` |
| 2.3 | `app.rs` | Add keyboard shortcut `5` for Guide mode |
| 2.4 | `ui/mod.rs` | Add `NavMode::Guide` to header tabs |

**Acceptance**: Can switch to Guide mode with `5`, see header tab

### Batch 3: Traits Tab (MVP)

| Task | File | Description |
|------|------|-------------|
| 3.1 | `guide/traits.rs` | Create constellation layout (5 traits) |
| 3.2 | `guide/traits.rs` | Create detail panel (selected trait info) |
| 3.3 | `data.rs` | Add `load_trait_stats()` Cypher query |
| 3.4 | `guide/mod.rs` | Integrate traits tab rendering |

**Acceptance**: Traits constellation visible, navigation works

### Batch 4: Layers Tab

| Task | File | Description |
|------|------|-------------|
| 4.1 | `guide/layers.rs` | Split view (Global | Tenant) |
| 4.2 | `guide/layers.rs` | Layer cards with kind counts |
| 4.3 | `data.rs` | Add `load_layer_stats()` if needed |
| 4.4 | `guide/mod.rs` | Integrate layers tab |

**Acceptance**: Layers split view renders correctly

### Batch 5: Arcs Tab

| Task | File | Description |
|------|------|-------------|
| 5.1 | `guide/arcs.rs` | Arc families grid |
| 5.2 | `guide/arcs.rs` | Arc scope section |
| 5.3 | `data.rs` | Add arc stats queries |
| 5.4 | `guide/mod.rs` | Integrate arcs tab |

**Acceptance**: Arc families and scope visible

### Batch 6: Pipeline Tab

| Task | File | Description |
|------|------|-------------|
| 6.1 | `guide/pipeline.rs` | Static pipeline diagram |
| 6.2 | `guide/pipeline.rs` | Animation with tick counter |
| 6.3 | `guide/pipeline.rs` | Space key to play/pause |
| 6.4 | `guide/mod.rs` | Integrate pipeline tab |

**Acceptance**: Pipeline renders, animation works with Space

### Batch 7: Drill-Down & Polish

| Task | File | Description |
|------|------|-------------|
| 7.1 | `guide/mod.rs` | Add drill-down state (trait -> kinds) |
| 7.2 | `guide/traits.rs` | Render kind list when drilled |
| 7.3 | `ui/overlays.rs` | Update help overlay with Guide shortcuts |
| 7.4 | All | Add breadcrumb rendering |

**Acceptance**: Full drill-down flow works

### Batch 8: Quick Jump & Ludic

| Task | File | Description |
|------|------|-------------|
| 8.1 | `guide/mod.rs` | Add quick jump shortcuts (gi, gl, gk, gd, gj) |
| 8.2 | `guide/mod.rs` | Add "Did you know?" tips rotation |
| 8.3 | `theme.rs` | Ensure trait colors are used |
| 8.4 | All | Final polish and edge cases |

**Acceptance**: All features from design doc implemented

### Batch 9: Verification & TDD

| Task | File | Description |
|------|------|-------------|
| 9.1 | `guide/mod.rs` | Unit tests for GuideState key handling |
| 9.2 | `guide/mod.rs` | Unit tests for tab navigation |
| 9.3 | `guide/mod.rs` | Unit tests for drill-down state |
| 9.4 | `data.rs` | Integration tests for trait/layer queries |
| 9.5 | All | Code review with rust-pro agent |
| 9.6 | All | Performance review with rust-perf agent |
| 9.7 | All | Final cargo test + clippy + fmt |

**Acceptance**: 100% test coverage for GuideState, all tests pass, clippy clean

---

## Key Implementation Details

### GuideTab Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GuideTab {
    #[default]
    Traits,
    Layers,
    Arcs,
    Pipeline,
}
```

### GuideState Struct

```rust
pub struct GuideState {
    pub tab: GuideTab,
    pub trait_cursor: usize,      // 0-4 for 5 traits
    pub layer_cursor: usize,
    pub layer_realm: usize,       // 0=global, 1=tenant
    pub arc_cursor: usize,
    pub pipeline_stage: usize,
    pub pipeline_animating: bool,
    pub drill_depth: usize,       // 0=overview, 1=kinds, 2=instances
    pub drill_cursor: usize,
}
```

### Cypher Query for Trait Stats

```cypher
MATCH (t:NodeTrait)<-[:HAS_TRAIT]-(nk:NodeKind)
RETURN t.key AS trait, COUNT(nk) AS kind_count
ORDER BY t.key
```

---

## Testing Strategy

1. **Unit tests** for GuideState key handling
2. **Integration tests** for Neo4j queries
3. **Visual testing** by running TUI manually

---

## Dependencies

- Existing `TaxonomyTree` in `data.rs`
- Theme colors from `theme.rs`
- Atlas module pattern for reference

---

## Notes

- Follow Atlas module structure as reference
- Use FxHashMap for any lookup tables (performance)
- Keep rendering functions pure (no side effects)
- Add comments for complex ASCII art
