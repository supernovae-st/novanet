# NovaNet Terminology (v11.2)

This file defines the canonical terminology for NovaNet. All code, documentation, and UI must use these terms consistently.

## Core Vocabulary

### Graph Elements

| Level | Vertex | Edge |
|-------|--------|------|
| **General** | Node | **Arc** |
| **Instance (data)** | NodeData | ArcData |
| **Definition (meta)** | NodeKind | ArcKind |

> **CRITICAL**: We use "Arc" (not "Edge" or "Relation") for directed links between nodes.
> This aligns with graph theory terminology for directed graphs.
> Exception: React Flow uses "Edge" internally — that's acceptable in React Flow-specific code.

### Node Classification (Faceted)

| Axis | Question | Type | Property | Values |
|------|----------|------|----------|--------|
| 1 | WHERE? | `NodeRealm` | `realm` | `shared`, `org` |
| 2 | WHAT? | `NodeLayer` | `layer` | `config`, `locale-knowledge`, `seo`, `foundation`, `structure`, `semantic`, `instruction`, `output` |
| 3 | HOW? | `NodeTrait` | `trait` | `invariant`, `localized`, `knowledge`, `generated`, `aggregated` |

### v11.2 Realm Architecture

| Realm | Layers | Nodes | Description |
|-------|--------|-------|-------------|
| `shared` | config, locale-knowledge | 32 | Universal locale knowledge (READ-ONLY) |
| `org` | config, foundation, structure, semantic, instruction, seo, output | 30 | Organization-specific content |

> **v11.2 Changes:**
> - Realm renames: `global` → `shared`, `tenant` → `org`
> - Trait split: `derived` → `generated` + `aggregated`, `job` removed
> - 3 job nodes removed (GenerationJob, SEOMiningRun, EvaluationSignal)
> - Total: 62 nodes (was 65)

> **v11.0 Changes:**
> - SEO moved from global to tenant (ADR-012 fix)
> - SHARED (2 layers): config, locale-knowledge
> - ORG (7 layers): config, foundation, structure, semantic, instruction, seo, output

### Arc Classification (Faceted)

| Axis | Question | Type | Property | Values |
|------|----------|------|----------|--------|
| 1 | SCOPE? | `ArcScope` | `scope` | `intra_realm`, `cross_realm` |
| 2 | FUNCTION? | `ArcFamily` | `family` | `ownership`, `localization`, `semantic`, `generation`, `mining` |
| 3 | MULTIPLICITY? | `ArcCardinality` | `cardinality` | `zero_to_one`, `one_to_one`, `one_to_many`, `many_to_many` |

## YAML Source Files (v11.2)

| File | Content |
|------|---------|
| `taxonomy.yaml` | Realm/Layer/Trait/ArcFamily/ArcScope definitions (v11.2: 2 realms, 9 layers, 5 traits) |
| `node-kinds/shared/` | 32 NodeKind definitions in shared realm |
| `node-kinds/org/` | 30 NodeKind definitions in org realm |
| `arc-kinds/` | 1 file per ArcKind, organized by ArcFamily |
| `relations.yaml` | Legacy format (deprecated, kept for parser compatibility) |

## File Naming

| Type | Convention | Example |
|------|------------|---------|
| NodeKind YAML | `kebab-case.yaml` | `locale-voice.yaml`, `entity-content.yaml`, `page-generated.yaml` |
| ArcKind YAML | `kebab-case.yaml` | `has-page.yaml`, `uses-entity.yaml` |
| TypeScript types | `PascalCase` | `NodeKind`, `ArcFamily`, `NodeRealm` |
| TypeScript files | `kebab-case.ts` | `arc-kinds.ts`, `node-layers.ts` |
| Rust structs | `PascalCase` | `ArcKind`, `NodeRealm` |
| Rust files | `snake_case.rs` | `arc_schema.rs`, `taxonomy.rs` |

## Node Naming Convention (v11.2)

> **RULE: Suffix indicates trait and relationship to parent invariant node**

| Pattern | Trait | Layer | When to Use | Example |
|---------|-------|-------|-------------|---------|
| `FooContent` | localized | semantic | Node has locale-specific content for invariant `Foo` | `EntityContent` (parent: `Entity`) |
| `FooGenerated` | generated | output | Node is generated output from invariant `Foo` | `PageGenerated` (parent: `Page`) |
| `FooCategory` | invariant | config | Categorical grouping for invariant `Foo` | `EntityCategory` |
| `Foo` | varies | varies | Node is standalone (no parent invariant) | `SEOKeyword`, `Term`, `Expression` |

**v11.2 Changes:**
- Trait `derived` split into `generated` (LLM output) and `aggregated` (computed metrics)
- `job` trait removed (3 nodes deleted)
- Realms renamed: `global` → `shared`, `tenant` → `org`

**v11.1 Changes:**
- `EntityCategory` added (shared/config layer, invariant trait, categorical grouping)
- `BELONGS_TO` arc added (Entity → EntityCategory, ownership family)

**v10.9 Changes:**
- `EntityL10n` renamed to `EntityContent` (semantic layer, localized trait)
- `PageL10n` renamed to `PageGenerated` (output layer, generated trait)
- `BlockL10n` renamed to `BlockGenerated` (output layer, generated trait)

**Arc Changes (v10.9):**
- `HAS_L10N` renamed to `HAS_CONTENT` (Entity → EntityContent)
- `HAS_OUTPUT` renamed to `HAS_GENERATED` (Page/Block → PageGenerated/BlockGenerated)

**Examples:**

```
✅ Entity (invariant) → EntityContent (localized)   # Semantic layer content
✅ Entity (invariant) → EntityCategory (invariant)  # shared/config categorization
✅ Page (invariant) → PageGenerated (generated)     # Output layer generated
✅ Block (invariant) → BlockGenerated (generated)   # Output layer generated
✅ Project (invariant) → ProjectContent (localized) # Foundation layer content
✅ SEOKeyword (knowledge, no parent)                # Correct: no suffix
✅ Term (knowledge atom, no parent)                 # Correct: no suffix
✅ SEOKeywordMetrics (aggregated)                   # Computed metrics

❌ EntityL10n (deprecated)                          # Use EntityContent
❌ PageL10n (deprecated)                            # Use PageGenerated
❌ BlockL10n (deprecated)                           # Use BlockGenerated
❌ ProjectL10n (deprecated)                         # Use ProjectContent
❌ derived (deprecated trait)                       # Use generated or aggregated
❌ job (removed trait)                              # Concept deferred to v12+
```

**Rationale:**
- `*Content` suffix indicates locale-specific semantic content (localized trait)
- `*Generated` suffix indicates LLM-generated output (generated trait)
- `*Metrics` suffix indicates computed/aggregated data (aggregated trait)
- `*Category` suffix indicates categorical grouping/taxonomy structure (invariant trait)
- `*L10n` suffix is DEPRECATED - all localized nodes now use `*Content` suffix
- Suffix choice reflects both the trait and the layer

## Property Naming

Properties use `snake_case` in YAML and TypeScript:

```yaml
# YAML
node:
  name: LocaleVoice
  realm: shared             # v11.2: renamed from global
  layer: locale-knowledge   # 2 realms (shared, org)
  trait: knowledge
  display_name: "Locale Voice"
  llm_context: "..."
```

```typescript
// TypeScript
interface NodeKind {
  name: string;
  realm: NodeRealm;
  layer: NodeLayer;
  trait: NodeTrait;
  display_name: string;
  llm_context?: string;
}
```

## Deprecated Terms

These terms are deprecated and should NOT be used:

| Deprecated | Use Instead | Notes |
|------------|-------------|-------|
| `Edge` | `Arc` | Except in React Flow code |
| `EdgeKind` | `ArcKind` | |
| `EdgeFamily` | `ArcFamily` | |
| `Relation` | `Arc` | |
| `RelationType` | Keep | Neo4j rel type string (e.g., `"HAS_PAGE"`) — ArcKind is the meta-node |
| `Scope` (for realm) | `Realm` | v9.0 renamed |
| `Subcategory` | `Layer` | v9.0 renamed |
| `NodeTypeMeta` | `Kind` | v9.0 renamed |
| `DataMode` | `NavigationMode` | v9.0 renamed |
| `category` | `trait` | YAML property |
| `global` | `shared` | v11.2 realm rename |
| `tenant` | `org` | v11.2 realm rename |
| `derived` | `generated` / `aggregated` | v11.2 trait split |
| `job` | (removed) | v11.2 trait removed |
| `EntityL10n` | `EntityContent` | v10.9 renamed (semantic layer) |
| `PageL10n` | `PageGenerated` | v10.9 renamed (output layer) |
| `BlockL10n` | `BlockGenerated` | v10.9 renamed (output layer) |
| `ProjectL10n` | `ProjectContent` | v11.0 renamed (foundation layer) |
| `HAS_L10N` | `HAS_CONTENT` | v10.9 renamed (Entity → EntityContent) |
| `HAS_OUTPUT` | `HAS_GENERATED` | v10.9 renamed (Page/Block → *Generated) |
| `BELONGS_TO_PROJECT_L10N` | `BELONGS_TO_PROJECT_CONTENT` | v11.0 renamed |
| `GenerationJob` | (removed) | v11.2 job nodes removed |
| `SEOMiningRun` | (removed) | v11.2 job nodes removed |
| `EvaluationSignal` | (removed) | v11.2 job nodes removed |
| `GEOSeedL10n` | `GEOQuery` | v10.7 new GEO schema |
| `GEOSeedMetrics` | `GEOMetrics` | v10.7 new GEO schema |

## Navigation Modes

| Mode | Content | Use Case |
|------|---------|----------|
| `data` | Real instances only | Default exploration |
| `meta` | Meta-graph only | Schema understanding |
| `overlay` | Data + meta combined | Architecture debugging |
| `query` | Faceted filter results | Targeted exploration |

## Visual Encoding

| Visual Channel | Encodes | Source |
|----------------|---------|--------|
| Fill color | Layer | `taxonomy.yaml` node_layers[].color |
| Border style | Trait | `visual-encoding.yaml` trait_borders |
| Border color | Realm | `taxonomy.yaml` node_realms[].color |
| Arc stroke | ArcFamily | `taxonomy.yaml` arc_families[].color |
| Arc dash | ArcScope | solid (intra) / dashed (cross) |

## Icons (v11.2)

Source of truth: `packages/core/models/visual-encoding.yaml` → `icons:` section

Each icon has dual format:
- `web`: Lucide icon name for Studio/web
- `terminal`: Unicode symbol for TUI

| Category | Purpose | Examples |
|----------|---------|----------|
| `realms` | Node ownership | ◉ shared, ◎ org |
| `layers` | Functional layer | ⚙ config, ◆ semantic, ● output |
| `traits` | Locale behavior | ■ invariant, □ localized, ◊ knowledge, ✦ generated, ⋆ aggregated |
| `arc_families` | Arc type | → ownership, ⇢ localization |
| `states` | UI empty states | ◐ loading, ∅ no_kinds, ⚠ error |
| `navigation` | Tree controls | ▼ expanded, ▶ collapsed |
| `quality` | Data completeness | ● complete, ◐ partial, * required |
| `modes` | Nav modes | M meta, D data, A atlas |

**TUI loading**: `Theme::with_root()` loads icons from YAML at startup.

**Fallback**: Default icons used if YAML loading fails (graceful degradation).

## Commands

Use Arc terminology in commands:

```bash
# Correct (v9.5)
novanet arc create --from=page1 --to=entity1 --kind=USES_ENTITY
novanet arc delete --id=abc123

# Deprecated (v9.0)
novanet relation create ...  # Still works, but deprecated
```

## Summary

1. **Arc** = directed link (not Edge, not Relation)
2. **NodeKind** = node type definition
3. **ArcKind** = arc type definition
4. **Realm/Layer/Trait** = node classification axes
5. **ArcFamily/ArcScope/ArcCardinality** = arc classification axes
6. **taxonomy.yaml** = source of truth for facet definitions
