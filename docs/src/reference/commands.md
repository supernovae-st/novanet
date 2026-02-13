# Command Reference

Complete reference for `novanet` CLI commands.

## Overview

```bash
novanet [OPTIONS] <COMMAND>
```

**Global Options**:
- `--root <PATH>` — Monorepo root (default: auto-detect)
- `--format <FORMAT>` — Output format: table, json, cypher
- `-h, --help` — Print help
- `-V, --version` — Print version

## Read Commands

### data

Query data nodes (non-meta).

```bash
novanet data [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--realm <REALM>` | Filter by realm (shared, org) |
| `--layer <LAYER>` | Filter by layer |
| `--kind <KIND>` | Filter by Kind name |
| `--format <FMT>` | Output format |

**Examples**:
```bash
novanet data --realm=org
novanet data --kind=Page --format=json
```

### meta

Query meta-graph (schema nodes).

```bash
novanet meta [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--format <FMT>` | Output format |

**Examples**:
```bash
novanet meta
novanet meta --format=json
```

### overlay

Combined data + meta view.

```bash
novanet overlay [OPTIONS]
```

### query

Faceted query with multiple filters.

```bash
novanet query [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--realm <REALM>` | Filter by realm |
| `--layer <LAYER>` | Filter by layer |
| `--trait <TRAIT>` | Filter by trait |
| `--kind <KIND>` | Filter by Kind |
| `--arc-family <FAM>` | Filter arcs by family |

**Examples**:
```bash
novanet query --realm=org --layer=semantic
novanet query --trait=localized
```

### search

Fulltext and property search.

```bash
novanet search --query <QUERY> [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--query <Q>` | Search string (required) |
| `--kind <KIND>` | Limit to specific Kind |
| `--limit <N>` | Maximum results (default: 50) |

**Examples**:
```bash
novanet search --query="page"
novanet search --query="qr code" --kind=Entity --limit=10
```

## Write Commands

### node create

Create a new node.

```bash
novanet node create --kind <KIND> --key <KEY> [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--kind <KIND>` | Node type (required) |
| `--key <KEY>` | Unique key (required) |
| `--props <JSON>` | Properties as JSON |

**Examples**:
```bash
novanet node create --kind=Page --key=my-page --props='{"title":"My Page"}'
```

### node edit

Modify node properties.

```bash
novanet node edit --key <KEY> --set <JSON>
```

| Option | Description |
|--------|-------------|
| `--key <KEY>` | Node key (required) |
| `--set <JSON>` | Properties to update |

### node delete

Delete a node.

```bash
novanet node delete --key <KEY> --confirm
```

### arc create

Create a relationship.

```bash
novanet arc create --from <KEY> --to <KEY> --kind <ARC_KIND>
```

| Option | Description |
|--------|-------------|
| `--from <KEY>` | Source node key |
| `--to <KEY>` | Target node key |
| `--kind <KIND>` | Arc type (e.g., HAS_PAGE) |

### arc delete

Delete a relationship.

```bash
novanet arc delete --id <ID> --confirm
```

## Schema Commands

### schema generate

Generate all artifacts from YAML.

```bash
novanet schema generate [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--dry-run` | Preview without writing |
| `--artifact <NAME>` | Generate specific artifact |

**Examples**:
```bash
novanet schema generate
novanet schema generate --dry-run
```

### schema validate

Validate YAML coherence.

```bash
novanet schema validate [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--strict` | Fail on warnings |

## Database Commands

### db seed

Execute seed Cypher files.

```bash
novanet db seed
```

### db migrate

Run migrations.

```bash
novanet db migrate
```

### db reset

Drop and re-seed (destructive).

```bash
novanet db reset --confirm
```

## Documentation Commands

### doc generate

Generate Mermaid diagrams.

```bash
novanet doc generate [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--view <VIEW>` | Generate specific view |
| `--list` | List available views |
| `--dry-run` | Preview without writing |

## Utility Commands

### blueprint

Rich ASCII visualization.

```bash
novanet blueprint [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--view <VIEW>` | View type: tree, flow, arcs, stats, glossary |
| `--no-validate` | Skip validation |

**Views**:
- `tree` — Realm > Layer > Kind hierarchy
- `flow` — Flow diagrams
- `arcs` — Arc families with relationships
- `stats` — Raw counts (supports --format=json)
- `glossary` — Term definitions
- `cardinality` — 1:1, 1:N, N:M constraints

### tui

Interactive terminal UI.

```bash
novanet tui
```

### doctor

System health check.

```bash
novanet doctor [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--skip-db` | Skip Neo4j connectivity check |

### completions

Generate shell completions.

```bash
novanet completions <SHELL>
```

**Shells**: bash, zsh, fish, powershell, elvish

**Example**:
```bash
novanet completions zsh > ~/.zsh/completions/_novanet
```

## Locale Commands

### locale list

List available locales.

```bash
novanet locale list [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--format <FMT>` | Output format |

### locale import

Import locale data.

```bash
novanet locale import --file <PATH>
```

### locale generate

Generate locale Cypher.

```bash
novanet locale generate --csv <PATH> --output <PATH>
```

## Knowledge Commands

### knowledge generate

Generate knowledge atoms from ATH data.

```bash
novanet knowledge generate [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--tier <TIER>` | Tier: all, terms, expressions |

### knowledge list

List knowledge tiers.

```bash
novanet knowledge list
```

## Entity Commands

### entity seed

Seed entities by phase.

```bash
novanet entity seed --project <NAME> [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--project <NAME>` | Project name (required) |
| `--phase <N>` | Specific phase number |

### entity list

List available phases.

```bash
novanet entity list --project <NAME>
```

### entity validate

Validate phase data.

```bash
novanet entity validate --project <NAME>
```

## Filter Command

### filter build

Build Cypher from JSON filter (Studio subprocess).

```bash
echo '{"realms":["org"]}' | novanet filter build
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | Neo4j connection failed |
| 4 | Validation error |
