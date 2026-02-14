# NovaNet Nomenclature v9.5 Design

> **Status**: Approved
> **Date**: 2026-02-03
> **Author**: Thibaut + Claude

## Overview

Complete nomenclature refactoring for NovaNet v9.5 to establish consistent, unambiguous terminology across YAML, TypeScript, Rust, Neo4j, and UI.

## Core Vocabulary

### Base Terms

| Level | Vertex | Edge |
|-------|--------|------|
| General | Node | Arc |
| Instance (data) | NodeData | ArcData |
| Definition (meta) | NodeKind | ArcKind |

### Symmetric Taxonomy

Prefix on TYPE, not on property.

**NodeKind Axes:**

| Axis | Type | Property | Values |
|------|------|----------|--------|
| WHERE? | `NodeRealm` | `realm` | `global`, `project`, `shared` |
| WHAT? | `NodeLayer` | `layer` | `config`, `knowledge`, `foundation`, `structure`, `semantic`, `instruction`, `output`, `seo`, `geo` |
| HOW? | `NodeTrait` | `trait` | `invariant`, `localized`, `knowledge`, `derived`, `job` |

**ArcKind Axes:**

| Axis | Type | Property | Values |
|------|------|----------|--------|
| SCOPE | `ArcScope` | `scope` | `intra_realm`, `cross_realm` |
| FUNCTION | `ArcFamily` | `family` | `ownership`, `localization`, `semantic`, `generation`, `mining` |
| MULTIPLICITY | `ArcCardinality` | `cardinality` | `one_to_one`, `one_to_many`, `many_to_many` |

### TypeScript Interfaces

```typescript
// Types with prefix (globally unique)
type NodeRealm = 'global' | 'project' | 'shared';
type NodeLayer = 'config' | 'knowledge' | 'foundation' | 'structure' | 'semantic' | 'instruction' | 'output' | 'seo' | 'geo';
type NodeTrait = 'invariant' | 'localized' | 'knowledge' | 'derived' | 'job';

type ArcScope = 'intra_realm' | 'cross_realm';
type ArcFamily = 'ownership' | 'localization' | 'semantic' | 'generation' | 'mining';
type ArcCardinality = 'one_to_one' | 'one_to_many' | 'many_to_many';

// Interfaces with short property names (context is clear)
interface NodeKind {
  name: string;
  realm: NodeRealm;
  layer: NodeLayer;
  trait: NodeTrait;
}

interface ArcKind {
  name: string;
  scope: ArcScope;
  family: ArcFamily;
  cardinality: ArcCardinality;
  source: NodeKind[];
  target: NodeKind[];
}
```

## Complete Rename Map

### YAML

| Old | New |
|-----|-----|
| `organizing-principles.yaml` | `taxonomy.yaml` |
| `nodes/` | `node-classes/` |
| `relations.yaml` | `arc-classes/**/*.yaml` (split by family) |
| `edge_families` | `arc_families` |
| `family: ownership` | `family: ownership` (unchanged) |
| (new) | `scopes: [intra_realm, cross_realm]` |
| (new) | `cardinalities: [one_to_one, one_to_many, many_to_many]` |

### TypeScript

| Old | New |
|-----|-----|
| `Edge` | `Arc` |
| `EdgeKind` | `ArcKind` |
| `EdgeFamily` | `ArcFamily` |
| `edges.ts` | `arcs.ts` |
| `schemaEdge.ts` | `schemaArc.ts` |
| `SchemaEdge` | `SchemaArc` |
| `relationType` | `arcKind` |
| (new) | `ArcScope` |
| (new) | `NodeRealm`, `NodeLayer`, `NodeTrait` |

### Rust

| Old | New |
|-----|-----|
| `EdgeFamily` | `ArcFamily` |
| `EdgeKind` | `ArcKind` |
| `edge_schema.rs` | `arc_schema.rs` |
| `relations.rs` | `arcs.rs` |
| `organizing.rs` | `taxonomy.rs` |

### Neo4j Labels

| Old | New |
|-----|-----|
| `:EdgeFamily:Meta` | `:ArcFamily:Meta` |
| `:EdgeKind:Meta` | `:ArcKind:Meta` |
| (new) | `:ArcScope:Meta` |

### UI Labels

| Old | New |
|-----|-----|
| "types" | "node kinds" / "arc kinds" |
| "edges" | "arcs" |
| "relations" | "arcs" |

## YAML Structure

### New Directory Layout

```
packages/core/models/
├── _index.yaml
├── taxonomy.yaml                    # Realm, Layer, Trait, ArcFamily, ArcScope, ArcCardinality
│
├── node-classes/                      # 1 file per NodeKind, organized by Realm/Layer
│   ├── global/
│   │   ├── config/
│   │   │   └── locale.yaml
│   │   └── knowledge/
│   │       ├── locale-voice.yaml
│   │       ├── locale-culture.yaml
│   │       └── ...
│   ├── project/
│   │   ├── foundation/
│   │   │   ├── project.yaml
│   │   │   └── brand-identity.yaml
│   │   ├── structure/
│   │   │   ├── page.yaml
│   │   │   └── block.yaml
│   │   ├── semantic/
│   │   ├── instruction/
│   │   └── output/
│   └── shared/
│       ├── seo/
│       └── geo/
│
├── arc-classes/                       # 1 file per ArcKind, organized by ArcFamily
│   ├── ownership/
│   │   ├── has-concept.yaml
│   │   ├── has-page.yaml
│   │   ├── has-block.yaml
│   │   └── ...
│   ├── localization/
│   │   ├── uses-locale.yaml
│   │   ├── has-l10n.yaml
│   │   └── ...
│   ├── semantic/
│   │   ├── uses-concept.yaml
│   │   ├── semantic-link.yaml
│   │   └── ...
│   ├── generation/
│   │   └── generated-by.yaml
│   └── mining/
│       └── targets-keyword.yaml
│
├── views/                           # Unchanged
├── config/                          # Unchanged
├── docs/                            # Generated
└── schema/                          # Generated
```

### taxonomy.yaml Example

```yaml
# packages/core/models/taxonomy.yaml
# Source of truth for NovaNet taxonomy structure
# v9.5.0 — Unified Nomenclature

version: "9.5.0"

# NodeKind classification axes
node_realms:
  - key: global
    display_name: Global
    emoji: "🌍"
    color: "#2aa198"

  - key: project
    display_name: Project
    emoji: "📦"
    color: "#6c71c4"

  - key: shared
    display_name: Shared
    emoji: "🎯"
    color: "#cb4b16"

node_layers:
  - key: config
    realm: global
    display_name: Configuration
    emoji: "⚙️"

  - key: knowledge
    realm: global
    display_name: Locale Knowledge
    emoji: "📚"

  # ... etc

node_traits:
  - key: invariant
    display_name: Invariant
    color: "#3b82f6"

  - key: localized
    display_name: Localized
    color: "#22c55e"

  # ... etc

# ArcKind classification axes
arc_scopes:
  - key: intra_realm
    display_name: Intra-Realm
    description: Arc between nodes in the same realm

  - key: cross_realm
    display_name: Cross-Realm
    description: Arc between nodes in different realms

arc_families:
  - key: ownership
    display_name: Ownership
    color: "#3b82f6"
    arrow_style: "-->"

  - key: localization
    display_name: Localization
    color: "#22c55e"
    arrow_style: ".->"

  # ... etc

arc_cardinalities:
  - key: one_to_one
    display_name: "1:1"

  - key: one_to_many
    display_name: "1:N"

  - key: many_to_many
    display_name: "N:M"
```

### arc-classes file example

```yaml
# packages/core/models/arc-classes/ownership/has-page.yaml
# ArcKind definition

name: HAS_PAGE
family: ownership
scope: intra_realm
cardinality: one_to_many

source: Project
target: Page

llm_context: |
  Project owns its page structures. Use to enumerate pages for generation.
```

## Implementation Plan

### Strategy

6 phases with Ralph Wiggum checkpoint between each. Phase 0 = DX first, then 5 implementation phases.

```
┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐
│ PHASE 0 │───▶│ PHASE 1 │───▶│ PHASE 2 │───▶│ PHASE 3 │───▶│ PHASE 4 │───▶│ PHASE 5 │
│   DX    │ 🔍 │  YAML   │ 🔍 │  RUST   │ 🔍 │   TS    │ 🔍 │   UI    │ 🔍 │  NEO4J  │
│  Docs   │    │ Source  │    │ Parsers │    │  Types  │    │ Studio  │    │  Seeds  │
└─────────┘    └─────────┘    └─────────┘    └─────────┘    └─────────┘    └─────────┘
```

### Phase 0: DX & Documentation — Update First

**Objective**: Update all developer-facing documentation and Claude Code tooling BEFORE implementation.

**Why first?**
- Claude Code uses these files to understand the project
- Skills reference terminology that will change
- CLAUDE.md files are the source of truth for AI assistants
- Prevents confusion during implementation

**Zones**:

1. **CLAUDE.md Files**
   - `/CLAUDE.md` (root) — Update terminology section
   - `/apps/studio/CLAUDE.md` — Edge → Arc in schema docs
   - `/tools/novanet/CLAUDE.md` — Update command references

2. **Claude Code Rules**
   - `.claude/rules/novanet-terminology.md` — Add Arc/ArcKind/ArcFamily
   - `.claude/rules/novanet-decisions.md` — Add v9.5 nomenclature decisions

3. **Claude Code Skills**
   - `/novanet-arch` — Update ASCII diagrams with Arc terminology
   - `/novanet-sync` — Reference taxonomy.yaml instead of organizing-principles.yaml
   - `/schema-add-node` — Update prompts for node-classes/ path
   - `/schema-edit-node` — Update prompts
   - `/schema-add-relation` — Rename to `/schema-add-arc`, update for arc-classes/

4. **Design Documents**
   - Update `nomenclature-v95-design.md` (this file) — ✓ Done
   - Update `v10-brainstorm-decisions.md` — Align terminology

**Checkpoint**:
- All CLAUDE.md files use Arc terminology
- All skills reference new paths (taxonomy.yaml, node-classes/, arc-classes/)
- Claude Code understands v9.5 nomenclature when asked
- No "Edge" in documentation (except React Flow references)

### Phase 1: YAML — Source of Truth

**Objective**: Restructure YAML without breaking the rest.

**Zones**:
1. `taxonomy.yaml` (new file from organizing-principles.yaml)
2. `node-classes/` (rename from nodes/)
3. `arc-classes/` (split from relations.yaml)
4. Cleanup (delete old files)

**Checkpoint**:
- All YAML files valid (yamllint)
- Consistent structure
- No orphan files
- Documentation updated

### Phase 2: Rust — Parsers & Generators

**Objective**: Update Rust parsers to read new YAML structure.

**Zones**:
1. Structs & Enums (EdgeFamily → ArcFamily, etc.)
2. Parsers (organizing.rs → taxonomy.rs, relations.rs → arcs.rs)
3. Generators (edge_schema.rs → arc_schema.rs)
4. Commands
5. Tests (396 must pass)

**Checkpoint**:
- `cargo build` ✓
- `cargo clippy -- -D warnings` ✓
- `cargo test` ✓ (396 tests)
- `cargo run -- schema validate` ✓
- `cargo run -- schema generate --dry-run` ✓

### Phase 3: TypeScript — Types & Core

**Objective**: Update @novanet/core with new nomenclature.

**Zones**:
1. Base types (edges.ts → arcs.ts)
2. Graph utilities (schemaEdge.ts → schemaArc.ts)
3. Filters
4. Regenerate from Rust
5. Tests

**Checkpoint**:
- `pnpm type-check` ✓
- `pnpm lint` ✓
- `pnpm test --filter=@novanet/core` ✓
- No "edge" or "Edge" in code (except React Flow)

### Phase 4: UI — Studio & TUI

**Objective**: Update user interface.

**Zones**:
1. Studio Components (SchemaEdge.tsx → SchemaArc.tsx)
2. Studio Stores
3. Studio Labels (UI text)
4. TUI
5. Tests + visual review

**Checkpoint**:
- `pnpm build` ✓
- `pnpm test` ✓ (all packages)
- UI visual review ✓
- No "edge" visible in UI

### Phase 5: Neo4j — Seeds & Migrations

**Objective**: Update database.

**Zones**:
1. Regenerate seeds
2. Migration script (for existing DB)
3. Reset & Seed

**Checkpoint**:
- Neo4j schema correct ✓
- All labels renamed ✓
- `cargo run -- meta` ✓ (shows ArcKind, ArcFamily)
- `cargo run -- tui` ✓
- E2E test: UI Studio + Neo4j ✓

## Summary

| Phase | Zone | Est. Files | Risk | Checkpoint |
|-------|------|------------|------|------------|
| 0 | DX & Docs | ~15 | Low | Claude Code understands v9.5 |
| 1 | YAML | ~60 | Low | yamllint, structure |
| 2 | Rust | ~20 | Medium | cargo test (400+) |
| 3 | TypeScript | ~20 | Medium | pnpm type-check, test |
| 4 | UI + Visual Encoding | ~35 | Medium | visual review, TUI colors |
| 5 | Neo4j | ~10 | Medium | DB verification |

**Total**: ~160 files, 6 separate PRs, 6 Ralph Wiggum checkpoints

**Phase 0 DX Details**:

| Zone | Files | Description |
|------|-------|-------------|
| CLAUDE.md | 3 | Root, Studio, TUI documentation |
| Rules | 2 | terminology.md, decisions.md |
| Skills | 5 | novanet-arch, novanet-sync, schema-* |
| Design docs | 2 | This file, v10-brainstorm |
| Misc | 3 | README updates, hooks |

**Phase 4 Visual Encoding Details**:

| Sub-Zone | Files | Description |
|----------|-------|-------------|
| 6. Foundation | 4 | visual-encoding.yaml, generator, 3 outputs |
| 7. Studio | 5 | hook, FilterChip, GraphNode, GraphArc, animations |
| 8. TUI | 3 | theme.rs, color detection, box drawing |

**Unified for all consumers**:
- Studio (React) — TypeScript types + JSON runtime
- TUI (Rust) — Compiled structs + terminal palette
- Future CLIs — Same YAML source, platform-specific generators

## Accepted Risks

- ⚠️ "Arc" ≠ Neo4j "Relationship" — mental translation layer accepted
- ⚠️ 50+ files in arc-classes/ — file explosion accepted
- ⚠️ Scope is derived but stored — explicit > implicit

---

## Visual Encoding System

### Principle: Separation of Concerns

```
taxonomy.yaml        → WHAT exists + COLORS (ontology, structure, palette)
visual-encoding.yaml → HOW it renders (mapping rules, states, animations)
```

**Critical Rule: NO COLOR DUPLICATION**
- Colors are defined ONCE in `taxonomy.yaml` (realms, layers, traits, arc_families)
- `visual-encoding.yaml` REFERENCES taxonomy colors via `channel_mapping`
- Never duplicate hex values between files

**Why separate?**
- taxonomy.yaml = stable structure + palette (changes rarely)
- visual-encoding.yaml = presentation rules (can evolve independently)
- Different consumers: TUI needs terminal palette, Studio needs CSS states
- Single source of truth for colors prevents drift

### visual-encoding.yaml

New file defining visual rules for all graph elements. References colors from `taxonomy.yaml`.

```yaml
# packages/core/models/visual-encoding.yaml
# Visual presentation rules for NovaNet graph elements
# v9.5.0 — Unified Visual System
#
# IMPORTANT: Colors are NOT defined here.
# Colors come from taxonomy.yaml (node_realms, node_layers, node_traits, arc_families)
# This file defines HOW to USE those colors, not WHAT they are.

version: "9.5.0"

# =============================================================================
# CHANNEL MAPPING — Which visual channel encodes which facet
# =============================================================================
# References taxonomy.yaml color definitions (no hex values here)

channel_mapping:
  node:
    fill_color: layer        # Uses taxonomy.node_layers[].color
    border_style: trait      # Uses trait_borders below
    border_color: realm      # Uses taxonomy.node_realms[].color at 60% opacity
    icon: kind               # Uses kind_icons below
    spatial_grouping: realm  # Layout clustering by realm

  arc:
    stroke_color: family     # Uses taxonomy.arc_families[].color
    stroke_style: scope      # solid (intra_realm) vs dashed (cross_realm)
    arrow_head: cardinality  # single (1:1), double (1:N), crow's foot (N:M)
    label_position: auto     # center for short labels, offset for long

# =============================================================================
# NODE VISUAL STATES (Studio: CSS, TUI: ratatui::Style)
# =============================================================================

node_states:
  default:
    opacity: 1.0
    scale: 1.0
    shadow: none

  filtered:
    opacity: 0.3
    scale: 1.0
    shadow: none

  focused:
    opacity: 1.0
    scale: 1.05
    shadow: "0 0 12px {color}40"  # {color} = resolved facet color
    ring: "2px solid {color}"

  hover:
    opacity: 1.0
    scale: 1.02
    shadow: "0 2px 8px rgba(0,0,0,0.2)"

  selected:
    opacity: 1.0
    scale: 1.0
    ring: "2px solid white"

# =============================================================================
# ARC VISUAL STATES
# =============================================================================

arc_states:
  default:
    opacity: 0.6
    stroke_width: 1.5

  highlighted:
    opacity: 1.0
    stroke_width: 2.5
    animated: true  # marching ants effect

  filtered:
    opacity: 0.15
    stroke_width: 1

  hover:
    opacity: 1.0
    stroke_width: 2
    label_visible: true

# =============================================================================
# TRAIT BORDER STYLES — Studio (CSS) + TUI (Unicode)
# =============================================================================

trait_borders:
  invariant:
    # Studio (CSS)
    css_style: solid
    css_width: 2px
    # TUI (Unicode box-drawing)
    unicode_char: "─"
    unicode_style: light
    # Documentation
    description: "Stable, doesn't change between locales"

  localized:
    css_style: dashed
    css_width: 2px
    css_dash_array: "6 3"
    unicode_char: "┄"
    unicode_style: dashed
    description: "Generated natively per locale"

  knowledge:
    css_style: double
    css_width: 3px
    unicode_char: "═"
    unicode_style: double
    description: "Locale knowledge reference data"

  derived:
    css_style: dotted
    css_width: 2px
    css_dash_array: "2 2"
    unicode_char: "·"
    unicode_style: dotted
    description: "Computed from other nodes"

  job:
    css_style: solid
    css_width: 1px
    css_corner_radius: 8px  # rounded corners for jobs
    unicode_char: "─"
    unicode_style: thin
    description: "Processing task / generation job"

# =============================================================================
# TERMINAL PALETTE — TUI color downsampling
# =============================================================================
# Maps taxonomy colors to terminal-safe values for different capabilities

terminal:
  # Auto-detect terminal capability, or force specific mode
  color_mode: auto  # auto | truecolor | 256 | 16

  # Graceful degradation chain:
  # truecolor (24-bit) → 256 colors → 16 colors (ANSI)

  # 256-color palette mapping (xterm color indices)
  # Used when terminal doesn't support truecolor
  palette_256:
    # Realms
    realm_global: 36      # cyan
    realm_project: 99     # violet
    realm_shared: 166     # orange

    # Layers
    layer_config: 245     # gray
    layer_knowledge: 135  # purple
    layer_foundation: 62  # blue-violet
    layer_structure: 71   # green
    layer_semantic: 178   # gold
    layer_instruction: 109 # teal
    layer_output: 72      # cyan-green
    layer_seo: 32         # blue
    layer_geo: 37         # cyan

    # Traits
    trait_invariant: 33   # blue
    trait_localized: 35   # green
    trait_knowledge: 135  # purple
    trait_derived: 214    # orange
    trait_job: 245        # gray

    # Arc Families
    arc_ownership: 33     # blue
    arc_localization: 35  # green
    arc_semantic: 178     # gold
    arc_generation: 135   # purple
    arc_mining: 166       # orange

  # 16-color ANSI fallback (for minimal terminals)
  palette_16:
    realm_global: cyan
    realm_project: magenta
    realm_shared: red
    layer_default: white
    trait_invariant: blue
    trait_localized: green
    arc_default: white

  # TUI-specific modifiers
  modifiers:
    selected: [bold, reverse]
    focused: [bold]
    dimmed: [dim]
    highlighted: [bold, underlined]

# =============================================================================
# ICON MAPPING — Lucide icons per Kind
# =============================================================================

kind_icons:
  # Global / Config
  Locale: "globe"

  # Global / Knowledge
  LocaleVoice: "mic"
  LocaleCulture: "heart"
  LocaleLexicon: "book-open"
  LocaleRulesAdaptation: "sliders"
  LocaleRulesFormatting: "type"
  LocaleRulesSlug: "link"

  # Project / Foundation
  Project: "folder"
  BrandIdentity: "palette"

  # Project / Structure
  Page: "file-text"
  Block: "square"
  PageType: "layout"
  BlockType: "component"

  # Project / Semantic
  Concept: "lightbulb"
  ConceptL10n: "languages"

  # Project / Instruction
  PagePrompt: "message-square"
  BlockPrompt: "terminal"
  BlockRules: "shield-check"

  # Project / Output
  PageL10n: "file-check"
  BlockL10n: "check-square"

  # Shared / SEO
  SEOKeywordL10n: "search"
  SEOKeywordMetrics: "bar-chart"
  SEOMiningRun: "pickaxe"

  # Shared / GEO
  GEOSeedL10n: "map-pin"
  GEOSeedMetrics: "trending-up"
  GEOMiningRun: "cpu"

# =============================================================================
# ANIMATION PRESETS (Studio only)
# =============================================================================

animations:
  expand_node:
    duration_ms: 200
    easing: "ease-out"

  filter_change:
    duration_ms: 150
    easing: "ease-in-out"
    stagger_ms: 20  # per node

  focus_transition:
    duration_ms: 300
    easing: "ease-out"

  layout_change:
    duration_ms: 500
    easing: "ease-in-out"
```

### Generator Output

Rust generator (`generators/visual_encoding.rs`) reads both `taxonomy.yaml` and `visual-encoding.yaml` to produce three outputs:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  INPUT                                                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│  taxonomy.yaml         → Colors (hex), display names, emojis                │
│  visual-encoding.yaml  → Channel mapping, states, terminal palette          │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  GENERATOR: tools/novanet/src/generators/visual_encoding.rs                 │
├─────────────────────────────────────────────────────────────────────────────┤
│  • Merges taxonomy colors with visual rules                                 │
│  • Validates no duplicate color definitions                                 │
│  • Generates platform-specific outputs                                      │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                    ┌───────────────┼───────────────┐
                    ▼               ▼               ▼
┌───────────────────────┐ ┌─────────────────┐ ┌─────────────────────────────┐
│  visual-encoding.ts   │ │  visual.json    │ │  visual_encoding.rs         │
│  @novanet/core        │ │  Studio runtime │ │  TUI compile-time           │
├───────────────────────┤ ├─────────────────┤ ├─────────────────────────────┤
│  • TypeScript types   │ │  • JSON bundle  │ │  • Rust structs + consts    │
│  • State constants    │ │  • Hot-reload   │ │  • Terminal palette lookup  │
│  • CSS helper fns     │ │  • useVisual()  │ │  • Unicode border chars     │
│  • Animation presets  │ │                 │ │  • Color mode detection     │
└───────────────────────┘ └─────────────────┘ └─────────────────────────────┘
```

| Output | Target | Purpose |
|--------|--------|---------|
| `visual-encoding.ts` | @novanet/core | TypeScript types, constants, CSS helpers |
| `visual-encoding.json` | Studio runtime | JSON for `useVisualEncoding()` hook |
| `visual_encoding.rs` | TUI compile-time | Embedded Rust structs, terminal colors |

### Integration Points

**Studio Components**:
- `FilterChip` — uses `node_states` for visual feedback
- `GraphNode` — uses `channel_mapping.node` for styling
- `GraphArc` — uses `channel_mapping.arc` + `arc_states`
- `useVisualEncoding()` hook — reads visual-encoding.json at runtime

**TUI (Rust)**:
- `tui/theme.rs` — imports generated `visual_encoding.rs`
- Terminal color mode auto-detection (truecolor → 256 → 16)
- Uses `palette_256` / `palette_16` for graceful degradation
- Uses `trait_borders.unicode_char` for box drawing characters
- Uses `terminal.modifiers` for selected/focused/dimmed states

**CLI**:
- Does not use visual encoding (text-only output)
- Colors handled separately via `--color=auto|always|never` flag

### Phase 4 Addition

Visual Encoding adds 3 sub-phases to Phase 4:

```
Phase 4: UI — Studio & TUI + Visual Encoding
══════════════════════════════════════════════════════════════════════════════

Zone 1-5: Original scope (SchemaArc.tsx, stores, labels, TUI, tests)

Zone 6: Visual Encoding Foundation
  ├── Create visual-encoding.yaml (channel mapping, states, terminal palette)
  ├── Create generators/visual_encoding.rs (Rust generator)
  ├── Generate visual-encoding.ts → @novanet/core
  ├── Generate visual-encoding.json → apps/studio/src/design/
  └── Generate visual_encoding.rs → tools/novanet/src/generated/

Zone 7: Studio Integration
  ├── Create useVisualEncoding() hook
  ├── Update FilterChip to use node_states from hook
  ├── Update GraphNode to use channel_mapping
  ├── Update GraphArc to use arc_states
  └── Replace hardcoded animation durations with animations config

Zone 8: TUI Integration
  ├── Create/update tui/theme.rs with VisualEncoding struct
  ├── Implement terminal color mode detection
  ├── Use palette_256/palette_16 for color downsampling
  ├── Use trait_borders.unicode_char for box drawing
  └── Use terminal.modifiers for state styling

Checkpoint:
  ✓ visual-encoding.yaml validates (yamllint)
  ✓ cargo run -- schema generate produces all 3 outputs
  ✓ Studio builds and renders correctly
  ✓ TUI renders with correct colors in 256-color terminal
  ✓ TUI gracefully degrades to 16 colors
  ✓ All tests pass
```

### Files Summary

| File | Type | Location |
|------|------|----------|
| `visual-encoding.yaml` | Source | `packages/core/models/` |
| `visual_encoding.rs` | Generator | `tools/novanet/src/generators/` |
| `visual-encoding.ts` | Generated | `packages/core/src/graph/` |
| `visual-encoding.json` | Generated | `apps/studio/src/design/` |
| `visual_encoding.rs` | Generated | `tools/novanet/src/generated/` |
| `useVisualEncoding.ts` | Hook | `apps/studio/src/hooks/` |
| `theme.rs` | TUI | `tools/novanet/src/tui/` |

---

## Version

This design targets **NovaNet v9.5.0**.
