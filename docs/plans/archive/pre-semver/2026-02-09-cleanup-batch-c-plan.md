# Cleanup Batch C: EntityContent Fixes + Keyword Pruning + Legacy L10n Removal

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development to execute tasks.

**Goal:** Fix EntityContent data quality issues, prune low-volume SEO keywords, remove legacy L10n terminology

**Architecture:** Direct Cypher edits + seed file updates + schema cleanup

**Tech Stack:** Cypher (Neo4j), Python/sed (bulk edits), Rust (validation)

---

## Phase C.1: Fix Description Mismatches (9 entities)

**Agent:** `general-purpose`

**Files:**
- Modify: `packages/db/seed/11-entity-content-fr-fr.cypher`

**Entities to fix:**

| entity_key | Line | Current (WRONG) | Should Be |
|-----------|------|-----------------|-----------|
| apple | 3426 | "Solutions QR Code pour Instagram" | "Solutions QR Code pour l'écosystème Apple" |
| google | 3490 | "Solutions QR Code pour paiements PayPal" | "Solutions QR Code pour Google Maps et Reviews" |
| qr-code-paypal | 1122 | "Lien vers page ou profil Facebook" | "Lien de paiement PayPal sécurisé" |
| qr-code-soundcloud | 1058 | "Lien direct vers profil Instagram" | "Lien vers piste ou profil SoundCloud" |
| qr-code-social | 850 | "Accès direct à document PDF" | "Liens vers profils réseaux sociaux" |
| qr-code-image-gallery | 818 | "Accès direct à document PDF" | "Galerie d'images accessible par QR Code" |
| transportation | 2994 | "Solutions QR Code pour restaurants" | "Solutions QR Code pour transport et mobilité" |
| marketing-agencies | 3090 | "QR Codes pour industrie : traçabilité" | "Solutions QR Code pour agences marketing" |
| soundcloud | 3442 | "Créer un lien intelligent" | "Intégration QR Code pour SoundCloud" |

**Steps:**
1. Read seed file
2. Fix each description at specified line
3. Run `cargo run -- schema validate`
4. Commit: "fix(seed): correct 9 wrong EntityContent descriptions"

---

## Phase C.2: Fix llm_context Corruptions (11 entities)

**Agent:** `general-purpose`

**Files:**
- Modify: `packages/db/seed/11-entity-content-fr-fr.cypher`

**Corruptions to fix:**

| entity_key | Line | Corruption | Fix |
|-----------|------|------------|-----|
| msi-plessey | 1685 | "warehoutiliser" | "warehouse" ou "entrepôt" |
| device-detection | 1893 | "utiliserr" | "utilisateur" |
| team-workspaces | 2005 | "multi-utiliserr" | "multi-utilisateur" |
| logistics | 3029 | "warehoutiliser" | "warehouse" ou "entrepôt" |
| developers | 3173 | "api utiliserr" | "api utilisateur" |

**Steps:**
1. Search for "warehoutiliser" → replace with "entrepôt"
2. Search for "utiliserr" → replace with "utilisateur"
3. Run validation
4. Commit: "fix(seed): correct llm_context corruptions"

---

## Phase C.3: Fix Typos (2 entities)

**Agent:** `general-purpose`

**Files:**
- Modify: `packages/db/seed/11-entity-content-fr-fr.cypher`

**Typos:**
- `qr-code-with-text`: "Textee" → "Texte" (3 occurrences)
- `nonprofits`: "Associationss" → "Associations" (3 occurrences)

**Steps:**
1. Replace all "Textee" with "Texte"
2. Replace all "Associationss" with "Associations"
3. Commit: "fix(seed): correct typos in EntityContent"

---

## Phase C.4: Delete Low-Volume Keywords (<20)

**Agent:** `general-purpose`

**Files:**
- Modify: `packages/db/seed/12-seokeyword-fr-fr.cypher`
- Modify: `packages/db/seed/41-seokeywords-fr-fr.cypher`

**Steps:**
1. Parse seed files for SEOKeyword nodes
2. Identify keywords with `volume < 20`
3. Remove CREATE/MERGE statements for low-volume keywords
4. Remove associated HAS_SEO_KEYWORDS arcs
5. Run `cargo run -- db seed` to verify
6. Query: `MATCH (k:SEOKeyword) WHERE k.volume < 20 RETURN count(k)` → should be 0
7. Commit: "chore(seed): remove low-volume SEO keywords (<20)"

---

## Phase C.5: Remove Legacy L10n Nodes

**Agent:** `general-purpose`

**Files to audit and clean:**
- `packages/db/seed/*.cypher` - Remove any `*L10n` nodes that are legacy
- `packages/core/models/node-classes/**/*.yaml` - Remove deprecated L10n definitions
- Neo4j: Delete legacy L10n nodes if present

**v10.9+ Naming Convention:**
- `EntityL10n` → `EntityContent` (already done)
- `PageL10n` → `PageGenerated`
- `BlockL10n` → `BlockGenerated`
- `HAS_L10N` → `HAS_CONTENT`

**Steps:**
1. Search for `L10n` in seed files (excluding EntityContent which is correct)
2. Identify legacy node types:
   - Any `*L10n` labels in Cypher
   - Any `has_l10n` or `HAS_L10N` arcs
3. Remove from seed files
4. Query Neo4j: `MATCH (n) WHERE any(label IN labels(n) WHERE label ENDS WITH 'L10n') RETURN labels(n), count(n)`
5. Delete legacy nodes: `MATCH (n) WHERE any(label IN labels(n) WHERE label =~ '.*L10n$' AND NOT label = 'EntityContent') DETACH DELETE n`
6. Verify schema: `cargo run -- schema validate`
7. Commit: "chore(schema): remove legacy L10n terminology"

---

## Execution Order

```
Phase C.1: Fix description mismatches (9 entities)
    ↓
Phase C.2: Fix llm_context corruptions (11 entities)
    ↓
Phase C.3: Fix typos (2 entities)
    ↓
Phase C.4: Delete low-volume keywords (<20 volume)
    ↓
Phase C.5: Remove legacy L10n nodes
    ↓
Final: Run db seed + validate + commit
```

---

## Success Criteria

| Phase | Metric | Target |
|-------|--------|--------|
| C.1 | Wrong descriptions fixed | 9/9 |
| C.2 | llm_context corruptions fixed | 11/11 |
| C.3 | Typos fixed | 2/2 |
| C.4 | Low-volume keywords removed | count(<20) = 0 |
| C.5 | Legacy L10n nodes | 0 in Neo4j |
| Final | Schema validates | 0 errors |

---

## Verification Queries

```cypher
-- Check no wrong descriptions remain
MATCH (e:EntityContent {entity_key: 'apple'})
RETURN e.description  -- Should NOT contain "Instagram"

-- Check no low-volume keywords
MATCH (k:SEOKeyword) WHERE k.volume < 20
RETURN count(k)  -- Should be 0

-- Check no legacy L10n
MATCH (n) WHERE any(l IN labels(n) WHERE l =~ '.*L10n$' AND l <> 'EntityContent')
RETURN labels(n), count(n)  -- Should be empty
```
