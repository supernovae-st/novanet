# MCP TIER 1 Foundation Plan

**Date:** 2026-02-19
**Target Version:** NovaNet MCP v0.14.1
**Effort:** ~4-6 hours
**Status:** In Progress

---

## Overview

Upgrade NovaNet MCP server with foundational improvements based on 2025 best practices research:

1. **rmcp 0.15 → 0.16** — Streaming HTTP, OAuth foundations
2. **tracing integration** — Structured observability
3. **SDK pattern alignment** — Follow official patterns

## Research Basis

- rmcp v0.16.0 official documentation
- Tokio patterns (State of the Crates 2025)
- MCP-use patterns (MCPRouter, streaming)

---

## Phase 1: rmcp Upgrade

**Effort:** ~1-2 hours

### Changes

```toml
# Cargo.toml
[dependencies]
rmcp = "0.16"  # was 0.15
```

### Breaking Changes in rmcp 0.16

| Change | Impact | Action |
|--------|--------|--------|
| Content::text() returns AnnotatedContent | Low | Update tool responses if needed |
| OAuth support added (RFC 8707) | None | Optional feature, not enabling yet |
| Streaming HTTP transport | None | Available for future use |

### Tasks

- [ ] Update Cargo.toml: rmcp 0.15 → 0.16
- [ ] Fix any Content API breaking changes
- [ ] Run `cargo build`, fix compilation errors
- [ ] Run `cargo test`, ensure 335 tests pass

---

## Phase 2: tracing Integration

**Effort:** ~2-3 hours

### Dependencies

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt"] }
```

### Implementation Pattern

```rust
use tracing::{info_span, instrument, warn, error};

#[instrument(skip(state), fields(tool = "novanet_generate"))]
pub async fn handle_generate(
    state: &State,
    params: GenerateParams,
) -> Result<GenerateResult, McpError> {
    let _query_span = info_span!("neo4j_query", cypher = %query).entered();

    // Query execution with automatic timing
    let result = state.pool.execute(query).await?;

    Ok(result)
}
```

### Tasks

- [ ] Add tracing + tracing-subscriber to Cargo.toml
- [ ] Initialize subscriber in main.rs
- [ ] Add `#[instrument]` to 7 tool handlers
- [ ] Add spans for Neo4j queries
- [ ] Add error event logging with context
- [ ] Test with `RUST_LOG=debug`

### Observability Levels

| Level | Content |
|-------|---------|
| ERROR | Tool failures, Neo4j connection errors |
| WARN | Token budget exceeded, fallback behaviors |
| INFO | Tool calls, response summaries |
| DEBUG | Full params, query details |
| TRACE | Individual query steps |

---

## Phase 3: SDK Alignment

**Effort:** ~1 hour

### Patterns to Verify

| Pattern | Current | SDK 0.16 | Action |
|---------|---------|----------|--------|
| Handler trait | ✅ NovaNetHandler | Handler trait | Keep |
| Error handling | Static strings | rmcp::model::Error | Minor update |
| Tool schema | schemars derive | schemars derive | Keep |
| State management | Arc<Pool> | Arc pattern | Keep |

### Tasks

- [ ] Review rmcp 0.16 examples for patterns
- [ ] Update error handling to use proper error codes
- [ ] Update CLAUDE.md with new version
- [ ] Update test assertions if needed

---

## Success Criteria

- [ ] `cargo build` succeeds with rmcp 0.16
- [ ] All 335 tests pass
- [ ] `RUST_LOG=debug cargo run` shows structured logs
- [ ] Tool calls produce observable spans
- [ ] Documentation updated

---

## Future (TIER 2+)

After TIER 1 complete:

- **B2: novanet_introspect** — Schema introspection for agents (MVP 8 Phase 3)
- **A4: Response streaming** — For large traversals
- **B3: Subscriptions** — Graph change notifications

---

## References

- [rmcp 0.16.0 changelog](https://crates.io/crates/rmcp)
- [tracing crate](https://crates.io/crates/tracing)
- ADR-033: Denomination Forms
- ADR-035: context_build_log
