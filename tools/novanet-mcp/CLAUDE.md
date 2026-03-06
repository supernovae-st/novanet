# NovaNet MCP Server

MCP (Model Context Protocol) server exposing the NovaNet knowledge graph to AI agents.

**Version**: 0.17.0 | **Rust**: 1.86 | **Edition**: 2024 | **rmcp**: 1.1

---

## Overview

NovaNet MCP implements **RLM-on-KG** (Recursive Language Model on Knowledge Graph) patterns for efficient context assembly. AI agents can query the NovaNet knowledge graph for content generation, SEO analysis, and locale-aware operations.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  NOVANET MCP ARCHITECTURE                                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Claude Code ──► stdio ──► NovaNet MCP Server ──► Neo4j (bolt://7687)      │
│                    │              │                                         │
│                    │              ├── novanet_query           (Cypher exec) │
│                    │              ├── novanet_describe        (Bootstrap)   │
│                    │              ├── novanet_search          (Fulltext)    │
│                    │              ├── novanet_traverse        (Traversal)   │
│                    │              ├── novanet_assemble        (Context)     │
│                    │              ├── novanet_atoms           (Knowledge)   │
│                    │              ├── novanet_generate        (RLM-on-KG)   │
│                    │              ├── novanet_introspect      (Schema)      │
│                    │              ├── novanet_batch           (Bulk ops)    │
│                    │              ├── novanet_cache_stats     (Cache info)  │
│                    │              ├── novanet_cache_invalidate (Cache ctrl) │
│                    │              ├── novanet_write           (Data writes) │
│                    │              ├── novanet_check           (Pre-write)   │
│                    │              └── novanet_audit           (Quality)     │
│                    │                                                        │
│               MCP Protocol                                                  │
│               (JSON-RPC 2.0)                                                │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  PHASE 1 (Complete)                                                         │
│  ├── Tools: novanet_query, novanet_describe                                 │
│  ├── State: Neo4j pool, Query cache, Token counter                          │
│  └── Error: Typed errors with MCP JSON-RPC mapping                          │
│                                                                             │
│  PHASE 2 (Complete)                                                         │
│  ├── Resources: entity://, kind://, locale://, view://                      │
│  └── Tools: search, traverse, assemble, atoms                               │
│                                                                             │
│  PHASE 3 (Complete)                                                         │
│  ├── Tool: novanet_generate (block/page mode, context anchors)              │
│  ├── Prompts: cypher_query, cypher_explain, block_generation,               │
│  │            page_generation, entity_analysis, locale_briefing             │
│  └── Context Anchors: Cross-page references via REFERENCES_PAGE arc         │
│                                                                             │
│  PHASE 4 (Complete)                                                         │
│  ├── Tool: novanet_batch (bulk operations with parallel execution)          │
│  ├── Tools: novanet_cache_stats, novanet_cache_invalidate (cache control)   │
│  ├── Feature: Error hints system with actionable suggestions                │
│  └── Total: 11 MCP tools                                                    │
│                                                                             │
│  PHASE 5 (Complete)                                                         │
│  ├── Tool: novanet_write (intelligent data writes)                          │
│  ├── Operations: upsert_node, create_arc, update_props                      │
│  ├── Security: Cypher injection prevention, TTL cache, auto-arcs            │
│  ├── MERGE pattern for idempotent writes                                    │
│  └── Total: 12 MCP tools (11 read + 1 write)                                │
│                                                                             │
│  PHASE 6 (Complete) - v0.17.0                                               │
│  ├── Tool: novanet_check (pre-write validation with Cypher preview)         │
│  ├── Tool: novanet_audit (quality audit with CSR metrics)                   │
│  ├── Feature: Ontology-driven suggestions from llm_context                  │
│  ├── Feature: Constraint Satisfaction Rate (CSR) from MMKG-RDS research     │
│  └── Total: 14 MCP tools (12 read + 2 validation)                           │
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

## Tools

### `novanet_query`

Execute read-only Cypher queries against Neo4j.

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
| `schema` | Full schema overview (realms, layers, kinds, arc families) |
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

Search the knowledge graph using fulltext or property search.

**Parameters:**
```json
{
  "query": "QR code",
  "mode": "hybrid",
  "kinds": ["Entity", "Page"],
  "realm": "org",
  "limit": 20
}
```

**Modes:**
- `fulltext` - Neo4j fulltext indexes
- `property` - Property-based search with partial matching
- `hybrid` - Fulltext first, property fallback

**Returns:**
```json
{
  "hits": [
    {
      "key": "qr-code-generator",
      "kind": "Entity",
      "score": 0.95,
      "matches": [{"property": "name", "value": "QR Code Generator"}],
      "properties": {...}
    }
  ],
  "total_hits": 15,
  "mode": "hybrid",
  "token_estimate": 500,
  "execution_time_ms": 12
}
```

### `novanet_traverse`

Traverse the graph from a starting node with configurable depth and filters.

**Parameters:**
```json
{
  "start_key": "homepage",
  "max_depth": 3,
  "direction": "outgoing",
  "arc_families": ["ownership", "semantic"],
  "target_kinds": ["Entity", "Block"],
  "limit": 50,
  "include_properties": true
}
```

**Direction:**
- `outgoing` - Follow outgoing arcs only
- `incoming` - Follow incoming arcs only
- `both` - Follow both directions

**Returns:**
```json
{
  "start": { "key": "homepage", "kind": "Page", "depth": 0, "path": [] },
  "nodes": [...],
  "arcs": [...],
  "max_depth_reached": 2,
  "limited": false,
  "token_estimate": 1200,
  "execution_time_ms": 45
}
```

### `novanet_assemble`

Assemble context for LLM generation with token budget management.

**Parameters:**
```json
{
  "focus_key": "homepage",
  "locale": "fr-FR",
  "token_budget": 50000,
  "strategy": "breadth",
  "include_entities": true,
  "include_knowledge": true,
  "include_structure": true,
  "max_depth": 3
}
```

**Strategies:**
- `breadth` - Breadth-first traversal (default)
- `depth` - Depth-first following ownership arcs
- `relevance` - Prioritize by relevance score
- `custom` - Custom traversal via arc families

**Returns:**
```json
{
  "focus": { "key": "homepage", "kind": "Page", "name": "Homepage" },
  "evidence": [
    {
      "source_key": "qr-code-generator",
      "source_kind": "Entity",
      "evidence_type": "definition",
      "distance": 1,
      "relevance": 0.9,
      "content": "QR Code Generator: Create custom QR codes...",
      "tokens": 45
    }
  ],
  "locale_context": {
    "locale_key": "fr-FR",
    "language": "French",
    "region": "France",
    "voice": "Professional, friendly"
  },
  "total_tokens": 12500,
  "budget_remaining": 37500,
  "nodes_visited": 25,
  "truncated": false,
  "execution_time_ms": 120
}
```

### `novanet_atoms`

Retrieve knowledge atoms for a specific locale.

**Parameters:**
```json
{
  "locale": "fr-FR",
  "atom_type": "term",
  "domain": "technical",
  "query": "QR",
  "limit": 50,
  "include_containers": true
}
```

**Atom Types:**
- `term` - Technical terms with definitions
- `expression` - Idiomatic expressions
- `pattern` - Text patterns/templates
- `cultureref` - Cultural references
- `taboo` - Cultural taboos to avoid
- `audiencetrait` - Audience characteristics
- `all` - All atom types

**Returns:**
```json
{
  "locale": "fr-FR",
  "atoms": [
    {
      "key": "qr-code",
      "atom_type": "Term",
      "value": "code QR",
      "domain": "technical",
      "properties": { "definition": "Code-barres bidimensionnel" },
      "container_key": "tech-terms-fr"
    }
  ],
  "containers": [
    {
      "key": "tech-terms-fr",
      "container_type": "TermSet",
      "domain": "technical",
      "atom_count": 150
    }
  ],
  "total_count": 25,
  "token_estimate": 800,
  "execution_time_ms": 35
}
```

### `novanet_generate`

Assemble complete generation context for block or page content. This is the composite tool that orchestrates traverse, assemble, and atoms for AI agents.

**Parameters:**
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

**Modes:**
- `block` - Single block generation (entities, knowledge atoms)
- `page` - Full page orchestration (structure, all blocks, cross-references)

**Returns:**
```json
{
  "prompt": "# Generation Context for homepage (fr-FR)...",
  "evidence_summary": [
    {
      "source_key": "qr-code",
      "evidence_type": "entity",
      "relevance": 0.95,
      "tokens": 120
    }
  ],
  "locale_context": {
    "locale_key": "fr-FR",
    "language": "French",
    "voice": "professionnel, accessible",
    "formality": "vous"
  },
  "context_anchors": [
    {
      "page_key": "pricing",
      "anchor_text": "page de tarifs",
      "slug": "/fr/tarifs",
      "context_hint": "Link when mentioning pricing or plans"
    }
  ],
  "token_usage": {
    "structure": 500,
    "entities": 3200,
    "knowledge": 1800,
    "locale": 400,
    "total": 5900,
    "budget_remaining": 44100
  },
  "metadata": {
    "blocks_discovered": 5,
    "entities_loaded": 12,
    "execution_time_ms": 250
  }
}
```

**Context Anchors:**

Context Anchors enable cross-page references in generated content. When a Block references another Page via the `REFERENCES_PAGE` arc, the anchor metadata is included for creating internal links.

Anchor syntax in generated content: `{{anchor:page_key|display text}}`

Example: `"Découvrez notre {{anchor:pricing|page de tarifs}} pour..."` resolves to a proper link.

### `novanet_introspect`

Introspect the NovaNet schema: query NodeClasses, ArcClasses, and their relationships. Enables agents to understand the knowledge graph structure.

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

**Example - List org realm classes:**
```json
{
  "target": "classes",
  "realm": "org"
}
```

**Returns:**
```json
{
  "target": "classes",
  "data": {
    "classes": [
      {"name": "Entity", "realm": "org", "layer": "semantic", "trait_type": "defined"},
      {"name": "EntityNative", "realm": "org", "layer": "semantic", "trait_type": "authored"}
    ],
    "total_count": 21,
    "filters": {"realm": "org", "layer": null}
  },
  "token_estimate": 850
}
```

**Example - Get specific class with arcs:**
```json
{
  "target": "class",
  "name": "Entity",
  "include_arcs": true
}
```

**Returns:**
```json
{
  "target": "class",
  "data": {
    "name": "Entity",
    "realm": "org",
    "layer": "semantic",
    "trait_type": "defined",
    "description": "Core semantic entity",
    "llm_context": "USE: when...",
    "incoming_arcs": ["HAS_ENTITY"],
    "outgoing_arcs": ["HAS_NATIVE", "BELONGS_TO"],
    "include_arcs": true
  },
  "token_estimate": 320
}
```

### `novanet_batch`

Execute multiple operations in a single request with parallel execution support.

**Parameters:**
```json
{
  "operations": [
    {
      "id": "op1",
      "tool": "novanet_search",
      "params": { "query": "qr code", "limit": 10 }
    },
    {
      "id": "op2",
      "tool": "novanet_traverse",
      "params": { "start_key": "homepage", "max_depth": 2 }
    }
  ],
  "parallel": true,
  "stop_on_error": false
}
```

**Returns:**
```json
{
  "results": [
    { "id": "op1", "success": true, "data": {...}, "execution_time_ms": 45 },
    { "id": "op2", "success": true, "data": {...}, "execution_time_ms": 32 }
  ],
  "total_execution_time_ms": 52,
  "operations_count": 2,
  "success_count": 2,
  "error_count": 0
}
```

**Use cases:**
- Batch context assembly for multiple entities
- Parallel search across different filters
- Bulk schema introspection

### `novanet_cache_stats`

Get statistics about the query cache.

**Parameters:**
```json
{}
```

**Returns:**
```json
{
  "entries": 1250,
  "max_entries": 10000,
  "hit_rate": 0.73,
  "hits": 8500,
  "misses": 3150,
  "memory_estimate_bytes": 2500000,
  "ttl_secs": 300
}
```

### `novanet_cache_invalidate`

Invalidate cache entries by pattern or completely clear the cache.

**Parameters:**
```json
{
  "pattern": "Entity:*",
  "clear_all": false
}
```

**Returns:**
```json
{
  "invalidated_count": 45,
  "pattern_used": "Entity:*",
  "cleared_all": false
}
```

**Use cases:**
- Clear cache after schema changes
- Invalidate specific entity caches during development
- Force fresh data retrieval for time-sensitive operations

### `novanet_write`

Intelligent data writes to Neo4j with schema validation. This is the ONLY write tool - all mutations go through it.

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
    "slug_form": "qr-code",
    "search_volume": 450000,
    "difficulty": 45,
    "intent": "informational",
    "source": "ahrefs",
    "retrieved_at": "2026-03-03T10:00:00Z"
  },
  "locale": "fr-FR"
}
```

**Returns:**
```json
{
  "success": true,
  "operation": "upsert_node",
  "key": "seo:qr-code@fr-FR",
  "created": false,
  "updated_properties": ["search_volume", "retrieved_at"],
  "auto_arcs_created": ["FOR_LOCALE"],
  "execution_time_ms": 12
}
```

**Schema Validation:**

The tool validates ALL writes against the schema stored in Neo4j (via introspect):

1. **Class exists**: `SEOKeyword` must be a valid NodeClass
2. **Properties match schema**: Only declared properties allowed
3. **Required properties present**: Validation based on schema `required` fields
4. **Trait allows writes**: Only `authored`, `imported`, `generated`, `retrieved` traits

**Write Permissions by Trait:**

| Trait | Writable | Example Classes | Who Controls |
|-------|----------|-----------------|--------------|
| `defined` | NO | Entity, Page, Block, Locale | Thibaut (YAML) |
| `authored` | YES | EntityNative, PageNative | Nika (MCP) |
| `imported` | YES | SEOKeyword, GeoTrend | Nika (MCP) |
| `generated` | YES | BlockNative | Nika (MCP) |
| `retrieved` | YES | Term, Expression | Nika (MCP) |

**Arc Creation Example:**

```json
{
  "operation": "create_arc",
  "arc_class": "TARGETS",
  "from_key": "entity:qr-code@fr-FR",
  "to_key": "seo:qr-code@fr-FR",
  "properties": {
    "rank": "primary",
    "is_slug_source": true
  }
}
```

> **Note:** TARGETS arc direction is EntityNative → SEOKeyword.
> EntityNative key pattern: `entity:{key}@{locale}` (NOT `entity-native:`)

**Update Properties Example:**

```json
{
  "operation": "update_props",
  "class": "EntityNative",
  "key": "entity:qr-code@fr-FR",
  "properties": {
    "denomination_forms": [
      {"type": "text", "value": "qr code", "priority": 1},
      {"type": "title", "value": "QR Code", "priority": 1},
      {"type": "url", "value": "qr-code", "priority": 1}
    ]
  }
}
```

**Idempotency:**

All writes use Cypher `MERGE` pattern - safe to retry without duplicates:

```cypher
MERGE (n:SEOKeyword {key: $key})
ON CREATE SET n += $properties, n.created_at = timestamp()
ON MATCH SET n += $properties, n.updated_at = timestamp()
```

**Cache Invalidation:**

After successful writes, related cache entries are automatically invalidated to ensure fresh reads.

**Security Features:**

The `novanet_write` tool includes multiple security layers:

| Feature | Protection | Implementation |
|---------|------------|----------------|
| **Cypher Injection Prevention** | Regex validation of class/arc names | `validation.rs` - rejects non-alphanumeric characters |
| **TTL Cache** | Memory leak prevention | `moka::sync::Cache` with automatic eviction |
| **HAS_NATIVE Auto-Arc** | Consistency enforcement | Automatic arc creation for `*Native` classes |
| **Required Property Validation** | Data integrity | Schema-based validation before writes |

**Class Name Validation:**

All class names are validated against strict regex patterns before use in Cypher:

```
Node classes: ^[A-Z][A-Za-z0-9]*$      (PascalCase, e.g., Entity, EntityNative)
Arc classes:  ^[A-Z][A-Z0-9_]*$        (SCREAMING_SNAKE_CASE, e.g., HAS_NATIVE)
```

Invalid input examples (all rejected):
- `Entity}DETACH DELETE n` - Cypher injection attempt
- `123Entity` - Invalid start character
- `entity` - Lowercase not allowed
- `HAS-NATIVE` - Hyphen not allowed in arc names

**HAS_NATIVE Auto-Arc:**

When upserting a `*Native` class (e.g., `EntityNative`, `PageNative`, `BlockNative`) with a key containing `@` (locale separator), the tool automatically creates a `HAS_NATIVE` arc to the base entity:

```
Key: "qr-code@fr-FR" + Class: "EntityNative"
  → Auto-creates: (Entity {key: "qr-code"})-[:HAS_NATIVE]->(EntityNative {key: "qr-code@fr-FR"})
```

### `novanet_check`

Pre-write validation with Cypher preview and ontology-driven suggestions. Call this BEFORE `novanet_write` to validate operations.

**Parameters:**
```json
{
  "operation": "upsert_node",
  "class": "EntityNative",
  "key": "qr-code@fr-FR",
  "properties": {
    "name": "Code QR",
    "description": "Générateur de codes QR"
  },
  "locale": "fr-FR"
}
```

**Returns:**
```json
{
  "valid": true,
  "issues": [],
  "cypher_preview": "MERGE (n:EntityNative {key: $key}) ON CREATE SET n += $props...",
  "schema_context": {
    "class_name": "EntityNative",
    "realm": "org",
    "layer": "semantic",
    "trait_type": "authored",
    "llm_context": "USE: for locale-specific entity content...",
    "required_properties": ["name"],
    "optional_properties": ["description", "denomination_forms"]
  },
  "suggestions": [
    {
      "field": "denomination_forms",
      "suggestion": "Consider adding denomination_forms for LLM entity references",
      "source": "llm_context"
    }
  ]
}
```

**Validation Checks:**
- Class exists in schema
- Trait allows writes (`authored`, `imported`, `generated`, `retrieved`)
- Required properties present
- Property types match schema
- Arc endpoints exist (for `create_arc` operations)

**Use Cases:**
- Validate before bulk imports
- Preview Cypher for debugging
- Get ontology-driven suggestions from `llm_context`

### `novanet_audit`

Post-write quality audit with CSR (Constraint Satisfaction Rate) metrics. Based on MMKG-RDS research.

**Parameters:**
```json
{
  "target": "all",
  "scope": {
    "class": "EntityNative",
    "locale": "fr-FR"
  },
  "limit": 100
}
```

**Targets:**
| Target | Description |
|--------|-------------|
| `coverage` | Find Entities without EntityNative for a locale |
| `orphans` | Find *Native nodes missing FOR_LOCALE or HAS_NATIVE arcs |
| `integrity` | Find broken references (e.g., EntityNative without parent Entity) |
| `freshness` | Find stale nodes (>30 days since last update) |
| `all` | Run all audit checks |

**Returns:**
```json
{
  "target": "all",
  "issues": [
    {
      "severity": "warning",
      "category": "coverage",
      "message": "Entity 'qr-code' missing EntityNative for locale 'de-DE'",
      "node_key": "qr-code",
      "node_class": "Entity"
    }
  ],
  "summary": {
    "total_issues": 5,
    "critical_count": 0,
    "warning_count": 3,
    "info_count": 2,
    "nodes_checked": 150,
    "arcs_checked": 320
  },
  "csr": {
    "rate": 0.92,
    "satisfied_count": 276,
    "violated_count": 24,
    "total_checked": 300,
    "constraints_checked": ["HAS_NATIVE mandatory", "FOR_LOCALE mandatory"]
  },
  "insights": {
    "most_violated_constraint": "EntityNative:FOR_LOCALE (mandatory)",
    "healthiest_layer": "knowledge",
    "attention_needed": ["semantic layer: 8 missing FOR_LOCALE arcs"]
  },
  "recommendations": [
    "Run EntityNative generation for de-DE locale",
    "Create missing FOR_LOCALE arcs for 8 EntityNative nodes"
  ],
  "execution_time_ms": 245
}
```

**CSR Severity Thresholds:**
| CSR Range | Severity | Meaning |
|-----------|----------|---------|
| ≥ 0.95 | Healthy | Graph is in good shape |
| 0.85 - 0.95 | Warning | Some issues need attention |
| < 0.85 | Critical | Significant constraint violations |

**Use Cases:**
- Post-import quality checks
- Scheduled graph health monitoring
- Identify coverage gaps before content generation
- Track CSR over time for quality metrics

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
│  ┌─────────────────────┐                ┌─────────────────────┐             │
│  │  brain/models/*.yaml │───generates──►│  Neo4j :Schema:Class │            │
│  │  (source of truth)   │               │  (runtime reference) │            │
│  └─────────────────────┘                └─────────────────────┘             │
│                                                   │                         │
│                                                   │ validates               │
│                                                   ▼                         │
│                                         ┌─────────────────────┐             │
│                                         │  novanet_write      │             │
│                                         │  (data mutations)   │             │
│                                         └─────────────────────┘             │
│                                                   │                         │
│                                                   │ creates/updates         │
│                                                   ▼                         │
│                                         ┌─────────────────────┐             │
│                                         │  :Entity, :SEOKeyword│            │
│                                         │  (data instances)    │            │
│                                         └─────────────────────┘             │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  ANALOGY: SQL Database                                                      │
│                                                                             │
│  Thibaut = DBA         │  Defines tables, columns, constraints              │
│  Nika = Application    │  INSERT, UPDATE, SELECT on existing tables         │
│                                                                             │
│  Nika CANNOT:          │  CREATE TABLE, ALTER TABLE, DROP TABLE             │
│  Nika CAN:             │  INSERT INTO, UPDATE, SELECT FROM                  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Key Principle**: Nika creates DATA instances within the SCHEMA that Thibaut defined.

**Virtuous Cycle:**

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  THE VIRTUOUS CYCLE                                                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. NIKA WRITES                                                             │
│     └─ SEOKeyword from Ahrefs                                               │
│     └─ EntityNative.denomination_forms with url form                        │
│     └─ BlockNative with generated content                                   │
│     └─ USES_TERM arcs for selective loading                                 │
│                                                                             │
│  2. NOVANET STORES                                                          │
│     └─ Graph grows with real SEO data                                       │
│     └─ Terms linked directly to EntityNatives                               │
│     └─ Relationships capture keyword rankings                               │
│                                                                             │
│  3. NIKA READS (via novanet_generate)                                       │
│     └─ Context assembly includes SEO insights                               │
│     └─ USES_TERM enables 1-hop term loading (vs 4 hops)                     │
│     └─ Keyword volumes inform content priorities                            │
│                                                                             │
│  4. LLM GENERATES                                                           │
│     └─ Better content because better context                                │
│     └─ SEO-optimized slugs from real search data                            │
│     └─ Native-quality text with locale-specific terms                       │
│                                                                             │
│  5. LOOP CONTINUES                                                          │
│     └─ Generated content triggers new SEO analysis                          │
│     └─ Graph becomes smarter with each iteration                            │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Error Hints

The MCP server provides actionable error hints for common issues:

| Error | Hint |
|-------|------|
| Entity not found | "Check key spelling. Use novanet_search to find similar entities." |
| Write operation blocked | "This server is read-only. Use the CLI for mutations." |
| Token budget exceeded | "Reduce limit, narrow filters, or increase budget." |
| Connection failed | "Verify Neo4j is running: docker ps | grep neo4j" |

Hints appear in the `hint` field of error responses to guide agents toward resolution.

---

## Resources

NovaNet MCP provides read-only access to knowledge graph data via URI-based resources.

### `entity://{key}`

Fetch an entity with its localized content and relationships.

```
entity://qr-code-generator
```

**Returns:**
```json
{
  "key": "qr-code-generator",
  "name": "QR Code Generator",
  "definition": "Application for creating custom QR codes",
  "category": "product",
  "content": [
    { "locale": "fr-FR", "name": "Générateur de codes QR", "description": "..." },
    { "locale": "en-US", "name": "QR Code Generator", "description": "..." }
  ],
  "related": [
    { "key": "qr-code", "name": "QR Code", "relationship": "USES_ENTITY", "direction": "outgoing" }
  ]
}
```

### `class://{name}`

Fetch a NodeClass definition from the schema-graph.

```
class://Entity
```

**Returns:**
```json
{
  "name": "Entity",
  "display_name": "Entity",
  "realm": "org",
  "layer": "semantic",
  "trait_type": "defined",
  "description": "Core semantic entity",
  "llm_context": "Entities represent invariant concepts...",
  "properties": [...],
  "outgoing_arcs": ["HAS_NATIVE", "USES_ENTITY"],
  "incoming_arcs": ["BELONGS_TO"],
  "instance_count": 150
}
```

### `locale://{key}`

Fetch locale configuration and knowledge summary.

```
locale://fr-FR
```

**Returns:**
```json
{
  "key": "fr-FR",
  "language": "French",
  "region": "France",
  "script": "Latin",
  "direction": "ltr",
  "knowledge_summary": {
    "term_count": 500,
    "expression_count": 200,
    "pattern_count": 50,
    "culture_ref_count": 30,
    "taboo_count": 15,
    "audience_trait_count": 25
  }
}
```

### `view://{id}`

Fetch a saved view/query definition.

```
view://composition
```

**Returns:**
```json
{
  "id": "composition",
  "name": "Page Composition",
  "description": "Page/Block composition hierarchy",
  "category": "contextual",
  "cypher": "MATCH (root {key: $nodeKey})...",
  "parameters": [
    { "name": "nodeKey", "param_type": "string", "required": true }
  ]
}
```

---

## MCP Prompts

NovaNet MCP provides 6 prompt templates for AI agent workflows. Prompts guide agents through tool orchestration for common tasks.

### `cypher_query`

Generate schema-aware Cypher queries from natural language.

**Arguments:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `intent` | string | Yes | Natural language description of what to query |
| `constraints` | string | No | Additional constraints (realm, layer, limit) |

**Use case:** Agent needs to query the graph but doesn't know the schema.

### `cypher_explain`

Explain query results in business context.

**Arguments:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `query` | string | Yes | The Cypher query that was executed |
| `results` | string | Yes | JSON array of query results |

**Use case:** Agent ran a query and needs to interpret results for the user.

### `block_generation`

Generate context for a single block's content.

**Arguments:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `block_key` | string | Yes | The block's key |
| `locale` | string | Yes | Target locale (BCP-47) |
| `token_budget` | number | No | Max tokens (default: 10000) |

**Use case:** Agent generating content for a specific Block.

**Orchestration:**
1. Call `novanet_traverse` to get block structure + entities
2. Call `novanet_atoms` to get locale-specific knowledge
3. Assemble evidence packets by relevance
4. Return context with anchor syntax for cross-page links

### `page_generation`

Orchestrate full page generation across all blocks.

**Arguments:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `page_key` | string | Yes | The page's key |
| `locale` | string | Yes | Target locale (BCP-47) |
| `token_budget` | number | No | Max tokens (default: 50000) |

**Use case:** Agent generating an entire page with multiple blocks.

**Orchestration:**
1. Discover page structure via `novanet_traverse`
2. For each block, invoke `block_generation` prompt
3. Assemble page-level metadata (title, meta_description)
4. Include context anchors for internal linking

### `entity_analysis`

Deep analysis of an entity with usage context.

**Arguments:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `entity_key` | string | Yes | The entity's key |
| `locale` | string | Yes | Analysis locale |

**Use case:** Agent needs comprehensive understanding of an entity.

**Returns:**
- Entity definition summary
- Locale-specific adaptations (EntityNative)
- Pages/Blocks using this entity
- Related entities (semantic connections)

### `locale_briefing`

Locale voice and culture summary for content generation.

**Arguments:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `locale_key` | string | Yes | Locale key (BCP-47) |

**Use case:** Agent needs to understand locale voice before generating content.

**Returns:**
- Voice guidelines (formality, tone, vocabulary)
- Cultural context (references, taboos)
- Technical formats (date, number, currency)
- Example phrases demonstrating the voice

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
│   └── handler.rs       # MCP handler (rmcp macros)
├── neo4j/
│   ├── mod.rs
│   └── pool.rs          # Connection pool + query execution
├── cache/
│   └── mod.rs           # Query cache (moka)
├── tokens/
│   ├── mod.rs
│   └── counter.rs       # Token counting (tiktoken-rs)
├── rlm/
│   └── mod.rs           # RLM-on-KG structures
├── tools/
│   ├── mod.rs
│   ├── query.rs         # novanet_query implementation
│   ├── describe.rs      # novanet_describe implementation
│   ├── search.rs        # novanet_search implementation
│   ├── traverse.rs      # novanet_traverse implementation
│   ├── assemble.rs      # novanet_assemble implementation
│   ├── atoms.rs         # novanet_atoms implementation
│   ├── generate.rs      # novanet_generate implementation
│   ├── introspect.rs    # novanet_introspect implementation
│   ├── batch.rs         # novanet_batch implementation
│   ├── cache_stats.rs   # novanet_cache_stats/invalidate implementation
│   └── write.rs         # novanet_write implementation (Phase 5)
├── hints.rs             # Error hints system
├── validation.rs        # Cypher injection prevention (regex validation)
├── schema_cache.rs      # Schema metadata cache (moka TTL)
├── resources/
│   └── mod.rs           # MCP resources (entity://, class://, locale://, view://)
└── prompts/
    └── mod.rs           # MCP prompts (Phase 3)
```

---

## Key Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `rmcp` | 1.1 | MCP protocol SDK (Anthropic official) |
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

**Pattern for LLM-friendly errors:**

```rust
// Good: Actionable error message
Error::invalid_cypher("Query contains DELETE. Use read-only queries only.")

// Bad: Technical error
Error::Internal("deadpool timeout".to_string())
```

---

## Token Counting Strategy

The server uses a **hybrid token counting strategy**:

1. **Estimate (fast)**: `chars / 4` for English, `chars * 1.5` for CJK
2. **Exact (slow)**: tiktoken cl100k_base BPE encoding
3. **Smart budget check**: Estimate first, exact only when within 10% margin

```rust
// Fast path: clearly within budget
if estimate < budget * 90% { return true; }

// Fast path: clearly over budget
if estimate > budget * 110% { return false; }

// Slow path: exact count needed
counter.count(text) <= budget
```

---

## Caching

Query results are cached using moka with:
- **Max entries**: 10,000 (configurable)
- **TTL**: 5 minutes (configurable)
- **Key**: Hash of (cypher + params)

Cache is automatically invalidated on TTL expiry. Manual invalidation available via `cache.invalidate(key)`.

---

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_validate_read_only

# Integration tests (requires Neo4j)
cargo test --test integration
```

**Current test count:** 573 tests (unit + integration tests, Neo4j integration tests require `NOVANET_MCP_NEO4J_PASSWORD` env var)

---

## Debugging

### Enable Debug Logging

```bash
RUST_LOG=novanet_mcp=debug cargo run
```

### Log Levels

| Level | Content |
|-------|---------|
| `error` | Connection failures, query errors |
| `warn` | Cache misses, high token usage |
| `info` | Server start, query execution |
| `debug` | Full query/response payloads |
| `trace` | Internal state changes |

### Common Issues

**Neo4j Connection Failed:**
```
Error: Neo4j connection failed to bolt://localhost:7687
```
- Check Neo4j is running: `docker ps | grep neo4j`
- Check credentials: `NOVANET_MCP_NEO4J_PASSWORD`
- Check URI: `NOVANET_MCP_NEO4J_URI`

**Write Operation Not Allowed:**
```
Error: Write operations not allowed: CREATE
```
- The server only allows read operations
- Remove CREATE/DELETE/MERGE/SET from query

**Token Budget Exceeded:**
```
Error: Token budget exceeded: 150000/100000
```
- Reduce query result size with LIMIT
- Increase budget: `NOVANET_MCP_DEFAULT_TOKEN_BUDGET`

---

## Integration with NovaNet

This MCP server reads from the same Neo4j instance as NovaNet Studio. Schema:

- **61 NodeClasses** across 2 realms (shared: 40, org: 21)
- **169 ArcClasses** in 5 families (ownership, localization, semantic, generation, mining)
- **200+ Locales** for multi-locale content generation

Key queries for agent bootstrap:

```cypher
-- Get schema overview (v0.13.0: Class, was Kind)
MATCH (c:Schema:Class)
WITH c.realm AS realm, c.layer AS layer, collect(c.name) AS classes
RETURN realm, layer, classes ORDER BY realm, layer

-- Get entity with context
MATCH (e:Entity {key: $key})
OPTIONAL MATCH (e)-[:BELONGS_TO]->(c:EntityCategory)
OPTIONAL MATCH (e)-[:HAS_NATIVE]->(en:EntityNative)
RETURN e, c.category_key, collect(DISTINCT en.locale) AS locales

-- Get locale knowledge atoms
MATCH (l:Locale {key: $locale})-[:HAS_TERMS]->(ts:TermSet)-[:CONTAINS_TERM]->(t:Term)
WHERE t.domain = $domain
RETURN t.key, t.value LIMIT 50
```

---

## Roadmap

### Phase 1 (Complete)
- [x] Basic tools: novanet_query, novanet_describe
- [x] Query caching (moka)
- [x] Token counting (tiktoken)
- [x] Read-only Cypher validation
- [x] Error mapping to MCP

### Phase 2 (Complete)
- [x] MCP Resources: entity://, kind://, locale://, view://
- [x] Tools: search, traverse, assemble, atoms
- [x] RLM traversal with hop-by-hop evidence packets

### Phase 3 (Complete)
- [x] Design: 6 MCP Prompts with Full RAG pattern
- [x] Design: Hybrid architecture (atomic tools + novanet_generate composite)
- [x] Design: Context Anchor pattern for cross-page references
- [x] Views: 4 generation views added to _registry.yaml
- [x] Tool: novanet_generate (block/page mode)
- [x] Prompts: cypher_query, cypher_explain, block_generation, page_generation, entity_analysis, locale_briefing
- [x] Context Anchors: REFERENCES_PAGE arc + anchor resolution
- [x] Evidence packet compression (~200 bytes target)

**Design document**: `docs/plans/2026-02-12-phase3-generate-prompts-design.md`

### Phase 4 (Complete)
- [x] Tool: novanet_batch (bulk operations with parallel execution)
- [x] Tools: novanet_cache_stats, novanet_cache_invalidate (cache management)
- [x] Feature: Error hints system with actionable suggestions
- [x] Total: 11 read-only MCP tools

### Phase 5 (In Progress) - Write Operations
- [ ] Tool: novanet_write (single write endpoint)
- [ ] Operations: upsert_node, create_arc, update_props
- [ ] Schema validation via introspect (runtime, no YAML parsing)
- [ ] Write permissions by trait (authored, imported, generated, retrieved)
- [ ] MERGE pattern for idempotent writes
- [ ] Auto-arc creation for mandatory relationships (FOR_LOCALE, HAS_NATIVE)
- [ ] Cache invalidation after writes
- [ ] Total: 12 MCP tools (11 read + 1 write)

**Design document**: `docs/sessions/2026-03-03-qrcode-seo-workflow/07-brainstorm-novanet-write.md`

### Phase 6 (Future)
- [ ] Integration tests with real Neo4j + seed data
- [ ] Claude Code integration testing (MCP protocol validation)
- [ ] Performance benchmarks (latency, token efficiency)
- [ ] Streaming support for large context assembly
- [ ] Subscription support for graph change notifications

---

## Related Documentation

| File | Purpose |
|------|---------|
| `/CLAUDE.md` | NovaNet monorepo overview |
| `/packages/core/models/` | YAML schema definitions (source of truth) |
| `/packages/core/models/views/_registry.yaml` | View definitions (19 views, 4 generation) |
| `/packages/db/seed/` | Neo4j seed scripts |
| `/tools/novanet/` | Rust CLI + TUI (sister binary) |
| `/.claude/rules/novanet-terminology.md` | Domain vocabulary |
| `/.claude/rules/novanet-decisions.md` | Architecture decisions (ADRs) |
| `docs/plans/2026-02-12-phase3-generate-prompts-design.md` | Phase 3 design document |
| `docs/novanet-write.md` | novanet_write tool documentation (Phase 5) |
| `.claude/rules/write-philosophy.md` | Schema vs Data separation philosophy |
| `.claude/skills/seo-workflow.md` | SEO workflow patterns with novanet_write |
| `docs/sessions/2026-03-03-qrcode-seo-workflow/` | Phase 5 design session |
