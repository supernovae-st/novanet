# Schema Management

NovaNet uses a YAML-first architecture where YAML files are the single source of truth. This guide covers schema modification workflows.

## Overview

```
YAML (source) → Rust CLI → Generated artifacts
                            ├── TypeScript types
                            ├── Cypher seeds
                            ├── Mermaid diagrams
                            └── Rust structs
```

## Directory Structure

```
packages/core/models/
├── _index.yaml               # Schema registry
├── taxonomy.yaml             # Realms, Layers, Traits, Colors
├── visual-encoding.yaml      # Icons, border styles
├── node-kinds/
│   ├── shared/               # 39 nodes
│   │   ├── config/           # Definitions (Locale, EntityCategory)
│   │   ├── locale/           # Locale settings
│   │   ├── geography/        # Geographic data
│   │   └── knowledge/        # Knowledge atoms, SEO, GEO
│   └── org/                  # 21 nodes
│       ├── config/           # OrgConfig
│       ├── foundation/       # Project, Brand
│       ├── structure/        # Page, Block
│       ├── semantic/         # Entity, Content
│       ├── instruction/      # Types, Prompts
│       └── output/           # Generated content
└── arc-kinds/                # 114 arcs by family
    ├── ownership/
    ├── localization/
    ├── semantic/
    ├── generation/
    └── mining/
```

## Adding a New Node Type

### Using Claude Code Commands

The recommended way to add nodes is via Socratic discovery:

```bash
/schema:add-node MyNewNode
```

This guides you through:
1. Choosing realm (shared vs org)
2. Selecting layer
3. Defining trait
4. Specifying properties
5. Setting up relationships

### Manual YAML Creation

Create a YAML file at the correct path:

```yaml
# packages/core/models/node-kinds/org/semantic/my-node.yaml
node:
  name: MyNode
  realm: org                    # Must match path
  layer: semantic               # Must match path
  trait: invariant
  display_name: "My Node"
  description: "Purpose of this node type"
  llm_context: "Context for AI generation"
  properties:
    key:
      type: string
      required: true
      description: "Unique identifier"
    title:
      type: string
      required: true
    metadata:
      type: object
      required: false
```

### Generate Artifacts

After creating/modifying YAML:

```bash
cd tools/novanet

# Generate all artifacts
cargo run -- schema generate

# Or preview first
cargo run -- schema generate --dry-run

# Validate coherence
cargo run -- schema validate
```

### Seed Neo4j

```bash
cargo run -- db seed
```

## Modifying an Existing Node

### Using Claude Code Commands

```bash
/schema:edit-node EntityContent
```

### Manual Edit

1. Find the YAML file:
   ```bash
   fd entity-content models/node-kinds/
   ```

2. Edit properties or metadata

3. Regenerate and reseed:
   ```bash
   cargo run -- schema generate && cargo run -- db seed
   ```

## Adding a New Arc Type

### Using Claude Code Commands

```bash
/schema:add-arc HAS_NEW_THING
```

### Manual YAML Creation

```yaml
# packages/core/models/arc-kinds/ownership/has-new-thing.yaml
arc:
  name: HAS_NEW_THING
  family: ownership
  scope: intra_realm
  cardinality: one_to_many
  source: Parent
  target: NewThing
  description: "Parent owns NewThing instances"
```

## Validation

### Schema Validation

```bash
cargo run -- schema validate
```

Checks:
- YAML syntax
- Required fields
- Path/content consistency (realm, layer)
- Reference integrity (arcs reference existing kinds)

### Strict Mode

```bash
cargo run -- schema validate --strict
```

Fails on warnings (unused definitions, deprecation notices).

## Generated Artifacts

The `schema generate` command produces 12 artifacts:

| Artifact | Location | Purpose |
|----------|----------|---------|
| TypeScript types | `packages/core/src/graph/` | Zod schemas, enums |
| Cypher seeds | `packages/db/seed/` | Neo4j seeding |
| Mermaid docs | `packages/core/models/docs/` | Visual documentation |
| Rust icons | `tools/novanet/src/tui/icons.rs` | TUI icon constants |

### Key Generated Files

```
packages/core/src/graph/
├── node-kinds.generated.ts      # Kind type definitions
├── arc-kinds.generated.ts       # ArcKind type definitions
├── taxonomy.generated.ts        # Realm, Layer, Trait enums
├── visual-encoding.generated.ts # Colors, icons
└── filters.generated.ts         # Facet filter types

packages/db/seed/
├── 00.0-constraints.cypher      # Neo4j constraints
├── 00.5-taxonomy.cypher         # Meta-graph nodes
├── 01-kinds.cypher              # Kind definitions
└── 02-arcs.cypher               # ArcKind definitions
```

## Best Practices

### Naming Conventions

| Element | Convention | Example |
|---------|------------|---------|
| Node name | PascalCase | `EntityContent`, `PageGenerated` |
| Arc name | UPPER_SNAKE_CASE | `HAS_CONTENT`, `USES_ENTITY` |
| YAML file | kebab-case | `entity-content.yaml` |
| Property | snake_case | `display_name`, `llm_context` |

### Node Naming Patterns

| Suffix | Trait | Meaning |
|--------|-------|---------|
| `*Content` | localized | Locale-specific content |
| `*Generated` | generated | LLM output |
| `*Metrics` | aggregated | Computed data |
| `*Category` | invariant | Categorical grouping |
| `*Set` | invariant | Container for atoms |

### Property Types

| Type | Neo4j | TypeScript | Example |
|------|-------|------------|---------|
| `string` | STRING | string | `"hello"` |
| `integer` | INTEGER | number | `42` |
| `boolean` | BOOLEAN | boolean | `true` |
| `float` | FLOAT | number | `3.14` |
| `date` | DATE | Date | `"2026-02-13"` |
| `datetime` | DATETIME | Date | ISO timestamp |
| `array` | LIST | array | `["a", "b"]` |
| `object` | MAP | object | `{key: "value"}` |

## Troubleshooting

### Path/Content Mismatch

```
Error: File at shared/knowledge/term.yaml has realm: org
```

**Fix**: Ensure `realm:` and `layer:` in YAML match the file path.

### Missing Reference

```
Error: Arc HAS_FOO references unknown Kind: Foo
```

**Fix**: Create the referenced Kind first, or fix the arc source/target.

### Regenerate Everything

When in doubt:

```bash
cd tools/novanet
cargo run -- schema generate
cargo run -- schema validate
cargo run -- db reset  # Careful: drops all data!
```

## Workflow Summary

1. **Add/modify YAML** in `packages/core/models/`
2. **Generate artifacts**: `cargo run -- schema generate`
3. **Validate**: `cargo run -- schema validate`
4. **Seed Neo4j**: `cargo run -- db seed`
5. **Verify in TUI**: `cargo run -- tui`
