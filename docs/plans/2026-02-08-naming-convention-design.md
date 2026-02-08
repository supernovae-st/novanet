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

### Open Questions (To Explore)

1. **Multiple pages per Entity**:
   - Landing page: `/fr/créer-qr-code`
   - Comparison page: `/fr/créer-qr-code-vs-barcode`
   - How to differentiate? Page.type? Separate Entity?
   - **Decision**: TBD

2. **Depth limits for [:CONTAINS]**:
   - Max nesting? /blog/category/subcategory/article?
   - Performance implications of deep traversal?
   - **Decision**: TBD

3. **Circular references**:
   - What if Entity A [:CONTAINS] B and B [:CONTAINS] A?
   - Need constraint? Or handle in URL builder?
   - **Decision**: TBD

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
