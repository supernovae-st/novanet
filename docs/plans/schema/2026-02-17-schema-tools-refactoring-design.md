# Schema Tools Refactoring: Auto-Fix, Hooks & Modern Rust Ecosystem

**Date**: 2026-02-17
**Status**: Proposed (TDD Implementation)
**Version**: v0.13.1 → v0.14.0+
**Context**: Refactoring 4 completed v0.13.1 features into production-grade, extensible tools

## Problem Statement

Four features were implemented in v0.13.1 as basic solutions:

1. **Duplicate seed file numbering resolution** - Manual detection, no auto-renumbering
2. **Composite key format linter** - Validates but provides no fix suggestions
3. **Arc count extraction from YAML** - Basic counting, no quality metrics
4. **CI drift detection** - Fails build but offers no actionable guidance

### Current Pain Points

**For Users**:
- Manual fixes required for all validation violations
- No actionable suggestions or context in error messages
- Sequential validation (slow on large schemas)
- No quality metrics or trend visualization

**For Maintainers**:
- Hard to add new validation rules (no plugin system)
- No automated version management (currently at v0.13.1, need intelligent semver)
- Limited test coverage approaches (missing property-based, mutation testing)
- No dependency audit automation

**For the Project**:
- Not "LLM vibe coding friendly" - lacks evolutionary architecture
- Manual version updates instead of automated intelligent workflow
- No hooks for extensibility
- Basic reporting (lacks rich context)

## Goals

### A. Robustness
- Comprehensive validation with edge case coverage
- Property-based testing for invariants
- Mutation testing to verify test quality
- Error handling with actionable context

### B. Utility
- **Auto-fix engine**: Automatically correct violations
- **Rich suggestions**: Context-aware fix recommendations
- **Quality metrics**: Coverage, compliance, drift trends
- **Multiple report formats**: Terminal (ASCII), JSON, HTML

### C. Maintainability
- **Plugin architecture**: Easy to add new rules
- **Hooks system**: Pre/post validation extensibility
- **Clear separation of concerns**: Validation / AutoFix / Reporting
- **Comprehensive documentation**: ADRs, examples, tests

### D. Performance
- **Parallel validation**: Rayon for 4x speedup (proven by cargo-semver-checks)
- **Incremental validation**: Only check changed nodes
- **Fast test suite**: cargo-nextest for 3x faster CI

### E. Evolutionary Architecture
- **Automated versioning**: cargo-semver-checks + cargo-release workflow
- **Hook system**: Custom extensions without modifying core
- **Modern Rust tooling**: Best practices from ecosystem research
- **LLM-friendly DX**: Clear patterns, good documentation

## Current Architecture (v0.13.1)

### File Structure

```
tools/novanet/src/
├── parsers/
│   └── schema_rules.rs        # 428 lines, monolithic validation
├── commands/
│   └── schema.rs              # schema_stats (lines 297-384)
└── .github/workflows/
    └── rust-ci.yml            # schema-drift job (lines 447-515)
```

### schema_rules.rs Issues

```rust
// Current implementation: monolithic function
pub fn validate_node(node: &ParsedNode) -> Vec<SchemaIssue> {
    let mut issues = Vec::new();

    // Rule 1: KEY_REQUIRED (20 lines)
    // Rule 2: DENORM_REQUIRED (20 lines)
    // Rule 3: TIMESTAMP_REQUIRED (15 lines)
    // Rule 4: PROP_ORDER (20 lines)
    // Rule 5: COMPOSITE_KEY_FORMAT (145 lines!)

    issues
}
```

**Problems**:
- All rules in one function (428 lines total)
- No auto-fix capability
- No plugin system
- Sequential execution (no parallelization)
- Rule 5 is 145 lines alone (composite key validation)

## Proposed Architecture

### 1. Global Architecture

```
tools/novanet/src/
├── validation/                 # NEW: Validation engine
│   ├── mod.rs                 # ValidationRule trait, Registry
│   ├── rules/                 # Individual validation rules
│   │   ├── composite_key.rs   # Rule 5 (145 lines → isolated)
│   │   ├── standard_props.rs  # Rules 1-3 (key, denorm, timestamps)
│   │   ├── property_order.rs  # Rule 4
│   │   └── mod.rs             # Rule registration
│   ├── autofix/               # Auto-fix engine
│   │   ├── mod.rs             # AutoFix trait
│   │   ├── composite_key.rs   # Fix composite key format
│   │   ├── property_order.rs  # Reorder standard properties
│   │   ├── timestamps.rs      # Add missing timestamps
│   │   └── strategies.rs      # Safe/Auto/DryRun modes
│   └── hooks.rs               # Hook system (pre/post validation)
│
├── quality/                    # NEW: Quality metrics & reporting
│   ├── metrics.rs             # QualityMetric trait + implementations
│   └── reports.rs             # Terminal/JSON/HTML formatters
│
├── parsers/
│   └── schema_rules.rs        # REFACTORED: use validation::rules
│
└── commands/
    └── schema.rs              # ENHANCED: use quality::metrics
```

### 2. Auto-Fix Engine

#### Traits

```rust
// validation/autofix/mod.rs
pub trait AutoFix: Send + Sync {
    /// Check if this fixer can handle the issue
    fn can_fix(&self, issue: &SchemaIssue) -> bool;

    /// Apply the fix
    fn fix(&self, node: &mut ParsedNode, issue: &SchemaIssue)
        -> Result<FixAction>;

    /// Human-readable description
    fn description(&self) -> &str;
}

pub enum FixAction {
    Modified { changes: Vec<Change> },
    Skipped { reason: String },
}

pub struct Change {
    pub field: String,
    pub old_value: Option<serde_yaml::Value>,
    pub new_value: serde_yaml::Value,
}
```

#### Fix Strategies

```rust
pub enum FixStrategy {
    Safe,      // Only fixes with 100% confidence
    Auto,      // Aggressive auto-fix
    DryRun,    // Preview only, no writes
}
```

#### Concrete Fixers

**CompositeKeyFixer** (`validation/autofix/composite_key.rs`):
```rust
impl AutoFix for CompositeKeyFixer {
    fn can_fix(&self, issue: &SchemaIssue) -> bool {
        issue.rule == "COMPOSITE_KEY_FORMAT"
    }

    fn fix(&self, node: &mut ParsedNode, issue: &SchemaIssue)
        -> Result<FixAction>
    {
        // Parse issue message to extract expected format
        // Update key property pattern
        // Regenerate examples with correct format
        Ok(FixAction::Modified { changes })
    }
}
```

**PropertyOrderFixer** (`validation/autofix/property_order.rs`):
```rust
impl AutoFix for PropertyOrderFixer {
    fn fix(&self, node: &mut ParsedNode, issue: &SchemaIssue)
        -> Result<FixAction>
    {
        // Reorder standard_properties using STANDARD_PROPS_ORDER
        // Preserve all properties, just change order
        Ok(FixAction::Modified { changes })
    }
}
```

**TimestampFixer** (`validation/autofix/timestamps.rs`):
```rust
impl AutoFix for TimestampFixer {
    fn fix(&self, node: &mut ParsedNode, issue: &SchemaIssue)
        -> Result<FixAction>
    {
        // Add missing created_at/updated_at with standard schema
        Ok(FixAction::Modified { changes })
    }
}
```

### 3. Hooks System

```rust
// validation/hooks.rs
pub trait Hook: Send + Sync {
    fn name(&self) -> &str;

    /// Called before validation starts
    fn on_validate_start(&self, ctx: &ValidationContext);

    /// Called after validation completes
    fn on_validate_end(&self, ctx: &ValidationContext, issues: &[SchemaIssue]);

    /// Called when a fix is applied
    fn on_fix_applied(&self, ctx: &ValidationContext, fix: &FixAction);
}

pub struct ValidationContext {
    pub root: PathBuf,
    pub nodes_count: usize,
    pub strategy: FixStrategy,
}

pub struct HookRegistry {
    hooks: Vec<Box<dyn Hook>>,
}

impl HookRegistry {
    pub fn register(&mut self, hook: Box<dyn Hook>) {
        self.hooks.push(hook);
    }

    pub fn trigger_validate_start(&self, ctx: &ValidationContext) {
        for hook in &self.hooks {
            hook.on_validate_start(ctx);
        }
    }
}
```

#### Built-in Hooks

**ProgressHook** - Terminal progress bar:
```rust
struct ProgressHook {
    bar: ProgressBar,
}

impl Hook for ProgressHook {
    fn on_validate_start(&self, ctx: &ValidationContext) {
        self.bar.set_length(ctx.nodes_count as u64);
        self.bar.set_message("Validating schema...");
    }

    fn on_validate_end(&self, ctx: &ValidationContext, issues: &[SchemaIssue]) {
        self.bar.finish_with_message(
            format!("Found {} issues", issues.len())
        );
    }
}
```

**MetricsHook** - Collect quality metrics:
```rust
struct MetricsHook {
    start_time: Instant,
}

impl Hook for MetricsHook {
    fn on_validate_end(&self, ctx: &ValidationContext, issues: &[SchemaIssue]) {
        let duration = self.start_time.elapsed();
        println!("Validation completed in {:?}", duration);
        println!("Throughput: {:.2} nodes/sec",
                 ctx.nodes_count as f64 / duration.as_secs_f64());
    }
}
```

**GitHook** - Auto-commit fixes:
```rust
struct GitHook;

impl Hook for GitHook {
    fn on_fix_applied(&self, ctx: &ValidationContext, fix: &FixAction) {
        if let FixAction::Modified { changes } = fix {
            // git add modified files
            // git commit with descriptive message
        }
    }
}
```

**SlackHook** - CI notifications:
```rust
struct SlackHook {
    webhook_url: String,
}

impl Hook for SlackHook {
    fn on_validate_end(&self, ctx: &ValidationContext, issues: &[SchemaIssue]) {
        let errors = issues.iter()
            .filter(|i| i.severity == IssueSeverity::Error)
            .count();

        if errors > 0 {
            // POST to Slack webhook with summary
        }
    }
}
```

### 4. Quality Metrics & Reporting

#### Metrics Trait

```rust
// quality/metrics.rs
pub trait QualityMetric: Send + Sync {
    fn name(&self) -> &str;
    fn collect(&self, nodes: &[ParsedNode], issues: &[SchemaIssue])
        -> MetricValue;
}

pub enum MetricValue {
    Percentage(f64),
    Count(usize),
    Duration(Duration),
}
```

#### Concrete Metrics

**ValidationCoverage**:
```rust
struct ValidationCoverage;

impl QualityMetric for ValidationCoverage {
    fn collect(&self, nodes: &[ParsedNode], issues: &[SchemaIssue])
        -> MetricValue
    {
        let validated = nodes.len();
        let total = validated + issues.len();
        MetricValue::Percentage(validated as f64 / total as f64 * 100.0)
    }
}
```

**RuleCompliance**:
```rust
struct RuleCompliance {
    rule_name: String,
}

impl QualityMetric for RuleCompliance {
    fn collect(&self, nodes: &[ParsedNode], issues: &[SchemaIssue])
        -> MetricValue
    {
        let violations = issues.iter()
            .filter(|i| i.rule == self.rule_name)
            .count();

        let compliant = nodes.len() - violations;
        MetricValue::Percentage(compliant as f64 / nodes.len() as f64 * 100.0)
    }
}
```

**DriftTrend**:
```rust
struct DriftTrend {
    history_file: PathBuf,
}

impl QualityMetric for DriftTrend {
    fn collect(&self, nodes: &[ParsedNode], issues: &[SchemaIssue])
        -> MetricValue
    {
        // Load historical data from .novanet/metrics/drift-history.json
        // Compare current issue count with previous runs
        // Return trend: improving/stable/degrading
        MetricValue::Count(issues.len())
    }
}
```

#### Report Formats

**Terminal (Colored ASCII)**:
```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  SCHEMA VALIDATION REPORT                                                     ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  Status: ✅ PASS (0 errors, 3 warnings)                                       ║
║  Coverage: 100% (61/61 nodes validated)                                       ║
║  Duration: 234ms (260 nodes/sec)                                              ║
║                                                                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║  QUALITY METRICS                                                              ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  Validation Coverage         100.0% ████████████████████████████████          ║
║  KEY_REQUIRED Compliance      98.4% ███████████████████████████████░          ║
║  PROP_ORDER Compliance        95.1% ███████████████████████████░░░░          ║
║  COMPOSITE_KEY Compliance    100.0% ████████████████████████████████          ║
║                                                                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║  DRIFT TREND                                                                  ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  Last 7 days: ↓ Improving (-5 violations)                                    ║
║  │                                                                            ║
║  8 │     ●                                                                    ║
║  7 │   ●   ●                                                                  ║
║  6 │ ●       ●                                                                ║
║  5 │           ●                                                              ║
║  4 │             ●                                                            ║
║  3 │               ● ←                                                        ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

**JSON (Machine-Readable)**:
```json
{
  "status": "pass",
  "summary": {
    "errors": 0,
    "warnings": 3,
    "nodes_validated": 61,
    "duration_ms": 234
  },
  "metrics": {
    "validation_coverage": 100.0,
    "rule_compliance": {
      "KEY_REQUIRED": 98.4,
      "PROP_ORDER": 95.1,
      "COMPOSITE_KEY_FORMAT": 100.0
    }
  },
  "drift_trend": {
    "current": 3,
    "previous": 8,
    "change": -5,
    "direction": "improving"
  },
  "issues": [
    {
      "node": "EntityCategory",
      "severity": "warning",
      "rule": "PROP_ORDER",
      "message": "Standard properties out of order",
      "fix_suggestion": "Run: novanet schema validate --fix --safe"
    }
  ]
}
```

**HTML (Rich Dashboard)**:
```html
<!DOCTYPE html>
<html>
<head>
  <title>NovaNet Schema Validation Report</title>
  <style>
    /* Tailwind-like styles */
    /* Chart.js for trend visualization */
  </style>
</head>
<body>
  <div class="container">
    <h1>Schema Validation Report</h1>

    <!-- Status badge -->
    <div class="badge badge-success">✅ PASS</div>

    <!-- Metrics cards -->
    <div class="grid">
      <div class="card">
        <h3>Coverage</h3>
        <div class="metric">100%</div>
        <div class="progress-bar" style="width: 100%"></div>
      </div>
      <!-- More cards... -->
    </div>

    <!-- Drift trend chart -->
    <canvas id="drift-chart"></canvas>

    <!-- Issues table with fix suggestions -->
    <table><!-- ... --></table>
  </div>
</body>
</html>
```

### 5. TDD Implementation Plan

#### Sprint 1: Auto-Fix + Suggestions (2-3 days)

**Goal**: Users can automatically fix validation violations

**RED Phase**:
```rust
#[test]
fn test_composite_key_fixer_adds_missing_pattern() {
    let mut node = create_test_node("EntityNative");
    node.standard_properties.get_mut("key").unwrap()
        .extra.remove("pattern"); // Missing pattern

    let issue = SchemaIssue {
        rule: "COMPOSITE_KEY_FORMAT",
        message: "Missing pattern",
        // ...
    };

    let fixer = CompositeKeyFixer;
    let result = fixer.fix(&mut node, &issue).unwrap();

    assert!(matches!(result, FixAction::Modified { .. }));
    assert!(node.standard_properties["key"].extra.contains_key("pattern"));
}
```

**GREEN Phase**:
- Implement `AutoFix` trait
- Implement `CompositeKeyFixer`, `PropertyOrderFixer`, `TimestampFixer`
- Implement `FixStrategy` enum
- Add `--fix` flag to `schema validate` command

**REFACTOR Phase**:
- Extract common YAML manipulation helpers
- Add property-based tests with `proptest`
- Optimize YAML parsing/writing

**Acceptance Criteria**:
- ✅ 3 fixers implemented with 100% test coverage
- ✅ `novanet schema validate --fix` works
- ✅ Dry-run mode prevents accidental writes
- ✅ Fix suggestions in JSON/Terminal output

#### Sprint 2: Hooks + Plugin Architecture (3-4 days)

**Goal**: Easy to extend with custom rules and hooks

**RED Phase**:
```rust
#[test]
fn test_hook_registry_triggers_in_order() {
    let mut registry = HookRegistry::new();
    let mut call_order = Arc::new(Mutex::new(Vec::new()));

    registry.register(Box::new(TestHook::new(1, call_order.clone())));
    registry.register(Box::new(TestHook::new(2, call_order.clone())));

    registry.trigger_validate_start(&ctx);

    assert_eq!(*call_order.lock().unwrap(), vec![1, 2]);
}
```

**GREEN Phase**:
- Implement `Hook` trait
- Implement `HookRegistry`
- Implement built-in hooks: Progress, Metrics, Git
- Refactor `schema_rules.rs` to use `ValidationRule` trait
- Create individual rule files in `validation/rules/`

**REFACTOR Phase**:
- Plugin discovery via directory scanning
- Configuration file for hooks (`novanet.toml`)
- Error handling for hook failures

**Acceptance Criteria**:
- ✅ Hook system with 3+ built-in hooks
- ✅ ValidationRule trait with 5 rules refactored
- ✅ Plugin directory for custom rules
- ✅ Zero performance regression vs monolithic

#### Sprint 3: Performance + Advanced Features (2-3 days)

**Goal**: 4x faster validation, mutation testing, rich reporting

**RED Phase**:
```rust
#[test]
fn test_parallel_validation_faster_than_sequential() {
    let nodes = load_all_test_nodes(); // 61 nodes

    let start_seq = Instant::now();
    let _ = validate_all_nodes_sequential(&nodes);
    let dur_seq = start_seq.elapsed();

    let start_par = Instant::now();
    let _ = validate_all_nodes_parallel(&nodes);
    let dur_par = start_par.elapsed();

    assert!(dur_par < dur_seq / 2); // At least 2x faster
}
```

**GREEN Phase**:
- Implement parallel validation with `rayon`
- Add quality metrics (Coverage, Compliance, Drift)
- Implement Terminal/JSON/HTML report formatters
- Add mutation testing with `cargo-mutants`

**REFACTOR Phase**:
- Benchmark with `criterion`
- Profile with `cargo-flamegraph`
- Optimize hot paths

**Acceptance Criteria**:
- ✅ 4x faster validation via parallelization
- ✅ Quality metrics dashboard
- ✅ HTML report generation
- ✅ Mutation testing: 0 mutants survived

### 6. Ecosystem Integration & Modern Rust Tooling

#### Semantic Versioning Automation

**cargo-semver-checks** (242 lints in 2025):
```yaml
# .github/workflows/semver.yml
name: SemVer Check

on: [pull_request]

jobs:
  semver:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo semver-checks check-release
```

**Benefits**:
- Automated breaking change detection
- Witness generation: concrete programs proving violations
- 4x faster via parallelization (rayon)
- Prevents accidental API breaks

**cargo-release Workflow**:
```bash
# Developer workflow
git commit -m "feat: auto-fix engine"

# CI detects "feat:" prefix
# → suggests MINOR bump (0.13.1 → 0.14.0)

# Review process
cargo semver-checks       # ✅ No breaking changes
cargo test                # ✅ All tests pass
cargo audit               # ✅ No vulnerabilities

# Automated release
cargo release minor       # Bumps version, tags, publishes
                         # Generates CHANGELOG from commits
```

#### Fast, Evolutionary Testing

**cargo-nextest** (3x faster than cargo test):
```toml
# .config/nextest.toml
[profile.default]
retries = 1
slow-timeout = { period = "60s", terminate-after = 2 }

[profile.ci]
test-threads = "num-cpus"
retries = { backoff = "exponential", count = 3, delay = "1s" }
```

**Benefits**:
- Up to 3x faster test execution
- Beautiful terminal UI with progress
- Filtersets: group tests by category
- CI partitioning: distribute across workers
- Retry flaky tests automatically

**proptest** (Property-Based Testing):
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn composite_key_format_always_valid(
        entity_key in "[a-z-]{3,20}",
        locale in "(en|fr|es|de|ja)-(US|FR|ES|DE|JP)"
    ) {
        let key = format!("entity:{}@{}", entity_key, locale);
        let node = create_node_with_key(&key);

        let issues = validate_node(&node);
        let format_issues: Vec<_> = issues.iter()
            .filter(|i| i.rule == "COMPOSITE_KEY_FORMAT")
            .collect();

        prop_assert!(format_issues.is_empty());
    }
}
```

**cargo-mutants** (Mutation Testing):
```bash
# Find test quality gaps
cargo mutants

# Example output:
# MISSED: Changed `<` to `<=` in validate_node (line 183)
# → No test caught this mutation!
# ACTION: Add boundary test
```

**insta** (Snapshot Testing) - Already Used:
```rust
#[test]
fn test_mermaid_generator_output() {
    let output = MermaidGenerator.generate(&root).unwrap();
    insta::assert_snapshot!(output);
}
// Creates: snapshots/test_mermaid_generator_output.snap
// Review with: cargo insta review
```

#### Security & Dependency Auditing

**cargo-audit** (RustSec Advisory Database):
```yaml
# .github/workflows/security.yml
- name: Security Audit
  run: cargo audit
  continue-on-error: false
```

**deny.toml** Policy:
```toml
[advisories]
vulnerability = "deny"
unmaintained = "warn"
ignore = [
    "RUSTSEC-2025-0012",  # backoff (neo4rs transitive) - waiting on neo4rs 0.9.0
]

[licenses]
allow = ["MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause"]
```

**Quarterly Review Process**:
1. Check if ignored advisories have fixes
2. Update dependencies with fixes
3. Re-run `cargo deny check`
4. Document any new exceptions

#### Improved CI/CD Pipeline

**Pre-Commit Hooks** (`.git/hooks/pre-commit`):
```bash
#!/bin/bash
cargo fmt --check || exit 1
cargo clippy -- -D warnings || exit 1
cargo nextest run || exit 1
cargo semver-checks check-release || exit 1
cargo deny check || exit 1
cargo audit || exit 1
```

**Parallel CI Jobs**:
```yaml
jobs:
  test:
    strategy:
      matrix:
        partition: [1, 2, 3, 4]
    steps:
      - run: cargo nextest run --partition count:${{ matrix.partition }}/4

  semver:
    steps:
      - run: cargo semver-checks

  security:
    steps:
      - run: cargo deny check && cargo audit

  schema-drift:
    steps:
      - run: cargo run -- schema stats --format=json
      - name: Check counts
        # Existing drift detection logic
```

**Automated PR Comments**:
```yaml
- name: Comment PR
  uses: actions/github-script@v7
  with:
    script: |
      const report = JSON.parse(fs.readFileSync('validation-report.json'));

      const comment = `
      ## Schema Validation Report

      ${report.status === 'pass' ? '✅' : '❌'} ${report.summary.errors} errors, ${report.summary.warnings} warnings

      ### Quality Metrics
      - Coverage: ${report.metrics.validation_coverage}%
      - Compliance: ${report.metrics.rule_compliance.KEY_REQUIRED}%

      ### Drift Trend
      ${report.drift_trend.direction === 'improving' ? '↓' : '↑'} ${report.drift_trend.change} violations vs previous

      ### Fix Suggestions
      \`\`\`bash
      novanet schema validate --fix --safe
      \`\`\`
      `;

      github.rest.issues.createComment({ ...context.repo, issue_number: context.issue.number, body: comment });
```

#### Intelligent Version Management

**Roadmap**:

```
v0.13.1 (current)
    │
    ├─ Sprint 1 Complete
    ▼
v0.14.0 (minor bump)
    ├─ Auto-fix engine
    ├─ Fix suggestions
    └─ Dry-run mode
    │
    ├─ Sprint 2 Complete
    ▼
v0.15.0 (minor bump)
    ├─ Hooks system
    ├─ Plugin architecture
    └─ Refactored validation rules
    │
    ├─ Sprint 3 Complete + Stabilization
    ▼
v1.0.0 (major bump)
    ├─ API stability guarantees
    ├─ cargo-semver-checks: prevent accidental breaks
    ├─ Comprehensive documentation
    ├─ 100% rule coverage
    └─ 0 mutants survived
```

**Automated Workflow**:
1. Developer: `git commit -m "feat: auto-fix engine"`
2. CI: Detects `feat:` prefix → suggests MINOR bump
3. Review: `cargo semver-checks` confirms no breaking changes
4. Merge: `cargo-release minor` → bumps version → tags v0.14.0
5. CHANGELOG: Generated automatically from conventional commits

**Versioning Rules**:
- `feat:` → MINOR bump (new features, backward compatible)
- `fix:` → PATCH bump (bug fixes)
- `BREAKING CHANGE:` → MAJOR bump (API changes)
- `chore:`, `docs:` → no version change

## Success Criteria

### Sprint 1 (Auto-Fix + Suggestions)
- ✅ 3 fixers implemented: Composite Key, Property Order, Timestamps
- ✅ `novanet schema validate --fix` command works
- ✅ Dry-run mode for preview
- ✅ Fix suggestions in JSON/Terminal output
- ✅ 100% test coverage with proptest
- ✅ Zero regression vs baseline

### Sprint 2 (Hooks + Plugin Architecture)
- ✅ Hook system with 4+ built-in hooks
- ✅ ValidationRule trait with 5 rules refactored
- ✅ Plugin directory for custom rules
- ✅ Configuration file (`novanet.toml`)
- ✅ Snapshot tests for all hooks

### Sprint 3 (Performance + Advanced)
- ✅ 4x faster validation via rayon parallelization
- ✅ Quality metrics dashboard (Coverage, Compliance, Drift)
- ✅ HTML report generation
- ✅ Mutation testing: 0 mutants survived
- ✅ Criterion benchmarks for performance tracking

### Ecosystem Integration
- ✅ cargo-nextest: 3x faster CI
- ✅ cargo-semver-checks: automated SemVer validation
- ✅ cargo-mutants: test quality verification
- ✅ cargo-audit: automated vulnerability scanning
- ✅ proptest: property-based testing for invariants

### v1.0.0 Readiness
- ✅ API stability guarantees
- ✅ Comprehensive documentation
- ✅ Zero breaking changes without major version bump
- ✅ All quality metrics > 95%
- ✅ Community feedback incorporated

## Implementation Timeline

```
Week 1:
├── Day 1-2: Sprint 1 (Auto-Fix RED-GREEN)
├── Day 3: Sprint 1 (Auto-Fix REFACTOR + proptest)
└── Day 4-5: Sprint 2 (Hooks RED-GREEN)

Week 2:
├── Day 1-2: Sprint 2 (Hooks REFACTOR + plugin arch)
├── Day 3-4: Sprint 3 (Performance + rayon)
└── Day 5: Sprint 3 (Reporting + mutation testing)

Week 3:
├── Day 1-2: Ecosystem integration (nextest, semver-checks)
├── Day 3: Documentation + examples
└── Day 4-5: v0.14.0 release prep + CHANGELOG
```

## Migration Strategy

### Phase 1: Backward Compatible (v0.14.0)
- Keep existing `schema_rules.rs` working
- Add new validation engine alongside
- `--fix` flag is opt-in
- No breaking changes to CLI

### Phase 2: Deprecation (v0.15.0)
- Mark old validation functions as deprecated
- Add migration guide
- Warn users in CLI output

### Phase 3: Removal (v1.0.0)
- Remove deprecated code
- Full migration to new architecture
- Major version bump

## References

### Ecosystem Research
- [cargo-semver-checks 2025 Year in Review](https://predr.ag/blog/cargo-semver-checks-2025-year-review/) - 242 lints, 4x performance
- [cargo-nextest](https://nexte.st/) - Next-gen test runner
- [Mutation Testing in Rust](https://matklad.github.io/2021/05/31/how-to-test.html) - cargo-mutants active development
- [Rust Security Best Practices 2025](https://anssi-fr.github.io/rust-guide/) - Validation patterns, cargo-audit

### ADRs Referenced
- ADR-024: Trait = Data Origin (defined/authored/imported/generated/retrieved)
- ADR-029: *Native Pattern (unified suffix)
- ADR-030: Slug Ownership (Page owns URL)

### Related Documents
- `.claude/rules/schema-standard.md` - BLOC ordering specification
- `tools/novanet/src/parsers/schema_rules.rs` - Current implementation
- `.github/workflows/rust-ci.yml` - CI pipeline

---

**Next Steps**: Mark this design approved → Create TDD implementation plan → Execute Sprint 1
