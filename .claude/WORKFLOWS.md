# NovaNet Workflows

Common development workflows and commands. See [README.md](./README.md) for overview.

---

## Schema Sync Pipeline (Rust-first)

```bash
# Validate YAML <-> Neo4j consistency
novanet schema validate

# Regenerate all artifacts from YAML
novanet schema generate

# Generate view-specific Mermaid diagrams
novanet doc generate                   # All views → packages/core/models/docs/*.md
novanet doc generate --view=<id>       # Single view
novanet doc generate --list            # List views with categories

# Seed database
novanet db seed

# Full reset
pnpm infra:down && pnpm infra:up && novanet db seed
```

---

## Development

```bash
# Start Neo4j + seed
pnpm infra:up && novanet db seed

# Start Studio
pnpm dev

# Type check
pnpm type-check

# Tests
pnpm test
```

---

## Documentation Maintenance

```bash
# Check documentation consistency
pnpm doc:audit

# Check skills/commands/rules against YAML
pnpm skill:audit

# Run all audits
pnpm audit:all

# Regenerate Mermaid view diagrams
pnpm doc:generate
```

**Source of truth:** `/VERSION` file contains the canonical schema version.

**`pnpm doc:audit` checks:**
- Outdated version references
- Deprecated terminology
- Incorrect node/arc counts (61 nodes, 169 arcs)
- Outdated realm/layer structure

**Auto-sync reminders:** Claude Code hooks trigger when YAML/docs are edited.

---

## Adding New DX Elements

### New Command

1. Create `.claude/commands/<name>.md`
2. Add frontmatter with `description` and optional `argument-hint`
3. Document actions and examples

### New Skill

1. Create `.claude/skills/<name>/SKILL.md`
2. Add frontmatter with `name`, `description`, `user-invocable`
3. Document triggers and what it provides

### New Agent

1. Create `.claude/agents/<name>.md`
2. Add frontmatter with `name`, `description`, `tools`, `model`
3. Document specialization and key patterns

---

## Pre-Commit Checklist

```bash
# Rust
cargo fmt && cargo clippy -- -D warnings && cargo nextest run && cargo deny check

# TypeScript
pnpm type-check && pnpm lint && pnpm test

# Security
cargo audit && pnpm audit
```

---

## v11.7 Implementation Skills

When implementing v11.7 Unified Tree Architecture:

| Phase | Skills | Agents |
|-------|--------|--------|
| Neo4j Migration | - | `neo4j-architect` |
| YAML Updates | `spn-writing:mermaid` | - |
| Type Definitions | `rust-core` | `rust-architect` |
| Generators | `rust-core` | `rust-pro` |
| TUI (Rust) | `rust-async`, `test-driven-development` | `rust-async-expert` |
| Studio (TS) | - | `feature-dev:code-architect` |
| Testing | `testing-anti-patterns` | `feature-dev:code-reviewer` |

### Pre-Implementation Checklist

```
- [ ] Neo4j running (`pnpm infra:up`)
- [ ] Schema seeded (`pnpm infra:seed`)
- [ ] Tests pass (`cargo nextest run && pnpm test`)
- [ ] Clean git status
- [ ] Create worktree (`/spn-powers:using-git-worktrees`)
```
