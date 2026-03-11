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
