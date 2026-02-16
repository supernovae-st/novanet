# NovaNet Agents

Specialized subagents for complex tasks. See [README.md](./README.md) for overview.

---

## `neo4j-architect`

**Model:** sonnet
**Tools:** Read, Grep, Glob, Neo4j MCP

**Specialization:**
- Graph schema design for AI context
- v0.13.0 Schema-Graph navigation (Realm/Layer/Class/Trait/ArcFamily)
- Efficient Cypher queries (data + schema-graph)
- Performance optimization
- Spreading activation patterns

**Key Cypher patterns:**

```cypher
-- Navigate schema-graph taxonomy
MATCH (r:Realm {key: $realm})-[:HAS_LAYER]->(l:Layer)-[:HAS_CLASS]->(c:Class)
RETURN r.key AS realm, l.key AS layer, collect(c.label) AS classes

-- Full Class context assembly
MATCH (c:Class {label: $classLabel})
MATCH (c)-[:IN_REALM]->(r:Realm)
MATCH (c)-[:IN_LAYER]->(l:Layer)
MATCH (c)-[:HAS_TRAIT]->(t:Trait)
RETURN c.label, c.schema_hint, r.key AS realm, l.key AS layer, t.key AS trait

-- Spreading activation
MATCH (e:Entity {key: $key})-[r:SEMANTIC_LINK*1..2]->(related)
WHERE ALL(rel IN r WHERE rel.temperature >= 0.3)
WITH related, reduce(a = 1.0, rel IN r | a * rel.temperature) AS activation
WHERE activation >= 0.3
RETURN related.key, activation ORDER BY activation DESC
```

---

## `code-reviewer`

**Model:** sonnet
**Tools:** Read, Grep, Glob

**Review focus (7 areas):**

1. **Code Quality (TypeScript)** - Best practices, naming, error handling, no `any`
2. **Code Quality (Rust)** - Ownership, `thiserror`/`color-eyre`, no `.unwrap()`, clippy
3. **Security** - Credentials, injection, XSS
4. **NovaNet Conventions** - Generation NOT translation, imports
5. **v0.13.0 Schema-Graph Conventions** - Realm/Layer/Class terminology, Graph/Nexus modes
6. **Rust-First Architecture** - Single `novanet` binary, TS limited to Studio + types
7. **Testing** - Coverage, edge cases, mocks

**Output format:**

```markdown
## Summary
[Overview]

## Issues Found
### Critical
### Warnings
### Suggestions

## Approval Status
[ ] Approved / [ ] Changes requested
```
