# Plan: Fix NovaNet MCP "Entity not found" Gap

**Date:** 2026-02-19
**Status:** Ready for execution
**Priority:** High (blocks Nika↔NovaNet integration)
**Estimated Effort:** 4-6 hours

---

## Problem Statement

The `novanet_generate` MCP tool returns "Entity not found" errors for entities that **exist** in Neo4j.

```bash
# Verified entities exist:
docker exec novanet-neo4j cypher-shell -u neo4j -p novanetpassword \
  "MATCH (e:Entity) RETURN e.key LIMIT 5"
# Returns: qr-code, smart-link, barcode, ...

# But novanet_generate fails:
Error: Entity not found: qr-code
```

This is a **NovaNet MCP server** issue, not a Nika issue. The integration works (we can connect, list tools, call novanet_describe), but entity lookup in novanet_generate is broken.

---

## Root Cause Hypothesis

Based on codebase knowledge, likely causes:

1. **Cypher query mismatch**: Query uses wrong property (e.g., `name` vs `key`)
2. **Label mismatch**: Query expects `:Entity:Semantic` but data has `:Entity` only
3. **Case sensitivity**: Query expects lowercase but data has mixed case
4. **Missing index**: Query times out without proper index

---

## Investigation Plan

### Phase 1: Locate the Bug (30 min)

**Agent: code-explorer** - Trace the novanet_generate execution path

```
Task: Trace novanet_generate from MCP handler to Cypher query

Files to examine:
- novanet-dev/tools/novanet-mcp/src/tools/generate.rs
- novanet-dev/tools/novanet-mcp/src/handlers/mod.rs
- novanet-dev/packages/db/src/queries/entity.ts (if TypeScript fallback)

Questions to answer:
1. What Cypher query is executed for entity lookup?
2. What property is used for matching (key, name, slug)?
3. Are there label constraints that might filter out valid entities?
4. Is there locale-specific filtering that might fail?
```

### Phase 2: Reproduce with Logging (30 min)

**Manual debugging steps:**

```bash
# 1. Enable trace logging in NovaNet MCP
RUST_LOG=trace cargo run --release 2>&1 | tee mcp-debug.log

# 2. Call novanet_generate via test
cd nika-dev/tools/nika
NOVANET_MCP_NEO4J_PASSWORD=novanetpassword \
  cargo test --test rig_integration_test test_real_mcp_novanet_describe -- --ignored

# 3. Check the log for the actual Cypher query
grep -A5 "MATCH" mcp-debug.log
```

### Phase 3: Fix the Query (1-2 hours)

**Agent: code-reviewer** - Review and fix the Cypher query

Once we identify the broken query, fix with TDD:

```rust
// Expected fix pattern in generate.rs
async fn lookup_entity(key: &str) -> Result<Entity> {
    // BEFORE (broken):
    // let query = "MATCH (e:Entity {name: $key}) RETURN e";

    // AFTER (fixed):
    let query = "MATCH (e:Entity {key: $key}) RETURN e";
    // OR with proper labels:
    let query = "MATCH (e:Entity:Semantic {key: $key}) RETURN e";
}
```

### Phase 4: Add Integration Tests (1 hour)

**Location:** `novanet-dev/tools/novanet-mcp/tests/`

```rust
#[tokio::test]
async fn test_generate_finds_existing_entity() {
    // Setup: ensure entity exists
    let entity_key = "qr-code";

    // Act: call novanet_generate
    let result = call_tool("novanet_generate", json!({
        "focus_key": entity_key,
        "locale": "fr-FR",
        "forms": ["text", "title"]
    })).await;

    // Assert: should succeed, not "not found"
    assert!(result.is_ok());
    assert!(!result.unwrap().contains("not found"));
}
```

### Phase 5: Validate denomination_forms (1 hour)

Per **ADR-033**, the response MUST include `denomination_forms`:

```json
{
  "entity": "qr-code",
  "locale": "fr-FR",
  "denomination_forms": {
    "text": "code QR",
    "title": "Code QR",
    "abbrev": "QR",
    "url": "code-qr"
  }
}
```

**Validation steps:**
1. Check if `denomination_forms` is returned in response
2. If missing, add to `generate.rs` response builder
3. Pull forms from `EntityNative` node properties

---

## Execution Checklist

| Step | Agent/Tool | Verification |
|------|------------|--------------|
| 1. Explore generate.rs | code-explorer | Print actual Cypher query |
| 2. Run with RUST_LOG=trace | Manual | Capture query in logs |
| 3. Compare query vs schema | Manual | Check property name mismatch |
| 4. Write failing test | TDD | Test shows "not found" |
| 5. Fix Cypher query | Manual edit | Test passes |
| 6. Add denomination_forms | Manual edit | Response includes forms |
| 7. Code review | code-reviewer | Review changes |
| 8. Run full test suite | CI | All tests pass |
| 9. Update ROADMAP.md | Manual | Mark gap as resolved |

---

## Files to Modify

| File | Change |
|------|--------|
| `novanet-dev/tools/novanet-mcp/src/tools/generate.rs` | Fix entity lookup query |
| `novanet-dev/tools/novanet-mcp/src/tools/generate.rs` | Add denomination_forms to response |
| `novanet-dev/tools/novanet-mcp/tests/generate_test.rs` | Add integration test |
| `supernovae-agi/ROADMAP.md` | Update MVP 4 status |

---

## Success Criteria

1. `cargo test --test rig_integration_test` passes ALL 6 tests without "Entity not found"
2. `novanet_generate` response includes `denomination_forms` per ADR-033
3. NovaNet MCP tests in novanet-dev pass
4. ROADMAP MVP 4 shows 100% completion

---

## Fallback Plan

If entity lookup requires deeper changes (e.g., schema migration):

1. Document the exact issue in an ADR
2. Create mock data that works with current query
3. Mark as "known limitation" in ROADMAP
4. Prioritize fix for next sprint
