# NovaNet Terminology (v10.1)

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
| 1 | WHERE? | `NodeRealm` | `realm` | `global`, `project`, `shared` |
| 2 | WHAT? | `NodeLayer` | `layer` | `config`, `knowledge`, `foundation`, `structure`, `semantic`, `instruction`, `output`, `seo` |
| 3 | HOW? | `NodeTrait` | `trait` | `invariant`, `localized`, `knowledge`, `derived`, `job` |

### Arc Classification (Faceted)

| Axis | Question | Type | Property | Values |
|------|----------|------|----------|--------|
| 1 | SCOPE? | `ArcScope` | `scope` | `intra_realm`, `cross_realm` |
| 2 | FUNCTION? | `ArcFamily` | `family` | `ownership`, `localization`, `semantic`, `generation`, `mining` |
| 3 | MULTIPLICITY? | `ArcCardinality` | `cardinality` | `one_to_one`, `one_to_many`, `many_to_many` |

## YAML Source Files

| v9.0 (current) | v9.5 (target) | Content |
|----------------|---------------|---------|
| `organizing-principles.yaml` | `taxonomy.yaml` | Realm/Layer/Trait/ArcFamily/ArcScope definitions |
| `nodes/` | `node-kinds/` | 1 file per NodeKind, organized by Realm/Layer |
| `relations.yaml` | `arc-kinds/` | 1 file per ArcKind, organized by ArcFamily |

## File Naming

| Type | Convention | Example |
|------|------------|---------|
| NodeKind YAML | `kebab-case.yaml` | `locale-voice.yaml`, `concept-l10n.yaml` |
| ArcKind YAML | `kebab-case.yaml` | `has-page.yaml`, `uses-concept.yaml` |
| TypeScript types | `PascalCase` | `NodeKind`, `ArcFamily`, `NodeRealm` |
| TypeScript files | `kebab-case.ts` | `arc-kinds.ts`, `node-layers.ts` |
| Rust structs | `PascalCase` | `ArcKind`, `NodeRealm` |
| Rust files | `snake_case.rs` | `arc_schema.rs`, `taxonomy.rs` |

## Node Naming Convention (v10.1)

> **RULE: `*L10n` suffix = has a parent invariant node**

| Pattern | When to Use | Example |
|---------|-------------|---------|
| `FooL10n` | Node has a parent invariant `Foo` with the same key | `PageL10n` (parent: `Page`) |
| `Foo` | Node is standalone (no parent invariant) | `SEOKeyword`, `Term`, `Expression` |

**Examples:**

```
✅ Page (invariant) → PageL10n (localized)     # Correct: L10n has parent
✅ Concept (invariant) → ConceptL10n (localized) # Correct: L10n has parent
✅ SEOKeyword (localized, no parent)           # Correct: no L10n suffix
✅ Term (knowledge atom, no parent)            # Correct: no L10n suffix

❌ SEOKeywordL10n (no parent invariant)        # Wrong: no parent = no L10n suffix
```

**Rationale:** The `L10n` suffix signals a paired relationship. If `FooL10n` exists, developers expect to find a `Foo` invariant. This prevents confusion when exploring the schema.

## Property Naming

Properties use `snake_case` in YAML and TypeScript:

```yaml
# YAML
node:
  name: LocaleVoice
  realm: global
  layer: knowledge
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

## Commands

Use Arc terminology in commands:

```bash
# Correct (v9.5)
novanet arc create --from=page1 --to=concept1 --kind=USES_CONCEPT
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
