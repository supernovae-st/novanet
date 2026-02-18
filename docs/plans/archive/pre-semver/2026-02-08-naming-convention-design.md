# Design: Naming Convention Refactor

**Date**: 2026-02-08
**Status**: Approved
**Brainstorm session**: EntityL10n → EntityContent + composite keys
**Version**: v10.9.0

---

## Summary

Refactor locale-specific node naming to be more consistent and semantic:
- **Semantic/Foundation layer**: `*L10n` → `*Content` (source of truth, human or AI authored)
- **Output layer**: `*L10n` → `*Generated` (AI-produced, derived)
- **Trait changes**: Output layer nodes change from `localized` → `derived`
- **Composite keys**: `type:base-key@locale` pattern (key = ASCII, slug = UTF-8)
- **Slugification**: Follow locale-specific rules (preserve accents for latin_preserve locales)
- **Page URLs**: Use EntityContent.slug directly (UTF-8 with accents for fr-FR)

---

## Key Decisions (from Brainstorm)

| Question | Decision |
|----------|----------|
| Content vs Generated | YES - distinct naming based on layer purpose |
| Arc split HAS_L10N | YES - HAS_CONTENT (semantic) + HAS_GENERATED (output) |
| Trait for output layer | CHANGE to `derived` (was `localized`) |
| Key format | ASCII (technical) - e.g., `entity:create-qr-code@fr-FR` |
| Slug format | UTF-8 (public URL) - e.g., `créer-qr-code` |
| SEOKeyword key | ASCII key + UTF-8 slug property |
| ProjectContent key | ADD key property for consistency |
| Backward compat | NONE (v0, clean design) |
| Legacy cleanup | YES, remove all backward compat code |

---

## Node Renames

### Semantic Layer (human-authored, source of truth)

| Before | After | Reason |
|--------|-------|--------|
| EntityL10n | **EntityContent** | "Content" = source data, human-curated |
| ProjectL10n | **ProjectContent** | Consistent with EntityContent |

### Output Layer (AI-generated, derived)

| Before | After | Trait Change | Reason |
|--------|-------|--------------|--------|
| BlockL10n | **BlockGenerated** | `localized` → `derived` | "Generated" = AI-produced, computed output |
| PageL10n | **PageGenerated** | `localized` → `derived` | Consistent with BlockGenerated |

**Why trait: derived?**
- "localized" = locale variant of invariant source (EntityContent)
- "derived" = computed/aggregated from inputs (BlockGenerated, PageGenerated)
- Output layer nodes are LLM-generated from prompts + context, not human-curated
- Aligns with EvaluationSignal, OutputArtifact (also trait: derived in output layer)

### SEO/GEO Layer (no change)

Already good naming:
- SEOKeyword, SEOQuestion, SEOComparison, SEOPreposition, SEOMetrics
- GEOPrompt, GEOResponse, GEOCitation, GEOMention, GEOMetrics

---

## Composite Key Pattern

### Format

```
type:base-key@locale
```

### Examples

| Node | Key Example (ASCII) | Slug Example (UTF-8) |
|------|---------------------|----------------------|
| EntityContent | `entity:create-qr-code@fr-FR` | `créer-qr-code` |
| ProjectContent | `project:qrcode-ai@fr-FR` | (uses Project.key) |
| BlockGenerated | `block:hero-section@fr-FR` | (no public slug) |
| PageGenerated | `page:home@fr-FR` | (uses EntityContent.slug) |
| SEOKeyword | `seo:creer-qr-code-gratuit@fr-FR` | `créer-qr-code-gratuit` |
| GEOPrompt | `geo:comment-creer-qr-code@fr-FR` | `comment-créer-qr-code` |

### Why "@" separator?

- Not used in kebab-case keys
- Semantic: "entity AT locale"
- Familiar pattern (email-style)
- Easy to parse: `key.split('@')` → `[type:base, locale]`

---

## KEY vs SLUG: Critical Distinction

**KEY** = Technical identifier (ASCII, for DB queries, CLI, logs)
**SLUG** = Public URL path (UTF-8, follows locale slugification rules)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  EntityContent (fr-FR)                                                      │
│  ├── key: "entity:create-qr-code@fr-FR"    ← ASCII (technical, DB lookup)   │
│  ├── slug: "créer-qr-code"                 ← UTF-8 (PUBLIC URL)             │
│  └── display_name: "Créer un QR Code"      ← UTF-8 (display)                │
│                                                                             │
│  Page URL: https://qrcode-ai.com/fr/créer-qr-code                           │
│                                   └─────────────── EntityContent.slug       │
│                                                    (WITH accents!)          │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Page = Materialization of EntityContent.slug

A page for a locale is **not** identified by its own slug. It uses the EntityContent.slug:

```
Entity (invariant)           EntityContent (fr-FR)         PageGenerated (fr-FR)
key: "create-qr-code"   →    slug: "créer-qr-code"    →    assembles blocks for
                             display_name: "Créer..."      this entity in fr-FR
                                    │
                                    ▼
                              PUBLIC URL
                              /fr/créer-qr-code
                              (UTF-8 with accents)
```

### SEOKeyword Key Format

SEOKeyword follows the same pattern:

```yaml
SEOKeyword:
  key: "seo:creer-qr-code-gratuit@fr-FR"     # ASCII (technical)
  value: "créer qr code gratuit"              # UTF-8 (the actual keyword)
  slug: "créer-qr-code-gratuit"               # UTF-8 (for URL if needed)
  locale_key: "fr-FR"                         # denormalized
```

**Rule**: Key is ASCII for technical safety, slug/value are UTF-8 for public display.

---

## Properties (Denormalized for Query Performance)

```yaml
EntityContent:
  key: "entity:create-qr-code@fr-FR"   # UNIQUE constraint
  entity_key: "create-qr-code"          # INDEX - filter by entity
  locale_key: "fr-FR"                   # INDEX - filter by locale
  slug: "créer-qr-code"                 # INDEX - URL routing
  display_name: "Créer un QR Code"
  # ... other content fields
```

**Rule**: `key` = source of truth, other fields = denormalized indexes

**Validation**: `key == f"{type}:{entity_key}@{locale_key}"`

---

## Slugification Rules

Slugs MUST follow the locale's Slugification rules (from global/config/slugification.yaml).

### Slug Rules by Locale Type

| slug_rule | Behavior | Locales | Example |
|-----------|----------|---------|---------|
| `latin_preserve` | Keep diacritics (é,ç,ü) | fr-FR, de-DE, es-ES | "créer-qr-code" |
| `latin_strip` | Remove diacritics (e,c,u) | en-US (SEO legacy) | "creer-qr-code" |
| `native_script` | Keep native script | ar-SA, zh-CN, ja-JP | "إنشاء-رمز-qr" |
| `transliterate` | All to ASCII | legacy systems | "creer-qr-code" |

### French (fr-FR) Example

```
Locale: fr-FR
slug_rule: latin_preserve
preserve_diacritics: true

Input:  "Créer un QR Code gratuit"
Output: "créer-qr-code-gratuit"  ← WITH accents!
NOT:    "creer-qr-code-gratuit"  ← WRONG for fr-FR
```

### Arabic (ar-SA) Example

```
Locale: ar-SA
slug_rule: native_script

Input:  "إنشاء رمز QR"
Output: "إنشاء-رمز-qr"  ← Arabic script preserved
```

---

## Semantic Connections

### EntityContent Connections

```
Locale ──[:HAS_SLUGIFICATION]──► Slugification
                                      │
                                      │ (rules applied to)
                                      ▼
Entity ──[:HAS_CONTENT]──► EntityContent ──[:FOR_LOCALE]──► Locale
   │                            │
   │                            ├── slug (follows Slugification rules)
   │                            │
   │                            └──[:TARGETS]──► SEOKeyword
   │                                                  │
   │                                                  └── key includes localized slug
   │
   └── key: "create-qr-code" (invariant, EN)
```

### Arc Renames

| Before | After |
|--------|-------|
| HAS_L10N | **HAS_CONTENT** (semantic layer) |
| HAS_L10N | **HAS_GENERATED** (output layer) |

---

## Page Architecture (Option C - Approved)

### Invariant Level (Structure)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  INVARIANT LEVEL (structure, defined once)                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Entity ◄──────────[:FOR_ENTITY]────────── Page ──[:HAS_BLOCK]──► Block     │
│    │                                         │                       │      │
│    │ key: "create-qr-code"                   │ key: "create-qr-code" │      │
│    │                                         │ (same as entity)      │      │
│    │                                         │                       │      │
│    │                                         └── defines structure   │      │
│    │                                             (which blocks)      │      │
│    │                                                                 │      │
│    └── semantic concept                      Block = template        │      │
│        (what this page is about)             (hero, features, cta)   │      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Localized Level (Content)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  LOCALIZED LEVEL (content, generated per locale)                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  EntityContent ◄──[:REPRESENTS]── PageGenerated ──[:ASSEMBLES]──► BlockGen  │
│       │                                 │                            │      │
│       │ slug: "créer-qr-code"           │ no own slug!               │      │
│       │       (PUBLIC URL)              │ uses EntityContent.slug    │      │
│       │                                 │                            │      │
│       └── source of URL                 └── assembles blocks for     │      │
│                                             this entity in fr-FR     │      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Complete Graph Flow

```
Entity (invariant)           Page (invariant)              Block (invariant)
key: "create-qr-code"   ◄────[:FOR_ENTITY]────   key: "create-qr-code"   ──[:HAS_BLOCK]──►   key: "hero"
        │                                                │
        │ [:HAS_CONTENT]                                 │ [:HAS_GENERATED]
        ▼                                                ▼
EntityContent (fr-FR)        PageGenerated (fr-FR)       BlockGenerated (fr-FR)
slug: "créer-qr-code"   ◄────[:REPRESENTS]────   key: "page:create-qr-code@fr-FR"   ──[:ASSEMBLES]──►
        │
        ▼
  PUBLIC URL
  /fr/créer-qr-code
```

### New Arcs Required

| Arc | Direction | Purpose |
|-----|-----------|---------|
| `FOR_ENTITY` | Page → Entity | "This page is about this entity" (invariant) |
| `REPRESENTS` | PageGenerated → EntityContent | "This generated page represents this content" (locale) |
| `HAS_BLOCK` | Page → Block | "This page includes this block template" (invariant) |
| `ASSEMBLES` | PageGenerated → BlockGenerated | "This page assembles these blocks" (locale) |

### Why This Architecture?

1. **Separation of concerns**:
   - Page = structure (which blocks, in what order)
   - Entity = meaning (what this page is about)
   - EntityContent = URL source (slug)

2. **URL routing clarity**:
   - PageGenerated does NOT have its own slug
   - URL comes from EntityContent.slug (single source of truth)
   - Query: `MATCH (pg:PageGenerated)-[:REPRESENTS]->(ec:EntityContent) RETURN ec.slug`

3. **Extensibility**:
   - Same Page structure → multiple locales
   - Entity can have multiple Pages (landing, detail, comparison)
   - Block reuse across Pages

### Resolved: Generic Pages Strategy

**Decision**: Entity pour chaque page, type = sémantique (pas type=PAGE)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  Entity.type = SÉMANTIQUE (ce que c'est)                                    │
│  Page existence = OPTIONNELLE (via [:FOR_ENTITY])                           │
│                                                                             │
│  Ces deux concepts sont ORTHOGONAUX !                                       │
└─────────────────────────────────────────────────────────────────────────────┘

Entity           type            Has Page?    URL
───────────────────────────────────────────────────────────────
pricing          FEATURE         ✅ oui       /fr/tarifs
about            THING           ✅ oui       /fr/a-propos
blog             FEATURE         ✅ oui       /fr/blog (listing)
contact          FEATURE         ✅ oui       /fr/contact
terms            THING           ✅ oui       /fr/conditions-generales
create-qr-code   ACTION          ✅ oui       /fr/créer-qr-code
qr-code          PRODUCT         ✅ oui       /fr/qr-code
error-correction CONCEPT         ❌ non       (pas de page dédiée)
```

**Key insight**: Pas besoin de type=PAGE. N'importe quelle Entity peut avoir une Page.

### Resolved: Blog & Hierarchical URLs

**Decision**: Option 1 - URL hierarchy via [:CONTAINS]

```
Entity: "blog" (type: FEATURE)
├── Page: oui (listing page) → /fr/blog
└── [:CONTAINS] →
       ├── Entity: "how-to-create-qr-code" (type: GUIDE)
       │      └── Page: oui → /fr/blog/comment-créer-qr-code
       └── Entity: "qr-code-best-practices" (type: GUIDE)
              └── Page: oui → /fr/blog/bonnes-pratiques-qr-code
```

**URL Construction**:
```
parent EntityContent.slug + "/" + child EntityContent.slug
= "blog" + "/" + "comment-créer-qr-code"
= /fr/blog/comment-créer-qr-code
```

**Why Option 1?**
- Graph already encodes hierarchy via [:CONTAINS]
- No redundancy (slug stored once per EntityContent)
- Query: traverse [:CONTAINS] to build full path
- Flexible: same Entity could appear in multiple parents (symlinks)

---

## Decisions from Socratic Exploration (10 Agents)

The following decisions were made after 10 specialized agents explored the design:

### Decision 1: Rename [:CONTAINS] → [:HAS_CHILD] for Entity Hierarchy

**Status**: ✅ APPROVED

**Problem**: `[:CONTAINS]` already exists for Knowledge Atoms (`*Set → *Atom`). Reusing it for Entity hierarchies creates semantic collision.

**Decision**: Create NEW arc `[:HAS_CHILD]` specifically for Entity hierarchy.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  DOMAIN SEPARATION                                                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  [:CONTAINS] (existing)           [:HAS_CHILD] (new)                        │
│  ─────────────────────            ──────────────────                        │
│  Family: ownership                Family: ownership                         │
│  Domain: Knowledge Atoms          Domain: Entity Hierarchy                  │
│  Pattern: *Set → *Atom            Pattern: Entity → Entity                  │
│  Example: TermSet → Term          Example: blog → article                   │
│                                                                             │
│  DIFFERENT PURPOSES - NO COLLISION                                          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Arc Definition** (`has-child.yaml`):
```yaml
arc:
  name: HAS_CHILD
  family: ownership
  scope: intra_realm
  cardinality: many_to_many  # Entity can have multiple parents (symlinks)

  properties:
    position:
      type: int
      required: true
      description: "Sort order among siblings"

    visibility:
      type: string
      enum: ["public", "draft", "internal"]
      required: true
      description: "Content visibility status"

    featured:
      type: boolean
      required: false
      description: "Highlight in parent's featured section"

  cycle_protection: true  # Prevent A → B → A

  relations:
    source: Entity
    target: Entity
    inverse: CHILD_OF  # Optional inverse
```

**Why many_to_many?**
- Enables symlinks/shortcuts (article in multiple sections)
- Enables featured collections
- Cycle detection prevents infinite loops

---

### Decision 2: Fix YAML Slug Pattern (ASCII → UTF-8)

**Status**: ✅ APPROVED

**Problem**: Current YAML pattern `^[a-z0-9-]+$` is ASCII-only, but design requires UTF-8 slugs with accents.

**Before** (WRONG):
```yaml
slug:
  pattern: "^[a-z0-9-]+$"  # ASCII only - rejects "créer-qr-code"
```

**After** (CORRECT):
```yaml
slug:
  type: string
  required: true
  pattern: "^[\\p{Ll}\\p{N}\\-]+$"  # Unicode lowercase letters + numbers + hyphens
  description: "UTF-8 slug following locale slugification rules"
  examples:
    - "créer-qr-code"       # fr-FR (latin_preserve)
    - "create-qr-code"      # en-US (latin_strip)
    - "إنشاء-رمز-qr"         # ar-SA (native_script)
```

**Unicode Normalization**:
- All slugs stored in NFC (Composed) form
- URL routing normalizes incoming URLs before lookup
- Browser encoding differences handled at HTTP layer

```typescript
// URL routing middleware
function normalizeSlug(slug: string): string {
  return decodeURIComponent(slug)
    .normalize('NFC')
    .toLowerCase();
}
```

---

### Decision 3: Add UNIQUE Constraint on Slug per Locale

**Status**: ✅ APPROVED

**Problem**: Nothing prevents two Entities from having the same slug in the same locale, causing URL collisions.

**Decision**: Add UNIQUE constraint on `(locale_key, slug)` tuple.

**Neo4j Constraint**:
```cypher
CREATE CONSTRAINT entity_content_slug_unique IF NOT EXISTS
FOR (ec:EntityContent)
REQUIRE (ec.locale_key, ec.slug) IS UNIQUE;
```

**Validation at Creation**:
```typescript
async function validateSlug(locale: string, slug: string, entityKey: string) {
  const existing = await query(`
    MATCH (ec:EntityContent {locale_key: $locale, slug: $slug})
    WHERE ec.entity_key <> $entityKey
    RETURN ec.entity_key
  `, { locale, slug, entityKey });

  if (existing.length > 0) {
    throw new Error(`Slug collision: "${slug}" already used by Entity "${existing[0]}"`);
  }
}
```

**Collision Resolution**:
- Add `slug_override` property for manual disambiguation
- Example: "banque" (collision) → "banque-financiere" (override)

---

### Decision 4: Add Redirect Mechanism for URL Changes

**Status**: ✅ APPROVED

**Problem**: When Entity parent changes, URL changes, breaking external links.

**Decision**: Add `slug_history[]` property for 301 redirect support.

```yaml
EntityContent:
  properties:
    slug:
      type: string
      required: true
      immutable: true  # Cannot be changed after creation

    slug_history:
      type: string[]
      required: false
      description: "Previous slugs for 301 redirect compatibility"
      example: ["old-slug-2024", "older-slug-2023"]
```

**URL Routing with Redirects**:
```typescript
async function resolveUrl(locale: string, slug: string): Promise<UrlResolution> {
  // 1. Try exact match
  const exact = await query(`
    MATCH (ec:EntityContent {locale_key: $locale, slug: $slug})
    RETURN ec
  `, { locale, slug });

  if (exact.length > 0) {
    return { type: 'found', entity: exact[0] };
  }

  // 2. Try slug_history (301 redirect)
  const historical = await query(`
    MATCH (ec:EntityContent {locale_key: $locale})
    WHERE $slug IN ec.slug_history
    RETURN ec.slug AS new_slug
  `, { locale, slug });

  if (historical.length > 0) {
    return {
      type: 'redirect',
      status: 301,
      newUrl: `/${locale}/${historical[0].new_slug}`
    };
  }

  // 3. Not found
  return { type: 'not_found', status: 404 };
}
```

**Slug Change Workflow**:
1. Slug is IMMUTABLE after creation
2. If Entity.display_name changes but slug should stay same → keep slug
3. If slug MUST change (rare, manual operation):
   - Add current slug to `slug_history[]`
   - Generate new slug
   - New content created with new slug
   - Old slug in history enables 301 redirect

---

### Decision 5: Set Maximum Hierarchy Depth = 3

**Status**: ✅ APPROVED

**Problem**: Deep hierarchies create long URLs with performance and SEO issues.

**Decision**: Maximum 3 levels of nesting via `[:HAS_CHILD]`.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  DEPTH LIMITS                                                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Depth 0: /fr/blog                    (root)                                │
│  Depth 1: /fr/blog/guides             (category)                            │
│  Depth 2: /fr/blog/guides/qr-codes    (subcategory)                         │
│  Depth 3: /fr/blog/guides/qr-codes/comment-créer  (article) ← MAX           │
│                                                                             │
│  ❌ Depth 4+: NOT ALLOWED                                                   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Why 3 levels?**
- SEO optimal: URLs under 75 characters
- Performance: O(n³) max traversal, acceptable
- User experience: Breadcrumbs fit on screen
- Covers 99% of content organization needs

**Validation**:
```rust
const MAX_HIERARCHY_DEPTH: usize = 3;

fn validate_depth(entity_key: &str) -> Result<(), ValidationError> {
    let depth = count_parents(entity_key)?;
    if depth > MAX_HIERARCHY_DEPTH {
        return Err(ValidationError::HierarchyTooDeep {
            entity: entity_key.to_string(),
            depth,
            max: MAX_HIERARCHY_DEPTH,
        });
    }
    Ok(())
}
```

**Edge Cases**:
- If depth 4+ needed: flatten hierarchy (use tags instead)
- Query param fallback: `/fr/entity/article?parent=blog` (rare)

---

### Decision 6: Arc Families for HAS_CONTENT and HAS_GENERATED

**Status**: ✅ APPROVED

**Problem**: Design didn't specify which arc families the new arcs belong to.

**Decision**:

| Arc | Family | Rationale |
|-----|--------|-----------|
| `HAS_CONTENT` | **localization** | Like old HAS_L10N, links invariant to locale variant |
| `HAS_GENERATED` | **generation** | Links to LLM-generated output (not human-curated) |
| `FOR_ENTITY` | **semantic** | Semantic relationship at invariant level |
| `REPRESENTS` | **semantic** | Semantic relationship at localized level |
| `ASSEMBLES` | **generation** | Assembly of generated output |
| `HAS_CHILD` | **ownership** | Parent owns/contains children |

**Arc Definitions**:

```yaml
# has-content.yaml
arc:
  name: HAS_CONTENT
  family: localization
  scope: intra_realm
  cardinality: one_to_many
  description: "Entity has locale-specific content (source of truth)"
  source: [Entity, Project]
  target: [EntityContent, ProjectContent]

# has-generated.yaml
arc:
  name: HAS_GENERATED
  family: generation
  scope: intra_realm
  cardinality: one_to_many
  description: "Page/Block has AI-generated locale output"
  source: [Page, Block]
  target: [PageGenerated, BlockGenerated]
```

**Visual Encoding**:
- `localization` arcs: dashed stroke (like existing L10n arcs)
- `generation` arcs: dotted stroke (computed output)
- `semantic` arcs: solid stroke (meaning relationships)
- `ownership` arcs: bold stroke (containment)

---

### Decision 7: Add curation_status Property to EntityContent

**Status**: ✅ APPROVED

**Problem**: 200+ locales but not all can be human-reviewed. Need quality tracking.

**Decision**: Add `curation_status` enum property.

```yaml
EntityContent:
  properties:
    curation_status:
      type: string
      required: true
      enum:
        - "human_authored"         # Copywriter wrote this
        - "machine_translated"     # TM + human review
        - "ai_generated"           # AI native, not reviewed
        - "ai_generated_reviewed"  # AI native + human review
      description: "Quality/origin indicator for content"

    reviewed_by:
      type: string
      required: false
      description: "User ID who reviewed (if reviewed)"

    reviewed_at:
      type: datetime
      required: false
      description: "When review occurred"
```

**Tiered Localization Strategy**:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  CONTENT TIERS BY LOCALE                                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Tier 1: Core Locales (en-US, fr-FR, de-DE, es-ES, ja-JP)                  │
│  ├── curation_status: "human_authored"                                      │
│  ├── Full copywriter involvement                                            │
│  └── SEO keyword optimization                                               │
│                                                                             │
│  Tier 2: Secondary Locales (it-IT, pt-BR, nl-NL, pl-PL)                    │
│  ├── curation_status: "machine_translated" or "ai_generated_reviewed"       │
│  ├── AI generation + human linguistic review                                │
│  └── Regional lead approval                                                 │
│                                                                             │
│  Tier 3: Emerging Locales (150+ others)                                     │
│  ├── curation_status: "ai_generated"                                        │
│  ├── AI native generation, no review                                        │
│  ├── Auto-publish with quality flag                                         │
│  └── Optional: "Help improve this" crowdsource link                         │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**UI Indicator**:
- Show curation_status badge on content in Studio
- Filter by curation_status in content management
- Dashboard: % human_authored per locale

---

### Decision 8: Denormalize full_path and parent_slug

**Status**: ✅ APPROVED

**Problem**: Traversing `[:HAS_CHILD]` for every URL lookup is O(n²) slow.

**Decision**: Denormalize hierarchy info on EntityContent.

```yaml
EntityContent:
  properties:
    # Identity
    key: "entity:create-qr-code@fr-FR"
    entity_key: "create-qr-code"
    locale_key: "fr-FR"
    slug: "créer-qr-code"

    # NEW: Hierarchy denormalization
    parent_slug:
      type: string
      required: false
      description: "Parent EntityContent.slug (null if root)"
      example: "blog"

    full_path:
      type: string
      required: true
      description: "Complete URL path (parent_slug/slug)"
      example: "blog/créer-qr-code"
      index: true  # Critical for URL routing

    depth:
      type: int
      required: true
      description: "Hierarchy depth (0 = root)"
      example: 1
```

**Performance Comparison**:

| Query Type | Without Denorm | With Denorm | Speedup |
|------------|----------------|-------------|---------|
| URL lookup | O(n × depth) ~100ms | O(1) ~1ms | **100x** |
| Build breadcrumbs | O(depth) ~50ms | O(1) ~1ms | **50x** |
| List children | O(n) ~10ms | O(n) ~10ms | Same |

**Maintenance Strategy**:
1. **On create**: Compute full_path from parent
2. **On parent change**: Mark children as stale (lazy update)
3. **On read**: Check if stale, rebuild if needed
4. **Background job**: Clean up stale paths daily

**Cypher Example**:
```cypher
// URL routing (O(1) with index)
MATCH (ec:EntityContent {locale_key: $locale, full_path: $path})
RETURN ec

// Instead of traversal (O(n²))
MATCH (root:Entity)-[:HAS_CHILD*0..3]->(child:Entity {key: $key})
MATCH (child)-[:HAS_CONTENT]->(ec:EntityContent {locale_key: $locale})
RETURN ec
```

---

### Decision 9: Multiple Pages per Entity = Semantic Decomposition

**Status**: ✅ APPROVED

**Problem**: How to handle landing page, FAQ page, comparison page for same Entity?

**Decision**: Each distinct URL = distinct Entity. Use SEMANTIC_LINK to connect.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  SEMANTIC DECOMPOSITION                                                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Entity: "create-qr-code" (ACTION)                                          │
│  ├── EntityContent: slug = "créer-qr-code"                                  │
│  └── Page: landing → /fr/créer-qr-code                                      │
│                                                                             │
│  Entity: "create-qr-code-vs-barcode" (COMPARISON)  ← SEPARATE ENTITY        │
│  ├── EntityContent: slug = "créer-qr-code-vs-barcode"                       │
│  ├── Page: comparison → /fr/créer-qr-code-vs-barcode                        │
│  └── [:SEMANTIC_LINK {type: "compares"}] → Entity("create-qr-code")         │
│                                                                             │
│  Entity: "create-qr-code-faq" (GUIDE)  ← SEPARATE ENTITY                    │
│  ├── EntityContent: slug = "créer-qr-code-faq"                              │
│  ├── Page: faq → /fr/créer-qr-code-faq                                      │
│  └── [:SEMANTIC_LINK {type: "explains"}] → Entity("create-qr-code")         │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Why Not Page.type?**
- Multiple Pages sharing one slug = URL collision
- Query params (`?type=faq`) aren't RESTful
- Each page is SEO-distinct (different keywords, different ranking)

**Rule**:
```
IF page_has_distinct_url THEN needs_distinct_entity
ELSE should_be_block_variant (within same page)
```

**Examples**:
- ✅ `/fr/créer-qr-code` → Entity "create-qr-code"
- ✅ `/fr/créer-qr-code-vs-barcode` → Entity "create-qr-code-vs-barcode"
- ✅ `/fr/créer-qr-code-faq` → Entity "create-qr-code-faq"
- ❌ `/fr/créer-qr-code#faq` → NOT a separate Entity (anchor within page)
- ❌ `/fr/créer-qr-code?view=comparison` → NOT RESTful, avoid

---

### Decision 10: Composite Key Validation Rules

**Status**: ✅ APPROVED

**Problem**: Key format needs explicit validation to prevent errors.

**Decision**: Implement 3-layer validation.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  COMPOSITE KEY VALIDATION                                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Format: type:base-key@locale                                               │
│                                                                             │
│  VALIDATION RULES:                                                          │
│  ─────────────────                                                          │
│  1. type_prefix: ^[a-z]+$  (entity, project, block, page, seo, geo)         │
│  2. base_key:    ^[a-z][a-z0-9-]*$  (no @ or : allowed)                    │
│  3. locale:      ^[a-z]{2}-[A-Z]{2}$  (BCP 47 format)                       │
│  4. max_length:  255 characters                                             │
│  5. type_prefix MUST match node label                                       │
│                                                                             │
│  EXAMPLES:                                                                  │
│  ─────────                                                                  │
│  ✅ entity:create-qr-code@fr-FR                                             │
│  ✅ seo:creer-qr-code-gratuit@fr-FR                                         │
│  ✅ block:hero-section@ja-JP                                                │
│  ❌ entity:créer-qr-code@fr-FR  (accents in base-key)                       │
│  ❌ entity:create@qr@code@fr-FR  (@ in base-key)                            │
│  ❌ Entity:create-qr-code@fr-FR  (uppercase type)                           │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**3-Layer Enforcement**:

```rust
// LAYER 1: Rust Type System (compile-time)
struct CompositeKey {
    type_prefix: String,  // "entity", "seo", etc.
    base_key: String,     // kebab-case, no @ or :
    locale: String,       // BCP 47
}

impl CompositeKey {
    fn parse(s: &str) -> Result<Self, ValidationError> {
        let re = Regex::new(r"^([a-z]+):([a-z][a-z0-9-]*)@([a-z]{2}-[A-Z]{2})$")?;
        // ... validation
    }

    fn validate_with_label(&self, label: &str) -> Result<(), Error> {
        let expected_prefix = label.to_lowercase().replace("content", "entity");
        if self.type_prefix != expected_prefix {
            return Err(Error::TypeLabelMismatch);
        }
        Ok(())
    }
}
```

```cypher
// LAYER 2: Neo4j Constraints (persistence)
CREATE CONSTRAINT entity_content_key_format IF NOT EXISTS
FOR (ec:EntityContent)
REQUIRE ec.key MATCHES '^entity:[a-z][a-z0-9-]*@[a-z]{2}-[A-Z]{2}$';

CREATE CONSTRAINT entity_content_key_unique IF NOT EXISTS
FOR (ec:EntityContent)
REQUIRE ec.key IS UNIQUE;
```

```typescript
// LAYER 3: Runtime Validation (every query)
function validateKeyConsistency(node: EntityContent): void {
  const computed = `entity:${node.entity_key}@${node.locale_key}`;
  if (node.key !== computed) {
    logger.warn(`Key mismatch: stored=${node.key}, computed=${computed}`);
  }
}
```

---

### Decision 11: Required Neo4j Indexes

**Status**: ✅ APPROVED

**Problem**: Current schema lacks indexes, causing 100x slower queries.

**Decision**: Add 6 critical indexes.

```cypher
// ═══════════════════════════════════════════════════════════════════════════
// INDEXES FOR EntityContent (CRITICAL FOR PERFORMANCE)
// ═══════════════════════════════════════════════════════════════════════════

// Identity (UNIQUE)
CREATE CONSTRAINT entity_content_key IF NOT EXISTS
FOR (ec:EntityContent) REQUIRE ec.key IS UNIQUE;

// Slug uniqueness per locale
CREATE CONSTRAINT entity_content_slug_unique IF NOT EXISTS
FOR (ec:EntityContent) REQUIRE (ec.locale_key, ec.slug) IS UNIQUE;

// Lookup by parent entity
CREATE INDEX entity_content_entity_key IF NOT EXISTS
FOR (ec:EntityContent) ON (ec.entity_key);

// Lookup by locale
CREATE INDEX entity_content_locale_key IF NOT EXISTS
FOR (ec:EntityContent) ON (ec.locale_key);

// URL routing (CRITICAL)
CREATE INDEX entity_content_full_path IF NOT EXISTS
FOR (ec:EntityContent) ON (ec.full_path);

// Composite for common pattern
CREATE INDEX entity_content_entity_locale IF NOT EXISTS
FOR (ec:EntityContent) ON (ec.entity_key, ec.locale_key);

// ═══════════════════════════════════════════════════════════════════════════
// INDEXES FOR SEOKeyword
// ═══════════════════════════════════════════════════════════════════════════

CREATE CONSTRAINT seo_keyword_key IF NOT EXISTS
FOR (sk:SEOKeyword) REQUIRE sk.key IS UNIQUE;

CREATE INDEX seo_keyword_slug IF NOT EXISTS
FOR (sk:SEOKeyword) ON (sk.slug);

CREATE INDEX seo_keyword_locale IF NOT EXISTS
FOR (sk:SEOKeyword) ON (sk.locale_key);
```

**Performance Impact**:

| Query | Before (no index) | After (indexed) | Improvement |
|-------|-------------------|-----------------|-------------|
| URL lookup by full_path | 100ms | <1ms | **100x** |
| Entity content by locale | 50ms | <1ms | **50x** |
| SEO keyword search | 200ms | 5ms | **40x** |

---

### Decision 12: Safe Migration Procedure (5 Phases)

**Status**: ✅ APPROVED

**Problem**: Migration without rollback strategy risks data loss.

**Decision**: Implement 5-phase safe migration.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  SAFE MIGRATION PROCEDURE                                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  PHASE 1: PREPARATION                                                       │
│  ├── Update YAML sources (node names, properties)                           │
│  ├── Run cargo run -- schema validate                                       │
│  ├── Run cargo run -- schema generate                                       │
│  └── Commit YAML changes                                                    │
│                                                                             │
│  PHASE 2: PRE-MIGRATION VALIDATION                                          │
│  ├── Count nodes by label (baseline)                                        │
│  ├── Check for NULL entity_key or locale_key                                │
│  ├── Check for duplicate (entity_key, locale_key) pairs                     │
│  ├── Verify all nodes have parent relationships                             │
│  └── Export counts to migration_baseline.json                               │
│                                                                             │
│  PHASE 3: BACKUP                                                            │
│  ├── neo4j-admin dump --database=neo4j --to=/backup/pre-v10.9.dump          │
│  └── Store backup with timestamp                                            │
│                                                                             │
│  PHASE 4: EXECUTE MIGRATION                                                 │
│  ├── Step 1: Rename labels (atomic per statement)                           │
│  │   MATCH (n:EntityL10n) SET n:EntityContent REMOVE n:EntityL10n;          │
│  │                                                                          │
│  ├── Step 2: Add constraints BEFORE key assignment                          │
│  │   CREATE CONSTRAINT entity_content_key ...                               │
│  │                                                                          │
│  ├── Step 3: Generate composite keys                                        │
│  │   MATCH (ec:EntityContent) WHERE ec.key IS NULL                          │
│  │   SET ec.key = 'entity:' + ec.entity_key + '@' + ec.locale_key;          │
│  │                                                                          │
│  ├── Step 4: Migrate relationships                                          │
│  │   MATCH (e:Entity)-[r:HAS_L10N]->(ec:EntityContent)                      │
│  │   CREATE (e)-[:HAS_CONTENT]->(ec) DELETE r;                              │
│  │                                                                          │
│  └── Step 5: Add indexes                                                    │
│      CREATE INDEX entity_content_full_path ...                              │
│                                                                             │
│  PHASE 5: POST-MIGRATION VALIDATION                                         │
│  ├── Count nodes (must equal baseline)                                      │
│  ├── Verify no NULL keys                                                    │
│  ├── Verify all keys match format                                           │
│  ├── Verify no old labels remain                                            │
│  ├── Compare relationship counts                                            │
│  └── Run TUI to verify display                                              │
│                                                                             │
│  ROLLBACK (if validation fails):                                            │
│  ├── neo4j-admin load --database=neo4j --from=/backup/pre-v10.9.dump        │
│  └── Investigate failure, fix, retry                                        │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Pre-Migration Validation Queries**:
```cypher
// Baseline counts
MATCH (n:EntityL10n) RETURN COUNT(n) AS entity_l10n_count;
MATCH (n:ProjectL10n) RETURN COUNT(n) AS project_l10n_count;
MATCH ()-[r:HAS_L10N]->() RETURN COUNT(r) AS has_l10n_count;

// Data quality checks
MATCH (n:EntityL10n) WHERE n.entity_key IS NULL RETURN COUNT(n) AS null_entity_key;
MATCH (n:EntityL10n) WHERE n.locale_key IS NULL RETURN COUNT(n) AS null_locale_key;

// Duplicate detection
MATCH (n:EntityL10n)
WITH n.entity_key AS ek, n.locale_key AS lk, COUNT(*) AS cnt
WHERE cnt > 1
RETURN ek, lk, cnt AS duplicates;
```

**Post-Migration Validation Queries**:
```cypher
// Verify migration completeness
MATCH (n:EntityL10n) RETURN COUNT(n) AS remaining_old_labels;  // Should be 0
MATCH (n:EntityContent) RETURN COUNT(n) AS new_label_count;     // Should match baseline
MATCH (n:EntityContent) WHERE n.key IS NULL RETURN COUNT(n);    // Should be 0
MATCH (n:EntityContent) WHERE NOT n.key STARTS WITH 'entity:' RETURN COUNT(n);  // Should be 0
```

---

## Open Questions (Resolved)

### ~~1. Multiple pages per Entity~~ → Decision 9: Semantic Decomposition
### ~~2. Depth limits for [:CONTAINS]~~ → Decision 5: Max Depth = 3
### ~~3. Circular references~~ → Decision 1: HAS_CHILD with cycle_protection

---

## Migration Impact

### Files to Update

**Node kinds (4 renames)**:
- `entity-l10n.yaml` → `entity-content.yaml`
- `project-l10n.yaml` → `project-content.yaml`
- `block-l10n.yaml` → `block-generated.yaml`
- `page-l10n.yaml` → `page-generated.yaml`

**Arc kinds (2 renames)**:
- `has-l10n.yaml` → split into `has-content.yaml` + `has-generated.yaml`

**Seed files**:
- `11-entityl10n-fr-fr.cypher` → regenerate with new node name + key format

**Documentation**:
- All Mermaid diagrams referencing L10n nodes
- CLAUDE.md terminology section

### Neo4j Migration

```cypher
// Rename labels
MATCH (n:EntityL10n) SET n:EntityContent REMOVE n:EntityL10n;
MATCH (n:ProjectL10n) SET n:ProjectContent REMOVE n:ProjectL10n;
MATCH (n:BlockL10n) SET n:BlockGenerated REMOVE n:BlockL10n;
MATCH (n:PageL10n) SET n:PageGenerated REMOVE n:PageL10n;

// Update keys to new format
MATCH (n:EntityContent)
SET n.key = 'entity:' + n.entity_key + '@' + n.locale_key;

// Update relationship types
MATCH (e:Entity)-[r:HAS_L10N]->(c:EntityContent)
CREATE (e)-[:HAS_CONTENT]->(c)
DELETE r;
```

---

## Semantic Connection: Slugification

**Decision**: Option A - Via Locale (normalized, 2 hops)

```
EntityContent ──[:FOR_LOCALE]──► Locale ──[:HAS_SLUGIFICATION]──► Slugification
     │                                                                │
     │                                                                ├── slug_rule
     │                                                                ├── preserve_diacritics
     │                                                                ├── stop_words
     │                                                                └── ...
     │
     └── slug (generated once at creation, stored)
```

**Workflow**:
1. Create EntityContent for locale X
2. Fetch: `Locale(X) → Slugification` → get rules
3. Apply rules → generate slug
4. Store slug on EntityContent
5. Runtime: use slug directly (no need to re-fetch rules)

**Why not direct link?**
- Slugification belongs to Locale, not EntityContent
- Slug is generated ONCE at creation, not at every query
- 2 hops acceptable for rare operation (content creation)
- Runtime queries use the stored slug, no rules needed

---

## Resolved Questions

1. **Slug in SEOKeyword key**: ✅ RESOLVED
   - Key = ASCII: `seo:creer-qr-code-gratuit@fr-FR`
   - Slug = UTF-8: `créer-qr-code-gratuit`
   - Rule: Key is technical (ASCII), slug is public (UTF-8)

2. **Trait for output layer**: ✅ RESOLVED
   - BlockGenerated, PageGenerated → `trait: derived` (was `localized`)
   - Reason: They are computed/aggregated outputs, not locale variants

3. **ProjectContent key**: ✅ RESOLVED
   - ADD key property: `project:qrcode-ai@fr-FR`
   - Consistency with EntityContent pattern

4. **Backward compatibility**: ✅ RESOLVED
   - NONE (v0 = clean design)
   - Clean up all legacy/backward compat code

---

## Implementation Checklist

### Phase 1: YAML Updates (Source of Truth)

- [ ] Rename `entity-l10n.yaml` → `entity-content.yaml`
- [ ] Rename `project-l10n.yaml` → `project-content.yaml`
- [ ] Rename `block-l10n.yaml` → `block-generated.yaml`
- [ ] Rename `page-l10n.yaml` → `page-generated.yaml`
- [ ] Update node names inside each YAML
- [ ] Change trait to `derived` for BlockGenerated, PageGenerated
- [ ] Add `key` property to ProjectContent
- [ ] Create `has-content.yaml` arc (for semantic layer)
- [ ] Create `has-generated.yaml` arc (for output layer)
- [ ] Run `cargo run -- schema validate`

### Phase 2: Generate Artifacts

- [ ] Run `cargo run -- schema generate`
- [ ] Verify TypeScript types regenerated
- [ ] Verify Mermaid diagrams regenerated
- [ ] Run `cargo test` (expect TUI failures)

### Phase 3: Rust TUI Updates

- [ ] Rename structs: `PageL10nData` → `PageGeneratedData`, etc.
- [ ] Update Cypher queries (3 queries in `data.rs`)
- [ ] Update guide strings and help text
- [ ] Update test fixtures
- [ ] Run `cargo test` (should pass)

### Phase 4: TypeScript Updates

- [ ] Rename interfaces: `EntityL10n` → `EntityContent`, etc.
- [ ] Update filter methods if needed
- [ ] Run `pnpm type-check`

### Phase 5: Seed Data Regeneration

- [ ] Verify EntityContent slugs have accents (fr-FR)
- [ ] Verify semantic order ("qr code" not "code qr")
- [ ] Update SEOKeyword key format to ASCII
- [ ] Regenerate `11-entitycontent-fr-fr.cypher`
- [ ] Regenerate `12-seokeyword-fr-fr.cypher`
- [ ] Regenerate `31-project-qrcode-ai.cypher`

### Phase 6: Neo4j Migration

- [ ] Write migration Cypher script
- [ ] Test on local Neo4j
- [ ] Run `pnpm infra:reset && pnpm infra:seed`
- [ ] Verify TUI displays correctly

### Phase 7: Documentation

- [ ] Update `.claude/rules/novanet-terminology.md`
- [ ] Update `.claude/rules/novanet-decisions.md` (add ADR-014)
- [ ] Update active design docs
- [ ] Clean up legacy references

### Phase 8: Cleanup

- [ ] Remove backward compat code
- [ ] Remove legacy files in `_legacy/` if obsolete
- [ ] Final `cargo test && pnpm test`
