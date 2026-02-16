# NovaNet Slash Commands

Detailed documentation for all slash commands. See [README.md](./README.md) for overview.

---

## `/novanet-arch` - Architecture Diagrams

Display NovaNet architecture in ASCII format.

| Argument | Description |
|----------|-------------|
| `source`, `yaml` | YAML source of truth structure |
| `meta`, `facets` | Schema-Graph (v0.13.0 faceted classification) |
| `pipeline`, `sync` | Source of Truth sync pipeline |
| `locale`, `knowledge` | Locale Knowledge node structure |
| `infra`, `neo4j` | Infrastructure (Docker, seeds, migrations) |
| `studio` | Studio web app (API routes, stores, Graph/Nexus modes) |
| `packages`, `deps` | Packages dependency graph (includes Rust) |
| `flow`, `generation` | LLM generation pipeline |
| `rust`, `cli` | Rust binary architecture (`tools/novanet/`) |
| _(empty)_ | Complete architecture |

**Examples:**
```bash
/novanet-arch              # Full architecture
/novanet-arch meta         # v0.13.0 faceted classification
/novanet-arch pipeline     # How YAML propagates to Neo4j
/novanet-arch rust         # Rust binary structure
```

---

## `/novanet-sync` - Schema Synchronization

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
novanet schema validate                # YAML <-> Neo4j consistency
novanet schema generate                # YAML → all artifacts
novanet doc generate                   # YAML views → Mermaid diagrams
novanet doc generate --view=<id>       # Single view diagram
novanet doc generate --list            # List available views
```

---

## `/novanet` - Start Studio Session

Start a development session in `apps/studio`.

**Actions:**
1. Greet and check git status
2. Run `pnpm type-check`
3. Load context from CLAUDE.md
4. Show quick actions menu

---

## `/novanet-bye` - End Session

Clean up and end development session.

**Actions:**
1. Run `type-check` and `lint`
2. Check for uncommitted changes
3. Summarize completed work
4. Sign off

---

## `/adr` - ADR Quick Lookup

Look up Architecture Decision Records by number, keyword, or domain.

| Argument | Description |
|----------|-------------|
| `<number>` | Look up by ADR number (e.g., `029`, `21`) |
| `<keyword>` | Search by keyword (e.g., `native`, `slug`, `trait`) |
| `list` | List all ADRs with status |
| `domain <name>` | List ADRs in a domain |
| `must-know` | Show essential ADRs for v0.13.0 |

**Examples:**
```bash
/adr 029               # Show ADR-029 (*Native Pattern)
/adr native            # Find ADRs containing "native"
/adr list              # List all 32 ADRs
/adr domain arc-design # List ADRs in arc-design domain
/adr must-know         # Show 6 essential ADRs for v0.13.0
```

**ADR Domains:** core-principles (5), schema-architecture (6), node-classification (4), arc-design (4), visual-encoding (4), ux-architecture (2), seo-geo (2), deprecated (5)

---

## `/ontology-audit` - Ontology Synchronization Audit

Comprehensive audit of YAML → TypeScript → Neo4j synchronization.

**Checks:**
- Mermaid diagrams match YAML
- TypeScript types match YAML
- Neo4j seeds match YAML
- Studio config matches YAML

**Use cases traced:** Block Generation, Locale Knowledge, SEO/GEO Pipeline, Page Assembly

---

## Schema Management Commands

Commands for editing the NovaNet knowledge graph schema (ontology).

**Source Files:**
- Node YAMLs: `packages/core/models/node-classes/{realm}/{layer}/{name}.yaml`
- Arc YAMLs: `packages/core/models/arc-classes/{family}/{name}.yaml`
- Taxonomy: `packages/core/models/taxonomy.yaml`

### `/schema` - Schema Overview

Master command for schema management.

| Argument | Description |
|----------|-------------|
| `status` | Show schema stats (default) |
| `add-node <name>` | Redirect to /schema:add-node |
| `edit-node <name>` | Redirect to /schema:edit-node |
| `add-arc <NAME>` | Redirect to /schema:add-arc |

```bash
/schema status     # 61 Classes, 169 ArcClasses, 2 Realms, 10 Layers
```

### `/schema:add-node` - Add New Node Type

Add a new node type using Socratic discovery workflow.

**Workflow:**
1. **Discovery** - Ask clarifying questions (realm, layer, trait, purpose)
2. **Validation** - Check conflicts and nomenclature compliance
3. **Creation** - Create YAML file
4. **Sync** - Run `novanet schema generate` + `validate`
5. **Seed** - Create migration if needed

**Nomenclature:**

| Pattern | Use For | Examples |
|---------|---------|----------|
| `*Native` | Locale-specific content | EntityNative, PageNative |
| `*Category` | Categorical grouping | EntityCategory |
| `*Set` | Container grouping atoms | TermSet, SEOKeywordSet |
| `*Metrics` | Time-series data | SEOKeywordMetrics |

### `/schema:edit-node` - Edit Existing Node

Modify an existing node type with impact analysis.

**Workflow:**
1. **Load** - Read current YAML definition
2. **Discovery** - Ask what to modify
3. **Impact Analysis** - Check breaking changes
4. **Modification** - Update YAML
5. **Migration** - Create migration for breaking changes

### `/schema:add-arc` - Add New Arc Type

Add a new arc type between nodes.

**Workflow:**
1. **Discovery** - Ask about from/to Classes, cardinality, ArcFamily
2. **Classification** - Assign to ArcFamily
3. **Bidirectionality** - Check if inverse needed
4. **Creation** - Add to arc-classes/
5. **Sync** - Validate and seed

**Naming Conventions:**

| Pattern | ArcFamily | Examples |
|---------|-----------|----------|
| `HAS_*` | ownership | HAS_PAGE, HAS_BLOCK |
| `HAS_NATIVE` | localization | Entity→EntityNative |
| `*_OF` | ownership (inverse) | NATIVE_OF, BLOCK_OF |
| `FOR_*` | localization | FOR_LOCALE |
| `USES_*` | semantic | USES_ENTITY |
