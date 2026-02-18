# Locale Knowledge Ontology v10 Design

**Date**: 2026-02-04
**Status**: Approved
**Participants**: Thibaut, Claude

---

## Executive Summary

Refactor the 14 locale knowledge nodes into a granular, retrieval-optimized structure of ~27 nodes per locale. This enables contextual loading based on content domain, audience segment, and register.

**Key Changes**:
- 14 nodes (v9) → 27 nodes (v10) per locale
- Fat nodes for technical/deterministic data
- Granular nodes for semantic/contextual data
- Domain-tagged retrieval for optimal context windows

---

## Problem Statement

### Current State (v9)

```
global/knowledge/ (14 nodes, inconsistent naming)
├── locale-voice           ← "Locale" prefix
├── locale-culture         ← "Locale" prefix
├── locale-culture-refs    ← "Locale" prefix
├── locale-identity        ← "Locale" prefix
├── locale-lexicon         ← "Locale" prefix
├── locale-market          ← "Locale" prefix
├── locale-rules-*         ← 3 nodes with prefix
├── constraint             ← NO prefix
├── expression             ← NO prefix
├── metaphor               ← NO prefix
├── pattern                ← NO prefix
└── reference              ← NO prefix
```

**Issues**:
1. Inconsistent naming ("Locale" prefix on some, not others)
2. No contextual retrieval — always load everything
3. ~47KB loaded per generation even for simple blocks
4. Wrong expressions/terms for specific contexts (B2B vs B2C, pricing vs support)

### Research Findings

From Neo4j "Going Meta" series and GraphRAG papers (2024-2025):

> "Knowledge graphs without well-defined schemas can easily lead to unmanageable results."

**Trade-offs Table**:

| Aspect | Fat Nodes | Fine-Grained |
|--------|-----------|--------------|
| Storage | Lower cost | Higher (node explosion) |
| Query speed | Fast broad retrieval | Precise but slower |
| Reasoning | Loses relations | Multi-hop possible |
| Accuracy | Good summaries | 15-20% extraction errors |

**Recommendation**: Hybrid approach — fat nodes for deterministic data, granular for semantic.

---

## Design Decision

### Principle: Contextual Retrieval

```
WRONG:  Load ALL locale knowledge (47KB) for every Block
RIGHT:  Load RELEVANT knowledge based on Block context (~15KB)

Block "Hero Pricing" for B2B audience needs:
✅ TermSet:pricing (20 terms)     NOT all 200 terms
✅ ExpressionSet:formal (30 expr) NOT casual expressions
✅ AudienceSet:b2b               NOT B2C insights
```

### Node Structure (v10)

#### TIER 1: Technical (Fat Nodes) — 3 nodes

Always loaded together, deterministic, no contextual variation.

| Node | Size | Content |
|------|------|---------|
| **Formatting** | ~3KB | dates, numbers, currency, phone, address, units |
| **Slugification** | ~2KB | transliteration, stop words, URL rules |
| **Adaptation** | ~3KB | length prefs, structure, SEO rules |

#### TIER 2: Style (Fat Node) — 1 node

Cohesive communication style, loaded as unit.

| Node | Size | Content |
|------|------|---------|
| **Style** | ~4KB | tone, formality, politeness, directness, humor, identity traits |

#### TIER 3: Semantic (Granular Nodes) — 23 nodes

Contextual retrieval based on domain, register, type, severity, segment.

**Terms (6 nodes by domain)**:
| Node | Key | Content |
|------|-----|---------|
| TermSet | `pricing` | tarification, prix, abonnement, facturation |
| TermSet | `features` | fonctionnalité, option, paramètre |
| TermSet | `technical` | API, webhook, intégration, SDK |
| TermSet | `marketing` | conversion, engagement, acquisition |
| TermSet | `support` | aide, assistance, ticket, FAQ |
| TermSet | `general` | common terms across all domains |

**Expressions (3 nodes by register)**:
| Node | Key | Content |
|------|-----|---------|
| ExpressionSet | `formal` | B2B appropriate idioms |
| ExpressionSet | `neutral` | Universal expressions |
| ExpressionSet | `casual` | B2C friendly expressions |

**Patterns (4 nodes by usage)**:
| Node | Key | Content |
|------|-----|---------|
| PatternSet | `cta` | Call-to-action templates |
| PatternSet | `headlines` | Title/headline structures |
| PatternSet | `body` | Body copy patterns |
| PatternSet | `social` | Social proof templates |

**Culture (4 nodes by type)**:
| Node | Key | Content |
|------|-----|---------|
| CultureSet | `values` | Core cultural values |
| CultureSet | `references` | Landmarks, events, shared knowledge |
| CultureSet | `celebrities` | Safe-to-reference personalities |
| CultureSet | `calendar` | Holidays, vacation periods, business calendar |

**Taboos (3 nodes by severity)**:
| Node | Key | Content |
|------|-----|---------|
| TabooSet | `avoid` | Topics to never mention |
| TabooSet | `careful` | Sensitive topics requiring care |
| TabooSet | `legal` | Legal/compliance constraints |

**Audience (3 nodes by segment)**:
| Node | Key | Content |
|------|-----|---------|
| AudienceSet | `b2b` | B2B purchasing behavior, trust factors |
| AudienceSet | `b2c` | B2C preferences, channels |
| AudienceSet | `general` | Market demographics, digital behavior |

---

## Graph Structure

### Node Hierarchy

```
Locale (fr-FR)
│
├──HAS_FORMATTING──► Formatting
├──HAS_SLUGIFICATION──► Slugification
├──HAS_ADAPTATION──► Adaptation
├──HAS_STYLE──► Style
│
├──HAS_TERMS──► TermSet {domain: "pricing"}
├──HAS_TERMS──► TermSet {domain: "features"}
├──HAS_TERMS──► TermSet {domain: "technical"}
├──HAS_TERMS──► TermSet {domain: "marketing"}
├──HAS_TERMS──► TermSet {domain: "support"}
├──HAS_TERMS──► TermSet {domain: "general"}
│
├──HAS_EXPRESSIONS──► ExpressionSet {register: "formal"}
├──HAS_EXPRESSIONS──► ExpressionSet {register: "neutral"}
├──HAS_EXPRESSIONS──► ExpressionSet {register: "casual"}
│
├──HAS_PATTERNS──► PatternSet {usage: "cta"}
├──HAS_PATTERNS──► PatternSet {usage: "headlines"}
├──HAS_PATTERNS──► PatternSet {usage: "body"}
├──HAS_PATTERNS──► PatternSet {usage: "social"}
│
├──HAS_CULTURE──► CultureSet {type: "values"}
├──HAS_CULTURE──► CultureSet {type: "references"}
├──HAS_CULTURE──► CultureSet {type: "celebrities"}
├──HAS_CULTURE──► CultureSet {type: "calendar"}
│
├──HAS_TABOOS──► TabooSet {severity: "avoid"}
├──HAS_TABOOS──► TabooSet {severity: "careful"}
├──HAS_TABOOS──► TabooSet {severity: "legal"}
│
├──HAS_AUDIENCE──► AudienceSet {segment: "b2b"}
├──HAS_AUDIENCE──► AudienceSet {segment: "b2c"}
└──HAS_AUDIENCE──► AudienceSet {segment: "general"}
```

### Arc Types (8 new arcs)

| Arc | Source | Target | Properties |
|-----|--------|--------|------------|
| HAS_FORMATTING | Locale | Formatting | — |
| HAS_SLUGIFICATION | Locale | Slugification | — |
| HAS_ADAPTATION | Locale | Adaptation | — |
| HAS_STYLE | Locale | Style | — |
| HAS_TERMS | Locale | TermSet | `domain: string` |
| HAS_EXPRESSIONS | Locale | ExpressionSet | `register: string` |
| HAS_PATTERNS | Locale | PatternSet | `usage: string` |
| HAS_CULTURE | Locale | CultureSet | `type: string` |
| HAS_TABOOS | Locale | TabooSet | `severity: string` |
| HAS_AUDIENCE | Locale | AudienceSet | `segment: string` |

---

## Retrieval Patterns

### Example: Block "Hero Pricing" for B2B Page

**Input Context**:
- Block type: `hero`
- Block domain: `pricing` (from BlockType or explicit tag)
- Page audience: `b2b`
- Locale: `fr-FR`

**Retrieval Query**:

```cypher
// Always load technical + style
MATCH (l:Locale {key: $locale})
MATCH (l)-[:HAS_FORMATTING]->(fmt:Formatting)
MATCH (l)-[:HAS_SLUGIFICATION]->(slug:Slugification)
MATCH (l)-[:HAS_ADAPTATION]->(adapt:Adaptation)
MATCH (l)-[:HAS_STYLE]->(style:Style)

// Load domain-specific terms
MATCH (l)-[:HAS_TERMS]->(terms:TermSet)
WHERE terms.domain IN [$domain, 'general']

// Load register-appropriate expressions
MATCH (l)-[:HAS_EXPRESSIONS]->(expr:ExpressionSet)
WHERE expr.register IN [$register, 'neutral']

// Load relevant patterns
MATCH (l)-[:HAS_PATTERNS]->(patterns:PatternSet)
WHERE patterns.usage IN ['cta', 'headlines']

// Load core culture + taboos
MATCH (l)-[:HAS_CULTURE]->(culture:CultureSet)
WHERE culture.type = 'values'
MATCH (l)-[:HAS_TABOOS]->(taboos:TabooSet)
WHERE taboos.severity = 'avoid'

// Load audience segment
MATCH (l)-[:HAS_AUDIENCE]->(audience:AudienceSet)
WHERE audience.segment = $segment

RETURN fmt, slug, adapt, style,
       collect(DISTINCT terms) as terms,
       collect(DISTINCT expr) as expressions,
       collect(DISTINCT patterns) as patterns,
       collect(DISTINCT culture) as culture,
       collect(DISTINCT taboos) as taboos,
       collect(DISTINCT audience) as audience
```

**Result**: ~15KB loaded vs ~47KB (68% reduction)

---

## Node Counts

| Category | v9 Nodes | v10 Nodes | Notes |
|----------|----------|-----------|-------|
| Technical | 3 | 3 | Formatting, Slugification, Adaptation |
| Style | 2 | 1 | voice + identity merged |
| Terms | 1 | 6 | Split by domain |
| Expressions | 1 | 3 | Split by register |
| Patterns | 1 | 4 | Split by usage |
| Culture | 3 | 4 | Split by type |
| Taboos | 1 | 3 | Split by severity |
| Audience | 1 | 3 | Split by segment |
| **TOTAL** | **14** | **27** | Per locale |

**Scale**: 27 nodes × 200 locales = 5,400 nodes (Neo4j handles easily)

---

## Migration Plan

### Phase 1: Create New Node Types

1. Create YAML definitions for 10 new node kinds:
   - `formatting.yaml`, `slugification.yaml`, `adaptation.yaml`
   - `style.yaml`
   - `term-set.yaml`, `expression-set.yaml`, `pattern-set.yaml`
   - `culture-set.yaml`, `taboo-set.yaml`, `audience-set.yaml`

2. Create arc kind YAMLs:
   - `has-formatting.yaml`, `has-slugification.yaml`, etc.

### Phase 2: Update Generators

1. Update Rust generators to emit new node/arc structure
2. Add `domain`, `register`, `type`, `severity`, `segment` properties to arcs

### Phase 3: Migrate Data

1. Transform existing 14 nodes → 27 nodes per locale
2. Split lexicon → 6 TermSets by domain
3. Split expressions → 3 ExpressionSets by register
4. Merge voice + identity → Style

### Phase 4: Delete Old Nodes

1. Remove deprecated node kinds:
   - `locale-voice`, `locale-identity`, `locale-lexicon`
   - `locale-culture`, `locale-culture-references`, `locale-market`
   - `locale-rules-adaptation`, `locale-rules-formatting`, `locale-rules-slug`
   - `constraint`, `expression`, `metaphor`, `pattern`, `reference`

2. Remove deprecated arcs:
   - `HAS_VOICE`, `HAS_IDENTITY`, `HAS_LEXICON`, etc.

### Phase 5: Update Retrieval

1. Update Context Assembly Engine to use contextual retrieval
2. Add domain/register/segment resolution from Block context

---

## Related Changes

### Concept → Thing Migration (v9.9)

**Status**: Partially complete — Thing added but Concept not renamed.

**Remaining work**:
- Rename `project/semantic/concept.yaml` → `project/semantic/thing.yaml`
- Rename `project/semantic/concept-l10n.yaml` → `project/semantic/thing-l10n.yaml`
- Update arcs: `USES_CONCEPT` → `MENTIONS`
- Thing (project) can `SPECIALIZES` Thing (shared)

---

## Success Criteria

1. **Contextual retrieval works**: Block generation loads only relevant knowledge
2. **Token reduction**: ~68% reduction in context window for typical blocks
3. **Naming consistency**: No more "Locale" prefix redundancy
4. **Neo4j performance**: Query time < 50ms for contextual retrieval
5. **Zero data loss**: All v9 content preserved in v10 structure

---

## References

- [Neo4j Going Meta Series](https://neo4j.com/video/going-meta-a-series-on-graphs-semantics-and-knowledge/)
- [KET-RAG Paper 2025](https://arxiv.org/pdf/2502.09304) — Hybrid chunking approach
- [GraphRAG Trade-offs](https://www.falkordb.com/blog/vectorrag-vs-graphrag-technical-challenges-enterprise-ai-march25/)
- `docs/plans/2026-02-03-v10-brainstorm-decisions.md` — Original brainstorm
