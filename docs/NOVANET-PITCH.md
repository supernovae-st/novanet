# NovaNet - Orchestrateur de Localisation par Graphe de Connaissances

## Pitch Executif

NovaNet est un orchestrateur de **generation native de contenu** multilingue pour 200+ locales. Contrairement aux systemes de traduction traditionnels, NovaNet utilise un graphe de connaissances Neo4j pour generer du contenu authentiquement local, pas des traductions.

**Application cible** : QR Code AI (https://qrcode-ai.com)

---

## Le Probleme Resolu

### Approche Traditionnelle (Translation) - INCORRECTE

```
Source FR -> Traduire -> Target EN -> Traduire -> Target ES...
```

**Problemes:**
- Perte de nuances culturelles
- Expressions idiomatiques incorrectes
- SEO local inexistant
- Ton inadapte par marche

### Approche NovaNet (Native Generation) - CORRECTE

```
Concept (invariant) -> Generer nativement -> ConceptL10n (local authentique)
```

**Avantages:**
- Contenu culturellement approprie
- SEO natif par locale
- Ton adapte au marche
- Scalable a 200+ locales

---

## Architecture Technique

### 1. Monorepo Turborepo

```
novanet-hq/
├── turbo.json                    # Pipeline Turborepo
├── pnpm-workspace.yaml           # Workspaces pnpm
│
├── packages/
│   ├── core/                     # @novanet/core - Types, schemas, filtres
│   │   ├── models/nodes/         # 35 fichiers YAML (SOURCE DE VERITE)
│   │   ├── models/relations.yaml # 50 types de relations
│   │   ├── src/types/            # Types TypeScript generes
│   │   └── src/graph/            # Hierarchie et layers
│   │
│   ├── db/                       # @novanet/db - Infrastructure Neo4j
│   │   ├── docker-compose.yml    # Neo4j 5.26.0 Community
│   │   └── seed/                 # 7 fichiers de seed Cypher
│   │
├── tools/
│   └── novanet/                  # Rust binary - schema, db, TUI, generation
│
└── apps/
    └── studio/                   # @novanet/studio - Visualisation web
        ├── src/app/api/          # 9 routes API Next.js
        ├── src/stores/           # 8 stores Zustand
        └── src/components/graph/ # React Flow visualization
```

### 2. Graphe de Connaissances - 35 Types de Noeuds

**Structure hierarchique : 3 Realms → 9 Layers → 35 Kinds**

#### Global Realm (15 nodes)

| Layer | Nodes | Description |
|-------|-------|-------------|
| **config** | Locale | Configuration des locales |
| **knowledge** | 14 LocaleKnowledge nodes | Connaissances culturelles |

**LocaleKnowledge nodes (14):**
- LocaleIdentity - Script, timezone, identite technique
- LocaleVoice - Formalite, ton, regles de prononciation
- LocaleCulture - Normes culturelles, tabous
- LocaleCultureReferences - Conteneur de references culturelles
- Reference - References culturelles specifiques
- Metaphor - Metaphores culturelles
- Constraint - Contraintes culturelles
- LocaleMarket - Donnees marche, demographics, plateformes
- LocaleLexicon - Preferences vocabulaire par locale
- Expression - Variantes de phrases specifiques
- LocaleRulesAdaptation - Regles d'adaptation de contenu
- LocaleRulesFormatting - Regles de format (dates, nombres)
- LocaleRulesSlug - Regles de generation de slugs URL
- Pattern - Patterns de formatage reutilisables

#### Shared Realm (6 nodes)

| Layer | Nodes | Description |
|-------|-------|-------------|
| **seo** | SEOKeywordL10n, SEOKeywordMetrics, SEOMiningRun | Optimisation moteurs de recherche |
| **geo** | GEOSeedL10n, GEOSeedMetrics, GEOMiningRun | Optimisation moteurs generatifs (ChatGPT, Perplexity) |

#### Project Realm (14 nodes)

| Layer | Nodes | Description |
|-------|-------|-------------|
| **foundation** | Project, BrandIdentity, ProjectL10n | Base du projet |
| **structure** | Page, Block | Structure de contenu |
| **semantic** | Concept, ConceptL10n | Concepts semantiques |
| **instruction** | PageType, PagePrompt, BlockType, BlockPrompt, BlockRules | Instructions de generation |
| **output** | PageL10n, BlockL10n | Contenu genere |

### 3. Relations Neo4j - 50 Types

Les 50 types de relations connectent les noeuds selon des patterns precis :

**Categories de relations:**
- Hierarchie (CONTAINS, BELONGS_TO, HAS_*)
- Localisation (LOCALIZED_AS, HAS_LOCALE, USES_KNOWLEDGE)
- Generation (GENERATED_FROM, PROMPTED_BY, RULED_BY)
- SEO/GEO (TARGETS_KEYWORD, MINED_BY, HAS_METRICS)
- Structure (REFERENCES, LINKED_TO, DERIVED_FROM)

**Cardinalites:**
- 1:1 - Project → BrandIdentity
- 1:N - Page → Block (une page a plusieurs blocs)
- N:1 - PageL10n → Locale (plusieurs pages vers une locale)
- N:M - Concept ↔ Block (concepts partages entre blocs)

---

## Flux de Donnees - De YAML a Neo4j

### Etape 1: Source de Verite (YAML)

```
models/nodes/
├── global/
│   ├── config/locale.yaml
│   └── knowledge/locale-*.yaml (14 fichiers)
├── shared/
│   ├── seo/*.yaml (3 fichiers)
│   └── geo/*.yaml (3 fichiers)
└── project/
    ├── foundation/*.yaml (3 fichiers)
    ├── structure/*.yaml (2 fichiers)
    ├── semantic/*.yaml (2 fichiers)
    ├── instruction/*.yaml (5 fichiers)
    └── output/*.yaml (2 fichiers)

models/relations.yaml (50 relations)
```

### Etape 2: Generateurs

Le binaire `novanet` Rust lit les fichiers YAML et genere tous les artefacts :

```
YAML ──► novanet schema generate ──┬──► Diagrammes Mermaid (.md)
                                    ├──► layers.ts / hierarchy.ts (TypeScript)
                                    └──► Seeds Cypher (.cypher)
```

### Etape 3: Neo4j

```
Docker: Neo4j 5.26.0 Community
├── Port Browser: 7474
├── Port Bolt: 7687
└── Credentials: neo4j / novanetpassword

Seed files (7 fichiers Cypher):
├── 00-constraints.cypher    # Contraintes d'unicite
├── 01-locales.cypher        # Locales de base
├── 02-locale-knowledge.cypher # Connaissances locales
├── 03-project.cypher        # Projet exemple
├── 04-pages-blocks.cypher   # Structure de contenu
├── 05-concepts.cypher       # Concepts semantiques
└── 06-seo-geo.cypher        # Donnees SEO/GEO
```

### Etape 4: Studio

```
Stack:
├── Next.js 16 (App Router)
├── React 19
├── React Flow (visualisation graphe)
├── Zustand 5 (state management)
└── ELK.js (layout automatique)

API Routes (9):
├── /api/chat            # Claude AI endpoint
├── /api/graph           # Main graph data
├── /api/graph/expand    # Expand node neighbors
├── /api/graph/ontology  # Ontology metadata
├── /api/graph/query     # Execute Cypher queries
├── /api/graph/schema    # Schema information
├── /api/graph/stats     # Graph statistics
├── /api/views           # Saved views CRUD
└── /api/views/[id]      # Individual view operations

Stores Zustand (8):
├── graphStore           # Nodes, edges, loading state
├── filterStore          # Node types, locale, presets (persisted)
├── uiStore              # View mode, panels, selection
├── chatStore            # AI chat messages, streaming
├── queryStore           # Cypher query state, history
├── viewStore            # Saved views management
├── aiQueryStore         # AI-assisted query state
└── animationStore       # Graph animation controls
```

---

## Modes de Visualisation

### Schema Mode (Ontologie)

- **Affiche:** Les 35 TYPES de noeuds et leurs relations
- **Source:** YAML models/nodes/ via `novanet schema generate`
- **Layout:** Groupe par Realm → Layer (ELK hierarchical)
- **Usage:** Comprendre la structure du graphe

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   GLOBAL    │     │   SHARED    │     │  PROJECT    │
│  ┌───────┐  │     │  ┌───────┐  │     │  ┌───────┐  │
│  │Locale │  │────►│  │SEO*   │  │────►│  │Page   │  │
│  └───────┘  │     │  └───────┘  │     │  └───────┘  │
│  ┌───────┐  │     │  ┌───────┐  │     │  ┌───────┐  │
│  │Know*  │  │     │  │GEO*   │  │     │  │Block  │  │
│  └───────┘  │     │  └───────┘  │     │  └───────┘  │
└─────────────┘     └─────────────┘     └─────────────┘
```

### Data Mode (Instances)

- **Affiche:** Les INSTANCES reelles de noeuds depuis Neo4j
- **Source:** Base de donnees Neo4j (Cypher queries)
- **Layout:** Force-directed ou hierarchical selon le filtre
- **Usage:** Explorer les donnees reelles

```
┌───────────┐     ┌───────────┐     ┌───────────┐
│  fr_FR    │     │  en_US    │     │  es_ES    │
│ (Locale)  │     │ (Locale)  │     │ (Locale)  │
└─────┬─────┘     └─────┬─────┘     └─────┬─────┘
      │                 │                 │
      ▼                 ▼                 ▼
┌───────────┐     ┌───────────┐     ┌───────────┐
│ Homepage  │     │ Homepage  │     │ Homepage  │
│  (Page)   │     │  (Page)   │     │  (Page)   │
└───────────┘     └───────────┘     └───────────┘
```

---

## Locale Behaviors (5 types)

Chaque type de noeud a un "behavior" qui definit comment il se comporte vis-a-vis de la localisation :

### 1. INVARIANT (Bleu)

Noeuds qui ne changent jamais entre locales.

**Exemples:** Project, Page, Block, Concept

### 2. LOCALIZED (Vert)

Noeuds generes nativement par locale.

**Exemples:** PageL10n, BlockL10n, ConceptL10n, ProjectL10n

### 3. KNOWLEDGE (Jaune)

Connaissances culturelles par locale.

**Exemples:** LocaleVoice, LocaleCulture, LocaleLexicon, LocaleMarket...

### 4. DERIVED (Violet)

Noeuds calcules a partir d'autres noeuds.

**Exemples:** SEOKeywordMetrics, GEOSeedMetrics

### 5. JOB (Gris)

Noeuds representant des jobs d'execution.

**Exemples:** SEOMiningRun, GEOMiningRun

---

## Commandes Essentielles

### Infrastructure

```bash
novanet db up              # Demarrer Neo4j Docker
novanet db seed            # Seeder la base de donnees
novanet db reset           # Reset complet (down + up + seed)
```

### Developpement

```bash
pnpm dev                   # Demarrer le Studio (localhost:3000)
pnpm build                 # Build tous les packages
pnpm test                  # Tests tous les packages
pnpm type-check            # Verification TypeScript
```

### Schema et Generation

```bash
novanet schema generate    # Generer TypeScript, Mermaid, Cypher
novanet schema validate    # Valider synchronisation YAML <-> artefacts
```

---

## Principe Fondamental: Source de Verite Unique

```
                    ┌─────────────────────┐
                    │   YAML MODELS       │
                    │  (Source of Truth)  │
                    │                     │
                    │  models/nodes/*.yaml│
                    │  models/relations.yaml│
                    └──────────┬──────────┘
                               │
              ┌────────────────┼────────────────┐
              │                │                │
              ▼                ▼                ▼
     ┌────────────────┐ ┌────────────────┐ ┌────────────────┐
     │   novanet      │ │   novanet      │ │   novanet      │
     │ schema generate│ │ schema generate│ │ schema generate │
     │ → .md diagrams │ │ → .ts types    │ │ → .cypher files│
     └────────────────┘ └────────────────┘ └────────────────┘
              │                │                │
              ▼                ▼                ▼
     ┌────────────────┐ ┌────────────────┐ ┌────────────────┐
     │ Documentation  │ │   TypeScript   │ │    Neo4j       │
     │   Mermaid      │ │   Codebase     │ │   Database     │
     └────────────────┘ └────────────────┘ └────────────────┘
```

**Regle d'or** : Toute modification commence dans les fichiers YAML. Les generateurs propagent automatiquement les changements vers TypeScript, Mermaid, et Neo4j.

---

## Resume Chiffre

| Metrique | Valeur |
|----------|--------|
| Types de noeuds | 35 |
| Types de relations | 50 |
| Realms | 3 (Global, Shared, Project) |
| Layers | 9 |
| Fichiers YAML (nodes) | 35 |
| Fichiers seed Neo4j | 7 |
| Routes API | 9 |
| Stores Zustand | 8 |
| Locales supportees | 200+ |

---

## Conclusion

NovaNet represente un changement de paradigme dans la localisation de contenu : passer de la **traduction** a la **generation native**. Le graphe de connaissances Neo4j permet de capturer les nuances culturelles, les strategies SEO locales, et les preferences utilisateur par marche, pour generer du contenu authentiquement local a grande echelle.

---

*Document genere pour Google NotebookLM - SuperNovae Studio*
