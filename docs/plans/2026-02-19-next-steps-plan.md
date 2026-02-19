# Next Steps Plan - 2026-02-19

**Context:** Other session completing MVP 6 documentation polish
**This Plan:** What to do AFTER MVP 6 is complete

---

## Current Status

```
┌────────────────────────────────────────────────────────────────────────────────┐
│  STATUS AS OF 2026-02-19                                                       │
├────────────────────────────────────────────────────────────────────────────────┤
│                                                                                │
│  ✅ DONE                             ⏳ IN PROGRESS (Other Session)            │
│  ├── MVP 0-5 Complete                ├── CLAUDE.md updates                     │
│  ├── MVP 4: NovaNet MCP v0.14.0      ├── README documentation                  │
│  ├── denomination_forms (ADR-033)    ├── v0.3 showcase examples                │
│  ├── context_build_log (ADR-035)     ├── INDEX.md cleanup                      │
│  ├── rig-core 0.31 in Cargo.toml     └── Duplicate file removal                │
│  ├── RigProvider wrapper (761 lines) │                                         │
│  ├── for_each parallelism            │                                         │
│  └── 602 tests passing               │                                         │
│                                                                                │
│  MVP 6: 98% → Other session finishing documentation                            │
│                                                                                │
└────────────────────────────────────────────────────────────────────────────────┘
```

---

## Phase 1: Tag v0.3.0 Release (When Other Session Done)

**Trigger:** Other session completes all 12 items
**Effort:** 15 minutes

### Steps

1. **Verify all tests pass**
   ```bash
   cd nika-dev/tools/nika && cargo test
   cd novanet-dev && pnpm test
   ```

2. **Create git tag**
   ```bash
   git tag -a v0.3.0 -m "Nika v0.3.0: for_each parallelism, rig-core ready, resilience patterns"
   git push origin v0.3.0
   ```

3. **Update VERSION files**
   ```bash
   echo "0.3.0" > nika-dev/VERSION
   echo "0.14.0" > novanet-dev/VERSION
   ```

### Success Criteria
- [ ] All tests pass (NovaNet + Nika)
- [ ] v0.3.0 tag created
- [ ] VERSION files updated

---

## Phase 2: MVP 7 - Rig-Core Migration (~8-12 hours)

**Priority:** HIGH - Major code simplification
**Effort:** 8-12 hours over 2-3 days

### Overview

Replace custom LLM providers with rig-core's native implementation:
- Delete ~1,420 lines of custom code
- Simplify AgentLoop from 717 → ~200 lines
- Native MCP integration via `.rmcp_tools()`

### Task 2.1: Rewrite AgentLoop (4 hours)

**File:** `nika-dev/tools/nika/src/runtime/agent_loop.rs`

**Before (717 lines):**
```rust
pub struct AgentLoop {
    provider: Arc<dyn Provider>,
    mcp_client: Arc<McpClient>,
    // ... custom tool calling logic
}
```

**After (~200 lines):**
```rust
use rig::agent::AgentBuilder;
use rig::providers::anthropic;

pub async fn run_agent(
    prompt: &str,
    mcp_tools: Vec<NikaMcpTool>,
    model: &str,
) -> Result<String> {
    let client = anthropic::Client::from_env();
    let agent = client
        .agent(model)
        .preamble("You are a helpful assistant...")
        .rmcp_tools(mcp_tools)  // Native MCP integration
        .build();

    agent.prompt(prompt).await
}
```

**Sub-tasks:**
1. [ ] Create new `agent_loop_rig.rs` with rig implementation
2. [ ] Write tests for new implementation
3. [ ] Compare behavior with existing implementation
4. [ ] Replace old implementation
5. [ ] Remove `agent_loop.rs` (keep as `_agent_loop_legacy.rs` temporarily)

### Task 2.2: Delete Old Providers (1 hour)

**Files to delete:**
| File | Lines | Reason |
|------|-------|--------|
| `provider/claude.rs` | ~325 | Replaced by `rig::providers::anthropic` |
| `provider/openai.rs` | ~280 | Replaced by `rig::providers::openai` |
| `provider/types.rs` | ~765 | Replaced by `rig::completion::*` types |

**Steps:**
1. [ ] Verify no imports from deleted files
2. [ ] Delete files
3. [ ] Update `provider/mod.rs` exports
4. [ ] Run tests to confirm nothing breaks

### Task 2.3: Update Tests (2 hours)

**Files to update:**
- `tests/agent_loop_test.rs`
- `tests/agent_edge_cases_test.rs`
- `tests/claude_chat_test.rs`
- `tests/rig_integration_test.rs`

**Steps:**
1. [ ] Replace mock providers with rig mocks
2. [ ] Update test assertions for new response format
3. [ ] Add new tests for rig-specific features
4. [ ] Run full test suite

### Task 2.4: Validate MCP Integration (2 hours)

**Goal:** Ensure `.rmcp_tools()` works with NovaNet MCP

**Test workflow:**
```yaml
workflow: test-rig-mcp
mcp:
  servers:
    - name: novanet
      command: ./novanet-mcp
      args: []
tasks:
  - id: test_agent
    agent:
      prompt: "Generate content for entity qr-code in fr-FR"
      tools: [novanet_generate, novanet_describe]
```

**Steps:**
1. [ ] Start NovaNet MCP server
2. [ ] Run test workflow
3. [ ] Verify denomination_forms in response
4. [ ] Verify context_build_log in response
5. [ ] Compare with pre-migration behavior

### Task 2.5: Performance Benchmarks (1 hour)

**Commands:**
```bash
cd nika-dev/tools/nika
cargo bench --bench resilience
```

**Metrics to track:**
- Agent turn latency (before/after)
- Tool call overhead
- Memory usage
- Token throughput

**Success criteria:**
- No regression > 10%
- Memory usage stable or improved

---

## Phase 3: Polish & Release v0.3.1 (2 hours)

### Task 3.1: Update Documentation

- [ ] Update CLAUDE.md with rig-core architecture
- [ ] Update README with new provider section
- [ ] Add migration guide for custom provider users

### Task 3.2: Create Release

```bash
git tag -a v0.3.1 -m "Nika v0.3.1: rig-core migration, -1420 lines"
git push origin v0.3.1
```

### Task 3.3: Update ROADMAP

- [ ] Mark MVP 7 as ✅ Done
- [ ] Add MVP 8 planning if needed

---

## Phase 4: Future Enhancements (Backlog)

### P1 - High Priority (Next Sprint)

| Item | Description | Effort |
|------|-------------|--------|
| TUI → Runner | Connect TUI to real workflow execution | 2h |
| CI Integration Tests | Setup Neo4j in CI for real MCP tests | 4h |
| Streaming Support | Add rig streaming for long responses | 2h |

### P2 - Medium Priority

| Item | Description | Effort |
|------|-------------|--------|
| Vector Search | Add embeddings to NovaNet entities | 8h |
| GraphRAG | Two-stage retrieval (semantic + graph) | 12h |
| Multi-Agent | Multiple agents collaborating | 8h |

### P3 - Nice to Have

| Item | Description | Effort |
|------|-------------|--------|
| Workspace Split | nika-core, nika-mcp, nika-tui crates | 4h |
| WebSocket Transport | Alternative to stdio for MCP | 4h |
| Plugin System | External tool providers | 8h |

---

## Execution Order

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  RECOMMENDED EXECUTION ORDER                                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  TODAY (after other session)                                                │
│  └── Phase 1: Tag v0.3.0 (15 min)                                          │
│                                                                             │
│  THIS WEEK                                                                  │
│  ├── Task 2.1: Rewrite AgentLoop (4h)                                      │
│  ├── Task 2.2: Delete old providers (1h)                                   │
│  └── Task 2.3: Update tests (2h)                                           │
│                                                                             │
│  NEXT WEEK                                                                  │
│  ├── Task 2.4: Validate MCP integration (2h)                               │
│  ├── Task 2.5: Performance benchmarks (1h)                                 │
│  └── Phase 3: Release v0.3.1 (2h)                                          │
│                                                                             │
│  FUTURE SPRINTS                                                            │
│  └── Phase 4: TUI→Runner, CI, Streaming, GraphRAG...                       │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Key Files Reference

### Nika (to modify)
| File | Action | Priority |
|------|--------|----------|
| `runtime/agent_loop.rs` | Rewrite with rig | P0 |
| `provider/claude.rs` | Delete | P0 |
| `provider/openai.rs` | Delete | P0 |
| `provider/types.rs` | Delete | P0 |
| `provider/mod.rs` | Simplify | P0 |
| `provider/rig.rs` | Keep (already done) | - |

### NovaNet (no changes needed)
| File | Status |
|------|--------|
| `tools/generate.rs` | ✅ denomination_forms + context_build_log |
| All 7 MCP tools | ✅ Complete |

### Tests
| File | Action |
|------|--------|
| `tests/agent_loop_test.rs` | Update for rig |
| `tests/rig_integration_test.rs` | Enable (remove #[ignore]) |
| `benches/resilience.rs` | Add rig benchmarks |

---

## Success Metrics

### MVP 7 Complete When:
- [ ] AgentLoop uses rig::AgentBuilder
- [ ] ~1,420 lines deleted
- [ ] All 602+ tests pass
- [ ] Benchmarks show no regression
- [ ] v0.3.1 tag created

### Overall Success:
- [ ] Nika v0.3.1 released with native rig-core
- [ ] Code is simpler (fewer custom implementations)
- [ ] MCP integration is more robust
- [ ] Future LLM provider additions are trivial
