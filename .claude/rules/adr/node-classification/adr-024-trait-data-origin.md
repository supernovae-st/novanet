---
id: "024"
title: "Trait Redefinition as Data Origin"
version: "v11.8"
status: "active"
domain: "node-classification"
---

# ADR-024: Trait Redefinition as "Data Origin"

**Status**: Approved (v11.8)

**Problem**: Current trait system is NOT orthogonal to Layer:

1. **60% redundancy**: Most layers have single trait (instruction=invariant, output=generated)
2. **Name collision**: "knowledge" trait vs "knowledge" layer
3. **Catch-all**: 31 nodes are "invariant" but serve very different purposes
4. **Mixed semantics**: Traits mix "locale behavior" with "data origin"

Analysis by 5 brainstorming agents revealed Layer already answers "WHAT functional category?" - Trait should answer a DIFFERENT question.

**Decision**: Redefine Trait as "Data Origin" (WHERE does data come from?):

| Before | After | Definition |
|--------|-------|------------|
| invariant | **defined** | Defined by human, created ONCE. Structure/template. |
| localized | **authored** | Written by human, PER locale. Editorial content. |
| knowledge | **imported** | External data brought in. APIs, databases, corpora. |
| generated | **generated** | Produced by OUR LLM. NovaNet generates this. |
| aggregated | **retrieved** | Retrieved from EXTERNAL APIs. Snapshots of third-party data. |

**True Orthogonality**:

```
LAYER answers:  "WHAT functional category?"
                config, structure, semantic, instruction, output, knowledge...

TRAIT answers:  "WHERE does the data come from?"
                defined, authored, imported, generated, retrieved
```

**Node Distribution**:

| Trait | Count | Examples |
|-------|-------|----------|
| defined | 31 | Page, Block, Entity, PageStructure, PageInstruction, BlockInstruction, Locale, OrgConfig |
| imported | 20 | Term, Expression, Pattern, Culture, SEOKeyword, GEOQuery |
| authored | 2 | EntityNative, ProjectNative |
| generated | 4 | PageNative, BlockNative, OutputArtifact, PromptArtifact |
| retrieved | 2 | GEOAnswer, SEOKeywordMetrics |

**Key Clarification - GEOAnswer**:
- GEOAnswer is `retrieved`, NOT `generated`
- It's a SNAPSHOT of what Claude/GPT/Perplexity returned
- We RETRIEVED it from their API, we didn't generate it
- It's evidence of how AI engines see our content

**Rationale**:

1. **defined**: Human creates once, doesn't vary by locale
2. **authored**: Human writes content, per locale (editorial)
3. **imported**: Data brought in from external sources
4. **generated**: Our LLM produces output
5. **retrieved**: Fetched from third-party APIs (we capture, not create)

**Research**: Drupal Config/Content Entity, Sanity Document/Object, OWL TBox/ABox, Neo4j labeling patterns

**Reference**: 5-agent analysis, `docs/plans/2026-02-13-nomenclature-v118-design.md`
