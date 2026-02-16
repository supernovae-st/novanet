# NovaNet Self-Descriptive LLM Ontology Analysis

**Date**: 2026-02-16
**Version**: v0.13.0
**Focus**: Entity/EntityNative patterns, spreading activation, ontology comparison

---

## Executive Summary

NovaNet implements a **self-describing context graph** where every node carries operational metadata for LLM consumption via the `llm_context` pattern. This analysis evaluates the pattern's sufficiency, identifies gaps compared to established ontology standards (OWL, SKOS, Schema.org), and provides recommendations for LLM optimization.

**Key Findings**:
1. The `llm_context` USE/TRIGGERS/NOT/RELATES pattern is **sufficient for basic disambiguation** but lacks formal reasoning capabilities
2. EntityNative **should add** `semantic_field` for domain clustering and token-efficient loading
3. Spreading activation is **well-designed** with temperature thresholds and task-specific modifiers
4. **Traversal hints are partially implemented** via `temperature_threshold` on arcs but could be enhanced
5. NovaNet is **stronger than SKOS, weaker than OWL** for formal reasoning, with unique strengths in LLM optimization

---

## Question 1: Is the llm_context Pattern Sufficient?

### Current Implementation

The `llm_context` pattern uses four components:

```
USE: [when to load this node into context]
TRIGGERS: [keywords that activate this node]
NOT: [disambiguation - what this is NOT]
RELATES: [connected nodes and their roles]
```

**Example from Entity**:
```yaml
llm_context: |
  USE: when discussing QR codes, scanning, 2D barcodes, or loading semantic context.
  TRIGGERS: "qr", "qr code", "scan", "code qr", "barcode 2D".
  NOT: for 1D barcodes (use Barcode entity), for smart links without QR (use Smart Link entity).
  RELATES: Project (owner via HAS_ENTITY), EntityNative (localized via HAS_NATIVE),
           Page (referencer via USES_ENTITY), Block (referencer via USES_ENTITY).
```

### Evaluation: Entity vs EntityNative Differentiation

| Criterion | Entity | EntityNative | Verdict |
|-----------|--------|--------------|---------|
| **Role clarity** | "invariant semantic unit" | "locale-native content" | Clear |
| **When to use** | "loading semantic context" | "loading localized entity data" | Clear |
| **What NOT to use** | "not for locale content" | "not for invariant definition" | Clear |
| **Traversal guidance** | Lists HAS_NATIVE | Lists NATIVE_OF | Sufficient |

**Assessment**: The pattern **is sufficient** for basic LLM understanding of when to use Entity vs EntityNative. The key differentiator is explicit:

- Entity: Load for **semantic understanding** (what IS this concept?)
- EntityNative: Load for **content generation** in a specific locale (how to EXPRESS this concept?)

### Gaps Identified

1. **No priority weighting**: All RELATES items appear equal, but some are more important
2. **No token cost estimation**: LLM doesn't know which path is cheaper
3. **No reasoning rules**: Unlike OWL, can't infer "if Entity.is_pillar, then EntityNative must exist for all priority locales"

### Recommendation: Enhance llm_context with Priority

```yaml
llm_context: |
  USE: when loading semantic context for content generation.
  TRIGGERS: "qr", "qr code", "scan" [weight: 0.9], "barcode 2D" [weight: 0.6].
  NOT: for 1D barcodes (use Barcode entity).
  RELATES:
    - EntityNative [priority: 1, tokens: ~200] (localized content via HAS_NATIVE)
    - Page [priority: 2, tokens: ~50] (referencer via USES_ENTITY)
    - SEOKeyword [priority: 3, tokens: ~30] (targeting via TARGETS)
```

---

## Question 2: Should EntityNative Include semantic_field?

### Current State

EntityNative has:
- `curation_status`: human_authored, machine_translated, ai_generated, ai_generated_reviewed
- `audience_segment`: professional, consumer, developer, enterprise
- No semantic_field or domain tags

Entity has:
- `[:BELONGS_TO] -> EntityCategory` (THING, ACTION, FEATURE, etc.)
- EntityCategory.question (WHAT?, HOW?, WHY?)
- No domain/industry tagging

### Gap Analysis

**Problem**: When loading context for a Block about "QR codes for restaurants", the LLM needs:
1. Restaurant domain vocabulary (menus, reservations, contactless)
2. QR code technical terms
3. Industry-specific benefits

Currently, this requires:
1. Load Entity:qr-code
2. Load EntityNative:qr-code@fr-FR
3. Traverse SEMANTIC_LINK to find restaurant-related entities
4. Hope the link_type helps identify relevance

**Missing**: Direct domain clustering for token-efficient subgraph loading.

### Recommendation: Add semantic_field to EntityNative

```yaml
# Proposed addition to entity-native.yaml
properties:
  semantic_field:
    type: string[]
    required: false
    indexed: true
    description: "Domain/industry tags for clustering and context loading"
    examples:
      - ["hospitality", "contactless", "menu"]
      - ["marketing", "analytics", "tracking"]
      - ["security", "authentication", "access-control"]
    llm_context: |
      USE: for filtering EntityNative nodes by domain when loading context.
      Query: MATCH (en:EntityNative) WHERE ANY(f IN en.semantic_field WHERE f IN $domains)
```

**Benefits**:
1. **Token efficiency**: Load only domain-relevant EntityNative nodes
2. **Clustering**: Group related content for coherent generation
3. **Cross-locale consistency**: Same domains across all EntityNative instances
4. **RAG optimization**: Pre-filter before vector similarity search

### Alternative: Domain as Node (More Formal)

```
Domain (shared/config, defined)
  key: "hospitality"
  display_name: "Hospitality & Food Service"
  llm_context: "USE: when generating content for restaurants, hotels, cafes..."

EntityNative --[:IN_DOMAIN]--> Domain
```

**Tradeoff**: More formal but adds traversal step. Recommend **property-based** for v0.13, consider **node-based** for v1.0 if domain hierarchy needed.

---

## Question 3: How Does LLM Know Which Related Entities to Load?

### Current Mechanism: Spreading Activation

NovaNet implements spreading activation via `SEMANTIC_LINK` arcs with temperature-weighted traversal.

**Mathematical Model** (from `spreading-activation.yaml`):
```
A_j(t) = δ × A_j(t-1) + Σᵢ [w_ij × A_i(t-1) × decay(t) × semantic_boost(type)]

Where:
- A_j(t) = activation level of node j at time t
- δ = retention factor (0.5)
- w_ij = edge weight (temperature)
- decay(t) = decay function over hops
- semantic_boost = task-specific multiplier
```

**Parameters**:
- `decay_factor`: 0.01 (rapid decay over hops)
- `retention_factor`: 0.5 (50% self-retention)
- `propagation_steps`: 2 (max 2 hops)
- `activation_threshold`: 0.3 (minimum to include)

### Link Type Weights

```yaml
semantic_link_defaults:
  is_action_on: 0.95    # "Create QR Code" IS_ACTION_ON "QR Code"
  type_of: 0.90         # "WiFi QR Code" TYPE_OF "QR Code"
  variant_of: 0.85      # "Dynamic QR" VARIANT_OF "QR Code"
  requires: 0.80        # "QR Scanner" REQUIRES "Camera"
  enables: 0.75         # "QR Code" ENABLES "Contactless Menu"
  used_with: 0.70       # "QR Code" USED_WITH "Restaurant Menu"
  related: 0.60         # Weak association
  compared_to: 0.40     # Comparison (lower relevance)
```

### Task-Specific Modifiers

```yaml
cta_modifier:
  is_action_on: 1.2     # Boost action relationships for CTAs
  enables: 1.3          # "This enables that" is strong CTA material

faq_modifier:
  requires: 1.4         # Requirements are FAQ-worthy
  compared_to: 1.3      # Comparisons answer "vs" questions
```

### How It Works in Practice

**Scenario**: Generate content for Block:qr-restaurant-hero@fr-FR

```cypher
-- Step 1: Load seed entity
MATCH (b:Block {key: "qr-restaurant-hero"})-[:USES_ENTITY]->(e:Entity)
WITH e, 1.0 AS activation

-- Step 2: Spreading activation (hop 1)
MATCH (e)-[r:SEMANTIC_LINK]->(related:Entity)
WHERE r.temperature >= 0.3
WITH related, (1.0 * r.temperature * 0.99) AS activation  -- decay applied

-- Step 3: Spreading activation (hop 2)
MATCH (related)-[r2:SEMANTIC_LINK]->(hop2:Entity)
WHERE r2.temperature >= 0.3 AND (activation * r2.temperature * 0.99) >= 0.3

-- Step 4: Load locale content
MATCH (activated:Entity)-[:HAS_NATIVE]->(en:EntityNative {locale_key: "fr-FR"})
WHERE activated IN [e] + collect(related) + collect(hop2)
RETURN en
```

### Assessment

**Strengths**:
- Temperature threshold prevents irrelevant traversal
- Task modifiers optimize for content type (CTA, FAQ, HERO)
- Mathematical model is well-documented
- 2-hop limit prevents context explosion

**Gaps**:
- No **token budget constraint** (could exceed context window)
- No **priority ordering** when multiple paths exist
- No **negative weights** (can't say "avoid this path")

### Recommendation: Add Token Budget

```yaml
# Add to spreading-activation.yaml
token_constraints:
  max_tokens_per_hop: 2000
  max_total_tokens: 5000
  estimation_method: "property_count * 50"  # Rough heuristic
  overflow_strategy: "truncate_by_activation"  # Keep highest activation
```

---

## Question 4: Should There Be Explicit traversal_hints?

### Current State

Traversal guidance is implicit via:
1. `temperature_threshold` on arcs (0.0-1.0)
2. `llm_context.RELATES` listing key relationships
3. Task modifiers in spreading activation config

### Gap: No Node-Level Traversal Priority

When an LLM lands on an Entity node, it sees `llm_context.RELATES` but all relations appear equal. In practice:

```
Entity:qr-code RELATES:
  - EntityNative (localized content)     <- ALWAYS load this
  - Page (referencer)                    <- Only if generating page
  - Block (referencer)                   <- Only if generating block
  - SEOKeyword (targeting)               <- Only if SEO task
  - SEMANTIC_LINK entities               <- Conditional on temperature
```

### Recommendation: Add context_priority to Nodes

```yaml
# Add to entity.yaml
properties:
  context_priority:
    type: object
    required: false
    description: "Task-specific traversal priorities"
    schema:
      type: object
      properties:
        default:
          type: object
          description: "Default priorities when no task specified"
        tasks:
          type: object
          additionalProperties:
            type: object
    example:
      default:
        HAS_NATIVE: 1.0      # Always load locale content
        BELONGS_TO: 0.8      # Category is useful
        USES_ENTITY: 0.3     # Only if needed
      tasks:
        seo_optimization:
          TARGETS: 1.0       # SEO keywords critical
          HAS_NATIVE: 0.9    # Need locale content
          SEMANTIC_LINK: 0.7 # Related entities for internal linking
        content_generation:
          HAS_NATIVE: 1.0    # Locale content is primary
          SEMANTIC_LINK: 0.9 # Related entities for context
          BELONGS_TO: 0.5    # Category for tone
```

### Alternative: Arc-Level Context Priority

Instead of on nodes, add to arc definitions:

```yaml
# In has-native.yaml
arc:
  name: HAS_NATIVE
  context_priority:
    default: 1.0           # Always high priority
    tasks:
      seo_optimization: 0.9
      content_generation: 1.0
      analytics: 0.3       # Less relevant for analytics
```

**Recommendation**: Start with **arc-level** (simpler, already have arc YAML structure), evolve to node-level if needed.

---

## Question 5: Comparison to OWL, SKOS, Schema.org

### Feature Comparison Matrix

| Feature | OWL | SKOS | Schema.org | NovaNet | Notes |
|---------|-----|------|------------|---------|-------|
| **Formal reasoning** | Full DL reasoning | None | Limited | None | OWL can infer relationships |
| **Property inheritance** | Yes (rdfs:subClassOf) | No | Yes | No | NovaNet uses explicit copying |
| **Symmetric relations** | owl:SymmetricProperty | skos:related | No | No | NovaNet uses explicit inverses |
| **Transitive relations** | owl:TransitiveProperty | skos:broader/narrower | No | Partial | SUBTOPIC_OF is transitive |
| **Multilingual labels** | rdfs:label @lang | skos:prefLabel @lang | Multiple properties | *Native nodes | NovaNet: separate nodes per locale |
| **Disambiguation** | owl:disjointWith | No | No | llm_context.NOT | NovaNet: prose-based |
| **Hierarchies** | rdfs:subClassOf | skos:broader | parent property | SUBTOPIC_OF | All similar |
| **Alternative names** | No standard | skos:altLabel | alternateName | TRIGGERS array | NovaNet: keyword-based |
| **Definitions** | rdfs:comment | skos:definition | description | entity_summary | All have definitions |
| **Usage context** | No standard | skos:scopeNote | No | llm_context.USE | NovaNet unique |
| **Token optimization** | No | No | No | temperature, spreading | NovaNet unique |

### What NovaNet Has That Others Don't

1. **LLM-Optimized Context Loading**
   - `llm_context` pattern with USE/TRIGGERS/NOT/RELATES
   - Spreading activation with temperature thresholds
   - Task-specific modifiers (CTA, FAQ, HERO)
   - Token-aware traversal (partially)

2. **Native Generation Model**
   - Entity (invariant) + EntityNative (locale-specific) separation
   - Explicit curation_status tracking
   - Cultural notes per locale

3. **Self-Describing Meta-Graph**
   - :Schema:Class nodes describe the schema itself
   - LLM can query "what node types exist?" and get llm_context for each

### What NovaNet Is Missing from OWL

1. **Property Chains**
   - OWL: `hasBrother . hasChild -> hasNephew`
   - NovaNet: Must define explicit NEPHEW_OF arc

2. **Inference Rules**
   - OWL: "If Entity.is_pillar AND EntityNative.status='published', then Entity is ready"
   - NovaNet: Must query explicitly

3. **Cardinality Constraints**
   - OWL: "Entity must have exactly 1 EntityNative per supported locale"
   - NovaNet: Documented in YAML, not enforced by reasoner

4. **Disjoint Classes**
   - OWL: "Entity and EntityNative are disjoint (can't be both)"
   - NovaNet: Implicit via separate YAML definitions

### What NovaNet Is Missing from SKOS

1. **Hierarchical Relations Vocabulary**
   - SKOS: `skos:broader`, `skos:narrower`, `skos:related`
   - NovaNet: `SUBTOPIC_OF`, `SEMANTIC_LINK` (less standardized)

2. **Collection Concept**
   - SKOS: `skos:Collection` for arbitrary groupings
   - NovaNet: No equivalent (could use EntityCategory)

3. **Mapping Relations**
   - SKOS: `skos:exactMatch`, `skos:closeMatch` for inter-ontology linking
   - NovaNet: No formal cross-system mapping

### What NovaNet Is Missing from Schema.org

1. **Standardized Property Names**
   - Schema.org: `name`, `description`, `url` are universal
   - NovaNet: `display_name`, `description`, `slug` (custom)

2. **Action Vocabulary**
   - Schema.org: `SearchAction`, `BuyAction`, `ViewAction`
   - NovaNet: Actions as Entity with category=ACTION (less formal)

3. **Structured Data Output**
   - Schema.org: Direct JSON-LD generation
   - NovaNet: Would need transformer to Schema.org

### Recommendations

#### Short-Term (v0.13.x)

1. **Add semantic_field to EntityNative** for domain clustering
2. **Add context_priority to arcs** for task-specific traversal guidance
3. **Add token estimation** to spreading activation config
4. **Standardize on SKOS-like hierarchy terms** where possible

#### Medium-Term (v0.14.x)

1. **Add inference rules in YAML** (declarative, not OWL DL):
   ```yaml
   inference_rules:
     - name: "pillar_requires_native"
       if: "Entity.is_pillar = true"
       then: "EntityNative must exist for all priority locales"
       enforcement: "validation_warning"
   ```

2. **Add Schema.org mapping layer**:
   ```yaml
   schema_org_mapping:
     Entity: "Thing"
     EntityNative: "Thing @language"
     Project: "Organization"
     Page: "WebPage"
   ```

3. **Add SKOS-compatible export**:
   ```yaml
   skos_export:
     Entity: "skos:Concept"
     SUBTOPIC_OF: "skos:broader"
     SEMANTIC_LINK[type_of]: "skos:broader"
     SEMANTIC_LINK[related]: "skos:related"
   ```

#### Long-Term (v1.0+)

1. **Optional OWL layer** for systems that need formal reasoning
2. **Property chain support** for complex traversal patterns
3. **Cardinality enforcement** in Neo4j constraints
4. **Cross-system concept mapping** (Wikidata, DBpedia)

---

## Summary: Ontology Pattern Recommendations

### Priority Matrix

| Recommendation | Effort | Impact | Priority |
|----------------|--------|--------|----------|
| Add semantic_field to EntityNative | Low | High | P0 |
| Add context_priority to arcs | Medium | High | P0 |
| Add token estimation to spreading activation | Low | Medium | P1 |
| Enhance llm_context with priority weights | Medium | Medium | P1 |
| Add inference rules (declarative) | Medium | High | P2 |
| Schema.org mapping layer | Medium | Medium | P2 |
| SKOS-compatible export | Low | Low | P3 |
| OWL layer for formal reasoning | High | Low | P3 |

### Key Insight

NovaNet occupies a **unique position** between lightweight taxonomies (SKOS) and formal ontologies (OWL):

```
SKOS -------- NovaNet -------- OWL
Lightweight   LLM-Optimized    Formal
No reasoning  Spreading act.   Full DL
Fast          Token-aware      Slow
```

This is **intentional and correct** for an LLM-first knowledge graph. The recommendations above enhance NovaNet's LLM optimization without adding OWL's complexity.

---

## Appendix: Implementation Checklist

### Phase 1: Core Enhancements (v0.13.x)

- [ ] Add `semantic_field: string[]` to EntityNative YAML
- [ ] Add `context_priority` section to all arc YAMLs (start with HAS_NATIVE, SEMANTIC_LINK)
- [ ] Update spreading-activation.yaml with token constraints
- [ ] Add domain index: `CREATE INDEX entity_native_semantic_field FOR (en:EntityNative) ON (en.semantic_field)`
- [ ] Update llm_context examples with priority indicators

### Phase 2: Inference Layer (v0.14.x)

- [ ] Define inference rule YAML schema
- [ ] Implement validation runner for inference rules
- [ ] Add Schema.org mapping YAML structure
- [ ] Create SKOS export generator in Rust

### Phase 3: Interoperability (v1.0+)

- [ ] OWL export for systems requiring formal reasoning
- [ ] Wikidata linking via `owl:sameAs` equivalent
- [ ] Property chain support in spreading activation

---

## References

- **GraphRAG Patterns**: Microsoft Research on knowledge graph + LLM integration
- **Context Graphs**: Operational knowledge graphs for AI systems
- **OWL 2 Primer**: W3C Web Ontology Language
- **SKOS Reference**: W3C Simple Knowledge Organization System
- **Schema.org**: Structured data vocabulary for the web
- **NovaNet ADRs**: ADR-024 (Trait), ADR-029 (*Native), ADR-028 (Page-Entity)
