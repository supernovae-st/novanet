# Session Brainstorming: Ontologie NovaNet + Workflows SEO

**Date:** 2026-03-03
**Participants:** Thibaut + Claude
**Objectif:** Comprendre et finaliser l'ontologie NovaNet pour générer du contenu natif SEO-optimisé pour 200 locales

---

## Contexte

On construit un système de génération de contenu NATIF (pas traduit) pour 200+ locales. Tout passe par:
- **NovaNet**: Knowledge graph (Neo4j) avec schema YAML modifiable
- **Nika**: Workflows qui peuplent Neo4j via `novanet_write` MCP

---

## Points Clés Établis

### 1. Génération NATIVE, pas traduction

```
❌ FAUX: Entity.denomination_forms.text "QR code" → traduire → "code QR"
✅ VRAI: SEO Research + Knowledge Atoms → découvrir ce que les gens CHERCHENT vraiment
```

Les `denomination_forms` de EntityNative ne viennent PAS d'une adaptation de l'anglais, mais de:
- Recherche SEO (DataForSEO) = ce que les gens cherchent
- Knowledge Atoms (Terms, Expressions) = comment on parle naturellement

### 2. Nika = Seul point d'entrée

Toutes les données passent par des workflows Nika:
- APIs externes (DataForSEO, Perplexity, Firecrawl) → Nika → `novanet_write` → Neo4j
- Le schema NovaNet peut être modifié si on découvre des besoins

### 3. Schema = Vivant, pas figé

On peut ajouter/modifier nodes, arcs, properties selon les besoins découverts.

---

## Questions et Réponses

### Q1: HAS_KEYWORD vs TARGETS - Lequel utiliser?

**Options:**
- A) `Entity --[:HAS_KEYWORD]--> SEOKeyword` (invariant → keyword)
- B) `EntityNative --[:TARGETS]--> SEOKeyword` (locale-specific → keyword)

**Réponse Thibaut:** Seulement B (TARGETS). Les keywords appartiennent à des KeywordSets par locale.

**Implication:** L'arc HAS_KEYWORD est peut-être redondant ou mal conçu. À revoir.

### Q2: Incohérence des patterns de clé Entity vs Page

**Constat:**
- Page: `page-qr-code` avec slug `qr-code`
- Entity: `qr-code` (sans préfixe)

**Question Thibaut:** Les deux sont invariants, ils devraient avoir la même forme?
- Option: `entity-qr-code` ou `entity:qr-code`?
- Les slugs localisés sont sur les *Native, pas sur les invariants

**À résoudre:** Harmoniser les patterns de clé entre Entity et Page

### Q3: PageNative = Assemblage de BlockNatives

**Compréhension:**
```
Page (invariant, structure)
  └── HAS_BLOCK --> Block[] (structure, position, block_type)
                      │
                      └── (per locale)
                            │
                            ▼
PageNative (generated) ──[:ASSEMBLES]──> BlockNative[] (dans le bon ordre)
```

PageNative = résultat de l'assemblage de tous les BlockNatives générés pour une locale.

---

## Architecture Actuelle (à valider/modifier)

```
NIVEAU 1: INVARIANTS (defined)
═══════════════════════════════

Entity "qr-code"              Page "page-qr-code"
  │                             │
  │  denomination_forms:        │  slug: "qr-code" (EN)
  │    text, title, abbrev      │
  │    (PAS de url!)            │
  │                             │
  └────[:REPRESENTED_BY]────────┘  (1:1, ADR-028)
            │
            └────[:HAS_BLOCK]────> Block (head-seo-meta, hero, features...)


NIVEAU 2: PER-LOCALE (authored/imported/generated)
══════════════════════════════════════════════════

Entity ──[:HAS_NATIVE]──> EntityNative "entity:qr-code@fr-FR"
                            │
                            ├── denomination_forms: text, title, abbrev, url (from SEO)
                            │
                            ├──[:FOR_LOCALE]──> Locale "fr-FR"
                            │
                            ├──[:TARGETS]──> SEOKeyword (N:N, avec rank + is_slug_source)
                            │
                            ├──[:USES_TERM]──> Term (temperature, context)
                            ├──[:USES_EXPRESSION]──> Expression
                            └──[:USES_PATTERN]──> Pattern

Page ──[:HAS_NATIVE]──> PageNative "page:qr-code@fr-FR"
                          │
                          ├──[:FOR_LOCALE]──> Locale "fr-FR"
                          │
                          └──[:ASSEMBLES]──> BlockNative[] (dans l'ordre)

Block ──[:HAS_NATIVE]──> BlockNative "block:head-seo-meta@page-qr-code@fr-FR"
                           │
                           ├── content: {slug, meta_title, meta_description}
                           │                ▲
                           │                │
                           │     FROM EntityNative.denomination_forms
                           │
                           └──[:INFLUENCED_BY]──> EntityNative
```

---

## Exploration Complète (2026-03-03)

### Arcs - 182 au total, 6 familles

| Famille | Count | But |
|---------|-------|-----|
| Ownership | 79 | Composition, hiérarchie, possession |
| Semantic | 62 | Relations entités, business logic |
| Generation | 12 | Provenance LLM, outputs |
| Localization | 20 | Hiérarchies locales, géo |
| Mining | 6 | SEO/GEO targeting |
| Schema | 3 | Instance → Class |

### Arcs Clés pour notre Flow

**EntityNative connections:**
- `USES_TERM` → Term (temperature, context) - charge ~50 termes pertinents
- `USES_EXPRESSION` → Expression (purpose, tone_match) - charge ~20 expressions
- `USES_PATTERN` → Pattern (usage_context) - charge ~20 patterns
- `USES_CULTURE_REF` → CultureRef (relevance) - charge ~10 refs culturelles
- `TARGETS` → SEOKeyword (rank, is_slug_source) - N:N, cross-realm
- `FOR_LOCALE` → Locale

**SEOKeyword connections:**
- `HAS_KEYWORD` ← Entity (rank, relevance) - PEUT-ÊTRE REDONDANT?
- `TARGETS` ← EntityNative (rank, is_slug_source)
- `CONTAINS_SEO_KEYWORD` ← SEOKeywordSet
- `HAS_SEO_KEYWORDS` ← Locale

### Knowledge Atoms - 6 types + 2 SEO/GEO

| Container | Atoms | Grouping |
|-----------|-------|----------|
| TermSet | Term | domain (7: pricing, features, technical...) |
| ExpressionSet | Expression | tous |
| PatternSet | Pattern | usage (cta, headline, body...) |
| CultureSet | CultureRef | type (holiday, event, tradition...) |
| TabooSet | Taboo | severity (critical, high, medium, low) |
| AudienceSet | AudienceTrait | segment (b2b_enterprise, b2c_mass...) |
| SEOKeywordSet | SEOKeyword | tous par locale |
| GEOQuerySet | GEOQuery | tous par locale |

### Pattern Locale → Atoms

```
Locale "fr-FR"
├─ [:HAS_TERMS]────────► TermSet ─[:CONTAINS_TERM]─► Term[]
├─ [:HAS_EXPRESSIONS]──► ExpressionSet ─[:CONTAINS_EXPRESSION]─► Expression[]
├─ [:HAS_PATTERNS]─────► PatternSet ─[:CONTAINS_PATTERN]─► Pattern[]
├─ [:HAS_CULTURE]──────► CultureSet ─[:CONTAINS_CULTURE_REF]─► CultureRef[]
├─ [:HAS_TABOOS]───────► TabooSet ─[:CONTAINS_TABOO]─► Taboo[]
├─ [:HAS_AUDIENCE]─────► AudienceSet ─[:CONTAINS_AUDIENCE_TRAIT]─► AudienceTrait[]
└─ [:HAS_SEO_KEYWORDS]─► SEOKeywordSet ─[:CONTAINS_SEO_KEYWORD]─► SEOKeyword[]
```

**Principe clé:** USES_* arcs permettent à EntityNative de charger SÉLECTIVEMENT des atoms (50 Terms au lieu de 20K blob)

---

## Décisions Prises

### D1: Key Patterns Harmonisés (Option B)
```
Entity: "qr-code" → "entity:qr-code"
Page: "page-qr-code" → "page:qr-code"
```
Cohérent avec EntityNative "entity:qr-code@fr-FR"

### D2: TARGETS Only, Supprimer HAS_KEYWORD
```
❌ Entity --[:HAS_KEYWORD]--> SEOKeyword  (SUPPRIMER - incohérent)
✅ EntityNative --[:TARGETS]--> SEOKeyword (GARDER - correct)
```
Raison: Entity invariant ne peut pas "posséder" des keywords locale-specific.

### D3: Slug SEULEMENT sur *Native
```
Page (invariant): PAS de slug (structure pure)
Entity (invariant): PAS de slug (concept pur)
EntityNative: denomination_forms.url (REQUIRED, vient du SEO)
BlockNative:head-seo-meta: content.slug (copie de EntityNative)
```

### D4: Le slug est un CHOIX du workflow (pas une propriété fixe)
```
❌ FAUX: TARGETS arc avec is_slug_source=true (propriété pré-définie)
✅ VRAI: Workflow Nika analyse tous les TARGETS + température et CHOISIT le slug

EntityNative
  │
  ├──[:TARGETS]──> SEOKeyword₁ (température: 0.9, volume: 5000)
  ├──[:TARGETS]──> SEOKeyword₂ (température: 0.7, volume: 8000)
  ├──[:TARGETS]──> SEOKeyword₃ (température: 0.5, volume: 2000)
  │
  └── Workflow Nika: analyse → décision → SET denomination_forms.url
```

### D5: Slugification Rules = Chargées depuis la Locale
```
Le slug n'est PAS juste un choix de keyword — il doit RESPECTER les règles de slugification de la locale!

Locale "fr-FR"
  │
  └──[:HAS_SLUGIFICATION]──> Slugification
                                │
                                ├── slug_rule: latin_preserve | latin_strip | native_script | transliterate
                                ├── stop_words: ["le", "la", "les", "de", "du", ...]
                                ├── transliteration_map: {"é": "e", "è": "e", ...}
                                ├── preserve_diacritics: true | false
                                └── max_slug_length: 60

WORKFLOW SLUG SELECTION:
1. Charger Slugification depuis Locale
2. Pour chaque keyword candidat TARGETS:
   - Appliquer slug_rule
   - Retirer stop_words
   - Appliquer transliteration_map (si slug_rule = transliterate)
   - Tronquer à max_slug_length
3. Scorer les candidats (volume, intent, longueur finale)
4. CHOISIR et SET EntityNative.denomination_forms.url
```

### D6: Locale Knowledge = Contexte de Génération
```
Quand on génère du contenu (BlockNative), on doit charger TOUTE la connaissance locale:

Locale "fr-FR"
  │
  ├──[:HAS_SLUGIFICATION]──> Slugification     (règles de slug)
  ├──[:HAS_STYLE]──────────> LocaleStyle       (guide de style: tone, formality, sentence_length)
  ├──[:HAS_CULTURE]────────> LocaleCulture     (contexte culturel: holidays, taboos, norms)
  ├──[:HAS_FORMATTING]─────> LocaleFormatting  (format: date, currency, numbers)
  ├──[:HAS_VOICE]──────────> LocaleVoice       (voix: pronoun_usage, politeness_level)
  │
  ├──[:HAS_TERMS]──────────> TermSet ──> Term[]
  ├──[:HAS_EXPRESSIONS]────> ExpressionSet ──> Expression[]
  ├──[:HAS_PATTERNS]───────> PatternSet ──> Pattern[]
  ├──[:HAS_CULTURE]────────> CultureSet ──> CultureRef[]
  ├──[:HAS_TABOOS]─────────> TabooSet ──> Taboo[]
  └──[:HAS_SEO_KEYWORDS]───> SEOKeywordSet ──> SEOKeyword[]

LE PROMPT DE GÉNÉRATION INCLUT:
- LocaleVoice → "Tu-tutoiement, style direct mais poli"
- LocaleStyle → "Phrases courtes, pas de passive"
- Taboos → "ÉVITER: références politiques, comparaisons culturelles"
- Terms/Expressions → Vocabulaire naturel de la locale
```

---

## Architecture des Workflows Nika

### Niveau 1: Workflows Atomiques

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  WORKFLOW 1: Bootstrap Entity (one-time)                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│  INPUT:  Concept à définir                                                  │
│  OUTPUT: Entity (invariant) + Page (invariant) + Blocks (invariant)         │
│                                                                             │
│  1. Créer Entity "entity:qr-code"                                           │
│     - denomination_forms: text, title, abbrev (EN seulement)                │
│     - PAS de url (vient du SEO par locale)                                  │
│                                                                             │
│  2. Créer Page "page:qr-code"                                               │
│     - REPRESENTS → Entity                                                   │
│     - PAS de slug (vient de EntityNative)                                   │
│                                                                             │
│  3. Créer Blocks (head-seo-meta, hero, features...)                         │
│     - HAS_BLOCK → Page                                                      │
│     - OF_TYPE → BlockType                                                   │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│  WORKFLOW 2: SEO Discovery (per locale)                                     │
├─────────────────────────────────────────────────────────────────────────────┤
│  INPUT:  Entity + Locale                                                    │
│  OUTPUT: SEOKeywords + EntityNative avec slug                               │
│                                                                             │
│  1. CHARGER CONTEXTE LOCALE (via novanet_traverse)                          │
│     - Locale → HAS_SLUGIFICATION → Slugification                            │
│     - Locale → HAS_VOICE → LocaleVoice                                      │
│     - Locale → HAS_STYLE → LocaleStyle                                      │
│                                                                             │
│  2. DataForSEO API → découvrir keywords pour cette locale                   │
│     - Volume, difficulté, intent                                            │
│                                                                             │
│  3. Créer SEOKeyword nodes                                                  │
│     - CONTAINS_SEO_KEYWORD ← SEOKeywordSet ← Locale                         │
│                                                                             │
│  4. Créer/Update EntityNative "entity:qr-code@fr-FR"                        │
│     - FOR_LOCALE → Locale                                                   │
│     - TARGETS → SEOKeyword[] (avec température)                             │
│                                                                             │
│  5. CHOISIR le slug (décision LLM avec Slugification rules!)                │
│     a. Pour chaque keyword TARGETS:                                         │
│        - Appliquer Slugification.slug_rule                                  │
│        - Retirer Slugification.stop_words                                   │
│        - Appliquer Slugification.transliteration_map (si applicable)        │
│        - Tronquer à Slugification.max_slug_length                           │
│     b. Scorer: volume × intent × longueur × brandabilité                    │
│     c. SET EntityNative.denomination_forms.url                              │
│                                                                             │
│  6. Générer les autres denomination_forms                                   │
│     - text: forme naturelle dans la locale                                  │
│     - title: forme titre (majuscules appropriées)                           │
│     - abbrev: forme abrégée                                                 │
│                                                                             │
│  7. Lier Knowledge Atoms                                                    │
│     - USES_TERM → Term[] pertinents                                         │
│     - USES_EXPRESSION → Expression[] pertinentes                            │
│     - USES_PATTERN → Pattern[] pertinents                                   │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│  WORKFLOW 3: Content Generation (per locale)                                │
├─────────────────────────────────────────────────────────────────────────────┤
│  INPUT:  Page + EntityNative + Locale                                       │
│  OUTPUT: PageNative assemblé de BlockNatives                                │
│                                                                             │
│  1. CHARGER CONTEXTE LOCALE COMPLET                                         │
│     a. Style & Voice (toujours chargés):                                    │
│        - LocaleVoice: pronoun_usage, politeness_level, formality            │
│        - LocaleStyle: sentence_length, passive_voice, contractions          │
│        - LocaleFormatting: date_format, currency_symbol, number_format      │
│     b. Safety (toujours chargé):                                            │
│        - Taboos via TabooSet: severity=critical TOUJOURS respectés          │
│     c. Content via EntityNative (déjà lié):                                 │
│        - USES_TERM → Terms pertinents (temperature)                         │
│        - USES_EXPRESSION → Expressions pertinentes                          │
│        - USES_PATTERN → Patterns pertinents                                 │
│                                                                             │
│  2. Pour chaque Block de la Page:                                           │
│     a. Construire prompt avec:                                              │
│        - BlockType.instructions + BlockType.rules                           │
│        - EntityNative.denomination_forms (text, title, url)                 │
│        - LocaleVoice + LocaleStyle directives                               │
│        - Terms/Expressions pertinentes                                      │
│     b. Générer BlockNative (LLM)                                            │
│     c. Pour head-seo-meta: COPIER slug de EntityNative.denomination_forms   │
│     d. INFLUENCED_BY → EntityNative                                         │
│                                                                             │
│  3. Créer PageNative "page:qr-code@fr-FR"                                   │
│     - FOR_LOCALE → Locale                                                   │
│     - ASSEMBLES → BlockNative[] (dans l'ordre)                              │
│                                                                             │
│  4. Valider cohérence                                                       │
│     - Slug match entre EntityNative et BlockNative:head-seo-meta            │
│     - Tous les blocks générés                                               │
│     - Aucun Taboo severity=critical violé                                   │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Niveau 2: Orchestration (Workflows de Workflows)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  META-WORKFLOW: Full Entity Pipeline                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│  INPUT:  Entity key + Liste de locales cibles                               │
│  OUTPUT: Contenu complet pour toutes les locales                            │
│                                                                             │
│  ORCHESTRATION:                                                             │
│                                                                             │
│  ┌──────────────────┐                                                       │
│  │ 1. Bootstrap     │ (once)                                                │
│  │    Entity        │────────────────────────────────────────┐              │
│  └──────────────────┘                                        │              │
│           │                                                  │              │
│           ▼                                                  ▼              │
│  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐          │
│  │ 2. SEO Discovery │  │ 2. SEO Discovery │  │ 2. SEO Discovery │          │
│  │    fr-FR         │  │    en-US         │  │    es-MX         │  ...     │
│  └──────────────────┘  └──────────────────┘  └──────────────────┘          │
│           │                     │                     │                     │
│           ▼                     ▼                     ▼                     │
│  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐          │
│  │ 3. Generation    │  │ 3. Generation    │  │ 3. Generation    │          │
│  │    fr-FR         │  │    en-US         │  │    es-MX         │  ...     │
│  └──────────────────┘  └──────────────────┘  └──────────────────┘          │
│                                                                             │
│  PARALLÉLISATION:                                                           │
│  - Étape 2: Toutes les locales en parallèle (concurrency: 10)               │
│  - Étape 3: Toutes les locales en parallèle (concurrency: 5)                │
│  - Fail-fast: false (continuer même si une locale échoue)                   │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│  META-WORKFLOW: Full Project Pipeline                                       │
├─────────────────────────────────────────────────────────────────────────────┤
│  INPUT:  Project key + Liste d'entities + Liste de locales                  │
│  OUTPUT: Tout le contenu du projet pour toutes les locales                  │
│                                                                             │
│  Pour chaque Entity du projet:                                              │
│    └── Appeler "Full Entity Pipeline"                                       │
│                                                                             │
│  Avec dépendances:                                                          │
│    - Entity pillar d'abord                                                  │
│    - Puis entities enfants (respecter SUBTOPIC_OF)                          │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Niveau 3: Usages Avancés de Nika

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  NIKA FEATURES UTILISÉES                                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. INVOKE (MCP tools)                                                      │
│     - novanet_write: créer/modifier nodes et arcs                           │
│     - novanet_query: lire données existantes                                │
│     - novanet_traverse: naviguer le graphe                                  │
│     - novanet_atoms: charger Knowledge Atoms                                │
│     - dataforseo_*: recherche SEO                                           │
│                                                                             │
│  2. INFER (LLM)                                                             │
│     - Adapter denomination_forms à la locale                                │
│     - Choisir le meilleur slug parmi les keywords                           │
│     - Générer contenu de blocks                                             │
│     - extended_thinking: true pour décisions complexes                      │
│                                                                             │
│  3. AGENT (multi-turn)                                                      │
│     - Validation SEO: vérifier cohérence keywords/slug                      │
│     - Quality check: valider contenu généré                                 │
│                                                                             │
│  4. FOR_EACH (parallélisation)                                              │
│     - Traiter N locales en parallèle                                        │
│     - Traiter N keywords en parallèle                                       │
│     - concurrency: configurable                                             │
│     - fail_fast: false pour résilience                                      │
│                                                                             │
│  5. INCLUDE (DAG fusion)                                                    │
│     - Partials réutilisables (setup, validation)                            │
│     - Composition de workflows                                              │
│                                                                             │
│  6. CONTEXT (fichiers)                                                      │
│     - Brand guidelines                                                      │
│     - SEO personas                                                          │
│     - Locale-specific rules                                                 │
│                                                                             │
│  7. SKILLS (prompts réutilisables)                                          │
│     - seo-scoring.md                                                        │
│     - content-generation.md                                                 │
│     - slug-selection.md                                                     │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Modifications à Faire

### NovaNet Schema (brain/models/)

1. **Key patterns** - Modifier les regex dans:
   - `node-classes/org/semantic/entity.yaml`
   - `node-classes/org/structure/page.yaml`

2. **Supprimer HAS_KEYWORD**:
   - `arc-classes/ownership/has-keyword.yaml`
   - `arc-classes/ownership/keyword-of.yaml` (inverse)

3. **Supprimer slug de Page**:
   - `node-classes/org/structure/page.yaml` - retirer property "slug"

4. **Mettre à jour références**:
   - Arcs qui référencent Entity/Page
   - Seeds Cypher
   - Tests

### Nika Workflows

1. Utiliser nouveaux key patterns
2. Ne PAS créer HAS_KEYWORD
3. Toujours set EntityNative.denomination_forms.url

---

## Points à Résoudre

1. [x] **Harmoniser Entity/Page key patterns** → Option B (`entity:xxx`, `page:xxx`)
2. [x] **Supprimer HAS_KEYWORD** → Oui, garder TARGETS seulement
3. [x] **Slug sur invariants** → Non, seulement sur *Native
4. [x] **Slugification rules liées** → D5 documenté (Locale → Slugification)
5. [x] **Locale knowledge liée** → D6 documenté (Voice, Style, Formatting, Taboos)
6. [x] **Valider le flow complet des 3 workflows** → Mis à jour avec chargement contexte
7. [ ] **Exécuter les modifications schema** → En cours
   - [x] Entity key pattern modifié
   - [x] Page key pattern modifié
   - [x] HAS_KEYWORD arc supprimé
   - [x] KEYWORD_OF arc supprimé
   - [x] Page.slug property supprimé
   - [x] TypeScript references mis à jour
   - [ ] Seeds Cypher à mettre à jour
   - [ ] Tests à mettre à jour
8. [ ] **Créer les workflows Nika** → À faire après validation schema

---

## Rappel Important

**LE SCHEMA EST MODIFIABLE!** On peut:
- Ajouter/supprimer des arcs
- Changer les patterns de clé
- Ajouter des properties
- Restructurer si ça a plus de sens

---

## Prochaines Étapes

1. [x] Brainstorm terminé
2. [x] Schema YAML modifié (voir D7, D8)
3. [ ] Exécuter migration Neo4j: `010-v016-cleanup-instances.cypher`
4. [ ] Créer les workflows Nika corrects
5. [ ] Tester sur fr-FR (POC)

---

## Changements v0.16 Effectués

### D7: Suppression des NodeClasses flous
```
SUPPRIMÉS (concepts pas clairs pour le moment):
├── AudiencePersona (org/semantic) — personas marketing
├── ChannelSurface (org/semantic) — canaux de distribution
├── TARGETS_PERSONA arc
└── FOR_CHANNEL arc

On peut les remettre plus tard si besoin.
```

### D8: Suppression HAS_KEYWORD + KEYWORD_OF
```
SUPPRIMÉS (remplacés par TARGETS sur EntityNative):
├── HAS_KEYWORD (ownership) — Entity → SEOKeyword
└── KEYWORD_OF (semantic) — inverse

CORRECT: EntityNative --[:TARGETS]--> SEOKeyword (avec is_slug_source)
```

### Schema Stats v0.16
```
Nodes:  59 (was 61, -AudiencePersona, -ChannelSurface)
Arcs:   178 (was 182, -HAS_KEYWORD, -KEYWORD_OF, -TARGETS_PERSONA, -FOR_CHANNEL)
```

### Migration Neo4j
```
Migration: packages/db/migrations/010-v016-cleanup-instances.cypher

GARDE:
└── Entity: qr-code, qr-code-generator, qr-code-scanner,
            qr-code-wifi, qr-code-menu, custom-qr-code, qr-code-art

SUPPRIME:
├── EntityNative (ALL)
├── Page (ALL)
├── PageNative (ALL)
├── BlockNative (ALL)
├── Block (ALL)
├── Entity (sauf les 7 ci-dessus)
├── AudiencePersona (ALL)
└── ChannelSurface (ALL)
```

---

*Session en cours...*
