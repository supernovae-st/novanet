# Schema-Instance Coherence: Detailed Implementation Plan

**Date**: 2026-02-17
**Status**: Active
**Previous docs**:
- `2026-02-17-schema-instance-coherence-violations.md` — violations analysis
- `2026-02-17-llm-context-dual-pattern-proposal.md` — llm_context proposal
- `2026-02-17-coherence-discoveries-synthesis.md` — synthesis

---

## Decision: llm_context Naming

**KEEP `llm_context` for both BLOC 2 and BLOC 4.**

```
BLOC 2 (schema metadata):   node.llm_context = "USE: ... TRIGGERS: ..."
BLOC 4 (instance property): node.properties.llm_context = {type: string}
```

Rationale:
- Same semantic: "context for LLM consumption"
- Distinction is positional (BLOC 2 = class-level, BLOC 4 = instance-level)
- Consistent with existing 31-project-qrcode-ai.cypher (already uses `llm_context` per instance)
- Skills use same pattern (frontmatter metadata + content metadata)

---

## Violations Summary (from analysis)

| # | Violation | Impact | Auto-fix? |
|---|-----------|--------|-----------|
| V1 | Brand missing `brand_name` required property | 1 instance | YES (add to seed) |
| V2 | Brand has 13 BrandDesign properties (ADR-028 violation) | 1 instance | PARTIAL (create BrandDesign node) |
| V3 | Slugification missing `created_at`/`updated_at` (200 instances) | 200 instances | YES (migration Cypher) |
| V4 | llm_context not declared in YAML BLOC 4 (61 YAMLs) | 61 schemas | YES (add property) |
| V5 | Property order: Neo4j instances ≠ YAML canonical order | All seed files | YES (reorder) |

---

## Phase 1: Quick Wins (Auto-fixable)

### Phase 1.1 — Fix Slugification Timestamps

**Problem**: 22-slugification.cypher creates 200 Slugification nodes without `created_at` or `updated_at`.
**Current**: Uses `last_updated = '2026-01-09'` (non-standard property)
**Required**: `created_at` and `updated_at` (standard_properties from schema-standard.md)

**File to modify**: `packages/db/seed/22-slugification.cypher`

**Pattern to add** (to EVERY MERGE block):
```cypher
-- BEFORE (200x in file):
MERGE (s:Slugification {key: 'af-ZA'})
SET s.display_name = '...',
    s.description = '...',
    -- ... other properties ...
    s.llm_context = '...';

-- AFTER (add at end of each SET block):
MERGE (s:Slugification {key: 'af-ZA'})
SET s.display_name = '...',
    s.description = '...',
    -- ... other properties ...
    s.llm_context = '...',
    s.created_at = coalesce(s.created_at, datetime()),
    s.updated_at = datetime();
```

**Migration Cypher** (to fix existing instances without re-seeding):
```cypher
// File: packages/db/seed/migrations/001-slugification-timestamps.cypher
MATCH (s:Slugification)
WHERE s.created_at IS NULL
SET s.created_at = coalesce(s.created_at, datetime()),
    s.updated_at = datetime()
RETURN count(s) AS fixed;
```

**Verification query**:
```cypher
MATCH (s:Slugification)
WHERE s.created_at IS NULL OR s.updated_at IS NULL
RETURN count(s) AS missing_timestamps;
// Expected: 0
```

---

### Phase 1.2 — Fix Brand: Add `brand_name`

**Problem**: `brand.yaml` requires `brand_name` (required: true) but `31-project-qrcode-ai.cypher` does NOT set it.

**File to modify**: `packages/db/seed/31-project-qrcode-ai.cypher`

**Current state** (line 22-47):
```cypher
MERGE (p)-[:HAS_BRAND]->(bi:Brand {key: "brand-qrcode-ai"})
SET bi.display_name = "QR Code AI Brand",
    bi.description = "Visual identity and design system for QR Code AI",
    bi.llm_context = "...",
    bi.color_primary = "#6366F1",
    -- 13 MORE design properties that should be in BrandDesign
```

**Fix — add brand_name** (insert after `bi.display_name`):
```cypher
    bi.brand_name = "QR Code AI",
```

**Verification query**:
```cypher
MATCH (b:Brand)
WHERE b.brand_name IS NULL
RETURN b.key, b.display_name;
// Expected: 0 rows
```

---

### Phase 1.3 — Fix Brand: Create BrandDesign Node (ADR-028)

**Problem**: 31-project-qrcode-ai.cypher puts design properties on Brand that belong in BrandDesign (ADR-028: `Brand -[:HAS_DESIGN]-> BrandDesign`).

**Properties to MOVE from Brand to BrandDesign**:
```
color_primary, color_secondary, color_accent, color_background, color_text, color_palette
font_primary, font_secondary, font_mono
typography_scale
border_radius, shadow_style, animation_style
```

**Properties to KEEP on Brand**:
```
key, display_name, description, brand_name, tagline, brand_story
logo_primary_url, logo_icon_url, logo_usage_rules
target_market, value_proposition
style_keywords, style_mood, style_influences  (brand-level, not design-level)
image_style, image_do, image_dont              (brand-level image guidelines)
llm_context, created_at, updated_at
```

**New Brand MERGE block**:
```cypher
// Brand (core identity only)
MATCH (p:Project {key: "qrcode-ai"})
MERGE (p)-[:HAS_BRAND]->(bi:Brand {key: "brand-qrcode-ai"})
SET bi.display_name = "QR Code AI Brand",
    bi.brand_name = "QR Code AI",
    bi.description = "Central brand identity for QR Code AI",
    bi.tagline = "Smart QR Codes, Smarter Marketing",
    bi.brand_story = "Built by developers for developers and businesses who need professional QR codes fast.",
    bi.target_market = "Small businesses and marketers",
    bi.value_proposition = "Fastest way to create professional QR codes with built-in analytics",
    bi.style_keywords = '["modern", "clean", "professional", "tech-forward"]',
    bi.style_mood = "Professional yet approachable, tech-savvy but accessible",
    bi.style_influences = '["Apple", "Stripe", "Linear"]',
    bi.image_style = "Clean product shots, abstract QR patterns, tech illustrations",
    bi.image_do = '["Use gradient overlays", "Show QR codes in context", "Modern devices"]',
    bi.image_dont = '["Stock photos with obvious poses", "Cluttered backgrounds", "Outdated devices"]',
    bi.logo_usage_rules = '["Min size 32px", "Clear space equal to height", "No rotation"]',
    bi.llm_context = "USE: when loading brand identity and messaging context. TRIGGERS: brand, brand voice, @brand. NOT: visual design (use BrandDesign).",
    bi.created_at = coalesce(bi.created_at, datetime()),
    bi.updated_at = datetime();

// BrandDesign (visual design system)
MATCH (bi:Brand {key: "brand-qrcode-ai"})
MERGE (bi)-[:HAS_DESIGN]->(bd:BrandDesign {key: "brand-design-qrcode-ai"})
SET bd.display_name = "QR Code AI Design System",
    bd.description = "Visual design tokens and guidelines for QR Code AI",
    bd.color_primary = "#6366F1",
    bd.color_secondary = "#8B5CF6",
    bd.color_accent = "#F59E0B",
    bd.color_background = "#FFFFFF",
    bd.color_text = "#1F2937",
    bd.color_palette = '["#6366F1", "#8B5CF6", "#F59E0B", "#10B981", "#EF4444"]',
    bd.font_primary = "Inter",
    bd.font_secondary = "Poppins",
    bd.font_mono = "JetBrains Mono",
    bd.typography_scale = '[{"name": "h1", "size": "3rem", "weight": "700"}, {"name": "body", "size": "1rem", "weight": "400"}]',
    bd.border_radius = "0.75rem",
    bd.shadow_style = "soft",
    bd.animation_style = "smooth",
    bd.llm_context = "USE: when generating visual content or applying brand design tokens. TRIGGERS: colors, fonts, design tokens, @brand.design.",
    bd.created_at = coalesce(bd.created_at, datetime()),
    bd.updated_at = datetime();
```

**Migration Cypher** (move properties from existing Brand to new BrandDesign):
```cypher
// File: packages/db/seed/migrations/002-brand-design-split.cypher
MATCH (b:Brand {key: "brand-qrcode-ai"})
MERGE (b)-[:HAS_DESIGN]->(bd:BrandDesign {key: "brand-design-qrcode-ai"})
SET bd.display_name = "QR Code AI Design System",
    bd.description = "Visual design tokens and guidelines",
    bd.color_primary = b.color_primary,
    bd.color_secondary = b.color_secondary,
    bd.color_accent = b.color_accent,
    bd.color_background = b.color_background,
    bd.color_text = b.color_text,
    bd.color_palette = b.color_palette,
    bd.font_primary = b.font_primary,
    bd.font_secondary = b.font_secondary,
    bd.font_mono = b.font_mono,
    bd.typography_scale = b.typography_scale,
    bd.border_radius = b.border_radius,
    bd.shadow_style = b.shadow_style,
    bd.animation_style = b.animation_style,
    bd.created_at = coalesce(bd.created_at, datetime()),
    bd.updated_at = datetime()
// Remove from Brand
REMOVE b.color_primary, b.color_secondary, b.color_accent,
       b.color_background, b.color_text, b.color_palette,
       b.font_primary, b.font_secondary, b.font_mono,
       b.typography_scale, b.border_radius, b.shadow_style, b.animation_style
SET b.brand_name = coalesce(b.brand_name, "QR Code AI")
RETURN b.key AS brand, bd.key AS brand_design;
```

**Verification query**:
```cypher
// Check Brand no longer has design properties
MATCH (b:Brand {key: "brand-qrcode-ai"})
RETURN b.color_primary, b.font_primary;
// Expected: null, null

// Check BrandDesign was created
MATCH (b:Brand)-[:HAS_DESIGN]->(bd:BrandDesign)
RETURN b.key, bd.key, bd.color_primary;
// Expected: brand-qrcode-ai | brand-design-qrcode-ai | #6366F1
```

---

## Phase 2: llm_context Dual Pattern (61 YAMLs)

### What to add to each YAML

Every node-class YAML needs a `llm_context` property in BLOC 4:

```yaml
  properties:
    # --- ADD THIS BLOCK to every YAML ---
    llm_context:
      type: string
      required: false
      description: |
        Instance-specific context for LLM generation.
        Describes unique characteristics, constraints, or requirements
        for this particular instance.
        Best practices: focused (2-4 key points), include data provenance if relevant.
      example: "Specific use case or constraint for this instance."
    # --- END BLOCK ---
```

**Placement rule**: After standard_properties block, FIRST property in `properties:` section.

**Exception nodes** (already have specific llm_context defined): None — add to all.

### File list for Phase 2

```
# SHARED (40 files)
packages/core/models/node-classes/shared/config/entity-category.yaml
packages/core/models/node-classes/shared/config/locale.yaml
packages/core/models/node-classes/shared/config/seo-keyword-format.yaml
packages/core/models/node-classes/shared/locale/culture.yaml
packages/core/models/node-classes/shared/locale/culture-set.yaml
packages/core/models/node-classes/shared/locale/formatting.yaml
packages/core/models/node-classes/shared/locale/locale-style.yaml
packages/core/models/node-classes/shared/locale/locale-voice.yaml
packages/core/models/node-classes/shared/locale/slugification.yaml
packages/core/models/node-classes/shared/geography/continent.yaml
packages/core/models/node-classes/shared/geography/country.yaml
packages/core/models/node-classes/shared/geography/geo-region.yaml
packages/core/models/node-classes/shared/geography/geo-sub-region.yaml
packages/core/models/node-classes/shared/geography/geo-zone.yaml
packages/core/models/node-classes/shared/geography/market.yaml
packages/core/models/node-classes/shared/geography/city.yaml
packages/core/models/node-classes/shared/knowledge/*.yaml  (24 files)

# ORG (21 files)
packages/core/models/node-classes/org/config/org-config.yaml
packages/core/models/node-classes/org/foundation/*.yaml  (6 files)
packages/core/models/node-classes/org/structure/*.yaml   (3 files)
packages/core/models/node-classes/org/semantic/*.yaml    (4 files)
packages/core/models/node-classes/org/instruction/*.yaml (4 files)
packages/core/models/node-classes/org/output/*.yaml      (3 files)
```

---

## Phase 3: Schema Standard Documentation Update

**File to update**: `.claude/rules/schema-standard.md`

**Current (wrong)**:
```markdown
**Note:** `llm_context` is at BLOC 2 level (schema metadata), NOT in standard_properties.
```

**Replace with**:
```markdown
## llm_context: Dual Pattern

llm_context exists at TWO levels:

### BLOC 2: Schema-level (CLASS directive)
```yaml
node:
  llm_context: |
    USE: when [primary use case].
    TRIGGERS: "keyword1", "keyword2".
    NOT: for [disambiguation].
    RELATES: [Source], [Target].
```
→ Describes how Claude should USE this node CLASS.
→ Required for all nodes.

### BLOC 4: Instance-level (data property)
```yaml
  properties:
    llm_context:
      type: string
      required: false
      description: "Instance-specific context for LLM generation."
```
→ Contains specific context for THIS particular instance.
→ Optional, not all instances need it.
→ Example seed: `s.llm_context = 'URL slugification rules for fr-FR. latin_preserve rule.'`
```

---

## Phase 4: Coherence Validation (coherence_check.rs)

### Module location
```
tools/novanet/src/
└── validators/
    └── coherence_check.rs   ← NEW
```

### Module interface
```rust
pub struct CoherenceChecker {
    root: PathBuf,
    strict: bool,
}

pub struct CoherenceReport {
    pub violations: Vec<CoherenceViolation>,
    pub auto_fixable: Vec<CoherenceViolation>,
    pub manual_required: Vec<CoherenceViolation>,
}

pub enum CoherenceSeverity {
    Critical,    // Blocks seed
    Warning,     // Logged
    Info,        // Reported
}

impl CoherenceChecker {
    pub fn check_all(&self) -> Result<CoherenceReport>
    pub fn check_node(&self, class_name: &str) -> Result<CoherenceReport>
    pub fn auto_fix(&self) -> Result<Vec<String>>
}
```

### Validation rules implemented

```rust
// Rule 1: KEY_REQUIRED
// Every non-satellite node must have `key` in standard_properties
fn check_key_required(yaml: &NodeYaml) -> Vec<CoherenceViolation>

// Rule 2: TIMESTAMP_REQUIRED
// All nodes must have created_at and updated_at in standard_properties
fn check_timestamps(yaml: &NodeYaml) -> Vec<CoherenceViolation>

// Rule 3: LLM_CONTEXT_BLOC4
// All nodes should declare llm_context in properties
fn check_llm_context_property(yaml: &NodeYaml) -> Vec<CoherenceViolation>

// Rule 4: PROP_ORDER
// standard_properties must follow: key → *_key → display_name → description → created_at → updated_at
fn check_prop_order(yaml: &NodeYaml) -> Vec<CoherenceViolation>

// Rule 5: ADR028_BRAND_DESIGN
// Brand must have HAS_DESIGN relation to BrandDesign
fn check_brand_architecture(yaml: &NodeYaml) -> Vec<CoherenceViolation>
```

### CLI integration
```
cargo run -- schema validate --coherence       # Run coherence checks
cargo run -- schema validate --coherence --fix # Auto-fix safe violations
```

---

## Execution Order

```
Step 1: Create migration files
   → packages/db/seed/migrations/001-slugification-timestamps.cypher
   → packages/db/seed/migrations/002-brand-design-split.cypher

Step 2: Update seed file 31-project-qrcode-ai.cypher
   → Add brand_name to Brand
   → Create BrandDesign node
   → Move 13 design properties to BrandDesign

Step 3: Update seed file 22-slugification.cypher
   → Add created_at/updated_at to all 200 MERGE blocks

Step 4: Add llm_context property to brand.yaml (reference example)
   → Verify property declaration matches seed usage

Step 5: Update schema-standard.md
   → Document dual llm_context pattern

Step 6: Create slugification.yaml BLOC 4 llm_context declaration
   → This node already has llm_context in instances, just needs schema declaration

Step 7: Add llm_context to remaining 59 YAMLs
   → Batch update with consistent template

Step 8: Implement coherence_check.rs
   → Validates the above rules
   → Integrates into CI

Step 9: Run full validation
   → cargo run -- schema validate --strict
   → pnpm infra:seed (if Neo4j running)
   → Verify 0 violations
```

---

## Validation Queries (run after each step)

```cypher
// 1. Check Slugification timestamps
MATCH (s:Slugification)
WHERE s.created_at IS NULL OR s.updated_at IS NULL
RETURN count(s) AS missing_timestamps;
-- Expected: 0

// 2. Check Brand brand_name
MATCH (b:Brand)
WHERE b.brand_name IS NULL
RETURN b.key;
-- Expected: 0 rows

// 3. Check BrandDesign exists
MATCH (b:Brand)-[:HAS_DESIGN]->(bd:BrandDesign)
RETURN b.key, bd.key;
-- Expected: brand-qrcode-ai | brand-design-qrcode-ai

// 4. Check Brand no longer has design props
MATCH (b:Brand {key: "brand-qrcode-ai"})
RETURN b.color_primary, b.font_primary;
-- Expected: null, null

// 5. Check all nodes have timestamps
MATCH (n)
WHERE NOT n:Brand AND NOT n:BrandDesign  // Exclude newly checked
  AND (n.created_at IS NULL OR n.updated_at IS NULL)
  AND any(label IN labels(n) WHERE label IN ['Slugification', 'Page', 'Block', 'Entity'])
RETURN labels(n) AS type, count(n) AS count;
-- Expected: 0 for all types
```

---

## Files Modified Summary

| File | Type | Action |
|------|------|--------|
| `packages/db/seed/31-project-qrcode-ai.cypher` | Seed | Add brand_name, create BrandDesign |
| `packages/db/seed/22-slugification.cypher` | Seed | Add created_at/updated_at (200 blocks) |
| `packages/db/seed/migrations/001-slugification-timestamps.cypher` | Migration | NEW: Fix existing instances |
| `packages/db/seed/migrations/002-brand-design-split.cypher` | Migration | NEW: Split Brand/BrandDesign |
| `packages/core/models/node-classes/org/foundation/brand.yaml` | Schema | Add llm_context property |
| `packages/core/models/node-classes/shared/locale/slugification.yaml` | Schema | Add llm_context property |
| `packages/core/models/node-classes/**/*.yaml` (59 more) | Schema | Add llm_context property |
| `.claude/rules/schema-standard.md` | Docs | Document dual llm_context pattern |
| `tools/novanet/src/validators/coherence_check.rs` | Rust | NEW: Validation module |

---

---

## Phase 5: Multi-Agent YAML Verification

After adding llm_context to all 61 YAMLs, launch 3 parallel agents to cross-verify:

### Agent 1 — YAML Structure Audit
```bash
# Check every YAML has both BLOC 2 and BLOC 4 llm_context
for f in $(find packages/core/models/node-classes -name "*.yaml"); do
  has_bloc2=$(grep -c "^  llm_context:" "$f" || true)
  has_bloc4=$(grep -c "^    llm_context:" "$f" || true)
  if [ "$has_bloc2" -eq 0 ] || [ "$has_bloc4" -eq 0 ]; then
    echo "MISSING: $f (BLOC2=$has_bloc2 BLOC4=$has_bloc4)"
  fi
done
```

### Agent 2 — YAML vs Seed Coherence
```bash
# Check every property declared in YAML exists in seed files
# Focus on required properties
```

### Agent 3 — Property Order Check
```bash
# Check standard_properties order in each YAML
# key → *_key → display_name → description → created_at → updated_at
```

---

## Phase 6: Neo4j Seed + Instance Verification

### Step 6.1 — Re-seed database

```bash
cd /path/to/novanet-hq
pnpm infra:seed
```

Or run migrations only (faster, if DB already seeded):
```bash
# Apply migration 001
cat packages/db/seed/migrations/001-slugification-timestamps.cypher | \
  cypher-shell -u neo4j -p novanetpassword

# Apply migration 002
cat packages/db/seed/migrations/002-brand-design-split.cypher | \
  cypher-shell -u neo4j -p novanetpassword
```

### Step 6.2 — Instance vs Schema verification queries

```cypher
// ─────────────────────────────────────────────────
// VERIFICATION 1: All Slugification have timestamps
// ─────────────────────────────────────────────────
MATCH (s:Slugification)
WHERE s.created_at IS NULL OR s.updated_at IS NULL
RETURN count(s) AS missing_timestamps;
// Expected: 0

// ─────────────────────────────────────────────────
// VERIFICATION 2: Brand has brand_name
// ─────────────────────────────────────────────────
MATCH (b:Brand)
WHERE b.brand_name IS NULL
RETURN b.key, b.display_name;
// Expected: 0 rows

// ─────────────────────────────────────────────────
// VERIFICATION 3: BrandDesign exists + HAS_DESIGN arc
// ─────────────────────────────────────────────────
MATCH (b:Brand)-[:HAS_DESIGN]->(bd:BrandDesign)
RETURN b.key, bd.key, bd.color_primary, bd.font_primary;
// Expected: brand-qrcode-ai | brand-design-qrcode-ai | #6366F1 | Inter

// ─────────────────────────────────────────────────
// VERIFICATION 4: Brand no longer has design props
// ─────────────────────────────────────────────────
MATCH (b:Brand {key: "brand-qrcode-ai"})
RETURN keys(b) AS all_properties;
// Expected: no color_*, font_*, border_radius, shadow_style, animation_style

// ─────────────────────────────────────────────────
// VERIFICATION 5: All nodes have llm_context (sample)
// ─────────────────────────────────────────────────
MATCH (s:Slugification {key: "fr-FR"})
RETURN s.llm_context, s.created_at, s.updated_at;
// Expected: non-null for all 3

// ─────────────────────────────────────────────────
// VERIFICATION 6: Schema coherence - count all node types
// ─────────────────────────────────────────────────
MATCH (n)
WHERE NOT n:_Bloom_Perspective_
RETURN labels(n) AS type, count(n) AS count,
       sum(CASE WHEN n.created_at IS NULL THEN 1 ELSE 0 END) AS missing_created_at,
       sum(CASE WHEN n.updated_at IS NULL THEN 1 ELSE 0 END) AS missing_updated_at
ORDER BY count DESC;

// ─────────────────────────────────────────────────
// VERIFICATION 7: Brand-BrandDesign full graph
// ─────────────────────────────────────────────────
MATCH (p:Project)-[:HAS_BRAND]->(b:Brand)
OPTIONAL MATCH (b)-[:HAS_DESIGN]->(bd:BrandDesign)
OPTIONAL MATCH (b)-[:HAS_PRINCIPLES]->(bp)
RETURN p.key, b.key, b.brand_name, bd.key, bp.key;
```

### Step 6.3 — YAML Schema vs Neo4j Instance property comparison

For each node type, compare declared properties in YAML vs actual properties in Neo4j:

```cypher
// Get all properties for a specific node type
MATCH (b:Brand {key: "brand-qrcode-ai"})
RETURN keys(b) AS neo4j_properties;
```

Then compare against brand.yaml `standard_properties` + `properties` keys.

---

## Phase 7: TUI Visual Verification

### What to verify in TUI

```bash
cargo run -- tui
```

Navigate to:

1. **Realm:org → Layer:foundation → Class:Brand**
   - Should show Brand node with properties (NO design props)
   - `brand_name` should be visible in info panel
   - `llm_context` should show instance context

2. **Realm:org → Layer:foundation → Class:BrandDesign** (NEW)
   - Should appear in tree (didn't exist before!)
   - Should show all design tokens (colors, fonts, etc.)
   - Should show `[:HAS_DESIGN]` arc from Brand

3. **Realm:shared → Layer:locale → Class:Slugification**
   - Sample instance (fr-FR, en-US, ar-SA) should show
   - `created_at` and `updated_at` visible in properties
   - `llm_context` summary visible

4. **Arc:HAS_DESIGN** in arc families
   - Should show Brand → BrandDesign relationship
   - Visible in ownership family

### TUI display improvement checks

| What | Expected TUI display | If wrong |
|------|---------------------|----------|
| Brand instance | Shows brand_name, no color_primary | Re-seed required |
| BrandDesign instance | Shows color_primary, font_primary | Run migration 002 |
| Slugification fr-FR | Shows created_at, llm_context | Run migration 001 |
| HAS_DESIGN arc | Visible in tree | Schema generate needed |

### Visual test command
```bash
# Quick non-interactive check
cargo run -- data --class=Brand --format=json | jq '.[] | {key, brand_name, color_primary}'
# Expected: brand_name present, color_primary absent

cargo run -- data --class=BrandDesign --format=json | jq '.[] | {key, color_primary, font_primary}'
# Expected: color_primary and font_primary present

cargo run -- data --class=Slugification --format=table | head -5
# Expected: created_at column visible
```

---

## Execution Order (Updated)

```
✅ DONE - Phase 1.1: Fix Slugification timestamps (22-slugification.cypher + migration 001)
✅ DONE - Phase 1.2: Fix Brand brand_name (31-project-qrcode-ai.cypher)
✅ DONE - Phase 1.3: Create BrandDesign node (31-project-qrcode-ai.cypher + migration 002)
✅ DONE - Phase 1.4: schema-standard.md dual llm_context documented
✅ DONE - Phase 1.5: brand.yaml + slugification.yaml BLOC 4 llm_context added
🔄 IN PROGRESS - Phase 2: Add llm_context to 59 remaining YAMLs (batch agents)
⏳ NEXT - Phase 3: Commit YAML changes
⏳ NEXT - Phase 4: Implement coherence_check.rs
⏳ NEXT - Phase 5: Multi-agent YAML verification (3 parallel agents)
⏳ NEXT - Phase 6: Neo4j seed + instance verification queries
⏳ NEXT - Phase 7: TUI visual verification + data commands
```

---

## Success Criteria

- [ ] 0 Slugification nodes without timestamps (Neo4j verified)
- [ ] 0 Brand nodes without brand_name (Neo4j verified)
- [ ] BrandDesign node exists with HAS_DESIGN arc from Brand (Neo4j verified)
- [ ] Brand has 0 design properties (Neo4j verified - keys() check)
- [ ] 61/61 YAMLs declare llm_context in BLOC 2 (schema-level)
- [ ] 61/61 YAMLs declare llm_context in BLOC 4 (properties-level)
- [ ] schema-standard.md documents dual llm_context pattern
- [ ] `cargo run -- schema validate --strict` exits 0
- [ ] `cargo test` still passes (1082+ tests)
- [ ] TUI shows BrandDesign as new node in org/foundation layer
- [ ] TUI shows Slugification instances with timestamps visible
- [ ] `cargo run -- data --class=Brand` shows brand_name, no design props
- [ ] `cargo run -- data --class=BrandDesign` shows design tokens
