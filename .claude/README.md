# NovaNet Developer Experience (DX)

Claude Code configuration for the NovaNet monorepo.

**Version**: v9.0.1 | **Docs**: [Claude Code Official](https://docs.anthropic.com/en/docs/claude-code)

---

## Quick Reference

```
╔═══════════════════════════════════════════════════════════════════════════════════════════════════╗
║                              NOVANET DX - v9.0.1                                                  ║
╠═══════════════════════════════════════════════════════════════════════════════════════════════════╣
║                                                                                                   ║
║   COMMANDS (slash commands)                                                                       ║
║   ├── /novanet-arch [section]    → Architecture diagrams (ASCII)                                  ║
║   ├── /novanet-sync [action]     → Schema validation/regeneration                                 ║
║   ├── /novanet                   → Start Studio session (apps/studio)                             ║
║   ├── /novanet-bye               → End session with cleanup                                       ║
║   ├── /ontology-audit            → Full ontology synchronization audit                            ║
║   │                                                                                               ║
║   │   SCHEMA MANAGEMENT (knowledge graph editing)                                                 ║
║   ├── /schema [action]           → Schema status and overview                                     ║
║   ├── /schema:add-node <name>    → Add new node type (Socratic discovery)                         ║
║   ├── /schema:edit-node <name>   → Modify existing node                                           ║
║   └── /schema:add-relation <REL> → Add new relationship                                           ║
║                                                                                                   ║
║   SKILLS (automatic context)                                                                      ║
║   ├── novanet-architecture       → ASCII architecture diagrams (v9 meta-graph + Rust)             ║
║   ├── novanet-sync               → YAML ↔ TypeScript ↔ Mermaid sync (v9 generators)              ║
║   ├── codebase-audit             → Parallel codebase health analysis                              ║
║   └── token-audit                → Design system token adoption                                   ║
║                                                                                                   ║
║   AGENTS (specialized subagents)                                                                  ║
║   ├── neo4j-architect            → Cypher queries, meta-graph design, v9 patterns                 ║
║   └── code-reviewer              → Code quality, security, TS/Rust review                         ║
║                                                                                                   ║
╚═══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

---

## Directory Structure

```
.claude/
├── README.md                    ← This file
├── settings.json                ← Project settings (permissions, env, hooks)
├── settings.local.json          ← Local overrides (gitignored)
├── hooks/                       ← Hook scripts
│   ├── session-start.sh         ← SessionStart: show project status
│   └── post-edit-format.sh      ← PostToolUse: auto-format after edits
├── rules/                       ← Path-specific rules
│   ├── rust.md                  ← Rust patterns (tools/novanet/**/*.rs)
│   ├── typescript.md            ← TypeScript patterns (packages/, apps/)
│   └── cypher.md                ← Cypher patterns (packages/db/seed/)
├── commands/                    ← Slash commands
│   ├── novanet-arch.md          ← /novanet-arch
│   ├── novanet-sync.md          ← /novanet-sync
│   ├── schema.md                ← /schema (master command)
│   ├── schema-add-node.md       ← /schema:add-node
│   ├── schema-edit-node.md      ← /schema:edit-node
│   └── schema-add-relation.md   ← /schema:add-relation
├── skills/                      ← Skill definitions
│   ├── novanet-architecture/
│   │   └── SKILL.md
│   ├── novanet-sync/
│   │   └── SKILL.md
│   ├── codebase-audit/
│   │   └── SKILL.md
│   └── token-audit/
│       └── SKILL.md
└── agents/                      ← Subagent definitions
    ├── neo4j-architect.md
    └── code-reviewer.md

apps/studio/.claude/
├── commands/
│   ├── novanet.md               ← /novanet (session start)
│   └── novanet-bye.md           ← /novanet-bye (session end)
├── rules/
│   ├── novanet-terminology.md   ← Domain vocabulary (v9 meta-graph)
│   └── novanet-decisions.md     ← Architecture decisions (ADR-001 to ADR-014)
└── settings.json

packages/core/.claude/
└── commands/
    └── ontology-audit.md        ← /ontology-audit
```

---

## Hooks

Automated scripts that run at specific lifecycle events.

### SessionStart Hook

**File:** `.claude/hooks/session-start.sh`
**Trigger:** When a Claude Code session starts

**Output:** Shows project version, git branch, and uncommitted changes count.

```
NovaNet v9.0.1 | Branch: main | Uncommitted: 3 files
```

### PostToolUse Hook (Write|Edit)

**File:** `.claude/hooks/post-edit-format.sh`
**Trigger:** After Write or Edit tool completes

**Actions:**
- `.rs` files → `rustfmt` (edition 2021)
- `.ts`, `.tsx`, `.js`, `.jsx`, `.json` → `prettier`

---

## Path-Specific Rules

Rules that apply only when working with matching files.

| Rule File | Paths | Content |
|-----------|-------|---------|
| `rust.md` | `tools/novanet/**/*.rs` | Error handling, async patterns, module structure |
| `typescript.md` | `packages/**/*.ts`, `apps/**/*.tsx` | Type safety, React patterns, v9 terminology |
| `cypher.md` | `packages/db/seed/**/*.cypher` | Meta-graph navigation, EdgeFamily patterns |

Rules use YAML frontmatter with `paths:` field for scoping:

```yaml
---
paths:
  - "tools/novanet/**/*.rs"
---
# Rust rules here...
```

---

## Commands

### `/novanet-arch` - Architecture Diagrams

Display NovaNet architecture in ASCII format.

| Argument | Description |
|----------|-------------|
| `source`, `yaml` | YAML source of truth structure |
| `meta`, `facets` | Meta-Graph (v9 faceted classification) |
| `pipeline`, `sync` | Source of Truth sync pipeline |
| `locale`, `knowledge` | Locale Knowledge node structure (14 types) |
| `infra`, `neo4j` | Infrastructure (Docker, seeds, migrations) |
| `studio` | Studio web app (API routes, stores, NavigationMode) |
| `packages`, `deps` | Packages dependency graph (includes Rust) |
| `flow`, `generation` | LLM generation pipeline |
| `rust`, `cli` | Rust binary architecture (`tools/novanet/`) |
| _(empty)_ | Complete architecture |

**Examples:**
```bash
/novanet-arch              # Full architecture
/novanet-arch meta         # v9 faceted classification
/novanet-arch pipeline     # How YAML propagates to Neo4j
/novanet-arch rust         # Rust binary structure
```

---

### `/novanet-sync` - Schema Synchronization

Validate or regenerate TypeScript/Mermaid from YAML sources.

| Argument | Description |
|----------|-------------|
| `validate`, `status` | Check sync status (default) |
| `generate`, `fix` | Regenerate from YAML |
| `all`, `full` | Validate then generate |

**Examples:**
```bash
/novanet-sync              # Check if files are in sync
/novanet-sync generate     # Regenerate all artifacts
```

**Underlying commands (Rust-first):**
```bash
novanet schema validate                # YAML <-> Neo4j consistency (authoritative)
novanet schema generate                # YAML → all artifacts (layers.ts, Mermaid, Cypher, hierarchy.ts)
novanet doc generate                   # YAML views → 12 Mermaid diagrams (per-view)
novanet doc generate --view=<id>       # Single view diagram
novanet doc generate --list            # List available views with categories
```

---

### `/novanet` - Start Studio Session

Start a development session in `apps/studio`.

**Actions:**
1. Greet and check git status
2. Run `pnpm type-check`
3. Load context from CLAUDE.md
4. Show quick actions menu

---

### `/novanet-bye` - End Session

Clean up and end development session.

**Actions:**
1. Run `type-check` and `lint`
2. Check for uncommitted changes
3. Summarize completed work
4. Sign off

---

### `/ontology-audit` - Ontology Synchronization Audit

Comprehensive audit of YAML → TypeScript → Neo4j synchronization.

**Checks:**
- Mermaid diagrams match YAML
- TypeScript types match YAML
- Neo4j seeds match YAML
- Studio config matches YAML

**Use cases traced:**
1. Block Generation Context
2. Locale Knowledge Context
3. SEO/GEO Pipeline
4. Page Assembly

---

## Schema Management Commands

Commands for editing the NovaNet knowledge graph schema (ontology).

**Source Files:**
- Node YAMLs: `packages/core/models/nodes/{realm}/{layer}/{node-name}.yaml`
- Relations: `packages/core/models/relations.yaml`
- Organizing Principles: `packages/core/models/organizing-principles.yaml`
- Generated Types: `packages/core/src/types/`

### `/schema` - Schema Overview

Master command for schema management.

| Argument | Description |
|----------|-------------|
| `status` | Show schema stats (default) |
| `add-node <name>` | Redirect to /schema:add-node |
| `edit-node <name>` | Redirect to /schema:edit-node |
| `add-relation <NAME>` | Redirect to /schema:add-relation |

**Example:**
```bash
/schema status     # Show current schema stats (44 Kinds, 50 EdgeKinds, 3 Realms, 9 Layers)
```

---

### `/schema:add-node` - Add New Node Type

Add a new node type using Socratic discovery workflow.

**Workflow:**
1. **Discovery** - Ask clarifying questions (realm, layer, trait, purpose, properties, relations)
2. **Validation** - Check for conflicts and nomenclature compliance
3. **Creation** - Create YAML file, update relations.yaml
4. **Sync** - Run `novanet schema generate` + `novanet schema validate`
5. **Seed** - Create migration if needed

**Nomenclature Rules:**
| Pattern | Use For | Examples |
|---------|---------|----------|
| `*L10n` | Localized content | ConceptL10n, PageL10n |
| `Locale*` | Locale knowledge | LocaleVoice, LocaleCulture |
| `*Metrics` | Time-series data | SEOKeywordMetrics |
| `*MiningRun` | Batch operations | SEOMiningRun |

**Example:**
```bash
/schema:add-node LocaleHumor    # Start Socratic dialog for new node
```

---

### `/schema:edit-node` - Edit Existing Node

Modify an existing node type with impact analysis.

**Workflow:**
1. **Load** - Read current YAML definition
2. **Discovery** - Ask what to modify (properties, relations)
3. **Impact Analysis** - Check breaking changes
4. **Modification** - Update YAML and relations
5. **Migration** - Create migration for breaking changes

**Safety Rules:**
- Never remove properties without confirmation
- Always create migrations for breaking changes
- Always validate sync after changes

**Example:**
```bash
/schema:edit-node Concept    # Show current definition, ask what to change
```

---

### `/schema:add-relation` - Add New Relationship

Add a new relationship type between nodes.

**Workflow:**
1. **Discovery** - Ask about from/to Kinds, cardinality, properties, EdgeFamily
2. **Classification** - Assign to EdgeFamily (ownership/localization/semantic/generation/mining)
3. **Bidirectionality** - Check if inverse relation needed
4. **Creation** - Add to relations.yaml with `family` field, update node YAMLs
5. **Sync** - Validate and seed

**Naming Conventions:**
| Pattern | EdgeFamily | Examples |
|---------|-----------|----------|
| `HAS_*` | ownership | HAS_PAGE, HAS_BLOCK, HAS_CONCEPT |
| `HAS_L10N` | localization | Concept→ConceptL10n, Project→ProjectL10n |
| `HAS_OUTPUT` | generation | Page→PageL10n, Block→BlockL10n |
| `*_OF` | ownership (inverse) | L10N_OF, BLOCK_OF, OUTPUT_OF |
| `FOR_*` | localization | FOR_LOCALE |
| `USES_*` | semantic | USES_CONCEPT |
| `SEMANTIC_LINK` | semantic | Concept→Concept |
| `TARGETS_*` | mining | TARGETS_SEO, TARGETS_GEO |

**Example:**
```bash
/schema:add-relation HAS_HUMOR    # Dialog to define new relationship
```

---

## Skills

### `novanet-architecture`

**Trigger:** Questions about architecture, system overview, codebase structure, meta-graph

**Provides:**
- Full architecture ASCII diagram
- v9 Meta-Graph (faceted classification with Realm/Layer/Kind/Trait/EdgeFamily)
- Source of Truth structure
- Pipeline sync diagram (4 generators + Rust validation)
- Locale Knowledge structure
- Infrastructure details
- Package dependencies (includes Rust binary)
- Generation pipeline
- Rust binary architecture (`tools/novanet/`)

---

### `novanet-sync`

**Trigger:** YAML changes, sync validation, schema questions

**Provides:**
- Source of Truth documentation (v9 terminology)
- Generated artifacts mapping (4 generators: Mermaid, Layer, Kind, EdgeSchema)
- Validation commands (TS sync + Rust authoritative)
- CI integration details
- v9 validation section (dual: TS sync check + Rust YAML<->Neo4j)
- Troubleshooting guide

---

### `codebase-audit` (Ralph Wiggum Loop)

**Trigger:** Before releases, after refactoring, periodic maintenance

**Process:**
1. **SCAN** - Launch parallel agents (haiku model)
2. **SYNTHESIZE** - Prioritize findings (CRITICAL → LOW)
3. **FIX** - Apply corrections with tests
4. **VERIFY** - Re-run until clean

**Parallel agents:**
- Dead Code Analysis
- Package.json Audit
- TypeScript Config Audit
- Documentation Accuracy
- Test Health
- Index Exports

---

### `token-audit`

**Trigger:** Design system consistency checks, gap/spacing verification

**Provides:**
- Gap/spacing token adoption analysis
- Non-tokenized pattern detection
- Design system consistency report

**Arguments:**

| Argument | Description |
|----------|-------------|
| `gaps` | Audit gap/spacing tokens only |
| `all` | Full token audit |
| `summary` | Quick summary |

---

## Agents

### `neo4j-architect`

**Model:** sonnet
**Tools:** Read, Grep, Glob, Neo4j MCP

**Specialization:**
- Graph schema design for AI context
- v9 Meta-Graph navigation (Realm/Layer/Kind/Trait/EdgeFamily)
- Efficient Cypher queries (data + meta-graph)
- Performance optimization
- Spreading activation patterns

**Key patterns:**
```cypher
-- v9: Navigate meta-graph taxonomy
MATCH (r:Realm {key: $realm})-[:HAS_LAYER]->(l:Layer)-[:HAS_KIND]->(k:Kind)
RETURN r.key AS realm, l.key AS layer, collect(k.label) AS kinds

-- v9: Full Kind context assembly
MATCH (k:Kind {label: $kindLabel})
MATCH (k)-[:IN_REALM]->(r:Realm)
MATCH (k)-[:IN_LAYER]->(l:Layer)
MATCH (k)-[:HAS_TRAIT]->(t:Trait)
RETURN k.label, k.schema_hint, r.key AS realm, l.key AS layer, t.key AS trait

-- Spreading activation
MATCH (c:Concept {key: $key})-[r:SEMANTIC_LINK*1..2]->(related)
WHERE ALL(rel IN r WHERE rel.temperature >= 0.3)
WITH related, reduce(a = 1.0, rel IN r | a * rel.temperature) AS activation
WHERE activation >= 0.3
RETURN related.key, activation ORDER BY activation DESC
```

---

### `code-reviewer`

**Model:** sonnet
**Tools:** Read, Grep, Glob

**Review focus (7 areas):**
1. **Code Quality (TypeScript)** - Best practices, naming, error handling, no `any`
2. **Code Quality (Rust)** - Ownership, `thiserror`/`color-eyre`, no `.unwrap()`, clippy
3. **Security** - Credentials, injection, XSS
4. **NovaNet Conventions** - Generation NOT translation, imports
5. **v9 Meta-Graph Conventions** - Realm/Layer/Kind terminology, NavigationMode, `:Meta` label
6. **Rust-First Architecture** - Single `novanet` binary for all operations, TS limited to Studio + types
7. **Testing** - Coverage, edge cases, mocks

**Output format:**
```
## Summary
[Overview]

## Issues Found
### Critical
### Warnings
### Suggestions

## Approval Status
[ ] Approved / [ ] Changes requested
```

---

## Rules

Located in `apps/studio/.claude/rules/`:

### `novanet-terminology.md`

Domain vocabulary reference (v9.0.0):
- Core concepts (Project, Concept, Page, Block, Locale, Context Graph)
- Meta-Graph: 6 meta-node types (Realm, Layer, Kind, Trait, EdgeFamily, EdgeKind)
- Full Kind Inventory (44 Kinds across 3 Realms)
- Meta-Graph relations (hierarchy, facets, edge schema, instance bridge)
- Key data relations (grouped by EdgeFamily)
- v8 → v9 rename mapping
- Locale Knowledge structure (14 nodes)
- Standard properties
- Abbreviations

### `novanet-decisions.md`

Architecture Decision Records (ADRs):
- **ADR-001:** 2D Graph Visualization
- **ADR-002:** Filter Presets with Keyboard Shortcuts
- **ADR-003:** AI Chat for Natural Language Queries
- **ADR-004:** Zustand with Persist + Immer
- **ADR-005:** DX-First Component Design
- **ADR-006:** Type Sharing with novanet-core
- **ADR-007:** Glassmorphism UI Theme
- **ADR-008:** Faceted Classification (v9)
- **ADR-009:** Self-Describing Meta-Graph (v9)
- **ADR-010:** CLI-First Architecture (v9) — Rust binary
- **ADR-011:** TS/Rust Boundary Rule (v9)
- **ADR-012:** NavigationMode (v9) — 4 modes
- **ADR-013:** OF_KIND Instance Bridge (v9)
- **ADR-014:** Trait-Based Visual Encoding (v9)

---

## Key Numbers (v9.0.0)

| Metric | Value |
|--------|-------|
| Kind (node types) | 35 |
| EdgeKind (relations) | 50 |
| Realms | 3 (global, project, shared) |
| Layers | 9 |
| Traits | 5 |
| EdgeFamilies | 5 |
| Meta-node total | ~108 |
| Locale Knowledge nodes | 14 |
| Seed files | 7 |
| View definitions | 13 (12 + complete-graph) |
| View doc diagrams | 12 (generated by `novanet doc generate`) |
| Migrations | 6 |
| API routes (Studio) | 10 |
| Zustand stores | 8 |
| Filter presets | 10 |
| Locales supported | 200+ |
| ADRs | 14 |

---

## Workflow Commands

### Schema Sync Pipeline (Rust-first)

```bash
# Validate YAML <-> Neo4j consistency
novanet schema validate

# Regenerate all artifacts from YAML
novanet schema generate

# Generate view-specific Mermaid diagrams
novanet doc generate                   # All 12 views → packages/core/models/docs/*.md
novanet doc generate --view=<id>       # Single view
novanet doc generate --list            # List views with categories

# Seed database
novanet db seed

# Full reset
pnpm infra:down && pnpm infra:up && novanet db seed
```

### Development

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

## Adding New DX Elements

### New Command

1. Create `.claude/commands/<name>.md`
2. Add frontmatter with `description` and optional `argument-hint`
3. Document actions and examples

### New Skill

1. Create `.claude/skills/<name>/SKILL.md`
2. Add frontmatter with `name`, `description`, and optional `user-invocable`
3. Document when to trigger and what to provide

### New Agent

1. Create `.claude/agents/<name>.md`
2. Add frontmatter with `name`, `description`, `tools`, and `model`
3. Document specialization and key patterns

---

## Keeping This Documentation Synced

This README should be updated when:

1. **Commands change** - New commands, renamed, removed
2. **Skills updated** - New sections, new triggers
3. **Agents modified** - New tools, changed prompts
4. **Schema version bumps** - New Kinds, EdgeKinds, Realms, Layers
5. **v9 migration milestones** - Rust binary additions, meta-graph changes

**Validation:**
```bash
# Check if counts match
/novanet-arch              # Should show same numbers as this README
/ontology-audit            # Validates all artifacts match YAML
```

---

## Related Documentation

| File | Purpose |
|------|---------|
| `/CLAUDE.md` | Monorepo overview (v9 migration context) |
| `/packages/core/CLAUDE.md` | Core package (types, schemas, YAML, v9 terminology) |
| `/packages/db/CLAUDE.md` | Database infrastructure |
| `/apps/studio/CLAUDE.md` | Studio application (NavigationMode, visual encoding) |
| `/tools/novanet/` | Rust binary — CLI + TUI + generators (replaces schema-tools) |
| `/docs/plans/2026-02-01-ontology-v9-design.md` | v9 migration plan (complete) |
