# NovaNet Developer Experience (DX)

Claude Code configuration for the NovaNet monorepo.

---

## Quick Reference

```
╔═══════════════════════════════════════════════════════════════════════════════════════════════════╗
║                              NOVANET DX - v8.2.0                                                  ║
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
║   ├── novanet-architecture       → ASCII architecture diagrams                                    ║
║   ├── novanet-sync               → YAML ↔ TypeScript ↔ Mermaid sync                               ║
║   └── codebase-audit             → Parallel codebase health analysis                              ║
║                                                                                                   ║
║   AGENTS (specialized subagents)                                                                  ║
║   ├── neo4j-architect            → Cypher queries, schema design                                  ║
║   └── code-reviewer              → Code quality, security review                                  ║
║                                                                                                   ║
╚═══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

---

## Directory Structure

```
.claude/
├── README.md                    ← This file
├── settings.json                ← Project settings
├── settings.local.json          ← Local overrides (gitignored)
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
│   └── codebase-audit/
│       └── SKILL.md
└── agents/                      ← Subagent definitions
    ├── neo4j-architect.md
    └── code-reviewer.md

apps/studio/.claude/
├── commands/
│   ├── novanet.md               ← /novanet (session start)
│   └── novanet-bye.md           ← /novanet-bye (session end)
├── rules/
│   ├── novanet-terminology.md   ← Domain vocabulary
│   └── novanet-decisions.md     ← Architecture decisions (ADRs)
└── settings.json

packages/core/.claude/
└── commands/
    └── ontology-audit.md        ← /ontology-audit
```

---

## Commands

### `/novanet-arch` - Architecture Diagrams

Display NovaNet architecture in ASCII format.

| Argument | Description |
|----------|-------------|
| `source`, `yaml` | YAML source of truth structure |
| `pipeline`, `sync` | Source of Truth sync pipeline |
| `locale`, `knowledge` | Locale Knowledge node structure (14 types) |
| `infra`, `neo4j` | Infrastructure (Docker, seeds, migrations) |
| `studio` | Studio web app (API routes, stores) |
| `packages`, `deps` | Packages dependency graph |
| `flow`, `generation` | LLM generation pipeline |
| _(empty)_ | Complete architecture |

**Examples:**
```bash
/novanet-arch              # Full architecture
/novanet-arch pipeline     # How YAML propagates to Neo4j
/novanet-arch locale       # Locale Knowledge structure
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

**Underlying commands:**
```bash
pnpm schema:validate       # CI validation
pnpm schema:generate       # Regenerate files
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
- Node YAMLs: `packages/core/models/nodes/{scope}/{subcategory}/{node-name}.yaml`
- Relations: `packages/core/models/relations.yaml`
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
/schema status     # Show current schema stats (35 nodes, 47 relations)
```

---

### `/schema:add-node` - Add New Node Type

Add a new node type using Socratic discovery workflow.

**Workflow:**
1. **Discovery** - Ask clarifying questions (scope, subcategory, purpose, properties, relations)
2. **Validation** - Check for conflicts and nomenclature compliance
3. **Creation** - Create YAML file, update relations.yaml
4. **Sync** - Run `pnpm schema:generate` + `pnpm schema:validate`
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
1. **Discovery** - Ask about from/to nodes, cardinality, properties
2. **Classification** - Determine if semantic or auxiliary
3. **Bidirectionality** - Check if inverse relation needed
4. **Creation** - Add to relations.yaml, update node YAMLs
5. **Sync** - Validate and seed

**Naming Conventions:**
| Pattern | Use For | Examples |
|---------|---------|----------|
| `HAS_*` | Ownership/containment | HAS_PAGE, HAS_BLOCK, HAS_CONCEPT |
| `HAS_L10N` | Localized content (human-curated) | Concept→ConceptL10n, Project→ProjectL10n |
| `HAS_OUTPUT` | Localized content (LLM-generated) | Page→PageL10n, Block→BlockL10n |
| `*_OF` | Inverse of HAS_* | L10N_OF, BLOCK_OF, OUTPUT_OF |
| `FOR_*` | Target association | FOR_LOCALE |
| `USES_*` | Reference/usage | USES_CONCEPT |
| `TARGETS_*` | Cross-scope targeting | TARGETS_SEO, TARGETS_GEO |

**Example:**
```bash
/schema:add-relation HAS_HUMOR    # Dialog to define new relationship
```

---

## Skills

### `novanet-architecture`

**Trigger:** Questions about architecture, system overview, codebase structure

**Provides:**
- Full architecture ASCII diagram
- Source of Truth structure
- Pipeline sync diagram
- Locale Knowledge structure
- Infrastructure details
- Package dependencies
- Generation pipeline

---

### `novanet-sync`

**Trigger:** YAML changes, sync validation, schema questions

**Provides:**
- Source of Truth documentation
- Generated artifacts mapping
- Validation commands
- CI integration details
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

## Agents

### `neo4j-architect`

**Model:** sonnet
**Tools:** Read, Grep, Glob, Neo4j MCP

**Specialization:**
- Graph schema design for AI context
- Efficient Cypher queries
- Performance optimization
- Spreading activation patterns

**Key patterns:**
```cypher
-- Spreading activation
MATCH (c:Concept {key: $key})-[r:SEMANTIC_LINK*1..2]->(related)
WHERE ALL(rel IN r WHERE rel.temperature >= 0.3)
WITH related, reduce(a = 1.0, rel IN r | a * rel.temperature) AS activation
WHERE activation >= 0.3
RETURN related.key, activation ORDER BY activation DESC

-- Context loading
MATCH (b:Block {key: $blockKey})
MATCH (b)-[:USES_CONCEPT]->(c:Concept)-[:HAS_L10N]->(cl:ConceptL10n)-[:FOR_LOCALE]->(l:Locale {key: $locale})
RETURN b.instructions, c.key, cl.title
```

---

### `code-reviewer`

**Model:** sonnet
**Tools:** Read, Grep, Glob

**Review focus:**
1. **Code Quality** - TypeScript best practices, naming, error handling
2. **Security** - Credentials, injection, XSS
3. **NovaNet Conventions** - Generation NOT translation, imports
4. **Testing** - Coverage, edge cases, mocks

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

Domain vocabulary reference:
- Core concepts (Project, Concept, Page, Block, Locale)
- Node scopes (35 nodes across 3 scopes)
- Localization patterns
- Locale Knowledge structure (14 nodes)
- Standard properties
- Key relationships
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

---

## Key Numbers (v8.2.0)

| Metric | Value |
|--------|-------|
| Node types | 35 |
| Relations | 47 |
| Scopes | 3 (Global, Shared, Project) |
| Locale Knowledge nodes | 14 |
| Seed files | 7 |
| Migrations | 6 |
| API routes | 9 |
| Zustand stores | 8 |
| Filter presets | 10 |
| Locales supported | 200+ |

---

## Workflow Commands

### Schema Sync Pipeline

```bash
# Validate (CI check)
pnpm schema:validate

# Regenerate from YAML
pnpm schema:generate

# Full reset
pnpm infra:reset
```

### Development

```bash
# Start Neo4j + seed
pnpm infra:up && pnpm infra:seed

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
4. **Schema version bumps** - New node types, relations, metrics

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
| `/CLAUDE.md` | Monorepo overview |
| `/packages/core/CLAUDE.md` | Core package (types, schemas, YAML) |
| `/packages/db/CLAUDE.md` | Database infrastructure |
| `/apps/studio/CLAUDE.md` | Studio application |
| `/packages/schema-tools/CLAUDE.md` | Schema validation tools |
