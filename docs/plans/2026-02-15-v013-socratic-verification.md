# v0.13.0 Socratic Verification Plan

## Questions Fondamentales

### 1. YAML Schema Coherence
- Les 4 *Native nodes ont-ils les bons traits (authored/generated)?
- Les arcs HAS_NATIVE/NATIVE_OF référencent-ils les bons source/target?
- Y a-t-il des références orphelines aux anciens noms (EntityContent, etc.)?

### 2. Cypher Seeds Synchronization
- Les seeds utilisent-ils les nouveaux labels (:EntityNative, :PageNative)?
- Les relationships [:HAS_NATIVE] sont-elles correctement définies?
- Les anciens patterns (HAS_CONTENT, HAS_GENERATED) sont-ils absents?

### 3. TypeScript Types Alignment
- Les types exportés correspondent-ils au YAML?
- Les filtres/queries utilisent-ils la nouvelle nomenclature?
- Les tests couvrent-ils les nouveaux patterns?

### 4. Rust Code Consistency
- Les générateurs produisent-ils du code v0.13.0?
- Le TUI affiche-t-il les bons noms?
- Les snapshots sont-ils à jour?

### 5. Neo4j Live Data
- Le schema Neo4j reflète-t-il v0.13.0?
- Les contraintes existent-elles pour les nouveaux labels?
- Les données de test utilisent-elles *Native?

### 6. Documentation Sync
- ADR-029 est-il complet et cohérent?
- CHANGELOG reflète-t-il tous les changements?
- Les CLAUDE.md sont-ils à jour?

### 7. Semantic Correctness
- Les llm_context des arcs sont-ils corrects?
- Les descriptions des nodes sont-elles précises?
- La génération native (pas traduction) est-elle claire?

### 8. Cross-Reference Integrity
- Tous les arc source/target existent-ils comme nodes?
- Les inverses sont-ils correctement déclarés?
- Les cardinalities sont-elles cohérentes?

### 9. View Definitions
- Les views Cypher utilisent-ils *Native?
- Les views contextuelles fonctionnent-elles?
- Pas de références aux anciens patterns?

### 10. Integration Points
- Studio queries utilisent-elles v0.13.0?
- API routes retournent-elles les bons types?
- Les tests d'intégration passent-ils?

## Agents à Lancer

| # | Agent | Focus | Questions Clés |
|---|-------|-------|----------------|
| 1 | YAML-NODES | Node classes | Traits corrects? Layers corrects? |
| 2 | YAML-ARCS | Arc classes | Source/target valides? Inverses déclarés? |
| 3 | CYPHER-SEEDS | Seed files | Labels v0.13? Relationships? |
| 4 | CYPHER-CONSTRAINTS | Constraints | Nouveaux labels contraints? |
| 5 | TS-TYPES | TypeScript | Types alignés? Exports corrects? |
| 6 | TS-QUERIES | Queries | Cypher v0.13? Filters corrects? |
| 7 | RUST-GEN | Generators | Output v0.13? Snapshots à jour? |
| 8 | RUST-TUI | TUI display | Noms corrects? Help à jour? |
| 9 | DOCS | Documentation | ADR-029 complet? CHANGELOG? |
| 10 | SEMANTIC | Coherence | llm_context? Descriptions? |
