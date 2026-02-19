# DX Audit Fixes - Implementation Plan

**Date:** 2026-02-19
**Status:** Ready for Execution
**Source:** 14 Sniper Agent Audit Results (v2)

---

## Executive Summary

L'audit complet par 14 agents explorateurs parallèles a identifié **12 issues** à corriger:
- **5 critiques** (tests failing, MCP gaps, CI)
- **5 medium** (documentation desync)
- **2 low** (typo, examples)

**Temps estimé total:** ~45-60 minutes

---

## Issues Consolidées

### 🔴 CRITIQUES (P0)

| # | Issue | Location | Impact |
|---|-------|----------|--------|
| C1 | 3 TypeScript tests fail (arc family 5→6) | `novanet-dev/packages/core/` | CI broken |
| C2 | 1 Rust test fail (missing file) | `novanet-dev/tools/novanet/` | CI broken |
| C3 | denomination_forms NOT in MCP response | `novanet-dev/tools/novanet-mcp/` | ADR-033 violation |
| C4 | context_build_log NOT implemented | `novanet-dev/tools/novanet-mcp/` | No debug trace |
| C5 | McpClient mock mode default | `nika-dev/tools/nika/src/mcp/client.rs` | Integration blocked |

### 🟡 MEDIUM (P1)

| # | Issue | Location | Impact |
|---|-------|----------|--------|
| M1 | Nika CLAUDE.md missing resilience patterns | `nika-dev/tools/nika/CLAUDE.md` | DX incomplete |
| M2 | Nika CLAUDE.md missing for_each docs | `nika-dev/tools/nika/CLAUDE.md` | DX incomplete |
| M3 | NovaNet CLAUDE.md missing MCP tools | `novanet-dev/CLAUDE.md` | DX incomplete |
| M4 | supernovae-agi README wrong clone URL | `supernovae-agi/README.md` | Onboarding broken |
| M5 | nika-dev README version badge mismatch | `nika-dev/README.md` | Confusing |

### 🟢 LOW (P2)

| # | Issue | Location | Impact |
|---|-------|----------|--------|
| L1 | ROADMAP.md "62 nodes" typo (should be 61) | `supernovae-agi/ROADMAP.md` | Minor confusion |
| L2 | v0.3 showcase examples missing | `nika-dev/tools/nika/examples/` | Onboarding incomplete |

---

## Tasks (Bite-Sized, 2-5 min each)

### Phase 1: Fix Failing Tests (15-20 min)

#### Task 1.1: Fix TypeScript arc family test (5 min)
```
File: novanet-dev/packages/core/tests/design-system-coherence.test.ts
Action: Update expected arc family count from 5 to 6 (schema family added v0.13.1)
Verify: pnpm test --filter=@novanet/core
```

#### Task 1.2: Find and fix Rust test failure (5-10 min)
```
File: novanet-dev/tools/novanet/tests/schema_generate_dry_run_integration.rs
Action: Check for missing file reference, fix path or create fixture
Verify: cargo test schema_generate_dry_run
```

#### Task 1.3: Run full test suite (5 min)
```
Commands:
  cd novanet-dev && pnpm test
  cd novanet-dev/tools/novanet && cargo test
Verify: All green
```

### Phase 2: Update Documentation (20-25 min)

#### Task 2.1: Add resilience patterns to Nika CLAUDE.md (5 min)
```
File: nika-dev/tools/nika/CLAUDE.md
Add section: Resilience Patterns
Content:
  - retry: Automatic retry with backoff
  - circuit_breaker: Fail-fast on repeated errors
  - rate_limiter: Provider API throttling
Verify: grep "resilience" CLAUDE.md
```

#### Task 2.2: Add for_each to Nika CLAUDE.md (3 min)
```
File: nika-dev/tools/nika/CLAUDE.md
Add to Verbs section: for_each parallelism
Verify: grep "for_each" CLAUDE.md
```

#### Task 2.3: Add MCP tools to NovaNet CLAUDE.md (5 min)
```
File: novanet-dev/CLAUDE.md
Add section: MCP Server Tools (7 tools)
  - novanet_generate
  - novanet_describe
  - novanet_traverse
  - novanet_search
  - novanet_entity_list
  - novanet_locale_list
  - novanet_stats
Verify: grep "novanet_generate" CLAUDE.md
```

#### Task 2.4: Fix supernovae-agi README clone URL (2 min)
```
File: supernovae-agi/README.md
Find: Wrong GitHub clone URL
Replace: Correct supernovae-agi URL
Verify: grep "git clone" README.md
```

#### Task 2.5: Fix nika-dev README version badge (2 min)
```
File: nika-dev/README.md
Find: Version badge showing 0.3.0
Replace: Correct version v0.2.0
Verify: grep "badge" README.md
```

#### Task 2.6: Fix ROADMAP.md node count typo (2 min)
```
File: supernovae-agi/ROADMAP.md
Find: "62 nodes"
Replace: "61 nodes"
Verify: grep "61 nodes" ROADMAP.md
```

### Phase 3: MCP denomination_forms Gap (10-15 min)

#### Task 3.1: Verify ADR-033 requirement (2 min)
```
File: .claude/rules/adr/schema-architecture/adr-033-denomination-forms.md
Confirm: denomination_forms MUST be returned by novanet_generate
```

#### Task 3.2: Locate novanet_generate implementation (3 min)
```
File: novanet-dev/tools/novanet-mcp/src/tools/generate.rs
Action: Find EntityNative response construction
```

#### Task 3.3: Add denomination_forms to response (5-8 min)
```
File: novanet-dev/tools/novanet-mcp/src/tools/generate.rs
Action: Include denomination_forms from EntityNative in response
Verify: cargo test generate
```

#### Task 3.4: Update MCP schema if needed (2 min)
```
File: novanet-dev/tools/novanet-mcp/schema.json
Action: Add denomination_forms to response schema
```

#### Task 3.5: Add context_build_log to MCP response (5-10 min)
```
File: novanet-dev/tools/novanet-mcp/src/tools/generate.rs
Action: Add context_build_log field tracking assembly steps
Content:
  - graph_expansion_steps: ["Entity → HAS_NATIVE", "EntityNative → FOR_LOCALE", ...]
  - token_budget_allocation: { entity: 5000, locale: 3000, seo: 2000 }
  - pruning_decisions: ["Skipped 3 low-priority terms"]
Verify: cargo test generate
```

#### Task 3.6: Remove McpClient mock mode default (3 min)
```
File: nika-dev/tools/nika/src/mcp/client.rs
Find: mock: bool = true or similar default
Replace: mock: bool only in test cfg, production uses real MCP
Verify: cargo test mcp_client
```

### Phase 4: v0.3 Examples (15-20 min)

#### Task 4.1: Create for_each + invoke showcase (10 min)
```
File: nika-dev/tools/nika/examples/v03-parallel-generation.yaml
Content:
  - for_each over locales with concurrency: 3
  - invoke novanet_generate for each locale
  - Demonstrate context aggregation
Verify: cargo run -- validate examples/v03-parallel-generation.yaml
```

#### Task 4.2: Create agent + MCP showcase (10 min)
```
File: nika-dev/tools/nika/examples/v03-agent-refinement.yaml
Content:
  - agent: with tool calling to NovaNet
  - Multi-turn conversation for content refinement
Verify: cargo run -- validate examples/v03-agent-refinement.yaml
```

---

## Verification Checklist

After all tasks:

- [ ] `cd novanet-dev && pnpm test` → All pass
- [ ] `cd novanet-dev/tools/novanet && cargo test` → All pass
- [ ] `cd nika-dev/tools/nika && cargo test` → 602+ pass
- [ ] NovaNet CLAUDE.md has MCP tools section
- [ ] Nika CLAUDE.md has resilience + for_each
- [ ] READMEs have correct URLs/versions
- [ ] novanet_generate returns denomination_forms
- [ ] novanet_generate returns context_build_log
- [ ] McpClient uses real connection by default
- [ ] v0.3 example workflows validate successfully

---

## Execution Options

### Option A: Subagent-Driven Development (Recommended)
Lance un agent par phase, avec review entre chaque phase.

### Option B: Parallel Session
Execute toutes les tâches dans la session courante, en séquence.

---

## Dependencies

```
Phase 1 (Tests) → Can run immediately
Phase 2 (Docs)  → Can run immediately (parallel with Phase 1)
Phase 3 (MCP)   → Requires understanding of novanet-mcp codebase
```

---

## Notes

- Skills audit: 100% valid (21 skills) ✅
- Hooks audit: 100% working (20 hooks) ✅
- Nika tests: 602 pass, 0 fail ✅
- NovaNet Rust tests: 1191 pass (before fix)

---

## ADR Recommendations

Based on the 14-agent exploration, the following ADR actions are recommended:

### New ADRs Needed

| ADR | Title | Reason |
|-----|-------|--------|
| ADR-034 | Context Build Log | Document structure for MCP `context_build_log` field |
| ADR-035 | Nika-NovaNet MCP Contract | Formalize the 7 tools, their params, and response schemas |
| ADR-036 | Resilience Patterns | Document retry/circuit-breaker/rate-limiter standard |

### ADR Updates Needed

| ADR | Change | Reason |
|-----|--------|--------|
| ADR-033 | Add MCP enforcement | Specify denomination_forms MUST be in MCP response |
| ADR-029 | Add MCP field mapping | Document how *Native pattern appears in MCP tools |
| ADR-022 | Add Nika TUI section | Unified tree pattern for Nika TUI (planned) |

### ADR Quick-Reference Refresh

The `.claude/rules/adr-quick-reference.md` should be updated with:
1. v0.3 verb additions (for_each)
2. MCP tool inventory (7 tools)
3. Resilience pattern references
4. New ADR-034/035/036 cross-references

---

## Architecture Diagram (14-Agent Findings)

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│  SUPERNOVAE-AGI COMPLETE ARCHITECTURE (v0.13.1)                                     │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│  ┌───────────────────────────────────┐      MCP Protocol     ┌───────────────────┐ │
│  │          NOVANET (Brain)          │◄─────────────────────►│   NIKA (Body)     │ │
│  │                                   │     JSON-RPC 2.0      │                   │ │
│  ├───────────────────────────────────┤     stdio transport   ├───────────────────┤ │
│  │                                   │                       │                   │ │
│  │  ┌─────────────┐ ┌─────────────┐  │    7 TOOLS:           │  ┌─────────────┐  │ │
│  │  │ Neo4j 5.26  │ │ Rust CLI    │  │  - novanet_query      │  │ 5 VERBS:    │  │ │
│  │  │ 22,189 nodes│ │ 1,139 tests │  │  - novanet_describe   │  │ • infer:    │  │ │
│  │  │ 182 arcs    │ │ 49,460 TUI  │  │  - novanet_search     │  │ • exec:     │  │ │
│  │  └─────────────┘ └─────────────┘  │  - novanet_traverse   │  │ • fetch:    │  │ │
│  │                                   │  - novanet_assemble   │  │ • invoke:   │  │ │
│  │  ┌─────────────┐ ┌─────────────┐  │  - novanet_atoms      │  │ • agent:    │  │ │
│  │  │ MCP Server  │ │ Studio UI   │  │  - novanet_generate   │  └─────────────┘  │ │
│  │  │ rmcp 0.15   │ │ Next.js 16  │  │                       │                   │ │
│  │  │ 7 tools     │ │ React 19    │  │  4 RESOURCES:         │  ┌─────────────┐  │ │
│  │  │ 4 resources │ │ TypeScript  │  │  entity://{key}       │  │ RESILIENCE: │  │ │
│  │  │ 6 prompts   │ │ 5.9         │  │  class://{name}       │  │ • retry     │  │ │
│  │  └─────────────┘ └─────────────┘  │  locale://{code}      │  │ • circuit   │  │ │
│  │                                   │  view://{id}          │  │ • rate_limit│  │ │
│  └───────────────────────────────────┘                       │  └─────────────┘  │ │
│           │                                                  │                   │ │
│           │                                                  │  ┌─────────────┐  │ │
│           ▼                                                  │  │ EVENTS:     │  │ │
│  ┌───────────────────────────────────┐                       │  │ 16 variants │  │ │
│  │        SCHEMA STRUCTURE           │                       │  │ NDJSON trace│  │ │
│  ├───────────────────────────────────┤                       │  └─────────────┘  │ │
│  │                                   │                       │                   │ │
│  │  2 REALMS:                        │                       └───────────────────┘ │
│  │  ├── shared (40 nodes, READ-ONLY) │                                            │
│  │  │   ├── config (3)               │                       ┌───────────────────┐ │
│  │  │   ├── locale (6)               │                       │ WORKFLOW EXAMPLE  │ │
│  │  │   ├── geography (7)            │                       ├───────────────────┤ │
│  │  │   └── knowledge (24)           │                       │                   │ │
│  │  │                                │                       │ tasks:            │ │
│  │  └── org (21 nodes)               │                       │   - invoke:       │ │
│  │      ├── config (1)               │                       │       novanet_    │ │
│  │      ├── foundation (6)           │       ─────────────►  │       generate    │ │
│  │      ├── structure (3)            │      denomination_    │     params:       │ │
│  │      ├── semantic (4)             │      forms +          │       entity: qr  │ │
│  │      ├── instruction (4)          │      context_build_   │       locale: fr  │ │
│  │      └── output (3)               │      log (MISSING!)   │                   │ │
│  │                                   │                       │   - infer:        │ │
│  │  5 TRAITS (Data Origin):          │                       │       "Generate   │ │
│  │  • defined (31) - Human, ONCE     │                       │       landing     │ │
│  │  • authored (2) - Human, PER loc  │                       │       page"       │ │
│  │  • imported (20) - External data  │                       │                   │ │
│  │  • generated (4) - Our LLM        │                       └───────────────────┘ │
│  │  • retrieved (2) - External APIs  │                                            │
│  │                                   │                                            │
│  │  6 ARC FAMILIES:                  │                                            │
│  │  ownership │ localization │       │                                            │
│  │  semantic  │ generation   │       │                                            │
│  │  mining    │ schema       │       │                                            │
│  │                                   │                                            │
│  └───────────────────────────────────┘                                            │
│                                                                                     │
├─────────────────────────────────────────────────────────────────────────────────────┤
│  KEY ADRs: 029 (*Native) | 030 (Slug) | 033 (Denomination) | 024 (Trait=Origin)   │
└─────────────────────────────────────────────────────────────────────────────────────┘
```

---

## Execution Priority

```
PHASE 1 ─────► PHASE 2 ─────► PHASE 3 ─────► PHASE 4
(Tests)        (Docs)         (MCP)           (Examples)
  │              │              │               │
  C1,C2          M1-M5          C3,C4,C5        L2
  L1                            Task 3.1-3.6    Task 4.1-4.2
  │              │              │               │
  └──────────────┴──────────────┴───────────────┘
         ▼
      v0.3 RELEASE READY
```
