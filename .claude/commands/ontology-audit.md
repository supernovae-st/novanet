---
description: Comprehensive audit of YAML to TypeScript to Neo4j synchronization
allowed-tools: Bash, Read, Glob, Grep
---

# NovaNet Ontology Synchronization Audit

Comprehensive audit verifying YAML source of truth propagates correctly to all generated artifacts.

## Audit Layers

### Layer 1: YAML Coherence

```bash
cargo run --quiet -- schema validate
```

Checks:
- All node YAMLs have required fields (name, realm, layer, trait)
- All arc YAMLs have required fields (name, family, scope, source, target)
- File paths match YAML content (`shared/knowledge/term.yaml` has `realm: shared, layer: knowledge`)
- No orphan references (arcs reference existing nodes)

### Layer 2: Mermaid Diagrams

```bash
cargo run --quiet -- doc generate --dry-run
```

Checks:
- Diagram node counts match YAML counts
- Arc relationships in diagrams exist in arc YAML
- View definitions in `views/*.yaml` are valid

### Layer 3: TypeScript Types

Compare generated files with YAML source:

| Generated File | Source |
|----------------|--------|
| `packages/core/src/schema/node-classes.ts` | `node-classes/**/*.yaml` |
| `packages/core/src/schema/arc-classes.ts` | `arc-classes/**/*.yaml` |
| `packages/core/src/config/taxonomy.ts` | `taxonomy.yaml` |
| `packages/core/src/graph/visual-encoding.ts` | `visual-encoding.yaml` |

### Layer 4: Neo4j Seeds

Compare Cypher seeds with YAML:

| Seed File | Source |
|-----------|--------|
| `packages/db/seed/00.5-taxonomy.cypher` | `taxonomy.yaml` |
| `packages/db/seed/01-schema.cypher` | `node-classes/**/*.yaml` |
| `packages/db/seed/02-arcs.cypher` | `arc-classes/**/*.yaml` |

### Layer 5: Use Case Tracing

Trace complete use cases through the system:

#### Use Case: Block Generation
```
BlockInstruction (org/instruction)
  --[:HAS_INSTRUCTION]--> Block (org/structure)
  --[:HAS_NATIVE {locale}]--> BlockNative (org/output)
```

#### Use Case: Locale Knowledge
```
Locale (shared/config)
  --[:HAS_TERMS]--> TermSet (shared/knowledge)
  --[:CONTAINS_TERM]--> Term (shared/knowledge)
```

#### Use Case: SEO Pipeline
```
Page (org/structure)
  --[:REPRESENTS]--> Entity (org/semantic)
  --[:TARGETS]--> SEOKeyword (shared/knowledge)
```

## Output Format

```
+===============================================================================+
|                     ONTOLOGY SYNCHRONIZATION AUDIT                            |
+===============================================================================+

  Layer 1: YAML Coherence
  -----------------------
  [PASS] 61 nodes validated
  [PASS] 182 arcs validated
  [PASS] Path/content alignment verified

  Layer 2: Mermaid Diagrams
  -------------------------
  [PASS] 11 view diagrams current
  [WARN] 1 diagram needs regeneration (block-generation.mmd)

  Layer 3: TypeScript Types
  -------------------------
  [PASS] node-classes.ts in sync
  [PASS] arc-classes.ts in sync
  [PASS] taxonomy.ts in sync

  Layer 4: Neo4j Seeds
  --------------------
  [PASS] taxonomy.cypher in sync
  [PASS] schema.cypher in sync
  [PASS] arcs.cypher in sync

  Layer 5: Use Cases
  ------------------
  [PASS] Block Generation pipeline valid
  [PASS] Locale Knowledge pipeline valid
  [PASS] SEO Pipeline valid

+===============================================================================+
|  SUMMARY: 4 PASSED, 1 WARNING, 0 FAILED                                       |
+===============================================================================+
```

## Auto-Fix Mode

If issues found:

```bash
cargo run -- schema generate
pnpm build
```

Then re-run audit to verify fixes.

## ADR References

| ADR | Topic |
|-----|-------|
| ADR-003 | YAML-First Architecture |
| ADR-021 | Query-First Architecture |
| ADR-024 | Trait = Data Origin |
| ADR-029 | *Native Pattern |
