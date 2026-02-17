# NovaNet Developer Experience (DX)

Claude Code configuration for the NovaNet monorepo.

**Version**: v0.13.1 | **Updated**: 2026-02-17 (Codebase audit + terminology fixes)

---

## 📊 Research Complete: Claude Code Patterns

**Context7 Analysis Completed**: 4,858 code snippets from 4 high-reputation libraries analyzed.

### Research Reports Available
- **Full Report**: `RESEARCH-context7-claude-code-patterns.md` (2,000+ lines)
- **Quick Summary**: `ACTION-SUMMARY.md` (implementation roadmap, 1-2 hours)
- **Comparison Matrix**: `PATTERNS-COMPARISON-MATRIX.md` (gap analysis)

### Assessment Results
- **Overall Score**: 79% (STRONG)
- **Your Strengths**: Exceptional CLAUDE.md, ADRs (+900%), skills/commands (+150%), rules (+600%)
- **Quick Wins**: 3 items, 55 minutes total
- **Value-Add**: 2 items, 90 minutes
- **Polish**: 3 items, 60 minutes (optional)

**Conclusion**: NovaNet exceeds typical open-source projects in Claude Code DX setup.

---

## Docs

---

## Quick Reference

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                           NOVANET DX - v0.13.0                                ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║  COMMANDS           → See COMMANDS.md                                         ║
║  ├── /novanet-arch    Architecture diagrams (ASCII)                           ║
║  ├── /novanet-sync    Schema validation/regeneration                          ║
║  ├── /schema:*        Schema management (add-node, edit-node, add-arc)        ║
║  └── /adr             ADR quick lookup                                        ║
║                                                                               ║
║  SKILLS             → See SKILLS.md                                           ║
║  ├── novanet-architecture   ASCII architecture diagrams                       ║
║  ├── novanet-sync           YAML ↔ TypeScript ↔ Mermaid sync                 ║
║  ├── novanet-tui            Galaxy-themed terminal UI                         ║
║  ├── security-audit         Rust + TypeScript security                        ║
║  └── codebase-audit         Parallel codebase analysis (10 agents)            ║
║                                                                               ║
║  AGENTS             → See AGENTS.md                                           ║
║  ├── neo4j-architect   Cypher queries, schema-graph design                    ║
║  └── code-reviewer     Code quality, security, TS/Rust review                 ║
║                                                                               ║
║  WORKFLOWS          → See WORKFLOWS.md                                        ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Documentation Index

| File | Content |
|------|---------|
| [COMMANDS.md](./COMMANDS.md) | Slash commands detailed documentation |
| [SKILLS.md](./SKILLS.md) | Skills with triggers and arguments |
| [AGENTS.md](./AGENTS.md) | Subagent specializations and patterns |
| [WORKFLOWS.md](./WORKFLOWS.md) | Development workflows and pipelines |

---

## Key Numbers (v0.13.0)

| Metric | Value |
|--------|-------|
| Classes (node types) | 61 |
| ArcClasses (relations) | 169 |
| Realms | 2 (shared, org) |
| Layers | 10 (4 shared + 6 org) |
| Traits | 5 (defined, authored, imported, generated, retrieved) |
| ArcFamilies | 5 (ownership, localization, semantic, generation, mining) |
| Rust tests | 1031 |
| TUI Modes | 2 (Graph, Nexus) |
| ADRs | 32 |

---

## Directory Structure

```
.claude/
├── README.md            ← This file (hub)
├── COMMANDS.md          ← Slash commands
├── SKILLS.md            ← Skills documentation
├── AGENTS.md            ← Agent documentation
├── WORKFLOWS.md         ← Development workflows
├── settings.json        ← Project settings (permissions, env, hooks)
├── hooks/               ← 7 active hooks
│   ├── session-start.sh
│   ├── post-edit-format.sh
│   ├── keybindings-reminder.sh
│   ├── yaml-sync-reminder.sh
│   ├── doc-sync-reminder.sh
│   ├── semantic-check.sh
│   ├── adr-context.sh
│   └── archive/         ← Archived hooks
├── rules/               ← Path-specific rules
│   ├── rust.md          ← tools/novanet/**/*.rs
│   ├── typescript.md    ← packages/, apps/
│   ├── cypher.md        ← packages/db/seed/
│   ├── security.md      ← All code
│   ├── novanet-terminology.md  ← Domain vocabulary
│   ├── novanet-decisions.md    ← ADR index
│   ├── arc-design-guide.md     ← Arc patterns
│   └── adr/             ← Full ADR content (32 ADRs)
├── commands/            ← Slash command definitions
├── skills/              ← Skill definitions
├── agents/              ← Agent definitions
└── guides/              ← DX testing guides
```

---

## Hooks

| Event | Hook | Purpose |
|-------|------|---------|
| SessionStart | `session-start.sh` | Show project status |
| PostToolUse | `post-edit-format.sh` | Auto-format after edits |
| PostToolUse | `keybindings-reminder.sh` | TUI file edit reminder |
| PostToolUse | `yaml-sync-reminder.sh` | YAML model edit reminder |
| PostToolUse | `doc-sync-reminder.sh` | Documentation edit reminder |
| PostToolUse | `semantic-check.sh` | Rust code validation |
| PostToolUse | `adr-context.sh` | ADR context loading |

Archived hooks in `.claude/hooks/archive/`.

---

## Path-Specific Rules

| Rule File | Paths | Content |
|-----------|-------|---------|
| `rust.md` | `tools/novanet/**/*.rs` | Error handling, async, modules |
| `typescript.md` | `packages/**/*.ts`, `apps/**/*.tsx` | Type safety, React patterns |
| `cypher.md` | `packages/db/seed/**/*.cypher` | Schema-graph, ArcFamily |
| `security.md` | All code | Security patterns, pre-commit |
| `novanet-terminology.md` | All | v0.13.0 domain vocabulary |
| `novanet-decisions.md` | All | ADR index (quick reference) |
| `arc-design-guide.md` | All | Arc design best practices |

Rules use YAML frontmatter with `paths:` field for scoping.

---

## v11.7 Unified Tree Architecture

**Principle:** "If it's a node in Neo4j, it's a node everywhere"

| Aspect | Before (v11.6) | After (v11.7) |
|--------|----------------|---------------|
| Modes | 5 (Meta/Data/Overlay/Query/Atlas) | 2 (Graph/Nexus) |
| Realm/Layer | Visual groupings | Clickable nodes |
| Icons | Mixed emoji | Dual: Lucide (web) + Unicode (terminal) |

```
[1]Graph   Unified tree: Realm > Layer > Class > Instance + Arcs
[2]Nexus   Hub: Quiz, Audit, Stats, Help
[/]        Search overlay
```

**Reference:** ADR-022 in `rules/novanet-decisions.md`

---

## Related Documentation

| File | Purpose |
|------|---------|
| `/CLAUDE.md` | Monorepo overview |
| `/packages/core/CLAUDE.md` | Core package (types, schemas, YAML) |
| `/apps/studio/CLAUDE.md` | Studio application |
| `/tools/novanet/CLAUDE.md` | Rust binary (CLI + TUI) |
| `/docs/plans/` | Design documents |

---

## Keeping Documentation Synced

Update when:
1. **Commands change** - New, renamed, or removed
2. **Skills updated** - New triggers or sections
3. **Schema version bumps** - New Classes, ArcClasses
4. **v0.13.0 changes** - Rust binary additions

**Validation:**
```bash
/novanet-arch       # Should show same numbers
/ontology-audit     # Validates all artifacts
```
