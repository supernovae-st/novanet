# NovaNet MCP Server - DX Audit Plan

Comprehensive Developer Experience documentation plan for the NovaNet MCP server.

**Audit Date:** 2026-02-12
**Version Analyzed:** 0.1.0 (Phase 1)

---

## 1. Recommended Skills

### 1.1 `novanet-mcp` (Core Skill)

**Location:** `.claude/skills/novanet-mcp/SKILL.md`

**Trigger:** Questions about MCP server, RLM-on-KG, AI agent integration, knowledge graph queries

**Contents:**
- Architecture overview (tools, resources, prompts roadmap)
- Tool reference (novanet_query, novanet_describe)
- Error handling patterns for LLM-friendly messages
- Token counting strategy (hybrid estimate/exact)
- Caching patterns
- Neo4j query patterns for agent bootstrap

**SKILL.md Draft:**

```markdown
---
name: novanet-mcp
description: NovaNet MCP Server patterns - RLM-on-KG, agent integration, knowledge graph queries. Use when working on MCP tools, resources, or AI agent features.
disable-model-invocation: false
user-invocable: true
allowed-tools: Bash, Read, Grep, Glob
argument-hint: [status|test|debug]
---

# NovaNet MCP Server

MCP server exposing NovaNet knowledge graph to AI agents.

## Quick Reference

| Component | Status | Location |
|-----------|--------|----------|
| novanet_query | Implemented | src/tools/query.rs |
| novanet_describe | Implemented | src/tools/describe.rs |
| Resources | Planned (Phase 2) | src/resources/mod.rs |
| Prompts | Planned (Phase 3) | src/prompts/mod.rs |

## Commands

Based on `$ARGUMENTS`:

### `status` (default)
Check server health and configuration:
\`\`\`bash
cd tools/novanet-mcp && cargo check
\`\`\`

### `test`
Run test suite:
\`\`\`bash
cd tools/novanet-mcp && cargo test
\`\`\`

### `debug`
Show debug configuration:
\`\`\`bash
cd tools/novanet-mcp && RUST_LOG=novanet_mcp=debug cargo run
\`\`\`

## Tool Patterns

### novanet_query
\`\`\`rust
// Good: Parameterized query with limit
QueryParams {
    cypher: "MATCH (e:Entity {key: $key}) RETURN e".to_string(),
    params: Some(params),
    limit: Some(10),
    timeout_ms: None,
}

// Bad: Dynamic query string (SQL injection risk)
format!("MATCH (e:Entity {{key: '{}'}}) RETURN e", user_input)
\`\`\`

### novanet_describe
\`\`\`rust
// Bootstrap sequence for new agent
describe("schema")    // 1. Understand schema
describe("relations") // 2. Learn arc types
describe("locales")   // 3. Know available locales
describe("stats")     // 4. Gauge graph size
\`\`\`

## Error Handling

\`\`\`rust
// Good: LLM-friendly error with action
Error::invalid_cypher("Use MATCH, not CREATE. The MCP server is read-only.")

// Good: Structured error with context
Error::Query {
    query: cypher.to_string(),
    source: neo4j_error
}

// Bad: Generic error
Error::Internal("Something went wrong".to_string())
\`\`\`

## Token Budget Pattern

\`\`\`rust
// Always check budget before returning large results
let estimate = counter.estimate(&json_result);
if estimate > budget {
    // Truncate or paginate
    let (truncated, actual) = counter.truncate_to_budget(&json_result, budget);
    return truncated;
}
\`\`\`
```

---

### 1.2 `rmcp-patterns` (Technology Skill)

**Location:** `.claude/skills/rmcp-patterns/SKILL.md`

**Trigger:** When implementing new MCP tools, resources, or prompts

**Contents:**
- rmcp 0.15 macro usage (#[tool], #[tool_router], #[tool_handler])
- ServerHandler implementation patterns
- Parameter validation with schemars
- Error response formatting

---

### 1.3 `neo4rs-patterns` (Technology Skill)

**Location:** `.claude/skills/neo4rs-patterns/SKILL.md`

**Trigger:** When working with Neo4j queries in Rust

**Contents:**
- Query building with parameters
- Row deserialization to serde_json::Value
- BoltType conversions
- Connection pool patterns
- Read-only query validation

---

## 2. CLAUDE.md Content (Implemented)

The CLAUDE.md file has been created at `/tools/novanet-mcp/CLAUDE.md` with:

- [x] Architecture overview diagram
- [x] Quick start (env vars, build, run)
- [x] Claude Code integration config
- [x] Tool documentation (novanet_query, novanet_describe)
- [x] Module architecture tree
- [x] Key dependencies table
- [x] Error handling reference
- [x] Token counting strategy
- [x] Caching documentation
- [x] Testing commands
- [x] Debugging guide (log levels, common issues)
- [x] NovaNet integration (schema overview, key queries)
- [x] Roadmap (Phase 1/2/3)
- [x] Related documentation links

---

## 3. Commands/Aliases

### Recommended Shell Aliases

Add to project `.envrc` or developer shell config:

```bash
# Build and run
alias mcp-build="cd $NOVANET_ROOT/tools/novanet-mcp && cargo build --release"
alias mcp-run="cd $NOVANET_ROOT/tools/novanet-mcp && cargo run"
alias mcp-debug="cd $NOVANET_ROOT/tools/novanet-mcp && RUST_LOG=novanet_mcp=debug cargo run"

# Testing
alias mcp-test="cd $NOVANET_ROOT/tools/novanet-mcp && cargo test"
alias mcp-test-watch="cd $NOVANET_ROOT/tools/novanet-mcp && cargo watch -x test"

# Linting
alias mcp-clippy="cd $NOVANET_ROOT/tools/novanet-mcp && cargo clippy -- -D warnings"
alias mcp-fmt="cd $NOVANET_ROOT/tools/novanet-mcp && cargo fmt"
```

### Recommended Claude Code Commands

**Location:** `.claude/commands/novanet-mcp.md`

```markdown
---
description: NovaNet MCP Server operations
argument-hint: [status|test|build|debug]
---

# NovaNet MCP Server

Based on `$ARGUMENTS`:

## status (default)
Check configuration and dependencies:
\`\`\`bash
cd tools/novanet-mcp && cargo check && cargo test --no-run
\`\`\`

## test
Run test suite:
\`\`\`bash
cd tools/novanet-mcp && cargo test
\`\`\`

## build
Build release binary:
\`\`\`bash
cd tools/novanet-mcp && cargo build --release
\`\`\`

## debug
Start with debug logging:
\`\`\`bash
cd tools/novanet-mcp && RUST_LOG=novanet_mcp=debug cargo run
\`\`\`
```

---

## 4. Essential Debugging Tips

### 4.1 MCP Protocol Debugging

```bash
# Capture raw MCP messages
RUST_LOG=rmcp=trace cargo run 2>mcp-debug.log

# Parse JSON-RPC requests/responses
cat mcp-debug.log | grep -E '(request|response)' | jq .
```

### 4.2 Neo4j Query Debugging

```bash
# Log all queries
RUST_LOG=novanet_mcp::neo4j=debug cargo run

# Test query in Neo4j Browser first
# http://localhost:7474
# Then copy validated query to code
```

### 4.3 Token Counting Debugging

```rust
// Add to query.rs for debugging
tracing::debug!(
    estimate = %counter.estimate(&json_string),
    exact = %counter.count(&json_string),
    len = %json_string.len(),
    "Token count comparison"
);
```

### 4.4 Cache Debugging

```rust
// Check cache stats
let stats = state.cache().stats();
tracing::info!(
    entries = %stats.entry_count,
    weighted_size = %stats.weighted_size,
    "Cache statistics"
);
```

---

## 5. Error Reporting for LLM Agents

### 5.1 Design Principles

1. **Actionable**: Tell the agent what to do differently
2. **Contextual**: Include relevant query/params in error
3. **Categorized**: Use appropriate JSON-RPC error codes
4. **Concise**: One sentence, no stack traces

### 5.2 Error Message Templates

```rust
// Query validation error
"Invalid Cypher: Query contains {keyword}. Only read operations (MATCH, RETURN, WITH, WHERE, ORDER BY, LIMIT) are allowed."

// Entity not found
"Entity not found: {key}. Use novanet_describe with describe='schema' to see available entity types."

// Token budget
"Token budget exceeded: Result is {used} tokens but budget is {budget}. Add LIMIT clause or use pagination."

// Connection error
"Neo4j connection failed. Ensure Neo4j is running at {uri} and NOVANET_MCP_NEO4J_PASSWORD is set."

// Query execution error
"Query execution failed: {neo4j_error}. Check Cypher syntax at http://localhost:7474."
```

### 5.3 Error Response Structure

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32602,
    "message": "Invalid Cypher: Query contains DELETE. Only read operations are allowed.",
    "data": null
  }
}
```

---

## 6. Integration Patterns with NovaNet

### 6.1 Schema Discovery Pattern

```
Agent                    MCP Server                    Neo4j
  |                          |                           |
  |-- describe(schema) ----->|                           |
  |                          |-- MATCH (k:Kind) -------->|
  |                          |<-- {realms, layers} ------|
  |<-- schema overview ------|                           |
  |                          |                           |
  |-- describe(relations) -->|                           |
  |                          |-- MATCH (a:ArcKind) ----->|
  |                          |<-- {families, arcs} ------|
  |<-- arc definitions ------|                           |
```

### 6.2 Entity Exploration Pattern

```
Agent                    MCP Server                    Neo4j
  |                          |                           |
  |-- query(entity by key) ->|                           |
  |                          |-- MATCH (e:Entity) ------>|
  |                          |<-- entity with relations -|
  |<-- entity context -------|                           |
  |                          |                           |
  |-- query(related atoms) ->|                           |
  |                          |-- MATCH ...-[:USES_*]-->--|
  |                          |<-- terms, expressions ----|
  |<-- knowledge atoms ------|                           |
```

### 6.3 Locale-Aware Generation Pattern

```
Agent                    MCP Server                    Neo4j
  |                          |                           |
  |-- describe(locales) ---->|                           |
  |                          |-- MATCH (l:Locale) ------>|
  |<-- available locales ----|                           |
  |                          |                           |
  |-- query(locale atoms) -->|                           |
  |                          |-- MATCH...-[:HAS_TERMS]-->|
  |<-- locale knowledge -----|                           |
  |                          |                           |
  |   [Generate content with locale context]             |
```

---

## 7. Recommended File Structure

```
tools/novanet-mcp/
├── CLAUDE.md                 # Main DX documentation (created)
├── Cargo.toml
├── Cargo.lock
├── .gitignore
├── src/
│   └── ...
├── tests/
│   └── integration/          # Integration tests (needs Neo4j)
│       └── mod.rs
└── docs/                     # Additional documentation
    ├── DX-AUDIT-PLAN.md      # This file
    ├── TOOLS.md              # Detailed tool documentation
    ├── ERRORS.md             # Error handling guide
    └── INTEGRATION.md        # NovaNet integration patterns
```

---

## 8. Implementation Priority

| Priority | Item | Effort | Impact |
|----------|------|--------|--------|
| P0 | CLAUDE.md | Done | High |
| P1 | novanet-mcp skill | 30 min | High |
| P2 | /novanet-mcp command | 15 min | Medium |
| P3 | Integration tests | 2 hrs | High |
| P4 | rmcp-patterns skill | 1 hr | Medium |
| P5 | neo4rs-patterns skill | 1 hr | Medium |
| P6 | Shell aliases | 5 min | Low |
| P7 | Additional docs | 1 hr | Medium |

---

## 9. Next Steps

1. **Create the novanet-mcp skill** at `.claude/skills/novanet-mcp/SKILL.md`
2. **Create the /novanet-mcp command** at `.claude/commands/novanet-mcp.md`
3. **Update `.claude/skills/INDEX.md`** to include new skill
4. **Update `.claude/README.md`** to document MCP server
5. **Add integration tests** with Neo4j test fixtures
6. **Create rmcp-patterns skill** if more MCP work planned
