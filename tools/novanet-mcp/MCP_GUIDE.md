# NovaNet MCP Developer Guide

Quick reference for integrating with NovaNet MCP Server (v0.17.2).

## Tool Selection Matrix

| Task | Tool | Example |
|------|------|---------|
| Find nodes | `novanet_search` | Search for "QR code" entities |
| Explore relationships | `novanet_traverse` | Follow arcs from an entity |
| Schema info | `novanet_introspect` | List all node classes |
| Content generation | `novanet_generate` | Assemble context for LLM |
| Locale knowledge | `novanet_atoms` | Get terms for fr-FR |
| Write data | `novanet_write` | Create EntityNative |
| Validate before write | `novanet_check` | Check if operation is valid |
| Quality audit | `novanet_audit` | Check coverage/integrity |

**Critical Rule**:
```
⚠️ novanet_query is LAST RESORT — only for custom analytics/aggregations
✅ Use specialized tools (search, traverse, introspect) for common tasks
```

## Common Workflows

### 1. Content Generation (Most Common)

```json
// Step 1: Generate context for a block
{
  "tool": "novanet_generate",
  "params": {
    "focus_key": "block:hero@qr-code",
    "locale": "fr-FR",
    "mode": "block",
    "token_budget": 4000
  }
}

// Returns: prompt, evidence_summary, locale_context, denomination_forms
```

### 2. Entity Discovery

```json
// Step 1: Search for entities
{
  "tool": "novanet_search",
  "params": {
    "query": "QR code",
    "mode": "hybrid",
    "kinds": ["Entity"],
    "limit": 10
  }
}

// Step 2: Explore relationships
{
  "tool": "novanet_traverse",
  "params": {
    "start_key": "entity:qr-code",
    "direction": "outgoing",
    "arc_families": ["semantic"],
    "max_depth": 2
  }
}
```

### 3. Schema Exploration

```json
// List all node classes
{
  "tool": "novanet_introspect",
  "params": {
    "target": "classes",
    "realm": "org"
  }
}

// Get details for a specific class
{
  "tool": "novanet_introspect",
  "params": {
    "target": "class",
    "name": "EntityNative",
    "include_arcs": true
  }
}
```

### 4. Write Data (with Validation)

```json
// Step 1: Always validate first
{
  "tool": "novanet_check",
  "params": {
    "operation": "upsert_node",
    "class": "EntityNative",
    "key": "qr-code@fr-FR",
    "properties": {
      "display_name": "Code QR",
      "denomination_forms": {
        "text": "code qr",
        "title": "Code QR"
      }
    },
    "locale": "fr-FR"
  }
}

// Step 2: If valid, write
{
  "tool": "novanet_write",
  "params": {
    // Same params as check
  }
}
```

## Key Concepts

### Denomination Forms (ADR-033)

Canonical forms for LLM references:

| Form | Usage | Example (fr-FR) |
|------|-------|-----------------|
| `text` | Body content | "code qr" |
| `title` | Headers | "Code QR" |
| `abbrev` | After first mention | "qr" |
| `url` | URL slug | "creer-code-qr" |

### Traits (ADR-024)

Controls what can be written:

| Trait | Writable | Examples |
|-------|----------|----------|
| `defined` | NO | Entity, Page, Block |
| `authored` | YES | EntityNative, PageNative |
| `imported` | YES | SEOKeyword, Term |
| `generated` | YES | BlockNative |
| `retrieved` | YES | GEOAnswer |

### Arc Families

| Family | Purpose | Examples |
|--------|---------|----------|
| `ownership` | Parent-child | HAS_BLOCK, HAS_NATIVE |
| `localization` | Locale links | FOR_LOCALE |
| `semantic` | Meaning | USES_ENTITY, REFERENCES |
| `generation` | AI context | HAS_INSTRUCTION |
| `mining` | External data | TARGETS, HAS_METRICS |
| `schema` | Meta-schema | OF_CLASS, FROM_CLASS |

## Error Handling

### Common Errors

| Error | Cause | Solution |
|-------|-------|----------|
| `ClassNotFound` | Invalid class name | Check `novanet_introspect` |
| `TraitPermission` | Writing to `defined` trait | Use writable trait classes |
| `MissingRequired` | Required properties missing | Check schema properties |
| `ValidationFailed` | Invalid operation | Run `novanet_check` first |

### Error Hints

The server provides actionable hints:

```json
{
  "error": "ClassNotFound",
  "hint": "Run novanet_introspect(target='classes') to see available classes"
}
```

## Performance Tips

1. **Use caching**: Queries are cached for 300s (configurable)
2. **Batch operations**: Use `novanet_batch` for multiple ops
3. **Token budgets**: Set appropriate `token_budget` to control response size
4. **Limit results**: Always use `limit` parameter

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `NOVANET_MCP_NEO4J_PASSWORD` | (required) | Neo4j password |
| `NOVANET_MCP_NEO4J_URI` | `bolt://localhost:7687` | Neo4j connection |
| `NOVANET_MCP_CACHE_TTL_SECS` | `300` | Query cache TTL |
| `NOVANET_MCP_DEFAULT_TOKEN_BUDGET` | `100000` | Default token limit |

## Claude Code Integration

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

## Resources

- **CLAUDE.md**: Full documentation
- **ADR-033**: Denomination forms specification
- **ADR-024**: Trait system explanation
- **write-philosophy.md**: Schema vs Data separation

## CSR Severity Thresholds (novanet_audit)

| CSR Range | Severity | Meaning |
|-----------|----------|---------|
| ≥ 0.95 | ✅ Healthy | Graph is in good shape |
| 0.85 - 0.95 | ⚠️ Warning | Some issues need attention |
| < 0.85 | 🔴 Critical | Significant constraint violations |

## Quick Reference

```
Find → novanet_search (NOT novanet_query)
Explore → novanet_traverse
Schema → novanet_introspect
Generate → novanet_generate
Write → novanet_check → novanet_write
Audit → novanet_audit
Query → LAST RESORT for custom analytics only
```
