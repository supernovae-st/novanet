# NovaNet Technical Architecture — Deep Dive Technique

**Date**: 2026-02-16
**Version**: v0.13.1 LLM-First BLOC Schema
**Auteur**: 10 agents Explore en parallèle

---

## Table des Matières

1. [Vue d'ensemble](#vue-densemble)
2. [YAML Structure Complète](#yaml-structure-complète)
3. [Système de Classification](#système-de-classification)
4. [Generation Arc Family](#generation-arc-family)
5. [Ownership Arc Family](#ownership-arc-family)
6. [Taxonomy & Visual Encoding](#taxonomy--visual-encoding)
7. [Query-First Architecture](#query-first-architecture)
8. [Générateurs Rust](#générateurs-rust)
9. [Neo4j Bootstrap](#neo4j-bootstrap)
10. [Pipeline Complet](#pipeline-complet)

---

## Vue d'ensemble

NovaNet utilise une **architecture YAML-First (ADR-003)** où **tous les fichiers YAML sont la source de vérité unique** et tout le code est généré.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  PHILOSOPHIE CENTRALE                                                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  "YAML files are the single source of truth. All code is generated."        │
│  — ADR-003: YAML-First Architecture                                         │
│                                                                             │
│  61 node-class YAMLs + 169 arc-class YAMLs + taxonomy.yaml                  │
│          ↓                                                                  │
│  12 générateurs Rust                                                        │
│          ↓                                                                  │
│  Cypher (Neo4j) + TypeScript (Studio) + Rust (TUI) + Mermaid (docs)        │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Statistiques v0.13.1**:
- **61 Classes** (40 shared + 21 org)
- **169 ArcClasses** (79 ownership, 61 semantic, 20 localization, 12 generation, 6 mining)
- **10 Layers** (4 shared: config, locale, geography, knowledge | 6 org: config, foundation, structure, semantic, instruction, output)
- **5 Traits** (defined, authored, imported, generated, retrieved — ADR-024)
- **12 Générateurs** produisant 12 artefacts

---

## YAML Structure Complète

### 1. Format BLOC v0.13.1 (LLM-First Schema)

Chaque node-class YAML suit une **structure canonique en 6 BLOCs** pour une compréhension optimale par les LLMs :

```yaml
node:
  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 1: IDENTITY (required, order: name → realm → layer → trait)
  # ═══════════════════════════════════════════════════════════════════════════
  name: Pattern              # PascalCase, Neo4j label unique
  realm: shared              # shared | org (WHERE does it live?)
  layer: knowledge           # 10 layers total (WHAT functional category?)
  trait: imported            # 5 traits (WHERE does data come from?)

  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 2: SEMANTIC (required, USE/TRIGGERS/NOT/RELATES pattern)
  # ═══════════════════════════════════════════════════════════════════════════
  description: "Content structure pattern or template belonging to a locale's PatternSet."

  llm_context: |
    USE: when loading content templates for native generation.
    TRIGGERS: "pattern", "template", "structure", "format".
    NOT: for vocabulary (use Term), for idioms (use Expression).
    RELATES: PatternSet (parent via CONTAINS), Locale (owner via HAS_PATTERNS).

  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 3: VISUAL (required, dual format icons)
  # ═══════════════════════════════════════════════════════════════════════════
  icon:
    web: clipboard           # Lucide icon name for Studio
    terminal: "▤"            # Unicode symbol for TUI (NO EMOJI!)

  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 4: DATA (required, standard_properties + properties)
  # ═══════════════════════════════════════════════════════════════════════════
  standard_properties:
    # Order canonical: key → *_key → display_name → description → created_at → updated_at
    key:
      type: string
      required: true
      pattern: "^[a-z][a-z0-9_-]*$"
      description: "Unique identifier"

    display_name:
      type: string
      required: true
      description: "Human-readable name"

    description:
      type: string
      required: false

    created_at:
      type: datetime
      required: true

    updated_at:
      type: datetime
      required: true

  properties:
    # Node-specific properties in logical groupings
    template:
      type: string
      required: true
      description: "Template text with {variables}"
      examples:
        - "Découvrez {product} dès maintenant"
        - "Obtenez {benefit} en {action}"

    context:
      type: string
      required: true
      enum: [cta, headline, description, benefit, feature]
      description: "Usage context"

    element:
      type: string
      required: true
      enum: [button, banner, card, hero, pricing]
      description: "UI element type"

    variables:
      type: "string[]"
      required: false
      description: "Placeholders in template"

    constraints:
      type: json
      required: false
      description: "Validation rules"
      example: {max_chars: 50, max_words: 8}

  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 5: GRAPH (relations structure)
  # ═══════════════════════════════════════════════════════════════════════════
  relations:
    incoming:
      - type: CONTAINS_PATTERN
        from: PatternSet
        cardinality: "1:N"
        description: "PatternSet contains multiple Patterns"

  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 6: REFERENCE (neo4j + example)
  # ═══════════════════════════════════════════════════════════════════════════
  neo4j:
    indexes:
      - "CREATE INDEX pattern_context IF NOT EXISTS FOR (p:Pattern) ON (p.context)"
      - "CREATE INDEX pattern_element IF NOT EXISTS FOR (p:Pattern) ON (p.element)"

  example:
    data:
      key: "cta_discover"
      display_name: "CTA Discover"
      template: "Découvrez {product} dès maintenant"
      context: "cta"
      element: "button"
      variables: ["product"]

    cypher: |
      MATCH (ps:PatternSet {domain: "cta"})<-[:HAS_PATTERNS]-(l:Locale {key: "fr-FR"})
      CREATE (p:Pattern {
        key: 'cta_discover',
        display_name: 'CTA Discover',
        template: 'Découvrez {product} dès maintenant',
        context: 'cta',
        element: 'button',
        created_at: datetime(),
        updated_at: datetime()
      })
      CREATE (ps)-[:CONTAINS_PATTERN]->(p)
      RETURN p
```

**Validation automatique** (`schema_rules.rs`):
- **KEY_REQUIRED**: Nodes non-satellite doivent avoir `key`
- **DENORM_REQUIRED**: Nodes avec clés composites doivent avoir `entity_key`, `locale_key`, etc.
- **TIMESTAMP_REQUIRED**: Tous doivent avoir `created_at` + `updated_at`
- **PROP_ORDER**: Standard properties dans l'ordre canonique

---

### 2. Composite Key Pattern (*Native Nodes)

Les nodes avec contenu par locale utilisent des **clés composites** :

```
Format: {type}:{invariant_key}@{locale_key}

Exemples:
  entity:qr-code-generator@fr-FR
  page:homepage@es-MX
  block:pricing:hero:1@ja-JP
```

**EntityNative Structure**:

```yaml
node:
  name: EntityNative
  realm: org
  layer: semantic
  trait: authored              # Humain écrit PAR locale

  standard_properties:
    key:
      type: string
      required: true
      pattern: "^entity:[a-z0-9-]+@[a-z]{2}-[A-Z]{2}$"
      example: "entity:create-qr-code@fr-FR"

    # Denormalized pour fast queries
    entity_key:
      type: string
      required: true
      indexed: true
      description: "Parent entity key (sans prefix 'entity:')"

    locale_key:
      type: string
      required: true
      indexed: true
      pattern: "^[a-z]{2}-[A-Z]{2}$"
      description: "BCP-47 locale code"

    display_name:
      type: string
      required: true
      description: "Localized name"

    # ... timestamps

  properties:
    curation_status:
      type: string
      enum: [human_authored, machine_translated, ai_generated, ai_generated_reviewed]

    definition:
      type: string
      description: "What it IS (localized)"

    purpose:
      type: string
      description: "Why it EXISTS (localized)"

    benefits:
      type: "string[]"
      description: "Value propositions (locale-native)"
```

**Pourquoi les propriétés dénormalisées ?**
- **Fast queries**: `WHERE entity_key = 'qr-code'` trouve tous les locales
- **No parsing**: Pas besoin d'extraire `entity_key` depuis la clé composite
- **Indexed**: Neo4j peut indexer directement

---

### 3. Arc-Class YAML Structure

```yaml
arc:
  name: HAS_NATIVE              # SCREAMING_SNAKE_CASE
  family: ownership             # ownership | localization | semantic | generation | mining
  scope: intra_realm            # intra_realm | cross_realm
  source: [Entity, Page, Block] # Multi-source possible
  target: [EntityNative, PageNative, BlockNative]
  cardinality: one_to_many      # 1:1 | 1:N | N:1 | N:M

  inverse: NATIVE_OF            # ADR-026: Inverse arc policy

  description: >-
    Links an invariant node to its locale-specific content.

  llm_context: |
    USE: when loading locale-specific content for a defined node.
    TRIGGERS: "content", "native", "locale", "localized", "l10n".
    NOT: for structure (use HAS_BLOCK), for definitions (read the invariant).
    RELATES: Entity (parent), EntityNative (locale content), FOR_LOCALE (locale link).

  properties:
    - name: locale
      type: string
      required: true
      description: "Target locale (BCP-47)"

  cypher_pattern: "(Entity, Page, Block)-[:HAS_NATIVE {locale}]->(EntityNative, PageNative, BlockNative)"
```

---

## Système de Classification

### 1. Les 3 Axes Orthogonaux

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  AXES DE CLASSIFICATION (v0.13.0)                                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  REALM → WHERE?      Portée (shared = universel, org = spécifique)          │
│    ├── shared (40 nodes, READ-ONLY)                                         │
│    └── org (21 nodes, business-specific)                                    │
│                                                                             │
│  LAYER → WHAT?       Catégorie fonctionnelle (10 layers)                    │
│    SHARED:                                                                  │
│    ├── config (3)    EntityCategory, Locale, SEOKeywordFormat               │
│    ├── locale (6)    Culture, Style, Formatting, Adaptation...              │
│    ├── geography (7) Continent, Country, GeoRegion...                       │
│    └── knowledge (24) Term, Expression, Pattern, SEO, GEO...                │
│    ORG:                                                                     │
│    ├── config (1)    OrgConfig                                              │
│    ├── foundation (6) Project, Brand, BrandDesign, PromptStyle...           │
│    ├── structure (3)  Page, Block, ContentSlot                              │
│    ├── semantic (4)   Entity, EntityNative, AudiencePersona...              │
│    ├── instruction (4) PageInstruction, BlockInstruction...                 │
│    └── output (3)     PageNative, BlockNative, OutputArtifact               │
│                                                                             │
│  TRAIT → FROM WHERE? Origine des données (ADR-024)                          │
│    ├── defined (33)   Humain crée UNE FOIS (structure, template)            │
│    ├── authored (2)   Humain écrit PAR locale (contenu éditorial)           │
│    ├── imported (20)  Données externes (APIs, bases)                        │
│    ├── generated (4)  Notre LLM produit (NovaNet génère)                    │
│    └── retrieved (2)  APIs externes (snapshots tiers)                       │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Orthogonalité critique**:
- **Layer** répond: "QUELLE catégorie fonctionnelle ?" (config, semantic, output...)
- **Trait** répond: "D'OÙ viennent les données ?" (defined, imported, generated...)
- **Realm** répond: "Quelle portée ?" (shared = tous, org = spécifique)

---

### 2. Trait = Data Origin (ADR-024)

| Trait | Qui crée ? | Exemples | Visual Encoding |
|-------|------------|----------|-----------------|
| **defined** | Humain, UNE FOIS | Page, Block, Entity, Locale | Solid border `─` |
| **authored** | Humain, PAR locale | EntityNative, ProjectNative | Dashed border `┄` |
| **imported** | Données externes | Term, SEOKeyword, GEOQuery | Dotted border `┈` |
| **generated** | Notre LLM | PageNative, BlockNative | Double border `═` |
| **retrieved** | APIs externes | GEOAnswer, SEOKeywordMetrics | Dotted heavy `┅` |

**Exemple concret**:

```
Entity (trait: defined)
  ├─ key: "qr-code-generator"
  ├─ Créé UNE FOIS par humain
  ├─ Décrit le concept (invariant, pas de locale)
  └─ Border: solid (─)

EntityNative (trait: authored)
  ├─ key: "entity:qr-code-generator@fr-FR"
  ├─ Écrit PAR un humain en français
  ├─ Contenu éditorial localisé
  └─ Border: dashed (┄)

SEOKeyword (trait: imported)
  ├─ key: "seo-creer-qr-code-gratuit-fr"
  ├─ Importé depuis DataForSEO/Ahrefs
  ├─ Volume, difficulty, intent
  └─ Border: dotted (┈)

PageNative (trait: generated)
  ├─ key: "page:homepage@fr-FR"
  ├─ Généré par NOTRE LLM
  ├─ Output pour Studio
  └─ Border: double (═)

GEOAnswer (trait: retrieved)
  ├─ Snapshot d'API externe (Claude, GPT, Perplexity)
  ├─ Récupéré à un instant T
  ├─ ai_visibility_score, citation_frequency
  └─ Border: dotted heavy (┅)
```

---

## Generation Arc Family

### 1. Les 12 Arcs de Génération

La **generation family** trace le **pipeline LLM complet** de l'instruction humaine jusqu'au déploiement.

| Arc | Source | Target | Card | Scope | Rôle |
|-----|--------|--------|------|-------|------|
| `INCLUDES_STYLE` | BlockInstruction | Style | N:1 | cross | Instruction référence style config |
| `INCLUDES_ENTITY` | PromptArtifact | Entity | 1:N | intra | Prompt inclut contexte entity |
| `COMPILED_FROM` | PromptArtifact | BlockInstruction | N:1 | intra | Prompt compilé depuis instruction |
| `GENERATED` | BlockInstruction | BlockNative/PageNative | 1:N | intra | Instruction produit contenu |
| `GENERATED_FROM` | BlockNative | BlockType | N:1 | intra | Contenu validé contre schéma |
| `INFLUENCED_BY` | BlockNative | EntityNative | N:M | intra | Contenu influencé par entités |
| `PRODUCED` | PromptArtifact | BlockNative/PageNative | N:M | intra | Prompt produit output |
| `PRODUCED_BY` | BlockNative/PageNative | PromptArtifact | N:M | intra | Output vient de prompt (inverse) |
| `ASSEMBLES` | PageNative | BlockNative | 1:N | intra | Page compose blocks (ordre) |
| `DERIVED_SLUG_FROM` | BlockNative | SEOKeyword | N:1 | cross | Slug copié depuis keyword |
| `BUNDLES` | OutputArtifact | PageNative/BlockNative | 1:N | intra | Artifact contient contenu |
| `PREVIOUS_VERSION` | *Native/*Artifact | *Native/*Artifact | 1:1 | intra | Chaîne de versions |

---

### 2. Pipeline en 5 Phases

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  PHASE 1: AUTHORING (Humain écrit instructions)                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  BlockInstruction (org/instruction, trait:defined)                          │
│      ├── content: "Highlight benefits of @entity:tier-pro vs @tier-basic"   │
│      └──[:INCLUDES_STYLE {cross_realm}]──> Style (shared/locale, read-only) │
│                                             └─ Voice, tone, formatting       │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  PHASE 2: COMPILATION (Instruction → Prompt Artifact)                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  BlockInstruction                                                           │
│      └──[:COMPILED_FROM]──< PromptArtifact (org/output, trait:generated)    │
│                                                                             │
│  PromptArtifact                                                             │
│      ├──[:INCLUDES_ENTITY]──> Entity (contexte)                             │
│      ├──[:INCLUDES_STYLE]──> Style (voice, tone)                            │
│      └── Prêt à envoyer à l'API LLM                                         │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  PHASE 3: GENERATION (Prompt → Content)                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  PromptArtifact                                                             │
│      └──[:PRODUCED {generation_timestamp, model_version, token_usage}]──>   │
│           BlockNative (org/output, trait:generated)                         │
│                                                                             │
│  REVERSE (audit):                                                           │
│  BlockNative                                                                │
│      └──[:PRODUCED_BY {generation_timestamp, model_version}]──>             │
│           PromptArtifact                                                    │
│                                                                             │
│  ALSO (provenance):                                                         │
│  BlockInstruction ──[:GENERATED {generated_at}]──> BlockNative              │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  PHASE 4: PROVENANCE & VALIDATION                                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  BlockNative ──[:INFLUENCED_BY {weight, entity_version}]──> EntityNative    │
│              └─ weight: "primary", "secondary", "minor"                     │
│                                                                             │
│  BlockNative ──[:GENERATED_FROM]──> BlockType                              │
│              └─ Validation rapide: "Match le schéma ?"                      │
│                                                                             │
│  BlockNative:head-seo-meta ──[:DERIVED_SLUG_FROM {cross_realm}]──>          │
│                    SEOKeyword (shared/knowledge)                            │
│                    └─ CRITICAL: slug COPIÉ de keyword.slug_form             │
│                    └─ PAS généré par LLM !                                  │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  PHASE 5: ASSEMBLY & DEPLOYMENT                                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Page (invariant) ──[:HAS_NATIVE {locale}]──> PageNative (generated)        │
│                                                                             │
│  PageNative ──[:ASSEMBLES {order}]──> BlockNative                          │
│             └─ Ordre de rendu (match HAS_BLOCK.order)                       │
│                                                                             │
│  OutputArtifact ──[:BUNDLES {position, checksum}]──> PageNative/BlockNative │
│                └─ Déploiement: quoi dans cette release ?                    │
│                                                                             │
│  *Native ──[:PREVIOUS_VERSION {self-referential}]──> *Native (older)        │
│        └─ Historique: version actuelle → précédente → ...                   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

### 3. Properties on Arcs

Les arcs portent des **métadonnées sur les relations** :

| Arc | Properties | Exemple |
|-----|-----------|---------|
| `PRODUCED` | `generation_timestamp`, `model_version`, `token_usage` | Tracking coût/reproducibilité |
| `INFLUENCED_BY` | `weight`, `entity_version` | "Quelle version d'entity ?" |
| `ASSEMBLES` | `order` | Séquence de rendu (1, 2, 3...) |
| `DERIVED_SLUG_FROM` | `derivation_score`, `alternatives_considered`, `no_repetition_applied` | Audit trail |
| `BUNDLES` | `position`, `checksum` | Intégrité déploiement |

---

## Ownership Arc Family

### 1. Distribution (79 Arcs Total)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  OWNERSHIP FAMILY DISTRIBUTION (ADR-026 Tier Policy)                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  TIER 1: Required Inverses (8 pairs)                                        │
│  ├── HAS_NATIVE ↔ NATIVE_OF                                                 │
│  ├── HAS_PAGE ↔ PAGE_OF                                                     │
│  ├── HAS_ENTITY ↔ ENTITY_OF                                                 │
│  ├── HAS_BLOCK ↔ BLOCK_OF                                                   │
│  ├── HAS_CHILD ↔ CHILD_OF                                                   │
│  ├── HAS_PROJECT ↔ PROJECT_OF                                               │
│  ├── HAS_BRAND ↔ BRAND_OF                                                   │
│  └── HAS_INSTRUCTION ↔ INSTRUCTION_OF                                       │
│                                                                             │
│  TIER 2: Recommended Inverses (6 pairs)                                     │
│  ├── HAS_TERMS ↔ TERMS_OF                                                   │
│  ├── HAS_EXPRESSIONS ↔ EXPRESSIONS_OF                                       │
│  ├── HAS_PATTERNS ↔ PATTERNS_OF                                             │
│  ├── HAS_CULTURE ↔ CULTURE_OF                                               │
│  ├── HAS_TABOOS ↔ TABOOS_OF                                                 │
│  └── HAS_AUDIENCE ↔ AUDIENCE_OF                                             │
│                                                                             │
│  TIER 3: Unidirectional (No Inverse)                                        │
│  ├── CONTAINS_TERM (TermSet → Term)                                         │
│  ├── CONTAINS_EXPRESSION (ExpressionSet → Expression)                       │
│  ├── CONTAINS_PATTERN (PatternSet → Pattern)                                │
│  ├── CONTAINS_CULTURE_REF (CultureSet → CultureRef)                         │
│  ├── CONTAINS_TABOO (TabooSet → Taboo)                                      │
│  └── CONTAINS_AUDIENCE_TRAIT (AudienceSet → AudienceTrait)                  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**ADR-026 Rationale**:
- **TIER 1**: Traversal bidirectionnel fréquent (core ownership)
- **TIER 2**: LLM context loading (knowledge atoms)
- **TIER 3**: Traversal downward only (containers)

---

### 2. HAS_* Pattern

Chaque ownership arc suit ce pattern:

```yaml
# Forward arc
arc:
  name: HAS_NATIVE
  inverse: NATIVE_OF
  family: ownership
  scope: intra_realm
  cardinality: one_to_many
  source: [Entity, Page, Block]
  target: [EntityNative, PageNative, BlockNative]
  properties:
    - name: locale
      type: string
      description: "Target locale (BCP-47)"

# Inverse arc
arc:
  name: NATIVE_OF
  inverse_of: HAS_NATIVE
  family: ownership
  scope: intra_realm
  cardinality: many_to_one
  source: [EntityNative, PageNative, BlockNative]
  target: [Entity, Page, Block]
```

**Validation**: Si `inverse: FOO` déclaré, alors `foo.yaml` DOIT exister.

---

## Taxonomy & Visual Encoding

### 1. Architecture Hybride (v0.12.5)

```
taxonomy.yaml (minimal central)
  ├── arc_scopes (2 values: intra_realm, cross_realm)
  ├── arc_cardinalities (5 values: 1:1, 1:N, N:1, N:M, 0:1)
  └── terminal_palette (256-color + 16-color graceful degradation)

realms/*.yaml (individual files)
  ├── shared.yaml {key, color, layers[], llm_context}
  └── org.yaml {key, color, layers[], llm_context}

layers/*.yaml (10 files)
  ├── config.yaml, locale.yaml, geography.yaml, knowledge.yaml (shared)
  └── foundation.yaml, structure.yaml, semantic.yaml, instruction.yaml, output.yaml (org)

traits/*.yaml (5 files)
  ├── defined.yaml {color, border_style: solid}
  ├── authored.yaml {color, border_style: dashed}
  ├── imported.yaml {color, border_style: dotted}
  ├── generated.yaml {color, border_style: double}
  └── retrieved.yaml {color, border_style: dotted-heavy}

arc-families/*.yaml (5 files)
  ├── ownership.yaml {color, stroke_style: solid}
  ├── localization.yaml {color, stroke_style: solid}
  ├── semantic.yaml {color, stroke_style: dashed}
  ├── generation.yaml {color, stroke_style: solid}
  └── mining.yaml {color, stroke_style: solid}
```

---

### 2. Visual Encoding Channels

| Channel | Encode | Source | Exemples |
|---------|--------|--------|----------|
| **Node fill** | Layer | `layer.color` | config=#64748b, semantic=#f97316, output=#22c55e |
| **Node border color** | Realm | `realm.color` | shared=#2aa198, org=#6c71c4 |
| **Node border style** | Trait | `trait.border_style` | solid, dashed, dotted, double |
| **Node icon** | Class | `class.icon.web` | globe, lightbulb, sparkles |
| **Arc stroke color** | Family | `family.color` | ownership=#3b82f6, semantic=#f97316 |
| **Arc stroke style** | Scope | `scope_strokes` | intra=solid, cross=dashed |
| **Arc arrow** | Cardinality | `cardinality_arrows` | 1:1, 1:N, N:M |

---

### 3. ADR-004: No Color Duplication

**Règle stricte**: Couleurs définies UNIQUEMENT dans YAML, jamais dupliquées.

```yaml
# realms/shared.yaml — SOURCE OF TRUTH
realm:
  color: "#2aa198"   # Hex value defined here

# visual-encoding.yaml — USES COLORS (no hex)
channel_mapping:
  node:
    fill_color: layer           # Référence taxonomy.layers[].color
    border_color: realm         # Référence taxonomy.realms[].color
    # NO HEX VALUES HERE!
```

**Generator Flow**:
```
realms/shared.yaml
    ↓ (colors.rs reads)
apps/studio/src/design/colors/generated.ts
    ↓
export const REALM_COLORS = {
  shared: {
    color: '#2aa198',         // From YAML
    bg: 'bg-[#2aa198]/20',    // Generated Tailwind
    text: 'text-[#2aa198]',
    border: 'border-[#2aa198]/30',
  }
}
```

---

### 4. Dual Icon Format (ADR-013)

**Chaque icon a 2 formats** (NO EMOJI dans le code !) :

```yaml
icons:
  realms:
    shared:
      web: globe              # Lucide icon name (Studio)
      terminal: "◉"           # Unicode (TUI)
    org:
      web: building-2
      terminal: "◎"

  layers:
    config:
      web: settings
      terminal: "⚙"
    semantic:
      web: lightbulb
      terminal: "◆"

  traits:
    defined:
      web: lock
      terminal: "■"
    authored:
      web: pen
      terminal: "□"
```

**8 catégories**: realms, layers, traits, arc_families, states, navigation, quality, modes

---

## Query-First Architecture

### 1. Principe ADR-021

> **"Cypher query = source of truth for graph visualization"**

```
ViewPicker (UI)
    ↓ Click "Entity Composition"
viewStore.executeView('entity-composition')
    ↓
GET /api/views/entity-composition/query
    ↓
ViewLoader.getCypher('entity-composition')
    ↓
views.yaml → cypher field
    ↓
neo4j.run(cypher, {$nodeKey})
    ↓
RETURN nodes + relationships
    ↓
React Flow renders with REALM_COLORS + TRAIT_ICONS
```

---

### 2. Views Structure (12 Views)

**File**: `/packages/core/models/views.yaml` (10,128 bytes)

```yaml
categories:
  schema:
    label: Schema
    icon: {web: database, terminal: "◆"}
    color: "#8b5cf6"

  data:
    label: Data
    icon: {web: boxes, terminal: "●"}
    color: "#6366f1"

  generation:
    label: Generation
    icon: {web: sparkles, terminal: "⚡"}
    color: "#ec4899"

  contextual:
    label: Contextual
    icon: {web: eye, terminal: "◎"}
    color: "#94a3b8"

views:
  - id: entity-truth
    name: Entity Truth
    category: contextual
    contextual: true
    applicable_types: [Entity, EntityNative]
    root_type: Entity
    icon: {web: target, terminal: "⊛"}
    color: "#f59e0b"
    cypher: |
      MATCH (e:Entity {key: $nodeKey})
      OPTIONAL MATCH (proj:Project)-[r_proj:HAS_ENTITY]->(e)
      OPTIONAL MATCH (e)-[r_native:HAS_NATIVE]->(en:EntityNative)
      WHERE en.locale_key IN ['en-US', 'fr-FR', 'es-MX']
      OPTIONAL MATCH (en)-[r_loc:FOR_LOCALE]->(l:Locale)
      OPTIONAL MATCH (page:Page)-[r_repr:REPRESENTS]->(e)
      # ... 23 lines total
      RETURN e, proj, r_proj, en, r_native, l, page, ...
      LIMIT 400
```

**12 Views**:
- **schema** (2): schema-complete, schema-arcs
- **data** (4): data-complete, data-project, data-locales, data-geography
- **generation** (3): gen-page, gen-block, gen-pipeline
- **contextual** (3): ctx-neighbors, ctx-entity, entity-truth

---

### 3. Contextual Views ($nodeKey Parameter)

**Contextual views** sont paramétrées par `$nodeKey`:

```cypher
# View: gen-page (contextual: true, applicable_types: [Page])
MATCH (p:Page {key: $nodeKey})
OPTIONAL MATCH (p)-[r1:HAS_BLOCK]->(b:Block)
OPTIONAL MATCH (b)-[r2:OF_TYPE]->(bt:BlockType)
RETURN p, r1, b, r2, bt
```

**Flow**:
1. User clique node "homepage" → viewStore détecte type `Page`
2. Sidebar affiche views applicables (filtrées par `applicable_types`)
3. User clique "Page Context"
4. Execute avec `$nodeKey = "homepage"`
5. Graph update subgraph centré

---

## Générateurs Rust

### 1. Les 12 Générateurs

| # | Generator | Input | Output | Type |
|---|-----------|-------|--------|------|
| 1 | **OrganizingGenerator** | taxonomy files | `00.5-taxonomy.cypher` | Cypher |
| 2 | **NodeClassGenerator** | 61 node YAMLs | `01-classes.cypher` | Cypher |
| 3 | **ArcClassGenerator** | 169 arc YAMLs | `02-arc-classes.cypher` | Cypher |
| 4 | **LayerGenerator** | nodes + taxonomy | `layers.ts` | TypeScript |
| 5 | **MermaidGenerator** | all YAMLs | `complete-graph.md` | Markdown |
| 6 | **AutowireGenerator** | node/arc relations | `99-autowire-classes.cypher` | Cypher |
| 7 | **HierarchyGenerator** | nodes | `hierarchy.ts` | TypeScript |
| 8 | **ColorsGenerator** | taxonomy colors | `colors/generated.ts` | TypeScript |
| 9 | **IconsGenerator** | visual-encoding | `icons/nodeIcons.generated.ts` | TypeScript |
| 10 | **VisualEncodingGenerator** | taxonomy + visual | `visual-encoding.ts` | TypeScript |
| 11 | **TuiIconsGenerator** | visual-encoding | `tui/icons.rs` | Rust |
| 12 | **TuiColorsGenerator** | taxonomy colors | `tui/colors.generated.rs` | Rust |

---

### 2. Execution Flow

```bash
cargo run -- schema generate

# Rust executes:
commands/schema.rs::schema_generate()
  └── for entry in all_generators():
      ├── entry.generator.generate(root)  # Calls generator::generate()
      ├── post_process (if any)           # Wraps Mermaid in Markdown
      ├── write to file
      └── record result (name, size, duration)

# Output:
✓ taxonomy     00.5-taxonomy.cypher         4.2 KB  [ 12ms]
✓ classes      01-classes.cypher           42.1 KB  [ 88ms]
✓ arcs         02-arc-classes.cypher       28.5 KB  [ 76ms]
✓ layers       layers.ts                   18.3 KB  [ 45ms]
✓ mermaid      complete-graph.md           128 KB   [203ms]
✓ autowire     99-autowire-classes.cypher   8.7 KB  [ 19ms]
✓ hierarchy    hierarchy.ts                12.2 KB  [ 31ms]
✓ colors       colors.generated.ts         12.1 KB  [ 28ms]
✓ icons        nodeIcons.generated.ts      19.4 KB  [ 52ms]
✓ visual       visual-encoding.ts          18.9 KB  [ 41ms]
✓ tui-icons    icons.rs                    24.3 KB  [ 67ms]
✓ tui-colors   colors.generated.rs         11.2 KB  [ 39ms]
───────────────────────────────────────────────────
Generated 12 artifacts in 701ms
```

---

### 3. Parser Layer (YAML → Rust Structs)

**yaml_node.rs** — Parse 61 nodes en parallèle (rayon):

```rust
pub fn load_all_nodes(root: &Path) -> Result<Vec<ParsedNode>> {
    let paths: Vec<PathBuf> = WalkDir::new(nodes_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension() == Some("yaml"))
        .map(|e| e.path().to_path_buf())
        .collect();

    // Parallel parsing (~4x speedup)
    let results: Vec<Result<ParsedNode>> = paths
        .par_iter()  // Rayon parallel iteration
        .map(|path| parse_single_node(path, &nodes_dir))
        .collect();

    results.into_iter().collect()
}
```

**Validation Path**:
```rust
let realm = doc.node.realm.clone();  // From YAML content ← SOURCE OF TRUTH
let layer = doc.node.layer.clone();

// Validate path matches
let path_realm = path.parent().parent().file_name()?;
let path_layer = path.parent().file_name()?;

if path_realm != &realm || path_layer != &layer {
    return Err(ValidationError::PathMismatch { ... });
}
```

---

### 4. Generator Trait

Tous implémentent:

```rust
pub trait Generator {
    fn name(&self) -> &'static str;
    fn generate(&self, root: &Path) -> Result<String>;
}
```

**Pattern**: Générateur est une pure function (no side effects)

---

## Neo4j Bootstrap

### 1. Seed File Numbering

**Exécution alphabétique** via `seed.sh`:

```
Phase 0: Schema
├── 00-constraints.cypher              → Constraints + indexes

Phase 1: Taxonomy (AUTO-GENERATED)
├── 00.5-taxonomy.cypher               → 2 Realms, 10 Layers, 5 Traits, 5 Families

Phase 2: Classes (AUTO-GENERATED)
├── 01-classes.cypher                  → 61 Class nodes
├── 01-vector-indexes.cypher           → Fulltext indexes
├── 01.1-entity-categories.cypher      → 13 categories

Phase 2b: Arcs (AUTO-GENERATED)
├── 02-arc-classes.cypher              → 169 ArcClass nodes

Phase 3: Knowledge (Shared Realm)
├── 20-locales.cypher                  → 200 Locale nodes
├── 22-slugification.cypher            → Slugification rules
├── 23-formatting.cypher               → Number/date formatting
├── 26-expression.cypher               → ExpressionSet + Expression
├── 27-geographic-taxonomy.cypher      → Continents, Regions, Countries

Phase 4: Project (Org Realm)
├── 30-org-config.cypher               → OrgConfig node
├── 31-project-qrcode-ai.cypher        → Project:qrcode-ai

Phase 5: Content
├── 10-entities-qrcode-ai.cypher       → Entity nodes
├── 11-entity-content-fr-fr.cypher     → EntityNative nodes
├── 31-en-us-seo-keywords.cypher       → SEOKeyword nodes

Phase 6: SEO/GEO
├── 49-blocknative-head-seo-meta.cypher → SEO metadata
├── 51-seokeywords-ahrefs.cypher       → Ahrefs data

Phase 9: Autowire (AUTO-GENERATED)
├── 99-autowire-classes.cypher         → [:OF_CLASS] bridges
```

---

### 2. seed.sh Orchestration

```bash
#!/bin/bash
set -e

# UTF-8 for diacritics
export LANG=C.UTF-8
export LC_ALL=C.UTF-8

# Wait for Neo4j
while ! cypher-shell -u neo4j -p novanetpassword "RETURN 1" &>/dev/null; do
  echo "Waiting for Neo4j..."
  sleep 1
done

# Execute seed files
for file in seed/*.cypher; do
  echo "Executing $file..."
  if ! cypher-shell -u neo4j -p novanetpassword -d neo4j --file "$file"; then
    echo "ERROR in $file"
    cat "$file"
    exit 1
  fi
done

# Execute migrations (optional)
if [ -d migrations ]; then
  for migration in migrations/*.cypher; do
    cypher-shell -u neo4j -p novanetpassword -d neo4j --file "$migration"
  done
fi

echo "✓ Seed complete!"
```

**Features**:
- **Idempotent**: MERGE throughout (safe re-run)
- **UTF-8**: Preserves diacritics (é, ñ, ü)
- **Error handling**: Shows file content on failure
- **Migration support**: Optional migrations/ directory

---

### 3. Taxonomy Cypher Example

```cypher
// 00.5-taxonomy.cypher (AUTO-GENERATED by OrganizingGenerator)

// ═══════════════════════════════════════════════════════════════════════════
// REALMS (2)
// ═══════════════════════════════════════════════════════════════════════════

MERGE (r_shared:Schema:Realm {key: 'shared'})
ON CREATE SET
  r_shared.display_name = 'Shared',
  r_shared.color = '#2aa198',
  r_shared.emoji = '◉',
  r_shared.llm_context = 'USE: when accessing universal locale knowledge (READ-ONLY)...',
  r_shared.created_at = datetime()
ON MATCH SET
  r_shared.updated_at = datetime();

MERGE (r_org:Schema:Realm {key: 'org'})
ON CREATE SET
  r_org.display_name = 'Organization',
  r_org.color = '#6c71c4',
  r_org.emoji = '◎',
  r_org.llm_context = 'USE: when working with organization-specific content...',
  r_org.created_at = datetime()
ON MATCH SET
  r_org.updated_at = datetime();

// ═══════════════════════════════════════════════════════════════════════════
// LAYERS (10)
// ═══════════════════════════════════════════════════════════════════════════

MERGE (l_config:Schema:Layer {key: 'config'})
ON CREATE SET
  l_config.display_name = 'Config',
  l_config.color = '#64748b',
  l_config.emoji = '⚙',
  l_config.llm_context = 'USE: when working with configuration definitions...',
  l_config.created_at = datetime();

// ... (9 more layers)

// ═══════════════════════════════════════════════════════════════════════════
// RELATIONSHIPS (Realm → Layer)
// ═══════════════════════════════════════════════════════════════════════════

MATCH (r:Realm {key: 'shared'}), (l:Layer {key: 'config'})
MERGE (r)-[:HAS_LAYER]->(l);

// ... (more HAS_LAYER relationships)
```

---

## Pipeline Complet

### 1. Flow YAML → Neo4j

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  COMPLETE PIPELINE (5 Stages)                                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  STAGE 1: YAML Source (Human edits)                                         │
│  ────────────────────────────────────                                       │
│  Developer writes pattern.yaml in packages/core/models/node-classes/        │
│    ├── 6-BLOC structure (Identity, Semantic, Visual, Data, Graph, Ref)      │
│    ├── Validation: realm/layer/trait required                               │
│    └── Path must match YAML content (source of truth)                       │
│                                                                             │
│  STAGE 2: Schema Validation (cargo run -- schema validate)                  │
│  ──────────────────────────────────────────────────────                     │
│    ├── Parse 61 node YAMLs + 169 arc YAMLs (parallel)                       │
│    ├── Validate 7 checks (KEY_REQUIRED, DENORM_REQUIRED, etc.)              │
│    └── Output: Vec<ValidationIssue> (Error/Warning)                         │
│                                                                             │
│  STAGE 3: Schema Generate (cargo run -- schema generate, ~700ms)            │
│  ─────────────────────────────────────────────────────────────              │
│    ├── 12 generators produce 12 artifacts                                   │
│    ├── Cypher: 00.5-taxonomy, 01-classes, 02-arc-classes, 99-autowire       │
│    ├── TypeScript: layers.ts, colors.ts, icons.ts, visual-encoding.ts       │
│    ├── Rust: tui/icons.rs, tui/colors.generated.rs                          │
│    └── Markdown: complete-graph.md (Mermaid)                                │
│                                                                             │
│  STAGE 4: Cypher Seed (pnpm infra:seed, ~30s)                               │
│  ──────────────────────────────────────────────                             │
│    ├── Execute seed files alphabetically (00 → 99)                          │
│    ├── MERGE statements (idempotent)                                        │
│    ├── UTF-8 encoding (preserves diacritics)                                │
│    └── Output: Neo4j graph populated                                        │
│                                                                             │
│  STAGE 5: Studio Loads Artifacts (pnpm dev, http://localhost:3000)          │
│  ────────────────────────────────────────────────────────────────           │
│    ├── ViewLoader.ts → loads views.yaml dynamically                         │
│    ├── Import: colors.ts, icons.ts, visual-encoding.ts                      │
│    ├── Execute: neo4j.run(cypher, {params})                                 │
│    └── Render: React Flow with REALM_COLORS + TRAIT_ICONS                   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

### 2. Dependency Chain

```
1. taxonomy.yaml (realms, layers, traits, families)
   ↓
2. OrganizingGenerator → 00.5-taxonomy.cypher
   ↓
3. node YAMLs (61 files)
   ↓
4. NodeClassGenerator → 01-classes.cypher
   ↓
5. arc YAMLs (169 files)
   ↓
6. ArcClassGenerator → 02-arc-classes.cypher
   ↓
7. [Parallel Generators]
   ├── LayerGenerator → layers.ts
   ├── MermaidGenerator → complete-graph.md
   ├── ColorsGenerator → colors.ts
   ├── IconsGenerator → icons.ts
   ├── VisualEncodingGenerator → visual-encoding.ts
   ├── TuiIconsGenerator → tui/icons.rs
   └── TuiColorsGenerator → tui/colors.generated.rs
   ↓
8. AutowireGenerator → 99-autowire-classes.cypher
   ↓
9. pnpm infra:seed → Execute all .cypher files
   ↓
10. Neo4j populated with schema + data
```

---

### 3. ADRs Gouvernant le Pipeline

| ADR | Title | Impact Pipeline |
|-----|-------|-----------------|
| **ADR-003** | YAML-First Architecture | YAML = truth, tout est généré |
| **ADR-004** | No Color Duplication | Couleurs UNIQUEMENT dans taxonomy |
| **ADR-013** | Icons Source of Truth | Dual format {web, terminal} |
| **ADR-021** | Query-First Architecture | views.yaml = source queries |
| **ADR-023** | Class/Instance Terminology | Generators use "Class" not "Kind" |
| **ADR-024** | Trait = Data Origin | defined/authored/imported/generated/retrieved |
| **ADR-026** | Inverse Arc Policy | TIER 1/2/3 classification |
| **ADR-029** | *Native Pattern | EntityNative, PageNative naming |
| **ADR-030** | Slug Ownership | Page owns URL, Entity owns semantics |

---

## Résumé Exécutif

### Points Clés

1. **YAML-First**: 61 nodes + 169 arcs en YAML → 12 générateurs → 12 artefacts
2. **6-BLOC Structure**: Identity, Semantic, Visual, Data, Graph, Reference
3. **3 Axes**: Realm (WHERE?), Layer (WHAT?), Trait (FROM WHERE?)
4. **5 Traits**: defined, authored, imported, generated, retrieved (ADR-024)
5. **Query-First**: views.yaml contient Cypher, pas de hardcode
6. **Dual Icons**: {web: Lucide, terminal: Unicode} (NO EMOJI!)
7. **No Duplication**: Couleurs dans taxonomy, référencées partout
8. **Parallel**: rayon accélère parsing (~4x)
9. **Idempotent**: MERGE dans tout Cypher (safe re-run)
10. **UTF-8**: Préserve diacritics (é, ñ, ü) partout

### Commandes Essentielles

```bash
# Validation
cargo run -- schema validate --strict

# Génération (12 artefacts en ~700ms)
cargo run -- schema generate

# Seed Neo4j (~30s)
pnpm infra:seed

# Studio (http://localhost:3000)
pnpm dev

# TUI
cargo run -- tui
```

---

**Fin du Document**
