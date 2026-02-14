# NovaNet v0.12.5 Architecture Visuelle

**Date**: 2026-02-14
**Status**: En validation
**Version**: v0.12.5

## Vue d'ensemble - Le Graph NovaNet

```
INVARIANT (defined)                      LOCALE-SPECIFIC (*Native)
══════════════════                       ═════════════════════════

┌─────────────┐                          ┌─────────────────┐
│   Project   │───[:HAS_NATIVE]─────────▶│ ProjectNative   │
│ key:"qrcode"│    {locale:"fr-FR"}      │ @fr-FR          │
└──────┬──────┘                          └─────────────────┘
       │
       │[:HAS_PAGE]        [:HAS_ENTITY]
       ▼                         ▼
┌─────────────┐            ┌─────────────┐
│    Page     │◀──────────▶│   Entity    │  [:REPRESENTS] (1:1)
│ key:"qr-ig" │            │ key:"qr-ig" │
│ slug:"insta"│            │             │
└──────┬──────┘            └──────┬──────┘
       │                          │
       │[:HAS_NATIVE]             │[:HAS_NATIVE]
       ▼                          ▼
┌─────────────────┐        ┌─────────────────┐
│  PageNative     │        │  EntityNative   │
│  @fr-FR         │        │  @fr-FR         │
│  slug:"insta"   │        │  title:"QR..."  │
│  full_path:     │        │  description... │
│  "/{locale}/..."│        │  benefits[...]  │
│  (generated)    │        │  (authored)     │
└─────────────────┘        └─────────────────┘
```

## Hiérarchies séparées (Entity vs Page)

```
SEMANTIC HIERARCHY (Entity)              URL HIERARCHY (Page)
Topic clusters pour SEO                  Routing pour navigation

Entity:qr-generator (PILLAR)             Page:qr-generator
├── key: "qr-generator"                  ├── key: "qr-generator"
├── is_pillar: true                      ├── slug: "qr-generator"
│                                        │
│ [:SUBTOPIC_OF]                         │ [:SUBTOPIC_OF]
│      ▲                                 │      ▲
│      │                                 │      │
Entity:qr-instagram                      Page:qr-instagram
├── key: "qr-instagram"                  ├── key: "qr-instagram"
├── is_pillar: false                     ├── slug: "instagram"  ◀── DIFFÉRENT
│                                        │
└── [:SEMANTIC_LINK]──▶ Entity:instagram │
    {link_type: "used_for", temp: 0.8}   │
                                         │
─────────────────────────────────────────┼───────────────────────────────────
RÉSULTAT URL:                            │
                                         ▼
PageNative:qr-instagram@fr-FR
└── full_path: "/{locale}/générateur-qr-code/instagram"
                ▲                    ▲
                │                    │
          parent.slug            this.slug
          (localisé)             (brand=invariant)
```

## Page → Block → BlockNative

```
Page:pricing
├── key: "pricing"
├── slug: "pricing"
│
├──[:HAS_BLOCK {order:1}]──▶ Block:pricing:hero:1
│                            ├── key: "pricing:hero:1"
│                            ├── anchor_id: "hero"
│                            │
│                            ├──[:OF_TYPE]──▶ BlockType:hero
│                            │               └── schema JSON
│                            │
│                            ├──[:HAS_INSTRUCTION]──▶ BlockInstruction
│                            │                       └── @ refs, directives
│                            │
│                            └──[:HAS_NATIVE {locale}]──▶ BlockNative
│                                                        ├── @fr-FR
│                                                        ├── title: "..."
│                                                        ├── body: "..."
│                                                        └── anchor_slug
│
├──[:HAS_BLOCK {order:2}]──▶ Block:pricing:features:1
│                            └── ...
│
└──[:HAS_BLOCK {order:3}]──▶ Block:pricing:cta:1
                             └── ...

COMPOSITE KEY FORMAT: {page_key}:{block_type}:{index}
Permet plusieurs blocks du même type par page
```

## SEO + GEO Intelligence

```
DUAL SEO PATTERN
════════════════

Entity:qr-generator
│
├──[:HAS_KEYWORD {rank:"primary"}]──▶ SEOKeyword:"qr code generator"
│   STRATÉGIE: "cette entité DOIT ranker sur ce mot-clé"
│
└──[:HAS_NATIVE]──▶ EntityNative:qr-generator@fr-FR
                    │
                    └──[:TARGETS]──▶ SEOKeyword:"générateur qr code"
                        TACTIQUE: "ce contenu CIBLE ce mot-clé"

─────────────────────────────────────────────────────────────────────────────

GEO MONITORING (AI Visibility)
══════════════════════════════

GEOQuery (imported)                      GEOAnswer (retrieved)
├── query: "best qr code generator"      ├── snapshot de la réponse AI
├── engine: "perplexity"                 ├── mentions_us: true/false
├── locale: "en-US"                      ├── position: 3
│                                        ├── competitors_mentioned: [...]
└──[:HAS_ANSWER]────────────────────────▶└── retrieved_at: timestamp

EntityNative ──[:MONITORS]──▶ GEOQuery
"Ce contenu surveille cette requête AI"
```

## L'arc HAS_NATIVE unifié

```
AVANT (v0.12.4)                      APRÈS (v0.12.5)
══════════════                       ═══════════════

Entity ──[:HAS_CONTENT]──▶ EntityContent    Entity ──[:HAS_NATIVE]──▶
Page ──[:HAS_GENERATED]──▶ PageGenerated           {locale:"fr-FR"}
Block ──[:HAS_GENERATED]──▶ BlockGenerated                │
                                                          ▼
2 arcs différents                           EntityNative / PageNative /
2 suffixes différents                       BlockNative / ProjectNative

                                            1 arc unifié
                                            1 suffixe (*Native)
                                            trait indique WHO creates:
                                            ├── authored = human
                                            └── generated = LLM
```

## Key Format

```
{type}:{invariant_key}@{locale}

Examples:
├── entity:qr-generator@fr-FR
├── page:pricing@de-DE
├── block:pricing:hero:1@ja-JP
└── project:qrcode-ai@es-MX
```

## Changements v0.12.5

| Catégorie | Avant | Après |
|-----------|-------|-------|
| **Nodes** | EntityContent | EntityNative |
| | PageGenerated | PageNative |
| | BlockGenerated | BlockNative |
| | ProjectContent | ProjectNative |
| **Arcs** | HAS_CONTENT + HAS_GENERATED | HAS_NATIVE {locale} |
| **Properties** | slug sur EntityNative | slug sur PageNative |
| | full_path sur EntityNative | full_path sur PageNative |
| | parent_slug | REMOVED (calculated) |
| | depth | REMOVED (calculated) |

## Décisions URL/Locale (Session 2)

### URL Format: BCP-47 complet

```
FORMAT: /{locale.key}/{parent.slug}/.../{page.slug}

EXEMPLES:
├── /fr-FR/générateur-qr-code/instagram
├── /en-US/qr-code-generator/instagram
├── /ar-SA/مولد-qr/انستغرام
└── /ja-JP/qrコードジェネレーター/インスタグラム
```

### Slug Fallback Pattern

```
effective_slug = PageNative.slug ?? Page.slug

Page.slug         = default EN (invariant)
PageNative.slug   = override localisé SI DIFFÉRENT (sinon NULL)
```

### Slug SEO-Driven (Option D)

Le slug est une décision éditoriale/SEO, pas une règle automatique.

```
PageNative (org/output, generated)
├── slug: string | null              Override ou NULL (fallback)
├── slug_rationale: string | null    Justification SEO si override
├── full_path: string (required)     Stocké + recalculé via trigger
│
└── [:TARGETS] ──▶ SEOKeyword        Justification implicite
```

### full_path: Stocké + Trigger (Option C)

```
full_path = "/" + locale.key + "/" + ancestor_slugs.join("/")

- Stocké sur PageNative pour performance
- Indexé dans Neo4j
- Recalculé automatiquement si parent.slug change
```

## Session 3: EntityNative = Package Complet

### Modèle Validé

```
EntityNative = Concept + Keywords (le package complet par locale)

Entity:qr-generator
│
├──[:HAS_NATIVE]──▶ EntityNative@fr-FR
│                   ├── title: "Générateur QR Code"
│                   ├── description: "..."
│                   │
│                   └──[:TARGETS]──▶ SEOKeyword* (MULTIPLE!)
│                       ├── "créer qr code"         (vol: 50,000)
│                       ├── "générateur qr code"    (vol: 5,000)
│                       ├── "qr code gratuit"       (vol: 80,000)
│                       └── ... (N keywords par locale)
│
├──[:HAS_NATIVE]──▶ EntityNative@en-US (50 keywords)
└──[:HAS_NATIVE]──▶ EntityNative@ja-JP (2 keywords)
```

### SEOKeyword = Locale-Specific

```
SEOKeyword (shared/knowledge, imported)
├── key: "fr-FR:creer-qr-code"
├── locale: "fr-FR"
├── text: "créer qr code"
├── volume: 50,000
├── slug_form: "créer-qr-code"  ← Avec accents (UTF-8)!
└── retrieved_at: timestamp
```

### Slugification par Locale

```
Locale:fr-FR
├── slugification:
│   ├── allow_accents: true        ← UTF-8 slugs
│   ├── allowed_chars: "a-zà-ÿ0-9-"
│   └── transform: "lowercase, normalize_nfd, hyphenate"
│
└── slug_form: "créer-qr-code"  ← Accents conservés!

Locale:en-US
├── slugification:
│   ├── allow_accents: false       ← ASCII only
│   └── allowed_chars: "a-z0-9-"

Locale:ar-SA
├── slugification:
│   ├── allow_arabic: true
│   └── direction: rtl
│
└── slug_form: "مولد-qr"
```

### Sémantique > Volume: SEMANTIC_LINK Coefficients

```
PROBLÈME: Volume seul donne le mauvais slug
═══════════════════════════════════════════

EntityNative:qr-generator@fr-FR TARGETS:
├── "créer qr code"      (vol: 50,000)
├── "qr code gratuit"    (vol: 80,000)  ← Plus gros mais FAUX!
└── "générateur qr"      (vol: 5,000)

"gratuit" est un ATTRIBUT, pas un substitut de "generator"
"créer" est L'ACTION, sémantiquement équivalent

SOLUTION: Coefficient par type de lien
══════════════════════════════════════

SEMANTIC_LINK.type → coefficient:
├── same_as:        1.0   (synonyme parfait)
├── action_for:     0.95  (l'action équivaut au tool)
├── synonym:        0.9
├── produces:       0.85  (le résultat représente l'outil)
├── subtopic_of:    0.7
├── related_to:     0.5
├── attribute_of:   0.3   ← "gratuit" pénalisé!
└── opposite_of:    0.0

Entity:qr-generator
├──[:SEMANTIC_LINK {type: "action_for", coef: 0.95}]──▶ Entity:create-qr
└──[:SEMANTIC_LINK {type: "attribute_of", coef: 0.3}]──▶ Entity:free
```

### Convergence: Keyword → Multiple Entities

```
INSIGHT: Keyword dans plusieurs entities liées = hub keyword
═══════════════════════════════════════════════════════════

SEOKeyword:"créer qr code" est TARGETS par:
├── EntityNative:create-qr@fr-FR      (l'action)
├── EntityNative:qr-generator@fr-FR   (l'outil)
└── EntityNative:make-qr-code@fr-FR   (synonyme)

Ces entities sont SEMANTIC_LINK entre elles!
→ "créer qr code" = CENTRE du cluster sémantique
→ BOOST: convergence_boost = 1 + (N × 0.2)
```

### Algorithme de Dérivation du Slug

```
FORMULE: score = volume × sem_coef × convergence_boost

EXEMPLE Page:qr-generator @fr-FR:
══════════════════════════════════

"générateur qr":
  base = 5,000
  sem_coef = 1.0 (direct)
  convergence = 1.0 (1 entity)
  SCORE = 5,000 × 1.0 × 1.0 = 5,000

"qr code gratuit":
  base = 80,000
  sem_coef = 0.3 (attribute_of → pénalisé!)
  convergence = 1.0
  SCORE = 80,000 × 0.3 × 1.0 = 24,000

"créer qr code":
  base = 50,000
  sem_coef = 0.95 (action_for)
  convergence = 1.2 (2 entities: qr-generator + create-qr)
  SCORE = 50,000 × 0.95 × 1.2 = 57,000  ← WINNER!

RESULT:
  slug = "créer-qr-code"
  score = 57,000
  sources = [Entity:qr-generator, Entity:create-qr]
```

### Cypher: Slug Derivation Query

```cypher
// Dériver le meilleur slug pour une Page dans une Locale
MATCH (p:Page)-[:REPRESENTS]->(e:Entity)
MATCH (e)-[:HAS_NATIVE]->(en:EntityNative {locale: $locale})
MATCH (en)-[:TARGETS]->(kw:SEOKeyword)

// Score direct
WITH p, e, kw, kw.volume AS base_vol, 1.0 AS sem_coef

// Ajouter keywords via SEMANTIC_LINK
UNION
MATCH (p:Page)-[:REPRESENTS]->(e:Entity)
MATCH (e)-[sl:SEMANTIC_LINK]->(related:Entity)
MATCH (related)-[:HAS_NATIVE]->(rn:EntityNative {locale: $locale})
MATCH (rn)-[:TARGETS]->(kw:SEOKeyword)
WITH p, e, kw, kw.volume AS base_vol,
     CASE sl.type
       WHEN 'same_as' THEN 1.0
       WHEN 'action_for' THEN 0.95
       WHEN 'synonym' THEN 0.9
       WHEN 'produces' THEN 0.85
       WHEN 'subtopic_of' THEN 0.7
       WHEN 'related_to' THEN 0.5
       WHEN 'attribute_of' THEN 0.3
       ELSE 0.5
     END AS sem_coef

// Calculer convergence (combien d'entities ont ce keyword)
WITH p, kw, base_vol, sem_coef,
     SIZE([(en2:EntityNative {locale: $locale})-[:TARGETS]->(kw) | en2]) AS convergence_count

// Score final
WITH p, kw,
     base_vol * sem_coef * (1 + convergence_count * 0.2) AS final_score
ORDER BY final_score DESC
LIMIT 1

RETURN kw.slug_form AS slug, final_score AS score, kw.key AS source
```

## Résumé Décisions Session 3

| Aspect | Décision |
|--------|----------|
| **Slugification** | Locale.slugification rules (accents OK en fr-FR, UTF-8) |
| **Sémantique** | SEMANTIC_LINK.type → coefficient (action_for: 0.95, attribute_of: 0.3) |
| **Convergence** | Keyword dans N entities → boost ×(1 + N×0.2) |
| **Formule** | `score = volume × sem_coef × convergence_boost` |
| **Storage** | PageNative.slug stocké + slug_source + slug_score pour audit |

## Session 4: Keywords, Slugs et Full Path

### Option A Validée: Keywords liés directement à EntityNative

```
EntityNative:qr-generator@fr-FR
└──[:TARGETS]──▶ SEOKeyword* (seulement ceux qui REPRÉSENTENT cette entity)
    ├── "générateur qr code"    ← OUI, représente
    ├── "créer qr code"         ← OUI, action équivalente
    └── "qr code gratuit"       ← NON! → va dans EntityNative:free-qr@fr-FR

Le filtrage est fait EN AMONT (import/curation).
Pas de coefficient sur TARGETS direct - tous sont pertinents.
```

### Keyword vs Slug: Deux Concepts Différents

```
SEOKeyword (imported)                    Slug (derived/generated)
══════════════════════                   ════════════════════════
• Données réelles de recherche           • URL segment final
• Volume, difficulty, trends             • Doit être URL-safe
• Importé depuis Ahrefs/SEMrush          • Peut être = keyword OU généré
• Plusieurs par EntityNative             • UN seul par PageNative

Le slug peut:
├── Être exactement un keyword existant ("créer-qr-code")
├── Être une variation d'un keyword
├── Être généré/mergé à partir du contexte
└── DOIT respecter les règles SEO du full_path
```

### RÈGLE CRITIQUE: Pas de Répétition dans le Full Path

```
EXEMPLE: Page:qr-instagram (enfant de Page:qr-generator)

❌ MAUVAIS:
full_path = /fr-FR/créer-qr-code/qr-code-pour-instagram
                   ^^^^^^          ^^^^^^
                   RÉPÉTITION de "qr-code" = pénalité SEO!

✅ BON:
full_path = /fr-FR/créer-qr-code/instagram
                                 ^^^^^^^^^
                   Juste la partie différenciante

LOGIQUE:
├── Parent slug contient déjà "qr-code" (via "créer-qr-code")
├── Child slug = seulement ce qui différencie ("instagram")
└── Google comprend le contexte via la hiérarchie
```

### Algorithme de Dérivation du Slug (v2)

```
INPUTS pour Page:qr-instagram @fr-FR:
═════════════════════════════════════
├── Entity:qr-instagram (definition)
├── EntityNative@fr-FR (title, keywords TARGETS)
├── Parent: Page:qr-generator
│   └── PageNative@fr-FR.slug = "créer-qr-code"
├── SEMANTIC_LINK entities
└── Locale:fr-FR (slugification rules)

ÉTAPE 1: Collecter les termes du parent path
──────────────────────────────────────────────
parent_terms = ["créer", "qr", "code"]

ÉTAPE 2: Analyser les keywords candidats
────────────────────────────────────────
EntityNative:qr-instagram@fr-FR TARGETS:
├── "qr code instagram"     → terms: [qr, code, instagram]
├── "instagram qr"          → terms: [instagram, qr]
└── "lien qr instagram"     → terms: [lien, qr, instagram]

ÉTAPE 3: Extraire la partie différenciante
──────────────────────────────────────────
Pour chaque keyword:
  new_terms = keyword_terms - parent_terms

"qr code instagram"  → new_terms = [instagram]     ← WINNER
"instagram qr"       → new_terms = [instagram]
"lien qr instagram"  → new_terms = [lien, instagram]

ÉTAPE 4: Générer le slug
────────────────────────
slug = "instagram"
full_path = parent.full_path + "/" + slug
          = "/fr-FR/créer-qr-code/instagram"
```

### Cas Complexes

```
CAS 1: Brand qui ne se traduit pas
══════════════════════════════════
Entity:qr-instagram @ar-SA

Parent: /ar-SA/مولد-qr/
Child:  /ar-SA/مولد-qr/instagram   ← "instagram" reste en latin (brand)


CAS 2: Plusieurs niveaux de profondeur
══════════════════════════════════════
/fr-FR/créer-qr-code/instagram/stories
       └── qr-generator  └── instagram  └── instagram-stories

Chaque niveau n'ajoute que sa partie différenciante.


CAS 3: Keyword parfait mais répétitif
═════════════════════════════════════
EntityNative:qr-instagram@fr-FR TARGETS "qr code instagram" (vol: 50K)

Mais le slug ne sera PAS "qr-code-instagram" car:
├── Parent a déjà "qr-code"
└── Slug = "instagram" seulement

Le volume du keyword "qr code instagram" valide quand même le choix,
mais le slug extrait seulement la partie nouvelle.
```

### Schema PageNative (v2)

```yaml
PageNative:
  slug: string                    # URL segment (partie différenciante)
  slug_source: enum
    - "keyword:{key}"             # Directement d'un keyword
    - "extracted:{key}"           # Extrait d'un keyword (sans répétition)
    - "merged:{key1}+{key2}"      # Fusion de keywords
    - "generated"                 # Créé par le système
  slug_rationale: string          # Explication du choix
  full_path: string               # Calculé: parent.full_path + "/" + slug
  parent_terms_excluded: [string] # Termes du parent exclus du slug
```

### Règles SEO pour Slug Derivation

```
1. PAS DE RÉPÉTITION
   └─ Exclure les termes déjà dans parent.full_path

2. PARTIE DIFFÉRENCIANTE SEULEMENT
   └─ Le slug = ce qui est NOUVEAU par rapport au parent

3. RESPECTER LA LOCALE
   └─ Accents si autorisés (fr-FR: oui, en-US: non)
   └─ Direction RTL si applicable (ar-SA)

4. BRANDS INVARIANTS
   └─ "instagram", "facebook" ne se traduisent pas
   └─ Sauf si keyword natif a plus de volume (cas rare)

5. LONGUEUR OPTIMALE
   └─ Slug court > slug long (SEO)
   └─ Éviter plus de 3-4 mots par segment
```

## Session 5: Architecture SEO - Pillar/Cluster/Sitemap

### Hiérarchies Distinctes (Page vs Entity)

```
Entity.SUBTOPIC_OF = Hiérarchie SÉMANTIQUE (topic clusters SEO)
Page.SUBTOPIC_OF   = Hiérarchie URL (routing, navigation)
Page.CLUSTER_OF    = Hiérarchie SEO (pillar/cluster strategy)

SOUVENT IDENTIQUES, MAIS PAS TOUJOURS.

EXEMPLE OÙ ILS DIFFÈRENT:
─────────────────────────

Entity:instagram (pillar sémantique - la plateforme)
├── SUBTOPIC_OF ← Entity:instagram-marketing
└── SUBTOPIC_OF ← Entity:qr-instagram         ← Sémantiquement sous Instagram

Mais côté URL:

Page:qr-generator (notre outil principal)
└── SUBTOPIC_OF ← Page:qr-instagram           ← URL sous notre outil

Résultat:
  - Entity:qr-instagram SUBTOPIC_OF → Entity:instagram (sémantique)
  - Page:qr-instagram SUBTOPIC_OF → Page:qr-generator (URL)

full_path = /créer-qr-code/instagram (suit Page.SUBTOPIC_OF)
topic_cluster = Instagram (suit Entity.SUBTOPIC_OF)
```

### Structure Pillar/Cluster

```
PILLAR PAGE: Page:qr-generator (is_pillar: true)
══════════════════════════════════════════════════

                    ┌──────────────────────┐
                    │   PILLAR PAGE        │
                    │   /créer-qr-code     │
                    │                      │
                    │  Links vers tous     │
                    │  les clusters        │
                    └──────────┬───────────┘
                               │
           ┌───────────────────┼───────────────────┐
           │                   │                   │
           ▼                   ▼                   ▼
    ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
    │  CLUSTER    │     │  CLUSTER    │     │  CLUSTER    │
    │ /instagram  │     │  /wifi      │     │  /vcard     │
    │             │     │             │     │             │
    │ Link back → │     │ Link back → │     │ Link back → │
    │ pillar      │     │ pillar      │     │ pillar      │
    └─────────────┘     └─────────────┘     └─────────────┘

RÈGLE SEO: Chaque cluster DOIT linker vers son pillar
```

### Modèle Page avec Pillar/Cluster

```yaml
Page (org/structure, defined):
  key: string
  slug: string (invariant, english)
  is_pillar: boolean
  pillar_strategy: string         # Pour pillars: description stratégie

  relations:
    REPRESENTS: Entity (1:1)
    SUBTOPIC_OF: Page (parent URL)
    CLUSTER_OF: Page (pillar SEO)
    HAS_BLOCK: Block*

PageNative (org/output, generated):
  slug: string
  full_path: string
  pillar_link_anchor: string      # Texte du lien retour vers pillar
  cluster_position: integer       # Ordre dans le cluster
```

### Blocks et Maillage Interne

```
BlockInstruction contient des liens:
├── [@page:qr-instagram]          ← Lien vers cluster sibling
├── [@page:qr-wifi]
└── [@page:barcode-generator]     ← Lien vers related pillar

Agrégé au niveau Page:

Page ──[:LINKS_TO]──▶ Page
       {
         via_blocks: ["related-tools:1"],
         link_type: "cluster_sibling" | "pillar_backlink" | "related",
         anchor_text_source: "keyword:X" | "entity:X" | "custom"
       }
```

### Arcs SEO Structure

```
[:CLUSTER_OF]
├── Source: Page (cluster)
├── Target: Page (pillar)
├── Cardinality: N:1
├── Properties: { priority: integer }
└── "This page is part of this pillar's topic cluster"

[:LINKS_TO]
├── Source: Page
├── Target: Page
├── Cardinality: N:N
├── Properties:
│   ├── via_blocks: [string]
│   ├── link_type: enum
│   ├── anchor_text_source: string
│   └── nofollow: boolean
└── "Internal link from one page to another"

[:PILLAR_FOR]
├── Source: Page (pillar)
├── Target: Entity (topic)
├── Cardinality: 1:1
└── "This pillar page covers this topic entity"
```

### Sitemap SEO View

```
PROJECT SITEMAP VIEW
════════════════════

⬢ PILLAR: créer-qr-code (12 clusters)
│
├──● instagram ←→ [wifi, vcard, menu]
├──● wifi ←→ [instagram, vcard]
├──● vcard ←→ [instagram, wifi, linkedin]
└──● ...

⬢ PILLAR: scan-qr-code (8 clusters)
│
├──● iphone ←→ [android, camera]
└──● ...

⚠ ORPHANS (need pillar assignment):
├──○ mentions-legales
└──○ contact

LINK HEALTH METRICS:
├── All clusters link to pillar: ✓ 100%
├── Pillar links to all clusters: ✓ 100%
├── Avg internal links per page: 5.2
└── Broken links: 0
```
