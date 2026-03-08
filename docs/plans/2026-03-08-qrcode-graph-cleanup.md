# QR-Code Graph Cleanup Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix 5 issues in the NovaNet knowledge graph for the qr-code entity to achieve 100% ontology coherence.

**Architecture:** Use MCP tools (novanet_check → novanet_write → novanet_audit) for all mutations. Key normalization via Cypher MERGE for safe renames. SEO keyword import with temperature-weighted TARGETS_KEYWORD arcs.

**Tech Stack:** NovaNet MCP Server, Neo4j, Cypher

---

## Issues Summary

| Issue | Severity | Current State | Target State |
|-------|----------|---------------|--------------|
| 1. EntityNative key format | CRITICAL | `qr-code@de-DE` | `entity:qr-code@de-DE` |
| 2. French denomination_forms | CRITICAL | `"code QR"` | `"QR code"` |
| 3. Missing PageNatives | HIGH | 0 for page:qr-code-landing | 2 (en-US, fr-FR) |
| 4. Missing fr-FR SEOKeywords | HIGH | 0 keywords | 20 top keywords |
| 5. Duplicate Pages cleanup | MEDIUM | 3 Page nodes | 1 canonical |

---

## Phase 1: Fix EntityNative Key Format

### Task 1.1: Rename qr-code@de-DE to entity:qr-code@de-DE

**Step 1: Query current node via Cypher**

Run:
```bash
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword <<'EOF'
MATCH (n:EntityNative {key: 'qr-code@de-DE'})
RETURN n.key, n.display_name, n.locale
EOF
```

Expected: `qr-code@de-DE | QR-Code | de-DE`

**Step 2: Rename key via Cypher MERGE pattern**

Run:
```bash
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword <<'EOF'
// Create new node with correct key
MATCH (old:EntityNative {key: 'qr-code@de-DE'})
MERGE (new:EntityNative {key: 'entity:qr-code@de-DE'})
ON CREATE SET new = old, new.key = 'entity:qr-code@de-DE', new.entity_key = 'qr-code', new.locale_key = 'de-DE'
ON MATCH SET new += old, new.key = 'entity:qr-code@de-DE'
WITH old, new

// Copy incoming relationships
CALL {
  WITH old, new
  MATCH (old)<-[r:HAS_NATIVE]-(parent)
  MERGE (parent)-[:HAS_NATIVE]->(new)
  RETURN count(*) AS incoming
}

// Copy outgoing relationships
CALL {
  WITH old, new
  MATCH (old)-[r:NATIVE_OF]->(parent)
  MERGE (new)-[:NATIVE_OF]->(parent)
  RETURN count(*) AS outgoing
}

// Copy FOR_LOCALE relationship
CALL {
  WITH old, new
  MATCH (old)-[r:FOR_LOCALE]->(locale)
  MERGE (new)-[:FOR_LOCALE]->(locale)
  RETURN count(*) AS locale_arcs
}

// Delete old node and its relationships
DETACH DELETE old
RETURN 'Renamed qr-code@de-DE → entity:qr-code@de-DE' AS result
EOF
```

Expected: `Renamed qr-code@de-DE → entity:qr-code@de-DE`

**Step 3: Verify rename**

Run:
```bash
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword <<'EOF'
MATCH (n:EntityNative) WHERE n.key CONTAINS 'qr-code@de-DE' RETURN n.key
EOF
```

Expected: `entity:qr-code@de-DE`

---

### Task 1.2: Rename qr-code@es-MX to entity:qr-code@es-MX

**Step 1: Rename via Cypher MERGE pattern**

Run:
```bash
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword <<'EOF'
MATCH (old:EntityNative {key: 'qr-code@es-MX'})
MERGE (new:EntityNative {key: 'entity:qr-code@es-MX'})
ON CREATE SET new = old, new.key = 'entity:qr-code@es-MX', new.entity_key = 'qr-code', new.locale_key = 'es-MX'
ON MATCH SET new += old, new.key = 'entity:qr-code@es-MX'
WITH old, new
CALL { WITH old, new MATCH (old)<-[r:HAS_NATIVE]-(parent) MERGE (parent)-[:HAS_NATIVE]->(new) RETURN count(*) AS i }
CALL { WITH old, new MATCH (old)-[r:NATIVE_OF]->(parent) MERGE (new)-[:NATIVE_OF]->(parent) RETURN count(*) AS o }
CALL { WITH old, new MATCH (old)-[r:FOR_LOCALE]->(locale) MERGE (new)-[:FOR_LOCALE]->(locale) RETURN count(*) AS l }
DETACH DELETE old
RETURN 'Renamed qr-code@es-MX → entity:qr-code@es-MX' AS result
EOF
```

Expected: `Renamed qr-code@es-MX → entity:qr-code@es-MX`

---

### Task 1.3: Rename qr-code@ja-JP to entity:qr-code@ja-JP

**Step 1: Rename via Cypher MERGE pattern**

Run:
```bash
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword <<'EOF'
MATCH (old:EntityNative {key: 'qr-code@ja-JP'})
MERGE (new:EntityNative {key: 'entity:qr-code@ja-JP'})
ON CREATE SET new = old, new.key = 'entity:qr-code@ja-JP', new.entity_key = 'qr-code', new.locale_key = 'ja-JP'
ON MATCH SET new += old, new.key = 'entity:qr-code@ja-JP'
WITH old, new
CALL { WITH old, new MATCH (old)<-[r:HAS_NATIVE]-(parent) MERGE (parent)-[:HAS_NATIVE]->(new) RETURN count(*) AS i }
CALL { WITH old, new MATCH (old)-[r:NATIVE_OF]->(parent) MERGE (new)-[:NATIVE_OF]->(parent) RETURN count(*) AS o }
CALL { WITH old, new MATCH (old)-[r:FOR_LOCALE]->(locale) MERGE (new)-[:FOR_LOCALE]->(locale) RETURN count(*) AS l }
DETACH DELETE old
RETURN 'Renamed qr-code@ja-JP → entity:qr-code@ja-JP' AS result
EOF
```

Expected: `Renamed qr-code@ja-JP → entity:qr-code@ja-JP`

---

### Task 1.4: Verify Phase 1 completion

**Step 1: Query all qr-code EntityNatives**

Run:
```bash
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword <<'EOF'
MATCH (n:EntityNative) WHERE n.entity_key = 'qr-code' OR n.key CONTAINS 'qr-code@'
RETURN n.key ORDER BY n.key
EOF
```

Expected: All keys should start with `entity:`
```
entity:qr-code@de-DE
entity:qr-code@en-US
entity:qr-code@es-MX
entity:qr-code@fr-FR
entity:qr-code@ja-JP
```

**Step 2: Commit Phase 1**

Run:
```bash
git add -A && git commit -m "fix(graph): normalize EntityNative keys to entity:{key}@{locale} format

Renamed:
- qr-code@de-DE → entity:qr-code@de-DE
- qr-code@es-MX → entity:qr-code@es-MX
- qr-code@ja-JP → entity:qr-code@ja-JP

Per ADR-029 *Native Pattern key convention.

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>"
```

---

## Phase 2: Fix French denomination_forms

### Task 2.1: Update entity:qr-code@fr-FR denomination_forms

**CRITICAL RULE:** In French we say **"QR code"** NOT "code QR"

**Step 1: Check current denomination_forms**

Run via MCP:
```
novanet_search(query="entity:qr-code@fr-FR", kinds=["EntityNative"])
```

Current (WRONG):
```json
[
  {"type": "text", "value": "code QR", "priority": 1},
  {"type": "title", "value": "Code QR", "priority": 1}
]
```

**Step 2: Validate update via novanet_check**

```json
{
  "operation": "update_props",
  "class": "EntityNative",
  "key": "entity:qr-code@fr-FR",
  "properties": {
    "denomination_forms": "[{\"type\":\"text\",\"value\":\"QR code\",\"priority\":1},{\"type\":\"title\",\"value\":\"QR Code\",\"priority\":1},{\"type\":\"abbrev\",\"value\":\"QR\",\"priority\":1},{\"type\":\"plural\",\"value\":\"QR codes\",\"priority\":5},{\"type\":\"url\",\"value\":\"qr-code\",\"priority\":1}]"
  }
}
```

**Step 3: Apply update via novanet_write**

```json
{
  "operation": "update_props",
  "class": "EntityNative",
  "key": "entity:qr-code@fr-FR",
  "properties": {
    "denomination_forms": "[{\"type\":\"text\",\"value\":\"QR code\",\"priority\":1},{\"type\":\"title\",\"value\":\"QR Code\",\"priority\":1},{\"type\":\"abbrev\",\"value\":\"QR\",\"priority\":1},{\"type\":\"plural\",\"value\":\"QR codes\",\"priority\":5},{\"type\":\"url\",\"value\":\"qr-code\",\"priority\":1}]",
    "display_name": "QR Code"
  }
}
```

Expected: `success: true, updated_properties: ["denomination_forms", "display_name"]`

**Step 4: Verify update**

Run:
```bash
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword <<'EOF'
MATCH (n:EntityNative {key: 'entity:qr-code@fr-FR'})
RETURN n.denomination_forms
EOF
```

Expected: Should contain `"QR code"` NOT `"code QR"`

---

## Phase 3: Create PageNatives for page:qr-code-landing

### Task 3.1: Create PageNative for en-US

**Step 1: Validate via novanet_check**

```json
{
  "operation": "upsert_node",
  "class": "PageNative",
  "key": "page:qr-code-landing@en-US",
  "properties": {
    "display_name": "QR Code Generator",
    "page_key": "page:qr-code-landing",
    "locale_key": "en-US",
    "meta_title": "Free QR Code Generator - Create Custom QR Codes Instantly",
    "meta_description": "Generate free QR codes for URLs, WiFi, vCards, and more. Customize colors, add logos, and download in PNG, SVG, or PDF.",
    "slug": "qr-code",
    "h1": "Free QR Code Generator"
  },
  "locale": "en-US"
}
```

**Step 2: Create via novanet_write**

Use the same params as Step 1.

Expected: `success: true, created: true, auto_arcs_created: ["FOR_LOCALE", "HAS_NATIVE"]`

**Step 3: Create NATIVE_OF inverse arc**

Run:
```bash
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword <<'EOF'
MATCH (p:Page {key: 'page:qr-code-landing'})-[:HAS_NATIVE]->(pn:PageNative {key: 'page:qr-code-landing@en-US'})
WHERE NOT (pn)-[:NATIVE_OF]->(p)
MERGE (pn)-[:NATIVE_OF {created_at: datetime()}]->(p)
RETURN 'Created NATIVE_OF arc for page:qr-code-landing@en-US' AS result
EOF
```

---

### Task 3.2: Create PageNative for fr-FR

**Step 1: Create via novanet_write**

```json
{
  "operation": "upsert_node",
  "class": "PageNative",
  "key": "page:qr-code-landing@fr-FR",
  "properties": {
    "display_name": "Générateur de QR Code",
    "page_key": "page:qr-code-landing",
    "locale_key": "fr-FR",
    "meta_title": "Générateur de QR Code Gratuit - Créez des QR Codes Personnalisés",
    "meta_description": "Générez des QR codes gratuits pour URL, WiFi, vCard et plus. Personnalisez les couleurs, ajoutez des logos, téléchargez en PNG, SVG ou PDF.",
    "slug": "qr-code",
    "h1": "Générateur de QR Code Gratuit"
  },
  "locale": "fr-FR"
}
```

**Note:** We use "QR code" (not "code QR") in French!

**Step 2: Create NATIVE_OF inverse arc**

Run:
```bash
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword <<'EOF'
MATCH (p:Page {key: 'page:qr-code-landing'})-[:HAS_NATIVE]->(pn:PageNative {key: 'page:qr-code-landing@fr-FR'})
WHERE NOT (pn)-[:NATIVE_OF]->(p)
MERGE (pn)-[:NATIVE_OF {created_at: datetime()}]->(p)
RETURN 'Created NATIVE_OF arc for page:qr-code-landing@fr-FR' AS result
EOF
```

---

### Task 3.3: Verify PageNatives and commit Phase 2-3

**Step 1: Verify PageNatives exist**

Run:
```bash
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword <<'EOF'
MATCH (p:Page {key: 'page:qr-code-landing'})-[:HAS_NATIVE]->(pn:PageNative)
RETURN pn.key, pn.locale_key, pn.meta_title
EOF
```

Expected:
```
page:qr-code-landing@en-US | en-US | Free QR Code Generator...
page:qr-code-landing@fr-FR | fr-FR | Générateur de QR Code Gratuit...
```

**Step 2: Commit Phase 2-3**

Run:
```bash
git add -A && git commit -m "fix(graph): fix French denomination_forms and create PageNatives

- Fixed entity:qr-code@fr-FR: 'code QR' → 'QR code'
- Created page:qr-code-landing@en-US PageNative
- Created page:qr-code-landing@fr-FR PageNative
- Created NATIVE_OF inverse arcs

CRITICAL: In French we say 'QR code' NOT 'code QR'!

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>"
```

---

## Phase 4: Import fr-FR SEO Keywords

### Task 4.1: Parse top 20 keywords from CSV

**Source:** `/Users/thibaut/Desktop/seo-keyword-qr-code/fr-FR/google_fr_qr_matching-terms_2026-03-07_12-03-44.csv`

**Top 20 keywords to import (by volume):**

| Rank | Keyword | Volume | Difficulty | Intent | Temperature |
|------|---------|--------|------------|--------|-------------|
| 1 | qr code | 109,000 | 54 | Informational | 0.95 |
| 2 | qr code generator | 50,000 | 91 | Informational | 0.90 |
| 3 | qr code gratuit | 31,000 | 35 | Informational | 0.92 |
| 4 | comment scanner un qr code | 17,000 | 1 | Informational | 0.75 |
| 5 | scanner qr code | 17,000 | 3 | Informational | 0.78 |
| 6 | créer un qr code | 14,000 | 33 | Informational | 0.88 |
| 7 | générer un qr code | 9,300 | 39 | Informational | 0.87 |
| 8 | générer qr code | 7,800 | 40 | Informational | 0.86 |
| 9 | scan qr code | 7,500 | 2 | Informational | 0.76 |
| 10 | créer qr code | 7,400 | 40 | Informational | 0.85 |
| 11 | générateur de qr code | 7,000 | 36 | Informational | 0.90 |
| 12 | qr code scanner | 6,800 | 3 | Informational | 0.77 |
| 13 | creer qr code | 6,500 | 35 | Informational | 0.84 |
| 14 | creer un qr code | 6,100 | 39 | Informational | 0.83 |
| 15 | création qr code | 5,600 | 35 | Informational | 0.82 |
| 16 | générateur qr code | 5,500 | 38 | Informational | 0.89 |
| 17 | qr | 5,400 | 28 | Informational | 0.70 |
| 18 | qr code monkey | 5,200 | 0 | Branded | 0.60 |
| 19 | generateur qr code | 5,100 | 40 | Informational | 0.88 |
| 20 | créer un qr code gratuitement | 4,300 | 22 | Informational | 0.91 |

**Temperature formula:**
- Primary keywords (exact match entity): 0.95
- Generator keywords: 0.88-0.92
- Scanner/reader keywords: 0.75-0.78
- Branded keywords: 0.60

---

### Task 4.2: Create SEOKeyword nodes via batch

**Step 1: Create seo:qr-code@fr-FR (PRIMARY)**

```json
{
  "operation": "upsert_node",
  "class": "SEOKeyword",
  "key": "seo:qr-code@fr-FR",
  "properties": {
    "display_name": "qr code",
    "value": "qr code",
    "locale_key": "fr-FR",
    "search_volume": 109000,
    "difficulty": 54,
    "cpc": 0.20,
    "intent": "Informational",
    "source": "ahrefs",
    "source_date": "2026-03-07",
    "description": "SEO keyword for fr-FR: qr code"
  },
  "locale": "fr-FR"
}
```

**Step 2: Create remaining keywords (batch via loop)**

For each keyword, use novanet_write with:
- `key`: `seo:{slug}@fr-FR` (where slug = keyword with spaces → hyphens)
- `value`: original keyword text
- `search_volume`, `difficulty`, `intent` from CSV

---

### Task 4.3: Create TARGETS_KEYWORD arcs with temperatures

**Arc direction:** `(EntityNative)-[:TARGETS_KEYWORD]->(SEOKeyword)`

**Step 1: Create arc for primary keyword**

```json
{
  "operation": "create_arc",
  "arc_class": "TARGETS_KEYWORD",
  "from_key": "entity:qr-code@fr-FR",
  "to_key": "seo:qr-code@fr-FR",
  "properties": {
    "temperature": 0.95,
    "rank": "primary",
    "is_slug_source": true
  }
}
```

**Step 2: Create arcs for generator keywords (temperature 0.88-0.92)**

Keywords: `qr code generator`, `qr code gratuit`, `créer un qr code`, etc.

```json
{
  "operation": "create_arc",
  "arc_class": "TARGETS_KEYWORD",
  "from_key": "entity:qr-code@fr-FR",
  "to_key": "seo:qr-code-generator@fr-FR",
  "properties": {
    "temperature": 0.90,
    "rank": "secondary"
  }
}
```

**Step 3: Create arcs for scanner keywords (temperature 0.75-0.78)**

Keywords: `scanner qr code`, `scan qr code`, `qr code scanner`

**Step 4: Skip branded keyword (qr code monkey) - different entity**

---

### Task 4.4: Verify SEO imports

**Step 1: Count fr-FR keywords**

Run:
```bash
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword <<'EOF'
MATCH (s:SEOKeyword) WHERE s.locale_key = 'fr-FR'
RETURN count(s) AS fr_fr_keywords
EOF
```

Expected: `19` (20 minus the branded one)

**Step 2: Verify TARGETS_KEYWORD arcs with temperatures**

Run:
```bash
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword <<'EOF'
MATCH (en:EntityNative {key: 'entity:qr-code@fr-FR'})-[r:TARGETS_KEYWORD]->(s:SEOKeyword)
RETURN s.value, r.temperature, r.rank
ORDER BY r.temperature DESC
EOF
```

Expected: 19 rows with temperatures 0.70-0.95

---

### Task 4.5: Commit Phase 4

Run:
```bash
git add -A && git commit -m "feat(seo): import 19 fr-FR SEO keywords with temperature-weighted arcs

Top keywords imported from Ahrefs CSV:
- qr code (109K volume, temp 0.95, PRIMARY)
- qr code generator (50K, temp 0.90)
- qr code gratuit (31K, temp 0.92)
- ...and 16 more

TARGETS_KEYWORD arc temperatures:
- Primary: 0.95
- Generator: 0.88-0.92
- Scanner: 0.75-0.78
- Excluded: qr code monkey (branded)

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>"
```

---

## Phase 5: Final Audit and Cleanup

### Task 5.1: Run full novanet_audit

```json
{
  "target": "all",
  "limit": 100
}
```

Expected: CSR = 1.0 (100%), 0 issues

### Task 5.2: Cleanup duplicate Pages (if needed)

**Step 1: Check for duplicate Pages**

Run:
```bash
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword <<'EOF'
MATCH (p:Page) WHERE p.key CONTAINS 'qr-code'
RETURN p.key, p.display_name, p.path
EOF
```

**Step 2: If duplicates exist, keep only `page:qr-code-landing`**

Run (only if duplicates found):
```bash
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword <<'EOF'
// Delete Pages without the 'page:' prefix that are duplicates
MATCH (p:Page) WHERE p.key IN ['qr-code'] AND EXISTS { MATCH (:Page {key: 'page:qr-code-landing'}) }
DETACH DELETE p
RETURN 'Cleaned up duplicate Page nodes' AS result
EOF
```

### Task 5.3: Final verification

**Step 1: Graph state summary**

Run:
```bash
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword <<'EOF'
// Count by type
MATCH (en:EntityNative) WHERE en.entity_key = 'qr-code' RETURN 'EntityNative' AS type, count(en) AS count
UNION ALL
MATCH (pn:PageNative) WHERE pn.page_key = 'page:qr-code-landing' RETURN 'PageNative' AS type, count(pn) AS count
UNION ALL
MATCH (s:SEOKeyword) WHERE s.locale_key = 'fr-FR' RETURN 'SEOKeyword (fr-FR)' AS type, count(s) AS count
UNION ALL
MATCH (en:EntityNative {key: 'entity:qr-code@fr-FR'})-[r:TARGETS_KEYWORD]->() RETURN 'TARGETS_KEYWORD arcs' AS type, count(r) AS count
EOF
```

Expected:
```
EntityNative        | 5
PageNative          | 2
SEOKeyword (fr-FR)  | 19
TARGETS_KEYWORD     | 19
```

### Task 5.4: Final commit and push

Run:
```bash
git add -A && git commit -m "chore(graph): final cleanup and audit verification

- CSR 100%
- All EntityNative keys normalized
- French denomination_forms corrected
- PageNatives created for en-US and fr-FR
- 19 fr-FR SEO keywords imported
- All TARGETS_KEYWORD arcs have temperatures

Graph state:
- 5 EntityNatives for qr-code entity
- 2 PageNatives for page:qr-code-landing
- 19 fr-FR SEOKeywords with TARGETS_KEYWORD arcs

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>"

git push origin main
```

---

## Verification Checklist

- [ ] All EntityNative keys start with `entity:`
- [ ] `entity:qr-code@fr-FR` uses "QR code" not "code QR"
- [ ] `page:qr-code-landing@en-US` exists with meta_title
- [ ] `page:qr-code-landing@fr-FR` exists with meta_title
- [ ] 19 fr-FR SEOKeywords exist
- [ ] All TARGETS_KEYWORD arcs have temperature property
- [ ] NATIVE_OF inverse arcs exist for all *Native nodes
- [ ] novanet_audit returns CSR = 1.0
- [ ] No duplicate Page nodes

---

## Rollback Plan

If issues occur, restore from git:

```bash
git log --oneline -5  # Find commit before changes
git revert <commit>   # Revert specific commit
```

Or reset database:
```bash
pnpm infra:reset
```
