# Design: HeadSeoMeta Block & Slugification Architecture

**Date**: 2026-02-16
**Status**: Approved
**Version**: v0.13.1
**ADRs impactés**: ADR-030 (mise à jour), ADR-031, ADR-032

---

## Résumé

Ce design introduit un nouveau BlockType `head-seo-meta` qui centralise toutes les métadonnées SEO d'une page (slug, metaTitle, metaDescription) et établit une relation explicite avec les règles de slugification de la locale via l'arc `[:SLUGIFIED_BY]`.

**Changements majeurs:**
- `slug` déménage de PageNative vers BlockNative:head-seo-meta
- Nouvel arc `[:SLUGIFIED_BY]` (BlockNative → Slugification) pour VALIDATION
- Arc `[:DERIVED_SLUG_FROM]` modifié (source: PageNative → BlockNative) pour PROVENANCE
- PageNative.full_path devient calculé (dérivé du slug)
- Architecture pillar/cluster avec règle no-repetition

---

## 1. BlockType: head-seo-meta

### 1.1 Définition YAML

```yaml
# packages/core/models/node-classes/org/instruction/head-seo-meta.yaml
node:
  name: HeadSeoMeta
  realm: org
  layer: instruction
  trait: defined
  description: |
    SEO metadata block type - MUST be the first block (order=0) of every page.
    Contains URL slug, meta title, and meta description.
    The slug field is the source of truth for URL generation.

properties:
  key:
    type: string
    required: true
    description: "Block type identifier: 'head-seo-meta'"

  display_name:
    type: string
    required: true
    default: "SEO Metadata"

  schema:
    type: object
    required: true
    description: "JSON Schema for BlockNative validation"
    default:
      type: object
      properties:
        slug:
          type: string
          pattern: "^[\\p{Ll}\\p{N}\\-]+$"
          description: "URL-safe localized slug (UTF-8 allowed)"
        meta_title:
          type: string
          maxLength: 60
          description: "Page title for search engines"
        meta_description:
          type: string
          maxLength: 160
          description: "Page description for search engines"
      required: ["slug", "meta_title", "meta_description"]

  llm_context:
    type: string
    value: |
      USE: when generating SEO metadata for a page.
      TRIGGERS: slug, URL, meta title, meta description, SEO, head.
      NOT: for page content (use other block types), for keywords (use TARGETS arc).
      RELATES: BlockNative (output), Slugification (rules), PageNative (assembly).

      CRITICAL: The slug MUST follow the locale's slugification rules.
      - Check [:SLUGIFIED_BY] arc to get the Slugification node
      - latin_preserve: keep diacritics (código-qr, not codigo-qr)
      - latin_strip: remove diacritics (codigo-qr)
      - native_script: use native characters (コード)

relations:
  incoming:
    - type: OF_TYPE
      from: Block
      cardinality: "N:1"
      description: "Blocks of this type"
```

### 1.2 Contrainte de structure

```
CONTRAINTE: Chaque Page DOIT avoir exactement 1 Block avec:
  - [:OF_TYPE] → BlockType:head-seo-meta
  - [:HAS_BLOCK {order: 0}] (premier bloc)

Validation Cypher:
MATCH (p:Page)
WHERE NOT EXISTS {
  MATCH (p)-[:HAS_BLOCK {order: 0}]->(b:Block)-[:OF_TYPE]->(bt:BlockType {key: 'head-seo-meta'})
}
RETURN p.key AS page_missing_head_seo_meta
```

---

## 2. Arc: SLUGIFIED_BY

### 2.1 Définition YAML

```yaml
# packages/core/models/arc-classes/semantic/slugified-by.yaml
arc:
  name: SLUGIFIED_BY
  family: semantic
  scope: cross_realm
  source: BlockNative
  target: Slugification
  cardinality: many_to_one

  description: |
    Links a BlockNative (specifically head-seo-meta) to the Slugification rules
    used to generate its slug. Provides traceability and validation.

  properties:
    validated:
      type: boolean
      description: "Whether the slug was validated against the rules"

    applied_rule:
      type: string
      description: "The slug_rule that was applied (latin_preserve, latin_strip, etc.)"

  llm_context: |
    USE: when validating or generating URL slugs for head-seo-meta blocks.
    TRIGGERS: slug, URL, diacritics, accents, slugification, validation.
    NOT: for page routing (use full_path), for SEO keywords (use TARGETS).
    RELATES: BlockNative (output with slug), Slugification (locale rules).

    VALIDATION: Check that BlockNative.slug conforms to Slugification rules:
    - If slug_rule = "latin_preserve" and preserve_diacritics = true:
      slug "código-qr" is VALID, "codigo-qr" is INVALID (missing accent)
    - If slug_rule = "latin_strip":
      slug "codigo-qr" is VALID, "código-qr" is INVALID (has accent)

  inverse:
    name: SLUGIFIES
    description: "Slugification rules that were applied to BlockNative nodes"
```

### 2.2 Exemple d'utilisation

```cypher
// Créer l'arc SLUGIFIED_BY
MATCH (bn:BlockNative {key: 'block:qr-code:head-seo-meta:1@es-MX'})
MATCH (s:Slugification {key: 'es-MX'})
MERGE (bn)-[r:SLUGIFIED_BY]->(s)
ON CREATE SET
  r.validated = true,
  r.applied_rule = 'latin_preserve',
  r.created_at = datetime();

// Valider qu'un slug respecte les règles
MATCH (bn:BlockNative)-[:SLUGIFIED_BY]->(s:Slugification)
WHERE bn.key ENDS WITH '@es-MX'
  AND s.slug_rule = 'latin_preserve'
  AND s.preserve_diacritics = true
  AND NOT bn.slug =~ '.*[áéíóúñ].*'  // Devrait avoir des accents!
RETURN bn.key AS invalid_slug, bn.slug AS slug_value;
```

---

## 2.5 Arc: DERIVED_SLUG_FROM (Modification)

L'arc `DERIVED_SLUG_FROM` existe déjà (ADR-032) mais sa **source doit changer** de PageNative à BlockNative.

### 2.5.1 Deux arcs complémentaires

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  DEUX ARCS POUR LE SLUG                                                       ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  1. DERIVED_SLUG_FROM (PROVENANCE)                                            ║
║     Question: "D'où vient ce slug?"                                           ║
║     Direction: BlockNative:head-seo-meta → SEOKeyword                         ║
║     Utilité: Audit, debugging, tracing SEO decisions                          ║
║                                                                               ║
║  2. SLUGIFIED_BY (VALIDATION)                                                 ║
║     Question: "Ce slug est-il valide pour cette locale?"                      ║
║     Direction: BlockNative:head-seo-meta → Slugification                      ║
║     Utilité: Validation des règles de diacritiques                            ║
║                                                                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║  FLUX COMPLET                                                                 ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  EntityNative@es-MX ──[:TARGETS {rank:'primary'}]──▶ SEOKeyword               ║
║                                                        │                      ║
║                                                        ├── value: 'código qr' ║
║                                                        └── slug_form: 'código-qr'
║                                                               │                ║
║                                                               │ COPIE DIRECTE  ║
║                                                               ▼                ║
║                                        BlockNative:head-seo-meta@es-MX        ║
║                                              │                                 ║
║                                              ├── slug: 'código-qr'             ║
║                                              │                                 ║
║                  ┌───────────────────────────┴────────────────────────────┐   ║
║                  │                                                        │   ║
║                  ▼                                                        ▼   ║
║        [:DERIVED_SLUG_FROM]                                    [:SLUGIFIED_BY] ║
║        (PROVENANCE)                                            (VALIDATION)   ║
║                  │                                                        │   ║
║                  ▼                                                        ▼   ║
║           SEOKeyword                                    Slugification:es-MX   ║
║           slug_form: 'código-qr'                        latin_preserve=true   ║
║           (SOURCE OF TRUTH)                             (VÉRIFIE que ó existe)║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### 2.5.2 Modification de derived-slug-from.yaml

```yaml
# packages/core/models/arc-classes/generation/derived-slug-from.yaml
# MODIFICATION: source change de PageNative à BlockNative

arc:
  name: DERIVED_SLUG_FROM
  family: generation
  scope: cross_realm  # BlockNative (org) -> SEOKeyword (shared)

  description: >-
    Links a head-seo-meta BlockNative's slug back to the SEO keyword it was
    derived from. Provides complete audit trail for URL slugification decisions.

  llm_context: |
    USE: when auditing slug derivation decisions for SEO.
    TRIGGERS: slug audit, keyword derivation, SEO slug, URL generation audit.
    NOT: page linking (use LINKS_TO), keyword targeting (use TARGETS).
    RELATES: BlockNative:head-seo-meta (source), SEOKeyword.slug_form (target, SOURCE OF TRUTH).

    CRITICAL: The slug in BlockNative MUST be COPIED from SEOKeyword.slug_form.
    SEOKeyword.slug_form already contains the correctly formatted slug WITH diacritics.
    Example: SEOKeyword.slug_form = 'código-qr' → BlockNative.slug = 'código-qr'

  # CHANGEMENT v0.13.1: PageNative → BlockNative
  source: BlockNative
  target: SEOKeyword
  cardinality: many_to_one

  properties:
    - name: derivation_score
      type: float
      required: true
      description: "Score = volume × sem_coef × convergence_boost"

    - name: derivation_rationale
      type: string
      required: false
      description: "LLM explanation of why this keyword was chosen"

    - name: no_repetition_applied
      type: boolean
      required: false
      default: false
      description: "Whether the no-repetition rule removed parent terms"

  cypher_pattern: "(BlockNative)-[:DERIVED_SLUG_FROM {derivation_score}]->(SEOKeyword)"
```

### 2.5.3 Règle critique: slug = SEOKeyword.slug_form

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  🔴 RÈGLE CRITIQUE: Le slug est COPIÉ, pas généré                             ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  SEOKeyword.slug_form = SOURCE OF TRUTH pour le slug                          ║
║                                                                               ║
║  Le LLM ne GÉNÈRE PAS le slug - il le COPIE depuis SEOKeyword.slug_form       ║
║                                                                               ║
║  Exemple (es-MX):                                                             ║
║  ┌─────────────────────────────────────────────────────────────────────────┐  ║
║  │ SEOKeyword {                                                            │  ║
║  │   key: 'seo:codigo-qr@es-MX',                                           │  ║
║  │   value: 'código qr',           // Keyword avec espaces                 │  ║
║  │   slug_form: 'código-qr'        // ← SOURCE OF TRUTH (AVEC accent!)    │  ║
║  │ }                                                                       │  ║
║  └─────────────────────────────────────────────────────────────────────────┘  ║
║                              │                                                ║
║                              │ COPIE DIRECTE (pas de génération)              ║
║                              ▼                                                ║
║  ┌─────────────────────────────────────────────────────────────────────────┐  ║
║  │ BlockNative:head-seo-meta@es-MX {                                       │  ║
║  │   slug: 'código-qr'             // ← IDENTIQUE à slug_form              │  ║
║  │ }                                                                       │  ║
║  └─────────────────────────────────────────────────────────────────────────┘  ║
║                                                                               ║
║  Ceci garantit que les accents sont TOUJOURS corrects car ils sont           ║
║  déjà présents dans SEOKeyword.slug_form (créé par l'import Ahrefs).         ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## 3. Modification de PageNative

### 3.1 Avant (v0.13.0)

```yaml
# PageNative avait:
routing_properties:
  slug:
    type: string
    required: true
    description: "URL-safe localized page identifier"
  full_path:
    type: string
    required: true
    description: "Full URL path from root"
```

### 3.2 Après (v0.13.1)

```yaml
# packages/core/models/node-classes/org/output/page-native.yaml
node:
  name: PageNative
  realm: org
  layer: output
  trait: generated
  description: |
    Locale-specific page output - assembly of all BlockNative nodes.
    The slug now lives in BlockNative:head-seo-meta.
    full_path is CALCULATED from parent hierarchy + head-seo-meta.slug.

properties:
  key:
    type: string
    required: true
    pattern: "^page:[a-z0-9-]+@[a-z]{2}-[A-Z]{2}$"
    description: "Composite key: page:{page_key}@{locale}"

  # slug SUPPRIMÉ - vit maintenant dans BlockNative:head-seo-meta

  full_path:
    type: string
    required: true
    indexed: true
    description: |
      CALCULATED full URL path. Formula:
      - If pillar or no parent: "/" + locale + "/" + head-seo-meta.slug
      - If cluster (has parent): parent.full_path + "/" + head-seo-meta.slug

  published_at:
    type: datetime
    description: "Publication timestamp"

  status:
    type: string
    enum: ["draft", "review", "published"]
    default: "draft"

relations:
  outgoing:
    - type: ASSEMBLES
      to: BlockNative
      cardinality: "1:N"
      properties:
        order: integer
      description: "BlockNative nodes assembled in order"

    - type: FOR_LOCALE
      to: Locale
      cardinality: "N:1"
      description: "Target locale for this page"

  incoming:
    - type: HAS_NATIVE
      from: Page
      cardinality: "1:N"
      description: "Parent invariant page"
```

### 3.3 Calcul de full_path

```cypher
// Fonction de calcul full_path
// Appelée après génération du BlockNative:head-seo-meta

// CAS 1: Pillar ou page racine (pas de parent)
MATCH (p:Page {is_pillar: true})-[:HAS_BLOCK {order: 0}]->(b:Block)-[:OF_TYPE]->(:BlockType {key: 'head-seo-meta'})
MATCH (b)-[:HAS_NATIVE]->(bn:BlockNative)
MATCH (p)-[:HAS_NATIVE]->(pn:PageNative)
WHERE bn.key ENDS WITH pn.key  // Même locale
SET pn.full_path = '/' + split(pn.key, '@')[1] + '/' + bn.slug;

// CAS 2: Cluster (a un parent via SUBTOPIC_OF)
MATCH (child:Page)-[:SUBTOPIC_OF]->(parent:Page)
MATCH (child)-[:HAS_BLOCK {order: 0}]->(b:Block)-[:OF_TYPE]->(:BlockType {key: 'head-seo-meta'})
MATCH (b)-[:HAS_NATIVE]->(bn:BlockNative)
MATCH (child)-[:HAS_NATIVE]->(childNative:PageNative)
MATCH (parent)-[:HAS_NATIVE]->(parentNative:PageNative)
WHERE bn.key ENDS WITH childNative.key
  AND childNative.key ENDS WITH parentNative.key  // Même locale
SET childNative.full_path = parentNative.full_path + '/' + bn.slug;
```

---

## 4. Architecture Pillar / Cluster

### 4.1 Structure des URLs

```
PILLAR (page hub principale)
══════════════════════════════════════════════════════════════════════

Page:qr-code-generator
    │ is_pillar: true
    │
    ├── [:HAS_BLOCK {order: 0}]
    │       └── Block:qr-code-generator:head-seo-meta:1
    │               └── BlockNative@es-MX
    │                       slug: "generador-código-qr"  ← AVEC ACCENT (latin_preserve)
    │                       meta_title: "Generador de Código QR Gratis"
    │                       [:SLUGIFIED_BY] → Slugification:es-MX
    │
    └── [:HAS_NATIVE]
            └── PageNative@es-MX
                    full_path: "/es-MX/generador-código-qr"


CLUSTERS (pages satellites)
══════════════════════════════════════════════════════════════════════

Page:qr-instagram
    │ is_pillar: false
    │
    ├── [:SUBTOPIC_OF] ────────→ Page:qr-code-generator  (URL hierarchy)
    ├── [:SEO_CLUSTER_OF] ─────→ Page:qr-code-generator  (SEO strategy)
    │
    ├── [:HAS_BLOCK {order: 0}]
    │       └── Block:qr-instagram:head-seo-meta:1
    │               └── BlockNative@es-MX
    │                       slug: "instagram"  ← DIFFÉRENCIATEUR SEULEMENT
    │                       meta_title: "QR Code para Instagram"
    │                       [:SLUGIFIED_BY] → Slugification:es-MX
    │
    └── [:HAS_NATIVE]
            └── PageNative@es-MX
                    full_path: "/es-MX/generador-código-qr/instagram"
                               ▲                          ▲
                               │                          │
                         parent.full_path          head-seo-meta.slug
```

### 4.2 Règle NO-REPETITION (ADR-032)

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  RÈGLE CRITIQUE: Pas de répétition dans l'URL                                 ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  ❌ MAUVAIS:                                                                  ║
║  /es-MX/generador-código-qr/código-qr-para-instagram                          ║
║                             ^^^^^^^^                                          ║
║                             "código-qr" répété = pénalité SEO!                ║
║                                                                               ║
║  ✅ BON:                                                                      ║
║  /es-MX/generador-código-qr/instagram                                         ║
║                             ^^^^^^^^^                                         ║
║                             Juste la partie différenciante                    ║
║                                                                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║  ALGORITHME (pour LLM):                                                       ║
║                                                                               ║
║  1. Récupérer parent.full_path → extraire les termes                          ║
║  2. Pour le keyword SEO ciblé → extraire les termes                           ║
║  3. slug = keyword_terms - parent_terms                                       ║
║                                                                               ║
║  Exemple:                                                                     ║
║  parent_terms = {"generador", "código", "qr"}                                 ║
║  keyword = "código qr para instagram"                                         ║
║  keyword_terms = {"código", "qr", "para", "instagram"}                        ║
║  new_terms = keyword_terms - parent_terms = {"para", "instagram"}             ║
║  slug = "instagram" (on enlève aussi les stop words comme "para")             ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### 4.3 Formule full_path

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  CALCUL DE full_path                                                          ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  SI page.is_pillar = true OU pas de [:SUBTOPIC_OF]:                           ║
║      full_path = "/" + locale_key + "/" + head-seo-meta.slug                  ║
║                                                                               ║
║  SI page a [:SUBTOPIC_OF] vers un parent:                                     ║
║      full_path = parent.full_path + "/" + head-seo-meta.slug                  ║
║                                                                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║  EXEMPLES (es-MX)                                                             ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  Pillar: qr-code-generator                                                    ║
║      slug: "generador-código-qr"                                              ║
║      full_path: "/es-MX/generador-código-qr"                                  ║
║                                                                               ║
║  Cluster: qr-instagram                                                        ║
║      slug: "instagram"                                                        ║
║      full_path: "/es-MX/generador-código-qr/instagram"                        ║
║                                                                               ║
║  Cluster: qr-wifi                                                             ║
║      slug: "wifi"                                                             ║
║      full_path: "/es-MX/generador-código-qr/wifi"                             ║
║                                                                               ║
║  Cluster: qr-menu                                                             ║
║      slug: "menu-restaurante"                                                 ║
║      full_path: "/es-MX/generador-código-qr/menu-restaurante"                 ║
║                                                                               ║
║  Sub-cluster: qr-menu-cafe (sous qr-menu)                                     ║
║      slug: "cafe"                                                             ║
║      full_path: "/es-MX/generador-código-qr/menu-restaurante/cafe"            ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## 5. Architecture Complète

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  VUE D'ENSEMBLE                                                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  Page:qr-code (invariant, defined)                                            ║
║      │                                                                        ║
║      ├── is_pillar: true                                                      ║
║      │                                                                        ║
║      ├──[:HAS_BLOCK {order: 0}]──▶ Block:qr-code:head-seo-meta:1              ║
║      │                                  │                                     ║
║      │                                  ├──[:OF_TYPE]                         ║
║      │                                  │       ▼                             ║
║      │                                  │   BlockType:head-seo-meta           ║
║      │                                  │                                     ║
║      │                                  └──[:HAS_NATIVE]                      ║
║      │                                          ▼                             ║
║      │                                  BlockNative@es-MX                     ║
║      │                                      slug: "código-qr"                 ║
║      │                                      meta_title: "Generador..."        ║
║      │                                      meta_description: "..."           ║
║      │                                          │                             ║
║      │                                          │[:SLUGIFIED_BY]              ║
║      │                                          ▼                             ║
║      │                                  Slugification:es-MX                   ║
║      │                                      slug_rule: "latin_preserve"       ║
║      │                                      preserve_diacritics: true         ║
║      │                                                                        ║
║      ├──[:HAS_BLOCK {order: 1}]──▶ Block:qr-code:hero:1                       ║
║      │                                  └── BlockNative@es-MX (hero content)  ║
║      │                                                                        ║
║      ├──[:HAS_BLOCK {order: 2}]──▶ Block:qr-code:features:1                   ║
║      │                                  └── BlockNative@es-MX (features)      ║
║      │                                                                        ║
║      └──[:HAS_NATIVE]──▶ PageNative@es-MX                                     ║
║                              full_path: "/es-MX/código-qr"                    ║
║                              status: "published"                              ║
║                                  │                                            ║
║                                  ├──[:ASSEMBLES {order: 0}]──▶ BlockNative:head-seo-meta@es-MX
║                                  ├──[:ASSEMBLES {order: 1}]──▶ BlockNative:hero@es-MX
║                                  ├──[:ASSEMBLES {order: 2}]──▶ BlockNative:features@es-MX
║                                  │                                            ║
║                                  └──[:FOR_LOCALE]──▶ Locale:es-MX             ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## 6. Queries Utiles

### 6.1 Récupérer le contexte SEO complet d'une page

```cypher
// Contexte SEO pour génération
MATCH (p:Page {key: $pageKey})
MATCH (p)-[:HAS_BLOCK {order: 0}]->(b:Block)-[:OF_TYPE]->(:BlockType {key: 'head-seo-meta'})
MATCH (b)-[:HAS_NATIVE]->(bn:BlockNative)-[:SLUGIFIED_BY]->(s:Slugification)
MATCH (p)-[:HAS_NATIVE]->(pn:PageNative)-[:FOR_LOCALE]->(l:Locale {key: $locale})
OPTIONAL MATCH (p)-[:SUBTOPIC_OF]->(parent:Page)-[:HAS_NATIVE]->(parentNative:PageNative)
WHERE parentNative.key ENDS WITH $locale

RETURN
  bn.slug AS slug,
  bn.meta_title AS meta_title,
  bn.meta_description AS meta_description,
  s.slug_rule AS slugification_rule,
  s.preserve_diacritics AS preserve_diacritics,
  pn.full_path AS full_path,
  parentNative.full_path AS parent_path,
  p.is_pillar AS is_pillar;
```

### 6.2 Valider tous les slugs d'une locale

```cypher
// Trouver les slugs qui ne respectent pas les règles
MATCH (bn:BlockNative)-[:SLUGIFIED_BY]->(s:Slugification)
WHERE bn.key CONTAINS ':head-seo-meta:'
  AND s.slug_rule = 'latin_preserve'
  AND s.preserve_diacritics = true

// Vérifier que les slugs espagnols ont des accents quand ils devraient
WITH bn, s
WHERE bn.key ENDS WITH '@es-MX'
  AND bn.slug CONTAINS 'codigo'  // Devrait être "código"
  AND NOT bn.slug CONTAINS 'ó'

RETURN bn.key AS invalid_slug, bn.slug AS current_slug, 'Missing accent in código' AS issue;
```

### 6.3 Construire l'arbre d'URLs complet

```cypher
// Récupérer toute la hiérarchie URL pour une locale
MATCH (p:Page)
OPTIONAL MATCH (p)-[:SUBTOPIC_OF]->(parent:Page)
MATCH (p)-[:HAS_NATIVE]->(pn:PageNative {key: $locale})
MATCH (p)-[:HAS_BLOCK {order: 0}]->(b:Block)-[:HAS_NATIVE]->(bn:BlockNative)
WHERE bn.key ENDS WITH $locale

RETURN
  p.key AS page_key,
  p.is_pillar AS is_pillar,
  parent.key AS parent_key,
  bn.slug AS slug,
  pn.full_path AS full_path
ORDER BY pn.full_path;
```

---

## 7. Migration

### 7.1 Fichiers à créer

| Fichier | Description |
|---------|-------------|
| `packages/core/models/node-classes/org/instruction/head-seo-meta.yaml` | BlockType definition |
| `packages/core/models/arc-classes/semantic/slugified-by.yaml` | Arc definition |
| `packages/db/seed/53-blocktype-head-seo-meta.cypher` | BlockType seed |
| `packages/db/seed/54-slugified-by-arcs.cypher` | SLUGIFIED_BY arcs |

### 7.2 Fichiers à modifier

| Fichier | Modification |
|---------|--------------|
| `packages/core/models/node-classes/org/output/page-native.yaml` | Supprimer `slug`, garder `full_path` (calculé) |
| `packages/db/seed/50-page-native.cypher` | Supprimer `slug`, recalculer `full_path` |
| `.claude/rules/adr/schema-architecture/adr-030-slug-ownership.md` | Mettre à jour |

### 7.3 Migration des données existantes

```cypher
// 1. Créer les BlockType:head-seo-meta
MERGE (bt:BlockType {key: 'head-seo-meta'})
SET bt.display_name = 'SEO Metadata',
    bt.realm = 'org',
    bt.layer = 'instruction',
    bt.trait = 'defined';

// 2. Pour chaque Page existante, créer le Block:head-seo-meta
MATCH (p:Page)
MERGE (b:Block {key: p.key + ':head-seo-meta:1'})
SET b.page_key = p.key,
    b.block_type = 'head-seo-meta',
    b.order = 0;

MERGE (p)-[:HAS_BLOCK {order: 0}]->(b);
MERGE (b)-[:OF_TYPE]->(:BlockType {key: 'head-seo-meta'});

// 3. Migrer les slugs de PageNative vers BlockNative
MATCH (pn:PageNative)
MATCH (p:Page)-[:HAS_NATIVE]->(pn)
MATCH (p)-[:HAS_BLOCK {order: 0}]->(b:Block)-[:OF_TYPE]->(:BlockType {key: 'head-seo-meta'})

// Extraire locale du key
WITH pn, b, split(pn.key, '@')[1] AS locale

MERGE (bn:BlockNative {key: b.key + '@' + locale})
SET bn.slug = pn.slug,
    bn.meta_title = 'TODO: Generate',
    bn.meta_description = 'TODO: Generate';

MERGE (b)-[:HAS_NATIVE]->(bn);

// 4. Créer les arcs SLUGIFIED_BY
MATCH (bn:BlockNative)
WHERE bn.key CONTAINS ':head-seo-meta:'
WITH bn, split(bn.key, '@')[1] AS locale
MATCH (s:Slugification {key: locale})
MERGE (bn)-[:SLUGIFIED_BY]->(s);

// 5. Supprimer slug de PageNative (full_path reste)
MATCH (pn:PageNative)
REMOVE pn.slug;
```

---

## 8. Mise à jour ADR-030

### Avant

> "Page owns URL (slug), Entity owns semantics"

### Après

> "Block:head-seo-meta owns slug, PageNative owns full_path (calculated).
> The slug lives in BlockNative:head-seo-meta with explicit [:SLUGIFIED_BY]
> relationship to the locale's Slugification rules."

---

## 9. Tests de validation

```typescript
describe('HeadSeoMeta Architecture', () => {
  it('every Page has exactly one head-seo-meta block at order 0', async () => {
    const result = await cypher(`
      MATCH (p:Page)
      WHERE NOT EXISTS {
        MATCH (p)-[:HAS_BLOCK {order: 0}]->(b:Block)-[:OF_TYPE]->(:BlockType {key: 'head-seo-meta'})
      }
      RETURN count(p) AS missing_count
    `);
    expect(result.missing_count).toBe(0);
  });

  it('every head-seo-meta BlockNative has SLUGIFIED_BY arc', async () => {
    const result = await cypher(`
      MATCH (bn:BlockNative)
      WHERE bn.key CONTAINS ':head-seo-meta:'
        AND NOT EXISTS { (bn)-[:SLUGIFIED_BY]->(:Slugification) }
      RETURN count(bn) AS missing_count
    `);
    expect(result.missing_count).toBe(0);
  });

  it('es-MX slugs preserve diacritics', async () => {
    const result = await cypher(`
      MATCH (bn:BlockNative)-[:SLUGIFIED_BY]->(s:Slugification {key: 'es-MX'})
      WHERE s.preserve_diacritics = true
        AND bn.slug CONTAINS 'codigo'
        AND NOT bn.slug CONTAINS 'ó'
      RETURN bn.key AS invalid
    `);
    expect(result.length).toBe(0);
  });

  it('full_path is correctly calculated for clusters', async () => {
    const result = await cypher(`
      MATCH (child:Page)-[:SUBTOPIC_OF]->(parent:Page)
      MATCH (child)-[:HAS_NATIVE]->(childNative:PageNative)
      MATCH (parent)-[:HAS_NATIVE]->(parentNative:PageNative)
      MATCH (child)-[:HAS_BLOCK {order: 0}]->(b:Block)-[:HAS_NATIVE]->(bn:BlockNative)
      WHERE childNative.key ENDS WITH parentNative.key
        AND bn.key ENDS WITH childNative.key
        AND childNative.full_path <> parentNative.full_path + '/' + bn.slug
      RETURN childNative.key AS invalid
    `);
    expect(result.length).toBe(0);
  });
});
```

---

## 10. Checklist d'implémentation

- [ ] Créer `head-seo-meta.yaml` (BlockType)
- [ ] Créer `slugified-by.yaml` (Arc)
- [ ] Modifier `page-native.yaml` (supprimer slug)
- [ ] Créer seed `53-blocktype-head-seo-meta.cypher`
- [ ] Créer seed `54-slugified-by-arcs.cypher`
- [ ] Mettre à jour seed `50-page-native.cypher`
- [ ] Mettre à jour ADR-030
- [ ] Régénérer artifacts: `cargo run -- schema generate`
- [ ] Valider schema: `cargo run -- schema validate`
- [ ] Exécuter migration Neo4j
- [ ] Ajouter tests de validation
- [ ] Mettre à jour Studio si nécessaire
