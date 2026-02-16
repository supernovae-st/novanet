# NovaNet Developer Experience (DX)

Claude Code configuration for the NovaNet monorepo.

**Version**: v0.13.0 | **Docs**: [Claude Code Official](https://docs.anthropic.com/en/docs/claude-code)

---

## Quick Reference

```
╔═══════════════════════════════════════════════════════════════════════════════════════════════════╗
║                              NOVANET DX - v0.13.0                                                  ║
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
║   └── /schema:add-arc <ARC>      → Add new arc type (relationship)                                ║
║                                                                                                   ║
║   SKILLS (automatic context)                                                                      ║
║   ├── novanet-architecture       → ASCII architecture diagrams (v9 meta-graph + Rust)             ║
║   ├── novanet-sync               → YAML ↔ TypeScript ↔ Mermaid sync (v9 generators)              ║
║   ├── novanet-tui                → Galaxy-themed terminal UI (keybindings, navigation)            ║
║   ├── security-audit             → Comprehensive security audit (Rust + TypeScript)               ║
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

## v11.7 Unified Tree Architecture

**ADR-022** introduces the Unified Tree Architecture, simplifying TUI/Studio navigation.

### Key Changes

| Aspect | Before (v11.6) | After (v11.7) |
|--------|----------------|---------------|
| Header tabs | 5 (Meta/Data/Overlay/Query/Atlas) | 2 (Graph/Nexus) |
| Realm/Layer | Visual groupings | Clickable nodes |
| Instances | Hidden or separate Data mode | Under Class, expandable |
| Atlas | Separate mode | Removed |
| Audit | In Atlas | In Nexus hub |
| Icons | Mixed emoji | Dual: Lucide (web) + Unicode (terminal) |

### Principle

**"If it's a node in Neo4j, it's a node everywhere"**

Realm, Layer, ArcFamily, ArcClass are all `:Schema:*` nodes in Neo4j. v11.7 makes them clickable everywhere.

### Navigation Modes

```
[1]Graph   Unified tree: Realm > Layer > Class > Instance + Arcs
[2]Nexus   Hub: Quiz, Audit, Stats, Help
[/]        Search overlay (replaces Query mode)
```

### v11.7 Implementation Skills

When implementing v11.7 Unified Tree Architecture, use these skills per phase:

| Phase | Skills | Agents |
|-------|--------|--------|
| Neo4j Migration | - | `neo4j-architect` |
| YAML Updates | `spn-writing:mermaid` | - |
| Type Definitions | `rust-core` | `rust-architect` |
| Generators | `rust-core` | `rust-pro` |
| TUI (Rust) | `rust-async`, `test-driven-development` | `rust-async-expert` |
| Studio (TS) | - | `feature-dev:code-architect` |
| Testing | `testing-anti-patterns` | `feature-dev:code-reviewer` |

### Pre-Implementation Checklist (v11.7)

```
- [ ] Neo4j running (`pnpm infra:up`)
- [ ] Schema seeded (`pnpm infra:seed`)
- [ ] Tests pass (`cargo nextest run && pnpm test`)
- [ ] Clean git status
- [ ] Create worktree (`/spn-powers:using-git-worktrees`)
```

### Reference

- Design: `docs/plans/2026-02-11-unified-tree-design.md`
- ADR-022: Unified Tree Architecture

---

## Directory Structure

```
.claude/
├── README.md                    ← This file
├── settings.json                ← Project settings (permissions, env, hooks)
├── settings.local.json          ← Local overrides (gitignored)
├── hooks/                       ← Hook scripts (7 active, see settings.json)
│   ├── session-start.sh         ← SessionStart: show project status
│   ├── post-edit-format.sh      ← PostToolUse: auto-format after edits
│   ├── keybindings-reminder.sh  ← TUI file edit reminder
│   ├── yaml-sync-reminder.sh    ← YAML model edit reminder
│   ├── doc-sync-reminder.sh     ← Documentation edit reminder
│   ├── semantic-check.sh        ← Rust code semantic validation
│   ├── adr-context.sh           ← ADR context loading
│   └── archive/                 ← Archived hooks (not active)
├── rules/                       ← Path-specific rules
│   ├── rust.md                  ← Rust patterns (tools/novanet/**/*.rs)
│   ├── typescript.md            ← TypeScript patterns (packages/, apps/)
│   ├── cypher.md                ← Cypher patterns (packages/db/seed/)
│   ├── security.md              ← Security patterns (all code)
│   ├── novanet-terminology.md   ← Domain vocabulary (v0.13.0)
│   ├── novanet-decisions.md     ← ADR index (quick reference)
│   ├── arc-design-guide.md      ← Arc design best practices
│   └── adr/                     ← Full ADR content by domain
│       ├── _index.yaml          ← Machine-readable ADR metadata
│       ├── core-principles/     ← ADRs 001, 003, 007, 010, 021
│       ├── schema-architecture/ ← ADRs 006, 012, 017, 028, 029, 030
│       ├── node-classification/ ← ADRs 002, 023, 024, 025
│       ├── arc-design/          ← ADRs 015, 016, 026, 027
│       ├── visual-encoding/     ← ADRs 004, 005, 009, 013
│       ├── ux-architecture/     ← ADRs 008, 022
│       ├── seo-geo/             ← ADRs 031, 032
│       └── deprecated/          ← ADRs 011, 014, 018, 019, 020
├── commands/                    ← Slash commands
│   ├── novanet-arch.md          ← /novanet-arch
│   ├── novanet-sync.md          ← /novanet-sync
│   ├── schema.md                ← /schema (master command)
│   ├── schema-add-node.md       ← /schema:add-node
│   ├── schema-edit-node.md      ← /schema:edit-node
│   ├── schema-add-arc.md        ← /schema:add-arc
│   └── adr.md                   ← /adr (ADR quick lookup)
├── skills/                      ← Skill definitions
│   ├── novanet-architecture/    ← ASCII architecture diagrams
│   ├── novanet-sync/            ← Schema sync validation
│   ├── novanet-tui/             ← Terminal UI keybindings
│   ├── novanet-adr/             ← ADR navigation and lookup
│   ├── security-audit/          ← Security audit (Rust + TS)
│   ├── codebase-audit/          ← Parallel codebase analysis
│   └── token-audit/             ← Design token adoption
└── agents/                      ← Subagent definitions
    ├── neo4j-architect.md
    └── code-reviewer.md

apps/studio/.claude/
├── commands/
│   ├── novanet.md               ← /novanet (session start)
│   └── novanet-bye.md           ← /novanet-bye (session end)
├── skills/                      ← 10 Studio-specific skills
│   ├── force-graph-patterns.md
│   ├── react-flow-patterns.md
│   ├── zustand-patterns.md
│   ├── radix-ui-patterns.md
│   └── ... (6 more)
└── settings.json

packages/core/.claude/
├── commands/
│   └── ontology-audit.md        ← /ontology-audit
└── skills/                      ← 4 Core-specific skills
    ├── context-graph-architect.md
    ├── neo4j-expert.md
    ├── spreading-activation.md
    └── dev-environment.md
```

---

## Hooks

Automated scripts at lifecycle events. See `.claude/settings.json` for configuration.

| Event | Hook | Purpose |
|-------|------|---------|
| SessionStart | `session-start.sh` | Show project status (version, branch, changes) |
| PostToolUse | `post-edit-format.sh` | Auto-format `.rs`/`.ts` after edits |
| PostToolUse | `keybindings-reminder.sh` | Remind on TUI file edits |
| PostToolUse | `yaml-sync-reminder.sh` | Remind to regenerate after YAML edits |
| PostToolUse | `doc-sync-reminder.sh` | Remind on documentation edits |
| PostToolUse | `semantic-check.sh` | Validate Rust code semantics |
| PostToolUse | `adr-context.sh` | Load ADR context when reading ADRs |

Archived hooks are in `.claude/hooks/archive/`.

---

## Path-Specific Rules

Rules that apply only when working with matching files.

| Rule File | Paths | Content |
|-----------|-------|---------|
| `rust.md` | `tools/novanet/**/*.rs` | Error handling, async patterns, module structure |
| `typescript.md` | `packages/**/*.ts`, `apps/**/*.tsx` | Type safety, React patterns, v0.13.0 terminology |
| `cypher.md` | `packages/db/seed/**/*.cypher` | Schema-graph navigation, ArcFamily patterns |
| `security.md` | `**/*.rs`, `**/*.ts`, `**/*.cypher` | Security patterns, pre-commit checklist |
| `novanet-terminology.md` | All files | v0.13.0 domain vocabulary |
| `novanet-decisions.md` | All files | ADR index (quick reference) |
| `arc-design-guide.md` | All files | Arc design best practices |
| `adr/*.md` | All files | Full ADR content by domain (32 ADRs) |

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
| `studio` | Studio web app (API routes, stores, Graph/Nexus modes) |
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

### `/adr` - ADR Quick Lookup

Look up Architecture Decision Records by number, keyword, or domain.

| Argument | Description |
|----------|-------------|
| `<number>` | Look up by ADR number (e.g., `029`, `21`) |
| `<keyword>` | Search by keyword (e.g., `native`, `slug`, `trait`) |
| `list` | List all ADRs with status |
| `domain <name>` | List ADRs in a domain (e.g., `schema-architecture`) |
| `must-know` | Show essential ADRs for v0.13.0 |

**Examples:**
```bash
/adr 029               # Show ADR-029 (*Native Pattern)
/adr native            # Find ADRs containing "native"
/adr list              # List all 32 ADRs
/adr domain arc-design # List ADRs in arc-design domain
/adr must-know         # Show 6 essential ADRs for v0.13.0
```

**ADR Domains:**
- `core-principles` (5): Philosophy and methodology
- `schema-architecture` (6): Realm, layer, node organization
- `node-classification` (4): Naming, traits, classification
- `arc-design` (4): Arc families, inverses, patterns
- `visual-encoding` (4): Colors, icons, representation
- `ux-architecture` (2): TUI/Studio navigation
- `seo-geo` (2): SEO pillar/cluster, URL architecture
- `deprecated` (5): Historical/superseded ADRs

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
| `add-arc <NAME>` | Redirect to /schema:add-arc |

**Example:**
```bash
/schema status     # Show current schema stats (61 Classes, 169 ArcClasses, 2 Realms, 10 Layers)
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
| `*L10n` | Localized content | EntityContent, PageGenerated |
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
/schema:edit-node Entity    # Show current definition, ask what to change
```

---

### `/schema:add-arc` - Add New Arc Type

Add a new arc type between nodes.

**Workflow:**
1. **Discovery** - Ask about from/to Kinds, cardinality, properties, ArcFamily
2. **Classification** - Assign to ArcFamily (ownership/localization/semantic/generation/mining)
3. **Bidirectionality** - Check if inverse arc needed
4. **Creation** - Add to arc-classes/ with `family` field, update node YAMLs
5. **Sync** - Validate and seed

**Naming Conventions:**
| Pattern | ArcFamily | Examples |
|---------|-----------|----------|
| `HAS_*` | ownership | HAS_PAGE, HAS_BLOCK |
| `HAS_CONTENT` | localization | Entity→EntityContent, Project→ProjectContent |
| `HAS_GENERATED` | generation | Page→PageGenerated, Block→BlockGenerated |
| `*_OF` | ownership (inverse) | L10N_OF, BLOCK_OF, OUTPUT_OF |
| `FOR_*` | localization | FOR_LOCALE |
| `USES_*` | semantic | USES_ENTITY |
| `SEMANTIC_LINK` | semantic | Entity→Entity |
| `EXPRESSES` | mining | EntityContent→SEOKeyword |

**Example:**
```bash
/schema:add-arc HAS_HUMOR    # Dialog to define new arc type
```

---

## Skills

### `novanet-architecture`

**Trigger:** Questions about architecture, system overview, codebase structure, meta-graph

**Provides:**
- Full architecture ASCII diagram
- v9 Meta-Graph (faceted classification with Realm/Layer/Kind/Trait/ArcFamily)
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

### `novanet-tui`

**Trigger:** TUI launch, keybindings questions, terminal UI navigation

**Provides:**
- Launch command (`cargo run -- tui`)
- Keybindings reference (navigation, Graph/Nexus modes, scrolling, overlays)
- Visual features (Galaxy theme, boot animation, effects engine)
- v11.7 Unified Tree (Realm/Layer/Kind/Instance as clickable nodes)
- Troubleshooting guide

**Arguments:**

| Argument | Description |
|----------|-------------|
| _(empty)_ | Launch TUI |
| `help`, `keys` | Show keybindings reference |
| `features` | Show visual features overview |

**v11.7 Navigation:**
```
[1]Graph   Unified tree with all nodes clickable
[2]Nexus   Hub (Quiz, Audit, Stats, Help)
[/]        Search overlay
```

---

### `security-audit`

**Trigger:** Security checks, dependency audits, vulnerability scanning

**Provides:**
- Rust audit (cargo-deny, cargo-audit, cargo-machete)
- TypeScript audit (pnpm audit, code patterns)
- CI security checks verification
- Exception review and management

**Arguments:**

| Argument | Description |
|----------|-------------|
| `rust` | Audit Rust dependencies only |
| `typescript` | Audit TypeScript dependencies only |
| `all` | Full audit (default) |
| `exceptions` | List security exceptions |

---

### `codebase-audit` (Ralph Wiggum Loop)

**Trigger:** Before releases, after refactoring, periodic maintenance

**Invocation:** `/codebase-audit [mode] [--fix]`

**Process:**
1. **SCAN** - Launch 10 parallel agents (haiku model)
2. **SYNTHESIZE** - Prioritize findings (CRITICAL → LOW)
3. **FIX** - Apply corrections with tests
4. **VERIFY** - Re-run until clean

**10 Parallel Agents:**
1. YAML Schema Validation
2. Generated Artifacts Sync
3. Rust Code Quality
4. Test Coverage Analysis
5. Documentation Freshness
6. Dependency Audit
7. Performance Patterns
8. Security Patterns
9. Dead Code Detection
10. Semantic Coherence

**Arguments:**

| Mode | Agents | Description |
|------|--------|-------------|
| `full` | 10 | Complete audit (default) |
| `quick` | 4 | Essential checks only |
| `yaml` | 2 | YAML schema + sync |
| `rust` | 2 | Rust quality + tests |
| `typescript` | 2 | TypeScript + dead code |
| `security` | 2 | Security + deps |
| `docs` | 2 | Documentation accuracy |
| `--fix` | - | Auto-fix issues (append to any mode) |

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
- v0.12.0 Schema-Graph navigation (Realm/Layer/Class/Trait/ArcFamily)
- Efficient Cypher queries (data + schema-graph)
- Performance optimization
- Spreading activation patterns

**Key patterns:**
```cypher
-- v0.12.0: Navigate schema-graph taxonomy
MATCH (r:Realm {key: $realm})-[:HAS_LAYER]->(l:Layer)-[:HAS_CLASS]->(c:Class)
RETURN r.key AS realm, l.key AS layer, collect(c.label) AS classes

-- v0.12.0: Full Class context assembly
MATCH (c:Class {label: $classLabel})
MATCH (c)-[:IN_REALM]->(r:Realm)
MATCH (c)-[:IN_LAYER]->(l:Layer)
MATCH (c)-[:HAS_TRAIT]->(t:Trait)
RETURN c.label, c.schema_hint, r.key AS realm, l.key AS layer, t.key AS trait

-- Spreading activation
MATCH (e:Entity {key: $key})-[r:SEMANTIC_LINK*1..2]->(related)
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
5. **v9 Meta-Graph Conventions** - Realm/Layer/Kind terminology, Graph/Nexus modes, `:Meta` label
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

Domain vocabulary reference (v11.0.0):
- Core concepts (Project, Entity, Page, Block, Locale, Context Graph)
- Schema Graph: 6 schema node types (Realm, Layer, Class, Trait, ArcFamily, ArcClass)
- Full Class Inventory (64 Classes across 2 Realms)
- Schema Graph relations (hierarchy, facets, arc schema, instance bridge)
- Key data relations (grouped by ArcFamily)
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
- **ADR-012:** NavigationMode (v9) — 4 modes (superseded by ADR-022)
- **ADR-013:** OF_KIND Instance Bridge (v9)
- **ADR-014:** Trait-Based Visual Encoding (v9)
- **ADR-020:** Schema Refinement (v11.5)
- **ADR-021:** Query-First Architecture (v11.6)
- **ADR-022:** Unified Tree Architecture (v11.7) — 2 modes (Graph/Nexus)

---

## Key Numbers (v0.13.0)

| Metric | Value |
|--------|-------|
| Class (node types) | 61 |
| ArcClass (relations) | 156 |
| Realms | 2 (shared, org) |
| Layers | 10 (4 shared + 6 org) |
| Traits | 5 |
| ArcFamilies | 5 |
| Shared nodes | 40 (config + locale + geography + knowledge) |
| Org nodes | 21 |
| Seed files | 11 |
| View definitions | 29 (13 contextual + 16 global) |
| View doc diagrams | 11 (generated by `novanet doc generate`) |
| Migrations | 6 |
| API routes (Studio) | 10 |
| Zustand stores | 8 |
| Filter presets | 10 |
| Locales supported | 200+ |
| ADRs | 28 |
| Rust tests | 1031 |
| TUI Modes | 2 (Graph, Nexus) |

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

### Documentation Maintenance

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

**Source of truth:** `/VERSION` file contains the canonical schema version (0.12.5).

**`pnpm doc:audit` checks:**
- Outdated version references (current: v0.13.0)
- Deprecated terminology (EntityL10n → EntityContent, PageL10n → PageGenerated, BlockL10n → BlockGenerated)
- Deprecated arcs (HAS_L10N → HAS_CONTENT, HAS_OUTPUT → HAS_GENERATED)
- Incorrect node/arc counts (61 nodes, 169 arcs expected)
- Outdated realm names (global → shared, tenant → org)
- Outdated layer structure (4 shared + 6 org = 10 layers)
- Deprecated 5-mode navigation (use 2-mode: Graph/Nexus)

**`pnpm skill:audit` checks:**
- Deprecated paths (`nodes/` → `node-classes/`, `organizing-principles.yaml` → `taxonomy.yaml`)
- Node/arc counts match YAML sources (61 node-classes, 169 arc-classes)
- Deprecated terminology in skills/commands/rules

**Auto-sync reminders:**
Claude Code hooks automatically remind you when:
- YAML model files are edited → regenerate artifacts
- Documentation files are edited → verify consistency
- Skills/commands/rules are edited → verify YAML alignment
- YAML is read → remember it's the source of truth

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
4. **Schema version bumps** - New Classes, ArcClasses, Realms, Layers
5. **v9 migration milestones** - Rust binary additions, schema graph changes

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
| `/CLAUDE.md` | Monorepo overview (v11.7 Unified Tree context) |
| `/packages/core/CLAUDE.md` | Core package (types, schemas, YAML, v11.7 terminology) |
| `/packages/db/CLAUDE.md` | Database infrastructure |
| `/apps/studio/CLAUDE.md` | Studio application (Graph/Nexus modes, visual encoding) |
| `/tools/novanet/` | Rust binary — CLI + TUI + generators (replaces schema-tools) |
| `/docs/plans/2026-02-01-ontology-v9-design.md` | v9 migration plan (complete) |
| `/docs/plans/2026-02-11-unified-tree-design.md` | v11.7 Unified Tree Architecture (ADR-022) |
