// NovaNet Constraints v0.13.0
//
// Schema definitions for Neo4j graph database.
// Uses IF NOT EXISTS for idempotent execution.
//
// NOTE: Locale-based filtering uses :FOR_LOCALE relation traversal (not property indexes).
// v10.4: Entity-Centric Architecture (Entity/EntityNative), GEO layer removed, 2 realms (global, project)
// v10.9: Naming convention refactor (EntityL10n→EntityNative, HAS_L10N→HAS_CONTENT, etc.)
// v0.13.0 ADR-029: *Native pattern (EntityNative→EntityNative, HAS_CONTENT→HAS_NATIVE, etc.)

// ═══════════════════════════════════════════════════════════════════════════════
// LOCALE
// ═══════════════════════════════════════════════════════════════════════════════

CREATE CONSTRAINT locale_key IF NOT EXISTS FOR (l:Locale) REQUIRE l.key IS UNIQUE;
CREATE INDEX locale_language IF NOT EXISTS FOR (l:Locale) ON (l.language_code);
CREATE INDEX locale_country IF NOT EXISTS FOR (l:Locale) ON (l.country_code);
// v10.7: Geographic clustering indexes for LLM retrieval
CREATE INDEX locale_region IF NOT EXISTS FOR (l:Locale) ON (l.region);
CREATE INDEX locale_language_family IF NOT EXISTS FOR (l:Locale) ON (l.language_family);
CREATE INDEX locale_script IF NOT EXISTS FOR (l:Locale) ON (l.script);
CREATE INDEX locale_text_direction IF NOT EXISTS FOR (l:Locale) ON (l.text_direction);

// ═══════════════════════════════════════════════════════════════════════════════
// CORE ENTITIES
// ═══════════════════════════════════════════════════════════════════════════════

CREATE CONSTRAINT project_key IF NOT EXISTS FOR (p:Project) REQUIRE p.key IS UNIQUE;
// v10.3: Entity replaces Concept
CREATE CONSTRAINT entity_key IF NOT EXISTS FOR (e:Entity) REQUIRE e.key IS UNIQUE;
// v11: Entity type filtering for semantic queries
CREATE INDEX entity_type IF NOT EXISTS FOR (e:Entity) ON (e.type);
// v11: Pillar filtering for content hierarchy navigation
CREATE INDEX entity_is_pillar IF NOT EXISTS FOR (e:Entity) ON (e.is_pillar);

// v10.9.0: EntityNative indexes (Decision 11 - naming convention refactor)
CREATE CONSTRAINT entity_content_key IF NOT EXISTS FOR (ec:EntityNative) REQUIRE ec.key IS UNIQUE;
CREATE CONSTRAINT entity_content_slug_unique IF NOT EXISTS FOR (ec:EntityNative) REQUIRE (ec.locale_key, ec.slug) IS UNIQUE;
CREATE INDEX entity_content_entity_key IF NOT EXISTS FOR (ec:EntityNative) ON (ec.entity_key);
CREATE INDEX entity_content_locale_key IF NOT EXISTS FOR (ec:EntityNative) ON (ec.locale_key);
CREATE INDEX entity_content_full_path IF NOT EXISTS FOR (ec:EntityNative) ON (ec.full_path);
CREATE INDEX entity_content_slug IF NOT EXISTS FOR (ec:EntityNative) ON (ec.slug);
CREATE INDEX entity_content_version IF NOT EXISTS FOR (ec:EntityNative) ON (ec.version);

// ═══════════════════════════════════════════════════════════════════════════════
// PROJECT NODES (v7.2.5)
// ═══════════════════════════════════════════════════════════════════════════════

// v0.12.4 ADR-028: Brand Architecture (BrandIdentity → Brand)
CREATE CONSTRAINT brand_key IF NOT EXISTS FOR (b:Brand) REQUIRE b.key IS UNIQUE;
CREATE CONSTRAINT branddesign_key IF NOT EXISTS FOR (bd:BrandDesign) REQUIRE bd.key IS UNIQUE;
CREATE CONSTRAINT brandprinciples_key IF NOT EXISTS FOR (bp:BrandPrinciples) REQUIRE bp.key IS UNIQUE;
CREATE CONSTRAINT promptstyle_key IF NOT EXISTS FOR (ps:PromptStyle) REQUIRE ps.key IS UNIQUE;
// v10.9.0: ProjectL10n renamed to ProjectNative (Decision 11)
CREATE INDEX projectcontent_updated IF NOT EXISTS FOR (pc:ProjectNative) ON (pc.updated_at);

// ═══════════════════════════════════════════════════════════════════════════════
// PAGE STRUCTURE
// ═══════════════════════════════════════════════════════════════════════════════

CREATE CONSTRAINT page_key IF NOT EXISTS FOR (p:Page) REQUIRE p.key IS UNIQUE;
CREATE INDEX po_date IF NOT EXISTS FOR (po:PageNative) ON (po.assembled_at);
// v8.1.0 REMOVED: PageMetrics (YAGNI - no time-series metrics needed)

// ═══════════════════════════════════════════════════════════════════════════════
// BLOCK STRUCTURE
// ═══════════════════════════════════════════════════════════════════════════════

CREATE CONSTRAINT blocktype_key IF NOT EXISTS FOR (bt:BlockType) REQUIRE bt.key IS UNIQUE;
CREATE CONSTRAINT block_key IF NOT EXISTS FOR (b:Block) REQUIRE b.key IS UNIQUE;
// v10.9: Block order index for TUI ORDER BY optimization
CREATE INDEX block_order IF NOT EXISTS FOR (b:Block) ON (b.order);
CREATE INDEX bo_date IF NOT EXISTS FOR (bo:BlockNative) ON (bo.generated_at);
// v7.8.5: BlockNative replaces BlockOutput

// ═══════════════════════════════════════════════════════════════════════════════
// INSTRUCTIONS (v0.13.1 ADR-025: BlockInstruction, BlockRules, BlockType, PromptArtifact only)
// Note: PageInstruction does NOT exist in v0.13.1 schema
// ═══════════════════════════════════════════════════════════════════════════════

CREATE INDEX blockinstruction_active IF NOT EXISTS FOR (bi:BlockInstruction) ON (bi.active);
CREATE INDEX blockinstruction_version IF NOT EXISTS FOR (bi:BlockInstruction) ON (bi.version);
CREATE INDEX blockrules_active IF NOT EXISTS FOR (br:BlockRules) ON (br.active);
CREATE INDEX blockrules_version IF NOT EXISTS FOR (br:BlockRules) ON (br.version);

// ═══════════════════════════════════════════════════════════════════════════════
// SEO STRUCTURE (v7.8.5: SEOSnapshot → SEOKeywordMetrics)
// ═══════════════════════════════════════════════════════════════════════════════

CREATE INDEX seo_volume IF NOT EXISTS FOR (s:SEOKeyword) ON (s.volume);
CREATE INDEX seo_intent IF NOT EXISTS FOR (s:SEOKeyword) ON (s.intent);
CREATE INDEX seo_difficulty IF NOT EXISTS FOR (s:SEOKeyword) ON (s.difficulty);
// v11: Traffic potential ranking for content gap analysis
CREATE INDEX seo_traffic_potential IF NOT EXISTS FOR (s:SEOKeyword) ON (s.traffic_potential);
CREATE TEXT INDEX seo_value_text IF NOT EXISTS FOR (s:SEOKeyword) ON (s.value);
// REMOVED v7.8.4: SEOVariation indexes (node deleted, variations are SEOKeyword nodes)
// REMOVED v11.2: SEOMiningRun index (node deleted, job concept deferred)
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
// Correct flow: Page -> Entity -> EntityNative -> SEOKeyword
CREATE INDEX infl_weight IF NOT EXISTS FOR ()-[r:INFLUENCED_BY]-() ON (r.weight);
// SEO provenance is implicit via: BlockNative → INFLUENCED_BY → EntityNative
CREATE INDEX gen_date IF NOT EXISTS FOR ()-[r:GENERATED]-() ON (r.generated_at);

// ═══════════════════════════════════════════════════════════════════════════════
// SCHEMA-GRAPH (v0.12.0 ADR-023)
// Faceted classification: Realm, Layer, Class, Trait, ArcFamily, ArcClass
// ═══════════════════════════════════════════════════════════════════════════════

CREATE CONSTRAINT realm_key IF NOT EXISTS FOR (r:Realm) REQUIRE r.key IS UNIQUE;
CREATE CONSTRAINT layer_key IF NOT EXISTS FOR (l:Layer) REQUIRE l.key IS UNIQUE;
// v10.9: Layer order index for TUI ORDER BY optimization
CREATE INDEX layer_order IF NOT EXISTS FOR (l:Layer) ON (l.order);
CREATE CONSTRAINT class_label IF NOT EXISTS FOR (c:Class) REQUIRE c.label IS UNIQUE;
CREATE CONSTRAINT trait_key IF NOT EXISTS FOR (t:Trait) REQUIRE t.key IS UNIQUE;
CREATE CONSTRAINT arcfamily_key IF NOT EXISTS FOR (af:ArcFamily) REQUIRE af.key IS UNIQUE;
CREATE CONSTRAINT arcclass_key IF NOT EXISTS FOR (ac:ArcClass) REQUIRE ac.key IS UNIQUE;

// ═══════════════════════════════════════════════════════════════════════════════
// KNOWLEDGE ATOMS (v10.7.0)
// Expression indexes for LLM context retrieval
// ═══════════════════════════════════════════════════════════════════════════════

CREATE CONSTRAINT expression_key IF NOT EXISTS FOR (e:Expression) REQUIRE e.key IS UNIQUE;
CREATE INDEX expr_semantic_field IF NOT EXISTS FOR (e:Expression) ON (e.semantic_field);
CREATE INDEX expr_register IF NOT EXISTS FOR (e:Expression) ON (e.register);
CREATE INDEX expr_locale_key IF NOT EXISTS FOR (e:Expression) ON (e.locale_key);
CREATE FULLTEXT INDEX expr_text_fulltext IF NOT EXISTS FOR (e:Expression) ON EACH [e.text, e.intention, e.context];

// ExpressionSet container (satellite node - no key constraint, identified by Locale→HAS_EXPRESSIONS→ExpressionSet)

// Culture and Market (already have key from locale)
CREATE CONSTRAINT culture_key IF NOT EXISTS FOR (c:Culture) REQUIRE c.key IS UNIQUE;
CREATE CONSTRAINT market_key IF NOT EXISTS FOR (m:Market) REQUIRE m.key IS UNIQUE;

// Formatting and Slugification
CREATE CONSTRAINT formatting_key IF NOT EXISTS FOR (f:Formatting) REQUIRE f.key IS UNIQUE;
CREATE CONSTRAINT slugification_key IF NOT EXISTS FOR (s:Slugification) REQUIRE s.key IS UNIQUE;

// =============================================================================
// MISSING KEY CONSTRAINTS (37 nodes)
// Added 2026-02-17 - Cypher audit cleanup
// =============================================================================

CREATE CONSTRAINT adaptation_key IF NOT EXISTS FOR (a:Adaptation) REQUIRE a.key IS UNIQUE;
CREATE CONSTRAINT audiencepersona_key IF NOT EXISTS FOR (ap:AudiencePersona) REQUIRE ap.key IS UNIQUE;
CREATE CONSTRAINT audiencetrait_key IF NOT EXISTS FOR (at:AudienceTrait) REQUIRE at.key IS UNIQUE;
CREATE CONSTRAINT blockinstruction_key IF NOT EXISTS FOR (bi:BlockInstruction) REQUIRE bi.key IS UNIQUE;
CREATE CONSTRAINT blocknative_key IF NOT EXISTS FOR (bn:BlockNative) REQUIRE bn.key IS UNIQUE;
CREATE CONSTRAINT channelsurface_key IF NOT EXISTS FOR (cs:ChannelSurface) REQUIRE cs.key IS UNIQUE;
CREATE CONSTRAINT contentslot_key IF NOT EXISTS FOR (cs:ContentSlot) REQUIRE cs.key IS UNIQUE;
CREATE CONSTRAINT continent_key IF NOT EXISTS FOR (c:Continent) REQUIRE c.key IS UNIQUE;
CREATE CONSTRAINT country_key IF NOT EXISTS FOR (c:Country) REQUIRE c.key IS UNIQUE;
CREATE CONSTRAINT culturalrealm_key IF NOT EXISTS FOR (cr:CulturalRealm) REQUIRE cr.key IS UNIQUE;
CREATE CONSTRAINT culturalsubrealm_key IF NOT EXISTS FOR (csr:CulturalSubRealm) REQUIRE csr.key IS UNIQUE;
CREATE CONSTRAINT cultureref_key IF NOT EXISTS FOR (cr:CultureRef) REQUIRE cr.key IS UNIQUE;
CREATE CONSTRAINT economicregion_key IF NOT EXISTS FOR (er:EconomicRegion) REQUIRE er.key IS UNIQUE;
CREATE CONSTRAINT entitycategory_key IF NOT EXISTS FOR (ec:EntityCategory) REQUIRE ec.key IS UNIQUE;
CREATE CONSTRAINT geoanswer_key IF NOT EXISTS FOR (ga:GEOAnswer) REQUIRE ga.key IS UNIQUE;
CREATE CONSTRAINT geoquery_key IF NOT EXISTS FOR (gq:GEOQuery) REQUIRE gq.key IS UNIQUE;
CREATE CONSTRAINT geoqueryset_key IF NOT EXISTS FOR (gqs:GEOQuerySet) REQUIRE gqs.key IS UNIQUE;
CREATE CONSTRAINT georegion_key IF NOT EXISTS FOR (gr:GeoRegion) REQUIRE gr.key IS UNIQUE;
CREATE CONSTRAINT geosubregion_key IF NOT EXISTS FOR (gsr:GeoSubRegion) REQUIRE gsr.key IS UNIQUE;
CREATE CONSTRAINT incomegroup_key IF NOT EXISTS FOR (ig:IncomeGroup) REQUIRE ig.key IS UNIQUE;
CREATE CONSTRAINT languagebranch_key IF NOT EXISTS FOR (lb:LanguageBranch) REQUIRE lb.key IS UNIQUE;
CREATE CONSTRAINT languagefamily_key IF NOT EXISTS FOR (lf:LanguageFamily) REQUIRE lf.key IS UNIQUE;
CREATE CONSTRAINT lendingcategory_key IF NOT EXISTS FOR (lc:LendingCategory) REQUIRE lc.key IS UNIQUE;
CREATE CONSTRAINT orgconfig_key IF NOT EXISTS FOR (oc:OrgConfig) REQUIRE oc.key IS UNIQUE;
CREATE CONSTRAINT outputartifact_key IF NOT EXISTS FOR (oa:OutputArtifact) REQUIRE oa.key IS UNIQUE;
CREATE CONSTRAINT pagenative_key IF NOT EXISTS FOR (pn:PageNative) REQUIRE pn.key IS UNIQUE;
CREATE CONSTRAINT pattern_key IF NOT EXISTS FOR (p:Pattern) REQUIRE p.key IS UNIQUE;
CREATE CONSTRAINT populationcluster_key IF NOT EXISTS FOR (pc:PopulationCluster) REQUIRE pc.key IS UNIQUE;
CREATE CONSTRAINT populationsubcluster_key IF NOT EXISTS FOR (psc:PopulationSubCluster) REQUIRE psc.key IS UNIQUE;
CREATE CONSTRAINT promptartifact_key IF NOT EXISTS FOR (pa:PromptArtifact) REQUIRE pa.key IS UNIQUE;
CREATE CONSTRAINT seokeyword_key IF NOT EXISTS FOR (kw:SEOKeyword) REQUIRE kw.key IS UNIQUE;
CREATE CONSTRAINT seokeywordformat_key IF NOT EXISTS FOR (kwf:SEOKeywordFormat) REQUIRE kwf.key IS UNIQUE;
CREATE CONSTRAINT seokeywordmetrics_key IF NOT EXISTS FOR (kwm:SEOKeywordMetrics) REQUIRE kwm.key IS UNIQUE;
CREATE CONSTRAINT seokeywordset_key IF NOT EXISTS FOR (kws:SEOKeywordSet) REQUIRE kws.key IS UNIQUE;
CREATE CONSTRAINT style_key IF NOT EXISTS FOR (s:Style) REQUIRE s.key IS UNIQUE;
CREATE CONSTRAINT taboo_key IF NOT EXISTS FOR (t:Taboo) REQUIRE t.key IS UNIQUE;
CREATE CONSTRAINT term_key IF NOT EXISTS FOR (t:Term) REQUIRE t.key IS UNIQUE;
