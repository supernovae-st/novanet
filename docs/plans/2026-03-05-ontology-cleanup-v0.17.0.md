# Ontology Cleanup v0.17.0

**Date**: 2026-03-05
**Status**: ✅ Brainstorming terminé - Prêt pour implémentation
**Authors**: Thibaut + Claude

## Résumé

Refonte majeure de l'ontologie NovaNet pour:
1. Simplifier les nodes (supprimer propriétés redondantes)
2. Ajouter traçabilité workflow Nika (workflow_id sur tous les nodes)
3. Clarifier les workflows Nika (Entity → EntityNative)
4. Améliorer la structure SEOKeyword (shared + project)
5. Consolider les semantic arcs (59 → ~10)
6. Ajouter les arcs inverses manquants

**Principe clé**: Tout passe par Nika. Le trait = source des données, workflow_id = traçabilité.

---

## Décisions Finalisées

### D1: Graph-Only RAG (suppression embeddings)

**Décision**: Supprimer `embedding_properties` de tous les nodes.

**Avant**:
```yaml
embedding_properties:
  embedding:
    type: vector
    dimensions: 1536
  embedding_source:
    type: string
  embedding_updated_at:
    type: datetime
```

**Après**: Supprimé de tous les nodes.

**Raison**:
- Nika + MCP tools font le retrieval via graph traversal
- Relations sémantiques riches (SEMANTIC_LINK avec 12 link_types)
- novanet_traverse, novanet_search, novanet_generate suffisent
- Simplifie le schema, élimine coût embeddings

**Impact**: ~20 nodes à modifier (tous ceux avec embedding_properties)

---

### D2: Entity Simplifié + Traçabilité Workflow

**Décision**: Entity = définition technique + relations riches + traçabilité Nika.

**Propriétés GARDÉES**:
| Propriété | Type | Description |
|-----------|------|-------------|
| `key` | string | `entity:qr-code` |
| `display_name` | string | "QR Code" (EN) |
| `description` | string | Définition technique pure |
| `denomination_forms` | array | text/title/abbrev (EN only, no url) |
| `is_pillar` | boolean | Pillar content? |
| `schema_org_type` | string | Schema.org type |
| `created_at` | datetime | - |
| `updated_at` | datetime | - |

**Propriétés AJOUTÉES** (traçabilité Nika):
| Propriété | Type | Description |
|-----------|------|-------------|
| `audience_segment` | enum | professional/consumer/developer/enterprise (déplacé depuis EntityNative) |
| `workflow_id` | string | Workflow Nika qui a créé ce node |
| `workflow_run_id` | string? | ID de l'exécution spécifique |

**Propriétés SUPPRIMÉES**:
| Propriété | Raison |
|-----------|--------|
| `entity_summary` | Redondant avec `description` |
| `llm_context` (instance) | Contexte = relations du graphe |
| `embedding_*` | Graph-only RAG (D1) |

**Relations Entity** (inchangées, c'est ici que vit le contexte):
- `HAS_NATIVE` → EntityNative
- `BELONGS_TO` → EntityCategory
- `SEMANTIC_LINK` → Entity (12 link_types)
- `HAS_CHILD` / `SUBTOPIC_OF` → Entity
- `REPRESENTED_BY` → Page
- `POPULAR_IN` → GeoRegion

---

### D3: SEOKeywordSet Structure (Shared + Project)

**Décision**: Deux niveaux de SEOKeywordSet.

**Structure**:
```
NIVEAU SHARED (importé DataForSEO)
────────────────────────────────────
Locale (fr-FR)
  └── HAS_SEO_KEYWORDS ──► SEOKeywordSet (shared)
        key: "seo-keywords@fr-FR"
        └── CONTAINS ──► SEOKeyword (15,000+ keywords)

NIVEAU PROJECT (curated)
────────────────────────────────────
Project (qrcode-ai)
  ├── SUPPORTS_LOCALE ──► Locale
  └── HAS_SEO_KEYWORDS ──► SEOKeywordSet (project)
        key: "seo-keywords:qrcode-ai@fr-FR"
        ├── FOR_LOCALE ──► Locale (fr-FR)
        └── CONTAINS ──► SEOKeyword (500 curated)

ASSOCIATION
────────────────────────────────────
EntityNative -[:TARGETS]-> SEOKeyword
(depuis shared OU project pool)
```

**Changements schema**:
1. `SEOKeywordSet`: ajouter incoming `HAS_SEO_KEYWORDS` from Project
2. `SEOKeywordSet`: ajouter outgoing `FOR_LOCALE` to Locale
3. `Project`: ajouter outgoing `HAS_SEO_KEYWORDS` to SEOKeywordSet
4. Key format: `seo-keywords:${project}@${locale}` pour project sets

**Workflow Nika - Trouver keywords pour EntityNative**:
1. Via PILLAR: traverser `SUBTOPIC_OF` → pillar EntityNative → TARGETS
2. Via PROJECT: Project → HAS_SEO_KEYWORDS → SEOKeywordSet → CONTAINS
3. Via FULLTEXT: recherche dans le pool shared
4. LLM choisit les meilleurs → crée TARGETS

---

### D4: EntityNative Refonte Complète

**Décision**: EntityNative = package complet localisé, OUTPUT de Nika.

**Changements majeurs**:
- **trait**: `authored` → `generated` (Nika génère)
- **Ajout**: `workflow_id`, `workflow_run_id`, `jsonld`
- **Suppression**: `curation_status`, `llm_context`, `audience_segment`, `cultural_notes`, `embedding_*`
- **Nouvelle relation**: `USES_CULTURE_REF` → CultureRef (remplace `cultural_notes` string)

**SPEC COMPLÈTE EntityNative v0.17.0**:

```yaml
node:
  name: EntityNative
  realm: org
  layer: semantic
  trait: generated  # ← CHANGÉ (était "authored")

  # STANDARD PROPERTIES
  standard_properties:
    key:
      type: string
      pattern: "^entity:[a-z][a-z0-9-]*@[a-z]{2}-[A-Z]{2}$"
      example: "entity:qr-code@fr-FR"
    entity_key:
      type: string
      description: "Parent Entity key (denorm)"
    locale_key:
      type: string
      description: "Target Locale key (denorm)"
    display_name:
      type: string
      description: "Localized display name"
    description:
      type: string
      description: "Short description (for meta)"
    created_at:
      type: datetime
    updated_at:
      type: datetime

  # CONTENT PROPERTIES (OUTPUT généré par Nika)
  properties:
    definition:
      type: string
      description: "Ce que c'est - detailed localized definition"

    purpose:
      type: string
      description: "Pourquoi ça existe - localized purpose"

    benefits:
      type: string[]
      description: "Value propositions localisées"

    usage_examples:
      type: string[]
      description: "Exemples concrets localisés"

    denomination_forms:
      type: array
      required: true
      description: |
        Official denomination forms for this entity in this locale.
        text   → prose and body content
        title  → H1, H2, meta_title
        abbrev → after first mention
        url    → URL-safe slug (generated by Nika using Slugification rules)
      items:
        type: object
        properties:
          type: { enum: [text, title, abbrev, url] }
          value: { type: string }
          priority: { type: int, default: 1 }

    # WORKFLOW PROPERTIES (traçabilité Nika)
    status:
      type: string
      enum: [draft, reviewed, published]
      default: draft
      description: "Workflow status"

    workflow_id:
      type: string
      required: true
      description: "Workflow Nika qui a créé ce node"
      examples:
        - "entity-native-bootstrap"
        - "seo-discovery-modular"
        - "content-generation"

    workflow_run_id:
      type: string
      required: false
      description: "ID de l'exécution spécifique (audit/debug)"

    version:
      type: int
      required: true
      description: "Content version for staleness detection"

    # SEO PROPERTIES
    jsonld:
      type: json
      required: false
      description: "Schema.org JSON-LD for this entity in this locale"
      example: |
        {
          "@context": "https://schema.org",
          "@type": "Product",
          "name": "QR Code WiFi",
          "description": "Créez un QR code pour partager..."
        }

  # RELATIONS
  relations:
    outgoing:
      - type: FOR_LOCALE
        to: Locale
        cardinality: "N:1"
      - type: TARGETS
        to: SEOKeyword
        cardinality: "N:N"
      - type: USES_TERM
        to: Term
        cardinality: "N:N"
      - type: USES_EXPRESSION
        to: Expression
        cardinality: "N:N"
      - type: USES_PATTERN
        to: Pattern
        cardinality: "N:N"
      - type: USES_CULTURE_REF  # ← NOUVEAU (remplace cultural_notes)
        to: CultureRef
        cardinality: "N:N"

    incoming:
      - type: HAS_NATIVE
        from: Entity
        cardinality: "1:N"
```

**Propriétés SUPPRIMÉES**:
| Propriété | Raison |
|-----------|--------|
| `llm_context` (instance) | Contexte = relations du graphe |
| `curation_status` | Redondant: trait=generated + status suffisent |
| `audience_segment` | Déplacé sur Entity (invariant) |
| `cultural_notes` | Remplacé par relation `USES_CULTURE_REF` |
| `embedding_*` | Graph-only RAG (D1) |

---

### Récap Traits ADR-024 (mise à jour v0.17.0)

| Trait | Qui Crée | Exemples v0.17.0 |
|-------|----------|------------------|
| **defined** | Humain, une fois | Entity, Page, Block, Locale, Project |
| **authored** | Humain, par locale | ProjectNative (brand voice écrit par humain) |
| **imported** | Données externes | Term, SEOKeyword, CultureRef, Expression |
| **generated** | LLM/Nika | EntityNative, PageNative, BlockNative |
| **retrieved** | APIs externes | SEOKeywordMetrics, GEOAnswer |

**Changement**: EntityNative passe de `authored` → `generated`

**Note**: Même si tout passe par Nika, le trait indique la SOURCE des données, pas l'orchestrateur.

---

### D5: Locale Config Layer (Boundaries Clarifiées)

**Décision**: Séparer clairement les responsabilités, supprimer Market.

**Structure v0.17.0**:
```
Locale ─┬─[:HAS_FORMATTING]────► Formatting    (dates, numbers, currency)
        ├─[:HAS_SLUGIFICATION]─► Slugification (URL rules)
        ├─[:HAS_CULTURE]───────► Culture       (saisons, fêtes, valeurs)
        ├─[:HAS_STYLE]─────────► Style         (formality, pronoun, tone)
        └─[:HAS_ADAPTATION]────► Adaptation    (FACT vs ILLUSTRATION)
```

**SUPPRIMÉ**: Market (analytics, pas LLM-context)

**Boundaries clarifiées**:

| Node | Responsabilité | Ce qu'il contient |
|------|----------------|-------------------|
| **Formatting** | Règles TECHNIQUES | dates, numbers, currency, phone, address |
| **Slugification** | Règles URL | slug_rule, stop_words, transliteration |
| **Culture** | Ce que la culture EST | saisons, fêtes, valeurs, business_hours |
| **Style** | Comment on ÉCRIT | formality, pronoun, directness, warmth, punctuation |
| **Adaptation** | Comment on ADAPTE | FACT vs ILLUSTRATION, technical_terms |

**Changements**:

1. **Culture** - SUPPRIME propriétés déplacées:
   - `communication_directness` → Style
   - `hierarchy_importance` → Style
   - `individualism_level` → Style

2. **Style** - SIMPLIFIE (~40 → ~15 props):
   - AJOUTE: `hierarchy_importance`, `individualism_level`
   - GARDE: formality_*, directness_*, warmth_*, pronoun_*, punctuation_rules
   - SUPPRIME: propriétés dupliquées, détails dans raw_markdown

3. **Market** - SUPPRIMÉ:
   - Node supprimé
   - Arc `HAS_MARKET` supprimé

---

## Décisions En Cours

### D6: Consolidation Semantic Arcs (59 → 12)

**Décision**: Consolider à 12 arcs sémantiques essentiels.

**TIER 1 - Entity Relations (7 arcs, 14 avec inverses)**:

| Arc | Inverse | Usage |
|-----|---------|-------|
| `TYPE_OF` | `HAS_TYPE` | Taxonomy (is-a) |
| `VARIANT_OF` | `HAS_VARIANT` | Variants (WiFi QR → QR Code) |
| `SUBTOPIC_OF` | `HAS_SUBTOPIC` | Pillar/cluster SEO |
| `REQUIRES` | `REQUIRED_BY` | Dependencies |
| `ENABLES` | `ENABLED_BY` | Capabilities |
| `SIMILAR_TO` | (symmetric) | Similarity |
| `REPRESENTS` | `REPRESENTED_BY` | Page ↔ Entity |

**TIER 2 - Knowledge Atoms (4 arcs, 8 avec inverses)**:

| Arc | Inverse | Usage |
|-----|---------|-------|
| `USES_TERM` | `TERM_USED_BY` | Terms in content |
| `USES_EXPRESSION` | `EXPRESSION_USED_BY` | Expressions in content |
| `USES_PATTERN` | `PATTERN_USED_BY` | Patterns in content |
| `USES_CULTURE_REF` | `CULTURE_REF_USED_BY` | Cultural references |

**TIER 3 - Geographic (1 arc)**:

| Arc | Inverse | Usage |
|-----|---------|-------|
| `POPULAR_IN` | - | Geographic popularity |

**FUSIONS**:

| Arc Supprimé | Fusionné Dans | Propriété |
|--------------|---------------|-----------|
| `ALTERNATIVE_TO` | `SIMILAR_TO` | `similarity_type: "functional"` |
| `COMPETES_WITH` | `SIMILAR_TO` | `similarity_type: "competitive"` |
| `CULTURALLY_SIMILAR` | `SIMILAR_TO` | `similarity_type: "cultural"` |
| `ENHANCES` | `ENABLES` | `enable_type: "enhance"` |
| `INCLUDES` | `TYPE_OF` | (direction inversée) |

**SUPPRESSIONS**:

| Arc | Raison |
|-----|--------|
| `SEMANTIC_LINK` | Remplacé par arcs typés |
| `REFERENCES_ENTITY` | Consolider en `REFERENCES` |
| `REFERENCES_PAGE` | Consolider en `REFERENCES` |
| `MENTIONS`, `MENTIONS_BRAND` | Consolider en `REFERENCES` |
| ~30 autres arcs | Rarement utilisés |

**Résultat**: 59 → 12 arcs (+ 11 inverses = 23 total)

---

### D7: Arcs Inverses (Résolu via D6)

**Décision**: Inverses couverts par D6 + patterns ownership existants.

**Semantic Arcs (D6)**: 11 inverses définis pour 12 arcs.

**Ownership Arcs**: Pattern HAS_*/OF déjà cohérent:
- `HAS_NATIVE` / `NATIVE_OF` ✓
- `HAS_PAGE` / `PAGE_OF` ✓
- `HAS_ENTITY` / `ENTITY_OF` ✓
- `HAS_BLOCK` / `BLOCK_OF` ✓
- `HAS_TERMS` / `TERMS_OF` ✓

**Localization Arcs**:
- `FOR_LOCALE` (pas besoin d'inverse, direction claire)
- `SUPPORTS_LOCALE` (pas besoin d'inverse)

**Résultat**: Avec D6, les arcs critiques ont leurs inverses. Les arcs ownership suivent déjà le pattern.

---

### D8: Universal Workflow Traceability

**Décision**: `workflow_id` sur TOUS les nodes créés par Nika (pas seulement Entity/EntityNative).

**Principe**: "Tout passe par Nika" - chaque node créé automatiquement a une traçabilité workflow.

**Propriétés standard ajoutées à TOUS les nodes**:

```yaml
# WORKFLOW TRACEABILITY (standard sur tous les nodes)
workflow_properties:
  workflow_id:
    type: string
    required: false  # Null si créé manuellement (TUI, import direct)
    description: "Workflow Nika qui a créé/modifié ce node"
    examples:
      - "00-entity-native-bootstrap"
      - "06-seo-discovery-modular"
      - "07-content-generation"
      - "onboarding"
      - null  # Si créé via TUI ou import manuel

  workflow_run_id:
    type: string
    required: false
    description: "ID d'exécution spécifique (audit/debug)"
```

**Nodes impactés par trait**:

| Trait | Nodes | workflow_id requis? |
|-------|-------|---------------------|
| **defined** | Entity, Page, Block, Project | Optionnel (peut être créé via TUI) |
| **authored** | ProjectNative | Optionnel (contenu humain) |
| **imported** | SEOKeyword, Term, Expression, CultureRef | Recommandé (trace l'import) |
| **generated** | EntityNative, PageNative, BlockNative | REQUIS (toujours Nika) |
| **retrieved** | SEOKeywordMetrics, GEOAnswer | Recommandé (trace la requête) |

**Workflows Nika identifiés** (du brainstorm précédent):

| Workflow | Nodes créés | workflow_id |
|----------|-------------|-------------|
| `00-entity-native-bootstrap` | EntityNative | `00-entity-native-bootstrap` |
| `06-seo-discovery-modular` | SEOKeyword (curated), EntityNative.TARGETS | `06-seo-discovery-modular` |
| `07-content-generation` | PageNative, BlockNative | `07-content-generation` |
| `onboarding` | Entity, EntityNative, relationships | `onboarding` |
| `seo-content-sprint` | PageNative, BlockNative (SEO) | `seo-content-sprint` |
| `entity-knowledge-retrieval` | (read-only) | N/A |
| `block-generation-locale-aware` | BlockNative | `block-generation-locale-aware` |
| `semantic-content-planning` | (planning only) | N/A |

**Impact implémentation**:

1. **Créer `workflow_properties` mixin** dans schema standard
2. **L'ajouter à tous les node YAMLs** comme propriété optionnelle
3. **Nika DOIT toujours setter workflow_id** quand il crée un node
4. **novanet_write MCP tool** accepte workflow_id en paramètre

**Avantages**:
- Audit trail complet: "qui a créé quoi et quand"
- Debug facile: retrouver le workflow qui a généré un contenu problématique
- Staleness detection: version + workflow_id = savoir si régénérer
- Analytics: quel workflow génère le plus de contenu

---

## Workflows Nika

### Workflow 1: Créer Entity (invariant)

```
INPUT
├── display_name (EN): "WiFi QR Code"
├── description (EN): "QR code that connects devices to WiFi"
├── denomination_forms (EN): text/title/abbrev
└── audience_segment: "consumer"

NIKA TRAVERSE
├── Project → où rattacher
├── EntityCategory → classification
├── Entity existants → SEMANTIC_LINK (type_of, enables, requires...)
└── Entity pillar → SUBTOPIC_OF si cluster

OUTPUT
├── CREATE (e:Entity {
│     ...,
│     workflow_id: "entity-bootstrap",
│     workflow_run_id: "run_xxx"
│   })
├── CREATE (p)-[:HAS_ENTITY]->(e)
├── CREATE (e)-[:BELONGS_TO]->(c)
└── CREATE (e)-[:SEMANTIC_LINK {...}]->(related)
```

### Workflow 2: Créer EntityNative (locale)

```
INPUT
├── Entity invariant
└── Locale cible (fr-FR)

NIKA TRAVERSE
├── Entity.description, denomination_forms
├── Entity relations (SEMANTIC_LINK, BELONGS_TO)
├── Locale context:
│   ├── Style → ton, formalité
│   ├── Slugification → règles URL (latin_preserve, stop_words)
│   ├── Terms → vocabulaire
│   └── CultureRef → références culturelles
└── SEOKeywords (via pillar, project, fulltext)

NIKA GÉNÈRE
├── denomination_forms (text, title, abbrev, url)
├── definition, purpose, benefits
├── jsonld (Schema.org)
└── Internet research (fetch/search)

OUTPUT
├── CREATE (en:EntityNative {
│     ...,
│     trait: "generated",
│     status: "draft",
│     workflow_id: "entity-native-bootstrap",
│     workflow_run_id: "run_xxx",
│     jsonld: {...}
│   })
├── CREATE (e)-[:HAS_NATIVE]->(en)
├── CREATE (en)-[:FOR_LOCALE]->(l)
├── CREATE (en)-[:TARGETS]->(kw)      // keywords choisis
├── CREATE (en)-[:USES_TERM]->(t)     // terms utilisés
└── CREATE (en)-[:USES_CULTURE_REF]->(cr)  // refs culturelles
```

---

## Fichiers à Modifier

### Phase 1: Suppression Embeddings
- [ ] `packages/core/models/node-classes/org/semantic/entity.yaml`
- [ ] `packages/core/models/node-classes/org/semantic/entity-native.yaml`
- [ ] ~18 autres nodes avec embedding_properties

### Phase 2: Entity Simplifié + Workflow
- [ ] `entity.yaml`:
  - Supprimer: `entity_summary`, `llm_context` instance, `embedding_*`
  - Ajouter: `audience_segment`, `workflow_id`, `workflow_run_id`

### Phase 3: EntityNative Refonte
- [ ] `entity-native.yaml`:
  - Changer trait: `authored` → `generated`
  - Supprimer: `curation_status`, `llm_context`, `audience_segment`, `cultural_notes`, `embedding_*`
  - Ajouter: `workflow_id`, `workflow_run_id`, `jsonld`
  - Ajouter relation: `USES_CULTURE_REF` → CultureRef

### Phase 4: SEOKeywordSet Structure
- [ ] `seo-keyword-set.yaml` - ajouter relations
- [ ] `project.yaml` - ajouter HAS_SEO_KEYWORDS
- [ ] Créer arc `FOR_LOCALE` si n'existe pas

### Phase 5: Locale Config (à définir)
- [ ] Culture, Style, Adaptation, Market - TBD

### Phase 6: Semantic Arcs (à définir)
- [ ] Consolidation 59 → ~10

### Phase 7: Inverses (à définir)
- [ ] ~50-70 nouveaux arcs inverses

---

## Changelog Preview

```markdown
## [0.17.0] - 2026-03-XX

### Breaking Changes
- **Embeddings removed** - All `embedding_properties` removed from nodes (D1)
- **Entity simplified** - Removed `entity_summary`, instance `llm_context` (D2)
- **EntityNative trait changed** - `authored` → `generated` (D4)
- **EntityNative properties changed** - Removed `curation_status`, `cultural_notes` (D4)
- **SEOKeywordSet restructured** - Now supports Locale + Project ownership (D3)
- **Market node removed** - Analytics data not needed for LLM context (D5)
- **Semantic arcs consolidated** - 59 → 12 arcs (D6)

### Added
- **Universal workflow traceability** - `workflow_id` on ALL nodes created by Nika (D8)
- `workflow_properties` mixin: `workflow_id`, `workflow_run_id` (D8)
- `audience_segment` on Entity (moved from EntityNative) (D2)
- `jsonld` on EntityNative for Schema.org data (D4)
- `USES_CULTURE_REF` relation on EntityNative (D4)
- `Project -[:HAS_SEO_KEYWORDS]-> SEOKeywordSet` relation (D3)
- `SEOKeywordSet -[:FOR_LOCALE]-> Locale` relation (D3)
- **Advanced Agentic Pipelines** (D9):
  - Multi-agent architecture: orchestrator, planner, executor, reviewer
  - Socratic verification with question-driven quality checks
  - Self-reflection loops with max_iterations
  - Quality gates with on_fail: abort
  - Structured output with JSON Schema (v0.21.0 integration)
  - Artifact system for output persistence
  - Nika skills library for NovaNet workflows

### Changed
- ADR-024 update: EntityNative now uses `generated` trait
- **Locale Config boundaries clarified** (D5):
  - Culture = ce que la culture EST (saisons, fêtes, valeurs)
  - Style = comment on ÉCRIT (formality, pronoun, tone)
  - Adaptation = comment on ADAPTE (FACT vs ILLUSTRATION)
- **Culture properties moved to Style**: communication_directness, hierarchy_importance, individualism_level
- **Style simplified**: ~40 → ~15 properties

### Removed
- `Market` node and `HAS_MARKET` arc (D5)
- `SEMANTIC_LINK` generic arc (D6) - replaced by typed arcs
- ~47 redundant semantic arcs (D6)
- `embedding_properties` from all nodes (D1)

### Nika Workflows (D9)
- `onboarding.nika.yaml` - Orchestration principale
- `00-entity-native-bootstrap.nika.yaml` - EntityNative multi-locale
- `06-seo-discovery-modular.nika.yaml` - Keywords discovery + curation
- `07-content-generation.nika.yaml` - PageNative + BlockNative
- Skills library:
  - `load-entity-context.nika.yaml`
  - `generate-denomination-forms.nika.yaml`
  - `verify-locale-fit.nika.yaml`
  - `write-with-traceability.nika.yaml`
  - `socratic-review.nika.yaml`
```

---

## Questions Résolues

1. ✅ **Locale Config**: Clarifiées - Culture (EST), Style (ÉCRIT), Adaptation (ADAPTE). Market supprimé.
2. ✅ **Semantic Arcs**: 12 arcs essentiels avec propriétés typed (similarity_type, enable_type).
3. ✅ **Workflow traceability**: workflow_id sur TOUS les nodes créés par Nika (D8).
4. ⏸️ **EntityCategory**: À valider lors des tests avec workflows Nika.

---

### D9: Advanced Agentic Pipelines

**Décision**: Utiliser les patterns agentic avancés pour des workflows NovaNet intelligents et auto-vérifiants.

**Référence**: `nika/docs/research/agentic-patterns-2024-2025.md` + [agentic-ai-systems](https://github.com/ThibautMelen/agentic-ai-systems)

#### 9.1 Architecture Multi-Agent (Orchestrator → Specialists)

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  MULTI-AGENT NOVANET PIPELINE                                                 ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  ┌─────────────────────────────────────────────────────────────────────────┐  ║
║  │                      ORCHESTRATOR AGENT                                 │  ║
║  │  • Comprend l'intent                                                    │  ║
║  │  • Décompose en sous-tâches                                             │  ║
║  │  • Route vers specialists                                               │  ║
║  │  • Valide le résultat global                                            │  ║
║  └────────────────────────────────────┬────────────────────────────────────┘  ║
║                                       │                                       ║
║          ┌────────────────────────────┼────────────────────────────┐          ║
║          │                            │                            │          ║
║  ┌───────▼───────┐           ┌────────▼────────┐          ┌────────▼────────┐ ║
║  │  PLANNER      │           │   EXECUTOR      │          │   REVIEWER      │ ║
║  │  Agent        │           │   Agent         │          │   Agent         │ ║
║  ├───────────────┤           ├─────────────────┤          ├─────────────────┤ ║
║  │ • Analyse ctx │           │ • Génère contenu│          │ • Critique      │ ║
║  │ • Plan steps  │           │ • Appelle MCP   │          │ • Vérifie facts │ ║
║  │ • Estimate    │           │ • Écrit nodes   │          │ • Score quality │ ║
║  └───────────────┘           └─────────────────┘          └─────────────────┘ ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

#### 9.2 Output Schema Exact avec `structured:`

```yaml
# Workflow Nika avec output JSON Schema strict
tasks:
  - id: generate_entity_native
    infer: |
      Generate EntityNative for {{entity_key}} in locale {{locale}}.
      Use the provided context from NovaNet.

    # OUTPUT SCHEMA EXACT (v0.21.0)
    structured:
      schema:
        type: object
        required: [display_name, description, definition, purpose, benefits, denomination_forms]
        properties:
          display_name:
            type: string
            description: "Localized display name"
          description:
            type: string
            maxLength: 160
            description: "Short description for meta"
          definition:
            type: string
            description: "Ce que c'est - detailed localized definition"
          purpose:
            type: string
            description: "Pourquoi ça existe"
          benefits:
            type: array
            items: { type: string }
            minItems: 3
            maxItems: 7
          denomination_forms:
            type: array
            items:
              type: object
              required: [type, value]
              properties:
                type: { enum: [text, title, abbrev, url] }
                value: { type: string }
                priority: { type: integer, default: 1 }
          jsonld:
            type: object
            description: "Schema.org JSON-LD"
      max_retries: 3
      enable_repair: true  # 4-layer defense

    # ARTIFACT (output file)
    artifact:
      path: "outputs/{{entity_key}}/{{locale}}/entity-native.json"
      format: json
```

#### 9.3 Self-Reflection Loop (Reviewer Agent)

```yaml
# Workflow avec auto-vérification critique
tasks:
  - id: generate_draft
    agent: executor
    infer: "Generate EntityNative content"
    use.ctx: draft

  - id: review_critically
    agent: reviewer
    infer: |
      You are a CRITICAL reviewer. Score this content:

      Content: {{use.draft}}

      CRITERIA (score 1-10 each):
      1. ACCURACY: Facts are correct and verifiable?
      2. COMPLETENESS: All required fields present?
      3. CULTURAL_FIT: Appropriate for {{locale}}?
      4. SEO_QUALITY: Keywords naturally integrated?
      5. BRAND_VOICE: Matches project tone?

      BE HARSH. If score < 7 on any criteria, REJECT.

      Output JSON: { scores: {...}, passed: bool, feedback: string }
    structured:
      schema: ./schemas/review-result.json
    use.ctx: review

  - id: revise_if_needed
    when: "!use.review.passed"
    agent: executor
    infer: |
      Revise based on feedback:
      {{use.review.feedback}}

      Original: {{use.draft}}
    use.ctx: revised
    max_iterations: 3  # Max 3 revision cycles
```

#### 9.4 Socratic Verification (Question-Driven)

```yaml
# Verification via questions critiques
tasks:
  - id: socratic_verify
    agent: reviewer
    infer: |
      SOCRATIC VERIFICATION for EntityNative:

      Content: {{use.entity_native}}
      Entity: {{entity_key}}
      Locale: {{locale}}

      Ask yourself these questions:

      1. "If I were a {{locale}} native speaker, would this sound natural?"
      2. "Does the definition match the Entity's technical description?"
      3. "Are the benefits actually benefits, or just features?"
      4. "Is the JSON-LD schema.org type correct for this entity?"
      5. "Would this rank well for the targeted SEO keywords?"
      6. "Is there any cultural insensitivity or taboo violation?"

      For each question, provide:
      - Your answer (YES/NO/PARTIAL)
      - Evidence from the content
      - If NO: specific fix needed

      Final verdict: APPROVED / NEEDS_REVISION / REJECTED
    structured:
      schema: ./schemas/socratic-verification.json
```

#### 9.5 Quality Gates avec Checkpoints

```yaml
# Pipeline avec quality gates obligatoires
workflow: entity-native-pipeline
name: "EntityNative Generation Pipeline"

# ARTIFACTS CONFIG
artifacts:
  base_path: "./outputs/{{workflow_run_id}}"
  formats: [json, yaml]

tasks:
  # PHASE 1: Context Loading
  - id: load_context
    invoke: novanet_generate
    params:
      focus_key: "{{entity_key}}"
      locale: "{{locale}}"
      token_budget: 8000
    use.ctx: novanet_context

  # GATE 1: Context Validation
  - id: gate_context
    infer: |
      Validate context quality:
      - Has Entity definition?
      - Has Locale style/culture?
      - Has SEO keywords?
      Score: PASS (all present) / FAIL (missing critical)
    structured:
      schema: { type: object, properties: { passed: { type: boolean }, missing: { type: array } } }
    on_fail: abort

  # PHASE 2: Generation
  - id: generate
    agent: executor
    depends_on: [gate_context]
    # ... generation task

  # GATE 2: Schema Validation (automatic via structured:)

  # GATE 3: Reviewer Approval
  - id: gate_review
    agent: reviewer
    depends_on: [generate]
    # ... review task

  # PHASE 3: Write to NovaNet
  - id: write_to_graph
    depends_on: [gate_review]
    when: "use.review.verdict == 'APPROVED'"
    invoke: novanet_write
    params:
      operation: upsert_node
      class: EntityNative
      key: "{{entity_key}}@{{locale}}"
      properties: "{{use.entity_native}}"
      workflow_id: "{{workflow.id}}"
      workflow_run_id: "{{run.id}}"
    artifact:
      path: "entity-native-{{locale}}.json"
```

#### 9.6 Workflows Nika pour NovaNet (Vue d'ensemble)

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  NIKA WORKFLOWS POUR NOVANET                                                  ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  NIVEAU 1: ORCHESTRATION                                                      ║
║  ─────────────────────────────────────────────────────────────────────────    ║
║  onboarding.nika.yaml                                                         ║
║  ├── Analyse nouveau projet/domain                                            ║
║  ├── Découvre entités à créer                                                 ║
║  ├── Orchestre tout le pipeline                                               ║
║  └── Génère rapport final                                                     ║
║                                                                               ║
║  NIVEAU 2: PHASES (incluses par orchestration)                                ║
║  ─────────────────────────────────────────────────────────────────────────    ║
║  00-entity-native-bootstrap.nika.yaml  → EntityNative multi-locale            ║
║  06-seo-discovery-modular.nika.yaml    → Keywords discovery + curation        ║
║  07-content-generation.nika.yaml       → PageNative + BlockNative             ║
║                                                                               ║
║  NIVEAU 3: SKILLS (réutilisables)                                             ║
║  ─────────────────────────────────────────────────────────────────────────    ║
║  skills/                                                                      ║
║  ├── load-entity-context.nika.yaml     → novanet_generate wrapper             ║
║  ├── generate-denomination-forms.nika.yaml → text/title/abbrev/url            ║
║  ├── verify-locale-fit.nika.yaml       → Cultural verification                ║
║  ├── write-with-traceability.nika.yaml → novanet_write + workflow_id          ║
║  └── socratic-review.nika.yaml         → Question-driven verification         ║
║                                                                               ║
║  AGENTS DÉFINIS                                                               ║
║  ─────────────────────────────────────────────────────────────────────────    ║
║  orchestrator  │ Decompose, route, validate                                   ║
║  planner       │ Analyze context, create plan                                 ║
║  executor      │ Generate content, call MCP                                   ║
║  reviewer      │ Critical evaluation, scores, feedback                        ║
║  researcher    │ Web search, fact-checking                                    ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

#### 9.7 Techniques Avancées

| Technique | Pattern Nika | Usage NovaNet |
|-----------|--------------|---------------|
| **Self-Reflection** | `agent: reviewer` après génération | Valide qualité avant write |
| **Socratic Verification** | Questions critiques structurées | Détecte incohérences |
| **Quality Gates** | `on_fail: abort` ou `when: condition` | Bloque si score < seuil |
| **Artifacts** | `artifact: { path, format }` | Sauvegarde outputs |
| **Structured Output** | `structured: { schema }` | JSON Schema strict |
| **Dynamic Replanning** | `max_iterations` + feedback loop | Améliore jusqu'à approval |
| **Multi-Agent** | `agents:` section + routing | Spécialistes coopèrent |
| **Tool Hints** | `tool_hints: { prefer, avoid }` | Guide sélection MCP |
| **Extended Thinking** | `thinking_mode: extended` | Raisonnement complexe |
| **Circuit Breaker** | `resilience: { circuit_breaker }` | Fail-safe si API down |

#### 9.8 Exemple Complet: EntityNative Pipeline

```yaml
schema: nika/workflow@0.3
name: entity-native-intelligent-pipeline
version: "1.0.0"

# CONFIGURATION
config:
  default_provider: claude
  thinking_mode: extended  # Pour décisions complexes

# AGENTS SPÉCIALISÉS
agents:
  orchestrator:
    system: |
      You are the pipeline orchestrator for NovaNet content generation.
      You decompose complex tasks, route to specialists, and validate results.
    provider: claude
    model: claude-sonnet-4-20250514

  executor:
    system: |
      You are a content generation specialist for {{locale}}.
      You create culturally-native content following brand guidelines.
    provider: claude
    model: claude-sonnet-4-20250514
    mcp: [novanet]

  reviewer:
    system: |
      You are a CRITICAL content reviewer. You NEVER approve mediocre content.
      You score harshly and provide actionable feedback.
      Your standards: accuracy, cultural fit, SEO quality, brand consistency.
    provider: claude
    model: claude-sonnet-4-20250514

# WORKFLOW TASKS
tasks:
  # --- PHASE 1: CONTEXT LOADING ---
  - id: load_novanet_context
    include: skills/load-entity-context.nika.yaml
    params:
      entity_key: "{{inputs.entity_key}}"
      locale: "{{inputs.locale}}"
      token_budget: 8000
    use.ctx: context

  # --- PHASE 2: PLANNING ---
  - id: plan_generation
    agent: orchestrator
    infer: |
      Analyze this context and create a generation plan:

      Entity: {{context.entity}}
      Locale: {{context.locale}}
      SEO Keywords: {{context.seo_keywords}}

      Create a plan with:
      1. Key content themes to cover
      2. Denomination forms strategy
      3. Cultural adaptations needed
      4. SEO integration approach
    structured:
      schema: ./schemas/generation-plan.json
    use.ctx: plan

  # --- PHASE 3: GENERATION ---
  - id: generate_entity_native
    agent: executor
    depends_on: [plan_generation]
    infer: |
      Generate EntityNative following this plan:
      {{use.plan}}

      Context: {{context}}

      Requirements:
      - ALL denomination_forms (text, title, abbrev, url)
      - Definition in {{locale}} voice
      - 3-7 localized benefits
      - Schema.org JSON-LD
    structured:
      schema: ./schemas/entity-native.json
      max_retries: 3
      enable_repair: true
    use.ctx: entity_native
    artifact:
      path: "drafts/{{inputs.entity_key}}-{{inputs.locale}}-draft.json"

  # --- PHASE 4: CRITICAL REVIEW ---
  - id: review_critically
    agent: reviewer
    depends_on: [generate_entity_native]
    include: skills/socratic-review.nika.yaml
    params:
      content: "{{use.entity_native}}"
      locale: "{{inputs.locale}}"
      entity: "{{context.entity}}"
    use.ctx: review

  # --- PHASE 5: REVISION IF NEEDED ---
  - id: revise_if_rejected
    when: "use.review.verdict == 'NEEDS_REVISION'"
    agent: executor
    depends_on: [review_critically]
    infer: |
      Revise based on reviewer feedback:

      FEEDBACK: {{use.review.feedback}}
      FAILED CRITERIA: {{use.review.failed_criteria}}

      Original: {{use.entity_native}}

      Fix ALL issues mentioned. Do not introduce new problems.
    structured:
      schema: ./schemas/entity-native.json
    use.ctx: revised_entity_native
    max_iterations: 2

  # --- PHASE 6: FINAL VERIFICATION ---
  - id: final_verify
    agent: reviewer
    depends_on: [revise_if_rejected]
    when: "use.revised_entity_native != null"
    infer: |
      Final verification of revised content.
      Ensure all previous issues are fixed.

      Original issues: {{use.review.failed_criteria}}
      Revised content: {{use.revised_entity_native}}
    structured:
      schema: ./schemas/review-result.json
    use.ctx: final_review

  # --- PHASE 7: WRITE TO NOVANET ---
  - id: write_to_graph
    depends_on: [final_verify, review_critically]
    when: |
      (use.review.verdict == 'APPROVED') ||
      (use.final_review?.verdict == 'APPROVED')
    invoke: novanet_write
    params:
      operation: upsert_node
      class: EntityNative
      key: "{{inputs.entity_key}}@{{inputs.locale}}"
      properties:
        $merge:
          - "{{use.revised_entity_native || use.entity_native}}"
          - workflow_id: "entity-native-intelligent-pipeline"
            workflow_run_id: "{{run.id}}"
            status: "draft"
            version: 1
    artifact:
      path: "final/{{inputs.entity_key}}-{{inputs.locale}}.json"

  # --- PHASE 8: CREATE RELATIONSHIPS ---
  - id: create_relationships
    depends_on: [write_to_graph]
    invoke: novanet_batch
    params:
      operations:
        - operation: create_arc
          arc_class: HAS_NATIVE
          from_key: "{{inputs.entity_key}}"
          to_key: "{{inputs.entity_key}}@{{inputs.locale}}"
        - operation: create_arc
          arc_class: FOR_LOCALE
          from_key: "{{inputs.entity_key}}@{{inputs.locale}}"
          to_key: "{{inputs.locale}}"
        # SEO keyword relationships from context
        - for_each: "{{context.seo_keywords}}"
          operation: create_arc
          arc_class: TARGETS
          from_key: "{{inputs.entity_key}}@{{inputs.locale}}"
          to_key: "{{item.key}}"
```

---

## Prochaines Étapes

### Décisions Finalisées
1. ✅ D1: Graph-only RAG (suppression embeddings)
2. ✅ D2: Entity simplifié + workflow_id
3. ✅ D3: SEOKeywordSet structure (shared + project)
4. ✅ D4: EntityNative refonte (trait=generated, workflow, jsonld)
5. ✅ D5: Locale Config layer (boundaries clarifiées, Market supprimé)
6. ✅ D6: Semantic Arcs consolidation (59 → 12)
7. ✅ D7: Arcs inverses (via D6 + patterns ownership)
8. ✅ D8: Universal Workflow Traceability (workflow_id sur TOUS les nodes)
9. ✅ D9: Advanced Agentic Pipelines (multi-agent, review, socratic, artifacts)

### Implémentation Schema (NovaNet)
10. 📝 Créer ADR-035: Ontology Cleanup v0.17.0
11. 🔧 Phase 1: Créer `workflow_properties` mixin standard
12. 🔧 Phase 2: Supprimer embedding_properties (~20 nodes)
13. 🔧 Phase 3: Modifier Entity (audience_segment)
14. 🔧 Phase 4: Refonte EntityNative (trait, properties, relations)
15. 🔧 Phase 5: SEOKeywordSet + Project relations
16. 🔧 Phase 6: Locale Config (Culture, Style, supprimer Market)
17. 🔧 Phase 7: Consolider semantic arcs (supprimer ~47 arcs)
18. 🔧 Phase 8: Ajouter workflow_properties à tous les nodes

### Implémentation Workflows (Nika)
19. 📝 Créer JSON Schemas pour outputs (`schemas/`)
    - `entity-native.json` - EntityNative complète
    - `generation-plan.json` - Plan de génération
    - `review-result.json` - Résultat de review
    - `socratic-verification.json` - Vérification socratique
20. 🔧 Créer skills library (`skills/`)
    - `load-entity-context.nika.yaml` - Wrapper novanet_generate
    - `generate-denomination-forms.nika.yaml` - Génération forms
    - `verify-locale-fit.nika.yaml` - Vérification culturelle
    - `write-with-traceability.nika.yaml` - novanet_write + workflow_id
    - `socratic-review.nika.yaml` - Review critique
21. 🔧 Créer workflows phases
    - `00-entity-native-bootstrap.nika.yaml` - Multi-locale avec review
    - `06-seo-discovery-modular.nika.yaml` - SEO avec curation
    - `07-content-generation.nika.yaml` - PageNative + BlockNative
22. 🔧 Créer orchestration
    - `onboarding.nika.yaml` - Pipeline complet avec quality gates
23. 🧪 Tests d'intégration
    - Test pipeline complet Entity → EntityNative → Page → PageNative
    - Test multi-locale (fr-FR, es-MX, de-DE)
    - Test quality gates (rejection, revision, approval)
    - Test artifacts output
