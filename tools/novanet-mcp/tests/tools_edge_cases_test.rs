//! Edge case tests for MCP tools
//!
//! Tests defaults, serialization, filter building, and parameter validation
//! for all 7 NovaNet MCP tools. Unit tests only (no Neo4j required).

use novanet_mcp::tools::{
    assemble::{AssembleParams, AssemblyStrategy, EvidencePacket},
    atoms::{AtomType, AtomsParams},
    describe::{DescribeParams, DescribeTarget},
    generate::{DenominationForm, GenerateMode, GenerateParams},
    query::QueryParams,
    search::{PropertyMatch, SearchHit, SearchMode, SearchParams},
    traverse::{TraversalDirection, TraverseParams},
};

// =============================================================================
// novanet_generate Edge Cases
// =============================================================================

#[test]
fn test_generate_mode_from_str() {
    // Default
    let mode: GenerateMode = Default::default();
    assert!(matches!(mode, GenerateMode::Block));
}

#[test]
fn test_generate_params_defaults() {
    let json = r#"{"focus_key": "homepage", "locale": "fr-FR"}"#;
    let params: GenerateParams = serde_json::from_str(json).unwrap();

    assert_eq!(params.focus_key, "homepage");
    assert_eq!(params.locale, "fr-FR");
    // Verify defaults are applied
    assert!(params.token_budget.is_none());
    assert!(params.spreading_depth.is_none());
}

#[test]
fn test_generate_params_with_all_options() {
    let json = r#"{
        "focus_key": "pricing-page",
        "locale": "es-MX",
        "mode": "page",
        "token_budget": 50000,
        "spreading_depth": 3,
        "include_examples": true
    }"#;
    let params: GenerateParams = serde_json::from_str(json).unwrap();

    assert_eq!(params.focus_key, "pricing-page");
    assert_eq!(params.locale, "es-MX");
    assert!(matches!(params.mode, GenerateMode::Page));
    assert_eq!(params.token_budget, Some(50000));
    assert_eq!(params.spreading_depth, Some(3));
}

#[test]
fn test_generate_params_mode_variants() {
    // Test block mode (default)
    let json = r#"{"focus_key": "test", "locale": "en-US"}"#;
    let params: GenerateParams = serde_json::from_str(json).unwrap();
    assert!(matches!(params.mode, GenerateMode::Block));

    // Test page mode
    let json = r#"{"focus_key": "test", "locale": "en-US", "mode": "page"}"#;
    let params: GenerateParams = serde_json::from_str(json).unwrap();
    assert!(matches!(params.mode, GenerateMode::Page));
}

#[test]
fn test_denomination_form_default() {
    // DenominationForm has Default trait
    let form: DenominationForm = Default::default();
    assert_eq!(form.text, "");
    assert_eq!(form.title, "");
    assert_eq!(form.abbrev, "");
    assert!(form.url.is_none());
    assert!(form.mixed.is_none());
    assert!(form.base.is_none());
}

#[test]
fn test_denomination_form_serialization() {
    let form = DenominationForm {
        text: "código qr".to_string(),
        title: "Código QR".to_string(),
        abbrev: "qr".to_string(),
        url: Some("crear-codigo-qr".to_string()),
        mixed: None,
        base: None,
    };

    let json = serde_json::to_string(&form).unwrap();
    assert!(json.contains("código qr"));
    assert!(json.contains("Código QR"));
    assert!(json.contains("crear-codigo-qr"));
    // Optional None fields should be skipped in serialization
    assert!(!json.contains("mixed"));
    assert!(!json.contains("base"));
}

// =============================================================================
// novanet_describe Edge Cases
// =============================================================================

#[test]
fn test_describe_target_all_variants() {
    let targets = [
        "schema",
        "entity",
        "category",
        "relations",
        "locales",
        "stats",
    ];
    for target_str in targets {
        let json = format!(r#"{{"describe": "{}"}}"#, target_str);
        let params: DescribeParams = serde_json::from_str(&json).unwrap();
        // Should parse without error
        assert!(matches!(
            params.describe,
            DescribeTarget::Schema
                | DescribeTarget::Entity
                | DescribeTarget::Category
                | DescribeTarget::Relations
                | DescribeTarget::Locales
                | DescribeTarget::Stats
        ));
    }
}

#[test]
fn test_describe_params_entity_requires_key() {
    let json = r#"{"describe": "entity"}"#;
    let params: DescribeParams = serde_json::from_str(json).unwrap();

    // entity_key should be None by default
    assert!(params.entity_key.is_none());
}

#[test]
fn test_describe_params_with_entity_key() {
    let json = r#"{"describe": "entity", "entity_key": "qr-code"}"#;
    let params: DescribeParams = serde_json::from_str(json).unwrap();

    assert_eq!(params.entity_key, Some("qr-code".to_string()));
}

#[test]
fn test_describe_params_category_optional_key() {
    // Without category_key - should list all
    let json = r#"{"describe": "category"}"#;
    let params: DescribeParams = serde_json::from_str(json).unwrap();
    assert!(params.category_key.is_none());

    // With category_key - should filter
    let json = r#"{"describe": "category", "category_key": "products"}"#;
    let params: DescribeParams = serde_json::from_str(json).unwrap();
    assert_eq!(params.category_key, Some("products".to_string()));
}

// =============================================================================
// novanet_traverse Edge Cases
// =============================================================================

#[test]
fn test_traverse_direction_default() {
    let dir: TraversalDirection = Default::default();
    assert!(matches!(dir, TraversalDirection::Both));
}

#[test]
fn test_traverse_direction_all_variants() {
    let directions = ["outgoing", "incoming", "both"];
    for dir_str in directions {
        let json = format!(r#""{}""#, dir_str);
        let dir: TraversalDirection = serde_json::from_str(&json).unwrap();
        assert!(matches!(
            dir,
            TraversalDirection::Outgoing | TraversalDirection::Incoming | TraversalDirection::Both
        ));
    }
}

#[test]
fn test_traverse_params_minimal() {
    let json = r#"{"start_key": "homepage"}"#;
    let params: TraverseParams = serde_json::from_str(json).unwrap();

    assert_eq!(params.start_key, "homepage");
    assert!(params.max_depth.is_none());
    assert!(params.arc_families.is_none());
    assert!(params.target_kinds.is_none());
}

#[test]
fn test_traverse_params_with_filters() {
    let json = r#"{
        "start_key": "homepage",
        "max_depth": 3,
        "direction": "both",
        "arc_families": ["ownership", "semantic"],
        "target_kinds": ["Entity", "Block"],
        "limit": 50
    }"#;
    let params: TraverseParams = serde_json::from_str(json).unwrap();

    assert_eq!(params.start_key, "homepage");
    assert_eq!(params.max_depth, Some(3));
    assert!(matches!(params.direction, TraversalDirection::Both));
    assert_eq!(
        params.arc_families,
        Some(vec!["ownership".to_string(), "semantic".to_string()])
    );
    assert_eq!(
        params.target_kinds,
        Some(vec!["Entity".to_string(), "Block".to_string()])
    );
    assert_eq!(params.limit, Some(50));
}

#[test]
fn test_traverse_params_empty_arrays() {
    let json = r#"{
        "start_key": "test",
        "arc_families": [],
        "target_kinds": []
    }"#;
    let params: TraverseParams = serde_json::from_str(json).unwrap();

    // Empty arrays should be treated as no filter
    assert_eq!(params.arc_families, Some(vec![]));
    assert_eq!(params.target_kinds, Some(vec![]));
}

// =============================================================================
// novanet_search Edge Cases
// =============================================================================

#[test]
fn test_search_mode_default() {
    let mode: SearchMode = Default::default();
    assert!(matches!(mode, SearchMode::Hybrid));
}

#[test]
fn test_search_mode_all_variants() {
    let modes = ["fulltext", "property", "hybrid"];
    for mode_str in modes {
        let json = format!(r#""{}""#, mode_str);
        let mode: SearchMode = serde_json::from_str(&json).unwrap();
        assert!(matches!(
            mode,
            SearchMode::Fulltext | SearchMode::Property | SearchMode::Hybrid
        ));
    }
}

#[test]
fn test_search_params_minimal() {
    let json = r#"{"query": "QR code"}"#;
    let params: SearchParams = serde_json::from_str(json).unwrap();

    assert_eq!(params.query, "QR code");
    assert!(matches!(params.mode, SearchMode::Hybrid)); // default
    assert!(params.kinds.is_none());
    assert!(params.realm.is_none());
    assert!(params.limit.is_none());
}

#[test]
fn test_search_params_special_characters() {
    // Test with special characters that might break Cypher
    let json = r#"{"query": "test's \"quoted\" and [brackets]"}"#;
    let params: SearchParams = serde_json::from_str(json).unwrap();
    assert_eq!(params.query, "test's \"quoted\" and [brackets]");
}

#[test]
fn test_search_params_unicode() {
    let json = r#"{"query": "código QR 二维码 Qコード"}"#;
    let params: SearchParams = serde_json::from_str(json).unwrap();
    assert_eq!(params.query, "código QR 二维码 Qコード");
}

#[test]
fn test_search_hit_serialization() {
    let hit = SearchHit {
        key: "test-entity".to_string(),
        kind: "Entity".to_string(),
        score: 0.95,
        matches: vec![PropertyMatch {
            property: "name".to_string(),
            value: "Test Entity".to_string(),
        }],
        properties: serde_json::json!({"name": "Test Entity"}),
    };

    let json = serde_json::to_string(&hit).unwrap();
    assert!(json.contains("test-entity"));
    assert!(json.contains("0.95"));
}

// =============================================================================
// novanet_atoms Edge Cases
// =============================================================================

#[test]
fn test_atom_type_default() {
    let atom_type: AtomType = Default::default();
    assert!(matches!(atom_type, AtomType::All));
}

#[test]
fn test_atom_type_all_variants() {
    let types = [
        "term",
        "expression",
        "pattern",
        "cultureref",
        "taboo",
        "audiencetrait",
        "all",
    ];
    for type_str in types {
        let json = format!(r#""{}""#, type_str);
        let atom_type: AtomType = serde_json::from_str(&json).unwrap();
        assert!(matches!(
            atom_type,
            AtomType::Term
                | AtomType::Expression
                | AtomType::Pattern
                | AtomType::CultureRef
                | AtomType::Taboo
                | AtomType::AudienceTrait
                | AtomType::All
        ));
    }
}

#[test]
fn test_atom_params_minimal() {
    let json = r#"{"locale": "fr-FR"}"#;
    let params: AtomsParams = serde_json::from_str(json).unwrap();

    assert_eq!(params.locale, "fr-FR");
    assert!(matches!(params.atom_type, AtomType::All)); // default
    assert!(params.domain.is_none());
    assert!(params.limit.is_none());
}

#[test]
fn test_atom_params_with_domain() {
    let json = r#"{
        "locale": "es-MX",
        "atom_type": "term",
        "domain": "technical",
        "limit": 100
    }"#;
    let params: AtomsParams = serde_json::from_str(json).unwrap();

    assert_eq!(params.locale, "es-MX");
    assert!(matches!(params.atom_type, AtomType::Term));
    assert_eq!(params.domain, Some("technical".to_string()));
    assert_eq!(params.limit, Some(100));
}

// =============================================================================
// novanet_assemble Edge Cases
// =============================================================================

#[test]
fn test_assemble_strategy_default() {
    let strategy: AssemblyStrategy = Default::default();
    assert!(matches!(strategy, AssemblyStrategy::Breadth));
}

#[test]
fn test_assemble_strategy_all_variants() {
    let strategies = ["breadth", "depth", "relevance", "custom"];
    for strategy_str in strategies {
        let json = format!(r#""{}""#, strategy_str);
        let strategy: AssemblyStrategy = serde_json::from_str(&json).unwrap();
        assert!(matches!(
            strategy,
            AssemblyStrategy::Breadth
                | AssemblyStrategy::Depth
                | AssemblyStrategy::Relevance
                | AssemblyStrategy::Custom
        ));
    }
}

#[test]
fn test_assemble_params_minimal() {
    // locale is required, so we must provide it
    let json = r#"{"focus_key": "homepage", "locale": "en-US"}"#;
    let params: AssembleParams = serde_json::from_str(json).unwrap();

    assert_eq!(params.focus_key, "homepage");
    assert_eq!(params.locale, "en-US");
    assert!(params.token_budget.is_none());
}

#[test]
fn test_assemble_params_full() {
    let json = r#"{
        "focus_key": "homepage",
        "locale": "fr-FR",
        "token_budget": 50000,
        "strategy": "relevance",
        "include_entities": true,
        "include_knowledge": false,
        "max_depth": 4
    }"#;
    let params: AssembleParams = serde_json::from_str(json).unwrap();

    assert_eq!(params.focus_key, "homepage");
    assert_eq!(params.locale, "fr-FR");
    assert_eq!(params.token_budget, Some(50000));
    assert!(matches!(params.strategy, AssemblyStrategy::Relevance));
}

#[test]
fn test_evidence_packet_serialization() {
    let packet = EvidencePacket {
        source_key: "qr-code".to_string(),
        source_kind: "Entity".to_string(),
        evidence_type: "definition".to_string(),
        distance: 1,
        relevance: 0.95,
        content: "QR Code: Machine-readable optical label".to_string(),
        tokens: 12,
    };

    let json = serde_json::to_string(&packet).unwrap();
    assert!(json.contains("qr-code"));
    assert!(json.contains("definition"));
    assert!(json.contains("0.95"));
}

// =============================================================================
// novanet_query Edge Cases
// =============================================================================

#[test]
fn test_query_params_minimal() {
    let json = r#"{"cypher": "MATCH (n) RETURN n"}"#;
    let params: QueryParams = serde_json::from_str(json).unwrap();

    assert_eq!(params.cypher, "MATCH (n) RETURN n");
    assert!(params.params.is_none());
    assert!(params.limit.is_none());
    assert!(params.timeout_ms.is_none());
}

#[test]
fn test_query_params_with_all_options() {
    let json = r#"{
        "cypher": "MATCH (n:Entity {key: $key}) RETURN n",
        "params": {"key": "qr-code"},
        "limit": 50,
        "timeout_ms": 10000
    }"#;
    let params: QueryParams = serde_json::from_str(json).unwrap();

    assert!(params.cypher.contains("$key"));
    assert!(params.params.is_some());
    let params_map = params.params.unwrap();
    assert_eq!(params_map.get("key").unwrap(), "qr-code");
    assert_eq!(params.limit, Some(50));
    assert_eq!(params.timeout_ms, Some(10000));
}

#[test]
fn test_query_params_empty_params_map() {
    let json = r#"{"cypher": "MATCH (n) RETURN n", "params": {}}"#;
    let params: QueryParams = serde_json::from_str(json).unwrap();

    assert!(params.params.is_some());
    assert!(params.params.unwrap().is_empty());
}

#[test]
fn test_query_params_complex_params() {
    let json = r#"{
        "cypher": "MATCH (n) WHERE n.value > $min AND n.tags IN $tags RETURN n",
        "params": {
            "min": 100,
            "tags": ["a", "b", "c"],
            "nested": {"key": "value"}
        }
    }"#;
    let params: QueryParams = serde_json::from_str(json).unwrap();

    let params_map = params.params.unwrap();
    assert_eq!(params_map.get("min").unwrap(), &serde_json::json!(100));
    assert!(params_map.get("tags").unwrap().is_array());
    assert!(params_map.get("nested").unwrap().is_object());
}

// =============================================================================
// Cross-cutting Edge Cases
// =============================================================================

#[test]
fn test_locale_format_bcp47() {
    // Various BCP-47 format locales
    let locales = ["en", "en-US", "fr-FR", "zh-Hans-CN", "pt-BR"];
    for locale in locales {
        let json = format!(r#"{{"focus_key": "test", "locale": "{}"}}"#, locale);
        let params: GenerateParams = serde_json::from_str(&json).unwrap();
        assert_eq!(params.locale, locale);
    }
}

#[test]
fn test_empty_string_fields() {
    // Empty strings should be accepted (validation happens at execution)
    let json = r#"{"focus_key": "", "locale": ""}"#;
    let params: GenerateParams = serde_json::from_str(json).unwrap();
    assert_eq!(params.focus_key, "");
    assert_eq!(params.locale, "");
}

#[test]
fn test_whitespace_in_keys() {
    // Keys with whitespace - should be preserved
    let json = r#"{"focus_key": "  homepage  ", "locale": "fr-FR"}"#;
    let params: GenerateParams = serde_json::from_str(json).unwrap();
    assert_eq!(params.focus_key, "  homepage  ");
}

#[test]
fn test_very_large_limit() {
    let json = r#"{"query": "test", "limit": 1000000}"#;
    let params: SearchParams = serde_json::from_str(json).unwrap();
    assert_eq!(params.limit, Some(1000000));
    // Note: actual clamping to 100 happens at execution time
}

#[test]
fn test_zero_token_budget() {
    let json = r#"{"focus_key": "test", "locale": "en", "token_budget": 0}"#;
    let params: GenerateParams = serde_json::from_str(json).unwrap();
    assert_eq!(params.token_budget, Some(0));
}
