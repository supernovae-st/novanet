# NovaNet v0.20 — Exhaustive Implementation Plan

**Based on**: Master Plan (A+) + 6 Swarm Audits (164+89+449+100+1530 files)
**10 Decisions**: ALL CONFIRMED by Thibaut
**Philosophy**: Zero backward compat. Delete all legacy. Update ALL docs.

---

## Decisions Summary (All Confirmed)

| # | Decision | Detail |
|---|----------|--------|
| D1 | Single v0.20 release | All 3 pillars ship together |
| D2 | `content` = plain WHAT+HOW | No relations, no structured format |
| D3 | `triggers[]` = always English | Graph-wide lingua franca, even on *Native |
| D4 | `novanet_search` modes | `find` / `walk` / `hybrid` / `triggers` |
| D5 | `novanet_context` modes | `page` / `block` / `knowledge` / `assemble` |
| D6 | `novanet_write` absorbs check | auto-check + `dry_run`, `novanet_check` DELETED |
| D7 | Cache tools DELETED | `novanet_cache_stats` + `novanet_cache_invalidate` gone |
| D8 | Evidence V2 shorter names | `source_key`->`key`, `source_kind`->`class`, etc. |
| D9 | Trigger-boost | `(1/d) * (1 + 0.5 * overlap)`, cap 1.5x, hardcoded |
| D10 | V0 clean break | Zero backward compat |

**Tools after v0.20**: `novanet_query`, `novanet_describe`, `novanet_search`, `novanet_introspect`, `novanet_context`, `novanet_write`, `novanet_audit`, `novanet_batch`

---

## Phase A: YAML Schema Migration (Pillar 1)

### A.1 — Update property template

**File**: `packages/core/models/_schema-properties-template.yaml`

**Change**: Remove `llm_context` property definition. Add `triggers` property definition.

```yaml
# Remove:
llm_context:
  type: string
  required: true
  description: "ADR-027 USE/TRIGGERS/NOT/RELATES"

# Add:
triggers:
  type: string[]
  required: true
  description: "Machine-readable routing keywords (max 10, lowercase, English)"
```

### A.2 — Write YAML migration script

**File**: NEW `scripts/migrate-yaml-v020.py`

**Logic**:
1. Walk `packages/core/models/node-classes/**/*.yaml` (61 files)
2. Walk `packages/core/models/arc-classes/**/*.yaml` (151 files)
3. For each file:
   - Parse `llm_context` value (USE/TRIGGERS/NOT/RELATES pattern)
   - Extract triggers from TRIGGERS line -> `triggers[]` array
   - Append USE/NOT guidance to `content` field
   - Remove RELATES (now implicit in graph)
   - Delete `llm_context` field
   - Write `triggers` field as string array
4. Walk other YAML files that reference `llm_context` (taxonomy.yaml, _index.yaml, etc.)

### A.3 — Run migration on 255 YAML files

**Node class files** (61 files):

| Directory | Files | Example |
|-----------|-------|---------|
| `packages/core/models/node-classes/shared/config/` | 3 | `locale.yaml`, `entity-category.yaml`, `seo-keyword-format.yaml` |
| `packages/core/models/node-classes/shared/locale/` | 5 | `locale-voice.yaml`, `locale-culture.yaml`, etc. |
| `packages/core/models/node-classes/shared/geography/` | 7 | `continent.yaml`, `country.yaml`, etc. |
| `packages/core/models/node-classes/shared/knowledge/` | 21 | `expression.yaml`, `pattern.yaml`, `seo-keyword.yaml`, etc. |
| `packages/core/models/node-classes/org/config/` | 1 | `org-config.yaml` |
| `packages/core/models/node-classes/org/foundation/` | 8 | `project.yaml`, `brand.yaml`, etc. |
| `packages/core/models/node-classes/org/structure/` | 3 | `page.yaml`, `block.yaml`, `content-slot.yaml` |
| `packages/core/models/node-classes/org/semantic/` | 2 | `entity.yaml`, `entity-native.yaml` |
| `packages/core/models/node-classes/org/instruction/` | 4 | `block-type.yaml`, `block-rules.yaml`, etc. |
| `packages/core/models/node-classes/org/output/` | 6 | `page-native.yaml`, `block-native.yaml`, etc. |

**Arc class files** (151 files):
- `packages/core/models/arc-classes/**/*.yaml` — all 151 files

**Other YAML files** (~43 files):
- `packages/core/models/_index.yaml` — property listing
- `packages/core/models/_schema-properties-template.yaml` — template
- `packages/core/models/taxonomy.yaml` — realm/layer definitions
- `packages/core/models/config/*.yaml` — config files
- `packages/core/models/realms/*.yaml` — realm definitions
- `packages/core/models/layers/*.yaml` — layer definitions
- `packages/core/models/arc-families/*.yaml` — family definitions
- `turbo/generators/templates/node.yaml.hbs` — Handlebars template

### A.4 — Validate & regenerate

```bash
cargo run -- schema validate    # Must pass zero errors
cargo run -- schema generate    # Regenerate Cypher/TS/Mermaid
```

### A.5 — Update Rust schema validation rules

**File**: `tools/novanet/src/parsers/schema_rules.rs`

**Changes** (29 `llm_context` references):
- Line ~123: Update `REQUIRED_PROPERTIES` constant — replace `llm_context` with `triggers`
- All validation functions referencing `llm_context` -> `triggers`
- Update property type checking: `triggers` is `Vec<String>` not `String`

**File**: `tools/novanet/src/parsers/formatting.rs`

**Changes** (line 117, 136):
- Remove `pub llm_context: String` field from struct
- Remove `generate_llm_context()` function
- Add `pub triggers: Vec<String>` field

**File**: `tools/novanet/src/parsers/taxonomy.rs` (24 refs)

**Changes**: All `llm_context` property references -> `content` + `triggers`

**File**: `tools/novanet/src/parsers/slugification.rs` (16 refs)

**Changes**: All `llm_context` property references -> `content` + `triggers`

**File**: `tools/novanet/src/parsers/expression.rs` (11 refs)

**Changes**: All `llm_context` property references -> `content` + `triggers`

**File**: `tools/novanet/src/generators/organizing.rs` (18 refs)

**Changes**: All Cypher generation referencing `llm_context` -> `triggers`

**File**: `tools/novanet/src/tui/data.rs` (47 refs)

**Changes**: All enrichment maps, display formatting `llm_context` -> `content` + `triggers`

---

## Phase B: Cypher Seed Migration (Pillar 1)

### B.1 — Write seed migration script

**File**: NEW `scripts/migrate-seeds-v020.py`

**Logic**:
1. For each `.cypher` file in `packages/db/seed/`:
   - Find all `SET x.llm_context = '...'` statements
   - Parse the JSON value
   - Extract `triggers` array from JSON `triggers` key
   - Merge `use`/`not_for` text into preceding `SET x.content = '...'`
   - Generate: `SET x.triggers = ['a', 'b', 'c']`
   - Remove: `SET x.llm_context = '...'`

### B.2 — Seed files to migrate (27 files, 22,897 SET statements)

| File | `llm_context` count | Type |
|------|---------------------|------|
| `packages/db/seed/26-expression.cypher` | 17,237 | Knowledge atoms |
| `packages/db/seed/25-culture-refs.cypher` | 2,074 | Culture refs |
| `packages/db/seed/01-classes.cypher` | 480 | Schema classes |
| `packages/db/seed/02-arc-classes.cypher` | 302 | Arc classes |
| `packages/db/seed/20-locales.cypher` | 218 | Locales |
| `packages/db/seed/22-slugification.cypher` | 204 | Slugification rules |
| `packages/db/seed/23-formatting.cypher` | 204 | Formatting rules |
| `packages/db/seed/24-culture.cypher` | 204 | Culture settings |
| `packages/db/seed/25.1-taboos.cypher` | ~200 | Taboos |
| `packages/db/seed/25.2-audience-traits.cypher` | ~200 | Audience traits |
| `packages/db/seed/25.3-patterns.cypher` | ~200 | Patterns |
| `packages/db/seed/25.4-enriched-culture.cypher` | ~200 | Enriched culture |
| `packages/db/seed/25.5-missing-locales.cypher` | ~150 | Missing locales |
| `packages/db/seed/25.6-research-audience.cypher` | ~150 | Research audience |
| `packages/db/seed/25.7-research-culture-refs.cypher` | ~150 | Research culture |
| `packages/db/seed/25.8-research-taboos.cypher` | ~150 | Research taboos |
| `packages/db/seed/26.2-enrichments.cypher` | ~100 | Enrichments |
| `packages/db/seed/27-geographic-taxonomy.cypher` | ~100 | Geography |
| `packages/db/seed/27.5-missing-locales-knowledge.cypher` | ~50 | Missing knowledge |
| `packages/db/seed/00.5-taxonomy.cypher` | ~30 | Taxonomy |
| `packages/db/seed/02.5-entity-categories.cypher` | ~20 | Categories |
| `packages/db/seed/02.6-destination-categories.cypher` | ~10 | Destinations |
| `packages/db/seed/28-locale-taxonomy-links.cypher` | ~5 | Links |
| `packages/db/seed/29-countries.cypher` | ~5 | Countries |
| `packages/db/seed/29.5-locale-country-links.cypher` | ~5 | Links |
| `packages/db/seed/98-indexes.cypher` | ~3 | Indexes — ADD triggers[] index |
| `packages/db/seed/99-autowire-classes.cypher` | ~10 | Autowire |

### B.3 — Add triggers[] index

**File**: `packages/db/seed/98-indexes.cypher`

**Add**:
```cypher
CREATE INDEX idx_triggers IF NOT EXISTS FOR (n:Entity) ON (n.triggers);
```

### B.4 — Verify migration

```bash
pnpm infra:reset   # Full reseed
# Then verify:
# MATCH (n) WHERE n.llm_context IS NOT NULL RETURN count(n) → 0
# MATCH (n) WHERE n.triggers IS NULL RETURN count(n) → 0
```

---

## Phase C: MCP Tool Implementation (Pillar 2 + 3)

### C.1 — `novanet_search`: Add modes (D4)

**File**: `tools/novanet-mcp/src/tools/search.rs`

**Changes**:
- Add `SearchMode` enum: `Find` (current fulltext/property), `Walk` (absorbs traverse), `Hybrid` (find+walk), `Triggers` (new: search by triggers[])
- Move traverse logic from `tools/traverse.rs` into `search.rs` as `Walk` mode handler
- Add `mode` parameter to `SearchParams` struct
- Add `start_key`, `max_depth`, `direction`, `arc_families`, `target_kinds` parameters (from TraverseParams) — only used in Walk/Hybrid mode
- Dispatch by mode in handler

**Response**: unified format for all modes with `mode` field in response.

### C.2 — `novanet_context`: Create new tool (D5)

**File**: NEW `tools/novanet-mcp/src/tools/context.rs`

**Logic**:
- Create `ContextMode` enum: `Page`, `Block`, `Knowledge`, `Assemble`
- Move logic from `generate.rs` → `Page` + `Block` modes
- Move logic from `atoms.rs` → `Knowledge` mode
- Move logic from `assemble.rs` → `Assemble` mode
- Create `ContextParams` struct combining all needed params
- Keep `focus_key`, `locale`, `token_budget`, `spreading_depth` from GenerateParams
- Keep `atom_type`, `domain`, `query`, `register` from AtomsParams
- Keep `strategy`, `include_entities`, `include_knowledge`, `include_structure` from AssembleParams
- Add `block_type` from GenerateParams (task-specific spreading)

**Response**: mode-specific payloads (purpose-built per D10).

### C.3 — `novanet_write`: Absorb checker (D6)

**File**: `tools/novanet-mcp/src/tools/write.rs`

**Changes**:
- Add `dry_run: Option<bool>` parameter to `WriteParams`
- Move validation logic from `checker/validation.rs` into write.rs
- Move types from `checker/types.rs` — absorb `CheckResult`, `SchemaContext`, `OntologySuggestion` into write module
- When `dry_run = true`: run validation only, return `CheckResult`-like response
- When `dry_run = false` (default): auto-validate, then execute if valid, return combined response with validation info + write result
- Add `triggers: Option<Vec<String>>` to WriteParams for content-aware validation
- Add `block_type: Option<String>` to WriteParams for task-specific spreading

### C.4 — Delete old tool files

**Files to DELETE**:
- `tools/novanet-mcp/src/tools/traverse.rs` — logic moved to search.rs
- `tools/novanet-mcp/src/tools/generate.rs` — logic moved to context.rs
- `tools/novanet-mcp/src/tools/assemble.rs` — logic moved to context.rs
- `tools/novanet-mcp/src/tools/atoms.rs` — logic moved to context.rs
- `tools/novanet-mcp/src/tools/cache_stats.rs` — deleted entirely (D7)
- `tools/novanet-mcp/src/tools/checker/mod.rs` — logic moved to write.rs
- `tools/novanet-mcp/src/tools/checker/types.rs` — logic moved to write.rs
- `tools/novanet-mcp/src/tools/checker/validation.rs` — logic moved to write.rs

**Directory to DELETE**: `tools/novanet-mcp/src/tools/checker/`

### C.5 — Update handler.rs

**File**: `tools/novanet-mcp/src/server/handler.rs`

**Changes**:
- DELETE handler registrations:
  - `#[tool(name = "novanet_traverse")]` (tool router macro)
  - `#[tool(name = "novanet_generate")]`
  - `#[tool(name = "novanet_assemble")]`
  - `#[tool(name = "novanet_atoms")]`
  - `#[tool(name = "novanet_check")]`
  - `#[tool(name = "novanet_cache_stats")]`
  - `#[tool(name = "novanet_cache_invalidate")]`
- ADD handler registrations:
  - `#[tool(name = "novanet_context")]` -> dispatch to context.rs
- UPDATE `novanet_search` handler to pass mode parameter
- UPDATE `novanet_write` handler for new auto-check + dry_run params

### C.6 — Update tools/mod.rs

**File**: `tools/novanet-mcp/src/tools/mod.rs`

**Changes**:
- Remove `mod traverse;`
- Remove `mod generate;`
- Remove `mod assemble;`
- Remove `mod atoms;`
- Remove `mod cache_stats;`
- Remove `mod checker;`
- Add `mod context;`
- Update `pub use` exports accordingly

### C.7 — Update batch.rs

**File**: `tools/novanet-mcp/src/tools/batch.rs`

**Changes** (dispatch logic, ~lines 230-258, 422-425):
- Remove dispatch arms for: `novanet_traverse`, `novanet_generate`, `novanet_assemble`, `novanet_atoms`, `novanet_check`, `novanet_cache_stats`, `novanet_cache_invalidate`
- Add dispatch arm for: `novanet_context`
- Update search dispatch for mode parameter

### C.8 — Update prompts/mod.rs

**File**: `tools/novanet-mcp/src/prompts/mod.rs`

**Changes**:
- Update `block_generation` prompt: reference `novanet_context(mode="block")` instead of `novanet_generate`
- Update `page_generation` prompt: reference `novanet_context(mode="page")` instead of `novanet_generate`
- Update `entity_analysis` prompt: reference `novanet_search(mode="walk")` instead of `novanet_traverse`
- Update `locale_briefing` prompt: reference `novanet_context(mode="knowledge")` instead of `novanet_atoms`
- Update any `llm_context` references in prompt text -> `content` + `triggers`

### C.9 — Update hints.rs

**File**: `tools/novanet-mcp/src/hints.rs`

**Changes**:
- Remove hints for deleted tools (traverse, generate, assemble, atoms, check, cache_*)
- Add hints for new tools (context modes, search modes)
- Update `llm_context` references -> `content` + `triggers`

### C.10 — Update schema_cache.rs

**File**: `tools/novanet-mcp/src/schema_cache.rs`

**Changes** (6 refs):
- `ClassMetadata.llm_context` field -> rename to `content` + add `triggers: Vec<String>`
- Update Cypher SELECT clause: `c.llm_context` -> `c.content, c.triggers`
- Update doc comments

### C.11 — Update resources/mod.rs

**File**: `tools/novanet-mcp/src/resources/mod.rs`

**Changes** (3 refs):
- `ClassResource.llm_context` field -> `content` + `triggers`
- Update Cypher queries and response mapping

### C.12 — Evidence Packet V2 (D8)

**File**: `tools/novanet-mcp/src/tools/assemble.rs` -> NOW IN `context.rs`

Rename fields in `EvidencePacket` struct (absorb into context.rs):
- `source_key` -> `key`
- `source_kind` -> `class`
- `evidence_type` -> `type` (use `r#type` in Rust or rename to `evidence_kind`)
- `content` -> `summary`
- ADD: `triggers: Vec<String>`

Also update `tools/novanet-mcp/src/context/spreading.rs` struct references.

### C.13 — Trigger-boosted spreading (D9)

**File**: `tools/novanet-mcp/src/context/spreading.rs`

**Changes**:
- In relevance calculation, add trigger overlap factor:
  ```
  let trigger_overlap = intersection(node.triggers, context.triggers).len() as f64
      / context.triggers.len().max(1) as f64;
  let boost = 1.0 + 0.5 * trigger_overlap;
  let relevance = (1.0 / distance as f64) * boost.min(1.5);
  ```
- Context triggers come from the focus node's triggers
- Add `triggers` field to `SpreadingConfig` or pass separately

### C.14 — Property renames in remaining tools

**File**: `tools/novanet-mcp/src/tools/introspect.rs` (3 refs)

**Changes**: Update Cypher SELECT clauses: `c.llm_context` -> `c.content, c.triggers`

**File**: `tools/novanet-mcp/src/tools/describe.rs`

**Changes**: Update property references if any

**File**: `tools/novanet-mcp/src/tools/query.rs`

**Changes**: Update property references if any

**File**: `tools/novanet-mcp/src/tools/write.rs` (2 refs beyond D6 changes)

**Changes**: Update Cypher SELECT, mapping for `llm_context` -> `content, triggers`

**File**: `tools/novanet-mcp/src/tools/auditor/queries.rs`

**Changes**: Update Cypher queries referencing `llm_context`

### C.15 — Token budget auto-tuning

In `context.rs` (from generate logic), add budget utilization reporting:
```json
{
  "utilization": 0.118,
  "suggestion": "Budget underutilized. Consider budget=10000."
}
```

### C.16 — CSR per-layer breakdown

**File**: `tools/novanet-mcp/src/tools/auditor/`

**Changes**: Add `layers` object to CSR response with per-layer rate + worst constraint.

### C.17 — Next-actions hints

Add `next_actions` field to `novanet_describe` and `novanet_introspect` responses suggesting what tools to use next.

---

## Phase D: TypeScript & Studio Updates (Pillar 1)

### D.1 — Core types

**File**: `packages/core/src/types/index.ts` (10 refs)

**Changes**:
- `StandardNodeProperties.llm_context: string` -> DELETE
- ADD `StandardNodeProperties.triggers: string[]`
- Line ~45: update interface

**File**: `packages/core/src/types/shared.ts` (12 refs)

**Changes**:
- 10+ interfaces with `llm_context: string` -> DELETE, add `triggers: string[]`

**File**: `packages/core/src/types/nodes.ts`

**Changes**: Update any `llm_context` references

### D.2 — Zod schemas

**File**: `packages/core/src/schemas/prompts.schema.ts` (6 refs)

**Changes**:
- Remove Zod regex validation for USE/TRIGGERS/NOT format (line ~71)
- Add `triggers: z.array(z.string()).max(10)` validation
- Remove `llm_context` field from schemas

### D.3 — Filters/Cypher generation

**File**: `packages/core/src/filters/` (multiple files)

**Changes**: Update Cypher generation that references `llm_context`

### D.4 — Studio components

**File**: `apps/studio/src/components/graph/nodes/card/LLMContextBadge.tsx`

**Changes**: RENAME to `TriggersBadge.tsx`. Update component to display `triggers[]` array as tags instead of `llm_context` string.

**File**: `apps/studio/src/lib/neo4j.ts` (line ~210)

**Changes**: Update property mapping: `llm_context` -> `triggers`

**File**: `apps/studio/src/components/` (other components referencing `llm_context`)

**Changes**: Update all references

### D.5 — Verify

```bash
pnpm type-check   # Compiler catches ALL TS references
pnpm lint
pnpm build
```

---

## Phase E: Documentation Blitz

### E.1 — New ADRs

**File**: NEW `dx/adr/novanet/adr-045-self-describing-properties.md`

**Content**: Document `content` priority convention + `triggers[]` specification

**File**: NEW `dx/adr/novanet/adr-046-mcp-tool-consolidation.md`

**Content**: Document 14->8 tool consolidation, mode-based design

### E.2 — Update existing ADRs

| File | Change | Priority |
|------|--------|----------|
| `dx/adr/novanet/adr-027-generation-family.md` | Mark `llm_context` pattern as SUPERSEDED by `content`+`triggers`. Keep for historical reference. | MUST |
| `dx/adr/novanet/adr-037-standard-properties.md` | Update 8-property table: `llm_context` -> `triggers` | MUST |
| `dx/adr/novanet/adr-038-provenance-tracking.md` | Update examples referencing `llm_context` | SHOULD |
| `dx/adr/novanet/adr-035-context-build-log.md` | Update references to `llm_context` in context assembly | SHOULD |
| Other ADRs (041, 042, etc.) | Scan and update any `llm_context` references | CHECK |

### E.3 — CLAUDE.md files (CRITICAL — 4 files)

**File**: `novanet/CLAUDE.md`

**Changes**:
- Version: v0.19.0 -> v0.20.0
- Tool count: 14 -> 8
- Remove `llm_context` from Required Properties table (replace with `triggers`)
- Update MCP tool listing (remove 6 tools, add 2 new)
- Update "Required Properties by Node Class" table
- Update all tool documentation sections
- Update Cypher examples
- Update LLM Context Pattern section (ADR-027 reference)

**File**: `novanet/tools/novanet-mcp/CLAUDE.md` (~1400 lines)

**Changes**: FULL REWRITE of tool documentation
- Remove sections for: traverse, generate, assemble, atoms, check, cache_stats, cache_invalidate (7 sections)
- Add sections for: context (with 4 modes), updated search (with 4 modes), updated write (with auto-check + dry_run)
- Update architecture diagram (14 tools -> 8 tools)
- Update Phase listing
- Update module architecture
- Remove `llm_context` from all examples
- Update all Cypher examples

**File**: `novanet/packages/core/CLAUDE.md`

**Changes**:
- Version references
- Remove `llm_context` from property listings
- Update Cypher query examples
- Update MCP tool references

**File**: `novanet/apps/studio/CLAUDE.md` (if exists)

**Changes**: Update tool references, component names (LLMContextBadge -> TriggersBadge)

### E.4 — DX rules files (CRITICAL — 8 files)

**File**: `dx/.claude/rules/novanet.md`

**Changes**:
- Version: v0.19.0 -> v0.20.0
- Tool count: 14 -> 8
- Update tool listing table (remove 6, add 2, rename)
- Remove `llm_context` references

**File**: `dx/.claude/rules/mcp-tool-selection.md`

**Changes**: REWRITE decision tree for 8 tools
- Remove traverse/generate/assemble/atoms/check/cache entries
- Add context modes and search modes
- Update anti-patterns section
- Update quick reference card

**File**: `dx/.claude/rules/adr-quick-reference.md`

**Changes**:
- Add ADR-045, ADR-046 to Must-Know table
- Update LLM Context Pattern section -> now `content` + `triggers`
- Update Common Mistakes table
- Update "When to Consult ADRs" table

**File**: `dx/.claude/rules/novanet-terminology.md`

**Changes**: Update terminology for `content`, `triggers`, new tool names

**File**: `dx/.claude/rules/schema-standard.md`

**Changes**: Update BLOC 1 standard properties: `llm_context` -> `triggers`

**File**: `dx/.claude/rules/arc-design-guide.md`

**Changes**: Update `llm_context` pattern references

**File**: `dx/.claude/rules/novanet-decisions.md`

**Changes**: Add ADR-045, ADR-046. Mark ADR-027 pattern as superseded.

**File**: `dx/.claude/rules/nika.md`

**Changes**: Update tool references for Nika MCP integration

### E.5 — MCP/Skills docs (HIGH — 8 files)

**File**: `dx/.claude/commands/novanet-mcp.md`

**Changes**: Update tool listings

**File**: `dx/.claude/skills/shared/novanet-mcp.md`

**Changes**: Update tool names and usage patterns

**File**: `dx/.claude/skills/shared/novanet-mcp-verify.md`

**Changes**: Update verification checks for 8 tools

Other skill files referencing deleted tools — scan and update.

### E.6 — ROADMAP & CHANGELOG

**File**: `novanet/ROADMAP.md`

**Changes**: Add v0.20.0 entry (3 pillars, 8 tools, self-describing properties)

**File**: `novanet/CHANGELOG-LATEST.md`

**Changes**: Write v0.20.0 release notes with ASCII banner per release-formatting rules

**File**: `novanet/README.md`

**Changes**: Update tool count, test badge, version references

### E.7 — External docs (Nika integration, sessions, design docs)

**File**: `novanet/tools/novanet-mcp/MCP_GUIDE.md`

**Changes**: Full rewrite for 8 tools with new examples

**File**: `novanet/docs/plans/2026-02-12-phase3-generate-prompts-design.md`

**Changes**: Add "SUPERSEDED by v0.20" note at top

**File**: `novanet/tools/novanet-mcp/docs/plans/2026-03-06-performance-optimization-plan.md`

**Changes**: Add "SUPERSEDED by v0.20" note at top

---

## Phase F: Testing & Release

### F.1 — DELETE test files (5 files, 71 tests)

| File | Tests | Reason |
|------|-------|--------|
| `tools/novanet-mcp/src/tools/generate.rs` | 10 | Tool deleted |
| `tools/novanet-mcp/src/tools/traverse.rs` | 4 | Tool deleted |
| `tools/novanet-mcp/src/tools/assemble.rs` | 27 | Tool deleted |
| `tools/novanet-mcp/src/tools/atoms.rs` | 22 | Tool deleted |
| `tools/novanet-mcp/src/tools/cache_stats.rs` | 8 | Tool deleted |

### F.2 — MAJOR test modifications (10 files, ~283 tests)

**File**: `tools/novanet-mcp/tests/integration_tests.rs` (49 tests)

**Changes**: Rewrite tool imports & invocations for new tool names

**File**: `tools/novanet-mcp/tests/tools_edge_cases_test.rs` (39 tests)

**Changes**:
- DELETE lines 20-101: `test_generate_*` (10 tests)
- DELETE lines 167-235: `test_traverse_*` (6 tests)
- DELETE lines 310-367: `test_atom_*` (7 tests)
- DELETE lines 373-441: `test_assemble_*` (5 tests)
- ADD: tests for `novanet_context` modes (page/block/knowledge/assemble)
- ADD: tests for `novanet_search` modes (find/walk/hybrid/triggers)

**File**: `tools/novanet-mcp/tests/e2e_agent_scenarios.rs` (10+ tests)

**Changes**: Rewrite agent workflow tests for new tool names

**File**: `tools/novanet-mcp/src/tools/batch.rs` (12 tests)

**Changes**: Rewrite batch dispatch tests for novanet_context

**File**: `tools/novanet-mcp/src/server/handler.rs` (15 tests)

**Changes**: Remove 7 handler tests, add novanet_context handler test

**File**: `tools/novanet-mcp/src/context/spreading.rs` (14 tests)

**Changes**: Update evidence assertions for V2 field names + trigger-boost tests

**File**: `packages/core/src/__tests__/schemas.test.ts` (50 tests)

**Changes**: Replace `llm_context` with `content` + `triggers[]` in all Zod schema tests

**File**: `packages/core/src/__tests__/filters.test.ts` (64 tests)

**Changes**: Update Cypher generation assertions

**File**: `packages/core/src/__tests__/types.test.ts` (20 tests)

**Changes**: Replace `llm_context` at 11+ lines

**File**: `apps/studio/src/lib/__tests__/novanetBridge.test.ts` (10 tests)

**Changes**: Update tool invocations for new tool names

### F.3 — Test modifications (11 files, ~225 tests)

| File | Tests | Change |
|------|-------|--------|
| `tools/novanet-mcp/src/tools/search.rs` | ~20 | Add mode parameter tests |
| `tools/novanet-mcp/src/tools/describe.rs` | ~15 | Update property references |
| `tools/novanet-mcp/src/tools/introspect.rs` | ~10 | Update Cypher assertions |
| `tools/novanet-mcp/src/tools/write.rs` | ~25 | Add auto-check + dry_run tests |
| `tools/novanet-mcp/src/tools/checker/validation.rs` | ~20 | Tests move to write.rs |
| `tools/novanet-mcp/src/tools/auditor/queries.rs` | ~15 | Update Cypher assertions |
| `tools/novanet-mcp/src/schema_cache.rs` | ~10 | Update field names |
| `apps/studio/src/components/design-system-coherence.test.ts` | ~30 | Component rename |
| `packages/core/src/__tests__/generator.test.ts` | ~20 | Update Cypher |
| `apps/studio/src/stores/viewStore.test.ts` | ~15 | Update tool refs |
| `apps/studio/src/lib/__tests__/neo4j.test.ts` | ~15 | Update property mapping |

### F.4 — New tests to write

| Test Category | Target File | Tests |
|---------------|-------------|-------|
| `triggers[]` validation | `context.rs` / `write.rs` | max 10, lowercase, no dupes, English |
| `novanet_search` modes | `search.rs` | find/walk/hybrid/triggers dispatch |
| `novanet_context` modes | `context.rs` | page/block/knowledge/assemble dispatch |
| `novanet_write` auto-check | `write.rs` | auto-validate before write, dry_run mode |
| Evidence V2 format | `context.rs` | Shorter field names, triggers field |
| Trigger-boosted spreading | `spreading.rs` | Overlap calculation, 1.5x cap |
| Token budget utilization | `context.rs` | Under/over/optimal reporting |
| CSR per-layer | `auditor/` | Layer breakdown in response |

### F.5 — CLI/TUI test updates

**File**: `tools/novanet/src/parsers/schema_rules.rs` tests

**Changes**: Update all test assertions for `triggers` instead of `llm_context`

**Files**: Various test files in `tools/novanet/` referencing `llm_context`

**Changes**: Property name updates (compiler-guided)

### F.6 — Full verification

```bash
# Rust
cargo test                     # All tests pass
cargo clippy -- -D warnings    # Zero warnings

# TypeScript
pnpm test                     # All tests pass
pnpm type-check               # Zero errors
pnpm lint                     # Zero errors

# Integration
pnpm infra:reset              # Full reseed
# Manual TUI verification
cargo run -- tui
```

### F.7 — Release

```bash
git tag -a v0.20.0 -m "v0.20.0 — Self-Describing Knowledge Graph"
git push origin v0.20.0
# GitHub Release with v0.20 notes
```

---

## Out of Scope (External repos — Thibaut handles separately)

These files exist in OTHER repos/directories and are NOT part of the novanet/ implementation:

| Location | Files | Action |
|----------|-------|--------|
| `nika/tools/nika/src/mcp/client.rs` | 1 | Nika team updates tool names |
| `nika/tools/nika/examples/*.nika.yaml` | 35+ | Update `invoke:` tool names |
| `nika/tools/nika/tests/mcp/*.rs` | 6+ | Update test assertions |
| `e2e/fixtures/workflows/35-*.yaml` | 3 | Rename/rewrite E2E tests |
| `private-data/sessions/**/*.nika.yaml` | 5+ | Update workflow tool names |
| `supernovae-docs/mintlify/novanet/mcp-*.mdx` | 4+ | Rewrite public API docs |
| `nika-v028/` | 100+ | Archive — can ignore |
| `claude-code-supernovae/` | 4+ | Update skill/plugin files |

---

## Execution Order Summary

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  EXECUTION ORDER                                                              ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  Phase A: YAML Schema Migration                                              ║
║  ├── A.1  Update property template                                           ║
║  ├── A.2  Write migration script                                             ║
║  ├── A.3  Run on 255 YAML files                                              ║
║  ├── A.4  schema validate + generate                                         ║
║  └── A.5  Update Rust parsers (schema_rules, formatting, etc.)               ║
║                                                                               ║
║  Phase B: Cypher Seed Migration                                               ║
║  ├── B.1  Write seed migration script                                        ║
║  ├── B.2  Run on 27 seed files (22,897 SETs)                                 ║
║  ├── B.3  Add triggers[] index                                               ║
║  └── B.4  infra:reset + verify                                               ║
║                                                                               ║
║  Phase C: MCP Tool Implementation                                             ║
║  ├── C.1  novanet_search: add 4 modes                                        ║
║  ├── C.2  novanet_context: create (4 modes)                                  ║
║  ├── C.3  novanet_write: absorb checker                                      ║
║  ├── C.4  Delete 8 old tool files + checker dir                              ║
║  ├── C.5  Update handler.rs                                                  ║
║  ├── C.6  Update mod.rs                                                      ║
║  ├── C.7  Update batch.rs                                                    ║
║  ├── C.8  Update prompts/mod.rs                                              ║
║  ├── C.9  Update hints.rs                                                    ║
║  ├── C.10 Update schema_cache.rs                                             ║
║  ├── C.11 Update resources/mod.rs                                            ║
║  ├── C.12 Evidence Packet V2                                                 ║
║  ├── C.13 Trigger-boosted spreading                                          ║
║  ├── C.14 Property renames in remaining tools                                ║
║  ├── C.15 Token budget auto-tuning                                           ║
║  ├── C.16 CSR per-layer breakdown                                            ║
║  └── C.17 Next-actions hints                                                 ║
║                                                                               ║
║  Phase D: TypeScript & Studio                                                 ║
║  ├── D.1  Core types (index.ts, shared.ts, nodes.ts)                         ║
║  ├── D.2  Zod schemas (prompts.schema.ts)                                    ║
║  ├── D.3  Filters/Cypher generation                                          ║
║  ├── D.4  Studio components (LLMContextBadge -> TriggersBadge)               ║
║  └── D.5  pnpm type-check + lint + build                                    ║
║                                                                               ║
║  Phase E: Documentation Blitz                                                 ║
║  ├── E.1  New ADRs (045, 046)                                                ║
║  ├── E.2  Update existing ADRs (027, 037, 038, 035, etc.)                    ║
║  ├── E.3  CLAUDE.md files (4 critical)                                       ║
║  ├── E.4  DX rules files (8 critical)                                        ║
║  ├── E.5  MCP/Skills docs (8 files)                                          ║
║  ├── E.6  ROADMAP + CHANGELOG + README                                       ║
║  └── E.7  External docs (MCP_GUIDE, design plans)                            ║
║                                                                               ║
║  Phase F: Testing & Release                                                   ║
║  ├── F.1  Delete test files (5 files, 71 tests)                              ║
║  ├── F.2  Major test mods (10 files, ~283 tests)                             ║
║  ├── F.3  Test modifications (11 files, ~225 tests)                          ║
║  ├── F.4  New tests (8 categories)                                           ║
║  ├── F.5  CLI/TUI test updates                                               ║
║  ├── F.6  Full verification (cargo test + pnpm test + clippy + lint)         ║
║  └── F.7  Tag v0.20.0 + GitHub release                                       ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## File Count Summary

| Phase | Files Modified | Files Deleted | Files Created |
|-------|---------------|---------------|---------------|
| A | ~260 | 0 | 1 (script) |
| B | ~27 | 0 | 1 (script) |
| C | ~15 | 9 (+1 dir) | 1 (context.rs) |
| D | ~10 | 0 | 1 (TriggersBadge rename) |
| E | ~35 | 0 | 2 (ADRs) |
| F | ~30 | 0 (tests in deleted files) | 0 |
| **TOTAL** | **~377** | **9** | **6** |

---

*Plan generated 2026-03-13 from 6 exhaustive swarm audits covering 2,332+ files*
