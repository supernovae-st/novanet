# v0.19.0 Migration Master Plan

**Version**: v0.19.0
**Date**: 2026-03-11
**Status**: PLANNING
**Scope**: MEGA-MIGRATION (~290 files)

---

## Spec Validée

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  8 STANDARD PROPERTIES — TOUTES LES 61 CLASSES                                ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  │ # │ Property      │ Type     │ Groupe    │ Description                   │ ║
║  │───┼───────────────┼──────────┼───────────┼───────────────────────────────│ ║
║  │ 1 │ key           │ string   │ IDENTITÉ  │ Identifiant unique            │ ║
║  │ 2 │ display_name  │ string   │ IDENTITÉ  │ Nom lisible                   │ ║
║  │ 3 │ node_class    │ string   │ IDENTITÉ  │ Classe du nœud                │ ║
║  │ 4 │ content       │ string   │ CONTENU   │ Markdown: WHAT the node IS    │ ║
║  │ 5 │ llm_context   │ string   │ CONTENU   │ Markdown: HOW to USE it       │ ║
║  │ 6 │ provenance    │ string   │ METADATA  │ JSON: {"source","file",...}   │ ║
║  │ 7 │ created_at    │ datetime │ METADATA  │ Création                      │ ║
║  │ 8 │ updated_at    │ datetime │ METADATA  │ Modification                  │ ║
║                                                                               ║
║  RÈGLES:                                                                      ║
║  • 8 props pour TOUT (data + schema) — zéro exception                         ║
║  • TOUT required — zéro optionnel                                             ║
║  • content + llm_context = markdown strings                                   ║
║  • provenance = JSON string (Neo4j limitation)                                ║
║  • node_class case: PascalCase = DATA, lowercase = SCHEMA                     ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Legacy Pollution Analysis

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  LEGACY POLLUTION — FULL INVENTORY                                            ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  PROBLÈME 1: description vs content                                           ║
║  ─────────────────────────────────────────────────────────────────────────    ║
║  │ Location              │ Current     │ Target   │ Files   │                ║
║  │───────────────────────┼─────────────┼──────────┼─────────│                ║
║  │ YAML models           │ description │ content  │ 55/57   │                ║
║  │ Seed files            │ mix         │ content  │ ~30     │                ║
║  │ MCP server queries    │ description │ content  │ 8       │                ║
║  │ TUI info.rs           │ description │ content  │ 1       │                ║
║  │ schema_rules.rs       │ description │ content  │ 1       │                ║
║                                                                               ║
║  PROBLÈME 2: provenance format                                                ║
║  ─────────────────────────────────────────────────────────────────────────    ║
║  │ Location              │ Current                │ Target              │    ║
║  │───────────────────────┼────────────────────────┼─────────────────────│    ║
║  │ Seeds anciens         │ created_by +           │ provenance JSON     │    ║
║  │                       │ created_by_file        │                     │    ║
║  │ Seeds récents         │ provenance_source +    │ provenance JSON     │    ║
║  │                       │ provenance_file        │                     │    ║
║  │ ADR-042               │ flattened props        │ JSON string         │    ║
║  │ MCP write.rs          │ created_by only        │ provenance JSON     │    ║
║                                                                               ║
║  PROBLÈME 3: node_class absent                                                ║
║  ─────────────────────────────────────────────────────────────────────────    ║
║  │ Location              │ Missing │ Action                              │   ║
║  │───────────────────────┼─────────┼─────────────────────────────────────│   ║
║  │ YAML standard_props   │ 57/57   │ Ajouter à tous                      │   ║
║  │ Seed files            │ 15+     │ Ajouter dans MERGE/SET              │   ║
║  │ Generators            │ partial │ Générer pour DATA nodes aussi       │   ║
║                                                                               ║
║  PROBLÈME 4: ordre des propriétés                                             ║
║  ─────────────────────────────────────────────────────────────────────────    ║
║  │ File                  │ Current Order                                 │   ║
║  │───────────────────────┼───────────────────────────────────────────────│   ║
║  │ schema_rules.rs       │ key,entity_key...,display_name,description,   │   ║
║  │                       │ llm_context,created_by,created_by_meta,       │   ║
║  │                       │ created_at,updated_at                         │   ║
║  │ yaml_panel.rs         │ key,display_name,content,llm_context,         │   ║
║  │                       │ node_class,provenance,created_at,updated_at   │   ║
║  │ info.rs               │ key,entity_key...,display_name,description,   │   ║
║  │                       │ llm_context,created_by,created_by_meta,       │   ║
║  │                       │ created_at,updated_at                         │   ║
║  │ TARGET (v0.19.0)      │ key,display_name,node_class,content,          │   ║
║  │                       │ llm_context,provenance,created_at,updated_at  │   ║
║                                                                               ║
║  PROBLÈME 5: deux STANDARD_PROPERTIES listes                                  ║
║  ─────────────────────────────────────────────────────────────────────────    ║
║  │ yaml_panel.rs (8)     │ info.rs (12)                                  │   ║
║  │─────────────────────────────────────────────────────────────────────│     ║
║  │ key                   │ key                                           │   ║
║  │ display_name          │ entity_key, page_key, block_key, locale_key   │   ║
║  │ content               │ display_name                                  │   ║
║  │ llm_context           │ description                                   │   ║
║  │ node_class            │ llm_context                                   │   ║
║  │ provenance            │ created_by                                    │   ║
║  │ created_at            │ created_by_meta                               │   ║
║  │ updated_at            │ created_at                                    │   ║
║  │                       │ updated_at                                    │   ║
║  │ → FUSIONNER en 8 props + composite keys séparément                    │   ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Checkpoint Protocol

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  CHECKPOINT PROTOCOL — EVERY PHASE ENDS WITH VERIFICATION                     ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  AFTER EACH PHASE:                                                            ║
║  ─────────────────────────────────────────────────────────────────────────    ║
║  1. ✅ Run verification commands (schema validate, tests, clippy)             ║
║  2. ✅ Check CSR if applicable (novanet_audit target=all)                     ║
║  3. ✅ Commit changes with granular message: type(scope): description         ║
║  4. ✅ Document blockers before proceeding                                    ║
║                                                                               ║
║  COMMIT PATTERN:                                                              ║
║  ─────────────────────────────────────────────────────────────────────────    ║
║  feat(schema): add node_class property to Entity YAML                         ║
║  fix(seed): rename description to content in 10-entities-bootstrap            ║
║  refactor(mcp): update queries to use content instead of description          ║
║                                                                               ║
║  CSR THRESHOLDS:                                                              ║
║  ─────────────────────────────────────────────────────────────────────────    ║
║  ≥0.95  → Healthy (green)  → Proceed                                          ║
║  0.85-0.95 → Warning (yellow) → Fix before proceeding                         ║
║  <0.85  → Critical (red)   → STOP, rollback if needed                         ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Phase 0: Documentation & ADRs

### 0.1 ADRs à créer/mettre à jour

| ADR | Status | Action |
|-----|--------|--------|
| ADR-037 | À CRÉER | Standard Properties Schema (8 props, order, types) |
| ADR-042 | À METTRE À JOUR | Provenance Tracking (JSON format, source enum) |
| ADR-027 | VÉRIFIER | llm_context pattern (USE/TRIGGERS/NOT) |
| ADR-034 | À METTRE À JOUR | Property Order (nouveau canonical order) |
| ADR-029 | VÉRIFIER | *Native pattern (content vs description) |

### 0.2 Documentation

| File | Action |
|------|--------|
| `docs/plans/2026-03-11-standard-properties-migration.md` | ✅ DONE |
| `docs/plans/2026-03-11-v019-migration-master-plan.md` | CE FICHIER |
| `CLAUDE.md` (novanet) | Mettre à jour section "Required Properties" |
| `CHANGELOG.md` | Préparer entry v0.19.0 |

### CHECKPOINT 0: Documentation Ready

```bash
# Verification
ls -la docs/plans/2026-03-11-*.md  # Both plans exist
grep "ADR-044" dx/adr/novanet/*.md   # ADR updated

# Commit
git add docs/ dx/adr/
git commit -m "docs(adr): add ADR-044 eight standard properties

- Define DEFINITION/ROLE/SPECS pattern for content
- Define USE/TRIGGERS/NOT/RELATES pattern for llm_context
- Document 8 required properties for all nodes

Co-Authored-By: Claude <noreply@anthropic.com>"
```

**Gate**: All ADRs and docs created before proceeding.

---

## Phase 1: Templates & Schema

### 1.1 Template Standard Properties

**File**: `packages/core/models/_standard-properties-template.yaml`

```yaml
# v0.19.0 Standard Properties Template
# 8 required properties for ALL 61 node classes

standard_properties:
  # === IDENTITÉ (1-3) ===
  key:
    type: string
    required: true
    indexed: true
    description: "Unique identifier"

  display_name:
    type: string
    required: true
    indexed: true
    description: "Human-readable name"

  node_class:
    type: string
    required: true
    indexed: true
    description: "Node class name (PascalCase=DATA, lowercase=SCHEMA)"

  # === CONTENU (4-5) ===
  content:
    type: string
    required: true
    description: "Markdown: WHAT the node IS"

  llm_context:
    type: string
    required: true
    description: "Markdown: HOW to USE it (ADR-027 pattern)"

  # === METADATA (6-8) ===
  provenance:
    type: string
    required: true
    description: "JSON string: {source, file, workflow}"

  created_at:
    type: datetime
    required: true
    description: "Creation timestamp"

  updated_at:
    type: datetime
    required: true
    description: "Last modification timestamp"
```

### 1.2 Provenance JSON Schema

```json
{
  "source": "seed" | "nika" | "mcp",
  "file": "10-entities-bootstrap.cypher",
  "workflow": "entity-native-gen",
  "workflow_run": "run-2026-03-11-001"
}
```

### CHECKPOINT 1: Templates Created

```bash
# Verification
ls packages/core/models/_standard-properties-template.yaml
cat packages/core/models/_standard-properties-template.yaml | grep -c "required: true"  # Should be 8

# Commit
git add packages/core/models/_standard-properties-template.yaml
git commit -m "feat(schema): add standard properties template

- 8 required properties: key, display_name, node_class, content, llm_context, provenance, created_at, updated_at
- Canonical order defined
- provenance JSON schema documented

Co-Authored-By: Claude <noreply@anthropic.com>"
```

**Gate**: Template file exists and validated.

---

## Phase 2: YAML Node Classes (61 files)

### 2.1 Files par Realm/Layer

| Realm | Layer | Count | Files |
|-------|-------|-------|-------|
| shared | config | 3 | locale-config, region-config, script-config |
| shared | locale | 5 | locale, locale-voice, locale-culture, locale-script, locale-region |
| shared | geography | 7 | geo-*, country, region, city |
| shared | knowledge | 21 | containers (7) + atoms (14) |
| org | config | 1 | org-config |
| org | foundation | 8 | project, brand, brand-voice, persona, audience, campaign, channel, touchpoint |
| org | structure | 3 | page, block, block-type |
| org | semantic | 2 | entity, entity-native |
| org | instruction | 4 | page-structure, page-instruction, block-structure, block-instruction |
| org | output | 3 | page-native, block-native, content-asset |
| **SCHEMA** | - | 4 | realm, layer, class, arc-class |
| **TOTAL** | | **61** | |

### 2.2 Changements par fichier

Pour CHAQUE fichier:
- [ ] Renommer `description` → `content`
- [ ] Ajouter `node_class` après `display_name`
- [ ] Ajouter `provenance` après `llm_context`
- [ ] Vérifier `llm_context` présent et required
- [ ] Réordonner dans l'ordre canonical (1-8)
- [ ] Vérifier `created_at` et `updated_at` présents

### CHECKPOINT 2: YAML Schema Valid

```bash
# Verification - Schema validation
cargo run -- schema validate --strict
# Expected: "✅ Schema valid: 57 node classes, 145 arc classes"

# Verification - Property count check
find packages/core/models/node-classes -name "*.yaml" | wc -l  # Should be 57

# Verification - No description property (should be content now)
grep -r "description:" packages/core/models/node-classes/ | grep -v "# description" | wc -l  # Should be 0

# Tests
cargo test --lib -- schema
# Expected: All schema tests pass

# Commit (per layer for granularity)
git add packages/core/models/node-classes/shared/
git commit -m "feat(schema): update shared realm YAMLs to v0.19.0 standard properties

- Renamed description to content
- Added node_class property
- Added provenance property
- Reordered to canonical (key, display_name, node_class, content, llm_context, provenance, created_at, updated_at)

Co-Authored-By: Claude <noreply@anthropic.com>"

git add packages/core/models/node-classes/org/
git commit -m "feat(schema): update org realm YAMLs to v0.19.0 standard properties

- Renamed description to content
- Added node_class property
- Added provenance property
- Reordered to canonical order

Co-Authored-By: Claude <noreply@anthropic.com>"
```

**Gate**: `cargo run -- schema validate` passes with zero warnings.

---

## Phase 3: Seed Files (~50 files)

### 3.1 Seed Files par catégorie

| Category | Count | Path |
|----------|-------|------|
| Schema | 5 | `00-*.cypher` (taxonomy, realms, layers) |
| Locale | 10 | `01-*.cypher` (locales, expressions) |
| Entities | 5 | `10-*.cypher`, `11-*.cypher` |
| Pages/Blocks | 10 | `40-*.cypher`, `48-*.cypher`, `49-*.cypher` |
| SEO/GEO | 10 | `51-*.cypher`, `52-*.cypher` |
| Other | 10 | Various |

### 3.2 Changements par seed file

Pour CHAQUE seed file:
- [ ] Renommer `description` → `content`
- [ ] Ajouter `node_class = 'ClassName'`
- [ ] Ajouter `provenance = '{"source":"seed","file":"XX-name.cypher"}'`
- [ ] Réordonner properties dans l'ordre canonical
- [ ] Vérifier `llm_context` présent

### 3.3 Exemple AVANT/APRÈS

```cypher
// AVANT
MERGE (e:Entity {key: 'entity:qr-code'})
ON CREATE SET
  e.display_name = 'QR Code',
  e.description = 'A two-dimensional barcode...',
  e.llm_context = 'USE: when generating...',
  e.created_at = datetime(),
  e.updated_at = datetime()

// APRÈS
MERGE (e:Entity {key: 'entity:qr-code'})
ON CREATE SET
  e.display_name = 'QR Code',
  e.node_class = 'Entity',
  e.content = 'A two-dimensional barcode...',
  e.llm_context = 'USE: when generating QR code content. TRIGGERS: QR, scan, barcode, 2D code. NOT: for NFC (use entity:nfc).',
  e.provenance = '{"source":"seed","file":"10-entities-bootstrap.cypher"}',
  e.created_at = datetime(),
  e.updated_at = datetime()
```

### CHECKPOINT 3: Seeds Validated

```bash
# Reset and reseed database
cargo run -- db reset
cargo run -- db seed
# Expected: "✅ Seeded X nodes, Y arcs"

# Verify in Neo4j
cypher-shell -u neo4j -p novanetpassword "MATCH (n) WHERE n.content IS NULL RETURN count(n)"
# Expected: 0 (all nodes have content)

cypher-shell -u neo4j -p novanetpassword "MATCH (n) WHERE n.node_class IS NULL RETURN count(n)"
# Expected: 0 (all nodes have node_class)

cypher-shell -u neo4j -p novanetpassword "MATCH (n) WHERE n.provenance IS NULL RETURN count(n)"
# Expected: 0 (all nodes have provenance)

# CSR Check
cargo run -- mcp-server &
# Then via MCP: novanet_audit(target="all")
# Expected: CSR ≥ 0.95

# Commit (per seed category)
git add packages/db/seed/00-*.cypher
git commit -m "fix(db): update schema seeds to v0.19.0 standard properties"

git add packages/db/seed/01-*.cypher packages/db/seed/02-*.cypher
git commit -m "fix(db): update locale seeds to v0.19.0 standard properties"

git add packages/db/seed/10-*.cypher packages/db/seed/11-*.cypher
git commit -m "fix(db): update entity seeds to v0.19.0 standard properties"

git add packages/db/seed/40-*.cypher packages/db/seed/48-*.cypher packages/db/seed/49-*.cypher
git commit -m "fix(db): update page/block seeds to v0.19.0 standard properties"
```

**Gate**: `cargo run -- db seed` succeeds AND CSR ≥ 0.95.

---

## Phase 4: MCP Server (Rust)

### 4.1 Files à modifier

| File | Changes |
|------|---------|
| `tools/novanet-mcp/src/tools/generate.rs` | Query `content` not `description`, include `provenance` |
| `tools/novanet-mcp/src/tools/write.rs` | Validate provenance JSON, require all 8 props |
| `tools/novanet-mcp/src/tools/check.rs` | Validate property order, provenance schema |
| `tools/novanet-mcp/src/tools/audit.rs` | Check for missing standard props |
| `tools/novanet-mcp/src/tools/search.rs` | Return `content` not `description` |
| `tools/novanet-mcp/src/tools/traverse.rs` | Include new props in response |

### 4.2 Query Changes

```rust
// BEFORE
let query = "MATCH (n) RETURN n.key, n.display_name, n.description";

// AFTER
let query = "MATCH (n) RETURN n.key, n.display_name, n.node_class, n.content, n.llm_context, n.provenance, n.created_at, n.updated_at";
```

### CHECKPOINT 4: MCP Server Updated

```bash
# Run MCP tests
cd tools/novanet-mcp
cargo test
# Expected: All tests pass

# Clippy
cargo clippy -- -D warnings
# Expected: Zero warnings

# Integration test with novanet_generate
# Start MCP server and call:
# novanet_generate(focus_key="entity:qr-code", locale="fr-FR", mode="block")
# Expected: Response contains 'content' field, not 'description'

# novanet_check validation test
# novanet_check(operation="upsert_node", class="EntityNative", key="test", properties={})
# Expected: errors[] contains "missing required property: content"

# Commit
git add tools/novanet-mcp/src/tools/
git commit -m "refactor(mcp): update queries to use content instead of description

- generate.rs: Query content not description
- write.rs: Validate provenance JSON, require all 8 props
- check.rs: Validate property order, provenance schema
- search.rs: Return content not description

Co-Authored-By: Claude <noreply@anthropic.com>"
```

**Gate**: MCP tests pass AND `novanet_generate` returns `content`.

---

## Phase 5: TUI (Rust)

### 5.1 Files à modifier

| File | Changes |
|------|---------|
| `tools/novanet/src/tui/panels/yaml_panel.rs` | Update STANDARD_PROPERTIES constant |
| `tools/novanet/src/tui/panels/info.rs` | Display order, provenance parsing |
| `tools/novanet/src/tui/theme.rs` | Property group colors if needed |

### 5.2 Constants

```rust
// tools/novanet/src/tui/panels/yaml_panel.rs
pub const STANDARD_PROPERTIES: &[&str] = &[
    "key",
    "display_name",
    "node_class",
    "content",
    "llm_context",
    "provenance",
    "created_at",
    "updated_at",
];

pub const STANDARD_PROPERTIES_COUNT: usize = 8;
```

### CHECKPOINT 5: TUI Updated

```bash
# Run TUI tests
cd tools/novanet
cargo test --lib -- tui
# Expected: All TUI tests pass

# Manual verification
cargo run -- tui
# Navigate to any node
# Expected: YAML panel shows properties in order:
#   key, display_name, node_class, content, llm_context, provenance, created_at, updated_at
# Expected: Info panel shows "STANDARD: 8"

# Commit
git add tools/novanet/src/tui/
git commit -m "refactor(tui): update STANDARD_PROPERTIES to v0.19.0 canonical order

- yaml_panel.rs: 8 properties in canonical order
- info.rs: Display content instead of description
- Removed description from property list

Co-Authored-By: Claude <noreply@anthropic.com>"
```

**Gate**: TUI displays 8 standard properties in correct order.

---

## Phase 6: Generators (Rust)

### 6.1 Files à modifier

| File | Changes |
|------|---------|
| `tools/novanet/src/generators/cypher.rs` | Generate with new order |
| `tools/novanet/src/generators/typescript.rs` | Update interfaces |
| `tools/novanet/src/generators/mermaid.rs` | Update diagrams if needed |

### 6.2 TypeScript Output

```typescript
// Generated: packages/core/src/graph/types.ts
interface StandardProperties {
  key: string;
  display_name: string;
  node_class: string;
  content: string;
  llm_context: string;
  provenance: string; // JSON string
  created_at: string; // ISO datetime
  updated_at: string; // ISO datetime
}

interface ProvenanceObject {
  source: 'seed' | 'nika' | 'mcp';
  file?: string;
  workflow?: string;
  workflow_run?: string;
}
```

### CHECKPOINT 6: Generators Updated

```bash
# Regenerate all artifacts
cargo run -- schema generate
# Expected: "✅ Generated: cypher, typescript, mermaid"

# Verify TypeScript output
cat packages/core/src/graph/types.ts | grep "content: string"
# Expected: Line exists (not description)

cat packages/core/src/graph/types.ts | grep "node_class: string"
# Expected: Line exists

# TypeScript type-check
cd packages/core && pnpm type-check
# Expected: Zero errors

# Commit
git add tools/novanet/src/generators/
git commit -m "refactor(generators): update Cypher/TypeScript generation for v0.19.0

- cypher.rs: Generate with canonical property order
- typescript.rs: StandardProperties interface with 8 props
- ProvenanceObject type added

Co-Authored-By: Claude <noreply@anthropic.com>"

git add packages/core/src/graph/
git commit -m "feat(core): regenerate TypeScript types for v0.19.0 standard properties

Co-Authored-By: Claude <noreply@anthropic.com>"
```

**Gate**: `pnpm type-check` passes AND generated files reflect new schema.

---

## Phase 7: Validators (Rust)

### 7.1 Files à modifier

| File | Changes |
|------|---------|
| `tools/novanet/src/validation/mod.rs` | Add property order rule |
| `tools/novanet/src/validation/rules/` | New rules for 8 props |
| `tools/novanet/src/validation/autofix/property_order.rs` | Update canonical order |

### 7.2 Validation Rules

```rust
// New validation rules
RULE_STANDARD_PROPS_COUNT: "All nodes must have exactly 8 standard properties"
RULE_STANDARD_PROPS_ORDER: "Properties must be in canonical order (1-8)"
RULE_PROVENANCE_JSON: "provenance must be valid JSON with 'source' field"
RULE_NODE_CLASS_CASE: "node_class must be PascalCase (DATA) or lowercase (SCHEMA)"
```

### CHECKPOINT 7: Validators Updated

```bash
# Run validator tests
cargo test --lib -- validation
# Expected: All validation tests pass

# Test new validation rules
cargo run -- schema validate --strict
# Expected: Reports any violations of new rules

# Test autofix
cargo run -- schema validate --fix
# Expected: Auto-fixes property order violations

# Commit
git add tools/novanet/src/validation/
git commit -m "feat(validation): add v0.19.0 standard properties validation rules

- RULE_STANDARD_PROPS_COUNT: Must have 8 properties
- RULE_STANDARD_PROPS_ORDER: Canonical order validation
- RULE_PROVENANCE_JSON: Valid JSON with source field
- RULE_NODE_CLASS_CASE: PascalCase for DATA, lowercase for SCHEMA
- property_order.rs autofix updated

Co-Authored-By: Claude <noreply@anthropic.com>"
```

**Gate**: `cargo run -- schema validate --strict` passes.

---

## Phase 8: Verification

### 8.1 Tests à exécuter

```bash
# Schema validation
cargo run -- schema validate --strict

# All tests pass
cargo test

# Clippy clean
cargo clippy -- -D warnings

# Seed works
cargo run -- db reset
cargo run -- db seed

# TUI displays correctly
cargo run -- tui
```

### 8.2 Verification Checklist

- [ ] `schema validate` passes with zero warnings
- [ ] All 1210+ tests pass
- [ ] Clippy zero warnings
- [ ] `db seed` executes without errors
- [ ] TUI shows "STANDARD: 8" for all nodes
- [ ] TUI displays properties in correct order
- [ ] Neo4j Browser shows properties in order
- [ ] MCP `novanet_check` validates new schema
- [ ] MCP `novanet_generate` returns `content` not `description`
- [ ] MCP `novanet_audit` finds no missing props

### CHECKPOINT 8: FINAL VERIFICATION

```bash
# ═══════════════════════════════════════════════════════════════════════════════
# FINAL VERIFICATION CHECKLIST — ALL MUST PASS BEFORE v0.19.0 RELEASE
# ═══════════════════════════════════════════════════════════════════════════════

# 1. Schema validation
cargo run -- schema validate --strict
# Expected: "✅ Schema valid: 57 node classes, 145 arc classes"

# 2. All Rust tests
cargo test
# Expected: "test result: ok. 1210+ passed"

# 3. Clippy clean
cargo clippy -- -D warnings
# Expected: Zero warnings

# 4. Database seed
cargo run -- db reset && cargo run -- db seed
# Expected: "✅ Seeded X nodes, Y arcs"

# 5. CSR audit
# Via MCP: novanet_audit(target="all")
# Expected: CSR ≥ 0.95 (healthy)

# 6. TypeScript build
cd packages/core && pnpm build
cd apps/studio && pnpm build
# Expected: Zero errors

# 7. TypeScript tests
pnpm test
# Expected: All tests pass

# 8. Neo4j property verification
cypher-shell -u neo4j -p novanetpassword "
  MATCH (n)
  WHERE n.content IS NULL OR n.node_class IS NULL OR n.provenance IS NULL
  RETURN count(n) AS missing_props
"
# Expected: 0

# 9. TUI manual check
cargo run -- tui
# Verify: Properties display in canonical order

# 10. MCP integration test
# novanet_generate(focus_key="entity:qr-code", locale="fr-FR", mode="block")
# Expected: Response contains 'content', 'denomination_forms', 'context_anchors'

# ═══════════════════════════════════════════════════════════════════════════════
# RELEASE COMMIT
# ═══════════════════════════════════════════════════════════════════════════════

# Update CHANGELOG
git add CHANGELOG.md CHANGELOG-LATEST.md
git commit -m "docs(changelog): add v0.19.0 release notes

- 8 standard properties for all nodes
- description → content migration
- provenance JSON format
- Canonical property order

Co-Authored-By: Claude <noreply@anthropic.com>"

# Tag release
git tag -a v0.19.0 -m "v0.19.0 - Standard Properties Migration"
git push origin main --tags
```

**Gate**: All 10 verification steps pass → v0.19.0 RELEASED.

---

## Execution Strategy

### Option A: Big Bang (Risky)
- Do everything in one PR
- High risk of breakage
- Fast if it works

### Option B: Phased (Recommended)
1. **PR 1**: ADRs + Templates + Plan docs
2. **PR 2**: YAML Node Classes (61 files)
3. **PR 3**: Seed Files (~50 files)
4. **PR 4**: MCP Server + TUI + Generators
5. **PR 5**: Final verification + CHANGELOG

### Option C: Parallel Agents
- Launch 10 agents for different areas
- Merge results
- High parallelism, needs coordination

---

## Agent Tasks (for parallel execution)

| Agent # | Task | Files |
|---------|------|-------|
| 1 | ADRs (create/update) | 5 ADRs |
| 2 | YAML shared/config + shared/locale | 8 files |
| 3 | YAML shared/geography + shared/knowledge | 28 files |
| 4 | YAML org/* | 21 files |
| 5 | YAML schema (realm, layer, class, arc) | 4 files |
| 6 | Seed files 00-19 | ~15 files |
| 7 | Seed files 40-59 | ~20 files |
| 8 | MCP Server Rust | ~10 files |
| 9 | TUI + Generators Rust | ~10 files |
| 10 | Validators + Tests | ~10 files |

---

## Rollback Plan

Si ça casse:

1. `git revert` to pre-migration commit
2. Keep `description` as alias for `content` temporarily
3. Add migration flag `--legacy-properties`
4. Cypher script to rename properties back

---

## Timeline

| Phase | Effort | Dependency |
|-------|--------|------------|
| Phase 0: Docs | 1h | None |
| Phase 1: Templates | 30min | Phase 0 |
| Phase 2: YAML | 3h | Phase 1 |
| Phase 3: Seeds | 2h | Phase 2 |
| Phase 4: MCP | 2h | Phase 2 |
| Phase 5: TUI | 1h | Phase 2 |
| Phase 6: Generators | 1h | Phase 2 |
| Phase 7: Validators | 1h | Phase 2 |
| Phase 8: Verification | 2h | All |
| **TOTAL** | **~13h** | |

---

## Success Criteria

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  v0.19.0 MIGRATION COMPLETE WHEN:                                             ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  ✅ All 61 YAML node classes have 8 standard properties in order              ║
║  ✅ All ~50 seed files use new property names and order                       ║
║  ✅ All ADRs created/updated (037, 042, 027, 034, 029)                        ║
║  ✅ MCP server queries/validates new schema                                   ║
║  ✅ TUI displays "STANDARD: 8" and correct order                              ║
║  ✅ Generators output correct TypeScript/Cypher                               ║
║  ✅ Validators check new rules                                                ║
║  ✅ `cargo test` passes (1210+ tests)                                         ║
║  ✅ `cargo clippy -- -D warnings` clean                                       ║
║  ✅ `db seed` works end-to-end                                                ║
║  ✅ CHANGELOG.md updated with v0.19.0 entry                                   ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```
