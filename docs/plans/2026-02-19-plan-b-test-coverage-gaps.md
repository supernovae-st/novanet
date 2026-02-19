# Plan B: Test Coverage Gaps

**Date:** 2026-02-19
**Status:** Ready for Execution
**Effort:** ~4-6 hours
**Methodology:** TDD (Red-Green-Refactor)

---

## Overview

L'audit a identifié des modules avec une couverture de tests insuffisante dans Nika v0.5.0.

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  TEST COVERAGE GAPS IDENTIFIED                                                  │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Module              Current Tests    Target    Gap                             │
│  ─────────────────────────────────────────────────────────────────────────────  │
│  store/              7                20+       13+ tests needed                │
│  provider/           8                15+       7+ tests needed                 │
│  mcp/ describe       1                5+        4+ tests needed                 │
│  mcp/ atoms          1                5+        4+ tests needed                 │
│  runtime/executor    Low coverage     10+       Token tracking = 0 bug          │
│                                                                                 │
│  TOTAL EFFORT: ~30 new tests                                                    │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Phase 1: Store Module (7 → 20+ tests)

**Location:** `nika-dev/tools/nika/src/store/`

### Current State
- 7 tests existing
- DataStore, TaskOutputStore, ConfigStore

### Tasks

| # | Task | Tests to Add | Status |
|---|------|--------------|--------|
| 1.1 | DataStore concurrent access | 3 tests | ⏳ |
| 1.2 | DataStore serialization/deserialization | 2 tests | ⏳ |
| 1.3 | TaskOutputStore ordering guarantees | 3 tests | ⏳ |
| 1.4 | TaskOutputStore memory limits | 2 tests | ⏳ |
| 1.5 | ConfigStore reload behavior | 3 tests | ⏳ |

### Test Templates

```rust
// tests/store_concurrent_test.rs

#[tokio::test]
async fn test_datastore_concurrent_writes() {
    let store = DataStore::new();
    let store = Arc::new(store);

    let handles: Vec<_> = (0..100).map(|i| {
        let store = store.clone();
        tokio::spawn(async move {
            store.set(&format!("key_{}", i), json!({"value": i}));
        })
    }).collect();

    for h in handles { h.await.unwrap(); }

    // All 100 keys should exist
    for i in 0..100 {
        assert!(store.get(&format!("key_{}", i)).is_some());
    }
}

#[tokio::test]
async fn test_datastore_concurrent_reads_writes() {
    // Test that reads don't block during writes
}

#[test]
fn test_datastore_serialization_roundtrip() {
    let store = DataStore::new();
    store.set("nested", json!({"a": {"b": {"c": 1}}}));

    let serialized = store.serialize().unwrap();
    let restored = DataStore::deserialize(&serialized).unwrap();

    assert_eq!(store.get("nested"), restored.get("nested"));
}
```

---

## Phase 2: Provider Module (8 → 15+ tests)

**Location:** `nika-dev/tools/nika/src/provider/`

### Current State
- 8 tests existing
- RigProvider wrapper (761 lines)

### Tasks

| # | Task | Tests to Add | Status |
|---|------|--------------|--------|
| 2.1 | RigProvider error handling | 3 tests | ⏳ |
| 2.2 | RigProvider model selection | 2 tests | ⏳ |
| 2.3 | RigProvider streaming behavior | 2 tests | ⏳ |

### Test Templates

```rust
// tests/provider_error_test.rs

#[tokio::test]
async fn test_rig_provider_handles_rate_limit() {
    let provider = RigProvider::new_mock();
    provider.set_response(MockResponse::RateLimit { retry_after: 5 });

    let result = provider.chat(messages).await;

    assert!(matches!(result, Err(NikaError::RateLimited { .. })));
}

#[tokio::test]
async fn test_rig_provider_handles_invalid_api_key() {
    let provider = RigProvider::with_invalid_key();

    let result = provider.chat(messages).await;

    assert!(matches!(result, Err(NikaError::AuthenticationError { .. })));
}

#[tokio::test]
async fn test_rig_provider_handles_context_too_long() {
    let provider = RigProvider::new_mock();
    let huge_message = "x".repeat(1_000_000);

    let result = provider.chat(vec![Message::user(huge_message)]).await;

    assert!(matches!(result, Err(NikaError::ContextTooLong { .. })));
}
```

---

## Phase 3: MCP Tools Integration (2 → 10+ tests)

**Location:** `nika-dev/tools/nika/tests/`

### Current State
- novanet_describe: 1 test
- novanet_atoms: 1 test
- Other MCP tools have better coverage

### Tasks

| # | Task | Tests to Add | Status |
|---|------|--------------|--------|
| 3.1 | novanet_describe edge cases | 4 tests | ⏳ |
| 3.2 | novanet_atoms edge cases | 4 tests | ⏳ |

### Test Templates

```rust
// tests/mcp_describe_test.rs

#[tokio::test]
async fn test_describe_schema_returns_all_classes() {
    let client = MockMcpClient::new();

    let result = client.call_tool("novanet_describe", json!({
        "describe": "schema"
    })).await.unwrap();

    let classes = result["data"]["classes"].as_array().unwrap();
    assert!(classes.len() >= 61); // All 61 node classes
}

#[tokio::test]
async fn test_describe_entity_not_found() {
    let client = MockMcpClient::new();

    let result = client.call_tool("novanet_describe", json!({
        "describe": "entity",
        "entity_key": "non-existent-key"
    })).await;

    assert!(matches!(result, Err(McpError::NotFound { .. })));
}

#[tokio::test]
async fn test_describe_relations_returns_arc_info() {
    let client = MockMcpClient::new();

    let result = client.call_tool("novanet_describe", json!({
        "describe": "relations"
    })).await.unwrap();

    let arcs = result["data"]["arcs"].as_array().unwrap();
    assert!(arcs.len() >= 182); // All 182 arc classes
}

// tests/mcp_atoms_test.rs

#[tokio::test]
async fn test_atoms_returns_terms_for_locale() {
    let client = MockMcpClient::new();

    let result = client.call_tool("novanet_atoms", json!({
        "locale": "fr-FR",
        "atom_type": "term"
    })).await.unwrap();

    assert!(result["atoms"].as_array().unwrap().len() > 0);
}

#[tokio::test]
async fn test_atoms_invalid_locale_returns_error() {
    let client = MockMcpClient::new();

    let result = client.call_tool("novanet_atoms", json!({
        "locale": "invalid-locale",
        "atom_type": "term"
    })).await;

    assert!(result.is_err());
}
```

---

## Phase 4: Token Tracking Bug Fix

**Location:** `nika-dev/tools/nika/src/runtime/executor.rs:380-385`

### Current Bug

```rust
// executor.rs:380-385 — Currently returns 0
AgentTurnMetadata {
    input_tokens: 0,   // BUG: Always 0
    output_tokens: 0,  // BUG: Always 0
    thinking: None,
}
```

### Tasks

| # | Task | Tests to Add | Status |
|---|------|--------------|--------|
| 4.1 | Write failing test for token tracking | 1 test | ⏳ |
| 4.2 | Fix token extraction from rig-core | Fix | ⏳ |
| 4.3 | Verify token counts are accurate | 1 test | ⏳ |

### Test Template (RED first)

```rust
// tests/token_tracking_test.rs

#[tokio::test]
async fn test_token_tracking_returns_nonzero() {
    let workflow = load_workflow("examples/simple-agent.yaml");
    let result = Runner::new(workflow).run().await.unwrap();

    // Find agent task output
    let agent_output = result.task_outputs.get("agent_task").unwrap();
    let metadata = agent_output.metadata.as_ref().unwrap();

    // CRITICAL: Token counts must be non-zero after agent execution
    assert!(metadata.input_tokens > 0, "input_tokens should be > 0");
    assert!(metadata.output_tokens > 0, "output_tokens should be > 0");
}
```

### Fix Strategy

```rust
// rig_agent_loop.rs — Extract tokens from streaming

impl RigAgentLoop {
    async fn extract_tokens(&self, response: &StreamedAssistantContent) -> TokenUsage {
        match response {
            StreamedAssistantContent::Final { usage, .. } => {
                TokenUsage {
                    input_tokens: usage.input_tokens,
                    output_tokens: usage.output_tokens,
                }
            }
            _ => TokenUsage::default(),
        }
    }
}
```

---

## Verification Checklist

### Phase 1: Store Module
```bash
cargo test store --all-features
# Expected: 20+ tests passing
```

### Phase 2: Provider Module
```bash
cargo test provider --all-features
# Expected: 15+ tests passing
```

### Phase 3: MCP Tools
```bash
cargo test mcp_describe mcp_atoms --all-features
# Expected: 10+ tests passing
```

### Phase 4: Token Tracking
```bash
cargo test token_tracking --all-features
# Expected: Non-zero token counts
```

### Final
```bash
cargo test --all-features
# Expected: 730+ tests (was 703)
```

---

## Success Criteria

| Metric | Before | After |
|--------|--------|-------|
| Total Nika tests | 703 | 730+ |
| Store module tests | 7 | 20+ |
| Provider module tests | 8 | 15+ |
| MCP describe tests | 1 | 5+ |
| MCP atoms tests | 1 | 5+ |
| Token tracking bug | Returns 0 | Returns actual counts |

---

## Execution Order

1. **Phase 4 FIRST** (Token tracking) — It's a bug, not coverage
2. **Phase 1** (Store) — Core infrastructure
3. **Phase 2** (Provider) — LLM integration
4. **Phase 3** (MCP) — External tool integration

---

## Notes

- Use TDD: Write failing test first, then implement
- Each test file should be self-contained
- Mock external services (Neo4j, LLM APIs)
- Run `cargo test` after each phase to verify
