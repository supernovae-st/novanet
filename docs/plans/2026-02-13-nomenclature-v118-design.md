# NovaNet Nomenclature v11.8 Design

**Date**: 2026-02-13
**Status**: Approved (pending implementation)
**Version**: v11.8

## Overview

This document captures the nomenclature decisions made during the brainstorming session to simplify and clarify NovaNet's terminology for both humans and LLMs.

## Decision 1: Kind → Class (ADR-023)

### Problem
- "Kind" is non-standard graph theory terminology
- LLMs have less training data on "Kind" vs "Class"
- French translation "Genre" was awkward

### Solution
Rename schema-level terminology from "Kind" to "Class":

| Before | After | Context |
|--------|-------|---------|
| NodeKind | NodeClass | Rust/TypeScript struct |
| ArcKind | ArcClass | Rust/TypeScript struct |
| KindInfo | ClassInfo | TUI struct |
| TreeItem::Kind | TreeItem::Class | Rust enum variant |
| [:FROM_KIND] | [:FROM_CLASS] | Neo4j relationship |
| [:TO_KIND] | [:TO_CLASS] | Neo4j relationship |
| [:HAS_KIND] | [:HAS_CLASS] | Neo4j relationship |
| "Node Kinds" | "Classes" | UI label |

### Rationale
- `rdfs:Class`, `owl:Class` are in LLM training data millions of times
- "Class/Instance" is THE canonical OOP and ontology pairing
- Universal: "Classe/Instance" (FR), "Clase/Instancia" (ES), "Klasse/Instanz" (DE)

## Decision 2: Meta Elimination (ADR-023)

### Problem
- "Meta" is ambiguous (Facebook collision, Spanish "meta" = goal)
- Mixed usage: "Meta Node", "KindMeta", Neo4j `:Meta:` labels

### Solution
Eliminate "Meta" prefix/suffix entirely - use semantic names:

| Before | After | Context |
|--------|-------|---------|
| KindMeta | Classification | TypeScript interface (realm/layer/trait) |
| KIND_META | CLASS_TAXONOMY | TypeScript constant |
| :Meta:Kind | :Schema:Class | Neo4j label |
| :Meta:ArcKind | :Schema:ArcClass | Neo4j label |
| "Meta Node" | "Class" | Glossary |
| "Data Node" | "Instance" | Glossary |
| "Meta mode" | "Schema view" | Studio UI |
| "Data mode" | "Graph view" | Studio UI |

### Rationale
- `Classification` describes WHAT it contains (realm/layer/trait axes)
- `Schema` describes WHAT it is (the schema, not data)
- "Meta" described NOTHING

## Decision 3: Trait Redefinition (ADR-024)

### Problem
Current traits conflate multiple concerns and overlap with Layer:

1. **60% redundancy**: Most layers have a single trait (instruction=invariant, output=generated)
2. **Name collision**: "knowledge" trait vs "knowledge" layer
3. **Catch-all category**: 31 nodes are "invariant" but serve very different purposes
4. **Mixed semantics**: Traits mix "locale behavior" with "data origin"

Analysis by 5 brainstorming agents revealed:
- Layer already answers "WHAT functional category?"
- Trait should answer a DIFFERENT question to be truly orthogonal

### Solution: Redefine Trait as "Data Origin"

**New question**: "WHERE does this data come from?"

| Before | After | Definition | Examples |
|--------|-------|------------|----------|
| invariant | **defined** | Defined by human, created ONCE. Structure/template. | Page, Block, PageType, BlockType, Locale, OrgConfig |
| localized | **authored** | Written by human, PER locale. Editorial content. | EntityContent, ProjectContent |
| knowledge | **imported** | External data brought in. APIs, databases, corpora. | Term, Expression, SEOKeyword, GEOQuery |
| generated | **generated** | Produced by OUR LLM. NovaNet generates this. | PageGenerated, BlockGenerated, OutputArtifact |
| aggregated | **retrieved** | Retrieved from EXTERNAL APIs. Snapshots of third-party data. | GEOAnswer, SEOKeywordMetrics, GEOMetrics |

### Rationale: True Orthogonality

```
LAYER answers:  "WHAT functional category?"
                config, structure, semantic, instruction, output, knowledge...

TRAIT answers:  "WHERE does the data come from?"
                defined, authored, imported, generated, retrieved
```

These axes are NOW truly independent:
- A `knowledge` layer node can be `imported` (Term) or `defined` (TermSet container)
- A `semantic` layer node can be `defined` (Entity) or `authored` (EntityContent)
- An `output` layer node is always `generated` (but that's a valid pattern, not redundancy)

### New Trait Definitions

**defined** (was invariant):
- Human creates this ONCE, it doesn't vary by locale
- Templates, configurations, structural definitions
- "This is how things are set up"

**authored** (was localized):
- Human writes this content, PER locale
- Editorial content, curated descriptions
- "A human wrote this in French/Japanese/etc."

**imported** (was knowledge):
- Data brought in from external sources
- Linguistic corpora, market data, discovered keywords
- "We imported this from [source]"

**generated** (unchanged):
- Our LLM produces this output
- Final content for publication
- "NovaNet generated this"

**retrieved** (was aggregated):
- Snapshots from third-party APIs
- We don't create it, we capture it
- "We fetched this from Claude/GPT/Ahrefs/etc."

### Key Clarification: GEOAnswer

GEOAnswer is `retrieved`, NOT `generated`:
- It's a SNAPSHOT of what Claude/GPT/Perplexity returned
- We RETRIEVED it from their API, we didn't generate it
- It's evidence of how AI engines see our content
- Trait `retrieved` is correct (external API snapshot)

## Trait Distribution (60 nodes)

| Trait | Count | Nodes |
|-------|-------|-------|
| defined | 31 | Page, Block, Entity, Project, BrandIdentity, OrgConfig, PageType, BlockType, PagePrompt, BlockPrompt, Locale, EntityCategory, SEOKeywordFormat, TermSet, ExpressionSet, etc. |
| imported | 22 | Term, Expression, Pattern, CultureRef, Taboo, AudienceTrait, Culture, Style, Formatting, Adaptation, Market, Slugification, SEOKeyword, GEOQuery, etc. |
| authored | 2 | EntityContent, ProjectContent |
| generated | 5 | PageGenerated, BlockGenerated, OutputArtifact, PromptArtifact |
| retrieved | 3 | GEOAnswer, SEOKeywordMetrics, GEOMetrics |

## Summary: Before → After

```
v11.7 (Current)                    v11.8 (Proposed)
─────────────────────────────────────────────────────────────────
SCHEMA LEVEL:
  NodeKind                    →    NodeClass
  ArcKind                     →    ArcClass
  KindMeta                    →    Classification
  :Meta:Kind                  →    :Schema:Class

DATA LEVEL:
  "Meta Node"                 →    "Class"
  "Data Node"                 →    "Instance"

TRAITS (redefined as "Data Origin"):
  invariant                   →    defined    (human-created once)
  localized                   →    authored   (human-written per locale)
  knowledge                   →    imported   (external data brought in)
  generated                   →    generated  (our LLM produces)
  aggregated                  →    retrieved  (fetched from external APIs)
```

## Migration Impact

| Zone | Files | Estimated Effort |
|------|-------|------------------|
| Rust (Kind→Class) | 43 | 4-8h |
| TypeScript (Kind→Class) | 19 | 2-4h |
| Neo4j Migration | - | 1h |
| Trait Renaming (YAML) | 60 | 2h |
| Trait Renaming (Rust/TS) | ~80 | 2-4h |
| TUI/Nexus | 20+ | 3-5h |
| Documentation | 14 | 1-2h |
| Studio | 8 | 2-3h |
| **Total** | | **17-29h** |

## Implementation Order

1. **Phase 1: Schema Changes**
   - Update `taxonomy.yaml` with new trait names
   - Update all 60 node YAML files (trait field)
   - Run `cargo run -- schema generate`

2. **Phase 2: Neo4j Migration**
   - Create migration script for label changes (:Meta: → :Schema:)
   - Update relationship types (KIND → CLASS)

3. **Phase 3: Rust Code**
   - Rename structs (NodeKind → NodeClass, etc.)
   - Update trait enums
   - Run tests

4. **Phase 4: TypeScript Code**
   - Rename types and interfaces
   - Update constants (KIND_META → CLASS_TAXONOMY)
   - Run tests

5. **Phase 5: TUI/Nexus**
   - Update glossary entries
   - Update i18n strings
   - Update UI labels

6. **Phase 6: Studio**
   - Update ViewPicker labels
   - Update component text

7. **Phase 7: Documentation**
   - Update ADRs
   - Update terminology reference
   - Update CLAUDE.md files

## References

- ADR-023: Class/Instance Terminology + Meta Elimination (novanet-decisions.md)
- ADR-024: Trait Redefinition as "Data Origin" (novanet-decisions.md)
- Brainstorming session: 2026-02-12/13
- 5-agent analysis: Pipeline flow, GEO/SEO, Trait×Layer matrix, Devil's advocate, Industry research
- Research: FRBR, SKOS, ISO 25964, Drupal Entity API, Sanity CMS, Contentful, Neo4j best practices
