# Quick Start Guide

Get NovaNet running in 5 minutes.

## Prerequisites

| Tool | Version | Check |
|------|---------|-------|
| Node.js | ≥20 | `node --version` |
| pnpm | ≥9 | `pnpm --version` |
| Docker | Latest | `docker --version` |
| Rust | ≥1.86 | `rustc --version` |

## Installation

### 1. Clone the Repository

```bash
git clone git@github.com:supernovae-st/novanet-hq.git
cd novanet-hq
```

### 2. Install Dependencies

```bash
pnpm install
```

### 3. Start Neo4j

```bash
pnpm infra:up
```

This starts a Neo4j container with:
- Browser: http://localhost:7474
- Bolt: bolt://localhost:7687
- Credentials: `neo4j` / `novanetpassword`

### 4. Seed the Database

```bash
pnpm infra:seed
```

This creates:
- Schema graph (Realms, Layers, Classes, Traits, ArcFamilies, ArcClasses)
- Sample data nodes
- Constraints and indexes

> **v0.12.0**: "Meta" → "Schema", "Kind" → "Class", "ArcKind" → "ArcClass"

### 5. Start Development

```bash
pnpm dev
```

Open http://localhost:3000 to see NovaNet Studio.

## Verification

### Check Neo4j

Open http://localhost:7474 and run:

```cypher
MATCH (n:Schema) RETURN labels(n), count(n)
```

Expected output (v0.12.4):
```
["Schema", "Realm"]      2
["Schema", "Layer"]      10
["Schema", "Class"]      61
["Schema", "Trait"]      5
["Schema", "ArcFamily"]  5
["Schema", "ArcClass"]   128
```

> **v0.12.0 ADR-023**: `:Meta` → `:Schema`, `Kind` → `Class`, `ArcKind` → `ArcClass`

### Check Rust CLI

```bash
cd tools/novanet
cargo run -- schema validate
```

Expected: `✅ Schema valid`

### Check Tests

```bash
# TypeScript tests
pnpm test

# Rust tests
cd tools/novanet && cargo test
```

## Interactive TUI

Launch the Terminal UI for graph exploration:

```bash
cd tools/novanet
cargo run -- tui
```

### TUI Navigation (v0.12.0)

| Key | Action |
|-----|--------|
| `1` | Graph mode (unified tree) |
| `2` | Nexus mode (Quiz, Audit, Stats, Help) |
| `j/k` | Navigate up/down |
| `h/l` | Collapse/expand |
| `Tab` | Switch panels |
| `/` | Search overlay |
| `?` | Help overlay |
| `q` | Quit |

### Nexus Hub Tabs

| Tab | Key | Description |
|-----|-----|-------------|
| Quiz | `Q` | Test NovaNet knowledge |
| Audit | `A` | Validate schema consistency |
| Stats | `S` | Matrix Control Tower dashboard |
| Help | `?` | Keybindings reference |

## Common Commands

### Development

| Command | Description |
|---------|-------------|
| `pnpm dev` | Start Studio dev server |
| `pnpm build` | Build all packages |
| `pnpm lint` | Lint all packages |
| `pnpm type-check` | TypeScript type checking |

### Infrastructure

| Command | Description |
|---------|-------------|
| `pnpm infra:up` | Start Neo4j |
| `pnpm infra:down` | Stop Neo4j |
| `pnpm infra:seed` | Seed database |
| `pnpm infra:reset` | Reset (down + up + seed) |

### Rust CLI

| Command | Description |
|---------|-------------|
| `cargo run -- data` | Query data nodes |
| `cargo run -- blueprint` | Query schema graph (v0.12.0: was `meta`) |
| `cargo run -- schema generate` | Regenerate artifacts |
| `cargo run -- schema validate` | Validate YAML coherence |
| `cargo run -- tui` | Interactive terminal UI |
| `cargo run -- blueprint` | Rich ASCII visualization |

## Next Steps

1. **Explore TUI** — `cargo run -- tui` for unified tree exploration
2. **Read Architecture** — [Architecture Overview](../architecture/overview.md)
3. **Learn Schema** — [Schema Management](./schema-management.md)
4. **Set up Claude Code** — [DX Overview](../claude-dx/overview.md)

## Troubleshooting

### Neo4j won't start

```bash
# Check if port is in use
lsof -i :7687

# Force recreate container
docker rm -f novanet-neo4j
pnpm infra:up
```

### Seed fails

```bash
# Check Neo4j is running
docker ps | grep neo4j

# Retry seed
pnpm infra:seed
```

### Type errors

```bash
# Rebuild types from YAML
cd tools/novanet
cargo run -- schema generate

# Then type-check
pnpm type-check
```

### Rust build fails

```bash
# Check Rust version (need 1.86+)
rustc --version

# Update if needed
rustup update stable
```
