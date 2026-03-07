# Entity-Page Architecture Cleanup

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix REPRESENTS arc direction and clean up Entity-Page architecture

**Architecture:** Entity-centric model where Entity is created first and REPRESENTS Page

**Tech Stack:** Neo4j, Cypher migrations, YAML schema, Rust CLI

**Date:** 2026-03-07
**Session:** Brainstorm continuation on Node Relationships
**Version:** v0.17.3

---

## Session Decisions Summary

### Decision 1: REPRESENTS Arc Direction

**BEFORE (WRONG):**
```
Page -[:REPRESENTS]-> Entity
```

**AFTER (CORRECT):**
```
Entity -[:REPRESENTS]-> Page
```

**Rationale:** Entity is created BEFORE Page. Entity is the semantic concept, Page is the structural representation. The Entity "represents" itself as a Page.

### Decision 2: Entity-Page Cardinality

| Rule | Description |
|------|-------------|
| **1 Page = 1 Entity** | Every Page has exactly ONE Entity representing it |
| **Not all Entities have Pages** | Many Entities exist as pure semantic concepts |
| **Entities combine via SEMANTIC_LINK** | Composite pages link to multiple entities |

### Decision 3: Key Pattern (Same Slug, Different Prefix)

```
Entity:       entity:{slug}              → entity:qr-code
Page:         page:{slug}                → page:qr-code
EntityNative: entity:{slug}@{locale}     → entity:qr-code@fr-FR
PageNative:   page:{slug}@{locale}       → page:qr-code@fr-FR
Block:        block:{page-slug}-{type}-{n} → block:qr-code-head-seo-meta-1
BlockNative:  block:{page}:{type}:{n}@{locale} → block:qr-code:head-seo-meta:1@fr-FR
```

### Decision 4: Composite Page Pattern

For a page like "QR Code Instagram Template":

```
entity:qr-code-instagram-template (NEW - represents the page)
    │
    ├─[:REPRESENTS]──► page:qr-code-instagram-template
    │
    ├─[:SEMANTIC_LINK]──► entity:instagram-qr-code
    ├─[:SEMANTIC_LINK]──► entity:template-qr-code
    ├─[:SEMANTIC_LINK]──► entity:qr-code-generator
    └─[:SEMANTIC_LINK]──► entity:social-network-qr
```

The SEMANTIC_LINK arcs provide context for LLM generation.

### Decision 5: HAS_CHILD vs SEMANTIC_LINK

| Arc | Purpose | Direction |
|-----|---------|-----------|
| **HAS_CHILD** | Hierarchical parent/child | Parent → Child |
| **SEMANTIC_LINK** | Horizontal association | Entity → Related Entity |

### Decision 6: Slug Derivation Flow (Confirmed)

```
SEOKeyword (shared/knowledge, imported)
    │ slug_form: "qr-code-generator"
    │
    │ ◄─[:TARGETS]─ EntityNative (is_slug_source: true)
    ▼
EntityNative (org/semantic, authored)
    │ denomination_forms.url: "qr-code-generator"
    │
    │ (copied during generation)
    ▼
BlockNative:head-seo-meta (org/output, generated)
    │ content.slug: "qr-code-generator"
    │ content.meta_title: "..."
    │ content.meta_description: "..."
    │
    │ [:DERIVED_SLUG_FROM] → EntityNative (provenance)
    │ [:SLUGIFIED_BY] → Slugification (locale rules)
    ▼
URL: /en-US/qr-code-generator
```

### Decision 7: BlockNative:head-seo-meta

- Always order=0 on every Page
- Contains: slug, meta_title, meta_description
- Created automatically when Page exists

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         ORG REALM                                           │
│                                                                             │
│   Entity (semantic/defined)                                                 │
│   key: "entity:qr-code"                                                     │
│       │                                                                     │
│       ├─[:REPRESENTS]──► Page (structure/defined)                           │
│       │                  key: "page:qr-code"                                │
│       │                      │                                              │
│       │                      ├─[:HAS_BLOCK]──► Block                        │
│       │                      │                 order: 0 (head-seo-meta)     │
│       │                      │                     │                        │
│       │                      │                     └─[:HAS_NATIVE]──►       │
│       │                      │                       BlockNative            │
│       │                      │                       content: {slug, ...}   │
│       │                      │                                              │
│       │                      └─[:HAS_NATIVE]──► PageNative                  │
│       │                                         [:ASSEMBLES]──► BlockNatives│
│       │                                                                     │
│       ├─[:HAS_NATIVE]──► EntityNative                                       │
│       │                  denomination_forms: {text, title, abbrev, url}     │
│       │                      │                                              │
│       │                      └─[:TARGETS]──► SEOKeyword (shared)            │
│       │                                      is_slug_source: true/false     │
│       │                                                                     │
│       ├─[:SEMANTIC_LINK]──► Entity (related concepts)                       │
│       └─[:HAS_CHILD]──► Entity (child concepts)                             │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Migration Tasks

### Task 1: Invert REPRESENTS Arc Direction

**Files:**
- Create: `brain/seed/migrations/053-invert-represents-direction.cypher`

**Migration:**
```cypher
// Migration 053: Invert REPRESENTS Arc Direction
// BEFORE: (Page)-[:REPRESENTS]->(Entity)
// AFTER:  (Entity)-[:REPRESENTS]->(Page)

// Step 1: Create new arcs with correct direction
MATCH (p:Page)-[old:REPRESENTS]->(e:Entity)
MERGE (e)-[:REPRESENTS]->(p);

// Step 2: Delete old arcs
MATCH (p:Page)-[old:REPRESENTS]->(e:Entity)
DELETE old;
```

### Task 2: Clean Duplicate Page Nodes

**Issue:** Two Page nodes with same content:
- `page:qr-code` (with prefix)
- `qr-code` (without prefix - legacy)

**Migration:**
```cypher
// Migration 054: Clean duplicate Page nodes
// Merge legacy Page nodes into prefixed versions

MATCH (legacy:Page)
WHERE NOT legacy.key STARTS WITH 'page:'
WITH legacy, 'page:' + legacy.key AS new_key
MATCH (prefixed:Page {key: new_key})
// Transfer any unique relationships from legacy to prefixed
// Then delete legacy
DETACH DELETE legacy;
```

### Task 3: Update Schema YAML

**File:** `brain/models/arc-classes/ownership/represents.yaml`

Update source/target:
```yaml
arc:
  name: REPRESENTS
  family: ownership
  scope: intra_realm
  source: Entity  # Was: Page
  target: Page    # Was: Entity
  cardinality: "1:1"
  description: "Entity represents itself as a Page for URL/content generation"
```

### Task 4: Verify and Regenerate

```bash
# 1. Validate schema
cargo run -- schema validate

# 2. Generate artifacts
cargo run -- schema generate

# 3. Reset and reseed
pnpm infra:reset

# 4. Audit
# Use novanet_audit for CSR check
```

---

## Verification Checklist

- [ ] REPRESENTS arc direction: Entity → Page
- [ ] No duplicate Page nodes (legacy without prefix)
- [ ] Schema YAML updated for REPRESENTS
- [ ] TypeScript types regenerated
- [ ] Cypher seeds updated
- [ ] Database reseeded
- [ ] CSR 100% (novanet_audit)
- [ ] All tests pass

---

## Cross-Realm Arc Rules (Reference)

| Direction | Count | Purpose |
|-----------|-------|---------|
| org → shared | 12 | Consuming shared knowledge |
| shared → org | 8 | Querying via inverse arcs |
| config | 2 | Bidirectional |

**Total:** 22 cross-realm arcs

---

## Related ADRs

- **ADR-029:** *Native Pattern (EntityNative, PageNative, BlockNative)
- **ADR-030:** Slug Ownership (Entity semantic, Page structure, BlockNative localized)
- **ADR-033:** Denomination Forms (text, title, abbrev, url)

---

## Session Notes

1. Entity is created BEFORE Page
2. Entity.key and Page.key share the same slug with different prefixes
3. SEMANTIC_LINK allows combining multiple Entities for composite pages
4. BlockNative:head-seo-meta is always order=0 and contains the localized slug
5. Slug flows: SEOKeyword → EntityNative.denomination_forms.url → BlockNative.content.slug
