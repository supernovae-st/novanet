---
name: spreading-activation
description: Use when implementing semantic traversal, concept discovery, context expansion, or relevance-weighted graph navigation
---

# Spreading Activation Skill

Implement cognitive-inspired graph traversal for intelligent context discovery.

## When to Use

- Discovering related concepts from seed concepts
- Building context for LLM generation
- Implementing relevance-weighted navigation
- Expanding query scope intelligently
- Finding semantic clusters

## Theory: Spreading Activation

Spreading activation is a cognitive model where:
1. **Seed nodes** start with activation = 1.0
2. **Activation propagates** through relationships
3. **Decay applies** at each hop (temperature)
4. **Cutoff stops** propagation below threshold
5. **Collected nodes** form relevant context

```
                    ┌─────────────┐
                    │  tier-pro   │ activation: 1.0
                    │   (SEED)    │
                    └──────┬──────┘
                           │
           ┌───────────────┼───────────────┐
           │ temp: 0.85    │ temp: 0.60    │ temp: 0.75
           ▼               ▼               ▼
    ┌──────────┐    ┌──────────┐    ┌──────────┐
    │ analytics│    │bulk-create│   │ security │
    │ act: 0.85│    │ act: 0.60│    │ act: 0.75│
    └────┬─────┘    └──────────┘    └────┬─────┘
         │ temp: 0.80                    │ temp: 0.50
         ▼                               ▼
    ┌──────────┐                    ┌──────────┐
    │ tracking │                    │ gdpr     │
    │ act: 0.68│                    │ act: 0.38│
    └──────────┘                    └──────────┘
                                         │ temp: 0.70
                                         ▼
                                    ┌──────────┐
                                    │ privacy  │
                                    │ act: 0.27│ ← BELOW CUTOFF (0.3)
                                    └──────────┘
```

## Configuration

```yaml
spreading_config:
  cutoff: 0.3        # Stop if activation < 0.3
  max_depth: 2       # Maximum 2 hops from seed
  decay: 0.9         # Optional: multiply each hop
  strategy: product  # product | min | weighted_avg
```

### Strategies

| Strategy | Formula | Use Case |
|----------|---------|----------|
| `product` | `a1 × a2 × ... × an` | Default, strong decay |
| `min` | `min(a1, a2, ..., an)` | Bottleneck paths |
| `weighted_avg` | `Σ(ai × wi) / Σwi` | Balanced paths |

## Core Cypher Patterns

### Basic Spreading (Single Seed)

```cypher
// Parameters: $seedKey, $cutoff, $maxDepth
MATCH path = (seed:Concept {key: $seedKey})
             -[rels:SEMANTIC_LINK*1..$maxDepth]->
             (target:Concept)
WHERE ALL(r IN rels WHERE r.temperature >= $cutoff)
WITH target,
     reduce(activation = 1.0, r IN rels | activation * r.temperature) AS activation,
     [r IN rels | r.type] AS path_types,
     length(path) AS depth
WHERE activation >= $cutoff
RETURN target.key AS concept,
       activation,
       path_types,
       depth
ORDER BY activation DESC;
```

### Multi-Seed Spreading

```cypher
// Parameters: $seedKeys (array), $cutoff, $maxDepth
UNWIND $seedKeys AS seedKey
MATCH (seed:Concept {key: seedKey})
OPTIONAL MATCH path = (seed)-[rels:SEMANTIC_LINK*1..$maxDepth]->(target:Concept)
WHERE ALL(r IN rels WHERE r.temperature >= $cutoff)
WITH seed, target,
     CASE WHEN target IS NULL THEN 1.0
          ELSE reduce(a = 1.0, r IN rels | a * r.temperature)
     END AS activation
WHERE activation >= $cutoff
WITH target, max(activation) AS best_activation, collect(seed.key) AS from_seeds
RETURN target.key AS concept,
       best_activation AS activation,
       from_seeds
ORDER BY activation DESC;
```

### Spreading with Content Loading

```cypher
// Full context: spreading + localized content
MATCH (seed:Concept {key: $seedKey})
       -[rels:SEMANTIC_LINK*0..2]->
       (c:Concept)
WHERE ALL(r IN rels WHERE r.temperature >= 0.3)
WITH c,
     CASE WHEN length(rels) = 0 THEN 1.0
          ELSE reduce(a = 1.0, r IN rels | a * r.temperature)
     END AS activation
WHERE activation >= 0.3

// Load localized content
MATCH (c)-[:HAS_L10N]->(cc:ConceptL10n {locale: $locale})

// Load SEO/GEO targeting (v7.0.0)
OPTIONAL MATCH (c)-[:TARGETS_SEO]->(seo:SEOKeywordL10n)-[:FOR_LOCALE]->(l:Locale {key: $locale})
OPTIONAL MATCH (c)-[:TARGETS_GEO]->(geo:GEOSeedL10n)-[:FOR_LOCALE]->(l)

RETURN c.key AS concept,
       activation,
       cc {.title, .definition, .benefits} AS content,
       collect(DISTINCT seo.value) AS seo_keywords,
       collect(DISTINCT geo.value) AS geo_seeds
ORDER BY activation DESC;
```

### Bidirectional Spreading

```cypher
// Spread in both directions (incoming + outgoing)
MATCH (seed:Concept {key: $seedKey})

// Outgoing links
OPTIONAL MATCH outPath = (seed)-[outRels:SEMANTIC_LINK*1..2]->(outTarget:Concept)
WHERE ALL(r IN outRels WHERE r.temperature >= 0.3)

// Incoming links (concepts that point to seed)
OPTIONAL MATCH inPath = (inTarget:Concept)-[inRels:SEMANTIC_LINK*1..2]->(seed)
WHERE ALL(r IN inRels WHERE r.temperature >= 0.3)

WITH collect({
       concept: outTarget.key,
       activation: reduce(a = 1.0, r IN outRels | a * r.temperature),
       direction: 'outgoing'
     }) + collect({
       concept: inTarget.key,
       activation: reduce(a = 1.0, r IN inRels | a * r.temperature),
       direction: 'incoming'
     }) AS results

UNWIND results AS r
WHERE r.activation >= 0.3
RETURN r.concept, r.activation, r.direction
ORDER BY r.activation DESC;
```

## Advanced Patterns

### Decay by Hop Count

```cypher
// Apply additional decay factor per hop
WITH 0.9 AS hopDecay
MATCH path = (seed:Concept {key: $seedKey})
             -[rels:SEMANTIC_LINK*1..3]->(target:Concept)
WITH target,
     reduce(a = 1.0, r IN rels | a * r.temperature) *
     (hopDecay ^ length(rels)) AS activation
WHERE activation >= 0.3
RETURN target.key, activation;
```

### Relationship Type Weighting

```cypher
// Different weights for different relationship types
WITH {
  is_action_on: 1.0,
  includes: 0.9,
  type_of: 0.85,
  related_to: 0.7
} AS typeWeights

MATCH path = (seed:Concept {key: $seedKey})
             -[rels:SEMANTIC_LINK*1..2]->(target:Concept)
WITH target,
     reduce(a = 1.0, r IN rels |
       a * r.temperature * coalesce(typeWeights[r.type], 0.5)
     ) AS activation
WHERE activation >= 0.3
RETURN target.key, activation
ORDER BY activation DESC;
```

### Exclusion Patterns

```cypher
// Exclude certain concepts or paths
MATCH path = (seed:Concept {key: $seedKey})
             -[rels:SEMANTIC_LINK*1..2]->(target:Concept)
WHERE ALL(r IN rels WHERE r.temperature >= 0.3)
  AND NOT target.key IN $excludeKeys
  AND NOT any(n IN nodes(path)[1..-1] WHERE n.type = 'deprecated')
WITH target,
     reduce(a = 1.0, r IN rels | a * r.temperature) AS activation
WHERE activation >= 0.3
RETURN target.key, activation;
```

### Cluster Detection

```cypher
// Find concept clusters (connected components)
MATCH (c:Concept)
WHERE EXISTS((c)-[:SEMANTIC_LINK {temperature: 0.7..1.0}]-())
WITH c
CALL {
  WITH c
  MATCH (c)-[:SEMANTIC_LINK*1..3 {temperature: 0.7..1.0}]-(connected:Concept)
  RETURN collect(DISTINCT connected.key) + [c.key] AS cluster
}
WITH cluster
WHERE size(cluster) >= 3
RETURN cluster, size(cluster) AS size
ORDER BY size DESC;
```

## Integration with NovaNet

### Block Context Loading with Spreading

```cypher
// Sub-agent loads context for a block
MATCH (b:Block {key: $blockKey})

// Direct concepts from block
MATCH (b)-[uc:USES_CONCEPT]->(direct:Concept)

// Spread from each direct concept
OPTIONAL MATCH spreadPath = (direct)-[rels:SEMANTIC_LINK*1..2]->(spread:Concept)
WHERE ALL(r IN rels WHERE r.temperature >= 0.3)

WITH b, direct, uc,
     collect({
       concept: spread,
       activation: reduce(a = uc.temperature, r IN rels | a * r.temperature)
     }) AS spreads

// Combine direct + spread concepts
WITH b,
     collect({concept: direct, activation: uc.temperature, source: 'direct'}) +
     [s IN spreads WHERE s.activation >= 0.3 |
       {concept: s.concept, activation: s.activation, source: 'spread'}
     ] AS allConcepts

UNWIND allConcepts AS ac
MATCH (ac.concept)-[:HAS_L10N]->(cc:ConceptL10n {locale: $locale})

RETURN ac.concept.key AS concept,
       ac.activation AS activation,
       ac.source AS source,
       cc.title AS title,
       cc.definition AS definition
ORDER BY activation DESC;
```

## Performance Considerations

1. **Limit depth** - Each hop multiplies result set
2. **Apply cutoff early** - Filter in path pattern, not after
3. **Index seed lookups** - Ensure Concept.key is indexed
4. **Batch spreading** - Use UNWIND for multiple seeds
5. **Cache results** - Spreading is deterministic, cache when possible

```cypher
-- Create index for spreading performance
CREATE INDEX concept_key IF NOT EXISTS FOR (c:Concept) ON (c.key);
CREATE INDEX semantic_link_temp IF NOT EXISTS FOR ()-[r:SEMANTIC_LINK]-() ON (r.temperature);
```

## Debugging Spreading

```cypher
// Visualize spreading path
MATCH path = (seed:Concept {key: $seedKey})
             -[rels:SEMANTIC_LINK*1..3]->(target:Concept)
WHERE ALL(r IN rels WHERE r.temperature >= 0.3)
RETURN path,
       [n IN nodes(path) | n.key] AS node_keys,
       [r IN rels | {type: r.type, temp: r.temperature}] AS edge_info,
       reduce(a = 1.0, r IN rels | a * r.temperature) AS final_activation;
```
