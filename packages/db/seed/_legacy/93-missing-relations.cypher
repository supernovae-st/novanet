// NovaNet Missing Relations v8.2.1
// Creates examples of 8 relations defined in YAML but not yet seeded
//
// Relations added:
//   1. DEFAULT_LOCALE      - Project → Locale (singleton default)
//   2. VARIANT_OF          - Locale → Locale (regional inheritance)
//   3. LINKS_TO            - Page → Page (internal linking)
//   4. SUBTOPIC_OF         - Page → Page (pillar/cluster hierarchy)
//   5. ASSEMBLES           - PageL10n → BlockL10n (page assembly)
//   6. BELONGS_TO_PROJECT_L10N - PageL10n → ProjectL10n (locale context)
//   7. GENERATED_FROM      - BlockL10n → BlockType (structure source)
//   8. PREVIOUS_VERSION    - *L10n → *L10n (version history)

// ═══════════════════════════════════════════════════════════════════════════════
// 1. DEFAULT_LOCALE - Project's primary locale (exactly one per project)
// ═══════════════════════════════════════════════════════════════════════════════

MATCH (p:Project {key: "project-qrcode-ai"}), (l:Locale {key: "en-US"})
MERGE (p)-[:DEFAULT_LOCALE]->(l);

// ═══════════════════════════════════════════════════════════════════════════════
// 2. VARIANT_OF - Regional variant inheritance
// ═══════════════════════════════════════════════════════════════════════════════
// fr-CA is a variant of fr-FR (inherits French base, Canadian specifics)

MATCH (frCA:Locale {key: "fr-CA"}), (frFR:Locale {key: "fr-FR"})
MERGE (frCA)-[:VARIANT_OF]->(frFR);

// ═══════════════════════════════════════════════════════════════════════════════
// 3. LINKS_TO - Internal page linking with concept-based anchors
// ═══════════════════════════════════════════════════════════════════════════════
// Home page links to Pricing (CTA) and Features (feature highlight)

MATCH (home:Page {key: "page-home"}), (pricing:Page {key: "page-pricing"})
MERGE (home)-[:LINKS_TO {
  concept_key: "tier-pro",
  context: "pricing_cta",
  seo_weight: 0.9,
  anchor_type: "exact_match",
  nofollow: false
}]->(pricing);

MATCH (home:Page {key: "page-home"}), (features:Page {key: "page-features"})
MERGE (home)-[:LINKS_TO {
  concept_key: "action-create-qr",
  context: "feature_highlight",
  seo_weight: 0.8,
  anchor_type: "partial_match",
  nofollow: false
}]->(features);

// ═══════════════════════════════════════════════════════════════════════════════
// 4. SUBTOPIC_OF - Pillar/Cluster SEO hierarchy
// ═══════════════════════════════════════════════════════════════════════════════
// Features page is a subtopic of Home (pillar)
// Pricing page is a subtopic of Home (pillar)

MATCH (features:Page {key: "page-features"}), (home:Page {key: "page-home"})
MERGE (features)-[:SUBTOPIC_OF]->(home);

MATCH (pricing:Page {key: "page-pricing"}), (home:Page {key: "page-home"})
MERGE (pricing)-[:SUBTOPIC_OF]->(home);

// ═══════════════════════════════════════════════════════════════════════════════
// 5. ASSEMBLES - PageL10n assembles BlockL10ns with position
// ═══════════════════════════════════════════════════════════════════════════════

MATCH (pl:PageL10n {key: "pagel10n-home-en-v1"}), (bl:BlockL10n {key: "blockl10n-home-hero-en-v1"})
MERGE (pl)-[:ASSEMBLES {position: 1}]->(bl);

MATCH (pl:PageL10n {key: "pagel10n-home-en-v1"}), (bl:BlockL10n {key: "blockl10n-home-features-en-v1"})
MERGE (pl)-[:ASSEMBLES {position: 2}]->(bl);

MATCH (pl:PageL10n {key: "pagel10n-home-fr-v1"}), (bl:BlockL10n {key: "blockl10n-home-hero-fr-v1"})
MERGE (pl)-[:ASSEMBLES {position: 1}]->(bl);

// ═══════════════════════════════════════════════════════════════════════════════
// 6. BELONGS_TO_PROJECT_L10N - PageL10n belongs to ProjectL10n (same locale)
// ═══════════════════════════════════════════════════════════════════════════════

MATCH (pageL10n:PageL10n {key: "pagel10n-home-en-v1"}), (projL10n:ProjectL10n)-[:FOR_LOCALE]->(:Locale {key: "en-US"})
MERGE (pageL10n)-[:BELONGS_TO_PROJECT_L10N]->(projL10n);

MATCH (pageL10n:PageL10n {key: "pagel10n-home-fr-v1"}), (projL10n:ProjectL10n)-[:FOR_LOCALE]->(:Locale {key: "fr-FR"})
MERGE (pageL10n)-[:BELONGS_TO_PROJECT_L10N]->(projL10n);

// ═══════════════════════════════════════════════════════════════════════════════
// 7. GENERATED_FROM - BlockL10n was generated from BlockType template
// ═══════════════════════════════════════════════════════════════════════════════

MATCH (bl:BlockL10n {key: "blockl10n-home-hero-en-v1"}), (bt:BlockType {key: "blocktype-hero"})
MERGE (bl)-[:GENERATED_FROM]->(bt);

MATCH (bl:BlockL10n {key: "blockl10n-home-hero-fr-v1"}), (bt:BlockType {key: "blocktype-hero"})
MERGE (bl)-[:GENERATED_FROM]->(bt);

MATCH (bl:BlockL10n {key: "blockl10n-home-features-en-v1"}), (bt:BlockType {key: "blocktype-feature-grid"})
MERGE (bl)-[:GENERATED_FROM]->(bt);

MATCH (bl:BlockL10n {key: "blockl10n-pricing-table-en-v1"}), (bt:BlockType {key: "blocktype-pricing-table"})
MERGE (bl)-[:GENERATED_FROM]->(bt);

// ═══════════════════════════════════════════════════════════════════════════════
// 8. PREVIOUS_VERSION - Version history chain
// ═══════════════════════════════════════════════════════════════════════════════
// Create a v2 of home page (en-US) that links back to v1

MATCH (page:Page {key: "page-home"}), (locale:Locale {key: "en-US"})
CREATE (pl2:PageL10n {
  key: "pagel10n-home-en-v2",
  display_name: "Home Page (en-US v2)",
  description: "Updated home page content v2",
  llm_context: "USE: v2 of home page. TRIGGERS: home, landing. NOT: v1.",
  version: 2,
  status: "published",
  meta_title: "QR Code AI - Smart QR Code Generator",
  meta_description: "Create professional QR codes in seconds with AI-powered design and real-time analytics.",
  word_count: 850,
  generated_at: datetime(),
  published_at: datetime(),
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (page)-[:HAS_OUTPUT]->(pl2)
CREATE (pl2)-[:OUTPUT_OF]->(page)
CREATE (pl2)-[:FOR_LOCALE]->(locale);

// Link v2 back to v1
MATCH (v2:PageL10n {key: "pagel10n-home-en-v2"}), (v1:PageL10n {key: "pagel10n-home-en-v1"})
CREATE (v2)-[:PREVIOUS_VERSION]->(v1);

// Create a v2 of hero block (en-US) that links back to v1
MATCH (block:Block {key: "block-home-hero"}), (locale:Locale {key: "en-US"})
CREATE (bl2:BlockL10n {
  key: "blockl10n-home-hero-en-v2",
  display_name: "Hero Block (en-US v2)",
  description: "Updated hero section v2",
  llm_context: "USE: v2 hero content. TRIGGERS: hero, headline. NOT: v1.",
  version: 2,
  status: "published",
  content: "Create stunning QR codes in seconds. AI-powered design meets powerful analytics.",
  word_count: 45,
  generated_at: datetime(),
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (block)-[:HAS_OUTPUT]->(bl2)
CREATE (bl2)-[:OUTPUT_OF]->(block)
CREATE (bl2)-[:FOR_LOCALE]->(locale);

// Link v2 back to v1
MATCH (v2:BlockL10n {key: "blockl10n-home-hero-en-v2"}), (v1:BlockL10n {key: "blockl10n-home-hero-en-v1"})
CREATE (v2)-[:PREVIOUS_VERSION]->(v1);

// ═══════════════════════════════════════════════════════════════════════════════
// SUMMARY
// ═══════════════════════════════════════════════════════════════════════════════
// Created examples of all 8 missing relations:
//   - DEFAULT_LOCALE:        1 (Project → en-US)
//   - VARIANT_OF:            1 (fr-CA → fr-FR)
//   - LINKS_TO:              2 (Home → Pricing, Home → Features)
//   - SUBTOPIC_OF:           2 (Features → Home, Pricing → Home)
//   - ASSEMBLES:             3 (PageL10n → BlockL10n with position)
//   - BELONGS_TO_PROJECT_L10N: 2 (PageL10n → ProjectL10n)
//   - GENERATED_FROM:        4 (BlockL10n → BlockType)
//   - PREVIOUS_VERSION:      2 (v2 → v1 for Page and Block)
