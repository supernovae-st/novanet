# SEO Discovery Workflows

**Nika workflows pour peupler NovaNet avec des données SEO.**

Ces workflows progressent du plus simple au plus complexe, permettant de tester chaque composant avant d'exécuter le pipeline complet.

---

## Vue d'ensemble

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  PROGRESSION DES WORKFLOWS                                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  NIVEAU 1: 01-test-novanet-write                                            │
│  ──────────────────────────────                                             │
│  But: Vérifier que novanet_write fonctionne                                 │
│  MCP: novanet                                                               │
│  Verbes: invoke                                                             │
│                                                                             │
│  NIVEAU 2: 02-seo-discovery-single                                          │
│  ──────────────────────────────────                                         │
│  But: 1 Entity + 1 Locale (fr-FR)                                           │
│  MCP: novanet, dataforseo                                                   │
│  Verbes: invoke, infer                                                      │
│                                                                             │
│  NIVEAU 3: 03-seo-discovery-multi-locale                                    │
│  ──────────────────────────────────────                                     │
│  But: 1 Entity + 5 Locales (for_each)                                       │
│  MCP: novanet, dataforseo                                                   │
│  Verbes: invoke, infer, for_each                                            │
│                                                                             │
│  NIVEAU 4: 04-seo-discovery-with-terms                                      │
│  ───────────────────────────────────                                        │
│  But: SEO + Terms (USES_TERM)                                               │
│  MCP: novanet, dataforseo, perplexity                                       │
│  Verbes: invoke, infer, for_each                                            │
│                                                                             │
│  NIVEAU 5: 05-seo-discovery-full                                            │
│  ────────────────────────────────                                           │
│  But: 200 locales + multi-agent                                             │
│  MCP: novanet, dataforseo, perplexity, firecrawl                            │
│  Verbes: invoke, infer, agent, exec, for_each                               │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Architecture Cible

```
                                DATAFORSEO API
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           NIKA WORKFLOW                                      │
│                                                                             │
│  ┌───────────┐    ┌───────────┐    ┌───────────┐    ┌───────────┐          │
│  │  DISCOVER │───►│  ANALYZE  │───►│   WRITE   │───►│  VERIFY   │          │
│  │ (DataFor) │    │   (LLM)   │    │ (NovaNet) │    │ (NovaNet) │          │
│  └───────────┘    └───────────┘    └───────────┘    └───────────┘          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         NOVANET KNOWLEDGE GRAPH                             │
│                                                                             │
│  Entity (qr-code)                                                           │
│      │                                                                      │
│      └─[:HAS_NATIVE]─► EntityNative (fr-FR)                                │
│                            │                                                │
│                            ├─[:TARGETS]─────► SEOKeyword                    │
│                            │                      │                         │
│                            │                      └─ value: "créer qr code" │
│                            │                      └─ volume: 14000          │
│                            │                      └─ is_slug_source: true   │
│                            │                                                │
│                            └─[:USES_TERM]───► Term                          │
│                                                  │                          │
│                                                  └─ value: "code QR"        │
│                                                  └─ domain: technical       │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Prérequis

### Variables d'environnement

```bash
# NovaNet (Neo4j)
export NEO4J_PASSWORD="novanetpassword"

# DataForSEO API
export DATAFORSEO_LOGIN="your-login"
export DATAFORSEO_PASSWORD="your-password"

# Perplexity (optionnel, niveau 4+)
export PERPLEXITY_API_KEY="your-key"

# Firecrawl (optionnel, niveau 5)
export FIRECRAWL_API_KEY="your-key"
```

### Services requis

```bash
# Neo4j doit être démarré
cd /Users/thibaut/dev/supernovae/novanet
pnpm infra:up

# Vérifier la connexion
curl http://localhost:7474
```

---

## Exécution

### Niveau 1: Test basique

```bash
cd /Users/thibaut/dev/supernovae/nika
cargo run -- run ../novanet/docs/sessions/2026-03-03-qrcode-seo-workflow/workflows/01-test-novanet-write.nika.yaml
```

### Niveau 2: Single locale

```bash
cargo run -- run ../novanet/docs/sessions/2026-03-03-qrcode-seo-workflow/workflows/02-seo-discovery-single.nika.yaml
```

### Niveau 3-5: Progressive complexity

```bash
# Multi-locale (5 locales)
cargo run -- run workflows/03-seo-discovery-multi-locale.nika.yaml

# Avec Terms
cargo run -- run workflows/04-seo-discovery-with-terms.nika.yaml

# Pipeline complet (200 locales)
cargo run -- run workflows/05-seo-discovery-full.nika.yaml
```

---

## Formule de Scoring SEO

```
score = volume × sem_coef × intent_boost × trend_factor
```

### Coefficients sémantiques (sem_coef)

| Relation | Coef | Description |
|----------|------|-------------|
| `same_as` | 1.00 | Même concept que l'entity |
| `action_for` | 0.95 | Action sur l'entity (créer, générer) |
| `produces` | 0.85 | Produit de l'entity |
| `subtopic_of` | 0.70 | Sous-thème de l'entity |
| `related_to` | 0.50 | Lié à l'entity |
| `attribute_of` | 0.30 | Attribut de l'entity |

### Intent boost

| Intent | Boost |
|--------|-------|
| `transactional` | 1.2 |
| `commercial` | 1.1 |
| `informational` | 1.0 |
| `navigational` | 0.8 |

### Trend factor

| Trend | Factor |
|-------|--------|
| `rising` | 1.2 |
| `stable` | 1.0 |
| `declining` | 0.7 |

### Exemple

```
Keyword: "créer qr code gratuit"
Volume: 14000
Relation: action_for (0.95)
Intent: transactional (1.2)
Trend: stable (1.0)

Score = 14000 × 0.95 × 1.2 × 1.0 = 15,960
```

---

## Structure des clés NovaNet

### SEOKeyword

```
Pattern: seo:{slug}@{locale}
Example: seo:creer-qr-code@fr-FR
```

### EntityNative

```
Pattern: entity:{entity_key}@{locale}
Example: entity:qr-code@fr-FR
```

### Term

```
Pattern: {term_key}
Example: qr_code
```

---

## Arcs créés

| Arc | From | To | Description |
|-----|------|-----|-------------|
| `TARGETS` | EntityNative | SEOKeyword | Keyword ciblé |
| `USES_TERM` | EntityNative | Term | Vocabulaire utilisé |
| `CONTAINS_TERM` | TermSet | Term | Appartenance au set |
| `FOR_LOCALE` | EntityNative | Locale | Locale cible |

### Propriétés de l'arc TARGETS

```yaml
rank: primary | secondary | tertiary
is_slug_source: true | false
semantic_relation: same_as | action_for | produces | ...
```

---

## Debugging

### Vérifier les keywords écrits

```cypher
MATCH (en:EntityNative {key: 'entity:qr-code@fr-FR'})-[t:TARGETS]->(kw:SEOKeyword)
RETURN kw.value, kw.volume, kw.intent, t.rank, t.is_slug_source
ORDER BY kw.volume DESC
```

### Vérifier les terms liés

```cypher
MATCH (en:EntityNative {key: 'entity:qr-code@fr-FR'})-[:USES_TERM]->(t:Term)
RETURN t.key, t.value, t.domain
```

### Stats par locale

```cypher
MATCH (en:EntityNative)-[:TARGETS]->(kw:SEOKeyword)
WHERE en.entity_key = 'qr-code'
RETURN en.locale_key, count(kw) AS keywords, sum(kw.volume) AS total_volume
ORDER BY total_volume DESC
```

---

## Prochaines étapes

1. **06-content-generation.nika.yaml** - Générer le contenu avec `novanet_generate`
2. **07-full-pipeline.nika.yaml** - SEO + Content en un seul workflow
3. **Scheduled runs** - Cron pour refresh mensuel des données SEO

---

## Références

- [NovaNet MCP CLAUDE.md](/tools/novanet-mcp/CLAUDE.md)
- [Write Philosophy](/tools/novanet-mcp/.claude/rules/write-philosophy.md)
- [Nika Workflow Docs](/nika/docs/workflows/)
- [ADR-029: *Native Pattern](/dx/adr/novanet/adr-029-native-pattern.md)
- [ADR-033: Denomination Forms](/dx/adr/novanet/adr-033-denomination-forms.md)
