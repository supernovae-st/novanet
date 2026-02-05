// NovaNet Constraints v10.4.0
//
// Schema definitions for Neo4j graph database.
// Uses IF NOT EXISTS for idempotent execution.
//
// NOTE: Locale-based filtering uses :FOR_LOCALE relation traversal (not property indexes).
// v10.4: Entity-Centric Architecture (Entity/EntityL10n), GEO layer removed, 2 realms (global, project)

// ═══════════════════════════════════════════════════════════════════════════════
// LOCALE
// ═══════════════════════════════════════════════════════════════════════════════

CREATE CONSTRAINT locale_key IF NOT EXISTS FOR (l:Locale) REQUIRE l.key IS UNIQUE;
CREATE INDEX locale_language IF NOT EXISTS FOR (l:Locale) ON (l.language_code);
CREATE INDEX locale_country IF NOT EXISTS FOR (l:Locale) ON (l.country_code);

// ═══════════════════════════════════════════════════════════════════════════════
// CORE ENTITIES
// ═══════════════════════════════════════════════════════════════════════════════

CREATE CONSTRAINT project_key IF NOT EXISTS FOR (p:Project) REQUIRE p.key IS UNIQUE;
// v10.3: Entity replaces Concept
CREATE CONSTRAINT entity_key IF NOT EXISTS FOR (e:Entity) REQUIRE e.key IS UNIQUE;
CREATE INDEX el10n_version IF NOT EXISTS FOR (el:EntityL10n) ON (el.version);

// ═══════════════════════════════════════════════════════════════════════════════
// PROJECT NODES (v7.2.5)
// ═══════════════════════════════════════════════════════════════════════════════

CREATE CONSTRAINT brandidentity_key IF NOT EXISTS FOR (bi:BrandIdentity) REQUIRE bi.key IS UNIQUE;
CREATE INDEX projectl10n_updated IF NOT EXISTS FOR (pl:ProjectL10n) ON (pl.updated_at);

// ═══════════════════════════════════════════════════════════════════════════════
// PAGE STRUCTURE
// ═══════════════════════════════════════════════════════════════════════════════

CREATE CONSTRAINT page_key IF NOT EXISTS FOR (p:Page) REQUIRE p.key IS UNIQUE;
CREATE INDEX po_date IF NOT EXISTS FOR (po:PageL10n) ON (po.assembled_at);
// v8.1.0 REMOVED: PageMetrics (YAGNI - no time-series metrics needed)

// ═══════════════════════════════════════════════════════════════════════════════
// BLOCK STRUCTURE
// ═══════════════════════════════════════════════════════════════════════════════

CREATE CONSTRAINT blocktype_key IF NOT EXISTS FOR (bt:BlockType) REQUIRE bt.key IS UNIQUE;
CREATE CONSTRAINT block_key IF NOT EXISTS FOR (b:Block) REQUIRE b.key IS UNIQUE;
CREATE INDEX bo_date IF NOT EXISTS FOR (bo:BlockL10n) ON (bo.generated_at);
// v7.8.5: BlockL10n replaces BlockOutput

// ═══════════════════════════════════════════════════════════════════════════════
// PROMPTS (v7.2.0)
// ═══════════════════════════════════════════════════════════════════════════════

CREATE INDEX pageprompt_active IF NOT EXISTS FOR (pp:PagePrompt) ON (pp.active);
CREATE INDEX pageprompt_version IF NOT EXISTS FOR (pp:PagePrompt) ON (pp.version);
CREATE INDEX blockprompt_active IF NOT EXISTS FOR (bp:BlockPrompt) ON (bp.active);
CREATE INDEX blockprompt_version IF NOT EXISTS FOR (bp:BlockPrompt) ON (bp.version);
CREATE INDEX blockrules_active IF NOT EXISTS FOR (br:BlockRules) ON (br.active);
CREATE INDEX blockrules_version IF NOT EXISTS FOR (br:BlockRules) ON (br.version);

// ═══════════════════════════════════════════════════════════════════════════════
// SEO STRUCTURE (v7.8.5: SEOSnapshot → SEOKeywordMetrics)
// ═══════════════════════════════════════════════════════════════════════════════

CREATE INDEX seo_volume IF NOT EXISTS FOR (s:SEOKeyword) ON (s.volume);
CREATE INDEX seo_intent IF NOT EXISTS FOR (s:SEOKeyword) ON (s.intent);
CREATE INDEX seo_difficulty IF NOT EXISTS FOR (s:SEOKeyword) ON (s.difficulty);
CREATE TEXT INDEX seo_value_text IF NOT EXISTS FOR (s:SEOKeyword) ON (s.value);
// REMOVED v7.8.4: SEOVariation indexes (node deleted, variations are SEOKeyword nodes)
CREATE INDEX seomr_status IF NOT EXISTS FOR (smr:SEOMiningRun) ON (smr.status);
// v7.8.5: SEOSnapshot → SEOKeywordMetrics
CREATE INDEX seokm_observed IF NOT EXISTS FOR (skm:SEOKeywordMetrics) ON (skm.observed_at);
CREATE INDEX seokm_source IF NOT EXISTS FOR (skm:SEOKeywordMetrics) ON (skm.source);

// ═══════════════════════════════════════════════════════════════════════════════
// RELATIONSHIP INDEXES
// ═══════════════════════════════════════════════════════════════════════════════

CREATE INDEX has_block_position IF NOT EXISTS FOR ()-[r:HAS_BLOCK]-() ON (r.position);
CREATE INDEX sl_temp IF NOT EXISTS FOR ()-[r:SEMANTIC_LINK]-() ON (r.temperature);
// v10.3: USES_ENTITY replaces USES_CONCEPT
CREATE INDEX ue_temp IF NOT EXISTS FOR ()-[r:USES_ENTITY]-() ON (r.temperature);
// v10.3: EXPRESSES replaces TARGETS_SEO
CREATE INDEX expresses_status IF NOT EXISTS FOR ()-[r:EXPRESSES]-() ON (r.status);
// REMOVED v10.3: TARGETS_GEO (GEO layer removed)
// Correct flow: Page -> Entity -> EntityL10n -> SEOKeyword
CREATE INDEX infl_weight IF NOT EXISTS FOR ()-[r:INFLUENCED_BY]-() ON (r.weight);
// SEO provenance is implicit via: BlockL10n → INFLUENCED_BY → EntityL10n
CREATE INDEX gen_date IF NOT EXISTS FOR ()-[r:GENERATED]-() ON (r.generated_at);

// ═══════════════════════════════════════════════════════════════════════════════
// META-GRAPH (v9.5.0)
// Faceted classification: Realm, Layer, Kind, Trait, ArcFamily, ArcKind
// ═══════════════════════════════════════════════════════════════════════════════

CREATE CONSTRAINT realm_key IF NOT EXISTS FOR (r:Realm) REQUIRE r.key IS UNIQUE;
CREATE CONSTRAINT layer_key IF NOT EXISTS FOR (l:Layer) REQUIRE l.key IS UNIQUE;
CREATE CONSTRAINT kind_label IF NOT EXISTS FOR (k:Kind) REQUIRE k.label IS UNIQUE;
CREATE CONSTRAINT trait_key IF NOT EXISTS FOR (t:Trait) REQUIRE t.key IS UNIQUE;
CREATE CONSTRAINT arcfamily_key IF NOT EXISTS FOR (af:ArcFamily) REQUIRE af.key IS UNIQUE;
CREATE CONSTRAINT arckind_key IF NOT EXISTS FOR (ak:ArcKind) REQUIRE ak.key IS UNIQUE;
