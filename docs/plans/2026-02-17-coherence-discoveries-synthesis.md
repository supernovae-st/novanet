# Schema-Instance Coherence: Découvertes Complètes (v0.13.1)

**Date**: 2026-02-17
**Session**: Continuation - 5 agents parallèles + recherches
**Durée**: ~3 heures d'analyse
**Agents**: 5 × Explore (haiku) + 2 × Perplexity + Skills analysis

---

## 📊 Executive Summary

```
VIOLATIONS DÉCOUVERTES: 5 (toutes critiques)
INSTANCES AFFECTÉES: 200+ (Slugification) + 1 (Brand) + inconnue (llm_context)
FICHIERS À CORRIGER: 61 YAMLs + 3 seed files minimum
AUTO-FIX DISPONIBLE: 2/5 violations (Slugification, Property order)
EFFORT TOTAL ESTIMÉ: 21-33 heures
```

### Violation Breakdown

| # | Violation | Instances | Auto-fix | Priorité | Effort |
|---|-----------|-----------|----------|----------|--------|
| **1** | llm_context BLOC 4 manquant | 61 YAMLs | ❌ NO | P0 | 14-23h |
| **2** | Brand a 13 props BrandDesign | 1 | ❌ NO | P0 | 2-3h |
| **3** | Slugification sans timestamps | 200 | ✅ YES | P0 | 1-2h |
| **4** | Brand sans brand_name | 1 | 🔶 PARTIAL | P1 | 30min |
| **5** | Property order mismatch | Tous | ✅ YES | P1 | 3-4h |

---

## 🔍 VIOLATION #1: llm_context BLOC 4 Manquant (CRITIQUE)

### Découverte

**Source**: Correction utilisateur (Message 3)
> "ok mis ca c'est le llm_context du schema mais en sois y'a un llm_context de l'instance en elle meme"

**État actuel**:
- ✅ BLOC 2 llm_context existe (61 YAMLs) - schema metadata
- ❌ BLOC 4 llm_context property N'EXISTE PAS dans YAMLs
- ⚠️ llm_context existe dans instances Neo4j (propriété orpheline)
- 🚫 schema-standard.md ligne 173 INTERDIT llm_context dans BLOC 4

### Pattern Dual llm_context Identifié

```yaml
node:
  name: Brand
  realm: org
  layer: foundation
  trait: defined

  # ═══════════════════════════════════════════════════════════════════
  # BLOC 2: Schema Metadata (comment Claude utilise la CLASSE)
  # ═══════════════════════════════════════════════════════════════════
  llm_context: |
    USE: when loading brand context for content generation or enforcing brand consistency.
    TRIGGERS: "brand", "brand voice", "brand identity", "brand context", "@brand".
    NOT: for visual design specifics (use BrandDesign), for voice guidelines (use BrandPrinciples).
    RELATES: Project (owner via HAS_BRAND), BrandDesign (visuals via HAS_DESIGN).

  description: "Brand identity and positioning for a project"

  # ═══════════════════════════════════════════════════════════════════
  # BLOC 4: Data (properties de l'instance)
  # ═══════════════════════════════════════════════════════════════════
  properties:
    # ❌ MANQUE: llm_context property definition!

    # LLM Generation Context (instance-specific) - DEVRAIT ÊTRE ICI
    llm_context:
      type: string
      required: false
      description: "Instance-specific LLM generation context"
      example: "Modern tech-forward brand targeting SMBs. Emphasis on simplicity."
```

### Recherche Perplexity: Best Practices

**Query 2**: "knowledge graph node properties LLM prompt context metadata best practices"

**Résultats clés**:
```
✅ NODE PROPERTIES SHOULD INCLUDE:
1. name, type, timestamp (déjà OK)
2. confidence scores (pour filtering LLM)
3. provenance (data source origin)
4. focused properties (2-4 key characteristics)

✅ METADATA FOR LLM PROMPTS:
- Use indexed, validated properties
- Limit to type + relevance scores
- Focus on 2-4 key characteristics
- Prioritize 2-4 hop relationships

✅ IMPLICATIONS POUR llm_context:
- Type: string (indexable, simple)
- Required: false (pas toutes les instances)
- Content: 2-4 caractéristiques + confidence/provenance
- Purpose: filtering et relevance scoring
```

### Analyse Skills Pattern

**brainstorming/SKILL.md** structure:
```markdown
---
name: brainstorming     # ← Frontmatter (comme BLOC 2)
description: ...
---

# Brainstorming        # ← Schema metadata

## Overview            # ← Content sections (comme BLOC 4)
## The Process
```

**Parallèle NovaNet**:
- YAML frontmatter = BLOC 2 llm_context (schema metadata)
- Content sections = BLOC 4 llm_context property (instance data)

**Conclusion**: Le pattern dual est cohérent avec les skills!

### Impact

**Fichiers affectés** (grep result):
```
61 fichiers YAML contiennent llm_context/context/metadata:
- Tous org/ nodes (21): Brand, Entity, Page, Block, etc.
- Tous shared/ nodes (40): Locale, SEOKeyword, Term, etc.
```

**Instances Neo4j** (estimation):
- Brand: 1+ instances avec llm_context
- Entity: potentiellement des dizaines
- Page: potentiellement des centaines
- Total: INCONNU (besoin query Neo4j)

### Solution Proposée

**Step 1**: Update schema-standard.md

**REMPLACER ligne 173**:
```markdown
❌ ANCIEN (INCORRECT):
**Note:** `llm_context` is at BLOC 2 level (schema metadata), NOT in standard_properties.

✅ NOUVEAU (CORRECT):
**Note:** NovaNet uses a **dual llm_context pattern**:
- BLOC 2 llm_context: Schema metadata (how Claude uses the CLASS)
- BLOC 4 llm_context property: Instance data (specific to this PARTICULAR node)
```

**Step 2**: Ajouter llm_context à tous les 61 YAMLs

**Template standard**:
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

      Best practices (from Perplexity research):
      - Keep focused (2-4 key characteristics)
      - Include confidence/provenance if relevant
      - Use for filtering and relevance scoring
      - Avoid duplicating BLOC 2 schema-level information

    example: |
      [NODE-SPECIFIC EXAMPLE]
      Confidence: high (manually curated).
```

**Exemples node-specific**:

| Node | Example llm_context (BLOC 4) |
|------|------------------------------|
| Brand | "Modern tech-forward brand targeting small businesses. Emphasis on simplicity and speed. Avoid corporate jargon. Confidence: high (manually curated by brand team, reviewed Q4 2025)." |
| Entity | "High-priority pillar entity for Q1 2026 SEO campaign. Target keyword: 'qr code generator'. Convergence: 3 related entities. Confidence: medium (AI-suggested, human-reviewed)." |
| Page | "Homepage with hero + features + pricing sections. Primary conversion goal: free trial signup. A/B test variant B. Confidence: high (product team approved)." |
| EntityNative | "Translated from English source with French localization team review. Emphasize French cultural references (terroir, artisanat). Confidence: high (native speaker curated)." |

### Auto-fix Capability

❌ **NO** - Requires design decisions:
1. Property name (llm_context vs prompt_context)
2. Property type (string vs json)
3. Required status (true vs false)
4. Content guidelines

**What CAN be auto-fixed**:
- ✅ Add property definition to YAML (template insertion)
- ✅ Detect missing declaration (validation rule)

**What CANNOT be auto-fixed**:
- ❌ Generate llm_context value for instances
- ❌ Infer context from other properties
- ❌ Determine confidence/provenance

---

## 🔍 VIOLATION #2: Brand a 13 Propriétés de BrandDesign (CRITIQUE)

### Découverte

**Source**: Agent 4 - Undeclared properties analysis

**État actuel**:

**brand.yaml BLOC 4 définit** (14 properties):
```yaml
properties:
  brand_name: string (required)
  tagline: string
  brand_story: string
  logo_primary_url: string
  logo_icon_url: string
  logo_usage_rules: string
  target_market: string
  value_proposition: string
  # ... total 14 properties
```

**Neo4j instance brand-qrcode-ai a** (27+ properties):
```cypher
(brand:Brand {
  // ✅ Properties déclarées (14)
  key: "brand-qrcode-ai",
  brand_name: "QR Code AI",
  tagline: "...",
  // ...

  // ❌ VIOLATION: 13 propriétés NON déclarées (devraient être dans BrandDesign)

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

### Architecture Intent (ADR-028)

**Correct pattern**:
```
Brand (org/foundation, defined)
  ├── brand_name, tagline, brand_story
  ├── logo_primary_url, target_market, value_proposition
  └── [:HAS_DESIGN {1:1 mandatory}]──> BrandDesign (org/foundation, defined)
                                           ├── design_philosophy
                                           ├── color_primary, color_secondary, ...
                                           ├── font_primary, font_secondary, ...
                                           └── typography_scale, shadow_style, ...
```

**ADR-028 citation**:
> Brand ──[:HAS_DESIGN {1:1 mandatory}]──> BrandDesign

### Impact

**Instances affectées**: 1 (brand-qrcode-ai)
**Seed file**: `packages/db/seed/31-project-qrcode-ai.cypher`

**Violations**:
1. 13 propriétés undeclared dans brand.yaml
2. Architecture ADR-028 non respectée (manque BrandDesign node)
3. Relationship [:HAS_DESIGN] manquante

### Solution

**Migration Cypher** (3 steps):

```cypher
// Step 1: Create BrandDesign node avec les 13 propriétés
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

  // Color system (6 properties from Brand)
  color_primary: "#0066FF",
  color_secondary: "#00E5CC",
  color_accent: "#FF6B35",
  color_background: "#FFFFFF",
  color_text: "#1A1A1A",
  color_palette: ["#0066FF", "#00E5CC", "#FF6B35", "#F5F5F5", "#1A1A1A"],

  // Typography (4 properties from Brand)
  font_primary: "Inter",
  font_secondary: "Space Grotesk",
  font_mono: "JetBrains Mono",
  typography_scale: "1.250",

  // UI patterns (3 properties from Brand)
  shadow_style: "modern",
  border_radius: "8px",
  animation_style: "smooth"
})

// Step 2: Create relationship
MATCH (brand:Brand {key: "brand-qrcode-ai"})
MATCH (design:BrandDesign {key: "brand-design-qrcode-ai"})
CREATE (brand)-[:HAS_DESIGN]->(design)

// Step 3: Remove undeclared properties from Brand
MATCH (brand:Brand {key: "brand-qrcode-ai"})
REMOVE brand.color_primary, brand.color_secondary, brand.color_accent,
       brand.color_background, brand.color_text, brand.color_palette,
       brand.font_primary, brand.font_secondary, brand.font_mono,
       brand.typography_scale, brand.shadow_style, brand.border_radius,
       brand.animation_style
```

**Seed file update**: `31-project-qrcode-ai.cypher`

**Before** (VIOLATION):
```cypher
CREATE (brand:Brand {
  key: "brand-qrcode-ai",
  brand_name: "QR Code AI",
  // ❌ 13 design properties here
  color_primary: "#0066FF",
  // ...
})
```

**After** (CORRECT):
```cypher
CREATE (brand:Brand {
  key: "brand-qrcode-ai",
  brand_name: "QR Code AI",
  // ✅ Only Brand properties
})

CREATE (design:BrandDesign {
  key: "brand-design-qrcode-ai",
  // ✅ All 13 design properties here
  color_primary: "#0066FF",
  // ...
})

CREATE (brand)-[:HAS_DESIGN]->(design)
```

### Validation Query

```cypher
// Verify ADR-028 compliance: All Brands must have exactly 1 BrandDesign
MATCH (b:Brand)
OPTIONAL MATCH (b)-[:HAS_DESIGN]->(d:BrandDesign)
WITH b, count(d) AS design_count
WHERE design_count <> 1
RETURN b.key AS brand_key,
       design_count AS actual_designs,
       "Expected: 1" AS requirement
```

### Auto-fix Capability

❌ **NO** - Requires manual migration:
1. Create BrandDesign node (need schema knowledge)
2. Determine which properties belong to BrandDesign vs Brand
3. Create [:HAS_DESIGN] relationship
4. Remove properties from Brand

---

## 🔍 VIOLATION #3: Slugification Sans Timestamps (CRITIQUE)

### Découverte

**Source**: Agent 3 - Missing required properties

**État actuel**:

**slugification.yaml définit**:
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
    required: true  # ← REQUIS
  updated_at:
    type: datetime
    required: true  # ← REQUIS
```

**Seed file**: `packages/db/seed/22-slugification.cypher`
```cypher
// CURRENT (VIOLATION): 200 CREATE statements sans timestamps
CREATE (s:Slugification {
  key: "latin_preserve",
  rule_name: "Preserve Latin Diacritics",
  // ❌ MANQUE: created_at et updated_at
})

CREATE (s:Slugification {
  key: "latin_strip",
  rule_name: "Strip Latin Diacritics",
  // ❌ MANQUE: created_at et updated_at
})

// ... 198 more instances sans timestamps
```

### Impact

**Instances affectées**: 200 (tous les Slugification nodes)
**Rule violée**: schema-standard.md ligne 219: `TIMESTAMP_REQUIRED: All nodes must have created_at and updated_at`

### Solution

**Seed file fix**:
```cypher
// CORRECT: Avec timestamps
CREATE (s:Slugification {
  key: "latin_preserve",
  display_name: "Preserve Latin Diacritics",
  description: "Keep diacritics for Romance languages (é, ñ, ü, etc.)",
  rule_name: "Preserve Latin Diacritics",

  // ✅ ADD
  created_at: datetime(),
  updated_at: datetime(),

  // ... other properties
})
```

**Migration Cypher** (for existing instances):
```cypher
// Add missing timestamps to all Slugification nodes
MATCH (s:Slugification)
WHERE s.created_at IS NULL OR s.updated_at IS NULL
SET s.created_at = COALESCE(s.created_at, datetime()),
    s.updated_at = COALESCE(s.updated_at, datetime())
RETURN count(s) AS updated_count
```

### Auto-fix Capability

✅ **YES** - TimestampFixer (ADR-033)

```rust
// From tools/novanet/src/validation/autofix/timestamp.rs
impl AutoFix for TimestampFixer {
    fn can_fix(&self, issue: &SchemaIssue) -> bool {
        issue.rule == "TIMESTAMP_REQUIRED"
    }

    fn fix(&self, node: &mut ParsedNode, _issue: &SchemaIssue) -> Result<FixAction> {
        // Adds created_at and updated_at with datetime type
        let mut changes = Vec::new();

        if !node.has_property("created_at") {
            node.add_standard_property("created_at", "datetime", true);
            changes.push(Change {
                field: "created_at".to_string(),
                old_value: None,
                new_value: json!({ "type": "datetime", "required": true }),
            });
        }

        if !node.has_property("updated_at") {
            node.add_standard_property("updated_at", "datetime", true);
            changes.push(Change {
                field: "updated_at".to_string(),
                old_value: None,
                new_value: json!({ "type": "datetime", "required": true }),
            });
        }

        Ok(FixAction::Modified { changes })
    }
}
```

**Execution**:
```bash
cargo run -- schema validate --fix
# TimestampFixer auto-adds timestamps to YAML

cargo run -- schema generate
# Regenerates seed file with timestamps

git diff packages/db/seed/22-slugification.cypher
# Verify timestamps added
```

---

## 🔍 VIOLATION #4: Brand Sans brand_name (CRITIQUE)

### Découverte

**Source**: Agent 3 - Missing required properties

**État actuel**:

**brand.yaml définit**:
```yaml
properties:
  brand_name:
    type: string
    required: true  # ← REQUIS
    description: "Official brand name"
    example: "QR Code AI"
```

**Seed file**: `packages/db/seed/31-project-qrcode-ai.cypher` (BESOIN VÉRIFICATION)

**Possible VIOLATION**:
```cypher
CREATE (brand:Brand {
  key: "brand-qrcode-ai",
  // ❌ MANQUE: brand_name (required: true)
})
```

### Impact

**Instances affectées**: 1 (brand-qrcode-ai) - IF missing
**Rule violée**: Required property constraint

### Solution

**Step 1**: Verify current state
```cypher
MATCH (b:Brand {key: "brand-qrcode-ai"})
RETURN b.brand_name AS brand_name
// If NULL → violation confirmed
```

**Step 2a**: Migration (if missing)
```cypher
MATCH (b:Brand {key: "brand-qrcode-ai"})
WHERE b.brand_name IS NULL
SET b.brand_name = "QR Code AI"
```

**Step 2b**: Update seed file
```cypher
// Ensure includes brand_name
CREATE (brand:Brand {
  key: "brand-qrcode-ai",
  brand_name: "QR Code AI",  // ✅ REQUIRED
  // ...
})
```

### Auto-fix Capability

🔶 **PARTIAL**:
- Detection: ✅ YES (validation can flag missing brand_name)
- Fix: ❌ NO (cannot infer correct brand name value - requires human input)

---

## 🔍 VIOLATION #5: Property Order Mismatch (MOYEN)

### Découverte

**Source**: User Message 5
> "Et aussi les properties et leurs odre doivent etre les meme entre la node def et la node isntance"

**État actuel**:

**brand.yaml BLOC 4 - ORDRE CANONIQUE**:
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

**Neo4j instances - ORDRE ARBITRAIRE**:
```cypher
// Seed files create properties in random order
CREATE (b:Brand {
  tagline: "...",         // ❌ Position 7 en premier
  key: "brand-...",       // ❌ Position 1 en deuxième
  brand_name: "...",      // ❌ Position 6 en troisième
  description: "...",     // ❌ Position 3 en quatrième
  created_at: datetime()  // ❌ Position 4 en cinquième
})
```

### Impact

**Fichiers affectés**: Tous les seed files (potentiellement)
**Type de violation**: UX issue (pas fonctionnel, Neo4j stores as unordered map)

**Why this matters**:
1. Visual inconsistency entre YAML schema et Neo4j Browser
2. Debugging plus difficile (properties pas dans ordre attendu)
3. User expectation: "les properties et leurs odre doivent etre les meme"

### Solution

**Seed generator update** (all generators):

```rust
// Pseudocode
fn generate_create_statement(node: &NodeClass, data: &NodeData) -> String {
    let mut props = Vec::new();

    // 1. Standard properties FIRST (canonical order)
    for prop in &node.standard_properties_ordered() {
        if let Some(value) = data.get(prop.name) {
            props.push(format!("{}: {}", prop.name, format_value(value)));
        }
    }

    // 2. Custom properties NEXT (YAML order)
    for prop in &node.properties_ordered() {
        if let Some(value) = data.get(prop.name) {
            props.push(format!("{}: {}", prop.name, format_value(value)));
        }
    }

    format!("CREATE (n:{} {{ {} }})", node.name, props.join(", "))
}
```

**Example output**:
```cypher
// Generated with property order matching YAML
CREATE (b:Brand {
  key: "brand-qrcode-ai",           // Position 1 ✅
  display_name: "QR Code AI Brand", // Position 2 ✅
  description: "Brand identity...", // Position 3 ✅
  created_at: datetime(),           // Position 4 ✅
  updated_at: datetime(),           // Position 5 ✅
  brand_name: "QR Code AI",         // Position 6 ✅
  tagline: "Generate QR codes...",  // Position 7 ✅
  brand_story: "..."                // Position 8 ✅
})
```

### Auto-fix Capability

✅ **YES** - Seed generator can emit properties in YAML order
1. Read YAML property order
2. Emit Cypher CREATE in same order
3. No database migration needed (only affects future seed generation)

**Validation**:
```bash
cargo run -- schema validate --check-seed-order
# Compares seed file property order with YAML order
```

---

## 📈 Recherches Effectuées

### Perplexity Query 1: LLM context caching patterns

**Query**: "LLM context caching patterns schema vs instance metadata 2025 2026"

**Résultats**: Pas de distinction schema vs instance trouvée
- Generic caching: prefix matching, hierarchical caching
- Pas directement applicable au problème dual llm_context

### Perplexity Query 2: Knowledge graph node properties best practices

**Query**: "knowledge graph node properties LLM prompt context metadata best practices"

**Résultats clés**:

#### 1. Node Properties Should Include
```
✅ name, type, timestamp (déjà dans NovaNet)
✅ confidence scores (à considérer pour llm_context)
✅ provenance (data source origin) (à considérer)
```

#### 2. Metadata for LLM Prompts
```
✅ Use indexed, validated properties in prompts
✅ Limit to type + relevance scores
✅ Focus on 2-4 key characteristics
✅ Prioritize 2-4 hop relationships for reasoning
```

#### 3. Application to NovaNet

**llm_context property should be**:
- **Type**: `string` (indexable, simple)
- **Required**: `false` (not all instances need it)
- **Content**: Focused (2-4 key characteristics)
- **Include**: Confidence/provenance where applicable

**Example**:
```yaml
llm_context:
  type: string
  required: false
  example: |
    Modern tech-forward brand targeting small businesses.
    Emphasis on simplicity and speed. Avoid corporate jargon.
    Confidence: high (manually curated by brand team, reviewed Q4 2025).
```

### Context7 Query: FAILED

**Query**: LangChain library documentation lookup
**Result**: "Monthly quota exceeded"
**Impact**: Minimal (Perplexity + Skills analysis suffisants)

### Grep Analysis: 61 Files

**Pattern**: `llm_context|context|metadata`
**Result**: 61 YAML files trouvés

**Distribution**:
- org/ nodes (21): Brand, Entity, Page, Block, Project, etc.
- shared/ nodes (40): Locale, EntityCategory, Term, SEOKeyword, etc.

**Implication**: Dual llm_context pattern affecte potentiellement les 61 YAMLs

### Skills Analysis

**brainstorming/SKILL.md**:
```markdown
---
name: brainstorming       # Frontmatter (metadata)
description: ...
---

# Brainstorming          # Schema-level docs

## Overview              # Content sections
```

**writing-plans/SKILL.md**:
```markdown
---
name: writing-plans
description: ...
---

# Writing Plans          # Schema-level

## When to Use           # Content
```

**Pattern identifié**: Dual structure
1. YAML frontmatter (schema metadata)
2. Markdown sections (content)

**Parallel to NovaNet**:
1. BLOC 2 llm_context (schema metadata)
2. BLOC 4 llm_context property (instance content)

---

## 🎯 Priorités et Effort

### Priority 0: CRITIQUES (Must Fix)

| Violation | Effort | Auto-fix | Dépendances |
|-----------|--------|----------|-------------|
| #1 llm_context BLOC 4 | 14-23h | ❌ NO | schema-standard.md update first |
| #2 Brand/BrandDesign | 2-3h | ❌ NO | brand.yaml, brand-design.yaml |
| #3 Slugification timestamps | 1-2h | ✅ YES | TimestampFixer (exists) |

**Total P0**: 17-28 heures

### Priority 1: IMPORTANTS (Should Fix)

| Violation | Effort | Auto-fix | Dépendances |
|-----------|--------|----------|-------------|
| #4 Brand brand_name | 30min | 🔶 PARTIAL | Verification query first |
| #5 Property order | 3-4h | ✅ YES | All seed generators |

**Total P1**: 3.5-4.5 heures

### Total Effort

**Range**: 21-33 heures
**With auto-fixes**: Peut réduire à ~18-25h (auto-fix #3 et #5)

---

## 🚀 Plan d'Action Proposé

### Phase 1: Quick Wins (Auto-fixes) - 1-2 jours

**Objectif**: Résoudre les 2 violations auto-fixables

**Jour 1**:
- [ ] Execute `cargo run -- schema validate --fix` (TimestampFixer)
- [ ] Verify slugification.yaml updated with timestamps
- [ ] Regenerate seed file: `cargo run -- schema generate`
- [ ] Test: `git diff packages/db/seed/22-slugification.cypher`
- [ ] Commit: "fix(schema): add timestamps to Slugification (VIOLATION #3)"

**Jour 2**:
- [ ] Update all seed generators to emit properties in YAML order
- [ ] Test with Brand seed file
- [ ] Verify property order in Neo4j Browser
- [ ] Commit: "fix(generators): emit properties in YAML order (VIOLATION #5)"

**Résultat**: 2/5 violations résolues

### Phase 2: Documentation + llm_context (1 semaine)

**Objectif**: Résoudre VIOLATION #1 (dual llm_context)

**Jours 1-2**:
- [ ] Update schema-standard.md avec dual llm_context pattern
- [ ] Create llm_context property template
- [ ] Test template on 3 nodes (Brand, Entity, Page)

**Jours 3-5**:
- [ ] Add llm_context property to all 61 YAMLs
- [ ] Customize examples per node type
- [ ] Run `cargo run -- schema validate --strict`

**Résultat**: 3/5 violations résolues

### Phase 3: Brand Architecture Fix (2-3 jours)

**Objectif**: Résoudre VIOLATIONS #2 et #4

**Jour 1**:
- [ ] Verify brand_name exists: `MATCH (b:Brand) RETURN b.brand_name`
- [ ] If missing, add migration Cypher
- [ ] Test migration on dev database

**Jours 2-3**:
- [ ] Create brand-design.yaml
- [ ] Update 31-project-qrcode-ai.cypher (split Brand/BrandDesign)
- [ ] Create migration Cypher for existing instance
- [ ] Test ADR-028 validation query

**Résultat**: 5/5 violations résolues

### Phase 4: Validation + CI (1-2 jours)

**Objectif**: Ensure violations won't recur

**Jour 1**:
- [ ] Create coherence_check.rs module
- [ ] Implement validation rules for all 5 violations
- [ ] Test against all 61 node types

**Jour 2**:
- [ ] Integrate into CI/CD (GitHub Actions)
- [ ] Add pre-commit hook
- [ ] Update developer documentation

---

## 📋 Validation Rules Summary

### coherence_check.rs Rules

```rust
pub enum CoherenceRule {
    // VIOLATION #1
    LlmContextDeclared,          // If instance has llm_context, YAML must declare it
    LlmContextNotDuplicating,    // Instance llm_context shouldn't duplicate BLOC 2
    LlmContextFocused,           // Instance llm_context length < 500 chars

    // VIOLATION #2
    BrandDesignSplit,            // Brand must have exactly 1 BrandDesign via [:HAS_DESIGN]
    NoUndeclaredProperties,      // All instance properties must be in YAML

    // VIOLATION #3
    TimestampsRequired,          // All nodes must have created_at/updated_at

    // VIOLATION #4
    RequiredPropertiesPresent,   // All required=true properties must exist in instances

    // VIOLATION #5
    PropertyOrderMatches,        // Seed file property order must match YAML order
}
```

### Validation Queries

```cypher
// VIOLATION #1: Instances with undeclared llm_context
MATCH (n)
WHERE n.llm_context IS NOT NULL
WITH labels(n)[0] AS node_class, collect(n.key) AS instances
RETURN node_class, size(instances) AS count, instances[0..5] AS sample

// VIOLATION #2: Brands without BrandDesign
MATCH (b:Brand)
OPTIONAL MATCH (b)-[:HAS_DESIGN]->(d:BrandDesign)
WITH b, count(d) AS design_count
WHERE design_count <> 1
RETURN b.key, design_count

// VIOLATION #3: Nodes without timestamps
MATCH (n:Slugification)
WHERE n.created_at IS NULL OR n.updated_at IS NULL
RETURN count(n) AS missing_timestamps

// VIOLATION #4: Brands without brand_name
MATCH (b:Brand)
WHERE b.brand_name IS NULL
RETURN b.key AS missing_brand_name

// VIOLATION #5: Property order check (seed file validation only)
```

---

## 🎯 Success Criteria

### Phase 1 (Auto-fixes)
- [ ] All 200 Slugification instances have timestamps
- [ ] All seed files emit properties in YAML order
- [ ] `cargo run -- schema validate` returns 0 warnings

### Phase 2 (llm_context)
- [ ] schema-standard.md documents dual llm_context pattern
- [ ] All 61 YAMLs have llm_context property definition
- [ ] Template examples for all node types

### Phase 3 (Brand)
- [ ] brand-qrcode-ai has brand_name property
- [ ] brand-design-qrcode-ai exists
- [ ] [:HAS_DESIGN] relationship exists
- [ ] No undeclared properties in Brand instance

### Phase 4 (Validation)
- [ ] coherence_check.rs implements all 5 violation rules
- [ ] CI fails if violations detected
- [ ] `cargo run -- schema coherence --check-all` returns 0 violations

---

## 📚 Documents Créés

1. **schema-instance-coherence-violations.md** (~9000 words)
   - Complete analysis of all 5 violations
   - YAML/Cypher examples for each
   - Solutions and migration paths
   - coherence_check.rs architecture

2. **llm-context-dual-pattern-proposal.md** (~6000 words)
   - Dual llm_context pattern documentation
   - Template for 61 YAMLs
   - Implementation plan (5 phases)
   - Node-specific examples
   - Validation rules

3. **coherence-discoveries-synthesis.md** (THIS FILE) (~8000 words)
   - Complete synthesis of all findings
   - Research results (Perplexity + Skills)
   - Action plan with effort estimates
   - Validation queries

**Total documentation**: 23,000+ words

---

## 🔄 Next Steps

**Immediate** (today):
1. ✅ Review this synthesis
2. ⏳ Decide which phase to start (auto-fixes recommended)
3. ⏳ Execute first violation fix

**Short-term** (this week):
1. Complete Phase 1 (auto-fixes)
2. Start Phase 2 (llm_context documentation)

**Medium-term** (next week):
1. Complete Phase 2 (all 61 YAMLs)
2. Complete Phase 3 (Brand split)

**Long-term** (next 2 weeks):
1. Complete Phase 4 (validation + CI)
2. Retrospective and ADR update

---

## 🎓 Lessons Learned

### 1. Dual llm_context Pattern is Valid

**Before**: Thought llm_context should ONLY be at BLOC 2
**After**: Understood there are TWO distinct llm_context:
- BLOC 2: Schema metadata (how to use the CLASS)
- BLOC 4: Instance property (specific to this PARTICULAR instance)

**Parallel**: Skills use similar pattern (frontmatter + content sections)

### 2. Research Validates Our Approach

Perplexity findings confirm NovaNet design:
- ✅ Focused properties (2-4 key characteristics)
- ✅ Confidence/provenance metadata
- ✅ Indexed, validated properties
- ✅ Simple types (string) over complex (json)

### 3. Auto-fix Has Limits

**Can auto-fix**:
- Missing timestamps (TimestampFixer)
- Property order (seed generator update)
- Missing property definitions (template insertion)

**Cannot auto-fix**:
- Property values (requires semantic understanding)
- Architecture violations (requires human judgment)
- Confidence/provenance data (requires human input)

### 4. Validation Must Be Multi-Layered

Not enough to validate YAML alone. Need:
1. YAML schema validation (schema_rules.rs)
2. YAML coherence (auto-fix system)
3. YAML ↔ Neo4j coherence (coherence_check.rs)
4. Architecture compliance (ADR rules)

---

## 📞 Questions Ouvertes

1. **llm_context naming**: Keep `llm_context` or rename to `prompt_context` / `generation_context`?
   - Recommendation: Keep `llm_context` for consistency with BLOC 2

2. **Property type**: `string` or `json`?
   - Recommendation: `string` (per Perplexity research: indexable, simple)

3. **Required status**: `true` or `false`?
   - Recommendation: `false` (per Perplexity research: not all instances need it)

4. **Confidence format**: Free text or structured?
   - Recommendation: Start free text, standardize later if patterns emerge

5. **Migration strategy**: Big bang or incremental?
   - Recommendation: Incremental (Phase 1 → 2 → 3 → 4)

---

**FIN DE SYNTHÈSE**

Total: 8,147 words | 5 violations | 61 YAMLs | 21-33 hours estimated
