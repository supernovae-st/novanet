# TUI Meta/Data View Toggle Design

**Date**: 2026-02-05
**Status**: Approved (brainstorm complete)
**Version**: v10.6.0

## Overview

Enhance the TUI to show both meta-graph (Kind definitions) and data-graph (instances) using the same tree structure, with content adapting based on the selected view.

## Current State

- `[1]Meta` view shows Kind definitions (Realm > Layer > Kind)
- `[2]Data` view exists but doesn't show instances under Kinds
- Tree structure is Realm > Layer > Kind only

## Design

### Core Concept

**Same tree, content adapts to view mode.**

When switching between `[1]Meta` and `[2]Data`:
- Tree selection is preserved
- Panel content changes based on view
- In Data view, Kinds expand to show instances

### View Comparison

```
┌─────────────────────────────────────────────────────────────────────────┐
│ [1]Meta                        │ [2]Data                                │
├─────────────────────────────────────────────────────────────────────────┤
│ TREE (left panel)              │ TREE (left panel)                      │
│ ▼ Global                       │ ▼ Global                               │
│   ▼ Locale Knowledge           │   ▼ Locale Knowledge                   │
│     • Locale      ◀─ selected  │     ▼ Locale         ◀─ expanded       │
│     • Term                     │       • fr-FR        ◀─ instances      │
│                                │       • en-US                          │
│                                │       • ja-JP                          │
├─────────────────────────────────────────────────────────────────────────┤
│ INFO (middle panel)            │ INFO (middle panel)                    │
│ Kind: Locale                   │ Instance: fr-FR                        │
│ realm: global                  │ kind: Locale                           │
│ layer: locale-knowledge        │ key: fr-FR                             │
│ trait: knowledge               │ language: fr                           │
│ Properties (12)...             │ region: FR                             │
│                                │ display_name: Français (France)        │
├─────────────────────────────────────────────────────────────────────────┤
│ YAML (right panel)             │ JSON (right panel)                     │
│ node:                          │ {                                      │
│   name: Locale                 │   "key": "fr-FR",                      │
│   realm: global                │   "language": "fr",                    │
│   ...                          │   ...                                  │
├─────────────────────────────────────────────────────────────────────────┤
│ DIAGRAM (bottom of info)       │ DIAGRAM (bottom of info)               │
│ Schema arcs possibles          │ Real + Missing arcs comparison         │
│ ┌────────┐                     │ ┌────────┐                             │
│ │ Locale │─[:HAS_TERMS]─>      │ │ fr-FR  │══[:HAS_TERMS]══> ✓          │
│ └────────┘─[:HAS_CULTURE]─>    │ └────────┘╌╌[:HAS_CULTURE]╌> ✗ missing │
│                                │            ══[:HAS_VOICE]══> ✓         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Behavior Details

#### Tree Panel (Left)

| View | Tree Structure | Selection |
|------|---------------|-----------|
| Meta | Realm > Layer > Kind | Kind = leaf node |
| Data | Realm > Layer > Kind > Instance | Instance = leaf node |

- Switching views preserves selection at Kind level
- In Data view, selecting a Kind shows instance list in info panel
- In Data view, selecting an Instance shows instance details

#### Info Panel (Middle)

| View | Content |
|------|---------|
| Meta | Kind definition: name, realm, layer, trait, properties, description |
| Data (Kind selected) | Instance count + list of instances |
| Data (Instance selected) | Instance properties from Neo4j |

#### Right Panel

| View | Content |
|------|---------|
| Meta | YAML source file of the Kind |
| Data | JSON properties of the selected instance |

#### Diagram (Bottom of Info)

| View | Content |
|------|---------|
| Meta | Schema arcs (what connections are possible) |
| Data | Comparison view: existing arcs (solid ══) vs missing arcs (dashed ╌╌) |

### Visual Encoding for Data Diagram

```
══════════  Solid double line = arc EXISTS in Neo4j
╌╌╌╌╌╌╌╌╌╌  Dashed line = arc MISSING (defined in schema but not created)

┌────────┐
│ fr-FR  │══[:HAS_TERMS]══════> TermSet:fr-FR-terms     ✓ exists
│ Locale │╌╌[:HAS_CULTURE]╌╌╌╌> (not connected)         ✗ missing
└────────┘══[:HAS_VOICE]══════> LocaleVoice:fr-FR       ✓ exists
```

This helps identify data completeness at a glance.

## Implementation Notes

### Dependencies

- Meta graph visual is being updated in a parallel session
- Coordinate to avoid conflicts in `src/tui/ui.rs`

### Required Changes

1. **app.rs**: Add instance data to state, load from Neo4j on view switch
2. **data.rs**: Extend TaxonomyTree to hold instances per Kind
3. **ui.rs**: Render instances in tree, adapt panels based on view
4. **Neo4j queries**: Fetch instances for selected Kind

### New Cypher Queries Needed

```cypher
// Get instances for a Kind
MATCH (n:{KindLabel})
RETURN n.key, n.display_name, properties(n)
ORDER BY n.key
LIMIT 100

// Get arcs for an instance (for diagram)
MATCH (n {key: $key})-[r]->(m)
RETURN type(r) as arc_type, m.key as target_key, labels(m)[0] as target_kind
```

## Out of Scope

- `[3]Overlay` and `[4]Query` views (future enhancement)
- Editing instances from TUI (read-only for now)
- Pagination for large instance counts (use LIMIT 100 initially)

## References

- Current TUI: `tools/novanet/src/tui/`
- Brainstorm session: 2026-02-05
