# Critical Test Coverage Plan

**Date:** 2026-02-19
**Target:** Close test gaps in Nika runner/output and NovaNet MCP handler
**Priority:** Critical (production paths with 0 tests)

---

## Executive Summary

| File | Current | Gap | Priority | Effort |
|------|---------|-----|----------|--------|
| `novanet-mcp/handler.rs` | 0 tests | 100% | **P0** | High |
| `nika/runtime/output.rs` | 3 tests | ~60% | P1 | Low |
| `nika/runtime/runner.rs` | ~18 tests | ~20% | P2 | Medium |

---

## 1. NovaNet MCP Handler (P0 Critical)

**File:** `novanet-dev/tools/novanet-mcp/src/server/handler.rs`
**Lines:** 343 | **Tests:** 0

### What Needs Testing

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  HANDLER.RS TEST REQUIREMENTS                                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  7 MCP Tools (each needs success + error tests):                            │
│  ├── novanet_query      → Cypher execution                                  │
│  ├── novanet_describe   → Schema bootstrap                                  │
│  ├── novanet_search     → Fulltext search                                   │
│  ├── novanet_traverse   → Graph traversal                                   │
│  ├── novanet_assemble   → Context assembly                                  │
│  ├── novanet_atoms      → Knowledge atoms                                   │
│  └── novanet_generate   → RLM-on-KG context                                 │
│                                                                             │
│  ServerHandler Implementation:                                              │
│  ├── get_info()         → Server capabilities                               │
│  ├── list_prompts()     → 6 prompts returned                                │
│  └── get_prompt()       → Prompt rendering with args                        │
│                                                                             │
│  Helper Functions:                                                          │
│  ├── convert_prompt_definition()                                            │
│  └── convert_prompt_message()                                               │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Test Strategy

Since tools delegate to `crate::tools::{module}::execute()`, we test:
1. **Handler routing** - Tools dispatch correctly
2. **Error mapping** - Errors convert to MCP JSON-RPC format
3. **Response formatting** - JSON serialization works
4. **Prompts** - List and render correctly

### Test Cases

```rust
// Location: novanet-dev/tools/novanet-mcp/tests/handler_test.rs

// ══════════════════════════════════════════════════════════════
// TOOL ROUTING TESTS (mock State, verify dispatch)
// ══════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_novanet_query_routes_to_query_module()

#[tokio::test]
async fn test_novanet_describe_routes_to_describe_module()

#[tokio::test]
async fn test_novanet_search_routes_to_search_module()

#[tokio::test]
async fn test_novanet_traverse_routes_to_traverse_module()

#[tokio::test]
async fn test_novanet_assemble_routes_to_assemble_module()

#[tokio::test]
async fn test_novanet_atoms_routes_to_atoms_module()

#[tokio::test]
async fn test_novanet_generate_routes_to_generate_module()

// ══════════════════════════════════════════════════════════════
// ERROR MAPPING TESTS
// ══════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_tool_error_maps_to_mcp_error_code_32000()

#[tokio::test]
async fn test_serialization_error_maps_to_code_32603()

// ══════════════════════════════════════════════════════════════
// SERVER INFO TESTS
// ══════════════════════════════════════════════════════════════

#[test]
fn test_get_info_returns_server_capabilities()

#[test]
fn test_get_info_enables_tools_and_prompts()

// ══════════════════════════════════════════════════════════════
// PROMPT TESTS
// ══════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_list_prompts_returns_all_6_prompts()

#[tokio::test]
async fn test_get_prompt_cypher_query_renders_correctly()

#[tokio::test]
async fn test_get_prompt_not_found_returns_error()

// ══════════════════════════════════════════════════════════════
// HELPER FUNCTION TESTS
// ══════════════════════════════════════════════════════════════

#[test]
fn test_convert_prompt_definition_maps_all_fields()

#[test]
fn test_convert_prompt_message_user_role()

#[test]
fn test_convert_prompt_message_assistant_role()
```

### Implementation Notes

- Mock `State` to avoid Neo4j dependency
- Focus on routing correctness, not tool logic (tools have own tests)
- Verify MCP error code mapping (-32000, -32001, -32603)

---

## 2. Nika Output Module (P1)

**File:** `nika-dev/tools/nika/src/runtime/output.rs`
**Lines:** 173 | **Tests:** 3 (inline)

### Existing Tests

```rust
✓ schema_cache_works
✓ schema_validation_rejects_invalid
✓ make_task_result_validates_json
```

### Missing Tests

```rust
// Add to output.rs #[cfg(test)] module

// ══════════════════════════════════════════════════════════════
// make_task_result EDGE CASES
// ══════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_make_task_result_no_policy_returns_text()

#[tokio::test]
async fn test_make_task_result_json_no_schema_parses_json()

#[tokio::test]
async fn test_make_task_result_invalid_json_returns_error_with_code()

#[tokio::test]
async fn test_make_task_result_text_format_returns_raw_string()

// ══════════════════════════════════════════════════════════════
// validate_schema ERROR PATHS
// ══════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_validate_schema_file_not_found_returns_error()

#[tokio::test]
async fn test_validate_schema_invalid_json_in_schema_file()

#[tokio::test]
async fn test_validate_schema_invalid_schema_structure()

#[tokio::test]
async fn test_validate_schema_multiple_validation_errors()

// ══════════════════════════════════════════════════════════════
// EDGE CASES
// ══════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_make_task_result_large_json_output()

#[tokio::test]
async fn test_make_task_result_unicode_content()

#[tokio::test]
async fn test_schema_cache_concurrent_access()
```

---

## 3. Nika Runner Module (P2)

**File:** `nika-dev/tools/nika/src/runtime/runner.rs`
**Lines:** 904 | **Tests:** ~18 (inline)

### Existing Tests

```rust
✓ test_for_each_collects_all_results
✓ test_for_each_preserves_order
✓ event_sequence_for_single_task
✓ event_sequence_for_chained_tasks
✓ event_sequence_for_parallel_tasks
✓ event_ids_are_monotonic
✓ timestamps_are_relative_and_increasing
✓ failed_task_emits_task_failed_event
✓ template_resolved_event_captures_before_and_after
✓ event_log_to_json_serializes_correctly
```

### Missing Unit Tests

```rust
// Add to runner.rs #[cfg(test)] module

// ══════════════════════════════════════════════════════════════
// get_ready_tasks UNIT TESTS
// ══════════════════════════════════════════════════════════════

#[test]
fn test_get_ready_tasks_returns_empty_when_all_done()

#[test]
fn test_get_ready_tasks_skips_failed_dependencies()

#[test]
fn test_get_ready_tasks_returns_multiple_independent_tasks()

// ══════════════════════════════════════════════════════════════
// all_done UNIT TESTS
// ══════════════════════════════════════════════════════════════

#[test]
fn test_all_done_false_when_tasks_pending()

#[test]
fn test_all_done_true_when_all_completed()

// ══════════════════════════════════════════════════════════════
// get_final_output UNIT TESTS
// ══════════════════════════════════════════════════════════════

#[test]
fn test_get_final_output_returns_first_successful()

#[test]
fn test_get_final_output_returns_none_when_all_failed()

// ══════════════════════════════════════════════════════════════
// DEADLOCK DETECTION
// ══════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_run_detects_deadlock_with_failed_dependency()

// ══════════════════════════════════════════════════════════════
// for_each FAIL_FAST
// ══════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_for_each_fail_fast_stops_remaining_iterations()

#[tokio::test]
async fn test_for_each_no_fail_fast_continues_after_error()

// ══════════════════════════════════════════════════════════════
// BINDING ERRORS
// ══════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_execute_task_iteration_binding_error_emits_task_failed()
```

---

## Implementation Order

1. **handler.rs tests** (most critical, 0 coverage)
2. **output.rs tests** (quick wins, low effort)
3. **runner.rs tests** (medium effort, good existing coverage)

---

## Success Criteria

| Metric | Target |
|--------|--------|
| handler.rs coverage | 15+ tests, all tools + prompts |
| output.rs coverage | 10+ tests, all error paths |
| runner.rs coverage | 25+ tests, unit tests for private methods |
| All tests pass | `cargo test` green |
| No regressions | Existing 670+ tests still pass |

---

## Estimated Effort

| Task | Time |
|------|------|
| handler.rs tests | 45 min |
| output.rs tests | 20 min |
| runner.rs tests | 30 min |
| Total | ~1.5 hours |
