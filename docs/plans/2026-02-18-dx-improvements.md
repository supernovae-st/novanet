# DX Improvements Plan for supernovae-agi

**Date:** 2026-02-18
**Status:** Implementation in Progress
**Author:** Claude + Thibaut

---

## Overview

Ce plan détaille les améliorations DX (Developer Experience) pour le workspace supernovae-agi, qui contient deux projets AI complémentaires :
- **NovaNet** (novanet-dev/) — Knowledge graph "cerveau"
- **Nika** (nika-dev/) — Workflow engine "corps"

---

## Architecture Cible

```
supernovae-agi/
├── .claude/
│   ├── CLAUDE.md                      # ✓ Existe - contexte workspace
│   ├── settings.json                  # NEW: Hooks partagés
│   ├── rules/                         # ✓ Existe - ADRs, architecture
│   └── skills/
│       └── workspace-nav/
│           └── SKILL.md               # NEW: Navigation inter-projets
│
├── novanet-dev/
│   ├── .claude/
│   │   ├── settings.json              # NEW: Hooks Neo4j + Rust
│   │   └── skills/
│   │       └── schema-validate/
│   │           └── SKILL.md           # NEW: Validation schema YAML
│   └── tools/novanet/                 # Rust CLI existant
│
└── nika-dev/
    ├── .claude/
    │   ├── settings.json              # NEW: Hooks Rust + MCP
    │   └── skills/
    │       └── workflow-validate/
    │           └── SKILL.md           # NEW: Validation workflows YAML
    └── tools/nika/                    # Rust CLI existant
```

---

## Phase 1: Hooks Configuration

### 1.1 Root Workspace Hooks

**Fichier:** `.claude/settings.json`

**Objectif:** Afficher le contexte workspace au démarrage de session.

```json
{
  "hooks": {
    "SessionStart": [
      {
        "type": "command",
        "command": "echo '╔═══════════════════════════════════════════════════════════╗' && echo '║  supernovae-agi workspace                                  ║' && echo '║  NovaNet (brain) + Nika (body)                             ║' && echo '╚═══════════════════════════════════════════════════════════╝'"
      }
    ]
  }
}
```

### 1.2 NovaNet Hooks

**Fichier:** `novanet-dev/.claude/settings.json`

**Objectifs:**
1. Vérifier le statut Neo4j au démarrage
2. Auto-check cargo fmt après édition de fichiers Rust
3. Rappeler les commandes importantes

```json
{
  "hooks": {
    "SessionStart": [
      {
        "type": "command",
        "command": "echo '── NovaNet Context ──' && (docker ps --filter 'name=neo4j' --format '{{.Status}}' 2>/dev/null | grep -q Up && echo '✓ Neo4j: Running' || echo '✗ Neo4j: Not running (pnpm infra:up)') && echo '  Schema: 61 nodes, 182 arcs (v0.13.1)' && echo '  TUI: cargo run -- tui'"
      }
    ],
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "if echo \"$CLAUDE_FILE_PATHS\" | grep -q '\\.rs$'; then cd \"$CLAUDE_PROJECT_DIR/tools/novanet\" 2>/dev/null && cargo fmt --check --quiet 2>/dev/null || echo '⚠ Run: cargo fmt'; fi"
          }
        ]
      }
    ]
  }
}
```

### 1.3 Nika Hooks

**Fichier:** `nika-dev/.claude/settings.json`

**Objectifs:**
1. Afficher le contexte Nika au démarrage
2. Auto-check cargo fmt après édition de fichiers Rust

```json
{
  "hooks": {
    "SessionStart": [
      {
        "type": "command",
        "command": "echo '── Nika Context ──' && echo '  Verbs: infer, exec, fetch, invoke, agent' && echo '  MCP: connects to NovaNet' && echo '  Run: cargo run -- run workflow.yaml'"
      }
    ],
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "if echo \"$CLAUDE_FILE_PATHS\" | grep -q '\\.rs$'; then cd \"$CLAUDE_PROJECT_DIR/tools/nika\" 2>/dev/null && cargo fmt --check --quiet 2>/dev/null || echo '⚠ Run: cargo fmt'; fi"
          }
        ]
      }
    ]
  }
}
```

---

## Phase 2: Skills Creation

### 2.1 Workspace Navigation Skill

**Fichier:** `.claude/skills/workspace-nav/SKILL.md`

**Usage:** `/workspace-nav` ou détection automatique quand on parle de navigation entre projets.

```markdown
---
name: workspace-nav
description: Navigate between NovaNet and Nika projects, understand their Brain/Body architecture
---

# supernovae-agi Workspace Navigation

## Architecture: Brain + Body

┌─────────────────────────────────────────────────────────────────────────────┐
│  supernovae-agi                                                             │
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
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘

## Quick Commands

### NovaNet (novanet-dev/)
- **TUI**: `cd novanet-dev/tools/novanet && cargo run -- tui`
- **Schema validate**: `cargo run -- schema validate`
- **Schema generate**: `cargo run -- schema generate`
- **Start Neo4j**: `cd novanet-dev && pnpm infra:up`
- **Seed Neo4j**: `cd novanet-dev && pnpm infra:seed`

### Nika (nika-dev/)
- **Run workflow**: `cd nika-dev/tools/nika && cargo run -- run workflow.yaml`
- **Validate workflow**: `cargo run -- validate workflow.yaml`
- **Build**: `cargo build --release`

## Project Paths

| Project | Main Code | Config | Tests |
|---------|-----------|--------|-------|
| NovaNet | `novanet-dev/tools/novanet/src/` | `novanet-dev/packages/core/models/` | `cargo test` |
| Nika | `nika-dev/tools/nika/src/` | `nika-dev/tools/nika/examples/` | `cargo test` |

## MCP Integration

Nika uses NovaNet via MCP tools:
- `novanet_generate` — Generate content for entity+locale
- `novanet_describe` — Get entity description
- `novanet_traverse` — Navigate knowledge graph
- `novanet_search` — Search entities
- `novanet_context` — Build LLM context

Example workflow using NovaNet:
```yaml
workflow: generate-page
steps:
  - invoke: novanet_generate
    params:
      entity: "qr-code"
      locale: "fr-FR"
    use.ctx: entity_context
```
```

### 2.2 NovaNet Schema Validate Skill

**Fichier:** `novanet-dev/.claude/skills/schema-validate/SKILL.md`

```markdown
---
name: schema-validate
description: Validate and regenerate NovaNet YAML schema artifacts
---

# NovaNet Schema Validation

## Quick Validation

Run schema validation:
```bash
cd tools/novanet && cargo run -- schema validate
```

## Full Regeneration

If validation passes, regenerate all artifacts:
```bash
cargo run -- schema generate
```

This regenerates 12 artifact types from YAML source of truth.

## Expected Counts (v0.13.1)

| Type | Count |
|------|-------|
| Nodes | 61 (40 shared + 21 org) |
| Arcs | 182 (6 families) |
| Layers | 10 (4 shared + 6 org) |
| Traits | 5 (defined, authored, imported, generated, retrieved) |

## Common Issues

1. **Missing timestamp properties**: Add `created_at` and `updated_at` to standard_properties
2. **Wrong trait**: Check ADR-024 (Trait = Data Origin)
3. **Composite key missing denorm**: Add `entity_key`, `locale_key` for *Native nodes

## YAML Location

Node classes: `packages/core/models/node-classes/{realm}/{layer}/{name}.yaml`
Arc classes: `packages/core/models/arc-classes/{family}/{name}.yaml`
```

### 2.3 Nika Workflow Validate Skill

**Fichier:** `nika-dev/.claude/skills/workflow-validate/SKILL.md`

```markdown
---
name: workflow-validate
description: Validate Nika YAML workflow syntax and DAG structure
---

# Nika Workflow Validation

## Validate a Workflow

```bash
cd tools/nika && cargo run -- validate workflow.yaml
```

## Run a Workflow

```bash
cargo run -- run workflow.yaml
```

## Workflow Structure

```yaml
workflow: my-workflow
description: "What this workflow does"

env:
  MODEL: claude-3-opus

steps:
  - id: step1
    infer: "Generate something"
    model: $MODEL
    use.ctx: result1

  - id: step2
    invoke: novanet_generate
    params:
      entity: "qr-code"
      locale: "fr-FR"
    depends_on: [step1]
    use.ctx: result2
```

## Semantic Verbs

| Verb | Purpose | Example |
|------|---------|---------|
| `infer:` | LLM generation | `infer: "Summarize this"` |
| `exec:` | Shell command | `exec: "npm run build"` |
| `fetch:` | HTTP request | `fetch: { url: "...", method: GET }` |
| `invoke:` | MCP tool call | `invoke: novanet_generate` |
| `agent:` | Agentic loop | `agent: { goal: "...", tools: [...] }` |

## DAG Rules

- `depends_on: [step_ids]` — explicit dependencies
- `use.ctx: var_name` — output variable binding
- `$var_name` — reference previous output
```

---

## Phase 3: CLAUDE.md Updates

### 3.1 Update Root CLAUDE.md

Add DX section to `.claude/CLAUDE.md`:

```markdown
## Developer Experience

### Hooks (Automatic)

| Hook | Location | Action |
|------|----------|--------|
| SessionStart | Root | Display workspace banner |
| SessionStart | novanet-dev | Check Neo4j status |
| SessionStart | nika-dev | Display Nika context |
| PostToolUse | Both | Check cargo fmt on .rs files |

### Skills (Manual)

| Skill | Command | Description |
|-------|---------|-------------|
| workspace-nav | `/workspace-nav` | Navigate between projects |
| schema-validate | `/schema-validate` | Validate NovaNet schema |
| workflow-validate | `/workflow-validate` | Validate Nika workflows |

### Common Tasks

```bash
# Start Neo4j
cd novanet-dev && pnpm infra:up

# Run NovaNet TUI
cd novanet-dev/tools/novanet && cargo run -- tui

# Run Nika workflow
cd nika-dev/tools/nika && cargo run -- run workflow.yaml

# Validate both projects
cd novanet-dev/tools/novanet && cargo test
cd nika-dev/tools/nika && cargo test
```
```

---

## Phase 4: Testing

### 4.1 Test Checklist

- [ ] Root SessionStart hook displays banner
- [ ] novanet-dev SessionStart shows Neo4j status
- [ ] nika-dev SessionStart shows context
- [ ] PostToolUse detects .rs files and checks fmt
- [ ] `/workspace-nav` skill works
- [ ] `/schema-validate` skill works
- [ ] `/workflow-validate` skill works

### 4.2 Test Commands

```bash
# Test hooks by starting new Claude session in each directory
cd supernovae-agi && claude
cd novanet-dev && claude
cd nika-dev && claude

# Test skills
/workspace-nav
/schema-validate
/workflow-validate
```

---

## Implementation Order

| Step | Task | Priority | Time |
|------|------|----------|------|
| 1 | Create novanet-dev/.claude/settings.json | P0 | 2min |
| 2 | Create nika-dev/.claude/settings.json | P0 | 2min |
| 3 | Create root .claude/settings.json | P0 | 1min |
| 4 | Create workspace-nav skill | P1 | 3min |
| 5 | Create schema-validate skill | P1 | 2min |
| 6 | Create workflow-validate skill | P1 | 2min |
| 7 | Update root CLAUDE.md | P2 | 3min |
| 8 | Test all hooks and skills | P2 | 5min |

**Total estimated time:** ~20 minutes

---

## Success Criteria

1. **SessionStart hooks** display relevant context for each project
2. **PostToolUse hooks** catch unformatted Rust code
3. **Skills** provide quick access to common operations
4. **CLAUDE.md** documents all DX features
5. **No breaking changes** to existing functionality
