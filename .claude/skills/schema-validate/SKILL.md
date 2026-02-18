---
name: schema-validate
description: Validate and regenerate NovaNet YAML schema artifacts
---

# NovaNet Schema Validation

## Quick Validation

Run schema validation to check YAML coherence:

```bash
cd tools/novanet && cargo run -- schema validate
```

For strict validation (fails on warnings):

```bash
cargo run -- schema validate --strict
```

## Full Regeneration

If validation passes, regenerate all artifacts from YAML source of truth:

```bash
cargo run -- schema generate
```

This regenerates 12 artifact types:
- TypeScript types
- Cypher seeds
- Mermaid diagrams
- Rust structs
- Neo4j constraints
- View definitions

## Expected Counts (v0.13.1)

| Type | Count |
|------|-------|
| Nodes | 61 (40 shared + 21 org) |
| Arcs | 182 (6 families) |
| Layers | 10 (4 shared + 6 org) |
| Traits | 5 (defined, authored, imported, generated, retrieved) |

## Common Issues

1. **Missing timestamp properties**: Add `created_at` and `updated_at` to standard_properties
2. **Wrong trait**: Check ADR-024 (Trait = Data Origin)
3. **Composite key missing denorm**: Add `entity_key`, `locale_key` for *Native nodes
4. **Path/content mismatch**: YAML `realm:` and `layer:` must match file path
5. **Icon format wrong**: Use `{ web: "lucide-name", terminal: "symbol" }` format

## YAML Location

Node classes: `packages/core/models/node-classes/{realm}/{layer}/{name}.yaml`
Arc classes: `packages/core/models/arc-classes/{family}/{name}.yaml`
Taxonomy: `packages/core/models/taxonomy.yaml`
Views: `packages/core/models/views.yaml`

## Validation Rules (13 rules)

The schema validator enforces:

| Rule | Description |
|------|-------------|
| KEY_REQUIRED | Non-satellite nodes must have `key` |
| DENORM_REQUIRED | Composite keys need denormalized props |
| TIMESTAMP_REQUIRED | All nodes need created_at/updated_at |
| PROP_ORDER | Standard properties in canonical order |
| PATH_CONTENT_MATCH | File path matches YAML realm/layer |
| TRAIT_VALID | Trait is one of 5 valid values |
| ICON_FORMAT | Icon uses web/terminal dual format |
| LLM_CONTEXT_PATTERN | llm_context follows USE/TRIGGERS/NOT/RELATES |
| ... | See schema_rules.rs for full list |

## Critical Path Requirement

**Validation is MANDATORY before committing.** It gates all downstream steps.

| Skip Validation? | Cost |
|------------------|------|
| ❌ Run validate (2 min) | Fast, catches errors locally |
| CI failure | 10+ min waiting for Actions to fail |
| Debug inconsistent types | 15+ min if artifacts diverge |
| Production debugging | Unacceptable |

**Why validation is non-negotiable:**
- **Artifact Regeneration**: TypeScript types, Cypher seeds, and Mermaid diagrams depend on valid YAML. Skipping validation means downstream artifacts silently contain old versions.
- **Trait Changes Are High-Risk**: Changing `trait: authored` → `trait: generated` alters which properties are required (ADR-024). Even one-line changes need full validation.
- **CI Enforcement**: GitHub Actions runs `schema validate --strict` on every PR. Committing invalid YAML wastes time in CI failure loops.

## Workflow

```
1. Edit YAML in packages/core/models/
   ↓
2. cargo run -- schema validate    ← MANDATORY (gates step 4-6)
   ↓
3. Fix any errors/warnings
   ↓
4. cargo run -- schema generate
   ↓
5. cargo run -- db seed (update Neo4j)
   ↓
6. Commit changes
```

**Never skip step 2.** "It's just one line" is how schema corruption starts.

## Related Commands

| Command | Description |
|---------|-------------|
| `cargo run -- schema validate` | Validate YAML coherence |
| `cargo run -- schema generate` | Regenerate artifacts |
| `cargo run -- doc generate` | Generate view diagrams |
| `cargo run -- db seed` | Seed Neo4j from Cypher |
| `cargo run -- tui` | Interactive TUI explorer |
