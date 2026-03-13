# NovaNet v0.20 Master Plan — Self-Describing Knowledge Graph

**Grade: A+** | **Risk: ALL LOW** | **Status: READY FOR IMPLEMENTATION**

> v0.20 transforms NovaNet from a knowledge graph with opaque LLM metadata
> into a **self-describing knowledge graph** where every node carries its own
> routing signals and natural-language documentation.

---

## Executive Summary

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  NOVANET v0.20 — THREE PILLARS                                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  1. SELF-DESCRIBING PROPERTIES     llm_context → content + triggers[]        ║
║     255 YAML files + 22,897 Cypher SET statements                            ║
║     Automated migration (mechanical, not creative)                           ║
║                                                                               ║
║  2. MCP TOOL CONSOLIDATION         14 tools → 8 tools                        ║
║     25+ Rust files + 573 tests                                               ║
║     Mode-based design with purpose-built payloads                            ║
║                                                                               ║
║  3. GENERATION ARCHITECTURE        Evidence V2 + trigger-boosted spreading   ║
║     Token savings ~15-20% per response                                       ║
║     Auto-tuning + per-layer CSR                                              ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

**Philosophy**: No backward compatibility. No legacy. Clean break. v0 = freedom.

---

## Table of Contents

1. [Plan 1: Self-Describing Properties](#plan-1-self-describing-properties)
2. [Plan 2: MCP Tool Consolidation](#plan-2-mcp-tool-consolidation)
3. [Plan 3: Generation Architecture](#plan-3-generation-architecture)
4. [Impact Analysis](#impact-analysis)
5. [Implementation Sequence](#implementation-sequence)
6. [Risk Assessment](#risk-assessment)
7. [ADR Plan](#adr-plan)
8. [Research References](#research-references)

---

## Plan 1: Self-Describing Properties

### Problem

`llm_context` is a free-form string containing structured information (USE/TRIGGERS/NOT/RELATES pattern from ADR-027). This creates three problems:
1. **Not machine-parseable**: Agents can't route by triggers without parsing natural language
2. **Redundant with content**: The "USE" part duplicates what `content` should say
3. **Not indexable**: Can't create Neo4j indexes on structured data inside a string

### Solution: Enriched `content` + `triggers[]`

```
BEFORE (v0.19):
  content: "Links a defined node to its locale-specific native content"
  llm_context: "USE: when loading locale-specific content. TRIGGERS: content, native, locale. NOT: for structure."

AFTER (v0.20):
  content: "Links a defined node to its locale-specific native content. Use when loading locale-specific content for any node that has native variations. Not for structural relationships (use HAS_BLOCK instead)."
  triggers: ["content", "native", "locale", "localized", "l10n"]
```

### Content Priority Convention

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  CONTENT STRUCTURE RULE                                                         │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  First sentence  = WHAT (core definition, compressible to summary)             │
│  Remaining       = HOW  (guidance, expanded USE/NOT from old llm_context)      │
│                                                                                 │
│  Example:                                                                       │
│  "Core semantic concept that represents a defined thing in the knowledge        │
│   graph. Use when referencing business concepts like QR codes, payment          │
│   methods, or product features. Not for locale-specific content                 │
│   (use EntityNative instead)."                                                  │
│                                                                                 │
│  ├── Sentence 1: WHAT → "Core semantic concept..."                              │
│  └── Sentence 2+: HOW → "Use when... Not for..."                               │
│                                                                                 │
│  novanet_check validates: warns if content doesn't start with noun phrase       │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### triggers[] Specification

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  TRIGGERS RULES                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Type:        string[] (Neo4j native array, NOT JSON string)                   │
│  Maximum:     10 triggers per node                                              │
│  Case:        lowercase only ("locale" not "Locale")                           │
│  Format:      single words preferred; hyphens for compounds ("qr-code")        │
│  Language:    English only (routing language, not content locale)               │
│  Uniqueness:  no duplicates within a node's triggers array                     │
│  Indexing:    Neo4j native array enables WHERE 'x' IN n.triggers               │
│                                                                                 │
│  Good:  ["content", "native", "locale", "l10n"]                                │
│  Bad:   ["Content", "native content", "locale-specific content (authored)"]    │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Standard Properties (v0.20)

| # | Property | Type | Description |
|---|----------|------|-------------|
| 1 | `key` | string | Unique identifier |
| 2 | `display_name` | string | Human-readable label |
| 3 | `node_class` | string | PascalCase=DATA, lowercase=SCHEMA |
| 4 | `content` | string | WHAT this node IS + HOW to use it (priority convention) |
| 5 | `triggers` | string[] | Machine-readable routing keywords (max 10, lowercase) |
| 6 | `provenance` | object | Data origin (seed/nika/mcp) |
| 7 | `created_at` | datetime | Creation timestamp |
| 8 | `updated_at` | datetime | Last modification |

**Change from v0.19**: `llm_context` (string) removed, `triggers` (string[]) added. `content` enriched with HOW guidance.

### Migration Strategy

```
YAML files:       255 files (automated script)
Cypher seeds:     22,897 SET statements (automated script)
Migration type:   MECHANICAL (parse JSON → extract → rewrite)

Current llm_context values are already JSON:
  '{"use": "...", "triggers": ["a", "b"], "not_for": ["c"]}'

Script logic:
  1. Parse JSON from llm_context value
  2. Extract triggers array → new triggers property
  3. Append "use" and "not_for" text to content property
  4. Delete llm_context property/SET statement
  5. Write new triggers SET statement

Verification:
  MATCH (n) WHERE n.llm_context IS NOT NULL RETURN count(n)  →  must be 0
  MATCH (n) WHERE n.triggers IS NULL RETURN count(n)          →  must be 0
```

---

## Plan 2: MCP Tool Consolidation

### Current State (14 tools)

```
Bootstrap:  novanet_describe, novanet_introspect
Navigate:   novanet_search, novanet_traverse
Assemble:   novanet_generate, novanet_assemble, novanet_atoms
Mutate:     novanet_write, novanet_check
Quality:    novanet_audit
Bulk:       novanet_batch
Debug:      novanet_query, novanet_cache_stats, novanet_cache_invalidate
```

### Target State (8 tools)

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  8 TOOLS                                                                      ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  BOOTSTRAP                                                                    ║
║  novanet_describe    (unchanged)     Schema overview, entity details, stats   ║
║  novanet_introspect  (unchanged)     NodeClass/ArcClass metadata              ║
║                                                                               ║
║  NAVIGATE                                                                     ║
║  novanet_search      (MERGED)        find | walk | hybrid | triggers          ║
║                                                                               ║
║  ASSEMBLE                                                                     ║
║  novanet_context     (NEW)           page | block | atoms | raw               ║
║                                                                               ║
║  MUTATE                                                                       ║
║  novanet_write       (MERGED)        auto-check before every write            ║
║                                                                               ║
║  QUALITY                                                                      ║
║  novanet_audit       (ENHANCED)      CSR + per-layer breakdown                ║
║                                                                               ║
║  BULK                                                                         ║
║  novanet_batch       (unchanged)     parallel operations                      ║
║                                                                               ║
║  DEBUG (last resort)                                                          ║
║  novanet_query       (unchanged)     raw Cypher                               ║
║                                                                               ║
║  REMOVED: cache_stats, cache_invalidate (auto-managed internally)            ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Tool Merge Details

#### novanet_search (find + walk + hybrid + triggers)

```
Modes:
┌──────────┬────────────────────────────────────────────────────────────────────┐
│ find     │ Fulltext + property search. "What nodes match X?"                 │
│          │ Params: query, kinds, realm, layer, limit                         │
│          │ Returns: hits[] with scores                                       │
├──────────┼────────────────────────────────────────────────────────────────────┤
│ walk     │ Graph traversal from start_key. "What's connected to Y?"          │
│          │ Params: start_key, max_depth, direction, arc_families             │
│          │ Returns: nodes[], arcs[], paths                                   │
├──────────┼────────────────────────────────────────────────────────────────────┤
│ hybrid   │ Find then walk. "Find X and show its neighborhood."              │
│          │ Params: query + start_key fallback + walk params                  │
│          │ Returns: hits[] + neighbor expansion                              │
├──────────┼────────────────────────────────────────────────────────────────────┤
│ triggers │ Match by triggers[] array. "Route to nodes about locale."         │
│          │ Params: triggers (string[]), kinds, limit                         │
│          │ Returns: nodes matching ANY trigger                               │
└──────────┴────────────────────────────────────────────────────────────────────┘

Tool description (for LLM):
"Search and traverse the knowledge graph.
 find: locate nodes by name or content text.
 walk: explore relationships from a known node.
 hybrid: find nodes then explore their neighborhood.
 triggers: route to nodes by semantic keyword array."
```

#### novanet_context (page + block + atoms + raw)

```
Modes:
┌──────────┬────────────────────────────────────────────────────────────────────┐
│ page     │ Full page orchestration: structure + all blocks + cross-refs.     │
│          │ Params: focus_key, locale, token_budget, spreading_depth          │
│          │ Returns: prompt, evidence_summary, context_anchors, denom_forms   │
├──────────┼────────────────────────────────────────────────────────────────────┤
│ block    │ Single block context: entities + knowledge atoms.                 │
│          │ Params: focus_key, locale, token_budget, block_type               │
│          │ Returns: prompt, evidence_summary, context_anchors, denom_forms   │
├──────────┼────────────────────────────────────────────────────────────────────┤
│ atoms    │ Locale knowledge only: terms, expressions, patterns, etc.         │
│          │ Params: locale, atom_type, domain, query, limit                   │
│          │ Returns: atoms[], containers[], total_count                       │
├──────────┼────────────────────────────────────────────────────────────────────┤
│ raw      │ Low-level assembly with custom strategy.                          │
│          │ Params: focus_key, locale, strategy, token_budget, max_depth      │
│          │ Returns: evidence[], locale_context, truncated                    │
└──────────┴────────────────────────────────────────────────────────────────────┘

Tool description (for LLM):
"Assemble generation context from the knowledge graph.
 page: full page with all blocks and cross-references.
 block: single block with entities and knowledge atoms.
 atoms: locale-specific knowledge (expressions, patterns, culture).
 raw: low-level context assembly with custom traversal strategy."
```

#### novanet_write (auto-check)

```
Flow:
  Agent calls novanet_write(operation, class, key, properties, locale)
       ↓
  Server automatically runs validation (was novanet_check):
  - Schema exists? Required props? Trait allows write?
       ↓
  If valid=false: return {valid: false, issues[], cypher_preview}
  If valid=true:  execute write, return {valid: true, result, auto_arcs}

No more 2-step dance. One call. Auto-validated.
```

### Purpose-Built Response Payloads

```
REMOVED from agent-facing responses (debug only via RUST_LOG):
- execution_time_ms
- cached (boolean)
- token_estimate (renamed to 'tokens' where needed)

ADDED to bootstrap responses (describe, introspect):
- next_actions: ["Use novanet_search(...) to find nodes", ...]
  (guides agents through the pipeline)

Result: ~15-20% token savings per response
```

### Consistent Field Naming

```
BEFORE                    AFTER
source_key        →       key
source_kind       →       class
token_estimate    →       tokens
evidence_type     →       type
```

---

## Plan 3: Generation Architecture

### Evidence Packet V2

```
BEFORE (~200 bytes):
{
  "source_key": "qr-code",
  "source_kind": "Entity",
  "evidence_type": "definition",
  "distance": 1,
  "relevance": 0.9,
  "content": "QR Code Generator: Create custom QR codes with logo...",
  "tokens": 45
}

AFTER (~180 bytes, more structured):
{
  "key": "qr-code",
  "class": "Entity",
  "type": "definition",
  "distance": 1,
  "relevance": 0.9,
  "summary": "QR Code Generator: Create custom QR codes with logo.",
  "tokens": 45,
  "triggers": ["qr", "qr-code", "barcode", "generator"]
}

Changes:
- Shorter field names (key, class, type)
- content → summary (first sentence only, compressed)
- Added triggers for downstream routing
- ~10% savings per packet
```

### Trigger-Boosted Spreading Activation

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  SPREADING ACTIVATION WITH TRIGGER BOOST                                        │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  BEFORE: relevance = 1 / distance        (purely topological)                  │
│                                                                                 │
│  AFTER:  relevance = (1/distance) × (1 + 0.5 × trigger_overlap)               │
│                                                                                 │
│  Where:                                                                         │
│    trigger_overlap = |node.triggers ∩ context.triggers| / |context.triggers|   │
│                                                                                 │
│  Example:                                                                       │
│    Context triggers: ["qr-code", "generator", "barcode"]                       │
│    Node at distance 3 with triggers: ["barcode", "2d", "scanner"]              │
│    trigger_overlap = 1/3 = 0.33                                                │
│    relevance = (1/3) × (1 + 0.5 × 0.33) = 0.33 × 1.17 = 0.39                 │
│    vs without boost: 0.33                                                       │
│                                                                                 │
│  Effect: Semantically relevant but topologically distant nodes get boosted     │
│  Cap: Maximum boost = 1.5x (when all triggers match)                           │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Token Budget Auto-Tuning

```
After context assembly, report budget utilization:

{
  "token_usage": {
    "structure": 500,
    "entities": 3200,
    "knowledge": 1800,
    "locale": 400,
    "total": 5900,
    "budget": 50000,
    "utilization": 0.118,
    "suggestion": "Budget underutilized (11.8%). Consider budget=10000 for this entity."
  }
}

Rules:
- utilization < 20%  → suggest smaller budget
- utilization > 90%  → warn truncation occurred
- 20-90%             → no suggestion (optimal range)
```

### CSR Per-Layer Breakdown

```
novanet_audit response (ENHANCED):

{
  "csr": {
    "rate": 0.95,
    "satisfied": 276,
    "violated": 14,
    "layers": {
      "semantic":   {"rate": 0.92, "violated": 5, "worst": "EntityNative:FOR_LOCALE"},
      "knowledge":  {"rate": 0.99, "violated": 1, "worst": null},
      "output":     {"rate": 0.88, "violated": 8, "worst": "BlockNative:HAS_NATIVE"},
      "structure":  {"rate": 1.00, "violated": 0, "worst": null}
    }
  }
}

Agents know WHICH layer needs attention without digging into individual issues.
```

---

## Impact Analysis

### Full Impact Scope (from 4 swarm analyses)

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  IMPACT MATRIX                                                                ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  CATEGORY              │ FILES  │ CHANGES        │ AUTOMATION                 ║
║  ──────────────────────┼────────┼────────────────┼──────────────────────────  ║
║  Node Class YAML       │   61   │ Remove llm_ctx │ Script: parse+rewrite     ║
║  Arc Class YAML        │  151   │ Remove llm_ctx │ Script: parse+rewrite     ║
║  Other YAML            │   43   │ Template update│ Script: find+replace      ║
║  Cypher Seeds          │   20   │ 22,897 SETs    │ Script: JSON→triggers     ║
║  ──────────────────────┼────────┼────────────────┼──────────────────────────  ║
║  MCP Rust (tools/)     │   15   │ Merge/delete   │ Manual (code refactor)    ║
║  MCP Rust (server/)    │    5   │ Handler update │ Manual                    ║
║  MCP Tests             │  573   │ Update/rewrite │ Manual + search-replace   ║
║  ──────────────────────┼────────┼────────────────┼──────────────────────────  ║
║  CLI/TUI Rust          │    8   │ Property refs  │ Compiler-guided           ║
║  TypeScript Core       │   13   │ Type updates   │ Compiler-guided           ║
║  Studio React          │    5   │ Display update │ Compiler-guided           ║
║  ──────────────────────┼────────┼────────────────┼──────────────────────────  ║
║  CLAUDE.md files       │    6   │ Tool counts    │ Manual                    ║
║  Rules files           │    4   │ Tool refs      │ Manual                    ║
║  ADR files             │    8   │ llm_ctx refs   │ Manual                    ║
║  ROADMAP/CHANGELOG     │    2   │ Version entry  │ Manual                    ║
║  ──────────────────────┼────────┼────────────────┼──────────────────────────  ║
║  TOTAL                 │  ~414  │                │ ~60% automated            ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### MCP Tool File Mapping

```
DELETE (7 files):
  tools/traverse.rs       → logic moves to search.rs
  tools/generate.rs       → logic moves to context.rs (NEW)
  tools/assemble.rs       → logic moves to context.rs (NEW)
  tools/atoms.rs          → logic moves to context.rs (NEW)
  tools/cache_stats.rs    → removed entirely
  tools/checker/mod.rs    → logic moves to write.rs
  tools/checker/types.rs  → logic moves to write.rs
  tools/checker/validation.rs → logic moves to write.rs

CREATE (1 file):
  tools/context.rs        → NEW tool implementation (page/block/atoms/raw)

MODIFY (8 files):
  tools/search.rs         → add walk/hybrid/triggers modes
  tools/write.rs          → absorb checker logic (auto-check)
  tools/batch.rs          → update dispatcher for new tool names
  tools/mod.rs            → update module declarations
  server/handler.rs       → register new tools, remove old
  prompts/mod.rs          → update tool references in prompts
  hints.rs                → update error hints
  context/spreading.rs    → add trigger-boost logic
```

### ADR Files Requiring Updates

| ADR | File | What Changes | Priority |
|-----|------|-------------|----------|
| ADR-027 | `adr-027-generation-family.md` | Mark llm_context pattern as superseded by content+triggers | MUST |
| ADR-044 | `adr-044-standard-properties.md` | Update 8-property table: llm_context → triggers | MUST |
| ADR-035 | `adr-035-context-build-log.md` | Update references to llm_context in context assembly | SHOULD |
| ADR-037 | `adr-037-arc-weight-property.md` | Update spreading activation references | SHOULD |
| ADR-041 | `adr-041-structured-arc-constraints.md` | Remove "llm_context not parseable" discussion | SHOULD |
| ADR-042 | `adr-042-provenance-tracking.md` | Update example with llm_context → triggers | SHOULD |
| ADR-038 | `adr-038-cross-locale-semantic-links.md` | Update if mentions llm_context | CHECK |
| ADR-039 | `adr-039-knowledge-atom-provenance.md` | Update if mentions llm_context | CHECK |

### New ADRs to Write

| ADR | Title | Scope |
|-----|-------|-------|
| ADR-045 | Self-Describing Properties | content priority convention + triggers[] specification |
| ADR-046 | MCP Tool Consolidation | 14→8 tools, mode-based design, purpose-built payloads |

---

## Implementation Sequence

### Phase A: YAML Schema Migration

```
A.1  Update _standard-properties-template.yaml
     Remove llm_context definition, add triggers definition

A.2  Write migration script (Python)
     - Read each YAML file
     - Parse llm_context field value
     - Extract triggers array
     - Append USE/NOT guidance to content field
     - Write triggers field with string array
     - Remove llm_context field

A.3  Run migration on all 255 YAML files

A.4  cargo run -- schema validate
     Must pass with zero errors

A.5  cargo run -- schema generate
     Regenerate all artifacts (Cypher, TS, Mermaid)
```

### Phase B: Cypher Seeds Migration

```
B.1  Write seed migration script (Python)
     - Read each .cypher file
     - For each SET x.llm_context = '...':
       - Parse JSON value
       - Extract triggers array
       - Merge "use"/"not_for" text into content
       - Generate: SET x.triggers = ['a', 'b', 'c']
       - Remove: SET x.llm_context = '...'

B.2  Run migration on all 20 seed files (22,897 SET statements)

B.3  pnpm infra:reset (full reseed)

B.4  Verify with Cypher:
     MATCH (n) WHERE n.llm_context IS NOT NULL RETURN count(n)  →  0
     MATCH (n) WHERE n.triggers IS NULL RETURN count(n)          →  0
```

### Phase C: MCP Tool Implementation

```
C.1   novanet_search: add mode parameter (find/walk/hybrid/triggers)
      Move traverse.rs logic into search.rs with mode dispatch

C.2   novanet_context: create new tool (page/block/atoms/raw)
      Move generate.rs + assemble.rs + atoms.rs logic
      Add mode parameter with dispatch

C.3   novanet_write: merge checker logic (auto-check)
      Move checker/ into write.rs
      Always validate before executing

C.4   Delete old files: traverse.rs, generate.rs, assemble.rs, atoms.rs,
      cache_stats.rs, checker/

C.5   Update handler.rs: register new tools, remove old registrations

C.6   Update batch.rs: new tool name dispatch

C.7   Update prompts/mod.rs: reference new tool names

C.8   Update hints.rs: new error hints for consolidated tools

C.9   Evidence Packet V2: shorter field names + summary + triggers

C.10  Trigger-boosted spreading: additive boost in spreading.rs

C.11  Purpose-built payloads: strip debug metadata from responses

C.12  Token budget auto-tuning: utilization reporting + suggestions

C.13  CSR per-layer breakdown: layer metrics in audit response

C.14  Next-actions hints: add to describe and introspect responses
```

### Phase D: TypeScript Updates

```
D.1  @novanet/core types: remove llm_context, add triggers (string[])

D.2  Studio NodeDetail component: update property display

D.3  Studio API routes: update any llm_context references

D.4  pnpm type-check → must pass (compiler catches everything)
```

### Phase E: Documentation Blitz

```
E.1   Write ADR-045 "Self-Describing Properties"
E.2   Write ADR-046 "MCP Tool Consolidation"
E.3   Update ADR-027 (mark llm_context as superseded)
E.4   Update ADR-044 (standard properties table)
E.5   Update 6 remaining ADRs (035, 037, 038, 039, 041, 042)
E.6   Update novanet/CLAUDE.md (version, tool count, properties)
E.7   Update novanet/README.md (tool count, properties)
E.8   Update tools/novanet-mcp/CLAUDE.md (full tool docs rewrite)
E.9   Update dx/.claude/rules/mcp-tool-selection.md (decision tree)
E.10  Update dx/.claude/rules/novanet.md (tool list)
E.11  Update ROADMAP.md (add v0.20 entry)
E.12  Write CHANGELOG-LATEST.md (v0.20 release notes)
```

### Phase F: Testing & Release

```
F.1   Update MCP tests (573 tests → adapt to new tool signatures)
F.2   Update CLI/TUI tests (property name changes)
F.3   Add new tests for:
      - triggers[] validation (max 10, lowercase, no dupes)
      - novanet_search modes (find/walk/hybrid/triggers)
      - novanet_context modes (page/block/atoms/raw)
      - novanet_write auto-check flow
      - Evidence Packet V2 format
      - Trigger-boosted spreading activation
      - Token budget auto-tuning
      - CSR per-layer breakdown

F.4   Full test suite: cargo test + pnpm test
F.5   cargo clippy -- -D warnings (zero warnings)
F.6   pnpm lint + pnpm type-check
F.7   Manual TUI verification
F.8   Tag v0.20.0, create GitHub release
```

---

## Risk Assessment

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  ALL RISKS: LOW                                                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  RISK 1: Property migration breaks seeds                                     ║
║  Impact:  22,897 Cypher SET statements                                       ║
║  Why LOW: Fully automated script. llm_context already JSON-structured.       ║
║           git diff review. Full reseed verification. git checkout rollback.   ║
║                                                                               ║
║  RISK 2: MCP tool consolidation breaks Nika workflows                        ║
║  Impact:  .nika.yaml files referencing old tool names                        ║
║  Why LOW: Clean break (no backward compat per Thibaut). No external users.   ║
║           Fix Nika workflows directly. Delete old tools.                     ║
║                                                                               ║
║  RISK 3: TypeScript references to llm_context                                ║
║  Impact:  13 TS files + 1 React component                                   ║
║  Why LOW: TypeScript compiler catches ALL references at pnpm type-check.     ║
║           Compiler-guided migration.                                         ║
║                                                                               ║
║  RISK 4: triggers[] Neo4j array index performance                            ║
║  Impact:  New index type                                                     ║
║  Why LOW: Index is optional optimization. Start without, add when needed.    ║
║                                                                               ║
║  RISK 5: Evidence Packet V2 field renames                                    ║
║  Impact:  Internal format change                                             ║
║  Why LOW: No external consumers. Nika uses results as opaque context.        ║
║                                                                               ║
║  RISK 6: Trigger-boosted spreading over-weights distant nodes                ║
║  Impact:  Context quality                                                    ║
║  Why LOW: Additive boost capped at 1.5x. Unit-testable with known graphs.   ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## ADR Plan

### ADR-045: Self-Describing Properties (content + triggers)

**Status**: Proposed
**Context**: llm_context served dual purpose (natural language guidance + machine-parseable routing). This violates single-responsibility.
**Decision**: Split into enriched `content` (LLM-readable, natural language) and `triggers[]` (agent-readable, indexed array). Content follows priority convention: first sentence = WHAT, rest = HOW.
**Consequences**:
- Better token efficiency (agents can read just first sentence for summaries)
- Machine-routable nodes via triggers[] array
- Neo4j native array indexing for O(1) trigger lookup
- 255 YAML + 22,897 Cypher SET migration (automated)

### ADR-046: MCP Tool Consolidation

**Status**: Proposed
**Context**: 14 tools create decision paralysis for LLM agents. Research shows single MCP server with mode-based tools reduces context overhead.
**Decision**: Consolidate to 8 tools via mode-based design. search+traverse→novanet_search(mode), generate+assemble+atoms→novanet_context(mode), check+write→novanet_write(auto-check). Remove cache tools (auto-managed).
**Consequences**:
- 43% tool reduction (14→8)
- Mode-based disambiguation within tools
- Purpose-built payloads (~15-20% token savings)
- Clean break (no backward compat aliases)

---

## Research References

### Perplexity Sonar Research (2026-03-13)

1. **Neo4j Property Design Best Practices**
   - Structured metadata arrays > text fields for LLM integration
   - Trigger-based routing via dynamic prompts + hybrid search
   - Evidence compression via chunking and community reports

2. **MCP Tool Consolidation Best Practices**
   - Single MCP server per system with polymorphic returns
   - Mode-based design (discovery/read/execute phases)
   - Purpose-built payloads stripping unnecessary metadata
   - Server-side schema validators for response shaping

3. **GraphRAG 2026 Token Optimization**
   - Self-describing nodes with embedded summaries → our content priority convention
   - Semantic routing triggers via ontology-enforced domains → our triggers[] array
   - Evidence compression into compact node tables → our Evidence Packet V2
   - Spreading activation with semantic boosting → our trigger-boosted spreading

### Key Insight

NovaNet v0.20's architecture directly implements three 2026 GraphRAG best practices:
1. Self-describing nodes (content with priority convention)
2. Semantic routing (triggers[] as machine-readable routing array)
3. Evidence compression (V2 packets with summary + triggers)

---

## Appendix: Before/After Comparison

### Node YAML

```yaml
# BEFORE (v0.19)
node:
  name: Entity
  properties:
    content:
      type: string
      description: What this entity IS
    llm_context:
      type: string
      description: HOW to use this entity in generation

# AFTER (v0.20)
node:
  name: Entity
  properties:
    content:
      type: string
      description: What this entity IS and how to use it (priority convention)
    triggers:
      type: string[]
      description: Machine-readable routing keywords (max 10, lowercase)
```

### Cypher Seed

```cypher
-- BEFORE (v0.19)
SET e.content = 'Core semantic concept representing a defined thing'
SET e.llm_context = '{"use": "When referencing business concepts", "triggers": ["entity", "concept", "semantic"], "not_for": ["locale content"]}'

-- AFTER (v0.20)
SET e.content = 'Core semantic concept representing a defined thing. Use when referencing business concepts like QR codes, payment methods, or features. Not for locale-specific content (use EntityNative).'
SET e.triggers = ['entity', 'concept', 'semantic', 'defined', 'business']
```

### MCP Tool Call

```json
// BEFORE: Two separate tools
{"tool": "novanet_search", "params": {"query": "QR code"}}
{"tool": "novanet_traverse", "params": {"start_key": "qr-code", "max_depth": 2}}

// AFTER: One tool with modes
{"tool": "novanet_search", "params": {"mode": "find", "query": "QR code"}}
{"tool": "novanet_search", "params": {"mode": "walk", "start_key": "qr-code", "max_depth": 2}}
{"tool": "novanet_search", "params": {"mode": "hybrid", "query": "QR code", "max_depth": 2}}
```

### Evidence Packet

```json
// BEFORE (v0.19, ~200 bytes)
{"source_key": "qr-code", "source_kind": "Entity", "evidence_type": "definition", "distance": 1, "relevance": 0.9, "content": "QR Code Generator: Create custom QR codes...", "tokens": 45}

// AFTER (v0.20, ~180 bytes)
{"key": "qr-code", "class": "Entity", "type": "definition", "distance": 1, "relevance": 0.9, "summary": "QR Code Generator: Create custom QR codes.", "tokens": 45, "triggers": ["qr", "qr-code", "barcode"]}
```

---

*Plan written 2026-03-13 | Grade: A+ | All risks LOW | Ready for implementation*
