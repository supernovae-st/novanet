# DX Audit 100% Completion Plan

**Date**: 2026-02-19
**Goal**: Complete remaining 4 DX items (8/12 → 12/12)

---

## Status Update from Exploration

| Item | Original Status | Exploration Finding |
|------|----------------|---------------------|
| DX-9 | Pending | Needs GH Actions workflow |
| DX-10 | Pending | **FALSE ALARM** — Mock is NOT default! |
| DX-11 | Pending | Add `ContextBuildLog` to generate.rs |
| DX-12 | Pending | Create 2-3 quickstart examples |

**DX-10 Correction**: The exploration found that mock mode is NOT the default in Nika. The `McpClient` requires explicit MCP configuration in workflow YAML. If no config exists, it returns `McpNotConfigured` error — NOT a fallback to mock.

---

## Execution Plan

### Task 1: DX-10 — Mark as DONE (was false alarm)

No code changes needed. Mock mode was never the default.

**Evidence** (from exploration):
```rust
// executor.rs:505-509
// If NO configuration exists, returns error McpNotConfigured
// Does NOT fall back to mock
```

### Task 2: DX-12 — Create v0.3 Quickstart Examples

Create 2 simple examples in `nika-dev/examples/`:

**File 1: `quickstart-mcp.nika.yaml`**
```yaml
# Minimal NovaNet MCP integration (v0.3)
# Demonstrates: invoke verb + MCP tool
workflow:
  name: quickstart-mcp
  version: "0.3"

mcp:
  novanet:
    command: cargo
    args: ["run", "--manifest-path", "../novanet-dev/tools/novanet-mcp/Cargo.toml"]

tasks:
  - id: query_stats
    invoke: novanet_stats
    use.ctx: stats

  - id: report
    infer: |
      Summarize these graph statistics in one sentence:
      {{ stats }}
```

**File 2: `quickstart-multilang.nika.yaml`**
```yaml
# Multi-locale generation with for_each (v0.3)
# Demonstrates: for_each + invoke + parallel MCP
workflow:
  name: quickstart-multilang
  version: "0.3"

mcp:
  novanet:
    command: cargo
    args: ["run", "--manifest-path", "../novanet-dev/tools/novanet-mcp/Cargo.toml"]

tasks:
  - id: generate_per_locale
    for_each:
      items: ["en-US", "fr-FR", "es-MX"]
      as: locale
    invoke: novanet_generate
    params:
      focus_key: "qr-code"
      locale: "{{ locale }}"
      mode: block
    use.ctx: context_{{ locale }}
```

### Task 3: DX-11 — Add context_build_log to MCP Generate

**File**: `tools/novanet-mcp/src/tools/generate.rs`

**Changes**:
1. Add `ContextBuildLog` struct after line ~104
2. Add `context_build_log` field to `GenerateResult`
3. Populate log entries during phases 1-7 in `execute()`

### Task 4: DX-9 — Create Integration Test Workflow

**File**: `.github/workflows/integration.yml`

```yaml
name: Integration Tests
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  integration:
    runs-on: ubuntu-latest
    services:
      neo4j:
        image: neo4j:5.26-community
        ports:
          - 7474:7474
          - 7687:7687
        env:
          NEO4J_AUTH: neo4j/testpassword

    steps:
      - uses: actions/checkout@v4
        with:
          submodules: true

      - name: Setup Rust
        uses: dtolnay/rust-action@stable

      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: 20

      - name: Install pnpm
        uses: pnpm/action-setup@v2
        with:
          version: 9

      - name: Seed NovaNet
        run: |
          cd novanet-dev
          pnpm install
          pnpm infra:seed

      - name: Run Nika Integration Test
        run: |
          cd nika-dev
          cargo run -- run examples/quickstart-mcp.nika.yaml
```

---

## Execution Order

1. ✅ DX-10: Mark done (no code change)
2. 🔨 DX-12: Create quickstart examples
3. 🔨 DX-11: Add context_build_log
4. 🔨 DX-9: Create integration workflow

---

## Progress Tracking

- [ ] DX-10: Verified as false alarm
- [ ] DX-12: quickstart-mcp.nika.yaml created
- [ ] DX-12: quickstart-multilang.nika.yaml created
- [ ] DX-11: ContextBuildLog struct added
- [ ] DX-11: context_build_log field in GenerateResult
- [ ] DX-11: Phase logging implemented
- [ ] DX-9: integration.yml created
- [ ] All tests pass
- [ ] Commit changes
