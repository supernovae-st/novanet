# NovaNet MCP Server

MCP (Model Context Protocol) server exposing the NovaNet knowledge graph to AI agents.

**Version**: 0.20.0 | **Rust**: 1.86+ | **Edition**: 2024 | **rmcp**: 0.16 (Nika-compatible)

---

## Overview

NovaNet MCP implements **RLM-on-KG** (Recursive Language Model on Knowledge Graph) patterns for efficient context assembly. AI agents can query the NovaNet knowledge graph for content generation, SEO analysis, and locale-aware operations.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  NOVANET MCP ARCHITECTURE (v0.20.0 — The Great Cleanup)                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Claude Code ──► stdio ──► NovaNet MCP Server ──► Neo4j (bolt://7687)      │
│                    │              │                                         │
│                    │              ├── novanet_query        (Cypher exec)    │
│                    │              ├── novanet_describe     (Bootstrap)      │
│                    │              ├── novanet_search       (Find+Explore)   │
│                    │              ├── novanet_introspect   (Schema)         │
│                    │              ├── novanet_context      (Context)        │
│                    │              ├── novanet_write        (Data writes)    │
│                    │              ├── novanet_audit        (Quality)        │
│                    │              └── novanet_batch        (Bulk ops)       │
│                    │                                                        │
│               MCP Protocol                                                  │
│               (JSON-RPC 2.0)                                                │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  v0.20.0: The Great Cleanup (14 → 8 tools)                                 │
│                                                                             │
│  Merged:                                                                    │
│  ├── novanet_traverse → novanet_search (mode=walk)                         │
│  ├── novanet_assemble + novanet_atoms + novanet_generate → novanet_context │
│  └── novanet_check → novanet_write (dry_run=true)                          │
│                                                                             │
│  Deleted:                                                                   │
│  └── novanet_cache_stats, novanet_cache_invalidate                         │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Quick Start

### Environment Variables

```bash
# Required
export NOVANET_MCP_NEO4J_PASSWORD="novanetpassword"

# Optional (with defaults)
export NOVANET_MCP_NEO4J_URI="bolt://localhost:7687"
export NOVANET_MCP_NEO4J_USER="neo4j"
export NOVANET_MCP_NEO4J_POOL_SIZE="16"
export NOVANET_MCP_CACHE_MAX_ENTRIES="10000"
export NOVANET_MCP_CACHE_TTL_SECS="300"
export NOVANET_MCP_DEFAULT_TOKEN_BUDGET="100000"
export NOVANET_MCP_MAX_HOPS="5"
export NOVANET_MCP_EVIDENCE_PACKET_SIZE="200"
```

### Build and Run

```bash
# Build
cargo build --release

# Run (for local testing with stdin/stdout)
cargo run

# Run with debug logging
RUST_LOG=novanet_mcp=debug cargo run
```

### Claude Code Integration

Add to `.claude/settings.json`:

```json
{
  "mcpServers": {
    "novanet": {
      "command": "/path/to/novanet-mcp/target/release/novanet-mcp",
      "env": {
        "NOVANET_MCP_NEO4J_PASSWORD": "novanetpassword"
      }
    }
  }
}
```

---

## Tools (8)

### `novanet_query`

Execute read-only Cypher queries against Neo4j. **DEBUG/ANALYTICS ONLY** — use specialized tools instead.

**Parameters:**
```json
{
  "cypher": "MATCH (n:Entity) RETURN n.key LIMIT 10",
  "params": { "key": "value" },
  "limit": 100,
  "timeout_ms": 30000
}
```

**Returns:**
```json
{
  "rows": [...],
  "row_count": 10,
  "token_estimate": 250,
  "cached": false,
  "execution_time_ms": 45
}
```

**Security:**
- Only read-only queries allowed (MATCH, RETURN, WITH, WHERE, ORDER BY, LIMIT)
- Blocked keywords: CREATE, DELETE, MERGE, SET, REMOVE, DROP, DETACH
- Automatic LIMIT injection if not present

### `novanet_describe`

Bootstrap agent understanding of the knowledge graph.

**Targets:**
| Target | Description |
|--------|-------------|
| `schema` | Full schema overview (realms, layers, classes, arc families) |
| `entity` | Specific entity details (requires `entity_key`) |
| `category` | EntityCategory members (optional `category_key`) |
| `relations` | All ArcClass definitions |
| `locales` | Available locales |
| `stats` | Graph statistics |

**Example:**
```json
{
  "describe": "schema"
}
```

### `novanet_search`

Search and explore the knowledge graph. 5 modes including graph traversal.

**Parameters:**
```json
{
  "query": "QR code",
  "mode": "hybrid",
  "kinds": ["Entity", "Page"],
  "realm": "org",
  "layer": "semantic",
  "limit": 20
}
```

**Modes:**
| Mode | Description | Key Params |
|------|-------------|------------|
| `fulltext` | Neo4j fulltext indexes | `query` |
| `property` | Property-based search with partial matching | `query`, `properties` |
| `hybrid` | Fulltext first, property fallback (default) | `query` |
| `walk` | Graph traversal from a start node (replaces `novanet_traverse`) | `start_key`, `max_depth`, `direction`, `arc_families`, `target_kinds` |
| `triggers` | Match by `triggers[]` array | `query` |

**Walk Mode Example (replaces novanet_traverse):**
```json
{
  "mode": "walk",
  "start_key": "homepage",
  "max_depth": 3,
  "direction": "outgoing",
  "arc_families": ["ownership", "semantic"],
  "target_kinds": ["Entity", "Block"],
  "include_properties": true
}
```

**Direction (walk mode):**
- `outgoing` - Follow outgoing arcs only
- `incoming` - Follow incoming arcs only
- `both` - Follow both directions (default)

### `novanet_introspect`

Introspect the NovaNet schema: query NodeClasses, ArcClasses, and their relationships.

**Parameters:**
```json
{
  "target": "classes",
  "name": null,
  "realm": "org",
  "layer": "semantic",
  "family": null,
  "include_arcs": false
}
```

**Targets:**
| Target | Description | Required Params |
|--------|-------------|-----------------|
| `classes` | List all NodeClasses (optionally filtered by realm/layer) | None |
| `class` | Get a specific NodeClass with optional arc info | `name` |
| `arcs` | List all ArcClasses (optionally filtered by family) | None |
| `arc` | Get a specific ArcClass | `name` |

### `novanet_context`

Unified context assembly for LLM content generation. Replaces `novanet_generate` + `novanet_assemble` + `novanet_atoms`.

**Modes:**
| Mode | Description | Key Params |
|------|-------------|------------|
| `page` | Full page orchestration (all blocks + cross-references) | `focus_key`, `locale` |
| `block` | Single block generation (entities + knowledge) (default) | `focus_key`, `locale` |
| `knowledge` | Locale-specific atoms (terms, expressions, patterns, etc.) | `locale`, `atom_type` |
| `assemble` | Low-level assembly with strategy control | `focus_key`, `locale`, `strategy` |

**Page/Block Mode Example:**
```json
{
  "focus_key": "homepage",
  "locale": "fr-FR",
  "mode": "page",
  "token_budget": 50000,
  "include_examples": true,
  "spreading_depth": 2
}
```

**Knowledge Mode Example:**
```json
{
  "locale": "fr-FR",
  "mode": "knowledge",
  "atom_type": "expression",
  "domain": "technical",
  "query": "QR",
  "limit": 50
}
```

**Atom Types (knowledge mode):**
- `term` - Technical terms with definitions
- `expression` - Idiomatic expressions
- `pattern` - Text patterns/templates
- `cultureref` - Cultural references
- `taboo` - Cultural taboos to avoid
- `audiencetrait` - Audience characteristics
- `all` - All atom types (default)

**Assemble Mode Example:**
```json
{
  "focus_key": "homepage",
  "locale": "fr-FR",
  "mode": "assemble",
  "strategy": "breadth",
  "token_budget": 50000,
  "include_entities": true,
  "include_knowledge": true,
  "include_structure": true,
  "max_depth": 3
}
```

**Assembly Strategies:**
- `breadth` - Breadth-first traversal (default)
- `depth` - Depth-first following ownership arcs
- `relevance` - Prioritize by relevance score
- `custom` - Custom traversal via arc families

**Returns (page/block modes):**
```json
{
  "prompt": "# Generation Context for homepage (fr-FR)...",
  "evidence_summary": [...],
  "locale_context": { "locale_key": "fr-FR", "language": "French", ... },
  "context_anchors": [
    { "page_key": "pricing", "anchor_text": "page de tarifs", "slug": "/fr/tarifs" }
  ],
  "denomination_forms": [
    { "text": "code QR", "title": "Code QR", "abbrev": "QR", "url": "code-qr" }
  ],
  "token_usage": { "total": 5900, "budget_remaining": 44100 }
}
```

**Context Anchors:**

Context Anchors enable cross-page references in generated content. When a Block references another Page via the `REFERENCES_PAGE` arc, the anchor metadata is included for creating internal links.

Anchor syntax: `{{anchor:page_key|display text}}`

### `novanet_write`

Write data to Neo4j with schema validation. Use `dry_run=true` to validate without executing (replaces `novanet_check`).

**Operations:**
| Operation | Description | Use Case |
|-----------|-------------|----------|
| `upsert_node` | Create or update a node | SEOKeyword import, BlockNative generation |
| `create_arc` | Create relationship between nodes | TARGETS, USES_TERM, FOR_LOCALE |
| `update_props` | Update specific properties | EntityNative.denomination_forms |

**Parameters:**
```json
{
  "operation": "upsert_node",
  "class": "SEOKeyword",
  "key": "seo:qr-code@fr-FR",
  "properties": {
    "keyword": "qr code",
    "search_volume": 450000,
    "difficulty": 45
  },
  "locale": "fr-FR",
  "dry_run": false
}
```

**Dry Run (replaces novanet_check):**
```json
{
  "operation": "upsert_node",
  "class": "EntityNative",
  "key": "qr-code@fr-FR",
  "properties": { "name": "Code QR" },
  "locale": "fr-FR",
  "dry_run": true
}
```

Returns validation results, Cypher preview, and ontology-driven suggestions without executing.

**Returns:**
```json
{
  "success": true,
  "operation": "upsert_node",
  "key": "seo:qr-code@fr-FR",
  "created": false,
  "updated_properties": ["search_volume"],
  "auto_arcs_created": ["FOR_LOCALE"],
  "execution_time_ms": 12
}
```

**Schema Validation:**
1. **Class exists**: Validates against Neo4j schema nodes
2. **Properties match schema**: Only declared properties allowed
3. **Required properties present**: Based on schema `required` fields
4. **MERGE pattern**: Idempotent writes (safe to retry)

**Security:**
| Feature | Protection |
|---------|------------|
| Cypher Injection Prevention | Regex validation of class/arc names |
| TTL Cache | Memory leak prevention (moka) |
| HAS_NATIVE Auto-Arc | Consistency enforcement for `*Native` classes |
| Required Property Validation | Schema-based validation before writes |

**Class Name Validation:**
```
Node classes: ^[A-Z][A-Za-z0-9]*$      (PascalCase)
Arc classes:  ^[A-Z][A-Z0-9_]*$        (SCREAMING_SNAKE_CASE)
```

### `novanet_audit`

Post-write quality audit with CSR (Constraint Satisfaction Rate) metrics.

**Parameters:**
```json
{
  "target": "all",
  "scope": { "class": "EntityNative", "locale": "fr-FR" },
  "limit": 100
}
```

**Targets:**
| Target | Description |
|--------|-------------|
| `coverage` | Find Entities without EntityNative for a locale |
| `orphans` | Find *Native nodes missing FOR_LOCALE or HAS_NATIVE arcs |
| `integrity` | Find broken references |
| `freshness` | Find stale nodes (>30 days since last update) |
| `all` | Run all audit checks |

**CSR Severity:**
| CSR Range | Severity |
|-----------|----------|
| >= 0.95 | Healthy |
| 0.85 - 0.95 | Warning |
| < 0.85 | Critical |

### `novanet_batch`

Execute multiple operations in a single request with parallel execution support.

**Parameters:**
```json
{
  "operations": [
    { "id": "op1", "tool": "novanet_search", "params": { "query": "qr code" } },
    { "id": "op2", "tool": "novanet_context", "params": { "focus_key": "homepage", "locale": "fr-FR" } }
  ],
  "parallel": true,
  "fail_fast": true
}
```

Supports all 8 tools: query, describe, search, introspect, context, write, audit, batch.

---

## Tool Selection Guide

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  WHAT DO I NEED?                           → USE THIS TOOL                      │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  "Understand the schema"                   → novanet_describe(describe="schema")│
│  "What classes/arcs exist?"                → novanet_introspect(target="classes")│
│  "Find nodes matching X"                   → novanet_search(query="X")          │
│  "Explore relationships from a node"       → novanet_search(mode="walk",        │
│                                               start_key="...")                  │
│  "Build LLM generation context"            → novanet_context(focus_key="...",   │
│                                               locale="fr-FR", mode="page")      │
│  "Get locale-specific knowledge"           → novanet_context(mode="knowledge",  │
│                                               locale="fr-FR")                   │
│  "Write/update data"                       → novanet_write(...)                 │
│  "Validate before writing"                 → novanet_write(dry_run=true)        │
│  "Check data quality"                      → novanet_audit(target="all")        │
│  "Multiple operations at once"             → novanet_batch(...)                 │
│  "Custom aggregations/analytics"           → novanet_query (LAST RESORT)        │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

**Rule**: `novanet_query` is LAST RESORT, not default.

---

## Write Philosophy: Schema vs Data

NovaNet separates concerns into two distinct layers:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  SCHEMA vs DATA                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  SCHEMA LAYER (Meta)                    DATA LAYER (Instances)              │
│  ─────────────────────                  ──────────────────────              │
│  WHO: Thibaut (human)                   WHO: Nika (AI workflows)            │
│  HOW: YAML files                        HOW: novanet_write MCP              │
│  WHAT: Class, ArcClass definitions      WHAT: Entity, SEOKeyword instances  │
│                                                                             │
│  Thibaut = DBA (defines schema)                                             │
│  Nika = Application (INSERT, UPDATE, SELECT)                                │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Error Hints

The MCP server provides actionable error hints for common issues:

| Error | Hint |
|-------|------|
| Entity not found | "Check key spelling. Use novanet_search to find similar entities." |
| Token budget exceeded | "Reduce limit, narrow filters, or increase budget." |
| Connection failed | "Verify Neo4j is running: docker ps \| grep neo4j" |

---

## Resources

NovaNet MCP provides read-only access to knowledge graph data via URI-based resources.

| Resource | Description |
|----------|-------------|
| `entity://{key}` | Entity with localized content and relationships |
| `class://{name}` | NodeClass definition from schema-graph |
| `locale://{key}` | Locale configuration and knowledge summary |
| `view://{id}` | Saved view/query definition |

---

## MCP Prompts

6 prompt templates for AI agent workflows:

| Prompt | Description | Key Args |
|--------|-------------|----------|
| `cypher_query` | Generate schema-aware Cypher from natural language | `intent` |
| `cypher_explain` | Explain query results in business context | `query`, `results` |
| `block_generation` | Generate context for a single block | `block_key`, `locale` |
| `page_generation` | Orchestrate full page generation | `page_key`, `locale` |
| `entity_analysis` | Deep analysis of an entity | `entity_key`, `locale` |
| `locale_briefing` | Locale voice and culture summary | `locale_key` |

---

## Module Architecture

```
src/
├── main.rs              # Entry point (stdio transport, tracing init)
├── lib.rs               # Public exports
├── error.rs             # Error types (thiserror + MCP mapping)
├── server/
│   ├── mod.rs           # Server struct (State + run loop)
│   ├── config.rs        # Environment configuration
│   ├── state.rs         # Shared state (Arc<StateInner>)
│   └── handler.rs       # MCP handler (rmcp macros, 8 tools)
├── neo4j/
│   ├── mod.rs
│   ├── pool.rs          # Connection pool + query execution
│   └── circuit_breaker.rs # Circuit breaker for resilience
├── cache/
│   └── mod.rs           # Query cache (moka)
├── tokens/
│   ├── mod.rs
│   └── counter.rs       # Token counting (tiktoken-rs)
├── context/
│   ├── mod.rs           # Context assembly engine
│   └── spreading.rs     # Spreading activation for relevance
├── rlm/
│   └── mod.rs           # RLM-on-KG structures
├── tools/
│   ├── mod.rs           # Tool re-exports
│   ├── query.rs         # novanet_query
│   ├── describe.rs      # novanet_describe
│   ├── search.rs        # novanet_search (5 modes incl. walk)
│   ├── introspect.rs    # novanet_introspect
│   ├── context.rs       # novanet_context (4 modes: page/block/knowledge/assemble)
│   ├── write.rs         # novanet_write (with dry_run)
│   ├── auditor/         # novanet_audit
│   │   ├── mod.rs       # Audit execution
│   │   ├── queries.rs   # Audit Cypher queries
│   │   └── types.rs     # AuditResult, CSR types
│   └── batch.rs         # novanet_batch
├── hints.rs             # Error hints system
├── metrics.rs           # Metrics collection
├── validation.rs        # Cypher injection prevention (regex)
├── schema_cache.rs      # Schema metadata cache (moka TTL)
├── resources/
│   └── mod.rs           # MCP resources (entity://, class://, locale://, view://)
└── prompts/
    └── mod.rs           # MCP prompts (6 templates)
```

---

## Key Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `rmcp` | 0.16 | MCP protocol SDK (Nika client compatible) |
| `neo4rs` | 0.8 | Neo4j driver with serde support |
| `tokio` | 1.43 | Async runtime |
| `moka` | 0.12 | High-performance concurrent cache |
| `tiktoken-rs` | 0.9 | Token counting (cl100k_base) |
| `parking_lot` | 0.12 | Fast Mutex/RwLock |
| `dashmap` | 6.1 | Concurrent HashMap |
| `schemars` | 1 | JSON Schema generation (rmcp requirement) |

---

## Error Handling

Errors map to MCP JSON-RPC 2.0 codes:

| Error Type | JSON-RPC Code | When |
|------------|---------------|------|
| `NotFound` | -32001 | Entity/resource not found |
| `InvalidCypher` | -32602 | Cypher validation failed |
| `WriteNotAllowed` | -32602 | Write operation attempted |
| `TokenBudgetExceeded` | -32602 | Token limit exceeded |
| `Connection` | -32603 | Neo4j connection failed |
| `Query` | -32603 | Query execution failed |
| `Internal` | -32603 | Unexpected error |

---

## Token Counting Strategy

Hybrid strategy:
1. **Estimate (fast)**: `chars / 4` for English, `chars * 1.5` for CJK
2. **Exact (slow)**: tiktoken cl100k_base BPE encoding
3. **Smart budget check**: Estimate first, exact only when within 10% margin

---

## Caching

Query results cached using moka:
- **Max entries**: 10,000 (configurable)
- **TTL**: 5 minutes (configurable)
- **Key**: Hash of (cypher + params)
- **Invalidation**: Automatic on TTL expiry, auto-invalidated after writes

---

## Testing

```bash
# Run all tests
cargo test

# Unit + edge case tests (no Neo4j needed)
cargo test --lib
cargo test --test tools_edge_cases_test
cargo test --test integration_tests

# E2E tests (requires Neo4j)
cargo test --test e2e_agent_scenarios
```

**Current test count:** 465+ tests (317 lib + 39 edge cases + 109 integration)

---

## Integration with NovaNet

This MCP server reads from the same Neo4j instance as NovaNet Studio:

- **60 NodeClasses** across 2 realms (shared: 36, org: 24)
- **151 ArcClasses** in 6 families (ownership, localization, semantic, generation, mining, schema)
- **200+ Locales** for multi-locale content generation

---

## v0.20.0 Migration Guide

Tools removed in v0.20.0 and their replacements:

| Old Tool | Replacement |
|----------|-------------|
| `novanet_traverse` | `novanet_search(mode="walk", start_key="...", direction="both")` |
| `novanet_generate` | `novanet_context(mode="page"/"block", focus_key="...", locale="...")` |
| `novanet_assemble` | `novanet_context(mode="assemble", strategy="breadth", ...)` |
| `novanet_atoms` | `novanet_context(mode="knowledge", atom_type="expression", ...)` |
| `novanet_check` | `novanet_write(dry_run=true, ...)` |
| `novanet_cache_stats` | Deleted (internal metrics only) |
| `novanet_cache_invalidate` | Deleted (auto-invalidation on writes) |

---

## Related Documentation

| File | Purpose |
|------|---------|
| `/CLAUDE.md` | NovaNet monorepo overview |
| `/packages/core/models/` | YAML schema definitions (source of truth) |
| `/tools/novanet/` | Rust CLI + TUI (sister binary) |
| `/.claude/rules/novanet-terminology.md` | Domain vocabulary |
| `.claude/rules/write-philosophy.md` | Schema vs Data separation philosophy |
