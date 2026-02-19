# supernovae-agi Skills Index

Catalog of all Claude Code skills for the supernovae-agi workspace (NovaNet + Nika).

**NovaNet**: v0.13.1 | **Nika**: v0.3.0 | **Total Skills**: 26 (9 core + 10 studio + 7 nika)

---

## Core Skills (9)

Located in `.claude/skills/`

| Skill | Invocation | Description |
|-------|------------|-------------|
| **workspace-nav** | `/workspace-nav` | Navigate between NovaNet (brain) and Nika (body) projects |
| **novanet-adr** | `/novanet-adr` | Navigate ADRs by number, domain, status, or keyword |
| **novanet-architecture** | `/novanet-arch` | ASCII architecture diagrams (schema graph, pipeline, Rust, Studio) |
| **novanet-sync** | `/novanet-sync` | Schema validation/regeneration (YAML → artifacts) |
| **novanet-tui** | `/novanet-tui` | Galaxy-themed terminal UI keybindings and features |
| **novanet-mcp** | `/novanet-mcp` | MCP Server operations (RLM-on-KG, agent integration) |
| **security-audit** | `/security-audit` | Comprehensive security audit (cargo-deny, pnpm audit) |
| **codebase-audit** | `/codebase-audit` | Parallel codebase health analysis (Ralph Wiggum Loop) |
| **token-audit** | — | Design system token adoption verification |

---

## Nika Skills (7)

Located in `nika-dev/.claude/skills/`

| Skill | Invocation | Description |
|-------|------------|-------------|
| **nika-arch** | `/nika-arch` | Architecture diagram and module structure |
| **nika-run** | `/nika-run` | Run workflows with validation |
| **nika-diagnose** | `/nika-diagnose` | Systematic workflow diagnosis |
| **nika-debug** | `/nika-debug` | Debug with traces and logging |
| **nika-binding** | `/nika-binding` | Data binding syntax reference |
| **workflow-validate** | `/workflow-validate` | Validate YAML syntax and DAG |
| **nika-spec** | `/nika-spec` | Workflow specification reference |

---

## Studio Skills (10)

Located in `apps/studio/.claude/skills/`

| Skill | Description |
|-------|-------------|
| **force-graph-patterns** | Force-directed graph layout patterns |
| **react-flow-patterns** | React Flow node/edge patterns |
| **zustand-patterns** | Zustand store patterns with persist + immer |
| **radix-ui-patterns** | Radix UI component patterns |
| **neo4j-patterns** | Neo4j Cypher query patterns |
| **tailwind-patterns** | Tailwind CSS styling patterns |
| **lucide-patterns** | Lucide icon usage patterns |
| **framer-motion-patterns** | Framer Motion animation patterns |
| **glassmorphism-patterns** | Glassmorphism UI theme patterns |
| **keyboard-shortcuts** | Keyboard shortcut patterns |

---

## Skill Triggers

| Trigger Keywords | Skill |
|------------------|-------|
| workspace, switch project, novanet, nika, brain, body | workspace-nav |
| ADR, architecture decision, decision record, why design | novanet-adr |
| architecture, overview, structure, schema graph, codebase | novanet-architecture |
| sync, validate, generate, schema, YAML | novanet-sync |
| tui, terminal, keybindings, galaxy | novanet-tui |
| mcp, agent, RLM, knowledge graph, AI integration | novanet-mcp |
| security, audit, vulnerability, cargo-deny | security-audit |
| codebase, health, dead code, cleanup | codebase-audit |
| token, design system, gap, spacing | token-audit |

---

## Usage

### User-Invocable Skills

Invoke directly with slash command:

```bash
/novanet-adr               # List all domains
/novanet-adr 029           # Look up ADR-029
/novanet-adr must-know     # Essential ADRs for v0.13.0
/novanet-adr native        # Search by keyword/tag
/novanet-arch              # Full architecture
/novanet-arch meta         # Meta-graph only
/novanet-sync              # Check sync status
/novanet-sync generate     # Regenerate artifacts
/novanet-tui               # Launch TUI
/novanet-mcp               # Check MCP server status
/novanet-mcp test          # Run MCP server tests
/novanet-mcp debug         # Start with debug logging
/security-audit all        # Full security audit
/codebase-audit            # Full 10-agent audit
/codebase-audit quick      # Essential checks only
/codebase-audit --fix      # Auto-fix issues
```

### Auto-Triggered Skills

Claude automatically activates these skills based on context:

- **codebase-audit**: Before releases, after refactoring
- **token-audit**: When checking design system consistency

---

## Adding New Skills

1. Create directory: `.claude/skills/<skill-name>/`
2. Create `SKILL.md` with frontmatter:

```yaml
---
name: my-skill
description: What this skill does. Use when [trigger conditions].
disable-model-invocation: false
user-invocable: true
---

# Skill Content
...
```

3. Update this INDEX.md
4. Update `.claude/README.md` if user-invocable

---

## Related Files

| File | Purpose |
|------|---------|
| `.claude/README.md` | DX overview with all skills listed |
| `.claude/commands/` | Slash command definitions |
| `.claude/agents/` | Subagent definitions |
| `.claude/rules/` | Path-specific rules |
