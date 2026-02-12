# Plan: Enrich Meta Node Detail Panels in TUI

**Date**: 2026-02-12
**Version**: v11.7.1
**Status**: Draft

## Brainstorm Decisions (2026-02-12)

| Question | Decision |
|----------|----------|
| **Objectives** | Pedagogical + Visual + All node types (A+C+D) |
| **Visual distinction** | Badge `[◇meta]` + Background color + Distinct icons |
| **Badge format** | `[◇meta]` (unicode diamond + text) |
| **Tree display** | Suffix badge on meta lines + different background + badge in info panel |
| **Info panel detail** | Ultra-complete: all sections + example instances + related arcs |

### Meta vs Data Visual System

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  VISUAL DISTINCTION: META vs DATA                                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  IN TREE VIEW:                                                              │
│  ─────────────                                                              │
│  ▼ ◉ Realm:shared [◇meta]        ▓▓▓▓ (darker background)                  │
│    ▼ ⚙ Layer:config [◇meta]      ▓▓▓▓                                      │
│      ▼ ◆ Kind:Locale [◇meta]     ▓▓▓▓                                      │
│        ● Locale:fr-FR            ░░░░ (normal background = data)           │
│        ● Locale:en-US            ░░░░                                      │
│                                                                             │
│  IN INFO PANEL HEADER:                                                      │
│  ─────────────────────                                                      │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │  [◇meta] Realm                                                        │  │
│  │  ══════════════════════════════════════════════════════════════════  │  │
│  │  type      Meta Node (schema)                                         │  │
│  │  key       shared                                                     │  │
│  │  ...                                                                  │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
│  BADGE LEGEND:                                                              │
│  ─────────────                                                              │
│  [◇meta]  = Schema/structure node (Realm, Layer, Kind, ArcFamily, ArcKind) │
│  (no badge) = Data/instance node (actual content)                          │
│                                                                             │
│  COLOR ENCODING:                                                            │
│  ───────────────                                                            │
│  Meta nodes: rgb(30, 35, 45) background (slightly darker)                  │
│  Data nodes: default terminal background                                   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Pedagogical Goal

Users should understand at a glance:
1. **Meta graph** = structure/schema (60 Kinds, 114 ArcKinds) — defines WHAT can exist
2. **Data graph** = instances (~20,000 nodes) — actual content
3. **Connection** = `OF_KIND` arc bridges meta→data (each instance points to its Kind)

## Problem Statement

The TUI detail panel displays vastly different amounts of information depending on node type:

| Node Type | Lines of Code | Sections Displayed |
|-----------|---------------|-------------------|
| **Kind** | ~280 lines | header, realm, layer, trait, instances, properties, PROPERTY COVERAGE, arcs, description, cypher |
| **Instance** | ~200 lines | header, realm, layer, trait, properties, PROPERTY COVERAGE, Arc Diagram |
| **Layer** | ~70 lines | type, key, realm, kinds, TRAIT BREAKDOWN |
| **Realm** | ~60 lines | type, key, layers, kinds, LAYER BREAKDOWN |
| **ArcFamily** | ~17 lines | type, key, arcs, hint |
| **ArcKind** | ~50 lines | type, key, family, from, to, cardinality, description |

The `llm_context` field from taxonomy.yaml is NEVER displayed, despite containing 60-200 words of rich documentation per meta node.

## YAML Fields Available (taxonomy.yaml)

### Realm
```yaml
- key: shared
  display_name: Shared
  emoji: "🌍"
  color: "#2aa198"
  llm_context: |
    Shared across ALL organizations. Universal locale knowledge...
    (150-200 words)
  layers: [...]
```

### Layer
```yaml
- key: config
  display_name: Config
  emoji: "⚙️"
  color: "#64748b"
  llm_context: |
    Invariant definitions for shared realm...
    (80-150 words)
```

### Trait
```yaml
- key: invariant
  display_name: Invariant
  color: "#3b82f6"
  border_style: solid
  border_width: 2
  unicode_border: "─"
  llm_context: |
    Nodes that do not change between locales...
```

### ArcFamily
```yaml
- key: ownership
  display_name: Ownership
  color: "#3b82f6"
  stroke_style: solid
  stroke_width: 2
  arrow_style: "-->"
  default_traversal: eager
  llm_context: |
    Parent-child structural relationships...
```

### kind_retrieval_defaults (per trait)
```yaml
kind_retrieval_defaults:
  invariant:
    traversal_depth: 2
    context_budget: 500
    token_estimate: 100
```

## Proposed Changes

### 1. Realm Panel Enhancement

**Current:**
```
type      Realm
key       shared
layers    4
kinds     39

LAYER BREAKDOWN
══════════════════════════
  Config           ████░░░░░░░░  8%   3
  Locale           ████░░░░░░░░ 15%   6
  ...
```

**Proposed:**
```
type      Realm
key       shared
icon      🌍 (web: globe)
color     #2aa198

layers    4
kinds     39
instances 20,630

DESCRIPTION
══════════════════════════
  Shared across ALL organizations. Universal locale
  knowledge that applies everywhere: cultural norms,
  linguistic patterns, formatting conventions.
  READ-ONLY for organizations.

LAYER BREAKDOWN
══════════════════════════
  Config           ████░░░░░░░░  8%   3
  ...

ARC STATISTICS
══════════════════════════
  Inbound:  12 arcs from org realm
  Outbound: 0 (read-only)

USAGE HINT
══════════════════════════
  Cross-realm traversal: org → shared (one-way)
  Query: MATCH (r:Realm {key: "shared"}) ...
```

### 2. Layer Panel Enhancement

**Current:**
```
type      Layer
key       config
realm     Shared
kinds     3

TRAIT BREAKDOWN
══════════════════════════
  ■ invariant      ████████████ 100%  3
```

**Proposed:**
```
type      Layer
key       config
icon      ⚙️ (web: settings)
color     #64748b

realm     ◉ Shared
kinds     3
instances 214

DESCRIPTION
══════════════════════════
  Invariant definitions for shared realm. All nodes
  have invariant trait. v11.5: Locale (BCP-47 codes),
  EntityCategory, SEOKeywordFormat.

TRAIT BREAKDOWN
══════════════════════════
  ■ invariant      ████████████ 100%  3

NODE KINDS BY TRAIT
══════════════════════════
  ■ invariant:
    • Locale (200)
    • EntityCategory (13)
    • SEOKeywordFormat (1)

ARC DIRECTIONS
══════════════════════════
  Intra-realm arcs: 5 types
  Cross-realm arcs: 2 types (→ org)

NAVIGATION
══════════════════════════
  Parent: shared (Realm)
  Children: 3 Kinds
```

### 3. ArcFamily Panel Enhancement

**Current:**
```
type      ArcFamily
key       ownership
arcs      43

h/l to collapse/expand
```

**Proposed:**
```
type      ArcFamily
key       ownership
icon      → (web: arrow-right)
color     #3b82f6

arcs      43
instances 23,295

DESCRIPTION
══════════════════════════
  Parent-child structural relationships. A node "owns"
  or "contains" another. Always follow in traversal
  (structural backbone).

VISUAL ENCODING
══════════════════════════
  stroke:     solid, 2px
  arrow:      -->
  traversal:  eager (always follow)

ARC SCOPE DISTRIBUTION
══════════════════════════
  intra_realm    ████████████  93%  40
  cross_realm    █░░░░░░░░░░░   7%   3

TOP ARC KINDS
══════════════════════════
  → HAS_PAGE        (12,340 instances)
  → HAS_BLOCK       (8,921 instances)
  → HAS_ENTITY      (1,034 instances)
  ...

NAVIGATION
══════════════════════════
  h/l to collapse/expand
```

### 4. ArcKind Panel Enhancement

**Current:**
```
type      ArcKind
key       HAS_PAGE
family    Ownership
from      Project
to        Page
cardin.   1:N

Description
  Project contains pages
```

**Proposed:**
```
type      ArcKind
key       HAS_PAGE
icon      → (family: ownership)
color     #3b82f6

family    → Ownership
scope     intra_realm
from      Project (org/foundation)
to        Page (org/structure)
cardin.   1:N

instances 12,340

DESCRIPTION
══════════════════════════
  Project contains pages. A project HAS_PAGE one or
  more Pages that form the website structure.

VISUAL ENCODING
══════════════════════════
  (inherited from Ownership family)
  stroke: solid, 2px
  traversal: eager

EXAMPLE INSTANCES
══════════════════════════
  qrcode-ai → homepage
  qrcode-ai → pricing
  qrcode-ai → features/dynamic-qr

RELATED ARCS
══════════════════════════
  Same source (Project):
    → HAS_ENTITY (1,034)
    → HAS_BRAND (1)
  Same target (Page):
    ← GENERATED_FROM (PageGenerated)

CYPHER
══════════════════════════
  MATCH ()-[r:HAS_PAGE]->() RETURN r LIMIT 100
```

## Implementation Tasks

### Phase 1: Data Loading (Rust)

1. **Load llm_context from taxonomy.yaml**
   - File: `tools/novanet/src/tui/data.rs`
   - Add `llm_context: String` field to `Realm`, `Layer` structs
   - Parse from `TaxonomyTree` during load

2. **Add instance counts to meta nodes**
   - Query Neo4j for instance counts per Realm/Layer
   - Add to tree data during async load

3. **Load ArcFamily visual encoding**
   - Add `stroke_style`, `stroke_width`, `default_traversal` to `ArcFamily` struct
   - Parse from taxonomy.yaml

4. **Load ArcKind scope**
   - Add `scope: String` field to `ArcKind` struct
   - Parse from arc-kinds/*.yaml files

### Phase 2: Panel Rendering (Rust)

5. **Enrich Realm panel** (`info.rs` ~line 582)
   - Add DESCRIPTION section with wrapped llm_context
   - Add icon and color display
   - Add instance count
   - Add ARC STATISTICS section

6. **Enrich Layer panel** (`info.rs` ~line 643)
   - Add DESCRIPTION section
   - Add icon and color display
   - Add instance count
   - Add NODE KINDS BY TRAIT section
   - Add ARC DIRECTIONS section

7. **Enrich ArcFamily panel** (`info.rs` ~line 992)
   - Add DESCRIPTION section
   - Add VISUAL ENCODING section
   - Add ARC SCOPE DISTRIBUTION
   - Add TOP ARC KINDS by instance count

8. **Enrich ArcKind panel** (`info.rs` ~line 1010)
   - Add scope display (CRITICAL - currently missing!)
   - Add instance count
   - Add VISUAL ENCODING section
   - Add EXAMPLE INSTANCES
   - Add RELATED ARCS section

### Phase 3: Tree Enhancements

9. **Add llm_context to YAML panel**
   - When viewing Realm/Layer, show llm_context in YAML panel
   - Syntax highlight the description text

10. **Add badges to tree items**
    - Layer: show instance count badge
    - ArcFamily: show arc count + instance count

### Phase 4: Meta/Data Visual Distinction (from Brainstorm)

11. **Add `[◇meta]` badge suffix to tree lines** (`tree.rs`)
    - Meta nodes (Realm, Layer, Kind, ArcFamily, ArcKind): append ` [◇meta]`
    - Data nodes (instances): no badge
    - Implementation: modify `render_tree_item()` function

12. **Add darker background for meta nodes** (`tree.rs`)
    - Define `META_BG` color in theme: `rgb(30, 35, 45)`
    - Apply to Realm, Layer, Kind, ArcFamily, ArcKind lines
    - Data nodes keep default background

13. **Add `[◇meta]` badge to info panel header** (`info.rs`)
    - First line of panel shows badge before type
    - Add `type: Meta Node (schema)` line for meta nodes
    - Add `type: Data Node (instance)` line for data nodes

14. **Update tree item rendering** (`tree.rs`)
    - Current: `▼ ◉ Realm:shared  ▦6 ◇21  │ ●org │R│`
    - New:     `▼ ◉ Realm:shared [◇meta]  ▦6 ◇21  │ ●org │R│`

### Phase 5: Ultra-Complete Info Panels (from Brainstorm)

15. **Add EXAMPLE INSTANCES section** (all meta panels)
    - Query Neo4j for 3-5 example instances
    - Show with clickable keys for navigation
    - Format: `  • instance_key (property_preview)`

16. **Add RELATED ARCS section** (ArcKind panel)
    - Same source: list other arcs from same source Kind
    - Same target: list other arcs to same target Kind
    - Show instance counts in parentheses

17. **Add RELATED KINDS section** (Realm/Layer panels)
    - For Realm: show Kinds grouped by Layer
    - For Layer: show Kinds grouped by Trait
    - Clickable navigation to Kind panels

## Files to Modify

| File | Changes |
|------|---------|
| `tools/novanet/src/tui/data.rs` | Add llm_context, visual encoding fields |
| `tools/novanet/src/tui/ui/info.rs` | Enrich all meta node panels, add [◇meta] badge |
| `tools/novanet/src/tui/ui/tree.rs` | Add [◇meta] suffix, darker background for meta |
| `tools/novanet/src/tui/theme.rs` | Add META_BG color constant |
| `tools/novanet/src/parsers/taxonomy.rs` | Parse llm_context field |
| `tools/novanet/src/parsers/yaml_arc.rs` | Parse scope field |

## Estimated Effort

| Phase | Tasks | Estimated Lines |
|-------|-------|-----------------|
| Phase 1 | Data loading | +100 lines |
| Phase 2 | Panel rendering | +400 lines |
| Phase 3 | Tree enhancements | +50 lines |
| Phase 4 | Meta/Data distinction | +80 lines |
| Phase 5 | Ultra-complete panels | +150 lines |
| **Total** | | **~780 lines** |

## Success Criteria

1. All meta nodes (Realm, Layer, ArcFamily, ArcKind) display `llm_context`
2. ArcKind displays `scope` (intra_realm / cross_realm)
3. All meta nodes show instance counts from Neo4j
4. Visual encoding (color, stroke, icon) is displayed
5. No regression on Kind/Instance panels (already rich)
6. **[◇meta] badge visible on ALL meta nodes** in tree and info panel
7. **Darker background** distinguishes meta from data in tree view
8. **EXAMPLE INSTANCES** section shows real data samples
9. **RELATED ARCS/KINDS** enable navigation discovery

## References

- `packages/core/models/taxonomy.yaml` - Source of truth for meta fields
- `tools/novanet/src/tui/ui/info.rs` - Current panel rendering code
- ADR-022 in `novanet-decisions.md` - Unified Tree Architecture
