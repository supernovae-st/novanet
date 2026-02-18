# TUI Tree Visual Enrichment Design

**Date**: 2026-02-10
**Status**: Approved
**Version**: v11.6.1

## Overview

Enrich the TUI tree panel to display Realms and Layers with the same visual treatment as NodeKinds: icons, counts, and type badges.

## Current State

```
▼ Organization                              (no badges)
  ▼ Config                                  (no badges)
    ■ OrgConfig →1←1 6/9p          ●org ◎cfg │i│
```

## Target State

```
◉ Organization                 ▦6 ◇21                  ●org │R│
  ⚙ Config                     ◇1                      ◎cfg │L│
    ■ OrgConfig                →1 ←1 ⊞6/9              ●org ◎cfg │i│
  ◆ Semantic Layer             ◇4                      ◆sem │L│
    ■ Entity                   →30 ←35 ⊞6/9            ●org ◆sem │i│
    □ EntityContent            →5 ←4 ⊞13/22            ●org ◆sem │l│
```

## New Icons (counts category)

| Key | Terminal | Web | Description |
|-----|----------|-----|-------------|
| `layers_count` | `▦` | `layers` | Number of layers in realm |
| `kinds_count` | `◇` | `file-type` | Number of node kinds |
| `arcs_out` | `→` | `arrow-right` | Outgoing arc types |
| `arcs_in` | `←` | `arrow-left` | Incoming arc types |
| `props` | `⊞` | `list` | Properties (required/total) |

## Type Indicators

| Type | Badge | Meaning |
|------|-------|---------|
| Realm | `│R│` | This row is a Realm node |
| Layer | `│L│` | This row is a Layer node |
| Kind | `│i│` `│l│` `│k│` `│g│` `│a│` | Trait: invariant/localized/knowledge/generated/aggregated |

## Files to Modify

### 1. packages/core/models/visual-encoding.yaml

Add new `counts` category:

```yaml
counts:
  layers_count:
    web: "layers"
    terminal: "▦"
    description: "Number of layers in realm"
  kinds_count:
    web: "file-type"
    terminal: "◇"
    description: "Number of node kinds"
  arcs_out:
    web: "arrow-right"
    terminal: "→"
    description: "Outgoing arc types"
  arcs_in:
    web: "arrow-left"
    terminal: "←"
    description: "Incoming arc types"
  props:
    web: "list"
    terminal: "⊞"
    description: "Properties (required/total)"
```

### 2. tools/novanet/src/tui/data.rs

Add helper method on `RealmInfo`:

```rust
impl RealmInfo {
    pub fn total_kinds(&self) -> usize {
        self.layers.iter().map(|l| l.kinds.len()).sum()
    }
}
```

### 3. tools/novanet/src/tui/ui/tree.rs

#### Realm rendering (~line 570)

```rust
// Before
format!("{} {}", chevron, realm.display_name)

// After
let icon = realm_badge_icon(&realm.key);
let layers_count = realm.layers.len();
let kinds_count = realm.total_kinds();
// Format: "◉ Organization     ▦6 ◇21     ●org │R│"
```

#### Layer rendering (~line 631)

```rust
// Before
format!("{} {}", chevron, layer.display_name)

// After
let icon = layer_badge_icon(&layer.key);
let kinds_count = layer.kinds.len();
// Format: "⚙ Config     ◇1     ◎cfg │L│"
```

#### Kind rendering (~line 750)

```rust
// Before: "→1←1 6/9p"
// After:  "→1 ←1 ⊞6/9"
```

### 4. tools/novanet/src/tui/ui/help.rs

Add icons legend section:

```
╭─ TREE ICONS ──────────────────────────────────────────────────╮
│  COUNTS        TYPES           BADGES                         │
│  ▦ = layers    R = Realm       ●org = org realm               │
│  ◇ = kinds     L = Layer       ◎cfg = config layer            │
│  → = arcs out  i = invariant   ◆sem = semantic layer          │
│  ← = arcs in   l = localized   ...                            │
│  ⊞ = props     g = generated                                  │
╰───────────────────────────────────────────────────────────────╯
```

## Post-Implementation

```bash
cargo run -- schema generate    # Regenerate artifacts
cargo test                      # Verify 950 tests pass
cargo run -- tui                # Visual verification
```

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| Column alignment on different terminal widths | Test with 80, 120, 160 char widths |
| Unicode support in minimal terminals | Already handled via ADR-009 graceful degradation |

## References

- ADR-009: Terminal Color Graceful Degradation
- ADR-013: Icons Source of Truth
- `packages/core/models/visual-encoding.yaml`
- `tools/novanet/src/tui/ui/tree.rs`
