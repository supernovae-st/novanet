# Contributing Guide

Guidelines for contributing to NovaNet.

## Development Setup

### Prerequisites

| Tool | Version | Check |
|------|---------|-------|
| Node.js | ≥20 | `node --version` |
| pnpm | ≥9 | `pnpm --version` |
| Docker | Latest | `docker --version` |
| Rust | ≥1.86 | `rustc --version` |

### Clone and Install

```bash
git clone git@github.com:supernovae-st/novanet-hq.git
cd novanet-hq
pnpm install
```

### Start Development

```bash
pnpm infra:up    # Start Neo4j
pnpm infra:seed  # Seed database
pnpm dev         # Start Studio
```

## Code Standards

### TypeScript

- 2 spaces indentation
- 100 character line limit
- Single quotes, semicolons
- Run `pnpm lint` before committing

### Rust

- Edition 2024
- Zero clippy warnings
- Run `cargo fmt` and `cargo clippy -- -D warnings`

### Commit Messages

Use Conventional Commits:

```
type(scope): description

Co-Authored-By: Nika <agent@nika.sh>
```

**Types**: feat, fix, docs, style, refactor, test, chore

**Examples**:
- `feat(tui): add Stats dashboard with heartbeat animation`
- `fix(schema): validate realm/layer path consistency`
- `docs(mintlify): update novanet mcp reference`

## Testing

### Run All Tests

```bash
# TypeScript
pnpm test

# Rust
cd tools/novanet
cargo test
```

### Test Categories

| Command | Description |
|---------|-------------|
| `cargo test` | Unit tests (fast) |
| `cargo test -- --ignored` | Integration tests (needs Neo4j) |
| `cargo nextest run` | Parallel runner |

### Coverage

```bash
cargo llvm-cov --html
open target/llvm-cov/html/index.html
```

## Pre-Commit Checklist

Before committing, verify:

```bash
# TypeScript
pnpm lint
pnpm type-check
pnpm test

# Rust
cd tools/novanet
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo deny check
```

## Pull Request Process

1. **Create feature branch**
   ```bash
   git checkout -b feat/my-feature
   ```

2. **Make changes** following code standards

3. **Test thoroughly**
   ```bash
   pnpm test && cd tools/novanet && cargo test
   ```

4. **Commit with conventional message**

5. **Push and create PR**
   ```bash
   git push -u origin feat/my-feature
   ```

## Schema Changes

When modifying the schema:

1. Edit YAML in `packages/core/models/`
2. Regenerate: `cargo run -- schema generate`
3. Validate: `cargo run -- schema validate`
4. Seed: `cargo run -- db seed`
5. Test: `cargo test`

## Security

### Dependency Audit

```bash
# Rust
cargo deny check
cargo audit

# TypeScript
pnpm audit
```

### Policy

- All dependencies must pass license check
- Critical vulnerabilities block merge
- Document exceptions in `deny.toml`

## Documentation

### Update Docs

When making significant changes, update documentation in `supernovae-docs/mintlify/novanet/`:

```
mintlify/novanet/
├── introduction.mdx   # Overview
├── concepts.mdx       # Core concepts
├── schema.mdx         # Schema system
├── nodes.mdx          # Node classes
├── arcs.mdx           # Arc classes
├── traits.mdx         # Traits
├── mcp*.mdx           # MCP tools reference
└── tutorial-*.mdx     # Tutorials
```

### Preview Docs

```bash
cd supernovae-docs/mintlify
npx mintlify dev    # Preview at http://localhost:3000
npx mintlify build  # Validate build
```

## Getting Help

- **Issues**: https://github.com/supernovae-st/novanet-hq/issues
- **Discussions**: Use GitHub Discussions for questions
- **Claude Code**: Use `/help` for command assistance
