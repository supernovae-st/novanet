# NovaNet Neo4j Setup (v7.1.0)

Neo4j 5.x avec APOC pour le modèle de données NovaNet.

## Quick Start

```bash
# 1. Lancer Neo4j (depuis la racine du monorepo)
npm run infra:up

# 2. Attendre que Neo4j soit prêt (~30s)
npm run infra:logs
# Attendre "Started."

# 3. Exécuter le seed
cd core/neo4j && ./seed.sh

# 4. Ouvrir Neo4j Browser
open http://localhost:7474
```

## Credentials

| Key | Value |
|-----|-------|
| URL | http://localhost:7474 |
| Bolt | bolt://localhost:7687 |
| User | `neo4j` |
| Password | `novanetpassword` |

## Structure des fichiers

```
neo4j/                            # Dans core/
├── seed.sh                       # Script d'exécution du seed
├── README.md                     # Ce fichier
└── seed/
    ├── 00-constraints.cypher     # Contraintes et index
    ├── 01-concepts-mvp.cypher    # MVP data (5 concepts x 20 locales)
    └── 02-locale-knowledge.cypher # Locale Knowledge data

# Docker config (au niveau monorepo)
infra/
└── docker-compose.yml            # Neo4j 5.x + APOC
```

## v7.1.0 Graph-Native Locale

Content nodes use `:FOR_LOCALE` relation:

```
ConceptL10n ──[:FOR_LOCALE]──▶ Locale {key: "fr-FR"}
PageL10n  ──[:FOR_LOCALE]──▶ Locale {key: "fr-FR"}
BlockL10n ──[:FOR_LOCALE]──▶ Locale {key: "fr-FR"}
```

### Locale Knowledge Architecture

```
Locale {key: "fr-FR"}
    │
    ├──[:HAS_IDENTITY]──▶ LocaleIdentity
    ├──[:HAS_VOICE]─────▶ LocaleVoice
    ├──[:HAS_CULTURE]───▶ LocaleCulture
    ├──[:HAS_MARKET]────▶ LocaleMarket
    └──[:HAS_LEXICON]───▶ LocaleLexicon ──[:HAS_EXPRESSION]──▶ Expression
```

## Commandes Docker

```bash
# Depuis la racine du monorepo:

# Démarrer
npm run infra:up

# Arrêter
npm run infra:down

# Reset complet (supprimer les données)
npm run infra:down
docker volume rm novanet_neo4j_data
npm run infra:up && cd core/neo4j && ./seed.sh

# Logs
npm run infra:logs

# Cypher shell
docker exec -it novanet-neo4j cypher-shell -u neo4j -p novanetpassword
```

## Queries (v7.1.0)

### Count nodes by type

```cypher
MATCH (n)
RETURN labels(n)[0] AS label, count(*) AS count
ORDER BY count DESC;
```

### Load Locale with Knowledge

```cypher
MATCH (l:Locale {key: $locale})
OPTIONAL MATCH (l)-[:HAS_IDENTITY]->(li:LocaleIdentity)
OPTIONAL MATCH (l)-[:HAS_VOICE]->(lv:LocaleVoice)
OPTIONAL MATCH (l)-[:HAS_CULTURE]->(lc:LocaleCulture)
OPTIONAL MATCH (l)-[:HAS_MARKET]->(lm:LocaleMarket)
OPTIONAL MATCH (l)-[:HAS_LEXICON]->(ll:LocaleLexicon)-[:HAS_EXPRESSION]->(e:Expression)
RETURN l, li, lv, lc, lm, ll, collect(e) AS expressions
```

### Load Block Context

```cypher
MATCH (b:Block {key: $blockKey})-[:OF_TYPE]->(bt:BlockType)
MATCH (b)-[:USES_CONCEPT]->(c:Concept)-[:HAS_L10N]->(cl:ConceptL10n)-[:FOR_LOCALE]->(l:Locale {key: $locale})
MATCH (l)-[:HAS_VOICE]->(lv:LocaleVoice)
RETURN b.instructions, c.key, cl.title, cl.definition, bt.rules, lv.formality_score, lv.default_formality
```

### Spreading Activation

```cypher
MATCH (c:Concept {key: "tier-pro"})-[r:SEMANTIC_LINK*1..2]->(c2:Concept)
WHERE ALL(rel IN r WHERE rel.temperature >= 0.3)
WITH c2, reduce(a = 1.0, rel IN r | a * rel.temperature) AS activation
WHERE activation >= 0.3
MATCH (c2)-[:HAS_L10N]->(cl:ConceptL10n)-[:FOR_LOCALE]->(l:Locale {key: $locale})
RETURN c2.key, cl.title, activation
ORDER BY activation DESC
```

### Fallback Chain

```cypher
MATCH (l:Locale {key: $locale})-[:FALLBACK_TO*0..3]->(fallback)
RETURN fallback.key AS locale
ORDER BY length((l)-[:FALLBACK_TO*]->(fallback))
```

## Key Relations (v7.1.0)

| Relation | From | To | Description |
|----------|------|----|-------------|
| `:SUPPORTS_LOCALE` | Project | Locale | Project supports locale |
| `:FALLBACK_TO` | Locale | Locale | Fallback chain |
| `:FOR_LOCALE` | Content | Locale | Content targets locale |
| `:HAS_IDENTITY` | Locale | LocaleIdentity | Technical identity |
| `:HAS_VOICE` | Locale | LocaleVoice | Tone & formality |
| `:HAS_CULTURE` | Locale | LocaleCulture | Cultural context |
| `:HAS_MARKET` | Locale | LocaleMarket | Market data |
| `:HAS_LEXICON` | Locale | LocaleLexicon | Vocabulary |
| `:HAS_EXPRESSION` | LocaleLexicon | Expression | Expressions |
| `:HAS_L10N` | Concept | ConceptL10n | Localized content |
| `:HAS_OUTPUT` | Page/Block | Output | Generated content |
| `:SEMANTIC_LINK` | Concept | Concept | Spreading activation |
| `:TARGETS_SEO` | Concept | SEOKeywordL10n | SEO targeting |
| `:TARGETS_GEO` | Concept | GEOSeedL10n | GEO targeting |

## See Also

- `models/GRAPH.md` - Full graph visualization
- `models/LOCALE-INDEX.md` - Locale-based queries
- `models/_index.yaml` - Model registry
- `docs/orchestrator.md` - Generation flow
