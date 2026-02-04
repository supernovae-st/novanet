# Locale-Based Node Index v7.6.0

How to query nodes for a specific locale.

## The :FOR_LOCALE Pattern

All localized content connects to Locale via `:FOR_LOCALE` relation:

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│                                    LOCALE: fr-FR                                        │
│                                                                                         │
│   ┌───────────────────────────────────────────────────────────────────────────────────┐ │
│   │  DIRECTLY LINKED (via :FOR_LOCALE)                                                │ │
│   │                                                                                   │ │
│   │  Project Layer:                                                                   │ │
│   │  └── ProjectL10n        "Identité et messaging localisés (CTAs, SEO)"             │ │
│   │                                                                                   │ │
│   │  Content Layer:                                                                   │ │
│   │  ├── ConceptL10n        "Définitions de concepts"                                 │ │
│   │  ├── PageL10n           "Pages générées"                                          │ │
│   │  └── BlockL10n          "Blocs générés"                                           │ │
│   │                                                                                   │ │
│   │  SEO/GEO Layer:                                                                   │ │
│   │  ├── SEOKeywordL10n         "Mots-clés SEO"                                           │ │
│   │  └── GEOSeedL10n            "Seeds GEO"                                               │ │
│   │                                                                                   │ │
│   └───────────────────────────────────────────────────────────────────────────────────┘ │
│                                                                                         │
│   ┌───────────────────────────────────────────────────────────────────────────────────┐ │
│   │  LOCALE KNOWLEDGE (via :HAS_*)                                                    │ │
│   │                                                                                   │ │
│   │  ├── LocaleIdentity     "Script, timezone, format defaults"                       │ │
│   │  ├── LocaleVoice        "Formality, tone, pronouns"                               │ │
│   │  ├── LocaleCulture      "Norms, taboos, celebrations"                             │ │
│   │  ├── LocaleMarket       "Demographics, competitors"                               │ │
│   │  ├── LocaleLexicon      "Vocabulary preferences"                                  │ │
│   │  └── Expression[]       "Semantic field expressions"                              │ │
│   │                                                                                   │ │
│   └───────────────────────────────────────────────────────────────────────────────────┘ │
│                                                                                         │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Query Patterns by Locale

### 1. Get All Content for a Locale

```cypher
// All localized content for fr-FR
MATCH (n)-[:FOR_LOCALE]->(l:Locale {key: 'fr-FR'})
RETURN labels(n)[0] AS type, count(*) AS count
ORDER BY count DESC
```

### 2. Get Project Context for a Locale

```cypher
// Full project context for content generation
MATCH (p:Project {key: $project_key})
MATCH (p)-[:HAS_L10N]->(pl:ProjectL10n)-[:FOR_LOCALE]->(l:Locale {key: $locale})
MATCH (p)-[:HAS_BRAND_IDENTITY]->(bi:BrandIdentity)
RETURN p, pl, bi
```

### 3. Get Concept Definitions for a Locale

```cypher
// Concepts with localized definitions
MATCH (c:Concept)-[:HAS_L10N]->(cl:ConceptL10n)-[:FOR_LOCALE]->(l:Locale {key: $locale})
RETURN c.key, cl.title, cl.definition
ORDER BY c.key
```

### 4. Get Locale Knowledge

```cypher
// All locale knowledge for generation context
MATCH (l:Locale {key: $locale})
OPTIONAL MATCH (l)-[:HAS_IDENTITY]->(id:LocaleIdentity)
OPTIONAL MATCH (l)-[:HAS_VOICE]->(v:LocaleVoice)
OPTIONAL MATCH (l)-[:HAS_CULTURE]->(c:LocaleCulture)
OPTIONAL MATCH (l)-[:HAS_MARKET]->(m:LocaleMarket)
OPTIONAL MATCH (l)-[:HAS_LEXICON]->(lex:LocaleLexicon)-[:HAS_EXPRESSION]->(e:Expression)
RETURN l, id, v, c, m, lex, collect(e) AS expressions
```

### 5. Get Page Content for a Locale

```cypher
// Page with all blocks for a locale
MATCH (p:Page {key: $page_key})-[:HAS_OUTPUT]->(pl:PageL10n)-[:FOR_LOCALE]->(l:Locale {key: $locale})
MATCH (p)-[:HAS_BLOCK]->(b:Block)-[:HAS_OUTPUT]->(bl:BlockL10n)-[:FOR_LOCALE]->(l)
RETURN p.key, pl.assembled, collect({block: b.key, output: bl.generated}) AS blocks
```

### 6. Get SEO Keywords for a Locale

```cypher
// SEO keywords with variations
MATCH (kw:SEOKeywordL10n)-[:FOR_LOCALE]->(l:Locale {key: $locale})
OPTIONAL MATCH (kw)-[:HAS_VARIATION]->(v:SEOVariation)
RETURN kw.value, kw.volume, kw.difficulty, collect(v.value) AS variations
ORDER BY kw.volume DESC
```

## Fallback Chain

When content is missing for a locale, follow the fallback chain:

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   fr-CA     │────►│    fr-FR    │────►│    en-US    │
│  (French    │     │  (French    │     │  (English   │
│   Canada)   │     │   France)   │     │   US)       │
└─────────────┘     └─────────────┘     └─────────────┘
     :FALLBACK_TO        :FALLBACK_TO
```

```cypher
// Query with fallback
MATCH (l:Locale {key: $locale})
MATCH path = (l)-[:FALLBACK_TO*0..3]->(fallback:Locale)
WITH collect(fallback) AS chain
UNWIND chain AS locale
MATCH (c:Concept {key: $concept_key})-[:HAS_L10N]->(cl:ConceptL10n)-[:FOR_LOCALE]->(locale)
RETURN cl
LIMIT 1
```

## Node Counts by Locale (Example)

| Node Type | en-US | fr-FR | es-MX | ja-JP |
|-----------|-------|-------|-------|-------|
| ConceptL10n | 100 | 100 | 50 | 30 |
| PageL10n | 10 | 10 | 5 | 3 |
| BlockL10n | 40 | 40 | 20 | 12 |
| SEOKeywordL10n | 500 | 300 | 200 | 100 |
| GEOSeedL10n | 50 | 30 | 20 | 10 |
| LocaleKnowledge | 6 | 6 | 6 | 6 |

## Context Budget by Priority

When loading context for LLM, filter by priority:

```cypher
// Load high-priority content only
MATCH (n)-[:FOR_LOCALE]->(l:Locale {key: $locale})
WHERE n.priority IN ['critical', 'high']
RETURN n
```

| Priority | Use Case |
|----------|----------|
| `critical` | Always include (brand, core concepts) |
| `high` | Include in most contexts |
| `medium` | Include when relevant |
| `low` | Include only when specifically needed |
