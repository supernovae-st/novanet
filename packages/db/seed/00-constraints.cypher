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
// INSTRUCTIONS (v11.8.0 ADR-025: PagePrompt→PageInstruction, BlockPrompt→BlockInstruction)
// ═══════════════════════════════════════════════════════════════════════════════

CREATE INDEX pageinstruction_active IF NOT EXISTS FOR (pi:PageInstruction) ON (pi.active);
CREATE INDEX pageinstruction_version IF NOT EXISTS FOR (pi:PageInstruction) ON (pi.version);
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

// ExpressionSet container
CREATE CONSTRAINT expressionset_key IF NOT EXISTS FOR (es:ExpressionSet) REQUIRE es.key IS UNIQUE;

// Culture and Market (already have key from locale)
CREATE CONSTRAINT culture_key IF NOT EXISTS FOR (c:Culture) REQUIRE c.key IS UNIQUE;
CREATE CONSTRAINT market_key IF NOT EXISTS FOR (m:Market) REQUIRE m.key IS UNIQUE;

// Formatting and Slugification
CREATE CONSTRAINT formatting_key IF NOT EXISTS FOR (f:Formatting) REQUIRE f.key IS UNIQUE;
CREATE CONSTRAINT slugification_key IF NOT EXISTS FOR (s:Slugification) REQUIRE s.key IS UNIQUE;
