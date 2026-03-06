# Chantier 1: NovaNet DB Cleanup - CSR 100%

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix 33 critical issues by creating missing parent nodes and arcs (NOT deleting orphans) to achieve CSR 100%

**Architecture:** Create Entity/Page/Block parents for orphaned *Native nodes, then link with HAS_NATIVE arcs. Create FOR_LOCALE arcs for SEOKeywords.

**Tech Stack:** NovaNet MCP (novanet_write, novanet_check, novanet_audit), Cypher migrations, Neo4j

---

## Context

### Current State (from novanet_audit)
- **CSR**: 99.95% (33 issues)
- **11 SEOKeywords** missing FOR_LOCALE arcs
- **5 PageNatives** orphaned (no parent Page)
- **5 BlockNatives** orphaned (no parent Block)
- **13 EntityNatives** orphaned (no parent Entity)

### Orphaned Nodes Detail

**PageNatives** (parent: `page:qr-code` missing):
- `page:qr-code@en-US`
- `page:qr-code@fr-FR`
- `page:qr-code@es-MX`
- `page:qr-code@ja-JP`
- `page:qr-code@de-DE`

**BlockNatives** (parent: `block:qr-code:head-seo-meta:1` missing):
- `block:qr-code:head-seo-meta:1@en-US`
- `block:qr-code:head-seo-meta:1@fr-FR`
- `block:qr-code:head-seo-meta:1@es-MX`
- `block:qr-code:head-seo-meta:1@ja-JP`
- `block:qr-code:head-seo-meta:1@de-DE`

**EntityNatives** (missing parents):
| Key | Missing Entity |
|-----|----------------|
| `entity:qr-code@fr-FR` | `entity:qr-code` |
| `entity:barcode@fr-FR` | `entity:barcode` |
| `entity:custom-qr-code@en-US/fr-FR` | `entity:custom-qr-code` |
| `entity:dynamic-qr-code@en-US/fr-FR` | `entity:dynamic-qr-code` |
| `entity:static-qr-code@en-US/fr-FR` | `entity:static-qr-code` |
| `entity:qr-code-generator@en-US/fr-FR` | `entity:qr-code-generator` |
| `entity:qr-code-art@fr-FR` | `entity:qr-code-art` |
| `entity:landing-page@fr-FR` | `entity:landing-page` |
| `entity:smart-link@fr-FR` | `entity:smart-link` |

---

## Task 1: Validate Current State with novanet_audit

**Files:** None (MCP tool only)

**Step 1: Run full audit**

```bash
# Using MCP tool via Claude Code
novanet_audit target=all
```

**Expected:** 33 issues, CSR 99.95%

**Step 2: Document baseline**

Save output to verify we fixed everything at the end.

---

## Task 2: Create Entity Parents (9 entities)

**Files:**
- Create: `brain/seed/migrations/029-create-entity-parents.cypher`

**Step 1: Use novanet_check to validate Entity schema**

```
novanet_check operation=upsert_node class=Entity key=entity:qr-code properties={...}
```

**Step 2: Write migration file**

```cypher
// ═══════════════════════════════════════════════════════════════════════════════
// MIGRATION 029: Create Entity Parents for Orphaned EntityNatives
// Date: 2026-03-06
// Version: NovaNet v0.17.0
// Purpose: Create 9 Entity nodes to parent existing EntityNative orphans
// ═══════════════════════════════════════════════════════════════════════════════

// Entity: qr-code (core concept)
MERGE (e:Entity {key: 'entity:qr-code'})
SET e.display_name = 'QR Code',
    e.description = 'Two-dimensional barcode that stores information in a machine-readable optical pattern',
    e.denomination_forms = {
      text: 'QR code',
      title: 'QR Code',
      abbrev: 'QR',
      url: 'qr-code'
    },
    e.created_at = datetime(),
    e.updated_at = datetime();

// Entity: barcode
MERGE (e:Entity {key: 'entity:barcode'})
SET e.display_name = 'Barcode',
    e.description = 'Machine-readable representation of data in visual format',
    e.denomination_forms = {
      text: 'barcode',
      title: 'Barcode',
      abbrev: 'barcode',
      url: 'barcode'
    },
    e.created_at = datetime(),
    e.updated_at = datetime();

// Entity: custom-qr-code
MERGE (e:Entity {key: 'entity:custom-qr-code'})
SET e.display_name = 'Custom QR Code',
    e.description = 'QR code with customized design elements like colors, logos, and shapes',
    e.denomination_forms = {
      text: 'custom QR code',
      title: 'Custom QR Code',
      abbrev: 'custom QR',
      url: 'custom-qr-code'
    },
    e.created_at = datetime(),
    e.updated_at = datetime();

// Entity: dynamic-qr-code
MERGE (e:Entity {key: 'entity:dynamic-qr-code'})
SET e.display_name = 'Dynamic QR Code',
    e.description = 'QR code with editable destination URL that can be changed after printing',
    e.denomination_forms = {
      text: 'dynamic QR code',
      title: 'Dynamic QR Code',
      abbrev: 'dynamic QR',
      url: 'dynamic-qr-code'
    },
    e.created_at = datetime(),
    e.updated_at = datetime();

// Entity: static-qr-code
MERGE (e:Entity {key: 'entity:static-qr-code'})
SET e.display_name = 'Static QR Code',
    e.description = 'QR code with fixed destination URL that cannot be changed after creation',
    e.denomination_forms = {
      text: 'static QR code',
      title: 'Static QR Code',
      abbrev: 'static QR',
      url: 'static-qr-code'
    },
    e.created_at = datetime(),
    e.updated_at = datetime();

// Entity: qr-code-generator
MERGE (e:Entity {key: 'entity:qr-code-generator'})
SET e.display_name = 'QR Code Generator',
    e.description = 'Tool or software that creates QR codes from input data',
    e.denomination_forms = {
      text: 'QR code generator',
      title: 'QR Code Generator',
      abbrev: 'QR generator',
      url: 'qr-code-generator'
    },
    e.created_at = datetime(),
    e.updated_at = datetime();

// Entity: qr-code-art
MERGE (e:Entity {key: 'entity:qr-code-art'})
SET e.display_name = 'QR Code Art',
    e.description = 'Artistic QR codes that integrate visual design while maintaining scannability',
    e.denomination_forms = {
      text: 'QR code art',
      title: 'QR Code Art',
      abbrev: 'QR art',
      url: 'qr-code-art'
    },
    e.created_at = datetime(),
    e.updated_at = datetime();

// Entity: landing-page
MERGE (e:Entity {key: 'entity:landing-page'})
SET e.display_name = 'Landing Page',
    e.description = 'Standalone web page designed for a specific marketing or advertising campaign',
    e.denomination_forms = {
      text: 'landing page',
      title: 'Landing Page',
      abbrev: 'LP',
      url: 'landing-page'
    },
    e.created_at = datetime(),
    e.updated_at = datetime();

// Entity: smart-link
MERGE (e:Entity {key: 'entity:smart-link'})
SET e.display_name = 'Smart Link',
    e.description = 'Intelligent URL that can redirect based on device, location, or other parameters',
    e.denomination_forms = {
      text: 'smart link',
      title: 'Smart Link',
      abbrev: 'smart link',
      url: 'smart-link'
    },
    e.created_at = datetime(),
    e.updated_at = datetime();
```

**Step 3: Run migration**

```bash
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword < brain/seed/migrations/029-create-entity-parents.cypher
```

**Step 4: Verify with novanet_traverse**

```
novanet_traverse start_key=entity:qr-code direction=outgoing arc_families=["localization"]
```

---

## Task 3: Create HAS_NATIVE Arcs for Entities

**Files:**
- Create: `brain/seed/migrations/030-entity-has-native-arcs.cypher`

**Step 1: Write migration**

```cypher
// ═══════════════════════════════════════════════════════════════════════════════
// MIGRATION 030: Create HAS_NATIVE Arcs for Entity → EntityNative
// ═══════════════════════════════════════════════════════════════════════════════

// qr-code → EntityNatives
MATCH (e:Entity {key: 'entity:qr-code'})
MATCH (en:EntityNative {key: 'entity:qr-code@fr-FR'})
MERGE (e)-[:HAS_NATIVE]->(en);

// barcode → EntityNatives
MATCH (e:Entity {key: 'entity:barcode'})
MATCH (en:EntityNative {key: 'entity:barcode@fr-FR'})
MERGE (e)-[:HAS_NATIVE]->(en);

// custom-qr-code → EntityNatives
MATCH (e:Entity {key: 'entity:custom-qr-code'})
MATCH (en:EntityNative {key: 'entity:custom-qr-code@en-US'})
MERGE (e)-[:HAS_NATIVE]->(en);

MATCH (e:Entity {key: 'entity:custom-qr-code'})
MATCH (en:EntityNative {key: 'entity:custom-qr-code@fr-FR'})
MERGE (e)-[:HAS_NATIVE]->(en);

// dynamic-qr-code → EntityNatives
MATCH (e:Entity {key: 'entity:dynamic-qr-code'})
MATCH (en:EntityNative {key: 'entity:dynamic-qr-code@en-US'})
MERGE (e)-[:HAS_NATIVE]->(en);

MATCH (e:Entity {key: 'entity:dynamic-qr-code'})
MATCH (en:EntityNative {key: 'entity:dynamic-qr-code@fr-FR'})
MERGE (e)-[:HAS_NATIVE]->(en);

// static-qr-code → EntityNatives
MATCH (e:Entity {key: 'entity:static-qr-code'})
MATCH (en:EntityNative {key: 'entity:static-qr-code@en-US'})
MERGE (e)-[:HAS_NATIVE]->(en);

MATCH (e:Entity {key: 'entity:static-qr-code'})
MATCH (en:EntityNative {key: 'entity:static-qr-code@fr-FR'})
MERGE (e)-[:HAS_NATIVE]->(en);

// qr-code-generator → EntityNatives
MATCH (e:Entity {key: 'entity:qr-code-generator'})
MATCH (en:EntityNative {key: 'entity:qr-code-generator@en-US'})
MERGE (e)-[:HAS_NATIVE]->(en);

MATCH (e:Entity {key: 'entity:qr-code-generator'})
MATCH (en:EntityNative {key: 'entity:qr-code-generator@fr-FR'})
MERGE (e)-[:HAS_NATIVE]->(en);

// qr-code-art → EntityNatives
MATCH (e:Entity {key: 'entity:qr-code-art'})
MATCH (en:EntityNative {key: 'entity:qr-code-art@fr-FR'})
MERGE (e)-[:HAS_NATIVE]->(en);

// landing-page → EntityNatives
MATCH (e:Entity {key: 'entity:landing-page'})
MATCH (en:EntityNative {key: 'entity:landing-page@fr-FR'})
MERGE (e)-[:HAS_NATIVE]->(en);

// smart-link → EntityNatives
MATCH (e:Entity {key: 'entity:smart-link'})
MATCH (en:EntityNative {key: 'entity:smart-link@fr-FR'})
MERGE (e)-[:HAS_NATIVE]->(en);
```

**Step 2: Run and verify**

```bash
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword < brain/seed/migrations/030-entity-has-native-arcs.cypher
```

---

## Task 4: Create Page Parent

**Files:**
- Create: `brain/seed/migrations/031-create-page-qr-code.cypher`

**Step 1: Write migration**

```cypher
// ═══════════════════════════════════════════════════════════════════════════════
// MIGRATION 031: Create Page:qr-code and Link to Entity
// ═══════════════════════════════════════════════════════════════════════════════

// Create Page
MERGE (p:Page {key: 'page:qr-code'})
SET p.display_name = 'QR Code Generator',
    p.description = 'Create custom QR codes for any purpose',
    p.llm_context = 'USE: as the main landing page for QR code creation. TRIGGERS: homepage, main page, generator page. NOT: for specific QR code types (use subpages).',
    p.created_at = datetime(),
    p.updated_at = datetime();

// Link Page → Entity (REPRESENTS)
MATCH (p:Page {key: 'page:qr-code'})
MATCH (e:Entity {key: 'entity:qr-code'})
MERGE (p)-[:REPRESENTS]->(e);

// Link Page → PageNatives (HAS_NATIVE)
MATCH (p:Page {key: 'page:qr-code'})
MATCH (pn:PageNative) WHERE pn.key STARTS WITH 'page:qr-code@'
MERGE (p)-[:HAS_NATIVE]->(pn);
```

---

## Task 5: Create Block Parent

**Files:**
- Create: `brain/seed/migrations/032-create-block-head-seo.cypher`

**Step 1: Write migration**

```cypher
// ═══════════════════════════════════════════════════════════════════════════════
// MIGRATION 032: Create Block:qr-code:head-seo-meta:1
// ═══════════════════════════════════════════════════════════════════════════════

// Create Block
MERGE (b:Block {key: 'block:qr-code:head-seo-meta:1'})
SET b.display_name = 'QR Code SEO Meta',
    b.description = 'SEO metadata block for QR code page',
    b.llm_context = 'USE: for meta title, description, keywords. TRIGGERS: seo, meta, head. NOT: for visible content.',
    b.anchor_id = 'head-seo-meta',
    b.created_at = datetime(),
    b.updated_at = datetime();

// Link Block → Page (belongs to)
MATCH (b:Block {key: 'block:qr-code:head-seo-meta:1'})
MATCH (p:Page {key: 'page:qr-code'})
MERGE (p)-[:HAS_BLOCK]->(b);

// Link Block → BlockNatives (HAS_NATIVE)
MATCH (b:Block {key: 'block:qr-code:head-seo-meta:1'})
MATCH (bn:BlockNative) WHERE bn.key STARTS WITH 'block:qr-code:head-seo-meta:1@'
MERGE (b)-[:HAS_NATIVE]->(bn);

// Link Block → BlockType
MATCH (b:Block {key: 'block:qr-code:head-seo-meta:1'})
MATCH (bt:BlockType {key: 'block-type:head-seo-meta'})
MERGE (b)-[:OF_TYPE]->(bt);
```

---

## Task 6: Create FOR_LOCALE Arcs for SEOKeywords

**Files:**
- Create: `brain/seed/migrations/033-seo-for-locale-arcs.cypher`

**Step 1: Write migration**

```cypher
// ═══════════════════════════════════════════════════════════════════════════════
// MIGRATION 033: Create FOR_LOCALE Arcs for SEOKeywords
// ═══════════════════════════════════════════════════════════════════════════════

// English SEOKeywords
MATCH (k:SEOKeyword {key: 'seo:qr-code@en-US'})
MATCH (l:Locale {key: 'en-US'})
MERGE (k)-[:FOR_LOCALE]->(l);

MATCH (k:SEOKeyword {key: 'seo:qr-code-generator@en-US'})
MATCH (l:Locale {key: 'en-US'})
MERGE (k)-[:FOR_LOCALE]->(l);

// French SEOKeywords
MATCH (k:SEOKeyword {key: 'seo:qr-code@fr-FR'})
MATCH (l:Locale {key: 'fr-FR'})
MERGE (k)-[:FOR_LOCALE]->(l);

MATCH (k:SEOKeyword {key: 'seo:generateur-qr-code@fr-FR'})
MATCH (l:Locale {key: 'fr-FR'})
MERGE (k)-[:FOR_LOCALE]->(l);

MATCH (k:SEOKeyword {key: 'seo:creer-un-qr-code@fr-FR'})
MATCH (l:Locale {key: 'fr-FR'})
MERGE (k)-[:FOR_LOCALE]->(l);

// Spanish (Mexican) SEOKeywords
MATCH (k:SEOKeyword {key: 'seo:codigo-qr@es-MX'})
MATCH (l:Locale {key: 'es-MX'})
MERGE (k)-[:FOR_LOCALE]->(l);

MATCH (k:SEOKeyword {key: 'seo:crear-codigo-qr@es-MX'})
MATCH (l:Locale {key: 'es-MX'})
MERGE (k)-[:FOR_LOCALE]->(l);

MATCH (k:SEOKeyword {key: 'seo:generador-codigo-qr@es-MX'})
MATCH (l:Locale {key: 'es-MX'})
MERGE (k)-[:FOR_LOCALE]->(l);

MATCH (k:SEOKeyword {key: 'seo:generador-de-qr@es-MX'})
MATCH (l:Locale {key: 'es-MX'})
MERGE (k)-[:FOR_LOCALE]->(l);

// German SEOKeyword
MATCH (k:SEOKeyword {key: 'seo:qr-code-erstellen@de-DE'})
MATCH (l:Locale {key: 'de-DE'})
MERGE (k)-[:FOR_LOCALE]->(l);

// Japanese SEOKeyword
MATCH (k:SEOKeyword {key: 'seo:qr-code-sakusei@ja-JP'})
MATCH (l:Locale {key: 'ja-JP'})
MERGE (k)-[:FOR_LOCALE]->(l);
```

---

## Task 7: Run All Migrations

**Step 1: Execute in order**

```bash
cd /Users/thibaut/dev/supernovae/brain

# Run each migration
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword < seed/migrations/029-create-entity-parents.cypher
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword < seed/migrations/030-entity-has-native-arcs.cypher
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword < seed/migrations/031-create-page-qr-code.cypher
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword < seed/migrations/032-create-block-head-seo.cypher
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword < seed/migrations/033-seo-for-locale-arcs.cypher
```

---

## Task 8: Final Verification

**Step 1: Run novanet_audit**

```
novanet_audit target=all
```

**Expected:** CSR 100%, 0 issues

**Step 2: Verify with novanet_traverse**

```
novanet_traverse start_key=entity:qr-code direction=both max_depth=3
```

**Step 3: Verify with novanet_query**

```cypher
// Count orphaned EntityNatives (should be 0)
MATCH (en:EntityNative)
WHERE NOT ()-[:HAS_NATIVE]->(en)
RETURN count(en) AS orphaned_entity_natives;

// Count orphaned PageNatives (should be 0)
MATCH (pn:PageNative)
WHERE NOT ()-[:HAS_NATIVE]->(pn)
RETURN count(pn) AS orphaned_page_natives;

// Count orphaned BlockNatives (should be 0)
MATCH (bn:BlockNative)
WHERE NOT ()-[:HAS_NATIVE]->(bn)
RETURN count(bn) AS orphaned_block_natives;

// Count SEOKeywords without FOR_LOCALE (should be 0)
MATCH (k:SEOKeyword)
WHERE NOT (k)-[:FOR_LOCALE]->()
RETURN count(k) AS seo_without_locale;
```

---

## Task 9: Commit and Push

**Step 1: Stage and commit**

```bash
cd /Users/thibaut/dev/supernovae/brain
git add seed/migrations/029-create-entity-parents.cypher
git add seed/migrations/030-entity-has-native-arcs.cypher
git add seed/migrations/031-create-page-qr-code.cypher
git add seed/migrations/032-create-block-head-seo.cypher
git add seed/migrations/033-seo-for-locale-arcs.cypher
git commit -m "feat(db): create parent nodes for orphaned natives (CSR 100%)

- Create 9 Entity parents for orphaned EntityNatives
- Create Page:qr-code for orphaned PageNatives
- Create Block:qr-code:head-seo-meta:1 for orphaned BlockNatives
- Create HAS_NATIVE arcs to link parents with natives
- Create FOR_LOCALE arcs for 11 SEOKeywords

CSR: 99.95% → 100%

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>"
```

**Step 2: Push**

```bash
git push origin main
```

---

## Success Criteria

- [ ] CSR = 100%
- [ ] 0 orphaned EntityNatives
- [ ] 0 orphaned PageNatives
- [ ] 0 orphaned BlockNatives
- [ ] 0 SEOKeywords without FOR_LOCALE
- [ ] All migrations committed and pushed
