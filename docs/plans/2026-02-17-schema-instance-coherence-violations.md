# Schema-Instance Coherence Violations Report (v0.13.1)

**Date**: 2026-02-17
**Session**: Continuation - Schema-Instance Coherence Check
**Agents**: 5 parallel Explore agents (haiku model)
**Research**: Perplexity (2 queries) + Skills analysis + Grep (61 files)

---

## Executive Summary

```
VIOLATIONS CRITIQUES: 5
Instances affectées: 200+ (Slugification) + 1 (Brand) + unknown (llm_context)
Seed files à corriger: 3 minimum (22-slugification.cypher, 31-project-qrcode-ai.cypher, +?)
Auto-fix capability: 2/5 (Slugification timestamps, Property order)
Manual fix required: 3/5 (llm_context BLOC 4, Brand/BrandDesign split, brand_name)
```

---

## VIOLATION #1: llm_context BLOC 4 Property (CRITIQUE)

### Discovery Source
- User correction (Message 3): "ok mis ca c'est le llm_context du schema mais en sois y'a un llm_context de l'instance en elle meme"
- Confirmed via brand.yaml read: llm_context exists in Neo4j instance but NOT declared in BLOC 4 properties

### Current State

**BLOC 2 (schema metadata) - EXISTS**:
```yaml
node:
  name: Brand
  llm_context: |
    USE: when loading brand context for content generation or enforcing brand consistency.
    TRIGGERS: "brand", "brand voice", "brand identity", "brand context", "@brand".
    NOT: for visual design specifics (use BrandDesign), for voice guidelines (use BrandPrinciples).
    RELATES: Project (owner via HAS_BRAND), BrandDesign (visuals via HAS_DESIGN).
```

**BLOC 4 (instance property) - MISSING**:
```yaml
properties:
  # ❌ No llm_context property declared here!
  brand_name:
    type: string
    required: true
  # ...
```

**Neo4j instance - ORPHAN PROPERTY**:
```cypher
(brand:Brand {
  key: "brand-qrcode-ai",
  llm_context: "Modern tech-forward brand targeting small businesses. Emphasis on simplicity and speed."
  # ↑ This property exists in database but has no YAML schema definition!
})
```

### Impact
- **All nodes** where instances have llm_context in Neo4j
- Grep result: 61 YAML files potentially affected
- Creates schema drift: instances have properties not defined in schema

### Root Cause
**Dual llm_context pattern not documented**:
1. BLOC 2 llm_context = Schema-level instructions for Claude (how to use this CLASS)
2. BLOC 4 llm_context property = Instance-specific context (unique to this PARTICULAR node)

Current schema-standard.md line 173 explicitly states:
```markdown
**Note:** `llm_context` is at BLOC 2 level (schema metadata), NOT in standard_properties.
```

This is **INCORRECT** - should allow BOTH levels.

### Solution

**Step 1: Update schema-standard.md**

```markdown
## llm_context Dual Pattern

NovaNet uses a **dual llm_context pattern**:

1. **BLOC 2 llm_context** (schema metadata):
   - Describes how Claude should USE the node CLASS
   - Generic instructions applicable to ALL instances
   - USE/TRIGGERS/NOT/RELATES pattern

2. **BLOC 4 llm_context property** (instance data):
   - Instance-specific generation context
   - Unique characteristics for THIS particular node
   - Optional (not all instances need custom context)

Example:
```yaml
node:
  name: Brand
  # BLOC 2: Schema-level (how to use Brand class)
  llm_context: |
    USE: when loading brand context for content generation.
    TRIGGERS: "brand", "@brand".

  properties:
    # BLOC 4: Instance-level (unique to this brand)
    llm_context:
      type: string
      required: false
      description: "Instance-specific LLM generation context and instructions"
```
```

**Step 2: Add llm_context property to ALL affected YAMLs**

Based on Perplexity research ("Metadata like confidence scores and data source origin for LLM filtering"):

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
      - Include confidence scores or provenance where relevant
      - Keep focused (2-4 key characteristics)
      - Use for filtering and relevance scoring
      - Avoid duplicating schema-level BLOC 2 information

    example: |
      Modern tech-forward brand targeting small businesses.
      Emphasis on simplicity and speed. Avoid corporate jargon.
      Confidence: high (manually curated).
```

**Step 3: Identify all nodes needing this property**

Candidates (from grep):
- Brand, BrandDesign, BrandPrinciples
- Entity, EntityNative
- Page, PageNative
- Block, BlockNative
- PromptStyle, PromptArtifact
- Project, ProjectNative

**Validation rule**: If seed file creates llm_context in Neo4j, YAML must declare it in BLOC 4

### Auto-fix Capability
❌ **NO** - Requires design decision on:
- Property name (llm_context vs prompt_context vs generation_context)
- Property type (string vs json)
- Required status (true vs false)
- Placement in property order

---

## VIOLATION #2: Brand Properties Merged with BrandDesign (CRITIQUE)

### Discovery Source
- Agent 4: Undeclared properties analysis
- Found 13 design properties in brand-qrcode-ai instance that don't exist in brand.yaml

### Current State

**brand.yaml BLOC 4 - DEFINED**:
```yaml
properties:
  brand_name:
    type: string
    required: true
  tagline:
    type: string
  brand_story:
    type: string
  # ... NO design properties declared
```

**Neo4j instance - UNDECLARED PROPERTIES**:
```cypher
(brand:Brand {
  key: "brand-qrcode-ai",

  // ❌ VIOLATION: 13 BrandDesign properties in Brand node
  // Should be in separate BrandDesign node per ADR-028

  // Color properties (6)
  color_primary: "#0066FF",
  color_secondary: "#00E5CC",
  color_accent: "#FF6B35",
  color_background: "#FFFFFF",
  color_text: "#1A1A1A",
  color_palette: ["#0066FF", "#00E5CC", "#FF6B35"],

  // Typography properties (4)
  font_primary: "Inter",
  font_secondary: "Space Grotesk",
  font_mono: "JetBrains Mono",
  typography_scale: "1.250",

  // Style properties (3)
  shadow_style: "modern",
  border_radius: "8px",
  animation_style: "smooth"
})
```

**Seed file**: `packages/db/seed/31-project-qrcode-ai.cypher` (lines unknown - needs verification)

### Impact
- **1 instance** (brand-qrcode-ai)
- Violates ADR-028 Brand Architecture: Brand ──[:HAS_DESIGN {1:1 mandatory}]──> BrandDesign
- Schema drift: 13 undeclared properties in Neo4j

### Architecture Intent (ADR-028)

```
Brand (org/foundation, defined)
  ├── brand_name, tagline, brand_story, logo_primary_url
  ├── value_proposition, target_market
  └── [:HAS_DESIGN {1:1}]──> BrandDesign (org/foundation, defined)
                                 ├── design_philosophy, style_keywords
                                 ├── color_primary, color_secondary, ...
                                 ├── font_primary, font_secondary, ...
                                 └── typography_scale, shadow_style, ...
```

### Solution

**Step 1: Create BrandDesign node in seed file**

```cypher
// 31-project-qrcode-ai.cypher

// Brand node (clean, only Brand properties)
CREATE (brand:Brand {
  key: "brand-qrcode-ai",
  display_name: "QR Code AI Brand",
  description: "Brand identity for QR Code AI platform",
  created_at: datetime(),
  updated_at: datetime(),

  // Brand-specific properties
  brand_name: "QR Code AI",
  tagline: "Generate QR codes with AI-powered customization",
  brand_story: "QR Code AI revolutionizes QR code generation by combining powerful AI with an intuitive interface...",
  logo_primary_url: "/assets/logo-primary.svg",
  logo_icon_url: "/assets/logo-icon.svg",
  logo_usage_rules: "Minimum clear space: 1x logo height. Never distort or rotate.",
  target_market: "Small to medium businesses, marketing teams, event organizers",
  value_proposition: "Create professional, customizable QR codes in seconds with AI-powered suggestions"
})

// BrandDesign node (all design properties)
CREATE (design:BrandDesign {
  key: "brand-design-qrcode-ai",
  display_name: "QR Code AI Design System",
  description: "Visual design system for QR Code AI brand",
  created_at: datetime(),
  updated_at: datetime(),

  // Design philosophy
  design_philosophy: "Modern, minimalist, accessible. Technology-forward without being intimidating.",
  style_keywords: ["modern", "clean", "tech-forward", "accessible", "professional"],
  style_mood: "Confident and approachable. Professional but not corporate.",

  // Color system (primitives)
  color_primary: "#0066FF",
  color_secondary: "#00E5CC",
  color_accent: "#FF6B35",
  color_background: "#FFFFFF",
  color_text: "#1A1A1A",
  color_palette: ["#0066FF", "#00E5CC", "#FF6B35", "#F5F5F5", "#1A1A1A"],

  // Typography
  font_primary: "Inter",
  font_secondary: "Space Grotesk",
  font_mono: "JetBrains Mono",
  typography_scale: "1.250",

  // UI patterns
  shadow_style: "modern",
  border_radius: "8px",
  animation_style: "smooth"
})

// 1:1 mandatory relationship
CREATE (brand)-[:HAS_DESIGN]->(design)
```

**Step 2: Remove design properties from Brand instance**

Migration Cypher:
```cypher
// Remove undeclared properties from Brand
MATCH (b:Brand {key: "brand-qrcode-ai"})
REMOVE b.color_primary, b.color_secondary, b.color_accent,
       b.color_background, b.color_text, b.color_palette,
       b.font_primary, b.font_secondary, b.font_mono,
       b.typography_scale, b.shadow_style, b.border_radius,
       b.animation_style
```

**Step 3: Verify ADR-028 compliance**

```cypher
// Validation query: Brand MUST have exactly one BrandDesign
MATCH (b:Brand)
OPTIONAL MATCH (b)-[:HAS_DESIGN]->(d:BrandDesign)
WITH b, count(d) AS design_count
WHERE design_count <> 1
RETURN b.key AS brand_key,
       design_count AS actual_designs,
       "Expected 1" AS expected_designs
```

### Auto-fix Capability
❌ **NO** - Requires manual data migration:
1. Create BrandDesign node with correct properties
2. Create [:HAS_DESIGN] relationship
3. Remove properties from Brand node
4. Requires understanding of which properties belong where (schema knowledge)

---

## VIOLATION #3: Slugification Missing Timestamps (CRITIQUE)

### Discovery Source
- Agent 3: Missing required properties analysis
- Found 200 Slugification instances without created_at/updated_at

### Current State

**slugification.yaml BLOC 4 - REQUIRES TIMESTAMPS**:
```yaml
standard_properties:
  key:
    type: string
    required: true
  display_name:
    type: string
    required: true
  description:
    type: string
    required: true
  created_at:
    type: datetime
    required: true  # ← REQUIRED
  updated_at:
    type: datetime
    required: true  # ← REQUIRED
```

**Seed file**: `packages/db/seed/22-slugification.cypher` - MISSING TIMESTAMPS:
```cypher
// CURRENT (VIOLATION)
CREATE (s:Slugification {
  key: "latin_preserve",
  rule_name: "Preserve Latin Diacritics",
  // ❌ MISSING: created_at and updated_at
})

// Create 200 more instances...
```

### Impact
- **200 instances** affected
- Violates schema-standard.md rule line 219: `TIMESTAMP_REQUIRED: All nodes must have created_at and updated_at`
- Breaks ADR-024 trait system (all nodes need timestamps for provenance)

### Solution

**Step 1: Update seed file generator**

All Slugification CREATE statements must include:
```cypher
CREATE (s:Slugification {
  key: "latin_preserve",
  display_name: "Preserve Latin Diacritics",
  description: "Keep diacritics for Romance languages (é, ñ, ü, etc.)",
  rule_name: "Preserve Latin Diacritics",

  // ✅ ADD THESE
  created_at: datetime(),
  updated_at: datetime(),

  // ... other properties
})
```

**Step 2: Migration Cypher for existing instances**

```cypher
// Add missing timestamps to all Slugification nodes
MATCH (s:Slugification)
WHERE s.created_at IS NULL OR s.updated_at IS NULL
SET s.created_at = COALESCE(s.created_at, datetime()),
    s.updated_at = COALESCE(s.updated_at, datetime())
RETURN count(s) AS updated_count
```

**Step 3: Validation rule**

```cypher
// Verify all Slugification nodes have timestamps
MATCH (s:Slugification)
WHERE s.created_at IS NULL OR s.updated_at IS NULL
RETURN s.key AS missing_timestamps
```

### Auto-fix Capability
✅ **YES** - TimestampFixer from ADR-033 can handle this:

```rust
// From tools/novanet/src/validation/autofix/timestamp.rs
impl AutoFix for TimestampFixer {
    fn fix(&self, node: &mut ParsedNode, issue: &SchemaIssue) -> Result<FixAction> {
        // Adds created_at and updated_at with datetime type
    }
}
```

Execution:
```bash
cargo run -- schema validate --fix
# TimestampFixer will automatically add created_at/updated_at to YAML
# Then regenerate seed file:
cargo run -- schema generate
# Migration Cypher will be generated in seed/migrations/
```

---

## VIOLATION #4: Brand Missing brand_name (CRITIQUE)

### Discovery Source
- Agent 3: Missing required properties analysis

### Current State

**brand.yaml BLOC 4 - REQUIRES brand_name**:
```yaml
properties:
  brand_name:
    type: string
    required: true  # ← REQUIRED
    description: "Official brand name"
    example: "QR Code AI"
```

**Seed file**: `packages/db/seed/31-project-qrcode-ai.cypher` - NEEDS VERIFICATION:
```cypher
// IF THIS EXISTS (VIOLATION):
CREATE (brand:Brand {
  key: "brand-qrcode-ai",
  // ❌ MISSING: brand_name (required: true)
})

// SHOULD BE:
CREATE (brand:Brand {
  key: "brand-qrcode-ai",
  brand_name: "QR Code AI",  // ✅ REQUIRED
  // ...
})
```

### Impact
- **1 instance** (brand-qrcode-ai) - IF missing
- Violates required property constraint
- Breaks validation rules

### Solution

**Step 1: Verify current state**

```cypher
// Check if brand_name exists
MATCH (b:Brand {key: "brand-qrcode-ai"})
RETURN b.brand_name AS brand_name
```

**Step 2a: If missing, add via migration**

```cypher
MATCH (b:Brand {key: "brand-qrcode-ai"})
WHERE b.brand_name IS NULL
SET b.brand_name = "QR Code AI"
```

**Step 2b: Update seed file**

Ensure 31-project-qrcode-ai.cypher includes brand_name in CREATE statement.

**Step 3: Validation rule**

```cypher
// All Brand nodes must have brand_name
MATCH (b:Brand)
WHERE b.brand_name IS NULL
RETURN b.key AS missing_brand_name
```

### Auto-fix Capability
❌ **PARTIAL** - Auto-fix can detect missing required property but CANNOT infer correct value:
- Detection: YES (validation can flag missing brand_name)
- Fix: NO (requires human to provide correct brand name value)

---

## VIOLATION #5: Property Order Mismatch (MOYEN)

### Discovery Source
- User Message 5: "Et aussi les properties et leurs odre doivent etre les meme entre la node def et la node isntance"

### Current State

**brand.yaml BLOC 4 - CANONICAL ORDER**:
```yaml
standard_properties:
  key:            # Position 1
  display_name:   # Position 2
  description:    # Position 3
  created_at:     # Position 4
  updated_at:     # Position 5

properties:
  brand_name:     # Position 6
  tagline:        # Position 7
  brand_story:    # Position 8
  # ...
```

**Neo4j instances - ARBITRARY ORDER**:
```cypher
// Current seed files create properties in random order
CREATE (b:Brand {
  tagline: "...",         // ❌ Position 7 listed first
  key: "brand-...",       // ❌ Position 1 listed second
  brand_name: "...",      // ❌ Position 6 listed third
  description: "...",     // ❌ Position 3 listed fourth
  created_at: datetime()  // ❌ Position 4 listed fifth
})
```

### Impact
- **All seed files** (potentially)
- Visual inconsistency between YAML schema and Neo4j Browser
- Makes debugging harder (properties not in expected order)
- Not a functional violation (Neo4j stores as unordered map) but UX issue

### Why This Matters

From schema-standard.md lines 163-172:
```markdown
## Standard Properties Order

For ALL nodes:
1. `key` (if node has identity)
2. `*_key` denormalized properties (for composite keys)
3. `display_name`
4. `description`
5. `created_at`
6. `updated_at`
```

User expectation: "les properties et leurs odre doivent etre les meme entre la node def et la node isntance"

### Solution

**Step 1: Update seed file generators**

All generators that emit Cypher CREATE statements must:
1. Read property order from YAML
2. Emit SET clauses or property map in YAML order

Example pseudocode:
```rust
// In seed file generator
fn generate_create_statement(node: &NodeClass, data: &NodeData) -> String {
    let mut props = Vec::new();

    // 1. Standard properties first (in canonical order)
    for prop in &node.standard_properties_ordered() {
        if let Some(value) = data.get(prop.name) {
            props.push(format!("{}: {}", prop.name, value));
        }
    }

    // 2. Custom properties next (in YAML order)
    for prop in &node.properties_ordered() {
        if let Some(value) = data.get(prop.name) {
            props.push(format!("{}: {}", prop.name, value));
        }
    }

    format!("CREATE (n:NodeLabel {{ {} }})", props.join(", "))
}
```

**Step 2: Migration for existing instances**

Cypher doesn't preserve property order in storage, but we can ensure CONSISTENT order in seed files for future deploys.

No migration needed for existing instances (Neo4j stores as map anyway).

**Step 3: Validation rule**

Validate seed files, not Neo4j instances:
```bash
# Check that generated seed files have properties in YAML order
cargo run -- schema validate --check-seed-order
```

### Auto-fix Capability
✅ **YES** - Seed file generator can be updated to emit properties in YAML order:
1. Read YAML property order
2. Emit Cypher in same order
3. No database migration needed (only affects future seed file generation)

---

## Research Synthesis

### Perplexity Query 1: LLM context caching patterns

**Result**: No specific findings on schema vs instance distinction
- Generic caching: prefix matching, hierarchical caching
- Not directly applicable to our dual llm_context problem

### Perplexity Query 2: Knowledge graph node properties best practices

**Key findings**:
```
✅ NODE PROPERTIES SHOULD INCLUDE:
1. name, type, timestamp (already have)
2. confidence scores (consider for llm_context)
3. provenance (data source origin) (consider for llm_context)

✅ METADATA FOR LLM PROMPTS:
- Use indexed, validated properties
- Limit to type + relevance scores
- Focus on 2-4 key characteristics
- Prioritize 2-4 hop relationships for reasoning

✅ RELEVANCE TO llm_context:
- Should be: string type (indexable)
- Should be: optional (not all instances need it)
- Should be: focused (2-4 key points)
- Should include: confidence/provenance where applicable
```

**Application to VIOLATION #1**:
```yaml
llm_context:
  type: string  # ✅ Indexable, simple
  required: false  # ✅ Not all instances need custom context
  description: |
    Instance-specific LLM generation context.

    Best practices:
    - Keep focused (2-4 key characteristics)  # ✅ From research
    - Include confidence/provenance if relevant  # ✅ From research
    - Use for filtering and relevance scoring  # ✅ From research
```

### Context7 Query: FAILED (quota exceeded)

Not applicable to solution.

### Skills Analysis

**brainstorming/SKILL.md** structure:
```markdown
---
name: brainstorming
description: ...
disable-model-invocation: false
---

# Brainstorming Ideas Into Designs  # ← Like BLOC 2 (schema metadata)

## Overview                          # ← Like BLOC 4 (content)
## Terminal Output (ASCII)
## The Process
```

**writing-plans/SKILL.md** structure:
```markdown
---
name: writing-plans
description: ...
---

# Writing Implementation Plans  # ← Schema metadata

## When to Use This Skill      # ← Content sections
## What Makes a Good Plan
```

**Pattern identified**: Skills use dual structure:
1. YAML frontmatter (schema metadata)
2. Markdown sections (content)

**Parallel to NovaNet**:
1. BLOC 2 llm_context (schema metadata - how to use)
2. BLOC 4 llm_context property (instance content - specific to this node)

### Grep Analysis: 61 Files with llm_context

Confirms widespread use of llm_context across:
- All org/ nodes (Brand, Project, Entity, Page, Block, etc.)
- All shared/ nodes (Locale, EntityCategory, SEOKeyword, etc.)
- All instruction nodes (BlockType, BlockInstruction, PromptArtifact)
- All output nodes (PageNative, BlockNative, OutputArtifact)

**Implication**: If we add llm_context as BLOC 4 property, affects 61 YAMLs.

---

## Recommendations Summary

### Priority 1: CRITIQUE Violations

1. **llm_context BLOC 4 Property** (VIOLATION #1)
   - Action: Add to schema-standard.md as valid dual pattern
   - Action: Add property to all 61 affected YAMLs
   - Auto-fix: NO (requires design decision)
   - Estimated effort: 8-12 hours (61 YAMLs + documentation)

2. **Brand/BrandDesign Split** (VIOLATION #2)
   - Action: Create BrandDesign node in seed file
   - Action: Create [:HAS_DESIGN] relationship
   - Action: Remove 13 design properties from Brand
   - Auto-fix: NO (requires data migration)
   - Estimated effort: 2-3 hours (1 seed file + migration)

3. **Slugification Timestamps** (VIOLATION #3)
   - Action: Update seed file generator to include timestamps
   - Action: Generate migration Cypher for 200 instances
   - Auto-fix: YES (TimestampFixer)
   - Estimated effort: 1-2 hours (auto-fix + testing)

4. **Brand brand_name** (VIOLATION #4)
   - Action: Verify if actually missing
   - Action: Update seed file if needed
   - Auto-fix: PARTIAL (detection only, not value inference)
   - Estimated effort: 30 minutes (verification + fix)

### Priority 2: MOYEN Violations

5. **Property Order** (VIOLATION #5)
   - Action: Update seed generators to emit in YAML order
   - Auto-fix: YES (generator update)
   - Estimated effort: 3-4 hours (all generators)

### Next Steps

1. **Immediate**: Create coherence_check.rs module
2. **Short-term**: Implement auto-fixes for #3 and #5
3. **Medium-term**: Manual fixes for #1, #2, #4
4. **Long-term**: Integrate coherence check into CI/CD

---

## Proposed coherence_check.rs Architecture

```rust
// tools/novanet/src/validation/coherence_check.rs

pub struct CoherenceChecker {
    yaml_schemas: HashMap<String, NodeClass>,
    neo4j_instances: HashMap<String, NodeInstance>,
}

pub enum CoherenceViolation {
    MissingProperty {
        node_key: String,
        property_name: String,
        required: bool,
    },
    UndeclaredProperty {
        node_key: String,
        property_name: String,
        property_value: serde_json::Value,
    },
    PropertyOrderMismatch {
        node_key: String,
        yaml_order: Vec<String>,
        instance_order: Vec<String>,
    },
    TypeMismatch {
        node_key: String,
        property_name: String,
        yaml_type: String,
        instance_type: String,
    },
}

impl CoherenceChecker {
    pub fn check_all(&self) -> Vec<CoherenceViolation> {
        // 1. For each NodeClass in YAML
        // 2. Query all instances of that class from Neo4j
        // 3. Compare properties
        // 4. Return violations
    }

    pub fn check_node(&self, node_key: &str) -> Vec<CoherenceViolation> {
        // Check single instance
    }

    pub fn auto_fix(&self, violations: &[CoherenceViolation]) -> Result<FixReport> {
        // Apply auto-fixes where possible
        // Return report of what was fixed
    }
}
```

**Integration**:
```bash
cargo run -- schema coherence --check-all
cargo run -- schema coherence --check-node=brand-qrcode-ai
cargo run -- schema coherence --fix --aggressive
```
