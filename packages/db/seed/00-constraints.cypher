// NovaNet Constraints v9.0.0
//
// Schema definitions for Neo4j graph database.
// Uses IF NOT EXISTS for idempotent execution.
//
// NOTE: Locale-based filtering uses :FOR_LOCALE relation traversal (not property indexes).
// NOTE: Removed in v7.2.5: Audience, AudienceL10n, ValuePropL10n, SocialProofL10n
// NOTE: v7.8.2: Renamed SEOKeyword → SEOKeywordL10n

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
CREATE CONSTRAINT concept_key IF NOT EXISTS FOR (c:Concept) REQUIRE c.key IS UNIQUE;
CREATE INDEX cl10n_version IF NOT EXISTS FOR (cl:ConceptL10n) ON (cl.version);

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

CREATE INDEX seo_volume IF NOT EXISTS FOR (s:SEOKeywordL10n) ON (s.volume);
CREATE INDEX seo_intent IF NOT EXISTS FOR (s:SEOKeywordL10n) ON (s.intent);
CREATE INDEX seo_difficulty IF NOT EXISTS FOR (s:SEOKeywordL10n) ON (s.difficulty);
CREATE TEXT INDEX seo_value_text IF NOT EXISTS FOR (s:SEOKeywordL10n) ON (s.value);
// REMOVED v7.8.4: SEOVariation indexes (node deleted, variations are SEOKeywordL10n nodes)
CREATE INDEX seomr_status IF NOT EXISTS FOR (smr:SEOMiningRun) ON (smr.status);
// v7.8.5: SEOSnapshot → SEOKeywordMetrics
CREATE INDEX seokm_observed IF NOT EXISTS FOR (skm:SEOKeywordMetrics) ON (skm.observed_at);
CREATE INDEX seokm_source IF NOT EXISTS FOR (skm:SEOKeywordMetrics) ON (skm.source);

// ═══════════════════════════════════════════════════════════════════════════════
// GEO STRUCTURE (v7.8.5: GEOCitation → GEOSeedMetrics)
// ═══════════════════════════════════════════════════════════════════════════════

CREATE INDEX geo_format IF NOT EXISTS FOR (g:GEOSeedL10n) ON (g.format);
CREATE INDEX geo_intent IF NOT EXISTS FOR (g:GEOSeedL10n) ON (g.intent);
CREATE INDEX geo_last_mined IF NOT EXISTS FOR (g:GEOSeedL10n) ON (g.last_mined_at);
// REMOVED v7.8.4: GEOReformulation indexes (node deleted, reformulations are GEOSeedL10n nodes)
// v7.8.5: GEOCitation → GEOSeedMetrics
CREATE INDEX geosm_observed IF NOT EXISTS FOR (gsm:GEOSeedMetrics) ON (gsm.observed_at);
CREATE INDEX geosm_platform IF NOT EXISTS FOR (gsm:GEOSeedMetrics) ON (gsm.platform);
CREATE INDEX geosm_cited IF NOT EXISTS FOR (gsm:GEOSeedMetrics) ON (gsm.cited);
CREATE INDEX geomr_status IF NOT EXISTS FOR (gmr:GEOMiningRun) ON (gmr.status);

// ═══════════════════════════════════════════════════════════════════════════════
// RELATIONSHIP INDEXES
// ═══════════════════════════════════════════════════════════════════════════════

CREATE INDEX has_block_position IF NOT EXISTS FOR ()-[r:HAS_BLOCK]-() ON (r.position);
CREATE INDEX sl_temp IF NOT EXISTS FOR ()-[r:SEMANTIC_LINK]-() ON (r.temperature);
CREATE INDEX uc_temp IF NOT EXISTS FOR ()-[r:USES_CONCEPT]-() ON (r.temperature);
CREATE INDEX tseo_status IF NOT EXISTS FOR ()-[r:TARGETS_SEO]-() ON (r.status);
CREATE INDEX tgeo_status IF NOT EXISTS FOR ()-[r:TARGETS_GEO]-() ON (r.status);
// REMOVED v7.8.1: PAGE_TARGETS_SEO and PAGE_TARGETS_GEO indexes
// Reason: Direct Page -> SEO/GEO bypasses semantic grouping
// Correct flow: Page -> Concept -> ConceptL10n -> SEOKeywordL10n/GEOSeedL10n
CREATE INDEX infl_weight IF NOT EXISTS FOR ()-[r:INFLUENCED_BY]-() ON (r.weight);
// REMOVED v7.9.0: USED_SEO_KEYWORD, USED_GEO_SEED indexes (relations removed)
// SEO/GEO provenance is implicit via: BlockL10n → INFLUENCED_BY → ConceptL10n → HAS_*_TARGET → SEO/GEO
CREATE INDEX gen_date IF NOT EXISTS FOR ()-[r:GENERATED]-() ON (r.generated_at);

// ═══════════════════════════════════════════════════════════════════════════════
// META-GRAPH (v9.0.0)
// Faceted classification: Realm, Layer, Kind, Trait, EdgeFamily, EdgeKind
// ═══════════════════════════════════════════════════════════════════════════════

CREATE CONSTRAINT realm_key IF NOT EXISTS FOR (r:Realm) REQUIRE r.key IS UNIQUE;
CREATE CONSTRAINT layer_key IF NOT EXISTS FOR (l:Layer) REQUIRE l.key IS UNIQUE;
CREATE CONSTRAINT kind_label IF NOT EXISTS FOR (k:Kind) REQUIRE k.label IS UNIQUE;
CREATE CONSTRAINT trait_key IF NOT EXISTS FOR (t:Trait) REQUIRE t.key IS UNIQUE;
CREATE CONSTRAINT edgefamily_key IF NOT EXISTS FOR (ef:EdgeFamily) REQUIRE ef.key IS UNIQUE;
CREATE CONSTRAINT edgekind_type IF NOT EXISTS FOR (ek:EdgeKind) REQUIRE ek.type IS UNIQUE;
