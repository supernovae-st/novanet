# Proposal: llm_context Dual Pattern (BLOC 2 + BLOC 4)

**Status**: DRAFT
**Date**: 2026-02-17
**Violation**: VIOLATION #1 from schema-instance coherence report
**Research basis**: Perplexity (2 queries) + Skills analysis

---

## Problem Statement

**Current situation**:
- BLOC 2 llm_context exists in all 61 node YAMLs (schema metadata)
- BLOC 4 llm_context property does NOT exist in YAMLs
- BUT llm_context exists in many Neo4j instances (orphan property)
- schema-standard.md line 173 explicitly forbids llm_context in BLOC 4

**User correction** (Message 3):
> "ok mis ca c'est le llm_context du schema mais en sois y'a un llm_context de l'instance en elle meme"

**Translation**: There are TWO distinct llm_context:
1. Schema-level (BLOC 2): How to use the CLASS
2. Instance-level (BLOC 4): Specific context for this PARTICULAR instance

---

## Research Findings

### Perplexity Query 2: Knowledge graph node properties for LLM prompts

**Best practices identified**:
```
✅ NODE PROPERTIES SHOULD INCLUDE:
- Confidence scores (metadata for LLM filtering)
- Provenance (data source origin)
- Focused properties (2-4 key characteristics)
- Indexed, validated properties for prompts
- Relevance scores for filtering

❌ AVOID:
- Unindexed text blobs
- Mixing schema and instance concerns
- Overly complex nested structures
```

**Application to llm_context**:
- Type: `string` (simple, indexable)
- Required: `false` (not all instances need custom context)
- Content: 2-4 key characteristics + confidence/provenance where applicable
- Purpose: Instance-specific filtering and relevance scoring

### Skills Analysis

**Pattern from brainstorming/SKILL.md**:
```markdown
---
name: brainstorming  # ← Frontmatter (like BLOC 2)
description: ...
---

# Brainstorming Ideas  # ← Schema metadata

## Overview           # ← Content sections (like BLOC 4)
```

**Parallel to NovaNet**:
- YAML frontmatter = BLOC 2 llm_context (schema metadata)
- Content sections = BLOC 4 llm_context property (instance data)

---

## Proposed Solution

### Step 1: Update schema-standard.md

**REPLACE line 173**:
```markdown
❌ OLD (INCORRECT):
**Note:** `llm_context` is at BLOC 2 level (schema metadata), NOT in standard_properties.

✅ NEW (CORRECT):
**Note:** NovaNet uses a **dual llm_context pattern**:
- BLOC 2 llm_context: Schema metadata (how Claude uses the CLASS)
- BLOC 4 llm_context property: Instance data (specific to this PARTICULAR node)
```

**ADD new section after line 172**:

```markdown
## llm_context Dual Pattern (BLOC 2 + BLOC 4)

NovaNet employs a **dual llm_context pattern** for optimal LLM generation:

### BLOC 2 llm_context (Schema Metadata)

**Purpose**: Describes how Claude should USE the node CLASS in general.

**Location**: Between `trait:` and `description:` in BLOC 2

**Format**: USE/TRIGGERS/NOT/RELATES pattern

**Example**:
```yaml
node:
  name: Brand
  realm: org
  layer: foundation
  trait: defined

  # BLOC 2: Schema-level instructions for Claude
  llm_context: |
    USE: when loading brand context for content generation or enforcing brand consistency.
    TRIGGERS: "brand", "brand voice", "brand identity", "brand context", "@brand".
    NOT: for visual design specifics (use BrandDesign), for voice guidelines (use BrandPrinciples).
    RELATES: Project (owner via HAS_BRAND), BrandDesign (visuals via HAS_DESIGN).

  description: "Brand identity and positioning for a project"
```

### BLOC 4 llm_context (Instance Property)

**Purpose**: Instance-specific context for LLM generation. Describes unique characteristics, constraints, or requirements for this PARTICULAR node.

**Location**: In `properties:` section, after standard_properties, before domain-specific properties

**Type**: `string` (indexable, simple)

**Required**: `false` (not all instances need custom context)

**Best practices** (from research):
- Keep focused: 2-4 key characteristics
- Include confidence scores or provenance where relevant
- Use for filtering and relevance scoring
- Avoid duplicating BLOC 2 schema-level information

**Property definition template**:
```yaml
properties:
  # ═══════════════════════════════════════════════════════════════════
  # LLM Generation Context (instance-specific)
  # ═══════════════════════════════════════════════════════════════════
  llm_context:
    type: string
    required: false
    description: |
      Instance-specific context for LLM generation. Describes unique characteristics,
      constraints, or requirements for this particular instance.

      Best practices:
      - Keep focused (2-4 key characteristics)
      - Include confidence/provenance if relevant (e.g., "Confidence: high (manually curated)")
      - Use for filtering and relevance scoring
      - Avoid duplicating BLOC 2 schema-level information

    example: |
      Modern tech-forward brand targeting small businesses.
      Emphasis on simplicity and speed. Avoid corporate jargon.
      Confidence: high (manually curated by brand team).
```

### When to Use BLOC 2 vs BLOC 4

| Aspect | BLOC 2 llm_context | BLOC 4 llm_context property |
|--------|-------------------|---------------------------|
| **Scope** | All instances of this CLASS | This specific INSTANCE only |
| **Audience** | Claude (how to use this type of node) | LLM generators (context for this particular node) |
| **Frequency** | Written once (schema definition) | Written per instance (data) |
| **Content** | USE/TRIGGERS/NOT/RELATES (generic instructions) | 2-4 unique characteristics (specific context) |
| **Example (Brand)** | "USE: when loading brand context for content generation" | "Modern tech-forward brand targeting SMBs. Emphasis on simplicity." |
| **Example (Entity)** | "USE: when loading semantic entity definition" | "High-priority pillar entity. SEO target for Q1 2026 campaign." |

### Validation Rule

**Rule**: If a Neo4j instance has `llm_context` property, the corresponding NodeClass YAML MUST declare `llm_context` in BLOC 4 properties.

**coherence_check.rs implementation**:
```rust
pub enum CoherenceViolation {
    UndeclaredProperty {
        node_key: String,
        property_name: String,
        property_value: serde_json::Value,
    },
}

impl CoherenceChecker {
    fn check_llm_context(&self, node: &NodeInstance, schema: &NodeClass) -> Option<CoherenceViolation> {
        if node.has_property("llm_context") && !schema.has_property("llm_context") {
            Some(CoherenceViolation::UndeclaredProperty {
                node_key: node.key.clone(),
                property_name: "llm_context".to_string(),
                property_value: node.get_property("llm_context").unwrap().clone(),
            })
        } else {
            None
        }
    }
}
```
```

---

### Step 2: Update brand.yaml with BLOC 4 llm_context

**Current brand.yaml** (lines 64-111 - properties section):
```yaml
properties:
  # Brand Identity
  brand_name:
    type: string
    required: true
    description: "Official brand name"
    example: "QR Code AI"

  tagline:
    type: string
    required: false
    description: "Brand tagline or slogan"
    max_length: 100
    example: "Generate QR codes with AI-powered customization"

  # ... (other properties)
```

**PROPOSED UPDATE**:
```yaml
properties:
  # ═══════════════════════════════════════════════════════════════════
  # LLM Generation Context (instance-specific)
  # ═══════════════════════════════════════════════════════════════════
  llm_context:
    type: string
    required: false
    description: |
      Instance-specific context for LLM generation. Describes unique brand characteristics,
      tone constraints, or target audience specifics for this particular brand.

      Best practices:
      - Keep focused (2-4 key characteristics)
      - Include brand personality traits
      - Specify tone constraints (e.g., avoid corporate jargon)
      - Include confidence/provenance if relevant

    example: |
      Modern tech-forward brand targeting small businesses and marketing teams.
      Emphasis on simplicity, speed, and accessibility. Professional but approachable—avoid corporate jargon.
      Confidence: high (manually curated by brand team, reviewed Q4 2025).

  # ═══════════════════════════════════════════════════════════════════
  # Brand Identity
  # ═══════════════════════════════════════════════════════════════════
  brand_name:
    type: string
    required: true
    description: "Official brand name"
    example: "QR Code AI"

  tagline:
    type: string
    required: false
    description: "Brand tagline or slogan"
    max_length: 100
    example: "Generate QR codes with AI-powered customization"

  # ... (rest of properties)
```

**BLOC placement**:
1. Standard properties (key, display_name, description, timestamps)
2. **llm_context** (instance-specific LLM context)
3. Domain-specific properties (brand_name, tagline, brand_story, etc.)

**Rationale for placement**:
- After standard properties: llm_context is metadata, not core identity
- Before domain properties: llm_context affects how domain properties are generated
- Consistent position across all node types

---

### Step 3: Create Template for All Node Types

**Property definition template** (copy-paste for all 61 YAMLs):

```yaml
  # ═══════════════════════════════════════════════════════════════════
  # LLM Generation Context (instance-specific)
  # ═══════════════════════════════════════════════════════════════════
  llm_context:
    type: string
    required: false
    description: |
      Instance-specific context for LLM generation. Describes unique characteristics,
      constraints, or requirements for this particular [NODE_TYPE] instance.

      Best practices:
      - Keep focused (2-4 key characteristics)
      - Include confidence/provenance if relevant
      - Use for filtering and relevance scoring
      - Avoid duplicating BLOC 2 schema-level information

    example: |
      [NODE_TYPE-SPECIFIC EXAMPLE]
      Confidence: high (manually curated).
```

**Node-specific examples**:

| Node Type | Example llm_context (BLOC 4 property) |
|-----------|--------------------------------------|
| Brand | "Modern tech-forward brand targeting SMBs. Emphasis on simplicity and speed. Avoid corporate jargon. Confidence: high (manually curated)." |
| Entity | "High-priority pillar entity for Q1 2026 SEO campaign. Target keyword: 'qr code generator'. Confidence: medium (AI-suggested, human-reviewed)." |
| Page | "Homepage with hero + features + pricing sections. Primary conversion goal: free trial signup. Confidence: high (product team approved)." |
| EntityNative | "Translated from English source with French localization team review. Emphasize French cultural references (terroir, artisanat). Confidence: high." |
| SEOKeyword | "High-volume keyword (50K searches/month). Primary target for organic traffic. Confidence: high (Ahrefs verified, updated 2026-02-15)." |
| PromptStyle | "Preset for hero section illustrations. Influenced by Japanese minimalism. Use for homepage and landing pages. Confidence: medium (design team draft)." |

---

### Step 4: Update Seed Files to Include llm_context

**Example**: `packages/db/seed/31-project-qrcode-ai.cypher`

```cypher
CREATE (brand:Brand {
  key: "brand-qrcode-ai",
  display_name: "QR Code AI Brand",
  description: "Brand identity for QR Code AI platform",
  created_at: datetime(),
  updated_at: datetime(),

  // ✅ ADD: Instance-specific llm_context
  llm_context: "Modern tech-forward brand targeting small businesses and marketing teams. Emphasis on simplicity, speed, and accessibility. Professional but approachable—avoid corporate jargon. Confidence: high (manually curated by brand team, reviewed Q4 2025).",

  // Brand-specific properties
  brand_name: "QR Code AI",
  tagline: "Generate QR codes with AI-powered customization",
  brand_story: "QR Code AI revolutionizes QR code generation...",
  logo_primary_url: "/assets/logo-primary.svg",
  // ...
})
```

---

### Step 5: Validation Rules

**coherence_check.rs rules**:

1. **RULE: llm_context_declared**
   ```
   IF instance has llm_context property
   THEN NodeClass YAML MUST declare llm_context in BLOC 4 properties
   ```

2. **RULE: llm_context_not_duplicating_schema**
   ```
   IF instance llm_context contains USE/TRIGGERS/NOT/RELATES pattern
   THEN WARN: instance llm_context should not duplicate BLOC 2 schema metadata
   ```

3. **RULE: llm_context_focused**
   ```
   IF instance llm_context length > 500 characters
   THEN WARN: instance llm_context should be focused (2-4 key characteristics)
   ```

**Validation query**:
```cypher
// Find instances with llm_context property
MATCH (n)
WHERE n.llm_context IS NOT NULL
RETURN labels(n)[0] AS node_class,
       n.key AS instance_key,
       length(n.llm_context) AS context_length,
       n.llm_context AS context_value
ORDER BY node_class, instance_key
```

---

## Implementation Plan

### Phase 1: Documentation (1-2 hours)

- [ ] Update schema-standard.md with dual llm_context pattern section
- [ ] Add llm_context property definition template
- [ ] Document BLOC 2 vs BLOC 4 decision matrix
- [ ] Update BLOC order documentation

### Phase 2: Schema Updates (8-12 hours)

**Priority nodes** (have llm_context in instances per grep):
- [ ] Brand, BrandDesign, BrandPrinciples (org/foundation)
- [ ] Entity, EntityNative (org/semantic)
- [ ] Page, PageNative (org/structure + org/output)
- [ ] Block, BlockNative (org/structure + org/output)
- [ ] PromptStyle, PromptArtifact (org/foundation + org/instruction)
- [ ] Project, ProjectNative (org/foundation)

**All 61 nodes** (for consistency):
- [ ] All shared/ nodes (40 nodes)
- [ ] All org/ nodes (21 nodes)

**Template approach**:
1. Create `llm_context_property_template.yaml` snippet
2. Use sed/awk to insert after standard_properties in all 61 YAMLs
3. Customize example for each node type
4. Verify with `cargo run -- schema validate`

### Phase 3: Seed File Updates (2-4 hours)

- [ ] Update all seed files that create instances with llm_context
- [ ] Ensure property order matches YAML (standard_props → llm_context → domain_props)
- [ ] Verify with Neo4j Browser

### Phase 4: Validation (2-3 hours)

- [ ] Implement coherence_check.rs rules for llm_context
- [ ] Add `cargo run -- schema coherence --check-llm-context` command
- [ ] Test against brand-qrcode-ai instance
- [ ] Document auto-fix limitations (cannot infer llm_context value)

### Phase 5: CI Integration (1-2 hours)

- [ ] Add coherence check to GitHub Actions workflow
- [ ] Fail build if instances have undeclared llm_context properties
- [ ] Add pre-commit hook reminder

**Total estimated effort**: 14-23 hours

---

## Auto-fix Capability

❌ **NO AUTO-FIX for llm_context values**

**Reason**: llm_context is instance-specific semantic content that requires human judgment.

**What CAN be auto-fixed**:
- ✅ Add property definition to YAML (template insertion)
- ✅ Detect missing declaration (validation rule)

**What CANNOT be auto-fixed**:
- ❌ Generate llm_context value for existing instances
- ❌ Infer appropriate context from other properties
- ❌ Determine confidence/provenance automatically

**Manual intervention required**:
- Review each instance that has llm_context in Neo4j
- Verify content is appropriate
- Add llm_context property to YAML
- Ensure seed files include llm_context when creating instances

---

## Rollout Strategy

### Week 1: Documentation + High-Priority Nodes

**Days 1-2**:
- Update schema-standard.md
- Create llm_context property template
- Update 6 high-priority nodes (Brand, Entity, Page, EntityNative, PageNative, BlockNative)

**Days 3-5**:
- Update seed files for QR Code AI project (31-project-qrcode-ai.cypher)
- Test coherence check against updated nodes
- Document findings

### Week 2: Remaining Nodes + Validation

**Days 6-8**:
- Update remaining 55 nodes with llm_context template
- Customize examples for each node type
- Verify with `cargo run -- schema validate`

**Days 9-10**:
- Implement coherence_check.rs rules
- Test against all seed files
- Fix any violations found

### Week 3: CI + Documentation

**Days 11-12**:
- Integrate coherence check into CI/CD
- Add pre-commit hooks
- Update developer documentation

**Day 13**:
- Review and retrospective
- Update ADR if needed

---

## Open Questions

1. **Property naming**: Should we use `llm_context` or alternative names?
   - `llm_context`: Consistent with BLOC 2
   - `prompt_context`: More specific to generation
   - `generation_context`: Clearer purpose
   - **RECOMMENDATION**: Keep `llm_context` for consistency with BLOC 2

2. **Property type**: Should it be `string` or `json`?
   - `string`: Simple, indexable, human-readable
   - `json`: Structured, machine-parseable, extensible
   - **RECOMMENDATION**: `string` per Perplexity research (indexable, simple)

3. **Required status**: Should it be required or optional?
   - Required: Ensures all instances have context
   - Optional: Not all instances need custom context
   - **RECOMMENDATION**: `false` per Perplexity research (not all instances need it)

4. **Confidence/provenance format**: Should we standardize the format?
   - Free text: Flexible but inconsistent
   - Structured: `Confidence: [high|medium|low] (source: [manual|AI|imported], updated: YYYY-MM-DD)`
   - **RECOMMENDATION**: Start with free text, standardize later if needed

---

## Success Criteria

1. ✅ **Documentation**: schema-standard.md documents dual llm_context pattern
2. ✅ **Schema compliance**: All 61 node YAMLs have llm_context property definition
3. ✅ **Seed files**: All seed files creating llm_context instances declare property in YAML
4. ✅ **Validation**: coherence_check.rs detects undeclared llm_context properties
5. ✅ **CI**: Build fails if instances have undeclared properties
6. ✅ **Zero violations**: `cargo run -- schema coherence --check-all` returns 0 violations

---

## References

- VIOLATION #1 report: `docs/plans/2026-02-17-schema-instance-coherence-violations.md`
- Perplexity research: Query 2 - Knowledge graph node properties best practices
- Skills analysis: brainstorming/SKILL.md, writing-plans/SKILL.md
- schema-standard.md: Current line 173 (INCORRECT statement to be updated)
- ADR-024: Trait = Data Origin (confidence/provenance metadata)
