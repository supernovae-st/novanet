# NovaNet Terminology (v10.9)

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
| 1 | WHERE? | `NodeRealm` | `realm` | `global`, `tenant` |
| 2 | WHAT? | `NodeLayer` | `layer` | `config`, `locale-knowledge`, `seo`, `foundation`, `structure`, `semantic`, `instruction`, `output` |
| 3 | HOW? | `NodeTrait` | `trait` | `invariant`, `localized`, `knowledge`, `derived`, `job` |

### v10.6 Realm Architecture

| Realm | Layers | Description |
|-------|--------|-------------|
| `global` | config, locale-knowledge, seo | Universal locale knowledge (READ-ONLY) |
| `tenant` | config, foundation, structure, semantic, instruction, output | Business-specific content |

> **v10.6 Changes:**
> - 3 realms -> 2 realms: GLOBAL + TENANT (merged organization + project)
> - GLOBAL (3 layers): config, locale-knowledge, seo
> - TENANT (6 layers): config, foundation, structure, semantic, instruction, output

### Arc Classification (Faceted)

| Axis | Question | Type | Property | Values |
|------|----------|------|----------|--------|
| 1 | SCOPE? | `ArcScope` | `scope` | `intra_realm`, `cross_realm` |
| 2 | FUNCTION? | `ArcFamily` | `family` | `ownership`, `localization`, `semantic`, `generation`, `mining` |
| 3 | MULTIPLICITY? | `ArcCardinality` | `cardinality` | `zero_to_one`, `one_to_one`, `one_to_many`, `many_to_many` |

## YAML Source Files (v10.6)

| File | Content |
|------|---------|
| `taxonomy.yaml` | Realm/Layer/Trait/ArcFamily/ArcScope definitions (v10.6: 2 realms) |
| `node-kinds/` | 1 file per NodeKind, organized by Realm/Layer |
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

## Node Naming Convention (v10.9)

> **RULE: Suffix indicates trait and relationship to parent invariant node**

| Pattern | Trait | Layer | When to Use | Example |
|---------|-------|-------|-------------|---------|
| `FooContent` | localized | semantic | Node has locale-specific content for invariant `Foo` | `EntityContent` (parent: `Entity`) |
| `FooGenerated` | derived | output | Node is generated output from invariant `Foo` | `PageGenerated` (parent: `Page`) |
| `FooL10n` | localized | foundation | Node has locale-specific settings for invariant `Foo` | `ProjectL10n` (parent: `Project`) |
| `Foo` | varies | varies | Node is standalone (no parent invariant) | `SEOKeyword`, `Term`, `Expression` |

**v10.9 Changes:**
- `EntityL10n` renamed to `EntityContent` (semantic layer, localized trait)
- `PageL10n` renamed to `PageGenerated` (output layer, derived trait)
- `BlockL10n` renamed to `BlockGenerated` (output layer, derived trait)
- `ProjectL10n` unchanged (foundation layer, localized trait)

**Arc Changes (v10.9):**
- `HAS_L10N` renamed to `HAS_CONTENT` (Entity → EntityContent)
- `HAS_OUTPUT` renamed to `HAS_GENERATED` (Page/Block → PageGenerated/BlockGenerated)

**Examples:**

```
✅ Entity (invariant) → EntityContent (localized)   # Semantic layer content
✅ Page (invariant) → PageGenerated (derived)       # Output layer generated
✅ Block (invariant) → BlockGenerated (derived)     # Output layer generated
✅ Project (invariant) → ProjectL10n (localized)    # Foundation layer settings
✅ SEOKeyword (localized, no parent)                # Correct: no suffix
✅ Term (knowledge atom, no parent)                 # Correct: no suffix

❌ EntityL10n (deprecated)                          # Use EntityContent
❌ PageL10n (deprecated)                            # Use PageGenerated
❌ BlockL10n (deprecated)                           # Use BlockGenerated
```

**Rationale:**
- `*Content` suffix indicates locale-specific semantic content (localized trait)
- `*Generated` suffix indicates derived output from generation pipeline (derived trait)
- `*L10n` suffix reserved for foundation-layer locale settings only
- Suffix choice reflects both the trait (localized vs derived) and the layer (semantic vs output vs foundation)

## Property Naming

Properties use `snake_case` in YAML and TypeScript:

```yaml
# YAML
node:
  name: LocaleVoice
  realm: global
  layer: locale-knowledge  # v10.6: 2 realms (global, tenant)
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
| `EntityL10n` | `EntityContent` | v10.9 renamed (semantic layer) |
| `PageL10n` | `PageGenerated` | v10.9 renamed (output layer) |
| `BlockL10n` | `BlockGenerated` | v10.9 renamed (output layer) |
| `HAS_L10N` | `HAS_CONTENT` | v10.9 renamed (Entity → EntityContent) |
| `HAS_OUTPUT` | `HAS_GENERATED` | v10.9 renamed (Page/Block → *Generated) |

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

## Icons (v10.6)

Source of truth: `packages/core/models/visual-encoding.yaml` → `icons:` section

Each icon has dual format:
- `web`: Lucide icon name for Studio/web
- `terminal`: Unicode symbol for TUI

| Category | Purpose | Examples |
|----------|---------|----------|
| `realms` | Node ownership | ◉ global, ◎ tenant |
| `layers` | Functional layer | ⚙ config, ◆ semantic, ● output |
| `traits` | Locale behavior | ■ invariant, □ localized, ◊ knowledge |
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
