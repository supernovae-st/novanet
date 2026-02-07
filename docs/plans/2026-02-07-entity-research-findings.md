# Entity Schema Research Findings

**Date**: 2026-02-07
**Status**: Complete
**Project**: NovaNet Entity/EntityL10n Optimization

---

## Executive Summary

Research conducted across Perplexity, Context7 (Neo4j docs), and Claude documentation to optimize Entity/EntityL10n schema for QR Code AI knowledge graph.

**Key Insight**: NovaNet's **Generation NOT Translation** philosophy aligns perfectly with modern knowledge graph best practices. The Entity (invariant) → EntityL10n (localized) pattern is optimal.

---

## Research Sources

| Source | Focus | Key Finding |
|--------|-------|-------------|
| Perplexity #1 | Ontology design | Top-down ontology-first, modular constraints |
| Perplexity #2 | Entity descriptions | Schema.org alignment, JSON-LD, NER patterns |
| Perplexity #3 | Spreading activation | Semantic arc types, relationship temperatures |
| Context7 Neo4j | Graph patterns | Relationship properties, MERGE patterns |
| Claude Docs | LLM context | Selective loading, structured outputs, memory |

---

## Key Patterns Discovered

### 1. Ontology-First Design (Validated)

NovaNet already follows this:

```
┌──────────────────────────────────────────────────────────────────┐
│  TOP-DOWN ONTOLOGY APPROACH                                      │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  1. Define classes/properties/rules (taxonomy.yaml)              │
│  2. Load as schema into graph (NodeKind, ArcKind)               │
│  3. Populate with domain data (Entity instances)                 │
│  4. LLM reads schema + data for generation                      │
│                                                                  │
│  NovaNet Status: ✅ Already implemented                          │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

### 2. Selective Context Loading (Apply to Entities)

**Pattern**: Load only relevant knowledge atoms, not entire definitions.

```
Entity → [:USES_TERM] → Term (atom)
       → [:USES_EXPRESSION] → Expression (atom)

LLM Context: Load 50 relevant atoms, not 20K blob
```

**Recommendation for Entity**:
- Keep Entity lightweight (~2KB invariant properties)
- Link to Knowledge Atoms via semantic arcs
- EntityL10n references locale-specific atoms

### 3. Semantic Arc Types with Temperature

**Pattern**: Relationship types encode semantic distance/activation.

| Arc Type | Activation | Use Case |
|----------|------------|----------|
| `is_a` | 0.9-1.0 | Direct type hierarchy |
| `type_of` | 0.8-0.9 | Subtype relationship |
| `requires` | 0.7-0.8 | Dependency |
| `enables` | 0.6-0.7 | Enablement |
| `used_for` | 0.5-0.6 | Purpose |
| `related_to` | 0.3-0.5 | Loose association |
| `contrasts_with` | 0.1-0.3 | Comparison |

**Recommendation**: Use `temperature` property on SEMANTIC_LINK arcs for spreading activation.

### 4. Structured Output Schema

**Pattern**: Define JSON schema for guaranteed-valid EntityL10n generation.

```json
{
  "type": "object",
  "properties": {
    "locale": {"type": "string"},
    "entity_key": {"type": "string"},
    "display_name": {"type": "string"},
    "description": {"type": "string"},
    "definition": {"type": "string"},
    "benefits": {"type": "array", "items": {"type": "string"}},
    "cultural_notes": {"type": "string"},
    "confidence": {"type": "number", "minimum": 0, "maximum": 1}
  },
  "required": ["locale", "entity_key", "display_name", "description"],
  "additionalProperties": false
}
```

**Benefit**: No parsing errors, guaranteed data types.

### 5. Entity Property Philosophy

**Best Practice from Research**:

| Property | Purpose | Reader |
|----------|---------|--------|
| `description` | WHAT it IS (comprehension) | Human + LLM |
| `llm_context` | HOW to USE it (generation) | LLM only |
| `definition` | Formal definition | Human + LLM |
| `purpose` | WHY it exists | Human + LLM |

**NovaNet Status**: ✅ Already implemented in v10.7 Entity schema.

### 6. Arc Properties for Rich Semantics

**Neo4j Pattern**: Relationships can have properties.

```cypher
(a:Entity)-[:SEMANTIC_LINK {
  link_type: "includes",
  temperature: 0.8,
  direction: "parent_to_child",
  context: "QR Code includes Custom QR Code as a style variant"
}]->(b:Entity)
```

**Recommendation**: Add `context` property to SEMANTIC_LINK for LLM comprehension.

---

## Schema Improvements

### Entity (Invariant) - v10.7 Validated

Current schema is optimal. Minor additions:

```yaml
# NEW: Add keywords for SEO + search
keywords:
  type: string[]
  required: false
  description: "Search/SEO keywords (invariant, EN)"
  example: ["qr code", "barcode", "2d code"]

# NEW: Add wikidata_id for universal linking
wikidata_id:
  type: string
  required: false
  description: "Wikidata entity ID for universal disambiguation"
  example: "Q1052379"
```

### EntityL10n (Localized) - Improvements

```yaml
# EXISTING (validated):
display_name: string (required)
description: string (required)
llm_context: string (required)
definition: string (optional)
purpose: string (optional)
benefits: string[] (optional)
cultural_notes: string (optional)
version: int (required)

# NEW RECOMMENDATIONS:
search_keywords:
  type: string[]
  required: false
  description: "Locale-specific search keywords"

usage_examples:
  type: string[]
  required: false
  description: "Locale-native usage examples"

audience_segment:
  type: string
  required: false
  description: "Target audience for this locale variant"
  enum: ["professional", "consumer", "developer", "enterprise"]
```

### SEMANTIC_LINK Arc - Improvements

```yaml
# EXISTING (validated):
temperature: float [0.0-1.0]
link_type: enum
direction: enum

# NEW RECOMMENDATIONS:
context:
  type: string
  required: false
  description: "LLM-readable explanation of this link"
  example: "QR Code includes Custom QR Code as a visual style variant"

bidirectional:
  type: boolean
  default: false
  description: "Whether link activates in both directions"
```

---

## QR Code AI Entity Type Mapping

Based on research, here's the optimal type classification:

| Entity | Type | Rationale |
|--------|------|-----------|
| QR Code | THING (pillar) | Core product, concrete |
| Smart Link | THING (pillar) | Core product, concrete |
| Short Link | THING | Infrastructure object |
| Custom QR Code | THING | Visual variant |
| QR Code Image | THING | Visual variant |
| QR Code Art | THING | Visual variant (AI-fused) |
| Dynamic QR Code | CONCEPT | Educational abstraction |
| Static QR Code | CONCEPT | Educational abstraction |
| Analytics | FEATURE | Product capability |
| Contextual Routing | FEATURE | Product capability |
| Custom Domain Name | FEATURE | Product capability |
| UTM Builder | FEATURE | Product capability |
| Custom Link Preview | FEATURE | Product capability |
| Bulk Creation | FEATURE | Product capability |
| Create QR Code | ACTION | User action |
| Scan QR Code | ACTION | User action |
| Customize QR Code | ACTION | User action |
| Create Smart Link | ACTION | User action |
| Track Performance | ACTION | User action |
| QR Code Generator | TOOL | Creation tool |
| QR Code Scanner | TOOL | Reading tool |
| Landing Page Builder | TOOL | Creation tool |
| Barcode Generator | TOOL | Related tool |
| QR Code URL | CONTENT_TYPE | Dynamic content |
| QR Code WiFi | CONTENT_TYPE | Static content |
| QR Code vCard | CONTENT_TYPE | Static content |
| QR Code PDF | CONTENT_TYPE | Dynamic content |

---

## Semantic Link Temperature Guidelines

Based on spreading activation research:

```
┌──────────────────────────────────────────────────────────────────┐
│  TEMPERATURE CALIBRATION                                         │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  0.9-1.0  │ STRONG   │ is_a, type_of (hierarchy)                │
│  0.7-0.9  │ MEDIUM+  │ includes, requires, enables              │
│  0.5-0.7  │ MEDIUM   │ used_for, is_action_on                   │
│  0.3-0.5  │ MEDIUM-  │ related_to                               │
│  0.1-0.3  │ WEAK     │ contrasts_with                           │
│                                                                  │
│  Example:                                                        │
│  QR Code --[includes, 0.85]--> Custom QR Code                   │
│  QR Code --[contrasts_with, 0.2]--> Barcode                     │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

---

## Implementation Checklist

- [x] Entity schema v10.7 documented
- [x] EntityL10n schema validated
- [x] Type enum expanded (10 types)
- [x] SEMANTIC_LINK link_types expanded (8 types)
- [ ] Add `keywords` property to Entity
- [ ] Add `wikidata_id` property to Entity
- [ ] Add `context` property to SEMANTIC_LINK arc
- [ ] Add `search_keywords` to EntityL10n
- [ ] Create QR Code AI entities in Neo4j
- [ ] Define all SEMANTIC_LINK arcs with temperatures

---

## References

- Perplexity: Knowledge graph ontology best practices
- Perplexity: Entity description schema for AI content
- Context7: Neo4j documentation patterns
- Claude Docs: Context loading, structured outputs
- Schema.org: Entity type patterns
- W3C: SKOS for taxonomies

