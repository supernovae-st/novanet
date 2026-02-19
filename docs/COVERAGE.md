# Test Coverage Standards

**Last Updated:** 2026-02-19
**Applies to:** supernovae-agi monorepo (NovaNet + Nika)

---

## Overview

This document defines test coverage targets, measurement tools, and CI enforcement for the supernovae-agi monorepo.

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  COVERAGE PHILOSOPHY                                                            │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  1. Test BEHAVIOR, not implementation                                           │
│  2. High coverage in CRITICAL paths (stores, core libs, resilience)            │
│  3. Lower thresholds for UI/API routes (change frequently)                     │
│  4. TDD preferred — write failing tests first                                   │
│  5. Measure coverage, don't worship it                                          │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Coverage Targets

### NovaNet (TypeScript)

| Module | Branches | Functions | Lines | Statements |
|--------|----------|-----------|-------|------------|
| **Global minimum** | 60% | 60% | 70% | 70% |
| `src/stores/**/*.ts` | 80% | 80% | 85% | 85% |
| `src/lib/**/*.ts` | 75% | 75% | 80% | 80% |

**Exclusions** (not counted toward coverage):
- `*.d.ts` — Type declarations
- `*.stories.{ts,tsx}` — Storybook stories
- `**/index.ts` — Re-exports
- `src/app/api/**/*` — API routes (tested via integration)

### NovaNet (Rust CLI)

| Module | Line Coverage | Branch Coverage |
|--------|---------------|-----------------|
| **Global minimum** | 70% | 60% |
| `src/core/**` | 85% | 75% |
| `src/tui/**` | 60% | 50% |
| `src/commands/**` | 70% | 60% |

### Nika (Rust)

| Module | Line Coverage | Branch Coverage |
|--------|---------------|-----------------|
| **Global minimum** | 70% | 60% |
| `src/resilience/**` | 90% | 85% |
| `src/ast/**` | 85% | 80% |
| `src/runtime/**` | 80% | 75% |
| `src/mcp/**` | 75% | 70% |
| `src/tui/**` | 60% | 50% |

---

## Measurement Tools

### TypeScript (Jest)

```bash
# Run tests with coverage
cd novanet-dev/apps/studio
npm run test:coverage

# Coverage report locations
# - Terminal: summary
# - HTML: coverage/lcov-report/index.html
# - LCOV: coverage/lcov.info (for CI)
```

**Configuration:** `jest.config.cjs`
```javascript
coverageThreshold: {
  global: { branches: 60, functions: 60, lines: 70, statements: 70 },
  'src/stores/**/*.ts': { branches: 80, functions: 80, lines: 85, statements: 85 },
  'src/lib/**/*.ts': { branches: 75, functions: 75, lines: 80, statements: 80 },
}
```

### Rust (cargo-llvm-cov + nextest)

```bash
# Install tools
cargo install cargo-llvm-cov cargo-nextest

# Run with coverage (NovaNet)
cd novanet-dev/tools/novanet
cargo llvm-cov nextest --profile coverage

# Run with coverage (Nika)
cd nika-dev/tools/nika
cargo llvm-cov nextest --profile coverage

# Generate HTML report
cargo llvm-cov nextest --html

# Generate LCOV for CI
cargo llvm-cov nextest --lcov --output-path lcov.info
```

**Configuration:** `.config/nextest.toml`
```toml
[profile.coverage]
fail-fast = false
slow-timeout = { period = "120s", terminate-after = 2 }
```

---

## CI Enforcement

### Pre-commit (local)

The `.husky/pre-commit` hook runs:
- `cargo fmt --check` — Format validation
- `cargo clippy -- -D warnings` — Lint warnings as errors
- `cargo deny check` — Security/license audit (on Cargo.toml changes)
- `pnpm lint` — TypeScript linting

### GitHub Actions

| Workflow | Coverage Check | Artifacts |
|----------|---------------|-----------|
| `ci.yml` | Jest + llvm-cov | lcov.info |
| `rust-ci.yml` | llvm-cov | coverage report |
| `integration.yml` | None (E2E focus) | — |

**Coverage upload** (Codecov):
```yaml
- name: Upload coverage
  uses: codecov/codecov-action@v4
  with:
    files: ./coverage/lcov.info,./target/llvm-cov/lcov.info
    fail_ci_if_error: true
```

---

## Test Profiles (nextest)

Both NovaNet and Nika use identical nextest profiles:

| Profile | Purpose | Options |
|---------|---------|---------|
| `default` | Local development | `fail-fast = true` |
| `ci` | GitHub Actions | `fail-fast = false, retries = 2, junit` |
| `coverage` | Coverage collection | `fail-fast = false, slow-timeout = 120s` |
| `integration` | Real service tests | `test-threads = 1, retries = 3` |
| `mutants` | Mutation testing | `test-threads = 1` |

```bash
# Run specific profile
cargo nextest run --profile ci
cargo nextest run --profile integration
```

---

## Test Categories

### Unit Tests

**Location:** Same file in `#[cfg(test)]` module
**Naming:** `test_<function>_<scenario>_<expected_outcome>()`
**Coverage target:** 85%+ for core modules

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_workflow_valid_yaml_returns_workflow() {
        // arrange
        let yaml = "schema: nika/workflow@0.2\nworkflow: test";
        // act
        let result = parse_workflow(yaml);
        // assert
        assert!(result.is_ok());
    }
}
```

### Integration Tests

**Location:** `tests/` directory
**Naming:** `<feature>_integration_test.rs`
**Coverage target:** 70%+ (behavior focus)

```rust
// tests/mcp_integration_test.rs
#[tokio::test]
async fn test_invoke_novanet_describe_returns_entity_data() {
    // Requires real NovaNet MCP server
}
```

### Snapshot Tests (insta)

**Location:** Alongside unit tests
**Artifacts:** `tests/snapshots/*.snap`
**Use cases:** Parser output, YAML generation, error messages

```rust
#[test]
fn test_workflow_serialization_snapshot() {
    let workflow = create_test_workflow();
    insta::assert_yaml_snapshot!(workflow);
}
```

### Property Tests (proptest)

**Location:** Alongside unit tests
**Use cases:** Parser fuzzing, edge cases
**Coverage target:** Critical parsers only

```rust
proptest! {
    #[test]
    fn test_binding_parse_never_panics(s in ".*") {
        let _ = Binding::parse(&s);
    }
}
```

---

## Mutation Testing

**Tool:** `cargo-mutants`
**Purpose:** Verify test quality (tests catch real bugs)
**Target:** 80%+ mutation score for core modules

```bash
# Install
cargo install cargo-mutants

# Run mutation testing
cd nika-dev/tools/nika
cargo mutants --profile mutants

# Run on specific module
cargo mutants -p nika -- src/resilience/
```

**Interpretation:**
- Mutation killed = test caught the bug = ✅
- Mutation survived = test didn't catch = ⚠️ improve tests
- 80%+ mutation score = tests are trustworthy

---

## Coverage Improvement Workflow

1. **Identify gaps:**
   ```bash
   cargo llvm-cov nextest --html
   open target/llvm-cov/html/index.html
   ```

2. **Focus on red zones:**
   - Uncovered branches
   - Error handling paths
   - Edge cases

3. **Write failing test first (TDD):**
   ```rust
   #[test]
   fn test_circuit_breaker_opens_after_threshold() {
       // This test should fail initially
   }
   ```

4. **Implement minimal code to pass**

5. **Verify coverage improved:**
   ```bash
   cargo llvm-cov nextest --profile coverage
   ```

---

## Exceptions

### When to accept lower coverage

| Situation | Minimum | Rationale |
|-----------|---------|-----------|
| TUI components | 50% | Visual testing needed |
| API routes | 40% | Integration tests cover |
| Generated code | 0% | Auto-generated, tested upstream |
| Deprecated code | 0% | Scheduled for removal |

### When to require higher coverage

| Situation | Minimum | Rationale |
|-----------|---------|-----------|
| Resilience patterns | 90% | Critical for production |
| Error handling | 85% | Must not fail silently |
| Parser/serializer | 85% | Edge cases matter |
| Security-sensitive | 90% | Cannot have gaps |

---

## Related Documentation

- **TDD Workflow:** See `test-driven-development` skill
- **CI Pipelines:** `.github/workflows/`
- **nextest Config:** `.config/nextest.toml`
- **Jest Config:** `apps/studio/jest.config.cjs`
- **Pre-commit:** `.husky/pre-commit`
