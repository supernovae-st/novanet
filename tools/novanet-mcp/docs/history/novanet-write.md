# novanet_write - Intelligent Data Writes

## Overview

`novanet_write` is the ONLY write tool in NovaNet MCP. All data mutations go through this single endpoint, which provides schema validation and intelligent defaults.

## Why a Single Write Tool?

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  DESIGN PHILOSOPHY                                                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Option A: Multiple Tools              Option B: Single Tool (CHOSEN)       │
│  ──────────────────────                ──────────────────────────────       │
│  create_seo_keyword                    novanet_write                        │
│  update_entity_native                    ├── operation: upsert_node         │
│  create_block_native                     ├── operation: create_arc          │
│  create_term                             └── operation: update_props        │
│  create_arc                                                                 │
│                                                                             │
│  Problems:                             Benefits:                            │
│  • Tool explosion (N classes = N tools)• Single validation point            │
│  • Duplicate validation logic          • Schema-driven, not hardcoded       │
│  • Hard to maintain                    • Extensible without new tools       │
│                                        • Consistent error handling          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Operations

### `upsert_node`

Create a new node or update existing (MERGE pattern).

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

**Cypher generated:**
```cypher
MERGE (n:SEOKeyword {key: $key})
ON CREATE SET n += $properties, n.created_at = timestamp()
ON MATCH SET n += $properties, n.updated_at = timestamp()
```

### `create_arc`

Create a relationship between two nodes.

```json
{
  "operation": "create_arc",
  "arc_class": "TARGETS",
  "from_key": "seo:qr-code@fr-FR",
  "to_key": "entity-native:qr-code@fr-FR",
  "properties": {
    "rank": "primary",
    "is_slug_source": true
  }
}
```

**Cypher generated:**
```cypher
MATCH (from {key: $from_key}), (to {key: $to_key})
MERGE (from)-[r:TARGETS]->(to)
SET r += $properties
```

### `update_props`

Update specific properties on an existing node.

```json
{
  "operation": "update_props",
  "class": "EntityNative",
  "key": "entity-native:qr-code@fr-FR",
  "properties": {
    "denomination_forms": [
      {"type": "text", "value": "qr code", "priority": 1},
      {"type": "title", "value": "QR Code", "priority": 1},
      {"type": "url", "value": "qr-code", "priority": 1}
    ]
  }
}
```

## Schema Validation

Every write is validated against the Neo4j schema:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  VALIDATION FLOW                                                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. Parse request                                                           │
│     └─ Extract operation, class, key, properties                            │
│                                                                             │
│  2. Fetch schema from Neo4j                                                 │
│     └─ MATCH (c:Schema:Class {name: $class}) RETURN c                       │
│                                                                             │
│  3. Validate class exists                                                   │
│     └─ Error if class not found                                             │
│                                                                             │
│  4. Check trait allows writes                                               │
│     └─ defined = REJECT                                                     │
│     └─ authored/imported/generated/retrieved = ALLOW                        │
│                                                                             │
│  5. Validate properties                                                     │
│     └─ Check all properties declared in schema                              │
│     └─ Check required properties present                                    │
│     └─ Check types match (string, number, array, object)                    │
│                                                                             │
│  6. Execute write                                                           │
│     └─ Generate Cypher with MERGE pattern                                   │
│     └─ Execute against Neo4j                                                │
│                                                                             │
│  7. Post-write actions                                                      │
│     └─ Create auto-arcs (FOR_LOCALE, HAS_NATIVE)                            │
│     └─ Invalidate related cache entries                                     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Write Permissions

Only certain traits allow writes:

| Trait | Writable | Rationale |
|-------|----------|-----------|
| `defined` | NO | Structure, controlled by YAML |
| `authored` | YES | Human-written content |
| `imported` | YES | External data (Ahrefs, etc.) |
| `generated` | YES | LLM-generated content |
| `retrieved` | YES | Discovered knowledge atoms |

## Auto-Arc Creation

When creating certain nodes, mandatory arcs are automatically created:

| Node Class | Auto-Arc | Target |
|------------|----------|--------|
| EntityNative | FOR_LOCALE | Locale |
| EntityNative | HAS_NATIVE (from Entity) | Entity |
| SEOKeyword | FOR_LOCALE | Locale |
| BlockNative | FOR_LOCALE | Locale |
| Term | FOR_LOCALE | Locale |

This ensures referential integrity without requiring explicit arc creation.

## Idempotency

All writes use MERGE pattern - safe to retry:

```
First call:  MERGE creates node   → created: true
Second call: MERGE finds node     → created: false, updated_properties: [...]
Third call:  Same properties      → created: false, updated_properties: []
```

## Response Format

```json
{
  "success": true,
  "operation": "upsert_node",
  "key": "seo:qr-code@fr-FR",
  "created": false,
  "updated_properties": ["search_volume", "retrieved_at"],
  "auto_arcs_created": ["FOR_LOCALE"],
  "execution_time_ms": 12,
  "cache_invalidated": ["SEOKeyword:*", "entity-native:qr-code@fr-FR"]
}
```

## Error Handling

Errors include actionable hints:

| Error | Hint |
|-------|------|
| Class not found | "Use novanet_introspect to list available classes" |
| Trait not writable | "Only authored/imported/generated/retrieved classes allow writes" |
| Missing required property | "Required: keyword, slug_form. Missing: slug_form" |
| Target node not found | "Target entity-native:xxx@fr-FR not found. Create it first." |

## Batch Writes

Use `novanet_batch` for multiple writes:

```json
{
  "operations": [
    {"id": "kw1", "tool": "novanet_write", "params": {...}},
    {"id": "kw2", "tool": "novanet_write", "params": {...}},
    {"id": "arc1", "tool": "novanet_write", "params": {...}}
  ],
  "parallel": true,
  "stop_on_error": false
}
```

## Security Considerations

- **No raw Cypher**: All writes go through validated operations
- **Schema enforcement**: Can't write unknown properties
- **Trait protection**: Can't modify `defined` trait nodes
- **Audit trail**: All writes logged with timestamps

## Implementation Status

- [ ] Core write.rs module
- [ ] Schema validation via introspect
- [ ] upsert_node operation
- [ ] create_arc operation
- [ ] update_props operation
- [ ] Auto-arc creation
- [ ] Cache invalidation
- [ ] Batch integration
- [ ] Tests

**Design document**: `docs/sessions/2026-03-03-qrcode-seo-workflow/07-brainstorm-novanet-write.md`
