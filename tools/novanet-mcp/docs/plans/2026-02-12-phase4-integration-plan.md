# Phase 4: Integration Testing, Claude Code Setup, Performance Benchmarks

**Date**: 2026-02-12
**Status**: Implementation Plan

---

## Overview

Phase 4 validates the Phase 3 implementation through:
- **A**: Integration tests with real Neo4j seed data
- **B**: Claude Code MCP server configuration
- **C**: Performance benchmarks for latency and token efficiency

---

## A: Integration Tests with Neo4j Seed Data

### A.1: New Test Module Structure

Add to `tests/integration_tests.rs`:

```rust
mod generate_tool {
    // Test novanet_generate with block mode
    // Test novanet_generate with page mode
    // Test context anchors resolution
    // Test token budget enforcement
}

mod prompts {
    // Test all 6 prompts render correctly
    // Test prompt argument validation
    // Test prompt message structure
}

mod tools_with_seed_data {
    // Test traverse with real Kind nodes
    // Test search with Entity nodes
    // Test assemble with locale knowledge
    // Test atoms retrieval
}
```

### A.2: Test Cases

| Test | Description | Validates |
|------|-------------|-----------|
| `test_generate_block_mode` | Generate context for a Block | novanet_generate block mode |
| `test_generate_page_mode` | Generate context for a Page | novanet_generate page mode |
| `test_generate_context_anchors` | Verify REFERENCES_PAGE resolution | Context anchor system |
| `test_generate_token_budget` | Verify budget enforcement | Token management |
| `test_prompt_cypher_query` | Render cypher_query prompt | Prompt system |
| `test_prompt_block_generation` | Render block_generation prompt | Prompt with args |
| `test_traverse_with_kinds` | Traverse from Kind node | Meta-graph traversal |
| `test_search_entities` | Search for Entity nodes | Fulltext search |
| `test_assemble_locale_context` | Assemble fr-FR context | Locale knowledge |

### A.3: Implementation

```rust
mod generate_tool {
    use super::*;

    #[tokio::test]
    async fn test_generate_block_mode() {
        require_neo4j!();
        // 1. Create State
        // 2. Call generate::execute with block mode
        // 3. Verify prompt structure
        // 4. Verify evidence packets
        // 5. Verify token counts
    }
}
```

---

## B: Claude Code Integration

### B.1: Configuration File

Create `.claude/settings.local.json` (gitignored):

```json
{
  "mcpServers": {
    "novanet": {
      "command": "/Users/thibaut/supernovae-st/novanet-hq/tools/novanet-mcp/target/release/novanet-mcp",
      "env": {
        "NOVANET_MCP_NEO4J_URI": "bolt://localhost:7687",
        "NOVANET_MCP_NEO4J_USER": "neo4j",
        "NOVANET_MCP_NEO4J_PASSWORD": "novanetpassword"
      }
    }
  }
}
```

### B.2: Build Release Binary

```bash
cargo build --release
```

### B.3: Validation Script

Create `scripts/validate-mcp.sh`:

```bash
#!/usr/bin/env bash
# Validate MCP server starts and responds to initialize

echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"capabilities":{}}}' | \
  NOVANET_MCP_NEO4J_PASSWORD=novanetpassword ./target/release/novanet-mcp

# Expected: JSON response with server capabilities
```

### B.4: DX Documentation

Update CLAUDE.md with:
- Integration instructions
- Environment variable reference
- Troubleshooting guide

---

## C: Performance Benchmarks

### C.1: Benchmark Module

Create `benches/mcp_benchmarks.rs`:

```rust
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn bench_token_counting(c: &mut Criterion) {
    // Benchmark token estimation vs exact counting
}

fn bench_query_execution(c: &mut Criterion) {
    // Benchmark Neo4j query latency
}

fn bench_generate_tool(c: &mut Criterion) {
    // Benchmark full generate pipeline
}

criterion_group!(benches,
    bench_token_counting,
    bench_query_execution,
    bench_generate_tool
);
criterion_main!(benches);
```

### C.2: Metrics to Measure

| Metric | Target | Method |
|--------|--------|--------|
| Token estimate latency | < 100μs | criterion |
| Token exact count latency | < 1ms | criterion |
| Simple query latency | < 50ms | criterion |
| Schema query latency | < 100ms | criterion |
| generate (block) latency | < 500ms | criterion |
| generate (page) latency | < 2s | criterion |

### C.3: Cargo.toml Addition

```toml
[dev-dependencies]
criterion = { version = "0.6", features = ["html_reports"] }

[[bench]]
name = "mcp_benchmarks"
harness = false
```

---

## Implementation Order

1. **A.1-A.3**: Add integration tests (tests/integration_tests.rs)
2. **C.3**: Add criterion dependency
3. **C.1-C.2**: Create benchmark module
4. **B.1**: Create Claude Code config
5. **B.2**: Build release binary
6. **B.3**: Create validation script
7. Run all tests and benchmarks
8. Commit

---

## Success Criteria

- [ ] All 24+ existing integration tests pass
- [ ] 9 new integration tests pass
- [ ] Claude Code can connect to MCP server
- [ ] novanet_describe returns schema
- [ ] Benchmarks show acceptable latency
- [ ] Documentation updated

---

## Files to Create/Modify

| File | Action |
|------|--------|
| `tests/integration_tests.rs` | Add generate/prompt tests |
| `Cargo.toml` | Add criterion |
| `benches/mcp_benchmarks.rs` | Create benchmark suite |
| `.claude/settings.local.json.example` | Create example config |
| `scripts/validate-mcp.sh` | Create validation script |
| `CLAUDE.md` | Update integration docs |
