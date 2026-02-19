# supernovae-agi

Monorepo workspace containing **NovaNet** (brain) and **Nika** (body).

## Auto-Imported Context

@.claude/CLAUDE.md @ROADMAP.md @CHANGELOG.md

---

## Quick Navigation

| Project | Description | Entry Point |
|---------|-------------|-------------|
| **NovaNet** | Knowledge graph (Neo4j) + MCP Server | `cd novanet-dev && pnpm dev` |
| **Nika** | Workflow engine (Rust) + MCP Client | `cd nika-dev && cargo run` |

## Core Philosophy

```
CRITICAL: Generation, NOT Translation

Source → Translate → Target                    ❌ WRONG
Entity (defined) → Generate natively → Native  ✅ RIGHT
```

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│  NovaNet (Brain)         MCP Protocol          Nika (Body)             │
│  ├── Knowledge Graph  ◄─────────────────────► ├── YAML Workflows       │
│  ├── 61 NodeClasses                           ├── 5 Verbs (infer,exec..)│
│  ├── 182 ArcClasses                           ├── DAG Execution        │
│  └── Neo4j + Rust TUI                         └── LLM Providers        │
└─────────────────────────────────────────────────────────────────────────┘
```

## Current Versions

| Project | Version | Key Changes |
|---------|---------|-------------|
| NovaNet | v0.14.0 | 7 MCP tools, denomination_forms, context_build_log, 1082 tests |
| Nika | v0.4.0 | rig-core v0.31, RigAgentLoop, RigProvider, 621+ tests, pure rig-core |

## Commands

```bash
# Workspace
pnpm install              # Install all
pnpm build                # Build all
pnpm test                 # Test all

# NovaNet
cd novanet-dev
pnpm dev                  # Studio: http://localhost:3000
cargo run -- tui          # TUI explorer
cargo run -- schema validate  # Validate YAML

# Nika
cd nika-dev
cargo run -- run workflow.yaml  # Execute workflow
cargo run -- tui workflow.yaml  # TUI observer
```

## ADRs (Must-Know)

| ADR | Decision | Impact |
|-----|----------|--------|
| **029** | *Native Pattern | EntityNative, PageNative naming |
| **030** | Slug Ownership | Page owns URL, Entity owns semantics |
| **033** | Denomination Forms | text/title/abbrev/url for LLM refs |
| **024** | Trait = Data Origin | defined/authored/imported/generated/retrieved |

**Full ADRs:** `.claude/rules/adr/` or use `/adr <number>` command

## Key Files

| Path | Purpose |
|------|---------|
| `.claude/rules/adr/CHEAT-SHEET.md` | Quick ADR reference |
| `.claude/rules/schema-standard.md` | YAML node structure |
| `.claude/rules/architecture.md` | Monorepo rules |
| `novanet-dev/CLAUDE.md` | NovaNet details |
| `nika-dev/CLAUDE.md` | Nika details |

## Claude Code DX

**Skills:** `/spn-powers:yo` for full inventory

**Key commands:**
- `/novanet-arch` — Architecture diagrams
- `/adr <N>` — ADR lookup
- `/schema:add-node` — Add node type
- `/workspace-nav` — Switch projects

## Conventions

| Aspect | Convention |
|--------|------------|
| Package Manager | pnpm |
| Commits | `type(scope): description` |
| Code Style | 2 spaces, 100 chars, single quotes |
| Testing | TDD preferred, 80% coverage |

---

**Target:** QR Code AI (https://qrcode-ai.com)
