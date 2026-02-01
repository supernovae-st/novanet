# Organizing Principles as Neo4j Nodes

**Date**: 2026-01-31
**Status**: Approved
**Author**: Thibaut + Claude
**Reviewed**: Architect Review 2026-01-31

## Summary

Add Scope and Subcategory as first-class Neo4j nodes (organizing principles) to enable:
- Query-based schema visualization grouping
- LLM context at every taxonomy level
- Self-describing knowledge graph

## Background

### Current State

The NovaNet schema organizes 35 node types into 3 scopes and 9 subcategories. Currently:

- **Source of truth**: YAML folder structure (`models/nodes/{scope}/{subcategory}/{node}.yaml`)
- **TypeScript exports**: `NODE_SCOPES`, `NODE_SUBCATEGORIES`, `SCOPE_HIERARCHY`
- **Studio**: Imports TypeScript, derives grouping for visualization
- **Neo4j**: Only domain nodes, no organizing principles

### Problem

1. **No LLM context** on scope/subcategory levels - can't guide generation with taxonomy context
2. **Not queryable** - can't `MATCH (s:Scope)-[:HAS_SUBCATEGORY]->(sub)` in Cypher
3. **Not self-describing** - the graph doesn't explain its own structure
4. **Visualization coupling** - Studio must import TypeScript instead of querying Neo4j

### Research Findings

Neo4j best practices (from official documentation):

> "The model uses the SAME node-and-relationship structure as the rest of the knowledge graph to describe the organizing principles"

Organizing principles (taxonomies, ontologies) SHOULD be nodes, not just metadata.

## Design

### New Node Types

#### Scope (3 instances)

| key | display_name | emoji | color | llm_context |
|-----|--------------|-------|-------|-------------|
| `global` | Global | `🌍` | `#2aa198` (cyan) | Shared across ALL projects. Locale-specific knowledge that applies universally: cultural norms, linguistic patterns, voice guidelines, idiomatic expressions. These nodes are READ-ONLY at project level. Changes here affect all projects using that locale. |
| `project` | Project | `📦` | `#6c71c4` (violet) | Business-specific nodes for a single project. Contains brand identity, page structure, semantic concepts, generation prompts, and localized outputs. These nodes define WHAT content to generate and HOW to structure it for this specific product/service. |
| `shared` | Shared | `🎯` | `#cb4b16` (orange) | Cross-project resources that can be linked to multiple projects. SEO keywords and GEO seeds with their metrics. These enable competitive intelligence sharing across the portfolio. |

#### Subcategory (9 instances)

| key | display_name | emoji | scope | llm_context |
|-----|--------------|-------|-------|-------------|
| `config` | Configuration | `⚙️` | global | Core configuration nodes. Locale definitions with their properties (language code, region, writing direction). Entry point for all locale-specific knowledge traversal. |
| `knowledge` | Locale Knowledge | `📚` | global | Deep locale-specific knowledge for native content generation. Cultural norms, linguistic patterns, voice guidelines, idiomatic expressions, formatting conventions. This is what makes generated content feel NATIVE rather than translated. |
| `foundation` | Foundation | `🏛️` | project | Core project identity. Brand voice, visual identity, value proposition. These nodes anchor ALL content generation for the project - every generated block must align with foundation. |
| `structure` | Structure | `🏗️` | project | Information architecture. Pages, blocks, and their types. Defines the SKELETON of the website - what pages exist, what blocks compose each page, and the rules for each block type. |
| `semantic` | Semantic Layer | `💡` | project | Meaning and concepts. Invariant ideas (Concept) that get localized per locale (ConceptL10n). The WHAT of content - pricing tiers, features, benefits, use cases. Concepts link via SEMANTIC_LINK for spreading activation during generation. |
| `instruction` | Instructions | `📝` | project | Generation directives. Prompts and rules that guide the LLM during content generation. PagePrompt for page-level guidance, BlockPrompt for block-specific instructions, BlockRules for constraints and validation. |
| `output` | Generated Output | `✨` | project | LLM-generated content. The final localized pages and blocks ready for rendering. These are the RESULTS of the generation pipeline - created by combining foundation, structure, semantic, and instruction nodes with locale knowledge. |
| `seo` | SEO Intelligence | `🔍` | shared | Search engine optimization data. Keywords with their localized forms, search volume metrics, and mining run history. Used to inject relevant keywords into generated content for organic search visibility. |
| `geo` | GEO Intelligence | `📍` | shared | Geographic/local SEO data. Location-based seeds with their localized forms, metrics, and mining history. Used for local business visibility and location-specific content generation. |

#### NodeTypeMeta (35 instances)

One per node type, with:
- `label`: The Neo4j label (e.g., "Concept")
- `display_name`: Human-readable name
- `description`: Short description
- `llm_context`: Generation guidance for this node type
- `required_properties`: Array of required property names
- `optional_properties`: Array of optional property names
- `relation_types_out`: Outgoing relation types
- `relation_types_in`: Incoming relation types
- `yaml_path`: Path to YAML definition for introspection

> **Note**: No full JSON schema stored (architect decision). Metadata only.

### New Relationships

| Relationship | From | To | Description |
|--------------|------|-----|-------------|
| `HAS_SUBCATEGORY` | Scope | Subcategory | Scope contains subcategories |
| `DEFINES_TYPE` | Subcategory | NodeTypeMeta | Subcategory defines node types |

### Complete Hierarchy

```
🌍 GLOBAL (15 types)
├── ⚙️ config (1)
│   └── Locale
└── 📚 knowledge (14)
    ├── LocaleVoice, LocaleCulture, LocaleStyle, LocaleTone
    ├── LocaleHumor, LocaleFormality, LocaleIdioms, LocaleTaboos
    ├── LocaleNumbers, LocaleDates, LocaleCurrency, LocaleUnits
    └── LocaleLexicon, Expression

📦 PROJECT (14 types)
├── 🏛️ foundation (3)
│   └── Project, BrandIdentity, ProjectL10n
├── 🏗️ structure (4)
│   └── Page, Block, PageType, BlockType
├── 💡 semantic (2)
│   └── Concept, ConceptL10n
├── 📝 instruction (3)
│   └── PagePrompt, BlockPrompt, BlockRules
└── ✨ output (2)
    └── PageL10n, BlockL10n

🎯 SHARED (6 types)
├── 🔍 seo (3)
│   └── SEOKeywordL10n, SEOKeywordMetrics, SEOMiningRun
└── 📍 geo (3)
    └── GEOSeedL10n, GEOSeedMetrics, GEOMiningRun
```

## Example Cypher

### Create Organizing Principles

```cypher
// Create Scopes
CREATE (:Scope {
  key: "global",
  display_name: "Global",
  emoji: "🌍",
  color: "#2aa198",
  llm_context: "Shared across ALL projects. Locale-specific knowledge...",
  created_at: datetime()
})

CREATE (:Scope {
  key: "project",
  display_name: "Project",
  emoji: "📦",
  color: "#6c71c4",
  llm_context: "Business-specific nodes for a single project...",
  created_at: datetime()
})

CREATE (:Scope {
  key: "shared",
  display_name: "Shared",
  emoji: "🎯",
  color: "#cb4b16",
  llm_context: "Cross-project resources that can be linked...",
  created_at: datetime()
})

// Create Subcategories and link to Scopes
MATCH (s:Scope {key: "project"})
CREATE (sub:Subcategory {
  key: "semantic",
  display_name: "Semantic Layer",
  emoji: "💡",
  llm_context: "Meaning and concepts. Invariant ideas...",
  created_at: datetime()
})
CREATE (s)-[:HAS_SUBCATEGORY]->(sub)

// Create NodeTypeMeta and link to Subcategories
MATCH (sub:Subcategory {key: "semantic"})
CREATE (t:NodeTypeMeta {
  label: "Concept",
  display_name: "Concept",
  description: "An invariant idea that gets localized per locale",
  llm_context: "Concepts represent the MEANING behind content...",
  created_at: datetime()
})
CREATE (sub)-[:DEFINES_TYPE]->(t)
```

### Query for Studio Visualization

```cypher
// Get complete hierarchy for grouping
MATCH (s:Scope)-[:HAS_SUBCATEGORY]->(sub:Subcategory)-[:DEFINES_TYPE]->(t:NodeTypeMeta)
RETURN
  s.key AS scope,
  s.display_name AS scopeName,
  s.color AS scopeColor,
  s.emoji AS scopeEmoji,
  sub.key AS subcategory,
  sub.display_name AS subcategoryName,
  sub.emoji AS subcategoryEmoji,
  collect({
    label: t.label,
    displayName: t.display_name,
    description: t.description
  }) AS nodeTypes
ORDER BY s.key, sub.key
```

### Query for LLM Context

```cypher
// Get full context for a node type (for generation)
MATCH (t:NodeTypeMeta {label: "Concept"})
MATCH (sub:Subcategory)-[:DEFINES_TYPE]->(t)
MATCH (s:Scope)-[:HAS_SUBCATEGORY]->(sub)
RETURN
  s.llm_context AS scopeContext,
  sub.llm_context AS subcategoryContext,
  t.llm_context AS nodeTypeContext
```

## Sync Workflow

Source of truth remains YAML folder structure:

```
YAML folders (source)
     │
     ▼
pnpm schema:generate
     │
     ├──► TypeScript (NODE_SCOPES, etc.) ──► Studio imports (backward compat)
     │
     └──► Neo4j Seed (Scope, Subcategory, NodeTypeMeta nodes)
              │
              └──► Studio queries for visualization (preferred)
```

## Implementation Steps

1. **YAML**: Add `organizing-principles.yaml` with Scope, Subcategory, NodeTypeMeta definitions
2. **TypeScript**: Generate types for new node types
3. **Seed**: Add seed script to create organizing principle nodes
4. **Validation**: Add sync test to verify YAML ↔ Neo4j consistency
5. **Studio**: Update to query Neo4j for grouping (optional, can keep TypeScript import)

## Migration

No breaking changes. Organizing principles are additive:
- Existing nodes unchanged
- TypeScript exports remain for backward compatibility
- Studio can migrate to Neo4j queries incrementally

## Resolved Questions (Architect Review)

### Q1: Should `NodeTypeMeta` include full JSON schema?

**Answer: NO**

Store metadata only (required_properties, relation_types), not full JSON schema.

**Rationale**:
- Schema already in YAML (duplication risk)
- LLMs don't need JSON Schema syntax; they need natural language (`llm_context`)
- Size concerns with complex nodes

### Q2: Should we add `:OF_TYPE` relationship from instances?

**Answer: NO**

Do NOT add `:OF_TYPE` from instances to `NodeTypeMeta`.

**Rationale**:
- Redundant: Neo4j labels already provide type information
- Performance: Label queries are faster than relationship traversal
- Cardinality: 50,000+ extra relationships at scale
- Maintenance: Every CREATE must maintain the relationship

**Pattern**: Use labels + separate metadata query
```cypher
// 1. Query instances by label (FAST)
MATCH (c:Concept {key: $key}) RETURN c

// 2. Get metadata separately (CACHED)
MATCH (meta:NodeTypeMeta {label: "Concept"}) RETURN meta.llm_context
```

## Decision Log

| Date | Decision | Rationale |
|------|----------|-----------|
| 2026-01-31 | Use meta-schema nodes (Option A) | Neo4j best practice, enables LLM context, queryable, self-describing |
| 2026-01-31 | Keep YAML as source of truth | Existing workflow, human-editable, generates both TS and Neo4j |
| 2026-01-31 | NO full JSON schema in NodeTypeMeta | Duplication risk, LLMs need llm_context not JSON Schema |
| 2026-01-31 | NO :OF_TYPE relationship | Redundant with labels, performance impact, maintenance burden |
| 2026-01-31 | New `organizing-principles.yaml` | True single source of truth, hierarchy.ts becomes generated |
| 2026-01-31 | Use MERGE for idempotent seeding | Re-seed safe, handles updates |

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Sync Complexity | HIGH | Create `OrganizingPrinciplesGenerator`, add to CI validation |
| Source of Truth Unclear | HIGH | `organizing-principles.yaml` is THE source, generate everything else |
| Schema Drift | HIGH | Add test: "Neo4j organizing principles match YAML" |
| Migration Path | MEDIUM | Phased rollout, backward compatible |

## Implementation Phases

### Phase 1: Foundation
1. Create `organizing-principles.yaml`
2. Create `OrganizingPrinciplesGenerator`
3. Add constraints for Scope, Subcategory, NodeTypeMeta
4. Add seed file `00.5-organizing-principles.cypher`
5. Add validation test

### Phase 2: Studio Integration
1. Create `useOrganizingPrinciples()` hook
2. Query Neo4j for hierarchy (with TypeScript fallback)
3. Test both sources

### Phase 3: Deprecation
1. Remove TypeScript fallback
2. Update documentation
