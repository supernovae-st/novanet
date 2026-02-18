# NovaNet MCP Server Architecture Plan

**Date**: 2026-02-12
**Version**: Draft v0.1
**Status**: Exploration Phase

## Executive Summary

NovaNet MCP Server exposes the self-describing knowledge graph to AI agents via MCP protocol, enabling RLM-style recursive reasoning over the graph for locale-native content generation.

## Context

### What is NovaNet?

NovaNet is a **Self-Describing Context Graph** for native content generation (NOT translation):
- 281 Entity nodes with semantic relationships
- 13 EntityCategory nodes for taxonomy
- 8 semantic relationship types (INCLUDES, ENABLES, REQUIRES, etc.)
- Knowledge Atoms (Terms, Expressions, Patterns) per locale
- `llm_context` property on every node with USE/TRIGGERS/NOT patterns

### Why MCP?

- **Separation of concerns**: Knowledge (NovaNet) vs Reasoning (Agent)
- **Interoperability**: Any MCP-compatible agent can use NovaNet
- **Token-aware**: Server manages context assembly within token budgets
- **Self-describing**: Agent discovers schema through the graph itself

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│  REASONING LAYER (Agent)                                        │
│  - RLM decomposition/recursion                                  │
│  - Claude/GPT/Llama                                             │
└───────────────────────────────┬─────────────────────────────────┘
                                │ MCP Protocol (JSON-RPC 2.0)
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│  NOVANET MCP SERVER (Rust)                                      │
│  tools/novanet-mcp/                                             │
│                                                                 │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐               │
│  │   search    │ │  traverse   │ │  assemble   │ ...           │
│  └──────┬──────┘ └──────┬──────┘ └──────┬──────┘               │
│         └───────────────┴───────────────┘                       │
│                         │                                       │
│         ┌───────────────┴───────────────┐                       │
│         │  Query Builder + Token Counter │                       │
│         └───────────────┬───────────────┘                       │
└─────────────────────────┼───────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│  KNOWLEDGE LAYER                                                │
│  Neo4j (281 entities, 114 arcs) + YAML (60 kinds)              │
└─────────────────────────────────────────────────────────────────┘
```

## Proposed MCP Tools (v0.1)

### 1. `novanet_search`
Semantic search by triggers (not vector similarity)

```typescript
interface Input {
  triggers: string[];           // Match llm_context.TRIGGERS
  category?: EntityCategory;
  locale?: string;
  limit?: number;
}
```

### 2. `novanet_traverse`
Graph traversal from entry point

```typescript
interface Input {
  from: string;                 // entity key
  relations: ArcType[];         // ["INCLUDES", "ENABLES"]
  depth: number;                // 1-5
  token_budget?: number;
}
```

### 3. `novanet_assemble_context`
Assemble LLM-ready context with token budget

```typescript
interface Input {
  entity_keys: string[];
  locale: string;
  include: {
    entity_content: boolean;
    terms: boolean;
    expressions: boolean;
    patterns: boolean;
    culture: boolean;
    taboos: boolean;
  };
  token_budget: number;
}
```

### 4. `novanet_get_atoms`
Load Knowledge Atoms (Terms, Expressions, Patterns)

```typescript
interface Input {
  locale: string;
  types: AtomType[];
  domain?: string;
  used_by?: string;             // entity key
}
```

### 5. `novanet_describe`
Self-describing meta-queries

```typescript
interface Input {
  describe: "schema" | "entity" | "category" | "relations" | "locales" | "stats";
  entity_key?: string;
}
```

## RLM Integration Pattern

```
USER QUERY
    │
    ▼
DECOMPOSE (Agent)
    │ "menu" + "restaurant" + "qr-code"
    ▼
SEARCH (MCP) ─────────────────────────────────────────┐
    │ novanet_search(triggers: ["menu", "restaurant"])│
    ▼                                                  │
TRAVERSE (MCP) ───────────────────────────────────────┤ RECURSE
    │ novanet_traverse(from: "digital-menu", depth: 2)│ if needed
    ▼                                                  │
ASSEMBLE (MCP) ───────────────────────────────────────┘
    │ novanet_assemble_context(locale: "fr-FR")
    ▼
GENERATE (Agent)
    │ LLM with structured context
    ▼
OUTPUT
```

## Open Questions

1. **Caching**: Redis for hot paths? In-memory LRU?
2. **Streaming**: Should assemble_context stream results?
3. **Resources**: Expose graph as MCP resources for browsing?
4. **Prompts**: Pre-built prompts for common patterns?
5. **RLM Depth**: How deep should recursion go? Token budget as limiter?
6. **Multi-hop**: Optimize for multi-hop queries (RLM-on-KG pattern)?

## Technology Candidates

### Rust MCP
- `rmcp` crate for MCP protocol
- Alternative: raw JSON-RPC implementation

### Neo4j
- `neo4rs` for async Neo4j driver
- Connection pooling with `deadpool`

### Token Counting
- `tiktoken-rs` for accurate GPT token counting
- Alternative: approximate counting for speed

### Serialization
- `serde` + `serde_json` for JSON
- Consider MessagePack for binary optimization

## Next Steps

1. [ ] Explore rmcp crate capabilities
2. [ ] Research RLM-on-KG papers and implementations
3. [ ] Find crates for knowledge graph traversal
4. [ ] Study latest context assembly techniques
5. [ ] Design caching strategy
6. [ ] Prototype first tool (novanet_search)

## References

- NovaNet CLAUDE.md
- ADR-021: Query-First Architecture
- ADR-022: Unified Tree Architecture
- GraphRAG papers
- RLM-on-KG (WordLift)
- MCP Protocol Specification
