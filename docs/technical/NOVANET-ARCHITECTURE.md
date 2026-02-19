# NovaNet Technical Architecture

**Version:** v0.14.0 | **Last Updated:** 2026-02-19

This document is the definitive technical reference for understanding NovaNet internals.

---

## Table of Contents

1. [System Overview](#1-system-overview)
2. [Knowledge Graph Architecture](#2-knowledge-graph-architecture)
3. [Schema System Deep-Dive](#3-schema-system-deep-dive)
4. [Classification System](#4-classification-system)
5. [*Native Pattern (ADR-029)](#5-native-pattern-adr-029)
6. [Denomination Forms (ADR-033)](#6-denomination-forms-adr-033)
7. [MCP Server Architecture](#7-mcp-server-architecture)
8. [Rust CLI Architecture](#8-rust-cli-architecture)
9. [TUI Architecture](#9-tui-architecture)
10. [TypeScript Studio Architecture](#10-typescript-studio-architecture)
11. [Data Flow Diagrams](#11-data-flow-diagrams)
12. [Appendix: File Locations](#appendix-file-locations)

---

## 1. System Overview

NovaNet is a **native content generation engine** that uses Neo4j knowledge graphs to generate culturally-native content across 200+ locales. It is NOT a translation system.

```
                       CRITICAL PRINCIPLE
    =========================================================
    Source -> Translate -> Target              X WRONG
    Entity (defined) -> Generate -> EntityNative   V RIGHT
    =========================================================
```

### High-Level Architecture

```
+-----------------------------------------------------------------------------+
|                        NOVANET SYSTEM ARCHITECTURE                           |
+-----------------------------------------------------------------------------+
|                                                                             |
|  +----------------------+     MCP Protocol      +----------------------+    |
|  |    NovaNet Brain     |<--------------------->|     Nika Body        |    |
|  |                      |    (JSON-RPC 2.0)     |                      |    |
|  |  +----------------+  |                       |  +----------------+  |    |
|  |  | Knowledge      |  |  7 MCP Tools:         |  | YAML Workflows |  |    |
|  |  | Graph          |  |  - novanet_generate   |  | - infer:       |  |    |
|  |  | (Neo4j)        |  |  - novanet_describe   |  | - exec:        |  |    |
|  |  +----------------+  |  - novanet_query      |  | - fetch:       |  |    |
|  |                      |  - novanet_search     |  | - invoke:      |  |    |
|  |  +----------------+  |  - novanet_traverse   |  | - agent:       |  |    |
|  |  | Schema System  |  |  - novanet_assemble   |  +----------------+  |    |
|  |  | (YAML->Cypher) |  |  - novanet_atoms      |                      |    |
|  |  +----------------+  |                       |  +----------------+  |    |
|  |                      |                       |  | rig-core 0.31  |  |    |
|  |  +----------------+  |                       |  | (LLM Provider) |  |    |
|  |  | Rust CLI+TUI   |  |                       |  +----------------+  |    |
|  |  | (1082 tests)   |  |                       |                      |    |
|  |  +----------------+  |                       |  +----------------+  |    |
|  |                      |                       |  | Event System   |  |    |
|  |  +----------------+  |                       |  | (NDJSON trace) |  |    |
|  |  | Studio (Web)   |  |                       |  +----------------+  |    |
|  |  | (React Flow)   |  |                       |                      |    |
|  |  +----------------+  |                       +----------------------+    |
|  +----------------------+                                                   |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### Core Components

| Component | Technology | Purpose | Location |
|-----------|------------|---------|----------|
| **Knowledge Graph** | Neo4j 5.26 | Stores 61 NodeClasses, 182 ArcClasses | `novanet-dev/packages/db/` |
| **Schema System** | YAML + Rust | Defines graph structure | `novanet-dev/packages/core/models/` |
| **Rust CLI** | Rust 1.86 | 13 commands, 12 generators | `novanet-dev/tools/novanet/` |
| **MCP Server** | Rust + rmcp 0.15 | 7 tools for AI agents | `novanet-dev/tools/novanet-mcp/` |
| **TUI** | ratatui 0.30 | Terminal graph explorer | `novanet-dev/tools/novanet/src/tui/` |
| **Studio** | Next.js 16 + React 19 | Web graph visualization | `novanet-dev/apps/studio/` |

---

## 2. Knowledge Graph Architecture

### Neo4j Database Structure

```
+-----------------------------------------------------------------------------+
|                         NEO4J GRAPH STRUCTURE                                |
+-----------------------------------------------------------------------------+
|                                                                             |
|  SCHEMA LAYER (Meta-Graph)                DATA LAYER (Instances)            |
|  -------------------------                --------------------------         |
|  :Schema:Class (61)                       :Entity, :Page, :Block (~19K)     |
|  :Schema:Realm (2)                        :EntityNative, :PageNative        |
|  :Schema:Layer (10)                       :Locale (~200)                    |
|  :Schema:Trait (5)                        :Project, :Brand                  |
|  :Schema:ArcFamily (6)                    :Term, :Expression (~20K)         |
|  :Schema:ArcClass (182)                                                     |
|                                                                             |
|  Link: (instance)-[:OF_CLASS]->(schema:Class)                               |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### 61 NodeClasses Taxonomy

```
+-----------------------------------------------------------------------------+
|                    NODE CLASS DISTRIBUTION (v0.14.0)                         |
+-----------------------------------------------------------------------------+
|                                                                             |
|  SHARED REALM (40 nodes, READ-ONLY)                                         |
|  ==================================                                         |
|                                                                             |
|  config (3)                                                                 |
|  +-- Locale              BCP-47 locale definitions                          |
|  +-- EntityCategory      Entity classification taxonomy                     |
|  +-- SEOKeywordFormat    SEO keyword formatting rules                       |
|                                                                             |
|  locale (6)                                                                 |
|  +-- LocaleVoice         Formality, tone, vocabulary guidelines             |
|  +-- LocaleCulture       Cultural context and references                    |
|  +-- LocaleFormatting    Date, number, currency formats                     |
|  +-- Market              Market characteristics                             |
|  +-- Slugification       URL slug generation rules                          |
|  +-- Adaptation          Locale adaptation strategies                       |
|                                                                             |
|  geography (7)                                                              |
|  +-- Continent           7 continents                                       |
|  +-- GeoRegion           UN M49 regions                                     |
|  +-- GeoSubRegion        UN M49 sub-regions                                 |
|  +-- EconomicRegion      Trade blocs (EU, NAFTA)                            |
|  +-- IncomeGroup         World Bank income classifications                  |
|  +-- LendingCategory     World Bank lending categories                      |
|  +-- Country             ISO 3166-1 countries                               |
|                                                                             |
|  knowledge (24)                                                             |
|  +-- TermSet/Term        Technical vocabulary per locale                    |
|  +-- ExpressionSet/Expression  Idiomatic expressions                        |
|  +-- PatternSet/Pattern  Text templates                                     |
|  +-- CultureSet/CultureRef  Cultural references                             |
|  +-- TabooSet/Taboo      Content to avoid                                   |
|  +-- AudienceSet/AudienceTrait  Audience characteristics                    |
|  +-- SEOKeyword          Search terms                                       |
|  +-- SEOKeywordMetrics   Time-series SEO data                               |
|  +-- GEOQuery            Geographic queries                                 |
|  +-- GEOAnswer           Geographic responses                               |
|  +-- GEOMetrics          Geographic metrics                                 |
|  +-- LanguageFamily      ISO 639-5 families                                 |
|  +-- LanguageBranch      Language branches                                  |
|  +-- CulturalRealm       Cultural classification                            |
|  +-- CulturalSubRealm    Cultural sub-classification                        |
|  +-- PopulationCluster   Population groups                                  |
|  +-- PopulationSubCluster  Population sub-groups                            |
|                                                                             |
|  ORG REALM (21 nodes, tenant-specific)                                      |
|  =====================================                                      |
|                                                                             |
|  config (1)                                                                 |
|  +-- OrgConfig           Organization settings                              |
|                                                                             |
|  foundation (6)                                                             |
|  +-- Project             Content project container                          |
|  +-- ProjectNative       Locale-specific project content                    |
|  +-- Brand               Brand identity (Atlas Pattern)                     |
|  +-- BrandDesign         Design tokens, typography                          |
|  +-- BrandPrinciples     Brand guidelines                                   |
|  +-- PromptStyle         AI generation style presets                        |
|                                                                             |
|  structure (3)                                                              |
|  +-- Page                Page structure definition                          |
|  +-- Block               Content block definition                           |
|  +-- BlockType           Block type classification                          |
|                                                                             |
|  semantic (4)                                                               |
|  +-- Entity              Core semantic concept (invariant)                  |
|  +-- EntityNative        Locale-specific entity content                     |
|  +-- TopicCluster        SEO topic hierarchy                                |
|  +-- SearchIntent        User intent classification                         |
|                                                                             |
|  instruction (4)                                                            |
|  +-- PageStructure       JSON block composition                             |
|  +-- PageInstruction     LLM generation directives                          |
|  +-- BlockInstruction    Block-level LLM directives                         |
|  +-- BlockRules          Block validation rules                             |
|                                                                             |
|  output (3)                                                                 |
|  +-- PageNative          Generated page content                             |
|  +-- BlockNative         Generated block content                            |
|  +-- OutputArtifact      Final rendered output                              |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### 182 ArcClasses Across 6 Families

```
+-----------------------------------------------------------------------------+
|                       ARC FAMILY ARCHITECTURE                                |
+-----------------------------------------------------------------------------+
|                                                                             |
|  FAMILY          | COLOR    | STYLE   | KEY ARCS                            |
|  ================|==========|=========|=====================================|
|  ownership       | blue     | solid   | HAS_PAGE, HAS_BLOCK, HAS_PROJECT,   |
|                  | #3b82f6  |         | OF_TYPE, BELONGS_TO                 |
|                  |          |         |                                     |
|  localization    | green    | solid   | HAS_NATIVE, FOR_LOCALE,             |
|                  | #22c55e  |         | SUPPORTS_LOCALE, NATIVE_OF          |
|                  |          |         |                                     |
|  semantic        | orange   | dashed  | USES_ENTITY, SEMANTIC_LINK,         |
|                  | #f97316  |         | MENTIONS, COVERS, RELATED_TO        |
|                  |          |         |                                     |
|  generation      | purple   | dotted  | GENERATED_FROM, HAS_INSTRUCTION,    |
|                  | #a855f7  |         | USES_PROMPT, HAS_OUTPUT             |
|                  |          |         |                                     |
|  mining          | pink     | dashed  | HAS_SEO_TARGET, EXPRESSES,          |
|                  | #ec4899  |         | TARGETS_THING, SATISFIES_INTENT     |
|                  |          |         |                                     |
|  schema          | indigo   | dotted  | OF_CLASS, FROM_CLASS, TO_CLASS      |
|                  | #6366f1  |         | (meta-schema relationships)         |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### Property Graph Model

Each node and relationship carries properties following strict conventions:

```yaml
# Standard Properties (all nodes)
standard_properties:
  key:           # Primary identifier (composite for *Native nodes)
    type: string
    required: true
    pattern: "^[a-z0-9-]+$"  # kebab-case

  display_name:  # Human-readable name
    type: string
    required: true

  description:   # One-line description
    type: string
    required: true

  created_at:    # ISO 8601 timestamp
    type: datetime
    required: true

  updated_at:    # ISO 8601 timestamp
    type: datetime
    required: true

# Composite Keys (locale-specific nodes)
# Pattern: {type}:{key}@{locale}
# Examples:
#   entity:qr-code@fr-FR
#   page:homepage@ja-JP
#   block:hero-section@es-MX
```

### Indexing Strategy

```cypher
-- Primary key indexes (unique constraints)
CREATE CONSTRAINT entity_key_unique FOR (e:Entity) REQUIRE e.key IS UNIQUE;
CREATE CONSTRAINT page_key_unique FOR (p:Page) REQUIRE p.key IS UNIQUE;
CREATE CONSTRAINT locale_key_unique FOR (l:Locale) REQUIRE l.key IS UNIQUE;

-- Composite key indexes (*Native nodes)
CREATE INDEX entity_native_key FOR (en:EntityNative) ON (en.key);
CREATE INDEX page_native_key FOR (pn:PageNative) ON (pn.key);

-- Denormalized key indexes (for efficient locale queries)
CREATE INDEX entity_native_locale FOR (en:EntityNative) ON (en.locale_key);
CREATE INDEX entity_native_entity FOR (en:EntityNative) ON (en.entity_key);

-- Fulltext search indexes
CALL db.index.fulltext.createNodeIndex('entitySearch', ['Entity'], ['key', 'display_name', 'description']);
CALL db.index.fulltext.createNodeIndex('termSearch', ['Term'], ['key', 'value', 'definition']);

-- Faceted query indexes (realm/layer/trait)
CREATE INDEX class_realm FOR (c:Schema:Class) ON (c.realm);
CREATE INDEX class_layer FOR (c:Schema:Class) ON (c.layer);
CREATE INDEX class_trait FOR (c:Schema:Class) ON (c.trait);
```

---

## 3. Schema System Deep-Dive

### 6 BLOCs Structure

Every NodeClass YAML file follows a strict 6-BLOC ordering for optimal LLM comprehension:

```
+-----------------------------------------------------------------------------+
|                     CANONICAL YAML BLOC STRUCTURE                            |
+-----------------------------------------------------------------------------+
|                                                                             |
|  BLOC 1: IDENTITY (required)                                                |
|  ===========================                                                |
|  node:                                                                      |
|    name: EntityNative                                                       |
|    realm: org                # shared | org                                 |
|    layer: semantic           # 10 options                                   |
|    trait: authored           # defined | authored | imported |              |
|                              # generated | retrieved                        |
|                                                                             |
|  BLOC 2: SEMANTIC (required)                                                |
|  ==========================                                                 |
|    description: "Locale-specific entity content authored by humans"         |
|                                                                             |
|    llm_context: |                                                           |
|      USE: when generating content that references this entity.              |
|      TRIGGERS: "entity content", "localized entity", "EntityNative".        |
|      NOT: for entity definitions (use Entity instead).                      |
|      RELATES: Entity (parent), Locale (context), Page (consumer).           |
|                                                                             |
|  BLOC 3: VISUAL (required)                                                  |
|  ========================                                                   |
|    icon:                                                                    |
|      web: file-text          # Lucide icon name                             |
|      terminal: "+"          # Unicode symbol for TUI                        |
|                                                                             |
|  BLOC 4: DATA (required)                                                    |
|  ======================                                                     |
|    standard_properties:                                                     |
|      key:                    # Composite: entity:{key}@{locale}             |
|        type: string                                                         |
|        required: true                                                       |
|        pattern: "^entity:[a-z0-9-]+@[a-z]{2}-[A-Z]{2}$"                     |
|                                                                             |
|      entity_key:             # Denormalized for efficient queries           |
|        type: string                                                         |
|        required: true                                                       |
|                                                                             |
|      locale_key:             # Denormalized locale reference                |
|        type: string                                                         |
|        required: true                                                       |
|                                                                             |
|      display_name:                                                          |
|        type: string                                                         |
|        required: true                                                       |
|                                                                             |
|      ... created_at, updated_at ...                                         |
|                                                                             |
|    properties:                                                              |
|      title:                  # Locale-specific title                        |
|        type: string                                                         |
|        required: true                                                       |
|                                                                             |
|      denomination_forms:     # ADR-033 prescriptive naming                  |
|        type: array                                                          |
|        items:                                                               |
|          type: object                                                       |
|          properties:                                                        |
|            type: { enum: [text, title, abbrev, url, mixed, base] }          |
|            value: { type: string }                                          |
|                                                                             |
|  BLOC 5: GRAPH (optional but recommended)                                   |
|  =======================================                                    |
|    relations:                # Outgoing arcs                                |
|      FOR_LOCALE:                                                            |
|        to: Locale                                                           |
|        cardinality: "N:1"                                                   |
|        required: true                                                       |
|        description: "Target locale for this content"                        |
|                                                                             |
|    incoming_relations:       # Incoming arcs                                |
|      HAS_NATIVE:                                                            |
|        from: Entity                                                         |
|        cardinality: "1:N"                                                   |
|        description: "Parent entity definition"                              |
|                                                                             |
|  BLOC 6: REFERENCE (optional but recommended)                               |
|  ============================================                               |
|    example:                                                                 |
|      data:                                                                  |
|        key: "entity:qr-code@fr-FR"                                          |
|        entity_key: "qr-code"                                                |
|        locale_key: "fr-FR"                                                  |
|        title: "Code QR"                                                     |
|      cypher: |                                                              |
|        MATCH (en:EntityNative {key: "entity:qr-code@fr-FR"})                |
|        RETURN en                                                            |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### YAML to Cypher Generation Pipeline

```
+-----------------------------------------------------------------------------+
|                    YAML -> CYPHER GENERATION PIPELINE                        |
+-----------------------------------------------------------------------------+
|                                                                             |
|  INPUT                    GENERATORS                      OUTPUT            |
|  -----                    ----------                      ------            |
|                                                                             |
|  models/                                                                    |
|  +-- node-classes/        +-- OrganizingGenerator         packages/db/seed/ |
|  |   +-- shared/          |   (taxonomy.yaml ->           +-- 00.5-taxonomy |
|  |   |   +-- config/      |    00.5-taxonomy.cypher)          .cypher       |
|  |   |   +-- locale/      |                                                 |
|  |   |   +-- geography/   +-- NodeClassGenerator          +-- 01-classes    |
|  |   |   +-- knowledge/   |   (node-classes/*.yaml ->         .cypher       |
|  |   +-- org/             |    01-classes.cypher)                           |
|  |       +-- config/      |                                                 |
|  |       +-- foundation/  +-- ArcClassGenerator           +-- 02-arc-classes|
|  |       +-- structure/   |   (arc-classes/*.yaml ->          .cypher       |
|  |       +-- semantic/    |    02-arc-classes.cypher)                       |
|  |       +-- instruction/ |                                                 |
|  |       +-- output/      +-- AutowireGenerator           +-- 03-autowire   |
|  |                        |   (structural relationships)      .cypher       |
|  +-- arc-classes/         |                                                 |
|  |   +-- ownership/       +-- LayerGenerator              packages/core/src/|
|  |   +-- localization/    |   (layers/*.yaml ->           +-- layers.ts     |
|  |   +-- semantic/        |    graph/layers.ts)                             |
|  |   +-- generation/      |                                                 |
|  |   +-- mining/          +-- HierarchyGenerator          +-- hierarchy.ts  |
|  |   +-- schema/          |   (organizing ->                                |
|  |                        |    graph/hierarchy.ts)                          |
|  +-- realms/              |                                                 |
|  +-- layers/              +-- ColorsGenerator             +-- design/colors/|
|  +-- traits/              |   (taxonomy.yaml ->               generated.ts  |
|  +-- arc-families/        |    palette + semantic)                          |
|  |                        |                                                 |
|  +-- visual-encoding.yaml +-- IconsGenerator              +-- icons.ts      |
|  +-- taxonomy.yaml        |   (visual-encoding.yaml ->                      |
|                           |    icon mapping)                                |
|                           |                                                 |
|                           +-- TuiIconsGenerator           tools/novanet/src/|
|                           |   (visual-encoding.yaml ->    +-- tui/icons.rs  |
|                           |    Rust constants)                              |
|                           |                                                 |
|                           +-- TuiColorsGenerator          +-- tui/colors    |
|                           |   (taxonomy.yaml ->               .generated.rs |
|                           |    xterm-256 palette)                           |
|                           |                                                 |
|                           +-- MermaidGenerator            models/docs/      |
|                           |   (node-classes ->            +-- *.mmd         |
|                           |    Mermaid diagrams)                            |
|                           |                                                 |
|                           +-- ViewMermaidGenerator        +-- views/*.mmd   |
|                               (views/*.yaml ->                              |
|                                view diagrams)                               |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### Validation Rules

The Rust `schema_rules` module enforces these rules at build time:

```
+-----------------------------------------------------------------------------+
|                        SCHEMA VALIDATION RULES                               |
+-----------------------------------------------------------------------------+
|                                                                             |
|  RULE                  | DESCRIPTION                      | AUTO-FIX       |
|  =====================|==================================|================|
|  KEY_REQUIRED         | Non-satellite nodes must have    | No             |
|                       | `key` in standard_properties     |                |
|                       |                                  |                |
|  DENORM_REQUIRED      | Composite key nodes must have    | Yes            |
|                       | denormalized keys (entity_key,   | (adds missing  |
|                       | page_key, locale_key)            | properties)    |
|                       |                                  |                |
|  TIMESTAMP_REQUIRED   | All nodes must have created_at   | Yes            |
|                       | and updated_at properties        | (adds datetime |
|                       |                                  | properties)    |
|                       |                                  |                |
|  PROP_ORDER           | Standard properties must follow  | Yes            |
|                       | canonical order: key -> *_key -> | (reorders)     |
|                       | display_name -> description ->   |                |
|                       | created_at -> updated_at         |                |
|                       |                                  |                |
|  COMPOSITE_KEY_FORMAT | Composite key fields must have   | Yes            |
|                       | pattern property with regex      | (adds pattern) |
|                       |                                  |                |
|  DESCRIPTION_REQUIRED | All nodes must have description  | Yes            |
|                       |                                  | (generates)    |
|                       |                                  |                |
|  EXAMPLE_DATA         | Example section must have valid  | Yes            |
|                       | type-aware example data          | (generates)    |
|                       |                                  |                |
|  PATH_CONTENT_MATCH   | File path must match YAML realm/ | No             |
|                       | layer: shared/locale/style.yaml  |                |
|                       | must have realm: shared, layer:  |                |
|                       | locale                           |                |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### llm_context Pattern: USE/TRIGGERS/NOT/RELATES

The `llm_context` field uses a standardized 4-part pattern for AI comprehension:

```yaml
llm_context: |
  USE: when [primary use case description].
  TRIGGERS: "keyword1", "keyword2", "keyword3".
  NOT: for [disambiguation - what NOT to use this for] (use [alternative] instead).
  RELATES: [Source] (role), [Target] (role), [Related] (relationship).
```

**Example for HAS_NATIVE arc:**

```yaml
llm_context: |
  USE: when loading locale-specific content for a defined node.
  TRIGGERS: "content", "native", "locale", "localized", "l10n".
  NOT: for structure (use HAS_BLOCK), for definitions (read the invariant).
  RELATES: Entity (parent), EntityNative (locale content), FOR_LOCALE (locale link).
```

---

## 4. Classification System

### 2 Realms: Shared vs Org

```
+-----------------------------------------------------------------------------+
|                         REALM ARCHITECTURE                                   |
+-----------------------------------------------------------------------------+
|                                                                             |
|   SHARED REALM (40 nodes)                  ORG REALM (21 nodes)             |
|   =======================                  ====================             |
|                                                                             |
|   - Universal knowledge                    - Tenant-specific content        |
|   - READ-ONLY at runtime                   - Multi-tenant isolation         |
|   - Seeded from external data              - Business-specific data         |
|   - No cross-tenant sharing                - Cross-project sharing          |
|                                                                             |
|   Examples:                                Examples:                        |
|   - Locale definitions (fr-FR)             - Project (qrcode-ai)            |
|   - Geographic data (France)               - Entity (qr-code)               |
|   - Knowledge atoms (Terms)                - Page (homepage)                |
|   - SEO keywords                           - Generated content              |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### 10 Layers: Functional Classification

```
+-----------------------------------------------------------------------------+
|                          LAYER ARCHITECTURE                                  |
+-----------------------------------------------------------------------------+
|                                                                             |
|   SHARED LAYERS (4)                        ORG LAYERS (6)                   |
|   ==================                       ==============                   |
|                                                                             |
|   config (3 nodes)                         config (1 node)                  |
|   +-- Classifications & settings           +-- OrgConfig                    |
|   +-- Locale, EntityCategory               |                                |
|   |                                        foundation (6 nodes)             |
|   locale (6 nodes)                         +-- Project, Brand               |
|   +-- Voice, Culture, Formatting           +-- Design, Principles           |
|   +-- Market, Slugification                |                                |
|   |                                        structure (3 nodes)              |
|   geography (7 nodes)                      +-- Page, Block, BlockType       |
|   +-- Continent, Region, Country           |                                |
|   +-- Economic & lending categories        semantic (4 nodes)               |
|   |                                        +-- Entity, EntityNative         |
|   knowledge (24 nodes)                     +-- TopicCluster, SearchIntent   |
|   +-- TermSet/Term pairs                   |                                |
|   +-- ExpressionSet/Expression             instruction (4 nodes)            |
|   +-- SEO/GEO intelligence                 +-- PageStructure                |
|   +-- Cultural & language families         +-- PageInstruction              |
|                                            +-- BlockInstruction             |
|                                            |                                |
|                                            output (3 nodes)                 |
|                                            +-- PageNative, BlockNative      |
|                                            +-- OutputArtifact               |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### 5 Traits: Data Origin (ADR-024)

```
+-----------------------------------------------------------------------------+
|                        TRAIT = DATA ORIGIN                                   |
+-----------------------------------------------------------------------------+
|                                                                             |
|   TRAIT       | WHO CREATES        | VISUAL STYLE | EXAMPLES                |
|   ============|====================|==============|=========================|
|   defined     | Human, ONCE        | solid border | Entity, Page, Block,    |
|               | (invariant defs)   |              | Locale, BlockType       |
|               |                    |              |                         |
|   authored    | Human, PER locale  | dashed       | EntityNative,           |
|               | (editorial)        |              | ProjectNative           |
|               |                    |              |                         |
|   imported    | External data      | dotted       | Term, Expression,       |
|               | (brought in)       |              | SEOKeyword, Pattern     |
|               |                    |              |                         |
|   generated   | Our LLM            | double       | PageNative,             |
|               | (AI output)        |              | BlockNative             |
|               |                    |              |                         |
|   retrieved   | External APIs      | dotted       | GEOAnswer,              |
|               | (snapshots)        |              | SEOKeywordMetrics       |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### Visual Encoding System

```
+-----------------------------------------------------------------------------+
|                        VISUAL ENCODING RULES                                 |
+-----------------------------------------------------------------------------+
|                                                                             |
|   ATTRIBUTE         | ENCODES              | VALUES                         |
|   ==================|======================|================================|
|   Fill color        | Layer                | config=gray, locale=gray,      |
|                     |                      | geography=emerald,             |
|                     |                      | knowledge=purple,              |
|                     |                      | semantic=orange, etc.          |
|                     |                      |                                |
|   Border color      | Realm                | shared=teal (#14b8a6)          |
|                     |                      | org=sky (#0ea5e9)              |
|                     |                      |                                |
|   Border style      | Trait                | defined=solid                  |
|                     |                      | authored=dashed                |
|                     |                      | imported=dotted                |
|                     |                      | generated=double               |
|                     |                      | retrieved=dotted               |
|                     |                      |                                |
|   Icon (web)        | Per-node             | Lucide icon names              |
|                     |                      | (e.g., "file-text")            |
|                     |                      |                                |
|   Icon (terminal)   | Per-node             | Unicode symbols                |
|                     |                      | (e.g., "+", "-", "*")          |
|                     |                      |                                |
|   Arc style         | ArcFamily            | ownership=solid blue           |
|                     |                      | localization=solid green       |
|                     |                      | semantic=dashed orange         |
|                     |                      | generation=dotted purple       |
|                     |                      | mining=dashed pink             |
|                     |                      | schema=dotted indigo           |
|                                                                             |
+-----------------------------------------------------------------------------+
```

---

## 5. *Native Pattern (ADR-029)

### Unified Suffix Convention

All locale-specific nodes use the `*Native` suffix to indicate locale-native content:

```
+-----------------------------------------------------------------------------+
|                       *NATIVE PATTERN (ADR-029)                              |
+-----------------------------------------------------------------------------+
|                                                                             |
|   BEFORE (v0.12.x)                         AFTER (v0.13.0+)                 |
|   =================                        ================                 |
|   EntityContent                      -->   EntityNative                     |
|   ProjectContent                     -->   ProjectNative                    |
|   PageGenerated                      -->   PageNative                       |
|   BlockGenerated                     -->   BlockNative                      |
|                                                                             |
|   ARC UNIFICATION                                                           |
|   ---------------                                                           |
|   HAS_CONTENT                              HAS_NATIVE (unified)             |
|   HAS_GENERATED                       -->  (single arc for all *Native)     |
|                                                                             |
|   CONTENT_OF                               NATIVE_OF (unified inverse)      |
|   GENERATED_FOR                       -->                                   |
|                                                                             |
|   RATIONALE                                                                 |
|   ---------                                                                 |
|   - Trait (authored vs generated) distinguishes data origin                 |
|   - Suffix no longer carries semantic meaning about creation                |
|   - Simpler graph model: one arc type for all native content                |
|   - Clearer mental model: "Native" = locale-specific                        |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### Entity to EntityNative Flow

```
                    ENTITY CONTENT GENERATION FLOW
    ================================================================

    +----------+        +-----------------+        +----------------+
    |  Entity  |  HAS_  |  EntityNative   |  FOR_  |    Locale      |
    |  (def)   |------->|   (authored)    |------->|   (fr-FR)      |
    +----------+ NATIVE +-----------------+ LOCALE +----------------+
         |                     |
         |                     | denomination_forms (ADR-033)
         |                     | [text, title, abbrev, url, mixed, base]
         |                     |
    key: "qr-code"       key: "entity:qr-code@fr-FR"
    display_name:        entity_key: "qr-code"
      "QR Code"          locale_key: "fr-FR"
                         title: "Code QR"
                         denomination_forms:
                           - { type: "text", value: "code QR" }
                           - { type: "title", value: "Code QR" }
                           - { type: "abbrev", value: "QR" }
```

---

## 6. Denomination Forms (ADR-033)

### Prescriptive Canonical Forms

The `denomination_forms` property provides **prescriptive** forms for entity references. The LLM MUST use ONLY these forms - no invention, no paraphrase.

```
+-----------------------------------------------------------------------------+
|                   DENOMINATION FORMS (ADR-033)                               |
+-----------------------------------------------------------------------------+
|                                                                             |
|   FORM     | USAGE                      | es-MX EXAMPLE                     |
|   =========|============================|===================================|
|   text     | Prose, body content        | "codigo qr"                       |
|            | (lowercase, natural flow)  |                                   |
|            |                            |                                   |
|   title    | H1, H2, meta_title         | "Codigo QR"                       |
|            | (title case)               |                                   |
|            |                            |                                   |
|   abbrev   | After first mention        | "qr"                              |
|            | (shortest recognizable)    |                                   |
|            |                            |                                   |
|   url      | URL-safe slug              | "crear-codigo-qr"                 |
|            | (SEO pipeline write-back)  | (populated after slug derivation) |
|            |                            |                                   |
|   mixed    | Tech/brand hybrid          | "QR码" (for zh-CN)                |
|            | (native_script locales)    |                                   |
|            |                            |                                   |
|   base     | International reference    | "QR Code" (for ja-JP)             |
|            | (native_script locales)    |                                   |
|                                                                             |
|   ABSOLUTE RULE: LLM must NOT invent forms. Only use listed values.         |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### MCP Integration

The `novanet_generate` tool returns `denomination_forms` in its response:

```json
{
  "denomination_forms": {
    "qr-code": {
      "text": "codigo qr",
      "title": "Codigo QR",
      "abbrev": "qr",
      "url": "crear-codigo-qr"
    }
  },
  "context_build_log": {
    "entities_phase": [
      "ADR-033: Loaded 1 denomination form(s)"
    ]
  }
}
```

---

## 7. MCP Server Architecture

### Tool Registration Pattern

```
+-----------------------------------------------------------------------------+
|                      MCP SERVER ARCHITECTURE                                 |
+-----------------------------------------------------------------------------+
|                                                                             |
|   TRANSPORT LAYER                                                           |
|   ===============                                                           |
|   stdio (JSON-RPC 2.0 over stdin/stdout)                                    |
|   Implemented via rmcp 0.15 SDK                                             |
|                                                                             |
|   +------------------+                                                      |
|   |   MCP Client     |  (Claude Code / Nika)                                |
|   +------------------+                                                      |
|            |                                                                |
|            | JSON-RPC 2.0                                                    |
|            v                                                                |
|   +------------------+                                                      |
|   |   MCP Server     |  (novanet-mcp binary)                                |
|   |   +------------+ |                                                      |
|   |   | Handler    | |  #[tool] macro registration                          |
|   |   +------------+ |                                                      |
|   |   | State      | |  Arc<StateInner> shared state                        |
|   |   +------------+ |                                                      |
|   +------------------+                                                      |
|            |                                                                |
|            | neo4rs                                                         |
|            v                                                                |
|   +------------------+                                                      |
|   |   Neo4j Pool     |  Deadpool connection pool                            |
|   +------------------+                                                      |
|            |                                                                |
|            | bolt://                                                        |
|            v                                                                |
|   +------------------+                                                      |
|   |   Neo4j 5.26     |                                                      |
|   +------------------+                                                      |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### 7 MCP Tools

```
+-----------------------------------------------------------------------------+
|                          MCP TOOLS (7 total)                                 |
+-----------------------------------------------------------------------------+
|                                                                             |
|   TOOL               | PURPOSE                          | PARAMS            |
|   ===================|==================================|===================|
|   novanet_generate   | Full RLM-on-KG context assembly  | focus_key, locale,|
|                      | Returns: prompt, evidence,       | mode, token_budget|
|                      | denomination_forms, anchors      |                   |
|                      |                                  |                   |
|   novanet_describe   | Bootstrap agent understanding    | describe (schema/ |
|                      | Returns: schema overview, stats  | entity/category)  |
|                      |                                  |                   |
|   novanet_query      | Execute read-only Cypher         | cypher, params,   |
|                      | Returns: rows, row_count         | limit, timeout_ms |
|                      |                                  |                   |
|   novanet_search     | Fulltext + property search       | query, mode,      |
|                      | Returns: hits, total_hits        | kinds, realm      |
|                      |                                  |                   |
|   novanet_traverse   | Graph traversal with depth       | start_key,        |
|                      | Returns: nodes, arcs, paths      | max_depth,        |
|                      |                                  | direction         |
|                      |                                  |                   |
|   novanet_assemble   | Token-aware context assembly     | focus_key, locale,|
|                      | Returns: evidence packets        | token_budget,     |
|                      |                                  | strategy          |
|                      |                                  |                   |
|   novanet_atoms      | Retrieve knowledge atoms         | locale, atom_type,|
|                      | Returns: atoms[], containers     | domain, query     |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### Request/Response Schemas

**novanet_generate Request:**

```json
{
  "focus_key": "homepage",
  "locale": "fr-FR",
  "mode": "page",
  "token_budget": 50000,
  "include_examples": true,
  "spreading_depth": 2
}
```

**novanet_generate Response:**

```json
{
  "prompt": "# Generation Context for homepage (fr-FR)...",
  "evidence_summary": [
    {
      "source_key": "qr-code",
      "evidence_type": "entity",
      "relevance": 0.95,
      "tokens": 120
    }
  ],
  "locale_context": {
    "locale_key": "fr-FR",
    "language": "French",
    "voice": "professionnel, accessible",
    "formality": "vous"
  },
  "context_anchors": [
    {
      "page_key": "pricing",
      "anchor_text": "page de tarifs",
      "slug": "/fr/tarifs",
      "context_hint": "Link when mentioning pricing"
    }
  ],
  "denomination_forms": {
    "qr-code": {
      "text": "code QR",
      "title": "Code QR",
      "abbrev": "QR"
    }
  },
  "token_usage": {
    "structure": 500,
    "entities": 3200,
    "knowledge": 1800,
    "locale": 400,
    "total": 5900,
    "budget_remaining": 44100
  },
  "context_build_log": {
    "structure_phase": ["Focus: homepage (Page)", "Mode: Page, Spreading depth: 2"],
    "entities_phase": ["Visited 12 nodes, collected 8 evidence packets"],
    "atoms_phase": ["Locale: fr-FR, retrieved 45 atoms"],
    "anchors_phase": ["Found 3 context anchor(s)"],
    "token_decisions": ["Token breakdown: structure=500, entities=3200..."]
  },
  "metadata": {
    "blocks_discovered": 5,
    "entities_loaded": 12,
    "atoms_loaded": 45,
    "execution_time_ms": 250
  }
}
```

### Error Handling

```
+-----------------------------------------------------------------------------+
|                      MCP ERROR MAPPING                                       |
+-----------------------------------------------------------------------------+
|                                                                             |
|   ERROR TYPE           | JSON-RPC CODE | WHEN                               |
|   =====================|===============|====================================|
|   NotFound             | -32001        | Entity/resource not found          |
|   InvalidCypher        | -32602        | Cypher validation failed           |
|   WriteNotAllowed      | -32602        | Write operation attempted          |
|   TokenBudgetExceeded  | -32602        | Token limit exceeded               |
|   Connection           | -32603        | Neo4j connection failed            |
|   Query                | -32603        | Query execution failed             |
|   Internal             | -32603        | Unexpected error                   |
|                                                                             |
+-----------------------------------------------------------------------------+
```

---

## 8. Rust CLI Architecture

### Command Pattern (clap)

```
+-----------------------------------------------------------------------------+
|                       RUST CLI ARCHITECTURE                                  |
+-----------------------------------------------------------------------------+
|                                                                             |
|   ENTRY POINT: main.rs                                                      |
|   ====================                                                      |
|   #[derive(Parser)]                                                         |
|   struct Cli {                                                              |
|       #[command(subcommand)]                                                |
|       command: Commands,                                                    |
|       #[arg(long, env = "NOVANET_ROOT")]                                    |
|       root: Option<PathBuf>,                                                |
|   }                                                                         |
|                                                                             |
|   COMMANDS (13 total)                                                       |
|   ===================                                                       |
|                                                                             |
|   READ COMMANDS                                                             |
|   +-- blueprint      Schema-graph visualization (11 views)                  |
|   +-- data           Data nodes only                                        |
|   +-- overlay        Data + Schema overlay                                  |
|   +-- query          Faceted query with filters                             |
|   +-- search         Fulltext + property search                             |
|                                                                             |
|   WRITE COMMANDS                                                            |
|   +-- node           create/edit/delete                                     |
|   +-- arc            create/delete                                          |
|   +-- db             seed/migrate/reset                                     |
|                                                                             |
|   SCHEMA COMMANDS                                                           |
|   +-- schema         generate/validate                                      |
|   +-- doc            generate (Mermaid diagrams)                            |
|   +-- filter         build (JSON -> Cypher)                                 |
|                                                                             |
|   OTHER COMMANDS                                                            |
|   +-- locale         list/import/generate                                   |
|   +-- knowledge      generate/list (ATH integration)                        |
|   +-- entity         seed/list/validate                                     |
|   +-- tui            Interactive terminal UI                                |
|   +-- completions    Shell completions                                      |
|   +-- doctor         System health check                                    |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### 12 Generators (YAML to Artifacts)

```
+-----------------------------------------------------------------------------+
|                         GENERATOR REGISTRY                                   |
+-----------------------------------------------------------------------------+
|                                                                             |
|   GENERATOR              | INPUT                 | OUTPUT                   |
|   =======================|=======================|==========================|
|   OrganizingGenerator    | taxonomy.yaml         | 00.5-taxonomy.cypher     |
|   NodeClassGenerator     | node-classes/*.yaml   | 01-classes.cypher        |
|   ArcClassGenerator      | arc-classes/*.yaml    | 02-arc-classes.cypher    |
|   AutowireGenerator      | Structural analysis   | 03-autowire.cypher       |
|   LayerGenerator         | layers/*.yaml         | graph/layers.ts          |
|   HierarchyGenerator     | organizing principles | graph/hierarchy.ts       |
|   ColorsGenerator        | taxonomy.yaml         | design/colors/generated  |
|   IconsGenerator         | visual-encoding.yaml  | design/icons.ts          |
|   VisualEncodingGenerator| visual-encoding.yaml  | visual-encoding.ts       |
|   TuiIconsGenerator      | visual-encoding.yaml  | tui/icons.rs             |
|   TuiColorsGenerator     | taxonomy.yaml         | tui/colors.generated.rs  |
|   MermaidGenerator       | node-classes/*.yaml   | models/docs/*.mmd        |
|   ViewMermaidGenerator   | views/*.yaml          | models/docs/views/*.mmd  |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### Schema Validation Pipeline

```
+-----------------------------------------------------------------------------+
|                    SCHEMA VALIDATION PIPELINE                                |
+-----------------------------------------------------------------------------+
|                                                                             |
|   PHASE 1: YAML PARSING                                                     |
|   =====================                                                     |
|   +-- Parse all node-classes/*.yaml                                         |
|   +-- Parse all arc-classes/*.yaml                                          |
|   +-- Validate YAML syntax                                                  |
|                                                                             |
|   PHASE 2: STRUCTURAL VALIDATION                                            |
|   ==============================                                            |
|   +-- KEY_REQUIRED: Check key exists                                        |
|   +-- DENORM_REQUIRED: Check denormalized keys                              |
|   +-- TIMESTAMP_REQUIRED: Check timestamps                                  |
|   +-- PROP_ORDER: Check property ordering                                   |
|                                                                             |
|   PHASE 3: PATH VALIDATION                                                  |
|   ========================                                                  |
|   +-- File path matches YAML realm/layer                                    |
|   +-- Example: shared/locale/style.yaml must have realm: shared,            |
|       layer: locale                                                         |
|                                                                             |
|   PHASE 4: CROSS-REFERENCE VALIDATION                                       |
|   ===================================                                       |
|   +-- Arc sources/targets exist                                             |
|   +-- Inverse arcs match                                                    |
|   +-- ArcFamily membership valid                                            |
|                                                                             |
|   PHASE 5: AUTO-FIX (optional --fix flag)                                   |
|   =======================================                                   |
|   +-- CompositeKeyFixer: Add pattern regex                                  |
|   +-- DenormalizedKeyFixer: Add denorm props                                |
|   +-- TimestampFixer: Add timestamps                                        |
|   +-- PropertyOrderFixer: Reorder props                                     |
|   +-- DescriptionFixer: Generate descriptions                               |
|   +-- ExampleDataFixer: Generate examples                                   |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### Output Formatters

```rust
// Three output formats supported
pub enum OutputFormat {
    Table,  // Pretty-printed table (default for interactive)
    Json,   // JSON output (for piping/scripting)
    Yaml,   // YAML output (for config editing)
}

// Usage:
// cargo run -- query --realm=org --format=json
// cargo run -- blueprint --view=stats --format=table
```

---

## 9. TUI Architecture

### Unified Tree (ADR-022)

```
+-----------------------------------------------------------------------------+
|                        TUI UNIFIED TREE                                      |
+-----------------------------------------------------------------------------+
|                                                                             |
|   PRINCIPLE: "If it's a node in Neo4j, it's a node everywhere"              |
|                                                                             |
|   BEFORE v11.7 (5 modes)                   AFTER v11.7 (2 modes)            |
|   ======================                   ====================             |
|   [1] Data mode                            [1] Graph mode                   |
|   [2] Meta mode                                 +-- Realm nodes             |
|   [3] Overlay mode                              +-- Layer nodes             |
|   [4] Query mode                                +-- Class nodes             |
|   [5] Atlas mode                                +-- Instance nodes (lazy)   |
|                                                 +-- ArcFamily nodes         |
|                                                 +-- ArcClass nodes          |
|                                                                             |
|                                            [2] Nexus mode                   |
|                                                 +-- Quiz                    |
|                                                 +-- Audit                   |
|                                                 +-- Stats                   |
|                                                 +-- Help                    |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### State Machine

```
+-----------------------------------------------------------------------------+
|                        TUI STATE MACHINE                                     |
+-----------------------------------------------------------------------------+
|                                                                             |
|   pub struct App {                                                          |
|       // Navigation state                                                   |
|       nav_mode: NavMode,           // Graph | Nexus                         |
|       tree: TaxonomyTree,          // Unified tree data                     |
|       selected_idx: usize,         // Current tree selection                |
|       scroll_offset: usize,        // Tree scroll position                  |
|                                                                             |
|       // Panel state                                                        |
|       yaml_scroll: usize,          // YAML panel scroll                     |
|       show_yaml: bool,             // Toggle YAML panel                     |
|                                                                             |
|       // Overlay state                                                      |
|       overlay: Option<Overlay>,    // Help | Search | None                  |
|       search_query: String,        // Current search text                   |
|                                                                             |
|       // Async loading                                                      |
|       pending_instance_load: Option<String>,  // Class key                  |
|       pending_arcs_load: Option<String>,      // Instance key               |
|                                                                             |
|       // Cached details                                                     |
|       class_arcs: Option<ClassArcs>,          // Loaded arc data            |
|       realm_details: Option<RealmDetails>,                                  |
|       layer_details: Option<LayerDetails>,                                  |
|                                                                             |
|       // Status                                                             |
|       status_message: Option<(String, Instant)>,                            |
|       tick: u64,                   // Animation frame counter               |
|   }                                                                         |
|                                                                             |
|   pub enum NavMode {                                                        |
|       Graph,   // Unified tree: Realm > Layer > Class > Instance            |
|       Nexus,   // Hub: Quiz, Audit, Stats, Help                             |
|   }                                                                         |
|                                                                             |
|   pub enum Overlay {                                                        |
|       Help,    // Keyboard shortcuts (/)                                    |
|       Search,  // Fuzzy search (f)                                          |
|   }                                                                         |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### Async Neo4j Queries via mpsc Channels

```
+-----------------------------------------------------------------------------+
|                     ASYNC LOADING PATTERN                                    |
+-----------------------------------------------------------------------------+
|                                                                             |
|   EVENT LOOP                              ASYNC LOADER                       |
|   ==========                              ============                       |
|                                                                             |
|   +----------------+                      +----------------+                 |
|   |  Key Handler   |                      |  TaxonomyTree  |                 |
|   |  (handle_key)  |                      |  ::load_*      |                 |
|   +----------------+                      +----------------+                 |
|          |                                       |                          |
|          | Set pending_*_load                    |                          |
|          v                                       |                          |
|   +----------------+                             |                          |
|   |  Main Loop     |                             |                          |
|   |  (100ms tick)  |                             |                          |
|   +----------------+                             |                          |
|          |                                       |                          |
|          | Check pending                         |                          |
|          v                                       |                          |
|   +----------------+   tokio::spawn    +----------------+                   |
|   |  Async Block   |------------------>|  Neo4j Query   |                   |
|   +----------------+                   +----------------+                   |
|          |                                       |                          |
|          | .await                                |                          |
|          v                                       |                          |
|   +----------------+                   +----------------+                   |
|   |  Update State  |<------------------|  Query Result  |                   |
|   +----------------+                   +----------------+                   |
|          |                                                                  |
|          | Re-render                                                        |
|          v                                                                  |
|   +----------------+                                                        |
|   |  terminal.draw |                                                        |
|   +----------------+                                                        |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### ratatui Rendering Pipeline

```
+-----------------------------------------------------------------------------+
|                     TUI RENDERING PIPELINE                                   |
+-----------------------------------------------------------------------------+
|                                                                             |
|   LAYOUT (3 panels)                                                         |
|   ==================                                                        |
|                                                                             |
|   +------------------+------------------+------------------+                 |
|   |                  |                  |                  |                 |
|   |   TREE PANEL     |   INFO PANEL     |   YAML PANEL     |                 |
|   |   (40%)          |   (30%)          |   (30%)          |                 |
|   |                  |                  |                  |                 |
|   |   Realm          |   Selected node  |   YAML preview   |                 |
|   |     Layer        |   properties,    |   of selected    |                 |
|   |       Class      |   stats, arcs    |   node class     |                 |
|   |         Instance |                  |                  |                 |
|   |                  |                  |                  |                 |
|   +------------------+------------------+------------------+                 |
|   |                  STATUS BAR                            |                 |
|   +--------------------------------------------------------+                 |
|                                                                             |
|   RENDERING ORDER                                                           |
|   ===============                                                           |
|   1. Clear terminal                                                         |
|   2. Render tree panel (with selection highlight)                           |
|   3. Render info panel (node details)                                       |
|   4. Render yaml panel (syntax highlighted)                                 |
|   5. Render status bar (mode, counts, status message)                       |
|   6. Render overlay if active (help/search)                                 |
|   7. Set cursor position                                                    |
|                                                                             |
+-----------------------------------------------------------------------------+
```

---

## 10. TypeScript Studio Architecture

### Next.js App Router

```
+-----------------------------------------------------------------------------+
|                    STUDIO NEXT.JS ARCHITECTURE                               |
+-----------------------------------------------------------------------------+
|                                                                             |
|   app/                                                                      |
|   +-- page.tsx              Main visualization page                         |
|   +-- layout.tsx            Root layout with providers                      |
|   +-- api/                                                                  |
|       +-- chat/             Claude AI endpoint                              |
|       +-- graph/                                                            |
|       |   +-- route.ts      Main graph data                                 |
|       |   +-- expand/       Expand node neighbors                           |
|       |   +-- schema/       Schema information                              |
|       |   +-- query/        Execute Cypher queries                          |
|       |   +-- stats/        Graph statistics                                |
|       |   +-- ontology/     Ontology metadata                               |
|       |   +-- organizing-principles/                                        |
|       +-- views/                                                            |
|           +-- route.ts      Saved views CRUD                                |
|           +-- [id]/         Individual view operations                      |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### React Flow for Graph Visualization

```
+-----------------------------------------------------------------------------+
|                  REACT FLOW INTEGRATION                                      |
+-----------------------------------------------------------------------------+
|                                                                             |
|   components/graph/                                                         |
|   +-- GraphCanvas.tsx       Main React Flow wrapper                         |
|   +-- nodes/                                                                |
|   |   +-- SchemaNode.tsx    Schema node renderer                            |
|   |   +-- DataNode.tsx      Data node renderer                              |
|   |   +-- NodeConfig.ts     Node type configurations                        |
|   |   +-- NodeStyles.ts     Visual encoding styles                          |
|   +-- edges/                                                                |
|   |   +-- CustomEdge.tsx    Arc renderer with labels                        |
|   |   +-- hooks/            Edge-specific hooks                             |
|   |   +-- effects/          Visual effects (animations)                     |
|   +-- schema/                                                               |
|   |   +-- SchemaLayout.tsx  ELK layout for schema mode                      |
|   +-- config/                                                               |
|       +-- LODConfig.ts      Level-of-detail configuration                   |
|                                                                             |
|   LAYOUT ENGINES                                                            |
|   ==============                                                            |
|   +-- ELK (hierarchical)    Primary layout for schema mode                  |
|   +-- Dagre (directed)      DAG layouts                                     |
|   +-- d3-force (force)      Force-directed layouts                          |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### Zustand Stores

```
+-----------------------------------------------------------------------------+
|                      ZUSTAND STORE ARCHITECTURE                              |
+-----------------------------------------------------------------------------+
|                                                                             |
|   STORE            | PURPOSE                          | PERSISTENCE         |
|   =================|==================================|=====================|
|   graphStore       | Nodes, edges, loading state      | No                  |
|                    | Progressive index building       |                     |
|                    | Node/edge lookup maps            |                     |
|                    |                                  |                     |
|   filterStore      | Node types, locale, presets      | localStorage        |
|                    | Realm/layer/trait checkboxes     |                     |
|                    |                                  |                     |
|   uiStore          | Navigation mode, panels,         | No                  |
|                    | selection state                  |                     |
|                    |                                  |                     |
|   queryStore       | Cypher query state, history      | sessionStorage      |
|                    | Query execution status           |                     |
|                    |                                  |                     |
|   viewStore        | Saved views (40 views)           | No (API-backed)     |
|                    | View execution                   |                     |
|                    |                                  |                     |
|   treeStore        | Unified tree expand/collapse     | No                  |
|                    | Lazy loading state               |                     |
|                    |                                  |                     |
|   chatStore        | AI chat messages, streaming      | sessionStorage      |
|                    |                                  |                     |
|   animationStore   | Graph animation controls         | No                  |
|                    |                                  |                     |
|   schemaStore      | Schema metadata cache            | No                  |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### ELK Layout Engine

```
+-----------------------------------------------------------------------------+
|                      ELK LAYOUT ENGINE                                       |
+-----------------------------------------------------------------------------+
|                                                                             |
|   ALGORITHM: elk.layered                                                    |
|   ============================                                              |
|                                                                             |
|   OPTIONS:                                                                  |
|   +-- algorithm: 'layered'                                                  |
|   +-- 'elk.direction': 'DOWN' | 'RIGHT'                                     |
|   +-- 'elk.spacing.nodeNode': 80                                            |
|   +-- 'elk.spacing.edgeNode': 40                                            |
|   +-- 'elk.layered.spacing.baseValue': 60                                   |
|   +-- 'elk.hierarchyHandling': 'INCLUDE_CHILDREN'                           |
|                                                                             |
|   LAYOUT PROCESS:                                                           |
|   1. Convert React Flow nodes/edges to ELK format                           |
|   2. Run elk.layout() async                                                 |
|   3. Map ELK positions back to React Flow nodes                             |
|   4. Apply positions with animation                                         |
|                                                                             |
|   SCHEMA MODE LAYOUT:                                                       |
|   +-- Realm groups at top level                                             |
|   +-- Layer groups nested within realms                                     |
|   +-- Class nodes within layers                                             |
|   +-- ArcFamily groups separate hierarchy                                   |
|   +-- ArcClass nodes within families                                        |
|                                                                             |
+-----------------------------------------------------------------------------+
```

---

## 11. Data Flow Diagrams

### YAML to Cypher Generation

```
+-----------------------------------------------------------------------------+
|                   YAML -> CYPHER GENERATION FLOW                             |
+-----------------------------------------------------------------------------+
|                                                                             |
|   +-----------------+                                                       |
|   |  Developer      |                                                       |
|   |  edits YAML     |                                                       |
|   +-----------------+                                                       |
|           |                                                                 |
|           | models/node-classes/org/semantic/entity.yaml                    |
|           v                                                                 |
|   +-----------------+                                                       |
|   |  cargo run --   |                                                       |
|   |  schema generate|                                                       |
|   +-----------------+                                                       |
|           |                                                                 |
|           | 1. Parse all YAML files                                         |
|           | 2. Validate against rules                                       |
|           | 3. Run 12 generators                                            |
|           v                                                                 |
|   +-----------------+     +-----------------+     +-----------------+       |
|   | Cypher Seeds    |     | TypeScript      |     | Mermaid Docs    |       |
|   | packages/db/    |     | packages/core/  |     | models/docs/    |       |
|   | seed/*.cypher   |     | src/graph/*.ts  |     | *.mmd           |       |
|   +-----------------+     +-----------------+     +-----------------+       |
|           |                                                                 |
|           | pnpm infra:seed                                                 |
|           v                                                                 |
|   +-----------------+                                                       |
|   |    Neo4j        |                                                       |
|   |  bolt://7687    |                                                       |
|   +-----------------+                                                       |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### MCP Request to Neo4j Query Flow

```
+-----------------------------------------------------------------------------+
|                  MCP REQUEST -> NEO4J FLOW                                   |
+-----------------------------------------------------------------------------+
|                                                                             |
|   +-----------------+                                                       |
|   |   AI Agent      |  (Claude / Nika)                                      |
|   +-----------------+                                                       |
|           |                                                                 |
|           | JSON-RPC: tools/call "novanet_generate"                         |
|           v                                                                 |
|   +-----------------+                                                       |
|   |   MCP Server    |  src/server/handler.rs                                |
|   |   (novanet-mcp) |  #[tool] macro dispatches                             |
|   +-----------------+                                                       |
|           |                                                                 |
|           | 1. Validate params                                              |
|           | 2. Check token budget                                           |
|           | 3. Build Cypher queries                                         |
|           v                                                                 |
|   +-----------------+                                                       |
|   |  tools/generate |  Phase 1: Structure (traverse)                        |
|   |      .rs        |  Phase 2: Entities (assemble)                         |
|   +-----------------+  Phase 3: Atoms (atoms)                               |
|           |            Phase 4: Anchors (references)                        |
|           |            Phase 5: Denomination forms (ADR-033)                |
|           v                                                                 |
|   +-----------------+                                                       |
|   |  Neo4j Pool     |  neo4j/pool.rs                                        |
|   |  (deadpool)     |  Connection pooling                                   |
|   +-----------------+                                                       |
|           |                                                                 |
|           | bolt://                                                         |
|           v                                                                 |
|   +-----------------+                                                       |
|   |    Neo4j        |  Execute Cypher queries                               |
|   |    5.26         |  Return rows                                          |
|   +-----------------+                                                       |
|           |                                                                 |
|           | Query results                                                   |
|           v                                                                 |
|   +-----------------+                                                       |
|   |   Response      |  Build GenerateResult                                 |
|   |   Assembly      |  - prompt                                             |
|   +-----------------+  - evidence_summary                                   |
|           |            - locale_context                                     |
|           |            - context_anchors                                    |
|           |            - denomination_forms                                 |
|           |            - token_usage                                        |
|           |            - context_build_log                                  |
|           v                                                                 |
|   +-----------------+                                                       |
|   |   AI Agent      |  Receives structured context                          |
|   +-----------------+  for content generation                               |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### Studio to Rust CLI Subprocess Flow

```
+-----------------------------------------------------------------------------+
|                STUDIO -> RUST CLI SUBPROCESS FLOW                            |
+-----------------------------------------------------------------------------+
|                                                                             |
|   +-----------------+                                                       |
|   |   User clicks   |                                                       |
|   |   filter/view   |                                                       |
|   +-----------------+                                                       |
|           |                                                                 |
|           | filterStore.setFilter()                                         |
|           v                                                                 |
|   +-----------------+                                                       |
|   |  Next.js API    |  /api/graph/query                                     |
|   |    Route        |                                                       |
|   +-----------------+                                                       |
|           |                                                                 |
|           | JSON filter object                                              |
|           v                                                                 |
|   +-----------------+                                                       |
|   |  novanetBridge  |  lib/novanetBridge.ts                                 |
|   |    .ts          |  spawn('cargo', ['run', '--', 'filter', 'build'])     |
|   +-----------------+                                                       |
|           |                                                                 |
|           | stdin: JSON filter                                              |
|           v                                                                 |
|   +-----------------+                                                       |
|   |  Rust CLI       |  commands/filter.rs                                   |
|   |  filter build   |  Parse JSON, generate Cypher                          |
|   +-----------------+                                                       |
|           |                                                                 |
|           | stdout: Cypher query                                            |
|           v                                                                 |
|   +-----------------+                                                       |
|   |  Neo4j Driver   |  @neo4j/driver                                        |
|   |  (TypeScript)   |  Execute query                                        |
|   +-----------------+                                                       |
|           |                                                                 |
|           | Query results                                                   |
|           v                                                                 |
|   +-----------------+                                                       |
|   |  graphStore     |  Set nodes, edges                                     |
|   |  .setData()     |  Build indexes progressively                          |
|   +-----------------+                                                       |
|           |                                                                 |
|           | React re-render                                                 |
|           v                                                                 |
|   +-----------------+                                                       |
|   |  React Flow     |  Display updated graph                                |
|   |  Canvas         |                                                       |
|   +-----------------+                                                       |
|                                                                             |
+-----------------------------------------------------------------------------+
```

---

## Appendix: File Locations

### Key Files Reference

```
+-----------------------------------------------------------------------------+
|                        KEY FILE LOCATIONS                                    |
+-----------------------------------------------------------------------------+
|                                                                             |
|   SCHEMA DEFINITIONS (Source of Truth)                                      |
|   ====================================                                      |
|   novanet-dev/packages/core/models/                                         |
|   +-- _index.yaml                    Model index                            |
|   +-- taxonomy.yaml                  Arc scopes, cardinalities, palette     |
|   +-- visual-encoding.yaml           Icons (web + terminal)                 |
|   +-- node-classes/                  61 node class YAML files               |
|   +-- arc-classes/                   182 arc class YAML files               |
|   +-- realms/                        Realm definitions                      |
|   +-- layers/                        Layer definitions                      |
|   +-- traits/                        Trait definitions                      |
|   +-- arc-families/                  Arc family definitions                 |
|   +-- views/                         View definitions (40 views)            |
|                                                                             |
|   RUST CLI                                                                  |
|   ========                                                                  |
|   novanet-dev/tools/novanet/                                                |
|   +-- src/main.rs                    Entry point                            |
|   +-- src/commands/                  13 command modules                     |
|   +-- src/generators/                12 generator modules                   |
|   +-- src/parsers/                   YAML parser modules                    |
|   +-- src/validation/                Validation + auto-fix                  |
|   +-- src/tui/                       TUI modules (ratatui)                  |
|                                                                             |
|   MCP SERVER                                                                |
|   ==========                                                                |
|   novanet-dev/tools/novanet-mcp/                                            |
|   +-- src/main.rs                    Entry point                            |
|   +-- src/server/                    Server, config, state                  |
|   +-- src/tools/                     7 tool implementations                 |
|   +-- src/neo4j/                     Connection pool                        |
|   +-- src/cache/                     Query cache (moka)                     |
|   +-- src/tokens/                    Token counting (tiktoken)              |
|                                                                             |
|   TYPESCRIPT STUDIO                                                         |
|   =================                                                         |
|   novanet-dev/apps/studio/                                                  |
|   +-- src/app/                       Next.js App Router                     |
|   +-- src/components/graph/          React Flow components                  |
|   +-- src/stores/                    Zustand stores (9)                     |
|   +-- src/config/                    Configuration files                    |
|   +-- src/hooks/                     Custom React hooks                     |
|                                                                             |
|   NEO4J SEEDS                                                               |
|   ===========                                                               |
|   novanet-dev/packages/db/seed/                                             |
|   +-- 00.5-taxonomy.cypher           Organizing principles                  |
|   +-- 01-classes.cypher              Node class definitions                 |
|   +-- 02-arc-classes.cypher          Arc class definitions                  |
|   +-- 03-autowire.cypher             Structural relationships               |
|   +-- 10-*.cypher                    Data seed files                        |
|                                                                             |
|   ADRS                                                                      |
|   ====                                                                      |
|   supernovae-agi/.claude/rules/adr/                                         |
|   +-- ADR-029-native-pattern.md      *Native suffix convention              |
|   +-- ADR-030-slug-ownership.md      Page owns URL                          |
|   +-- ADR-033-denomination-forms.md  Prescriptive naming                    |
|   +-- ADR-024-trait-data-origin.md   Trait definitions                      |
|   +-- ADR-022-unified-tree.md        2-mode navigation                      |
|   +-- ADR-021-query-first.md         Cypher as source of truth              |
|                                                                             |
+-----------------------------------------------------------------------------+
```

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| v0.14.0 | 2026-02-19 | Added context_build_log for debugging, MCP v0.4.0 |
| v0.13.1 | 2026-02-17 | Added 6th arc family (schema), 182 arcs |
| v0.13.0 | 2026-02-15 | *Native pattern (ADR-029), Slug ownership (ADR-030) |
| v0.12.5 | 2026-02-14 | Brand Architecture (ADR-028), 61 nodes |
| v0.12.0 | 2026-02-13 | SemVer transition, Class/Instance terminology |

---

**Document maintained by:** NovaNet Engineering
**Last review:** 2026-02-19
