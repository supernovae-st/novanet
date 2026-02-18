# NovaNet: YAML ↔ Neo4j Property Coherence Plan

**Date**: 2026-02-17
**Status**: Planning (v3 — batched execution with TDD + skills)
**Context**: v0.13.1 LLM-First BLOC Schema

---

## Mission

Full coherence between YAML class definitions and Neo4j instance properties,
visible in the TUI: Class view (right panel = YAML source) ↔ Instance view
(center panel = Neo4j properties) must be identical in content and order.

**Source of truth**: YAML (ADR-003). Neo4j follows YAML, never the inverse.

---

## TUI Coherence Goal

```
┌─ Class View ───────────────────┐   ┌─ Instance View ─────────────────────┐
│  YAML defines:                 │   │  Neo4j has:                          │
│  1. key          string ✓      │   │  key            "entity:foo@fr-FR"   │
│  2. entity_key   string ✓      │ = │  entity_key     "foo"                │
│  3. locale_key   string ✓      │   │  locale_key     "fr-FR"              │
│  4. display_name string ✓      │   │  display_name   "..."                │
│  5. description  string ✓      │   │  description    "..."                │
│  6. created_at   datetime ✓    │   │  created_at     2024-01-15T10:00:00  │
│  7. updated_at   datetime ✓    │   │  updated_at     2024-01-15T10:00:00  │
│  8. curation_status enum ✓     │   │  curation_status "human_authored"    │
│  ...                           │   │  ...                                 │
└────────────────────────────────┘   └─────────────────────────────────────┘
    SAME PROPS, SAME ORDER, NO EXTRAS, NO MISSING
```

**TUI drift badge** (in Class tree nodes):
```
▼ ◆ Class:EntityNative [324]  ⚠ 3 drift
```
Driven by pre-computed `coherence-report.json` file (no live Neo4j query in TUI).

---

## Skills Per Batch

| Batch | Skills Used |
|-------|-------------|
| 0 | `neo4j-architect` (audit Cypher) |
| 1 | `spn-rust:rust-pro`, `spn-powers:test-driven-development`, `feature-dev:code-reviewer` |
| 2–6 | `neo4j-architect`, `spn-powers:verification-before-completion`, `spn-powers:requesting-code-review` |
| 7 | `neo4j-architect`, `spn-powers:verification-before-completion` |
| 8 | `neo4j-architect`, `feature-dev:code-reviewer` |
| 9 | `shell-scripting:bash-pro`, `spn-powers:verification-before-completion` |
| 10 | `spn-rust:rust-pro`, `spn-powers:test-driven-development`, `feature-dev:code-reviewer` |

---

## 10 Axes of Improvement (A–J)

```
A  REMOVE    props in Neo4j absent from YAML (orphelins)
B  ADD       required YAML props absent from Neo4j
C  FIX       type mismatches (float→int, string→datetime)
D  VALIDATE  enum values not listed in YAML enum:
E  VERIFY    composite key doesn't match pattern
F  CONSISTENT heterogeneous prop sets across instances
G  ENFORCE   Neo4j 5 type constraints (IS :: FLOAT, etc.)
H  ORDER     REMOVE+SET canonical BLOC 4 order
I  CI        cargo run schema coherence --check in seed.sh
J  ARCS      on-arc properties drift (169 arc types)
```

---

## Execution Batches

```
╔══════════════════════════════════════════════════════════════════════╗
║  BATCH 0  Live Audit (prerequisite — no implementation)              ║
║  BATCH 1  Rust src/coherence/ module (TDD)                          ║
║  BATCH 2  P0 — EntityNative migration (324 instances)               ║
║  BATCH 3  P0 — Style rebuild (1 instance, full)                     ║
║  BATCH 4  P1 — Country / GeoRegion / GeoSubRegion / Culture         ║
║  BATCH 5  P2 — EntityCategory / Market                              ║
║  BATCH 6  P3 — BlockNative (9 instances, anchor_slug removal)       ║
║  BATCH 7  Type constraints in 00-constraints.cypher (Axis G)        ║
║  BATCH 8  Arc coherence audit + fixes (Axis J)                      ║
║  BATCH 9  CI integration — seed.sh + coherence-report.json          ║
║  BATCH 10 schema_rules new rules (8 new rules, TDD)                 ║
╚══════════════════════════════════════════════════════════════════════╝
```

---

## BATCH 0 — Live Audit

**Goal**: Know EXACTLY what's in Neo4j before touching anything.
**Skill**: `neo4j-architect`
**Output**: Audit report (no writes)

### Tasks

**B0.1 — Property audit per node type**
```cypher
// EntityNative — what props exist?
MATCH (n:EntityNative) RETURN DISTINCT keys(n) AS prop_set, count(n) AS cnt
ORDER BY cnt DESC LIMIT 10

// Which orphan props?
MATCH (n:EntityNative)
WITH n, [k IN keys(n) WHERE NOT k IN
  ['key','entity_key','locale_key','display_name','description',
   'created_at','updated_at','llm_context','curation_status','status',
   'version','definition','purpose','benefits','usage_examples',
   'audience_segment','cultural_notes']]
  AS orphans
WHERE size(orphans) > 0
RETURN orphans, count(n) AS cnt ORDER BY cnt DESC
```

Run same pattern for: `Style`, `Country`, `GeoRegion`, `Culture`, `Market`,
`EntityCategory`, `BlockNative`

**B0.2 — Type audit (Neo4j 5 valueType())**
```cypher
// Check actual stored types
MATCH (n:EntityNative)
RETURN DISTINCT
  valueType(n.created_at) AS created_at_type,
  valueType(n.version) AS version_type,
  count(n) AS instances
LIMIT 5
```

**B0.3 — Heterogeneity check**
```cypher
MATCH (n:EntityNative)
WITH collect(DISTINCT keys(n)) AS all_prop_sets
WHERE size(all_prop_sets) > 1
RETURN size(all_prop_sets) AS variants, all_prop_sets
```

**B0.4 — Arc property audit**
```cypher
// Arc props existing vs YAML schema
MATCH ()-[r:HAS_NATIVE]->()
RETURN DISTINCT keys(r) AS arc_props, count(r) AS cnt
ORDER BY cnt DESC LIMIT 5
```

**Verification**: All audit queries return results (no errors).
**✅ DONE when**: Audit report complete, no implementation yet.
**→ Ready for feedback.**

---

## BATCH 1 — Rust `src/coherence/` Module (TDD)

**Goal**: Implement the coherence detection engine in Rust.
**Skills**: `spn-rust:rust-pro`, `spn-powers:test-driven-development`, `feature-dev:code-reviewer`
**Approach**: RED → GREEN → REFACTOR (strict TDD)

### Tasks

**B1.1 — Module scaffold**
```
src/coherence/
  mod.rs      (pub use)
  drift.rs    (DriftKind, DriftReport, NodeDrift, ArcDrift)
  type_map.rs (TYPE_MAP, YamlPropType)
  generator.rs (CoherenceGenerator, TypeConstraintGenerator)
  validator.rs (async fn validate_node_coherence)
```

**B1.2 — RED: Write tests FIRST (will fail)**

```rust
// drift.rs tests
#[test]
fn test_drift_kind_orphan_serializes() { /* must fail */ }

#[test]
fn test_drift_kind_type_mismatch() { /* must fail */ }

// type_map.rs tests
#[test]
fn test_yaml_prop_type_from_str_float() { /* must fail */ }

#[test]
fn test_neo4j_constraint_for_datetime() {
    let t = YamlPropType::Datetime;
    assert_eq!(t.neo4j_type_constraint(), "IS :: ZONED DATETIME");
    // must fail: YamlPropType doesn't exist yet
}

#[test]
fn test_type_map_covers_all_yaml_types() {
    // All YAML type strings in TYPE_MAP
    let yaml_types = ["string","int","float","boolean","datetime","string[]"];
    for t in yaml_types {
        assert!(TYPE_MAP.iter().any(|(k, _)| *k == t));
    }
}

// generator.rs snapshot test
#[test]
fn test_coherence_generator_entity_native() {
    // snapshot with insta
    let script = CoherenceGenerator::generate_for("EntityNative", &mock_yaml()).unwrap();
    insta::assert_snapshot!(script.content);
}
```

Run `cargo nextest run src/coherence` → ALL RED ✅

**B1.3 — GREEN: Implement minimum to pass**

```rust
// drift.rs
pub enum DriftKind {
    Orphan,
    Missing { yaml_type: String },
    TypeMismatch { yaml_type: String, neo4j_type: String },
    EnumViolation { invalid_value: String, allowed: Vec<String> },
    KeyPatternViolation { actual: String, pattern: String },
    OrderViolation { expected_position: usize, actual_position: usize },
    HeterogeneousInstances { variant_count: usize },
    ArcOrphanProp { arc_type: String, prop: String },
    SatelliteOrphan,
}

pub struct DriftReport {
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub schema_version: String,
    pub nodes: Vec<NodeDrift>,
    pub arcs: Vec<ArcDrift>,
    pub total_drift_count: usize,
    pub critical_count: usize,
}

// type_map.rs
pub const TYPE_MAP: &[(&str, &str)] = &[
    ("string",   "STRING"),
    ("int",      "INTEGER"),
    ("float",    "FLOAT"),
    ("boolean",  "BOOLEAN"),
    ("datetime", "ZONED DATETIME"),
    ("string[]", "LIST<STRING NOT NULL>"),
];

pub enum YamlPropType { String, Integer, Float, Boolean, Datetime, StringList, IntegerList }

impl YamlPropType {
    pub fn from_yaml_str(s: &str) -> Option<Self> {
        match s {
            "string" => Some(Self::String),
            "int" => Some(Self::Integer),
            "float" => Some(Self::Float),
            "boolean" => Some(Self::Boolean),
            "datetime" => Some(Self::Datetime),
            "string[]" => Some(Self::StringList),
            _ => None,
        }
    }

    pub fn neo4j_type_constraint(&self) -> &'static str {
        match self {
            Self::Float    => "IS :: FLOAT",
            Self::Datetime => "IS :: ZONED DATETIME",
            Self::Integer  => "IS :: INTEGER",
            Self::String   => "IS :: STRING",
            Self::StringList => "IS :: LIST<STRING NOT NULL>",
            _ => "IS :: STRING",
        }
    }
}
```

Run `cargo nextest run src/coherence` → ALL GREEN ✅

**B1.4 — REFACTOR: Clean, clippy**
```bash
cargo clippy -- -D warnings
cargo fmt
cargo nextest run
```

**B1.5 — Code review**
Use `feature-dev:code-reviewer` agent on `src/coherence/`.

**B1.6 — Wire CLI command (schema coherence)**
In `src/commands/schema.rs`:
```
schema validate    → existing (YAML-only)
schema coherence   → new (YAML + Neo4j)
  --check          → produce DriftReport JSON
  --generate       → emit Cypher scripts
  --apply          → execute (requires --confirm)
```

**Verification**:
```bash
cargo nextest run          # all tests green
cargo clippy -- -D warnings # 0 warnings
cargo build                # compiles
```
**✅ DONE when**: Tests green, 0 clippy, CLI command registered.
**→ Ready for feedback.**

---

## BATCH 2 — P0: EntityNative Migration (324 instances)

**Goal**: Remove orphan props (`locale`, `title`, `slug_terms`), ensure standard props present.
**Skill**: `neo4j-architect`, `spn-powers:verification-before-completion`
**Risk**: HIGH (324 instances, P0 — must be idempotent + rollback-safe)

### Tasks

**B2.1 — Read EntityNative YAML**
```bash
cat packages/core/models/node-classes/org/semantic/entity-native.yaml
```

**B2.2 — Pre-migration validation query (baseline)**
```cypher
// BEFORE: count orphan props
MATCH (n:EntityNative)
WITH n, [k IN keys(n) WHERE k IN ['locale','title','slug_terms']] AS orphans
WHERE size(orphans) > 0
RETURN sum(size(orphans)) AS total_orphan_props, count(n) AS affected_instances
```

**B2.3 — Write idempotent migration Cypher**
File: `packages/db/seed/migration-coherence-01-entity-native.cypher`

```cypher
// migration-coherence-01-entity-native.cypher
// NovaNet v0.13.1 coherence — EntityNative
// Removes: locale, title, slug_terms (ADR-030 violation)
// Adds: display_name default if missing
// Idempotent: safe to run multiple times

// STEP 1: Remove orphan props
MATCH (n:EntityNative)
WHERE n.locale IS NOT NULL OR n.title IS NOT NULL OR n.slug_terms IS NOT NULL
REMOVE n.locale, n.title, n.slug_terms
RETURN count(n) AS cleaned
;

// STEP 2: Add display_name default if missing (derive from key)
MATCH (n:EntityNative)
WHERE n.display_name IS NULL AND n.key IS NOT NULL
SET n.display_name = split(split(n.key, '@')[0], ':')[1]
RETURN count(n) AS display_name_added
;

// STEP 3: Set version default if missing
MATCH (n:EntityNative)
WHERE n.version IS NULL
SET n.version = 1
RETURN count(n) AS version_added
;

// STEP 4: Verification (should return 0 rows)
MATCH (n:EntityNative)
WITH n, [k IN keys(n) WHERE k IN ['locale','title','slug_terms']] AS remaining_orphans
WHERE size(remaining_orphans) > 0
RETURN n.key AS still_dirty, remaining_orphans
LIMIT 5
;
```

**B2.4 — Execute on dev Neo4j**
```bash
# Via pnpm infra:seed (picks up migration file) OR direct
cat packages/db/seed/migration-coherence-01-entity-native.cypher | \
  docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword
```

**B2.5 — Post-migration validation (must return 0)**
```cypher
MATCH (n:EntityNative)
WITH n,
  [k IN keys(n) WHERE k IN ['locale','title','slug_terms']] AS orphans,
  [k IN ['key','entity_key','locale_key'] WHERE NOT k IN keys(n)] AS missing_critical
WHERE size(orphans) > 0 OR size(missing_critical) > 0
RETURN n.key, orphans, missing_critical LIMIT 10
```

**B2.6 — Update seed file to not reintroduce orphans**
Check `packages/db/seed/` files that create EntityNative nodes:
- Remove any `locale:`, `title:`, `slug_terms:` from MERGE/SET statements

**Rollback strategy**: If validation fails, the REMOVE is idempotent.
Re-run with `SET n.locale = props.locale` to restore from `WITH n, properties(n) AS props` snapshot.
(Document in migration header: "Rollback: add back locale from entity.key pattern if needed")

**Verification** (use `spn-powers:verification-before-completion`):
```bash
# Run validation query → 0 rows
# Run post-migration check → 0 orphans
# TUI: cargo run -- tui → EntityNative instances clean
```
**✅ DONE when**: Validation query = 0 rows, seed updated, TUI shows clean.
**→ Ready for feedback.**

---

## BATCH 3 — P0: Style Rebuild (1 instance)

**Goal**: Rebuild the single Style instance from 8 props to 35+ props per YAML.
**Skill**: `neo4j-architect`, `spn-powers:verification-before-completion`

### Tasks

**B3.1 — Read Style YAML**
```bash
cat packages/core/models/node-classes/shared/locale/style.yaml
```

**B3.2 — Inspect current Style instance**
```cypher
MATCH (s:Style) RETURN properties(s)
```

**B3.3 — Write Style rebuild migration**
File: `packages/db/seed/migration-coherence-02-style.cypher`

Strategy: DELETE + re-seed from scratch (1 instance only — safe).

**B3.4 — Execute + validate**
```cypher
// Post-migration: Style must have all YAML props
MATCH (s:Style)
WITH s, [k IN ['key','display_name','description','created_at','updated_at'] WHERE NOT k IN keys(s)] AS missing
WHERE size(missing) > 0
RETURN s.key, missing
```

**✅ DONE when**: Style instance matches YAML schema exactly.
**→ Ready for feedback.**

---

## BATCH 4 — P1: Country / GeoRegion / Culture

**Goal**: Rename `name` → `display_name`, add missing standard props.
**Skill**: `neo4j-architect`, `spn-powers:verification-before-completion`, `spn-powers:requesting-code-review`

### Tasks

**B4.1 — Read YAMLs**
```bash
cat packages/core/models/node-classes/shared/geography/country.yaml
cat packages/core/models/node-classes/shared/geography/geo-region.yaml
cat packages/core/models/node-classes/shared/locale/culture.yaml
```

**B4.2 — Pre-migration audit**
```cypher
// Country: name vs display_name
MATCH (n:Country)
RETURN exists(n.name) AS has_name, exists(n.display_name) AS has_display_name, count(n)

// Culture: old format detection
MATCH (n:Culture) RETURN DISTINCT keys(n) AS prop_set, count(n) AS cnt
ORDER BY cnt DESC LIMIT 5
```

**B4.3 — Write migrations (one file per node type)**
- `migration-coherence-03-country.cypher`
- `migration-coherence-04-geo-region.cypher`
- `migration-coherence-05-culture.cypher`

Culture is a MAJOR migration (old format → new format).
Strategy: For each Culture, create new props from old, REMOVE old props.

**B4.4 — Execute in dependency order** (Country → GeoRegion → Culture)

**B4.5 — Post-migration validation per type**

**B4.6 — Code review** via `spn-powers:requesting-code-review`

**✅ DONE when**: All 3 node types validated, 0 orphan props.
**→ Ready for feedback.**

---

## BATCH 5 — P2: EntityCategory / Market

**Goal**: Add `llm_context`, fix type mismatches.
**Skill**: `neo4j-architect`, `spn-powers:verification-before-completion`

### Tasks

**B5.1 — Read YAMLs**
```bash
cat packages/core/models/node-classes/shared/config/entity-category.yaml
# Market YAML path TBD from audit
```

**B5.2 — EntityCategory: add llm_context**
```cypher
// migration-coherence-06-entity-category.cypher
MATCH (ec:EntityCategory)
WHERE ec.llm_context IS NULL
SET ec.llm_context = "USE: when categorizing entities. TRIGGERS: entity type, category. NOT: for instance filtering. RELATES: Entity (categorized), EntityCategory (category)."
```

**B5.3 — Market: fix type mismatches**
```cypher
// Fix population_millions from INTEGER to FLOAT
MATCH (m:Market)
WHERE valueType(m.population_millions) = 'INTEGER'
SET m.population_millions = toFloat(m.population_millions)
```

**✅ DONE when**: Validation queries = 0 issues, types correct.
**→ Ready for feedback.**

---

## BATCH 6 — P3: BlockNative

**Goal**: Remove `anchor_slug` and `generated` (wrong type) orphan props.
**Skill**: `neo4j-architect`

### Tasks

**B6.1 — Audit BlockNative**
```cypher
MATCH (n:BlockNative)
WITH n, [k IN keys(n) WHERE k IN ['anchor_slug','generated']] AS orphans
WHERE size(orphans) > 0
RETURN n.key, orphans
```

**B6.2 — Migration**
File: `migration-coherence-07-block-native.cypher`

```cypher
MATCH (n:BlockNative)
WHERE n.anchor_slug IS NOT NULL OR (n.generated IS NOT NULL AND valueType(n.generated) = 'STRING')
REMOVE n.anchor_slug, n.generated
```

**✅ DONE when**: 0 orphan props, head-seo-meta instances have slug/full_path/meta_title/meta_description.
**→ Ready for feedback.**

---

## BATCH 7 — Type Constraints (Axis G)

**Goal**: Add Neo4j 5 `IS :: TYPE` constraints to prevent future drift.
**Must run AFTER Batches 2–6** (data clean, constraints won't block).
**Skill**: `neo4j-architect`, `spn-powers:verification-before-completion`

### Tasks

**B7.1 — Generate type constraints**
Use `TypeConstraintGenerator` (Rust) to produce additions to `00-constraints.cypher`.

```bash
cargo run -- schema generate  # TypeConstraintGenerator runs as part of schema generate
```

**B7.2 — Review generated constraints**
Example output:
```cypher
// EntityNative type constraints
CREATE CONSTRAINT en_version_int IF NOT EXISTS
  FOR (n:EntityNative) REQUIRE n.version IS :: INTEGER;
CREATE CONSTRAINT en_created_datetime IF NOT EXISTS
  FOR (n:EntityNative) REQUIRE n.created_at IS :: ZONED DATETIME;
CREATE CONSTRAINT en_entity_key_str IF NOT EXISTS
  FOR (n:EntityNative) REQUIRE n.entity_key IS :: STRING;
CREATE CONSTRAINT en_locale_key_str IF NOT EXISTS
  FOR (n:EntityNative) REQUIRE n.locale_key IS :: STRING;
CREATE CONSTRAINT market_pop_float IF NOT EXISTS
  FOR (n:Market) REQUIRE n.population_millions IS :: FLOAT;
```

**B7.3 — Apply to dev Neo4j + verify**
```cypher
// Verify constraint applied
SHOW CONSTRAINTS WHERE entityType = 'NODE' AND name STARTS WITH 'en_'
```

**✅ DONE when**: All constraints created, `SHOW CONSTRAINTS` lists them.
**→ Ready for feedback.**

---

## BATCH 8 — Arc Coherence (Axis J)

**Goal**: Audit and fix on-arc property drift for 6 priority arcs.
**Skill**: `neo4j-architect`, `feature-dev:code-reviewer`

**Priority arcs**:
- `HAS_NATIVE` → must have `locale` property
- `HAS_BLOCK` → must have `order` property (integer)
- `DERIVED_SLUG_FROM` → must have `extracted_terms`, `derivation_score`
- `SLUGIFIED_BY` → must have `validated`, `applied_rule`
- `LINKS_TO` → must have `via_blocks`, `link_type`, `pr_weight`
- `SEO_CLUSTER_OF` → must have `cluster_role`, `link_priority`

### Tasks

**B8.1 — Audit each arc**
```cypher
// HAS_NATIVE arc props audit
MATCH ()-[r:HAS_NATIVE]->()
WITH r, [k IN keys(r) WHERE NOT k IN ['locale']] AS orphan_arc_props,
        [k IN ['locale'] WHERE NOT k IN keys(r)] AS missing_arc_props
WHERE size(orphan_arc_props) > 0 OR size(missing_arc_props) > 0
RETURN type(r) AS arc, orphan_arc_props, missing_arc_props LIMIT 5
```

**B8.2 — Write migration for each arc with issues**
File: `migration-coherence-08-arc-coherence.cypher`

**B8.3 — Add arc coherence to DriftReport** (Rust `drift.rs` → `ArcDrift`)

**✅ DONE when**: Priority arcs have correct props, ArcDrift reports 0 critical.
**→ Ready for feedback.**

---

## BATCH 9 — CI Integration (Axis I)

**Goal**: `schema coherence --check` runs after every seed, blocks on drift.
**Skill**: `shell-scripting:bash-pro`, `spn-powers:verification-before-completion`

### Tasks

**B9.1 — Update `packages/db/seed.sh`**
```bash
# After all .cypher files executed, add:
echo "Running schema coherence check..."
cargo run --manifest-path tools/novanet/Cargo.toml -- \
  schema coherence --check --output=json > /tmp/coherence-report.json

CRITICAL=$(jq '.critical_count' /tmp/coherence-report.json)
if [ "$CRITICAL" -gt "0" ]; then
  echo "DRIFT DETECTED: $CRITICAL critical issues"
  jq '.nodes[] | select(.critical_count > 0) | {node: .name, issues: .drift_count}' \
    /tmp/coherence-report.json
  exit 1
fi

# Copy report for TUI badge
cp /tmp/coherence-report.json packages/db/coherence-report.json
echo "Schema coherence: OK"
```

**B9.2 — Test seed.sh still works**
```bash
pnpm infra:seed  # must exit 0 after clean data
```

**B9.3 — Test drift detection works**
Temporarily introduce a fake orphan prop, run seed.sh, verify exit 1.

**✅ DONE when**: `pnpm infra:seed` exits 0, drift detection exits 1, report file created.
**→ Ready for feedback.**

---

## BATCH 10 — New schema_rules (TDD)

**Goal**: Add 8 new YAML validation rules to the schema_rules engine.
**Skills**: `spn-rust:rust-pro`, `spn-powers:test-driven-development`, `feature-dev:code-reviewer`
**Approach**: RED → GREEN → REFACTOR

### New Rules

| Rule | Severity | Check |
|------|---------|-------|
| `ENUM_VALUES_DEFINED` | Error | Props with `enum:` must list all values |
| `KEY_PATTERN_DEFINED` | Error | Composite key nodes must have `pattern:` |
| `TYPE_CONSTRAINT_READY` | Warning | Scalar props must have explicit `type:` |
| `EXAMPLE_COHERENCE` | Warning | `example:` keys must match `properties:` keys |
| `LLM_CONTEXT_FORMAT` | Error | `llm_context` must contain USE:/TRIGGERS:/NOT:/RELATES: |
| `LLM_CONTEXT_QUALITY` | Warning | TRIGGERS: 4+ phrases, NOT: 2+ alternatives |
| `LLM_CONTEXT_SCOPE` | Warning | Should have SCOPE: cardinality hint |
| `TRIGGER_COLLISION_CHECK` | Warning | 3+ shared triggers, no NOT disambiguation |

### Tasks

**B10.1 — RED: Write tests for each rule**
In `src/parsers/schema_rules.rs` (or `src/validation/`):

```rust
#[test]
fn test_enum_values_defined_fails_when_empty_enum() {
    // YAML with enum: [] should produce ENUM_VALUES_DEFINED error
}

#[test]
fn test_llm_context_format_fails_without_use_section() {
    // YAML with llm_context: "no structure" → error
}

#[test]
fn test_llm_context_quality_requires_four_triggers() {
    // YAML with only 2 triggers → warning
}

#[test]
fn test_trigger_collision_check_across_nodes() {
    // Two nodes with same 3+ triggers, no NOT disambiguation → warning
}
```

Run → ALL RED ✅

**B10.2 — GREEN: Implement rules**
Each rule is a small `fn check_X(node: &ParsedNode) -> Vec<SchemaIssue>` function.
`TRIGGER_COLLISION_CHECK` operates across ALL nodes (different signature).

**B10.3 — REFACTOR + clippy**

**B10.4 — AutoFix for LLM_CONTEXT_FORMAT**
In `src/validation/autofix/description.rs` or new `llm_context.rs`:
```rust
impl AutoFix for LlmContextFormatFixer {
    fn can_fix(&self, issue: &SchemaIssue) -> bool {
        issue.rule == "LLM_CONTEXT_FORMAT"
    }
    fn fix(&self, node: &mut ParsedNode, issue: &SchemaIssue) -> Result<FixAction> {
        // Add USE:/TRIGGERS:/NOT:/RELATES: template if missing
    }
}
```

**B10.5 — Code review** via `feature-dev:code-reviewer`

**B10.6 — Run full validation on all 61 YAMLs**
```bash
cargo run -- schema validate --strict
```
Document all new warnings/errors found. Fix or accept-as-known-debt.

**Verification**:
```bash
cargo nextest run          # all tests green
cargo clippy -- -D warnings # 0 warnings
cargo run -- schema validate # new rules active
```

**✅ DONE when**: 8 new rules active, tests green, full validation run documented.
**→ Ready for feedback.**

---

## Post-All-Batches: Finishing

Use `spn-powers:finishing-a-development-branch` skill to:
1. Final verification: `cargo nextest run && cargo clippy -- -D warnings && pnpm infra:seed`
2. Present integration options (PR vs direct merge)
3. Write CHANGELOG entry

---

## Property Order (BLOC 4 Canonical)

For ALL node instances, properties must follow this order:

1. `key` — primary identity
2. `*_key` — denormalized keys (entity_key, page_key, block_key, locale_key)
3. `display_name`
4. `description`
5. `created_at`
6. `updated_at`
7. `llm_context` — instance-level (if defined in properties:)
8. `[publication]` — status, curation_status, version
9. `[content]` — definition, purpose, benefits, etc.
10. `[domain-specific]` — node-specific properties

---

## Migration Order (Dependencies)

```
1. EntityCategory     (leaf, no deps)
2. Country            (leaf in geo hierarchy)
3. GeoRegion          (depends on Country)
4. GeoSubRegion       (depends on GeoRegion)
5. Locale             (depends on Country + Language data)
6. Culture            (depends on Locale, Country)
7. Market             (depends on Country, Locale)
8. EntityNative       (depends on EntityCategory via BELONGS_TO)
9. PageNative         (depends on Entity + Page)
10. BlockNative       (depends on Page + BlockType)
11. Style             (leaf — full rebuild)
```

---

## Definition of Done (Per Node Type)

- [ ] BATCH 0 audit completed (know baseline)
- [ ] Migration Cypher written + idempotent + rollback documented
- [ ] Migration executed on dev Neo4j
- [ ] Pre/post validation queries return 0 issues
- [ ] Seed file updated (no regression source)
- [ ] TUI: Class view props == Instance view props
- [ ] Arc coherence checked (Axis J)
- [ ] Type constraints added to `00-constraints.cypher`
- [ ] Code reviewed via `feature-dev:code-reviewer`

---

## Files Involved

| File | Role |
|------|------|
| `packages/core/models/node-classes/**/*.yaml` | Source of truth (YAML schemas) |
| `packages/core/models/arc-classes/**/*.yaml` | Arc schemas (Axis J) |
| `packages/db/seed/migration-coherence-0X-*.cypher` | Migration scripts per node |
| `packages/db/seed/00-constraints.cypher` | + type constraints post-migration |
| `packages/db/seed.sh` | + coherence check step |
| `packages/db/coherence-report.json` | Pre-computed drift report for TUI |
| `tools/novanet/src/coherence/` | NEW: drift, generator, validator, type_map |
| `tools/novanet/src/generators/type_constraints.rs` | NEW: TypeConstraintGenerator |
| `tools/novanet/src/parsers/schema_rules.rs` | + 8 new rules |
| `tools/novanet/src/validation/autofix/` | + LlmContextFormatFixer |

---

## Rollback Strategy

Each migration file is:
1. **Idempotent**: safe to run multiple times (uses WHERE guards)
2. **Non-destructive for reads**: audit queries (STEP 4 in each file) are SELECT-only
3. **Documented rollback**: each file header explains how to reverse if needed

For P0 EntityNative (highest risk):
- The `locale` prop was removed → restore via: `SET n.locale = split(n.key, '@')[1]` if needed
- The `title` prop → restore from `display_name` if needed
- All changes are property-level, no arc deletions in Batches 2–6

**Emergency**: `pnpm infra:reset` restores from seed files (after seed files are updated = last resort)
