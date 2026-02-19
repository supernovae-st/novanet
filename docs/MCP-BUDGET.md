# MCP Token Budget Guide

Guide for optimizing token usage when working with NovaNet MCP tools.

## Token Estimates by Tool

| Tool | Typical Response | Token Estimate | Use Case |
|------|------------------|----------------|----------|
| `novanet_query` | Variable | 50-5000 | Raw Cypher queries |
| `novanet_describe` | Schema/Entity | 200-800 | Bootstrap understanding |
| `novanet_search` | Hit list | 100-500 | Find entities |
| `novanet_traverse` | Graph subset | 300-2000 | Navigate relationships |
| `novanet_assemble` | Context window | 1000-4000 | LLM context preparation |
| `novanet_atoms` | Knowledge atoms | 500-3000 | Locale-specific data |
| `novanet_generate` | Full context | 2000-8000 | Content generation |

## Budget Strategies

### 1. Start Small, Expand as Needed

```yaml
# Bad: Immediate full context load
- invoke: novanet_generate
  params:
    entity: "qr-code"
    token_budget: 8000  # Too aggressive

# Good: Start with describe, then expand
- invoke: novanet_describe
  params:
    describe: "entity"
    entity_key: "qr-code"
# Then only if needed:
- invoke: novanet_generate
  params:
    token_budget: 3000
```

### 2. Use Search Before Traverse

```yaml
# Bad: Traverse from root with high depth
- invoke: novanet_traverse
  params:
    start_key: "root"
    max_depth: 5  # Exponential growth!

# Good: Search first, traverse specific node
- invoke: novanet_search
  params:
    query: "qr code scanner"
    limit: 5
- invoke: novanet_traverse
  params:
    start_key: "{{use.search_result.0.key}}"
    max_depth: 2
```

### 3. Token Budget Parameter

Most MCP tools accept `token_budget` parameter:

```yaml
- invoke: novanet_assemble
  params:
    focus_key: "entity:qr-code"
    token_budget: 2000  # Hard cap on response size
    strategy: "relevance"  # Prioritize most relevant
```

### 4. Batch Operations with for_each

```yaml
# Efficient: Parallel with controlled concurrency
tasks:
  - id: load_locales
    for_each: ["fr-FR", "en-US", "de-DE"]
    concurrency: 3  # Limit parallel calls
    invoke: novanet_atoms
    params:
      locale: "{{use.locale}}"
      atom_type: "term"
      domain: "qr-codes"  # Filter by domain
```

## Context Assembly Phases

`novanet_generate` uses `context_build_log` (v0.14.0+) to show token decisions:

```json
{
  "context_build_log": {
    "structure_phase": { "tokens": 150 },
    "entities_phase": { "tokens": 450 },
    "atoms_phase": { "tokens": 800 },
    "anchors_phase": { "tokens": 200 },
    "token_decisions": [
      { "action": "included", "item": "Entity:qr-code", "tokens": 300 },
      { "action": "truncated", "item": "TermSet:fr-FR", "tokens": 500, "reason": "budget" }
    ]
  }
}
```

## Red Flags

| Pattern | Issue | Fix |
|---------|-------|-----|
| `max_depth: >3` | Exponential node count | Use search + targeted traverse |
| No `token_budget` | Unbounded response | Always set budget for assemble/generate |
| `limit: 100` on search | Too many results | Start with 5-10, paginate if needed |
| Repeated describe calls | Redundant context | Cache in workflow binding |

## Monitoring

In Nika workflows, token usage is logged per task:

```
[AgentTurnCompleted] input_tokens: 1523, output_tokens: 847
```

Total workflow tokens available in trace:
```bash
nika trace show <id> --format json | jq '.events[] | select(.kind == "AgentTurnCompleted") | .input_tokens + .output_tokens'
```

## Cost Optimization Checklist

- [ ] Set `token_budget` on all assemble/generate calls
- [ ] Use `novanet_search` before `novanet_traverse`
- [ ] Limit traverse `max_depth` to 2-3
- [ ] Filter atoms by `domain` when possible
- [ ] Use `describe: "schema"` once at workflow start
- [ ] Cache entity context in workflow bindings
- [ ] Use `for_each.concurrency` to control parallel calls
