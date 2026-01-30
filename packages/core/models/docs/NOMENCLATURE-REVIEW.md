# NovaNet Nomenclature Deep Review v7.6.1

Multi-phase analysis of NovaNet graph naming conventions against industry best practices.

---

## Research Sources Consulted

| Source | Focus |
|--------|-------|
| Neo4j Cypher Manual 2025 | Official naming conventions |
| Schema.org | Vocabulary design philosophy |
| Ontology Design Patterns | ODP best practices |
| Ontology Anti-Patterns | Common mistakes to avoid |
| KG Construction Best Practices 2026 | Modern techniques |
| Perplexity + Context7 | Current industry standards |

---

## Phase 1: Industry Standards Summary

### Neo4j Official Conventions (Cypher Manual 2025)

| Element | Convention | Example |
|---------|------------|---------|
| **Node Labels** | CamelCase, uppercase first | `:VehicleOwner` not `:vehicle_owner` |
| **Relationship Types** | UPPER_SNAKE_CASE | `:OWNS_VEHICLE` not `:ownsVehicle` |
| **Properties** | snake_case | `created_at`, `display_name` |

### Schema.org Philosophy

```
Pragmatic > Pure
Practical usability > Ontological elegance
Incremental evolution > Global restructuring
Local coherence > Global consistency
```

### Ontology Design Patterns

- **TBox/ABox Separation**: Schema (classes) vs Data (instances)
- **SemSet Pattern**: Group synonyms, not separate classes
- **Property Reification**: N-ary relations as first-class entities
- **Punning**: Same IRI as class AND individual (metadata about classes)

---

## Phase 2: NovaNet Current State

### What We Have (v7.6.1)

```yaml
Nodes: 37 total
  - Labels: CamelCase (PageL10n, BlockType, LocaleVoice)
  - Properties: snake_case (display_name, created_at, llm_context)

Relationships: 43 total
  - Types: UPPER_SNAKE_CASE (HAS_L10N, FOR_LOCALE, SEMANTIC_LINK)
  - Props: snake_case (status, position, temperature)

Naming Patterns:
  - *L10n suffix: All localized content
  - Locale* prefix: Locale knowledge nodes
  - *Prompt suffix: AI instructions
  - *Rules suffix: Generation rules
  - *Metrics suffix: External data
```

---

## Phase 3: Socratic Questioning

### Q1: Why "L10n" suffix and not "Localized"?

**Current**: `PageL10n`, `BlockL10n`, `ConceptL10n`

**Challenge**: L10n is jargon. Is this clear to newcomers?

**Counter-argument**:
- L10n is industry-standard abbreviation (Localization = L + 10 letters + n)
- Keeps node names short (PageL10n vs PageLocalized)
- Already used in software: i18n, l10n, a11y

**Verdict**: KEEP. Industry standard, concise.

---

### Q2: Why both HAS_L10N and HAS_OUTPUT for localized content?

**Current**:
```
:HAS_L10N   -> human-curated (ConceptL10n, ProjectL10n)
:HAS_OUTPUT -> LLM-generated (PageL10n, BlockL10n)
```

**Challenge**: Why two relationships? Both produce localized content.

**Counter-argument**:
- Source matters for provenance
- Human content = reviewed, trusted
- LLM content = generated, needs validation
- Query pattern: `MATCH (x)-[:HAS_OUTPUT]->(y)` = find all generated content

**Verdict**: KEEP. Semantic distinction is valuable for audit trails.

---

### Q3: Why "Locale" and not "Language" or "Region"?

**Current**: `Locale` with key like `fr-FR`, `en-US`

**Challenge**: "Locale" conflates language + region. Is this right?

**Counter-argument**:
- Locale = language + region + cultural context
- BCP-47 standard uses locale codes
- We need cultural adaptation, not just translation
- `fr-FR` vs `fr-CA` = different cultural references, not just spelling

**Verdict**: KEEP. Locale is semantically correct for our use case.

---

### Q4: Why LocaleVoice, LocaleCulture, etc. and not Voice, Culture?

**Current**: `LocaleVoice`, `LocaleCulture`, `LocaleMarket`, etc.

**Challenge**: Redundant prefix since all are attached to Locale?

**Counter-argument**:
- Without prefix: `Voice` is ambiguous (brand voice? user voice?)
- Prefix provides namespace: all `Locale*` = locale knowledge
- Query pattern: `MATCH (n) WHERE n:Locale*` (would need label filtering)
- Self-documenting in Neo4j Browser

**Alternative considered**: Nested labels `:Locale:Voice`
- Problem: Neo4j doesn't have label namespaces

**Verdict**: KEEP. Prevents ambiguity, self-documenting.

---

### Q5: Why SEMANTIC_LINK and not more specific relationships?

**Current**: `SEMANTIC_LINK {type: "is_action_on", temperature: 0.95}`

**Challenge**: Why not separate relationships?
```
INSTEAD OF:  (A)-[:SEMANTIC_LINK {type: "includes"}]->(B)
WHY NOT:     (A)-[:INCLUDES]->(B)
```

**Counter-argument**:
- Spreading activation needs uniform traversal
- `MATCH (c)-[:SEMANTIC_LINK*1..2]->(c2)` = simple 2-hop query
- With specific types: would need `[:INCLUDES|IS_ACTION_ON|TYPE_OF*1..2]`
- Temperature is per-link, not per-type

**Alternative**: Index on `type` prop for filtering

**Verdict**: KEEP. Uniform traversal wins for spreading activation algorithm.

---

### Q6: Why singular relationship names? (HAS_BLOCK not HAS_BLOCKS)

**Current**: `HAS_BLOCK`, `HAS_CONCEPT`, `HAS_L10N`

**Challenge**: A page HAS multiple blocks. Shouldn't it be HAS_BLOCKS?

**Counter-argument**:
- Each relationship instance = ONE connection
- `(Page)-[:HAS_BLOCK]->(Block1)` and `(Page)-[:HAS_BLOCK]->(Block2)`
- NOT: `(Page)-[:HAS_BLOCKS]->(all blocks)`
- Neo4j convention: singular describes the relationship type, not cardinality

**Verdict**: KEEP. Follows Neo4j convention.

---

## Phase 4: Devil's Advocate

### Challenge 1: L10n Nodes Are Not TBox-Clean

**Issue**: ConceptL10n, PageL10n exist per-locale. Are they TBox or ABox?

```
TBox = Schema (classes, properties)
ABox = Data (instances, facts)
```

**Current situation**:
- `Concept` = invariant = TBox-like (defines what concepts exist)
- `ConceptL10n` = per-locale = ABox (instantiated data)

**Problem?**: No. This is correct.
- Concept is the class definition
- ConceptL10n is instance data for that concept in a locale
- Follows TBox/ABox separation correctly

---

### Challenge 2: Too Many Node Types?

**Issue**: 37 nodes. Is this too granular?

**Industry benchmark** (from anti-patterns doc):
- "Too Big Enterprise Graph" = trying to model everything
- "Start small, grow organically"

**Defense**:
- 37 nodes for a content generation system is reasonable
- Clear separation: Structure / Semantic / Locale / Generation / SEO / GEO
- Each node has distinct purpose
- No "synonyms as classes" anti-pattern

**Verdict**: 37 is reasonable. Well-organized by category.

---

### Challenge 3: Locale Knowledge Explosion

**Issue**: 14 nodes just for locale knowledge. Too many?

```
LocaleIdentity, LocaleVoice, LocaleCulture, LocaleMarket, LocaleLexicon,
LocaleRulesAdaptation, LocaleRulesFormatting, LocaleRulesSlug,
LocaleCultureReferences, Expression, Reference, Metaphor, Pattern, Constraint
```

**Defense**:
- Follows "Property Reification" pattern (N-ary relations)
- Each category serves LLM context differently
- Alternative: single `LocaleKnowledge` with JSON blobs
  - Problem: Loses graph queryability
  - Problem: Can't traverse `Expression -> used_in -> BlockL10n`

**Potential simplification**:
- Could merge `LocaleRulesAdaptation + LocaleRulesFormatting + LocaleRulesSlug` into `LocaleRules`
- But: Each has different schema, different queries

**Verdict**: KEEP granular. Graph-native > JSON blobs.

---

### Challenge 4: FOR_LOCALE vs Attached-To-Locale Confusion

**Issue**: Two patterns for locale association:

```
Pattern 1: :FOR_LOCALE (localized content)
  ConceptL10n -[:FOR_LOCALE]-> Locale
  = "This content is FOR French audience"

Pattern 2: :HAS_* (locale knowledge)
  Locale -[:HAS_VOICE]-> LocaleVoice
  = "French HAS this voice"
```

**Question**: Is this confusing?

**Defense**:
- Different semantics:
  - FOR_LOCALE = content targeting that locale
  - HAS_* = knowledge about that locale
- Direction matters:
  - Content -> Locale (many-to-one)
  - Locale -> Knowledge (one-to-many)

**Verdict**: KEEP. Semantically distinct patterns.

---

### Challenge 5: Mining Nodes Don't Follow *L10n Pattern

**Issue**: SEOMiningRun, GEOMiningRun don't have L10n suffix

**Current**:
- SEOKeywordL10n has L10n (localized)
- SEOMiningRun doesn't (it's a job, not content)

**Question**: Should it be SEOMiningRunL10n?

**Defense**: No.
- Mining runs are jobs, not content
- They don't have locale-specific versions
- One run produces keywords for one locale, but the run itself is invariant

**Verdict**: CORRECT as-is. Jobs ≠ Content.

---

## Phase 5: Ralph Wiggum Analysis

*"I'm Idaho!"* - Finding things that seem weird to a newcomer.

### Weird Thing 1: BlockL10n has GENERATED_FROM -> BlockType

```
BlockL10n -[:GENERATED_FROM]-> BlockType
```

**Ralph says**: "Wait, why does output point to type? Isn't Block the parent?"

**Answer**: Optimization shortcut. Allows direct type lookup without:
```cypher
MATCH (bl:BlockL10n)<-[:HAS_OUTPUT]-(b:Block)-[:OF_TYPE]->(bt:BlockType)
-- vs --
MATCH (bl:BlockL10n)-[:GENERATED_FROM]->(bt:BlockType)
```

**Verdict**: Documented optimization. Keep but note in docs.

---

### Weird Thing 2: PageL10n has BELONGS_TO_PROJECT_L10N

```
PageL10n -[:BELONGS_TO_PROJECT_L10N]-> ProjectL10n
```

**Ralph says**: "But PageL10n comes from Page which comes from Project. Why duplicate?"

**Answer**: Locale-aligned optimization for generation context.
```cypher
-- Without shortcut (multiple hops + locale filtering):
MATCH (pl:PageL10n)-[:FOR_LOCALE]->(l:Locale)
MATCH (p:Project)-[:HAS_L10N]->(proj:ProjectL10n)-[:FOR_LOCALE]->(l)

-- With shortcut (1 hop, locale-aligned):
MATCH (pl:PageL10n)-[:BELONGS_TO_PROJECT_L10N]->(proj:ProjectL10n)
-- ProjectL10n provides generation context (voice, tagline, CTAs) in same locale
```

**Constraint**: `PageL10n.locale == ProjectL10n.locale` (validated by application/SHACL)

**Verdict**: Documented optimization. Locale-aligned for native generation context.

---

### Weird Thing 3: SEOVariation VARIATES SEOKeywordL10n

```
SEOVariation -[:VARIATES]-> SEOKeywordL10n
SEOKeywordL10n -[:HAS_VARIATION]-> SEOVariation
```

**Ralph says**: "Two relationships for same thing? Pick one!"

**Answer**: Bidirectional optimization.
- HAS_VARIATION: "What variations does this keyword have?"
- VARIATES: "What keyword does this variation come from?"

Both queries are common; having both avoids MATCH direction issues.

**Verdict**: Keep both. Document as optimization pair.

---

### Weird Thing 4: Expression, Reference, Metaphor, Pattern, Constraint

**Ralph says**: "These don't have Locale prefix but they're locale things!"

**Current**:
```
LocaleLexicon -[:HAS_EXPRESSION]-> Expression
LocaleCultureReferences -[:HAS_REFERENCE]-> Reference
LocaleCultureReferences -[:HAS_METAPHOR]-> Metaphor
LocaleRulesFormatting -[:HAS_PATTERN]-> Pattern
LocaleCulture -[:HAS_CONSTRAINT]-> Constraint
```

**Question**: Should they be `LocaleExpression`, `LocaleReference`, etc.?

**Argument FOR current**:
- They're always attached to a Locale* parent
- Names are generic terms (Expression, not LocaleExpression)
- Shorter names

**Argument AGAINST**:
- Inconsistent with other Locale* nodes
- In isolation, "Expression" is ambiguous

**Recommendation**: Consider renaming for consistency:
```
Expression -> LocaleExpression
Reference -> LocaleReference
Metaphor -> LocaleMetaphor
Pattern -> LocalePattern
Constraint -> LocaleConstraint
```

**Verdict**: POTENTIAL ISSUE. Consider renaming for consistency.

---

### Weird Thing 5: AudienceL10n mentioned but no Audience node

**Ralph says**: "relations.yaml mentions AudienceL10n but where's Audience?"

```yaml
# From relations.yaml:
HAS_L10N:
  from: "[Concept, Project, Audience]"
  to: "[ConceptL10n, ProjectL10n, AudienceL10n]"
```

**Check**: Is Audience defined in nodes?

**Finding**: No Audience node defined. This is a documentation error.

**Verdict**: BUG. Remove Audience/AudienceL10n from relations.yaml or add node definitions.

---

### ~~Weird Thing 6: PAGE_TARGETS_SEO vs TARGETS_SEO~~ **RESOLVED v7.8.1**

**Ralph says**: "Why PAGE_TARGETS_SEO when we have TARGETS_SEO?"

```yaml
TARGETS_SEO: { from: Concept, to: SEOKeywordL10n }
PAGE_TARGETS_SEO: { from: Page, to: SEOKeywordL10n }  # REMOVED v7.8.1
```

**Resolution (v7.8.1)**: REMOVED PAGE_TARGETS_SEO and PAGE_TARGETS_GEO entirely.

**Why removed instead of merged**:
- Direct Page → SEO/GEO targeting **bypasses semantic grouping**
- SEO/GEO should flow through Concept for proper semantic organization
- Correct flow: `Page → Concept → ConceptL10n → SEOKeywordL10n/GEOSeedL10n`
- This ensures keywords are semantically grouped by concept, not scattered by page

**Verdict**: ~~POTENTIAL SIMPLIFICATION~~ **REMOVED** - relations were dangerous, not just redundant.

---

## Phase 6: Recommendations Summary

### Keep As-Is (Validated)

| Element | Reason |
|---------|--------|
| L10n suffix | Industry standard, concise |
| HAS_L10N vs HAS_OUTPUT | Semantic distinction (human vs LLM) |
| Locale naming | Correct for language+region+culture |
| Locale* prefix | Prevents ambiguity |
| SEMANTIC_LINK with props | Uniform traversal for spreading activation |
| Singular relationship names | Neo4j convention |
| 37 nodes | Reasonable for domain |
| FOR_LOCALE vs HAS_* | Semantically distinct |
| Job nodes without L10n | Correct: jobs ≠ content |

### Potential Issues to Address

| Issue | Severity | Recommendation |
|-------|----------|----------------|
| Expression, Reference, etc. naming | Medium | Consider `LocaleExpression`, `LocaleReference`, etc. |
| AudienceL10n without Audience | High | Remove from relations.yaml or add node |
| ~~PAGE_TARGETS_SEO redundancy~~ | ~~Low~~ | **RESOLVED v7.8.1** - Removed (bypassed semantic grouping) |
| Optimization relationships undocumented | Low | Add "optimization shortcut" notes to docs |

### Alignment with Industry Standards

| Standard | NovaNet Alignment |
|----------|-------------------|
| Neo4j naming conventions | FULL |
| Schema.org philosophy | HIGH (pragmatic, incremental) |
| Ontology Design Patterns | HIGH (TBox/ABox, reification) |
| Anti-patterns avoided | HIGH (no synonyms-as-classes, no polysemy) |

---

## Conclusion

NovaNet v7.6.1 nomenclature is **well-designed** and **industry-aligned**.

Minor improvements suggested:
1. Rename leaf locale nodes for consistency (Expression → LocaleExpression)
2. Fix AudienceL10n documentation inconsistency
3. ~~Consider simplifying PAGE_TARGETS_SEO → TARGETS_SEO~~ **RESOLVED v7.8.1** - Removed entirely
4. Document optimization shortcuts explicitly

The dual-axis classification system (Locale Behavior + Functional Role) is a strong addition that enhances understanding without changing the underlying schema.
