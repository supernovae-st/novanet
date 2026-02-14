# Schema Graph

The schema graph is NovaNet's self-describing schema layer. It enables the graph to describe its own structure.

> **v0.12.0 ADR-023**: "Meta" eliminated from terminology. Neo4j labels use `:Schema:` prefix instead of `:Meta:`.
> "Kind" renamed to "Class" (NodeKind→NodeClass, ArcKind→ArcClass).

## Overview

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {
  'primaryColor': '#6366f1',
  'lineColor': '#64748b'
}}}%%
flowchart TB
    subgraph SCHEMA["Schema Layer (:Schema label)"]
        REALM["Realm\n2 nodes"]
        LAYER["Layer\n10 nodes"]
        TRAIT["Trait\n5 nodes"]
        CLASS["Class\n61 nodes"]
        ARCFAM["ArcFamily\n5 nodes"]
        ARCCLASS["ArcClass\n156 nodes"]
    end

    REALM -->|HAS_LAYER| LAYER
    LAYER -->|HAS_CLASS| CLASS
    CLASS -->|HAS_TRAIT| TRAIT
    ARCFAM -->|HAS_ARC_CLASS| ARCCLASS

    style REALM fill:#2aa198,color:#fff
    style LAYER fill:#268bd2,color:#fff
    style TRAIT fill:#6c71c4,color:#fff
    style CLASS fill:#d33682,color:#fff
    style ARCFAM fill:#cb4b16,color:#fff
    style ARCCLASS fill:#859900,color:#fff
```

## Schema Node Types

All schema nodes carry the `:Schema` label in addition to their specific label.

### Realm

**Labels**: `:Schema:Realm`

Represents the top-level organizational scope.

| Property | Type | Description |
|----------|------|-------------|
| `key` | string | Unique identifier (`shared`, `org`) |
| `display_name` | string | Human-readable name |
| `color` | string | Hex color code |

### Layer

**Labels**: `:Schema:Layer`

Represents functional categories within a realm.

| Property | Type | Description |
|----------|------|-------------|
| `key` | string | Unique identifier |
| `realm` | string | Parent realm |
| `display_name` | string | Human-readable name |
| `color` | string | Hex color code |

**Shared layers** (4): config, locale, geography, knowledge
**Org layers** (6): config, foundation, structure, semantic, instruction, output

### Trait

**Labels**: `:Schema:Trait`

Represents data origin patterns (ADR-024: "WHERE does data come from?").

| Property | Type | Description |
|----------|------|-------------|
| `key` | string | Unique identifier |
| `display_name` | string | Human-readable name |
| `border_style` | string | CSS border style |

**Values (v0.12.0 ADR-024)**:
- `defined` — Human-created once (templates, configs)
- `authored` — Human-written per locale (editorial content)
- `imported` — External data brought in (corpora, APIs)
- `generated` — Produced by NovaNet LLM
- `retrieved` — Fetched from external APIs (snapshots)

### Class

**Labels**: `:Schema:Class`

Represents a node type definition.

| Property | Type | Description |
|----------|------|-------------|
| `name` | string | PascalCase type name |
| `realm` | string | Parent realm |
| `layer` | string | Parent layer |
| `trait` | string | Data origin |
| `display_name` | string | Human-readable name |
| `description` | string | Purpose description |
| `llm_context` | string | Context for AI generation |

### ArcFamily

**Labels**: `:Schema:ArcFamily`

Represents arc functional categories.

| Property | Type | Description |
|----------|------|-------------|
| `key` | string | Unique identifier |
| `display_name` | string | Human-readable name |
| `color` | string | Hex color code |

**Values**: ownership, localization, semantic, generation, mining

### ArcClass

**Labels**: `:Schema:ArcClass`

Represents an arc type definition.

| Property | Type | Description |
|----------|------|-------------|
| `name` | string | UPPER_SNAKE_CASE type name |
| `family` | string | Parent family |
| `scope` | string | intra_realm or cross_realm |
| `cardinality` | string | 1:1, 1:N, N:M |
| `source` | string | Source Class name |
| `target` | string | Target Class name |

## Classification Axes

### Node Classification

| Axis | Question | Property | Values |
|------|----------|----------|--------|
| WHERE? | Scope | `realm` | shared, org |
| WHAT? | Function | `layer` | 10 layers |
| HOW? | Origin | `trait` | 5 traits |

### Arc Classification

| Axis | Question | Property | Values |
|------|----------|----------|--------|
| SCOPE? | Realm crossing | `scope` | intra_realm, cross_realm |
| FUNCTION? | Purpose | `family` | 5 families |
| MULT? | Cardinality | `cardinality` | 1:1, 1:N, N:M |

## Visual Encoding

The schema graph drives visual encoding in both Studio and TUI:

| Visual Channel | Encodes | Source |
|----------------|---------|--------|
| Fill color | Layer | `taxonomy.yaml` |
| Border style | Trait | `visual-encoding.yaml` |
| Border color | Realm | `taxonomy.yaml` |
| Arc stroke | ArcFamily | `taxonomy.yaml` |
| Arc dash | ArcScope | solid/dashed |

### Trait Border Styles

| Trait | Border Style |
|-------|--------------|
| defined | solid |
| authored | dashed |
| imported | double |
| generated | dotted |
| retrieved | dotted thin |

## Querying the Schema Graph

### List All Classes

```cypher
MATCH (c:Schema:Class)
RETURN c.name, c.realm, c.layer, c.trait
ORDER BY c.realm, c.layer, c.name
```

### List All Arcs

```cypher
MATCH (a:Schema:ArcClass)
RETURN a.name, a.family, a.scope, a.source, a.target
ORDER BY a.family, a.name
```

### Classes by Layer

```cypher
MATCH (l:Schema:Layer {key: $layer})-[:HAS_CLASS]->(c:Schema:Class)
RETURN c.name, c.trait
```

### Arcs by Family

```cypher
MATCH (f:Schema:ArcFamily {key: $family})-[:HAS_ARC_CLASS]->(a:Schema:ArcClass)
RETURN a.name, a.source, a.target
```

## Generation from YAML

The schema graph is generated from YAML sources:

```
packages/core/models/
├── taxonomy.yaml            → Realms, Layers, Traits, Families
├── node-kinds/**/*.yaml     → Class nodes
└── arc-kinds/**/*.yaml      → ArcClass nodes
```

Regenerate with:

```bash
cd tools/novanet
cargo run -- schema generate
cargo run -- db seed
```

## Statistics (v0.12.4)

| Schema Type | Count |
|-------------|-------|
| Realm | 2 |
| Layer | 10 |
| Trait | 5 |
| Class | 61 |
| ArcFamily | 5 |
| ArcClass | 156 |
| **Total Schema Nodes** | 211 |

## Related Documentation

- [Architecture Overview](./overview.md) — System architecture
- [Ontology Evolution](./ontology-v9.md) — Version history
- [Rust CLI](./rust-cli.md) — Command reference
