# Plan: Vérification Cohérence Schéma ↔ Instance

**Objectif**: Garantir que toutes les instances Neo4j respectent les contraintes `required: true` définies dans les YAMLs.

**Date**: 2026-02-17
**Version**: v0.13.1
**Status**: DRAFT (en attente de précisions utilisateur)

---

## Principe

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  COHÉRENCE SCHÉMA ↔ INSTANCE                                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  YAML (Source of Truth)          Neo4j (Runtime Data)                       │
│  ━━━━━━━━━━━━━━━━━━━━           ━━━━━━━━━━━━━━━━━━━                        │
│                                                                             │
│  Page:                                                                      │
│    standard_properties:          MATCH (p:Page)                             │
│      key:                        WHERE p.key IS NULL      ← VIOLATION       │
│        required: true            RETURN p                                   │
│                                                                             │
│      display_name:               MATCH (p:Page)                             │
│        required: true            WHERE p.display_name IS NULL ← VIOLATION   │
│                                                                             │
│      description:                MATCH (p:Page)                             │
│        required: false           WHERE p.description IS NULL ← OK           │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Phase 1: Analyse des Schémas YAML

### 1.1 Parser Tous les YAMLs

**Input**: `packages/core/models/node-classes/**/*.yaml` (61 fichiers)

**Output**: Structure `SchemaDefinition` pour chaque Class

```rust
struct SchemaDefinition {
    class_name: String,
    realm: String,
    layer: String,
    required_properties: Vec<RequiredProperty>,
    optional_properties: Vec<OptionalProperty>,
}

struct RequiredProperty {
    name: String,
    prop_type: String,        // string, integer, boolean, datetime
    pattern: Option<String>,   // regex pattern si composite key
}
```

**Action**:
```rust
// Utiliser le parser existant
let nodes = yaml_node::parse_all_nodes(&root)?;

// Extraire les required properties
for node in nodes {
    let required = node.def.standard_properties
        .iter()
        .filter(|(_, prop)| prop.required == Some(true))
        .collect();
}
```

### 1.2 Créer le Registre des Contraintes

**Output**: `schema_constraints.json`

```json
{
  "Page": {
    "required": ["key", "display_name", "description", "created_at", "updated_at"],
    "optional": ["icon"]
  },
  "Block": {
    "required": ["key", "display_name", "created_at", "updated_at"],
    "optional": ["description", "icon"]
  },
  "EntityNative": {
    "required": ["key", "entity_key", "locale_key", "display_name", "created_at", "updated_at"],
    "optional": ["description"]
  }
}
```

**Commande**:
```bash
cargo run -- schema analyze --output=schema_constraints.json
```

---

## Phase 2: Interrogation Neo4j

### 2.1 Query Cypher par Class

Pour chaque Class, générer une query de validation :

```cypher
// Template pour Page
MATCH (n:Page)
OPTIONAL MATCH (n)-[:HAS_CLASS]->(c:Class {name: "Page"})
RETURN
  n.key AS node_key,
  n.key IS NOT NULL AS has_key,
  n.display_name IS NOT NULL AS has_display_name,
  n.description IS NOT NULL AS has_description,
  n.created_at IS NOT NULL AS has_created_at,
  n.updated_at IS NOT NULL AS has_updated_at,

  // Violations
  CASE
    WHEN n.key IS NULL THEN ['key']
    ELSE []
  END +
  CASE
    WHEN n.display_name IS NULL THEN ['display_name']
    ELSE []
  END +
  CASE
    WHEN n.description IS NULL THEN ['description']
    ELSE []
  END +
  CASE
    WHEN n.created_at IS NULL THEN ['created_at']
    ELSE []
  END +
  CASE
    WHEN n.updated_at IS NULL THEN ['updated_at']
    ELSE []
  END AS missing_required
```

### 2.2 Requête Globale (Toutes Classes)

```cypher
// Trouver TOUTES les violations across all classes
MATCH (n)
WHERE n:Page OR n:Block OR n:Entity OR n:EntityNative // ... (61 labels)
OPTIONAL MATCH (n)-[:HAS_CLASS]->(c:Class)

WITH n, c, labels(n)[0] AS class_label
WITH n, class_label,
  // Check required properties based on class
  CASE class_label
    WHEN 'Page' THEN
      CASE WHEN n.key IS NULL THEN ['key'] ELSE [] END +
      CASE WHEN n.display_name IS NULL THEN ['display_name'] ELSE [] END +
      CASE WHEN n.description IS NULL THEN ['description'] ELSE [] END
    WHEN 'EntityNative' THEN
      CASE WHEN n.key IS NULL THEN ['key'] ELSE [] END +
      CASE WHEN n.entity_key IS NULL THEN ['entity_key'] ELSE [] END +
      CASE WHEN n.locale_key IS NULL THEN ['locale_key'] ELSE [] END
    // ... autres classes
    ELSE []
  END AS missing_required

WHERE size(missing_required) > 0

RETURN
  class_label,
  n.key AS instance_key,
  missing_required,
  size(missing_required) AS violation_count
ORDER BY violation_count DESC, class_label
```

---

## Phase 3: Détection des Violations

### 3.1 Structure de Rapport

```rust
struct ViolationReport {
    class_name: String,
    instance_key: Option<String>,
    missing_properties: Vec<String>,
    severity: Severity,
}

enum Severity {
    Critical,   // `key` manquant → instance non identifiable
    High,       // propriété required manquante
    Medium,     // propriété optionnelle manquante mais recommandée
}
```

### 3.2 Algorithme de Détection

```rust
fn detect_violations(
    schema: &SchemaDefinition,
    instances: Vec<Neo4jNode>
) -> Vec<ViolationReport> {
    let mut violations = Vec::new();

    for instance in instances {
        let mut missing = Vec::new();

        // Check each required property
        for required_prop in &schema.required_properties {
            if !instance.has_property(&required_prop.name) {
                missing.push(required_prop.name.clone());
            }
        }

        if !missing.is_empty() {
            let severity = if missing.contains(&"key".to_string()) {
                Severity::Critical
            } else {
                Severity::High
            };

            violations.push(ViolationReport {
                class_name: schema.class_name.clone(),
                instance_key: instance.get_key(),
                missing_properties: missing,
                severity,
            });
        }
    }

    violations
}
```

---

## Phase 4: Rapport de Violations

### 4.1 Format Console

```
╔═══════════════════════════════════════════════════════════════════════════╗
║  SCHEMA COHERENCE VALIDATION REPORT                                       ║
╠═══════════════════════════════════════════════════════════════════════════╣
║                                                                           ║
║  Total Classes: 61                                                        ║
║  Total Instances: 1,247                                                   ║
║  Violations: 23 instances across 5 classes                                ║
║                                                                           ║
╠═══════════════════════════════════════════════════════════════════════════╣
║  CRITICAL (3) - Missing `key` property                                    ║
╠═══════════════════════════════════════════════════════════════════════════╣
║                                                                           ║
║  ❌ Page (instance: UNKNOWN)                                              ║
║     Missing: key, display_name                                            ║
║                                                                           ║
║  ❌ Block (instance: UNKNOWN)                                             ║
║     Missing: key                                                          ║
║                                                                           ║
╠═══════════════════════════════════════════════════════════════════════════╣
║  HIGH (20) - Missing required properties                                  ║
╠═══════════════════════════════════════════════════════════════════════════╣
║                                                                           ║
║  ⚠️  EntityNative (entity:qr-code@fr-FR)                                  ║
║     Missing: created_at, updated_at                                       ║
║                                                                           ║
║  ⚠️  Page (page:homepage)                                                 ║
║     Missing: description                                                  ║
║                                                                           ║
║  ... (18 more)                                                            ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
```

### 4.2 Format JSON

```json
{
  "summary": {
    "total_classes": 61,
    "total_instances": 1247,
    "violations": 23,
    "by_severity": {
      "critical": 3,
      "high": 20
    }
  },
  "violations": [
    {
      "class": "Page",
      "instance_key": null,
      "missing_properties": ["key", "display_name"],
      "severity": "critical"
    },
    {
      "class": "EntityNative",
      "instance_key": "entity:qr-code@fr-FR",
      "missing_properties": ["created_at", "updated_at"],
      "severity": "high"
    }
  ]
}
```

---

## Phase 5: Stratégies de Correction

### 5.1 Auto-Fix (Safe)

Pour les propriétés générables automatiquement :

```rust
match missing_property {
    "created_at" | "updated_at" => {
        // Auto-fix: ajouter timestamp actuel
        let now = Utc::now();
        instance.set_property("created_at", now);
        instance.set_property("updated_at", now);
    }
    "description" => {
        // Auto-fix: générer description par défaut
        let desc = format!("{} node in the {} layer (realm: {})",
            schema.name, schema.layer, schema.realm);
        instance.set_property("description", desc);
    }
    "key" => {
        // CRITIQUE: Ne peut pas auto-fix, demander intervention humaine
        Err("Cannot auto-generate key - human intervention required")
    }
    _ => {
        // Autres propriétés: dépend du type
    }
}
```

### 5.2 Manual Fix

Pour les propriétés nécessitant intervention humaine :

```cypher
// Template Cypher pour fix manuel
MATCH (n:Page)
WHERE n.key IS NULL

// Option 1: Supprimer l'instance orpheline
DELETE n

// Option 2: Assigner une clé manuelle
SET n.key = "generated-page-key-{uuid}"
```

### 5.3 Batch Fix

```bash
# Dry-run (preview)
cargo run -- schema coherence --check

# Auto-fix les propriétés safe
cargo run -- schema coherence --fix-safe

# Fix manuel avec prompts interactifs
cargo run -- schema coherence --fix-interactive

# Force fix (dangereux)
cargo run -- schema coherence --fix-force
```

---

## Phase 6: Intégration CI/CD

### 6.1 GitHub Action

```yaml
name: Schema Coherence Check

on:
  pull_request:
    paths:
      - 'packages/core/models/**/*.yaml'
      - 'packages/db/seed/**/*.cypher'

jobs:
  coherence:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Start Neo4j
        run: docker-compose -f packages/db/docker-compose.yml up -d

      - name: Seed Database
        run: pnpm infra:seed

      - name: Check Schema Coherence
        run: |
          cd tools/novanet
          cargo run -- schema coherence --check --format=json > violations.json

      - name: Fail if Violations
        run: |
          violations=$(jq '.summary.violations' violations.json)
          if [ "$violations" -gt 0 ]; then
            echo "❌ Found $violations schema violations"
            cat violations.json
            exit 1
          fi
```

### 6.2 Pre-Commit Hook

```bash
#!/bin/bash
# .claude/hooks/schema-coherence-check.sh

if [[ $(git diff --cached --name-only | grep -E '\.yaml$|\.cypher$') ]]; then
    echo "🔍 Checking schema coherence..."
    cargo run -- schema coherence --check --format=table

    if [ $? -ne 0 ]; then
        echo "❌ Schema coherence violations detected"
        echo "Run: cargo run -- schema coherence --fix-safe"
        exit 1
    fi
fi
```

---

## Phase 7: Commandes CLI

### 7.1 Nouvelle Commande

```rust
// src/commands/coherence.rs

pub struct CoherenceCommand {
    #[arg(long)]
    check: bool,           // Vérifier seulement

    #[arg(long)]
    fix_safe: bool,        // Auto-fix propriétés safe

    #[arg(long)]
    fix_interactive: bool, // Fix interactif

    #[arg(long)]
    class: Option<String>, // Limiter à une classe

    #[arg(long, default_value = "table")]
    format: OutputFormat,  // table, json, cypher
}

pub async fn execute(cmd: CoherenceCommand, graph: &Graph) -> Result<()> {
    // 1. Load schema definitions
    let schemas = load_all_schemas()?;

    // 2. Query Neo4j instances
    let violations = check_coherence(&graph, &schemas).await?;

    // 3. Report
    match cmd.format {
        OutputFormat::Table => print_table(&violations),
        OutputFormat::Json => print_json(&violations),
        OutputFormat::Cypher => generate_fix_cypher(&violations),
    }

    // 4. Fix if requested
    if cmd.fix_safe {
        apply_safe_fixes(&graph, &violations).await?;
    }

    Ok(())
}
```

### 7.2 Usage

```bash
# Check all classes
cargo run -- schema coherence --check

# Check specific class
cargo run -- schema coherence --check --class=Page

# Auto-fix safe properties
cargo run -- schema coherence --fix-safe

# Generate Cypher for manual fixes
cargo run -- schema coherence --check --format=cypher > fix.cypher

# Interactive fix
cargo run -- schema coherence --fix-interactive
```

---

## Phase 8: Tests

### 8.1 Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_missing_required_property() {
        let schema = SchemaDefinition {
            class_name: "Page".to_string(),
            required_properties: vec![
                RequiredProperty {
                    name: "key".to_string(),
                    prop_type: "string".to_string(),
                    pattern: None,
                }
            ],
            ..Default::default()
        };

        let instance = Neo4jNode {
            labels: vec!["Page".to_string()],
            properties: HashMap::new(), // No properties!
        };

        let violations = detect_violations(&schema, vec![instance]);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].missing_properties, vec!["key"]);
        assert_eq!(violations[0].severity, Severity::Critical);
    }

    #[test]
    fn test_no_violations_when_all_required_present() {
        let schema = SchemaDefinition {
            class_name: "Page".to_string(),
            required_properties: vec![
                RequiredProperty {
                    name: "key".to_string(),
                    prop_type: "string".to_string(),
                    pattern: None,
                }
            ],
            ..Default::default()
        };

        let instance = Neo4jNode {
            labels: vec!["Page".to_string()],
            properties: hashmap!{
                "key" => "page:homepage".to_string()
            },
        };

        let violations = detect_violations(&schema, vec![instance]);

        assert_eq!(violations.len(), 0);
    }
}
```

### 8.2 Integration Tests

```rust
#[tokio::test]
#[ignore] // Requires Neo4j
async fn test_coherence_check_against_real_db() {
    let graph = connect_to_test_db().await.unwrap();

    // Seed avec une instance violante
    graph.run(query("
        CREATE (p:Page {display_name: 'Test Page'})
        // Missing: key, description, created_at, updated_at
    ")).await.unwrap();

    let schemas = load_all_schemas().unwrap();
    let violations = check_coherence(&graph, &schemas).await.unwrap();

    let page_violations: Vec<_> = violations.iter()
        .filter(|v| v.class_name == "Page")
        .collect();

    assert_eq!(page_violations.len(), 1);
    assert!(page_violations[0].missing_properties.contains(&"key".to_string()));
}
```

---

## Questions Ouvertes

### Q1: Scope
- ✅ Vérifier TOUTES les 61 classes ?
- ✅ Ou se limiter aux classes principales (Page, Block, Entity, EntityNative) ?

### Q2: Performance
- Avec 200 locales × 1000 entities = 200K instances
- Query Cypher peut être lente
- Solution: Pagination ? Parallélisation ?

### Q3: Auto-Fix Aggressif
- Quelles propriétés peut-on auto-générer sans risque ?
  - ✅ `created_at`, `updated_at` → timestamp actuel
  - ✅ `description` → template par défaut
  - ❌ `key` → trop critique
  - ❓ `display_name` → peut-on générer depuis `key` ?

### Q4: Instances Orphelines
- Si `key` manque → instance non identifiable
- Delete automatiquement ? Ou flag pour review manuel ?

### Q5: Composite Keys
- EntityNative: `key = "entity:{entity_key}@{locale_key}"`
- Si `key` présent mais `entity_key` manque → violation ?
- Auto-extract depuis `key` ?

---

## Prochaines Étapes

1. **Utilisateur donne précisions** sur :
   - Scope (toutes classes ou subset)
   - Stratégie auto-fix (conservative vs aggressive)
   - Traitement instances orphelines

2. **Implémentation Phase 1** (Analyse YAML)
   - Extraire required properties
   - Générer schema_constraints.json

3. **Implémentation Phase 2** (Query Neo4j)
   - Générer queries Cypher
   - Tester sur DB de dev

4. **Implémentation Phase 3-4** (Détection + Rapport)
   - Structure ViolationReport
   - Format console + JSON

5. **Tests + Documentation**
   - Tests unitaires
   - Tests intégration Neo4j
   - Mise à jour CLAUDE.md

---

## Références

- ADR-003: YAML-First Architecture
- ADR-033: Auto-Fix System (infrastructure réutilisable)
- `tools/novanet/src/parsers/yaml_node.rs` (parser existant)
- `tools/novanet/src/validation/autofix/` (patterns auto-fix)
