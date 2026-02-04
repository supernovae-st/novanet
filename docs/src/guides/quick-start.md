# Quick Start Guide

Get NovaNet running in 5 minutes.

## Prerequisites

| Tool | Version | Check |
|------|---------|-------|
| Node.js | ≥20 | `node --version` |
| pnpm | ≥9 | `pnpm --version` |
| Docker | Latest | `docker --version` |
| Rust | ≥1.84 | `rustc --version` |

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
- Meta-graph (Realms, Layers, Kinds, Traits, ArcFamilies, ArcKinds)
- Sample data nodes
- Constraints and indexes

### 5. Start Development

```bash
pnpm dev
```

Open http://localhost:3000 to see NovaNet Studio.

## Verification

### Check Neo4j

Open http://localhost:7474 and run:

```cypher
MATCH (n:Meta) RETURN labels(n), count(n)
```

Expected output:
```
["Meta", "Realm"]     3
["Meta", "Layer"]     9
["Meta", "Kind"]      35
["Meta", "Trait"]     5
["Meta", "ArcFamily"]  5
["Meta", "ArcKind"]   76
```

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
| `cargo run -- meta` | Query meta-graph |
| `cargo run -- schema generate` | Regenerate artifacts |
| `cargo run -- tui` | Interactive terminal UI |

## Next Steps

1. **Explore Studio** — Navigate the graph at http://localhost:3000
2. **Read Architecture** — [Architecture Overview](../architecture/overview.md)
3. **Learn Commands** — [Schema Management](./schema-management.md)
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
