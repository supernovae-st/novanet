# NovaNet Terminology (v0.13.0)

This file defines the **canonical terminology** for NovaNet. All code, documentation, and UI must use these terms consistently.

> **Purpose**: Quick lookup reference for current terminology. For rationale and history, see individual ADR files in `.claude/rules/adr/`.

---

## Core Vocabulary

### Graph Elements

| Level | Vertex | Edge |
|-------|--------|------|
| **General** | Node | **Arc** |
| **Instance (data)** | NodeData | ArcData |
| **Class (schema)** | NodeClass | ArcClass |

> **Arc terminology**: See [ADR-001](adr/core-principles/adr-001-arc-terminology.md).
> Exception: React Flow uses "Edge" internally.

### Node Classification (3 Axes)

| Axis | Question | Type | Property | Values |
|------|----------|------|----------|--------|
| 1 | WHERE? | `NodeRealm` | `realm` | `shared`, `org` |
| 2 | WHAT? | `NodeLayer` | `layer` | `config`, `locale`, `geography`, `knowledge`, `foundation`, `structure`, `semantic`, `instruction`, `output` |
| 3 | HOW? | `NodeTrait` | `trait` | `defined`, `authored`, `imported`, `generated`, `retrieved` |

> **Trait as Data Origin**: See [ADR-024](adr/node-classification/adr-024-trait-data-origin.md).

### Current Architecture (v0.13.0)

| Realm | Layers | Nodes | Description |
|-------|--------|-------|-------------|
| `shared` | config, locale, geography, knowledge | 40 | Universal knowledge (READ-ONLY) |
| `org` | config, foundation, structure, semantic, instruction, output | 21 | Organization-specific content |

**Totals**: 61 nodes, 169 arcs, 10 layers (4 shared + 6 org), 5 traits

> **Architecture details**: See [ADR-012](adr/schema-architecture/adr-012-two-realm.md), [ADR-028](adr/schema-architecture/adr-028-page-entity.md).

### Arc Classification (3 Axes)

| Axis | Question | Type | Property | Values |
|------|----------|------|----------|--------|
| 1 | SCOPE? | `ArcScope` | `scope` | `intra_realm`, `cross_realm` |
| 2 | FUNCTION? | `ArcFamily` | `family` | `ownership`, `localization`, `semantic`, `generation`, `mining` |
| 3 | MULTIPLICITY? | `ArcCardinality` | `cardinality` | `zero_to_one`, `one_to_one`, `one_to_many`, `many_to_many` |

> **Arc design**: See [ADR-026](adr/arc-design/adr-026-inverse-arc-policy.md), [ADR-027](adr/arc-design/adr-027-generation-family.md).

---

## YAML Source Files

| File | Content |
|------|---------|
| `taxonomy.yaml` | Realm/Layer/Trait/ArcFamily/ArcScope definitions |
| `node-classes/shared/` | 40 NodeClass definitions (config: 3, locale: 6, geography: 7, knowledge: 24) |
| `node-classes/org/` | 21 NodeClass definitions (config: 1, foundation: 6, structure: 3, semantic: 4, instruction: 4, output: 3) |
| `arc-classes/` | 1 file per ArcClass, organized by ArcFamily |
| `visual-encoding.yaml` | Icons and visual encoding rules |

> **YAML-first architecture**: See [ADR-003](adr/core-principles/adr-003-yaml-first.md).

## File Naming

| Type | Convention | Example |
|------|------------|---------|
| NodeClass YAML | `kebab-case.yaml` | `locale-voice.yaml`, `entity-native.yaml`, `page-native.yaml` |
| ArcClass YAML | `kebab-case.yaml` | `has-page.yaml`, `has-native.yaml`, `uses-entity.yaml` |
| TypeScript types | `PascalCase` | `NodeClass`, `ArcFamily`, `NodeRealm` |
| TypeScript files | `kebab-case.ts` | `arc-classes.ts`, `node-layers.ts` |
| Rust structs | `PascalCase` | `ArcClass`, `NodeRealm` |
| Rust files | `snake_case.rs` | `arc_schema.rs`, `taxonomy.rs` |

## Node Naming Convention

| Pattern | Trait | When to Use | Example |
|---------|-------|-------------|---------|
| `FooNative` | authored/generated | Locale-specific content | `EntityNative`, `PageNative` |
| `FooCategory` | defined | Categorical grouping | `EntityCategory` |
| `FooSet` | defined | Container grouping atoms | `TermSet`, `SEOKeywordSet` |
| `Foo` | varies | Standalone node | `SEOKeyword`, `Term` |

> **\*Native Pattern**: See [ADR-029](adr/schema-architecture/adr-029-native-pattern.md).
> **Slug Ownership**: See [ADR-030](adr/schema-architecture/adr-030-slug-ownership.md).

### Current \*Native Nodes

| Node | Layer | Trait | Description |
|------|-------|-------|-------------|
| `EntityNative` | semantic | authored | Human-written entity content |
| `ProjectNative` | foundation | authored | Human-written project content |
| `PageNative` | output | generated | LLM-generated page content |
| `BlockNative` | output | generated | LLM-generated block content |

### Key Arcs

| Arc | Direction | Purpose |
|-----|-----------|---------|
| `HAS_NATIVE` | Parent → Native | Unified arc with `locale` property |
| `NATIVE_OF` | Native → Parent | Inverse |

## Property Naming

Properties use `snake_case` in YAML and TypeScript:

```yaml
node:
  name: LocaleVoice
  realm: shared
  layer: knowledge
  trait: imported
  display_name: "Locale Voice"
```

```typescript
interface NodeClass {
  name: string;
  realm: NodeRealm;
  layer: NodeLayer;
  trait: NodeTrait;
  display_name: string;
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
| `RelationType` | Keep | Neo4j rel type string (e.g., `"HAS_PAGE"`) — ArcClass is the schema-node |
| `Scope` (for realm) | `Realm` | v9.0 renamed |
| `Subcategory` | `Layer` | v9.0 renamed |
| `NodeTypeMeta` | `Class` | v9.0 renamed (Kind→Class in v0.12.0) |
| `DataMode` | `NavigationMode` | v9.0 renamed |
| `category` | `trait` | YAML property |
| `global` | `shared` | v11.2 realm rename |
| `tenant` | `org` | v11.2 realm rename |
| `derived` | `generated` / `aggregated` | v11.2 trait split |
| `job` | (removed) | v11.2 trait removed |
| `EntityL10n` | `EntityNative` | v10.9 renamed (semantic layer) |
| `PageL10n` | `PageNative` | v10.9 renamed (output layer) |
| `BlockL10n` | `BlockNative` | v10.9 renamed (output layer) |
| `ProjectL10n` | `ProjectNative` | v11.0 renamed (foundation layer) |
| `HAS_L10N` | `HAS_NATIVE` | v10.9 renamed (Entity → EntityNative) |
| `HAS_OUTPUT` | `HAS_NATIVE` | v10.9 renamed (Page/Block → *Generated) |
| `BELONGS_TO_PROJECT_L10N` | `BELONGS_TO_PROJECT_CONTENT` | v11.0 renamed |
| `GenerationJob` | (removed) | v11.2 job nodes removed |
| `SEOMiningRun` | (removed) | v11.2 job nodes removed |
| `EvaluationSignal` | (removed) | v11.2 job nodes removed |
| `GEOSeedL10n` | `GEOQuery` | v10.7 new GEO schema |
| `GEOSeedMetrics` | `GEOMetrics` | v10.7 new GEO schema |
| `locale-knowledge` | `locale` / `geography` / `knowledge` | v11.3 layer split |
| `Organization` | `OrgConfig` | v11.3 node merge |
| `Tenant` | `OrgConfig` | v11.3 node merge |
| `org/seo` layer | `shared/knowledge` | v11.5 SEO nodes consolidated |
| `org/geo` layer | `shared/knowledge` | v11.5 GEO nodes consolidated |
| `Locale` in `shared/locale` | `Locale` in `shared/config` | v11.5 definitions layer pattern |
| `data` mode | `graph` | v11.7 unified tree |
| `meta` mode | `graph` | v11.7 unified tree |
| `overlay` mode | `graph` | v11.7 unified tree |
| `query` mode | `graph` + filters | v11.7 unified tree |
| `atlas` mode | `nexus` | v11.7 renamed |
| emoji icons | dual format `{ web, terminal }` | v11.7 icon system |
| **v0.12.0 Kind→Class + Meta elimination** | | |
| `NodeKind` | `NodeClass` | v0.12.0 terminology |
| `ArcKind` | `ArcClass` | v0.12.0 terminology |
| `KindInfo` | `ClassInfo` | v0.12.0 TUI struct |
| `KindMeta` | `Classification` | v0.12.0 (realm/layer/trait axes) |
| `KIND_META` | `CLASS_TAXONOMY` | v0.12.0 TypeScript constant |
| `:Meta:Kind` | `:Schema:Class` | v0.12.0 Neo4j label |
| `:Meta:ArcKind` | `:Schema:ArcClass` | v0.12.0 Neo4j label |
| `[:FROM_KIND]` | `[:FROM_CLASS]` | v0.12.0 Neo4j relationship |
| `[:TO_KIND]` | `[:TO_CLASS]` | v0.12.0 Neo4j relationship |
| `[:HAS_KIND]` | `[:HAS_CLASS]` | v0.12.0 Neo4j relationship |
| "Meta Node" | "Class" | v0.12.0 glossary |
| "Data Node" | "Instance" | v0.12.0 glossary |
| "Meta mode" | "Schema view" | v0.12.0 Studio UI |
| "Data mode" | "Graph view" | v0.12.0 Studio UI |
| **v0.12.0 Trait Redefinition (ADR-024)** | | |
| `invariant` | `defined` | v0.12.0 trait rename (human-created once) |
| `localized` | `authored` | v0.12.0 trait rename (human-written per locale) |
| `knowledge` (trait) | `imported` | v0.12.0 trait rename (external data brought in) |
| `aggregated` | `retrieved` | v0.12.0 trait rename (fetched from external APIs) |
| **v0.12.0 Instruction Layer (ADR-025)** | | |
| `PageType` | `PageStructure` | v0.12.0 (JSON defining block order) |
| `PagePrompt` | `PageInstruction` | v0.12.0 (Markdown with @ refs) |
| `BlockPrompt` | `BlockInstruction` | v0.12.0 (Markdown with @ refs) |
| `[:OF_TYPE]` (Page→PageType) | `[:HAS_STRUCTURE]` | v0.12.0 arc rename |
| `[:HAS_PROMPT]` (Page→PagePrompt) | `[:HAS_INSTRUCTION]` | v0.12.0 arc rename |
| `[:HAS_PROMPT]` (Block→BlockPrompt) | `[:HAS_INSTRUCTION]` | v0.12.0 arc rename |
| **v0.13.0 *Native Pattern (ADR-029)** | | |
| `EntityContent` | `EntityNative` | v0.13.0 *Native pattern (semantic layer, authored) |
| `PageGenerated` | `PageNative` | v0.13.0 *Native pattern (output layer, generated) |
| `BlockGenerated` | `BlockNative` | v0.13.0 *Native pattern (output layer, generated) |
| `ProjectContent` | `ProjectNative` | v0.13.0 *Native pattern (foundation layer, authored) |
| `HAS_CONTENT` | `HAS_NATIVE` | v0.13.0 unified arc (replaces HAS_CONTENT + HAS_GENERATED) |
| `HAS_GENERATED` | `HAS_NATIVE` | v0.13.0 unified arc (with `locale` property) |
| `CONTENT_OF` | `NATIVE_OF` | v0.13.0 inverse arc rename |
| `GENERATED_FOR` | `NATIVE_OF` | v0.13.0 inverse arc rename |
| **v0.13.0 Slug Ownership (ADR-030)** | | |
| `EntityNative.slug` | `PageNative.slug` | v0.13.0 URL segment moved to Page layer |
| `EntityNative.full_path` | `PageNative.full_path` | v0.13.0 full URL path moved to Page layer |
| `EntityNative.parent_slug` | (removed) | v0.13.0 calculated from Page.SUBTOPIC_OF |
| `EntityNative.depth` | (removed) | v0.13.0 calculated from Page hierarchy |

---

## Navigation Modes

| Mode | Content | Use Case |
|------|---------|----------|
| `graph` | Unified tree (Realm > Layer > Class > Instance + Arcs) | Default exploration |
| `nexus` | Hub (Quiz, Audit, Stats, Help) | Learning & validation |

> **Unified Tree Architecture**: See [ADR-022](adr/ux-architecture/adr-022-unified-tree.md).

---

## Visual Encoding

| Visual Channel | Encodes | Source |
|----------------|---------|--------|
| Fill color | Layer | `taxonomy.yaml` |
| Border style | Trait | `visual-encoding.yaml` |
| Border color | Realm | `taxonomy.yaml` |
| Arc stroke | ArcFamily | `taxonomy.yaml` |
| Arc dash | ArcScope | solid (intra) / dashed (cross) |

> **Visual encoding details**: See [ADR-005](adr/visual-encoding/adr-005-trait-visual-encoding.md).

---

## Icons

Source of truth: `packages/core/models/visual-encoding.yaml` → `icons:` section

> **Icons architecture**: See [ADR-013](adr/visual-encoding/adr-013-icons-source.md).

### Dual Format

| Context | Format | Example |
|---------|--------|---------|
| Studio (web) | Lucide name | `icon.web: "globe"` |
| TUI (terminal) | Unicode | `icon.terminal: "◉"` |

> **Rule**: NO emoji in code. Use `{ web, terminal }` format.

### Icon Categories

| Category | Purpose | Examples |
|----------|---------|----------|
| `realms` | Node ownership | ◉ shared, ◎ org |
| `layers` | Functional layer | ⚙ config, ◆ geography, ■ semantic |
| `traits` | Data origin | ■ defined, □ authored, ✦ generated |
| `arc_families` | Arc type | → ownership, ⇢ localization |
| `states` | UI states | ◐ loading, ⚠ error |
| `navigation` | Tree controls | ▼ expanded, ▶ collapsed |

---

## Query-First Architecture

Cypher queries are the single source of truth for graph visualization.

> **Full architecture**: See [ADR-021](adr/core-principles/adr-021-query-first.md).

### Key Terms

| Term | Definition |
|------|------------|
| **Schema-Graph** | NodeClass + ArcClass nodes (61 nodes, 169 arcs) |
| **CLASS_QUERY** | Query fetching all NodeClass instances |
| **ARCS_QUERY** | Query fetching all ArcClass instances |
| **View** | Parameterized Cypher template in YAML |

### View Categories

| Category | Purpose |
|----------|---------|
| `global` | Full graph exploration |
| `contextual` | Node-specific subgraph |
| `generation` | AI agent context |
| `mining` | SEO/GEO intelligence |

---

## Quick Reference

| Concept | Current Term | ADR |
|---------|--------------|-----|
| Directed link | **Arc** | [001](adr/core-principles/adr-001-arc-terminology.md) |
| Node type definition | **NodeClass** | [023](adr/node-classification/adr-023-class-instance.md) |
| Arc type definition | **ArcClass** | [023](adr/node-classification/adr-023-class-instance.md) |
| Data origin | **Trait** (defined/authored/imported/generated/retrieved) | [024](adr/node-classification/adr-024-trait-data-origin.md) |
| Locale-specific nodes | **\*Native** suffix | [029](adr/schema-architecture/adr-029-native-pattern.md) |
| URL ownership | **Page** owns slug, **Entity** owns semantics | [030](adr/schema-architecture/adr-030-slug-ownership.md) |
| Graph display | **Query-First** (Cypher = source of truth) | [021](adr/core-principles/adr-021-query-first.md) |
| TUI navigation | **Graph/Nexus** modes | [022](adr/ux-architecture/adr-022-unified-tree.md) |
| Icons | **Dual format** `{ web, terminal }` | [013](adr/visual-encoding/adr-013-icons-source.md) |
