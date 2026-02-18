# supernovae-agi — Claude Code Context

This workspace contains two complementary AI systems:

## The Brain & Body Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  supernovae-agi ARCHITECTURE                                                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────┐         MCP Protocol        ┌─────────────────┐   │
│  │     NOVANET         │◄──────────────────────────►│      NIKA       │   │
│  │     (Brain)         │                             │     (Body)      │   │
│  ├─────────────────────┤                             ├─────────────────┤   │
│  │ • Knowledge Graph   │    novanet_generate         │ • YAML Workflows│   │
│  │ • Entity Memory     │    novanet_describe         │ • LLM Providers │   │
│  │ • Locale Context    │    novanet_traverse         │ • DAG Execution │   │
│  │ • SEO/GEO Intel     │◄────────────────────────────│ • Tool Calling  │   │
│  │ • ADR Decisions     │                             │ • State Machine │   │
│  └─────────────────────┘                             └─────────────────┘   │
│         │                                                     │            │
│         ▼                                                     ▼            │
│  novanet-dev/                                          nika-dev/           │
│  └── tools/novanet/                                    └── tools/nika/     │
│      └── src/                                              └── src/        │
│          ├── core/     ← shared patterns                       ├── core/   │
│          └── tui/      ← terminal UI                           └── tui/    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Projects

### NovaNet (`novanet-dev/`)

Knowledge graph localization orchestrator. Generates native content across 200+ locales.

**Key concepts:**
- **Entity** → Semantic concept (invariant)
- **EntityNative** → Locale-specific content (authored)
- **Page/Block** → Structure (defined) → PageNative/BlockNative (generated)
- **MCP Server** → Exposes knowledge via novanet_* tools

**Tech:** Neo4j, TypeScript, Rust CLI+TUI

### Nika (`nika-dev/`)

Semantic YAML workflow engine. Executes multi-step AI workflows.

**Key concepts:**
- **Workflow** → YAML definition with semantic verbs
- **Verbs** → `infer:`, `exec:`, `fetch:`, `invoke:`, `agent:`
- **DAG** → Dependency resolution and execution
- **MCP Client** → Consumes NovaNet knowledge

**Tech:** Rust, tokio, MCP SDK

## Integration Pattern

```yaml
# Nika workflow using NovaNet context
workflow: generate-page
steps:
  - invoke: novanet_generate
    params:
      entity: "qr-code"
      locale: "fr-FR"
      forms: ["text", "title"]
    use.ctx: entity_context

  - infer: "Generate landing page"
    context: $entity_context
```

## Shared Patterns

Both Rust CLIs follow identical structure:

| Module | Purpose | NovaNet | Nika |
|--------|---------|---------|------|
| `core/config.rs` | Root discovery | ✓ | ✓ |
| `core/error.rs` | Error handling | ✓ | ✓ |
| `core/output.rs` | Format abstraction | ✓ | ✓ |
| `tui/` | Terminal UI | ✓ | ✓ (planned) |

## Documentation Structure

```
supernovae-agi/
├── docs/                   ← Cross-project (this workspace)
│   ├── plans/
│   └── research/
├── novanet-dev/docs/       ← NovaNet-specific
└── nika-dev/docs/          ← Nika-specific
```

## Commands

```bash
# Workspace-level
pnpm install          # Install all packages
pnpm build            # Build all
pnpm test             # Test all

# Project-level
cd novanet-dev && pnpm dev
cd nika-dev && cargo run
```

## Claude Code DX

**Skills**: `/workspace-nav` to switch projects. See `.claude/skills/INDEX.md` for all 19 skills.

**Hooks**: Auto-formatting, schema sync reminders, ADR context. See `.claude/settings.json`.
