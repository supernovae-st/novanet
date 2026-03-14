//! novanet_write tool (v0.20.0)
//!
//! Intelligent data writes to Neo4j with schema validation.
//! Single tool with 3 operations: upsert_node, create_arc, update_props.
//!
//! v0.20.0 (D6): Absorbs novanet_check via dry_run parameter.
//! When dry_run=true: validates everything, returns Cypher preview + suggestions.
//! When dry_run=false/None: validates then executes.
//!
//! Submodules:
//! - types: WriteOperation, WriteParams, WriteResult, and related types
//! - validation: Parameter validation, dry-run checks, denomination forms
//! - operations: MCP_PROVENANCE and upsert/arc/update executors

mod operations;
mod types;
mod validation;

pub use types::*;

use crate::error::{Error, Result};
use crate::server::State;
use operations::{execute_create_arc, execute_update_props, execute_upsert_node};
use validation::{execute_dry_run, fetch_and_validate_class, validate_params};

// ═══════════════════════════════════════════════════════════════════════════════
// Main Execute Function
// ═══════════════════════════════════════════════════════════════════════════════

/// Execute the novanet_write tool
pub async fn execute(state: &State, params: WriteParams) -> Result<WriteResult> {
    // D6: dry_run mode — validate without executing
    if params.dry_run.unwrap_or(false) {
        let result = execute_dry_run(state, &params).await?;
        return Ok(WriteResult::DryRun(result));
    }

    // Normal write: validate then execute
    validate_params(&params)?;

    let result = match params.operation {
        WriteOperation::UpsertNode => {
            let class = params
                .class
                .as_ref()
                .ok_or_else(|| Error::InvalidParams("upsert_node requires 'class'".into()))?;
            let meta = fetch_and_validate_class(state, class).await?;
            execute_upsert_node(state, &params, &meta).await?
        }
        WriteOperation::CreateArc => execute_create_arc(state, &params).await?,
        WriteOperation::UpdateProps => {
            let class = params
                .class
                .as_ref()
                .ok_or_else(|| Error::InvalidParams("update_props requires 'class'".into()))?;
            let meta = fetch_and_validate_class(state, class).await?;
            execute_update_props(state, &params, &meta).await?
        }
    };

    Ok(WriteResult::Executed(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema_cache::ClassMetadata;

    #[test]
    fn test_write_operation_deserialize() {
        let json = r#""upsert_node""#;
        let op: WriteOperation = serde_json::from_str(json).unwrap();
        assert_eq!(op, WriteOperation::UpsertNode);

        let json = r#""create_arc""#;
        let op: WriteOperation = serde_json::from_str(json).unwrap();
        assert_eq!(op, WriteOperation::CreateArc);

        let json = r#""update_props""#;
        let op: WriteOperation = serde_json::from_str(json).unwrap();
        assert_eq!(op, WriteOperation::UpdateProps);
    }

    #[test]
    fn test_write_params_deserialize_with_dry_run() {
        let json = r#"{
            "operation": "upsert_node",
            "class": "SEOKeyword",
            "key": "seo:qr-code@fr-FR",
            "properties": {
                "keyword": "qr code",
                "search_volume": 110000
            },
            "dry_run": true
        }"#;

        let params: WriteParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.operation, WriteOperation::UpsertNode);
        assert_eq!(params.class, Some("SEOKeyword".to_string()));
        assert_eq!(params.key, Some("seo:qr-code@fr-FR".to_string()));
        assert_eq!(params.dry_run, Some(true));
    }

    #[test]
    fn test_write_params_deserialize_without_dry_run() {
        let json = r#"{
            "operation": "upsert_node",
            "class": "SEOKeyword",
            "key": "seo:qr-code@fr-FR"
        }"#;

        let params: WriteParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.dry_run, None);
    }

    #[test]
    fn test_executed_result_serialize() {
        let result = WriteResult::Executed(ExecutedResult {
            success: true,
            operation: "upsert_node".to_string(),
            key: "seo:qr-code@fr-FR".to_string(),
            created: true,
            updated_properties: vec![],
            auto_arcs_created: vec!["FOR_LOCALE".to_string()],
            execution_time_ms: 45,
            cache_invalidated: vec!["SEOKeyword:*".to_string()],
        });

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("seo:qr-code@fr-FR"));
        assert!(json.contains("FOR_LOCALE"));
    }

    #[test]
    fn test_dry_run_result_serialize() {
        let result = WriteResult::DryRun(DryRunResult {
            valid: true,
            would_create: true,
            issues: vec![],
            cypher_preview: Some("MERGE (n:SEOKeyword {key: $key})".to_string()),
            schema_context: Some(SchemaContext {
                class_name: "SEOKeyword".to_string(),
                realm: "shared".to_string(),
                layer: "knowledge".to_string(),
                content: Some("SEO keyword imported from external tools".to_string()),
                triggers: Some(vec!["seo".to_string(), "keyword".to_string()]),
                required_properties: vec!["keyword".to_string()],
                optional_properties: vec!["search_volume".to_string()],
            }),
            suggestions: vec![OntologySuggestion {
                action: "Provenance auto-injected".to_string(),
                reason: "created_by set to mcp:novanet_write".to_string(),
                example: None,
            }],
            token_estimate: 150,
        });

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("would_create"));
        assert!(json.contains("cypher_preview"));
        assert!(json.contains("schema_context"));
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Parameter Validation Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_validate_params_upsert_node_ok() {
        let params = WriteParams {
            operation: WriteOperation::UpsertNode,
            class: Some("SEOKeyword".to_string()),
            key: Some("seo:test@fr-FR".to_string()),
            arc_class: None,
            from_key: None,
            to_key: None,
            properties: Some(serde_json::Map::new()),
            locale: None,
            dry_run: None,
        };
        assert!(validate_params(&params).is_ok());
    }

    #[test]
    fn test_validate_params_upsert_node_missing_class() {
        let params = WriteParams {
            operation: WriteOperation::UpsertNode,
            class: None,
            key: Some("seo:test@fr-FR".to_string()),
            arc_class: None,
            from_key: None,
            to_key: None,
            properties: None,
            locale: None,
            dry_run: None,
        };
        let err = validate_params(&params).unwrap_err();
        assert!(err.to_string().contains("class"));
    }

    #[test]
    fn test_validate_params_create_arc_ok() {
        let params = WriteParams {
            operation: WriteOperation::CreateArc,
            class: None,
            key: None,
            arc_class: Some("TARGETS".to_string()),
            from_key: Some("seo:qr-code@fr-FR".to_string()),
            to_key: Some("entity-native:qr-code@fr-FR".to_string()),
            properties: Some(serde_json::Map::new()),
            locale: None,
            dry_run: None,
        };
        assert!(validate_params(&params).is_ok());
    }

    #[test]
    fn test_validate_params_create_arc_missing_to_key() {
        let params = WriteParams {
            operation: WriteOperation::CreateArc,
            class: None,
            key: None,
            arc_class: Some("TARGETS".to_string()),
            from_key: Some("seo:qr-code@fr-FR".to_string()),
            to_key: None,
            properties: None,
            locale: None,
            dry_run: None,
        };
        let err = validate_params(&params).unwrap_err();
        assert!(err.to_string().contains("to_key"));
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Validation Issue Tests (absorbed from checker)
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_validate_operation_params_upsert_missing_both() {
        let params = WriteParams {
            operation: WriteOperation::UpsertNode,
            class: None,
            key: None,
            arc_class: None,
            from_key: None,
            to_key: None,
            properties: None,
            locale: None,
            dry_run: Some(true),
        };
        let issues = validation::validate_operation_params(&params);
        assert_eq!(issues.len(), 2);
        assert_eq!(issues[0].code, "E001");
        assert_eq!(issues[1].code, "E002");
    }

    #[test]
    fn test_validate_operation_params_create_arc_missing_all() {
        let params = WriteParams {
            operation: WriteOperation::CreateArc,
            class: None,
            key: None,
            arc_class: None,
            from_key: None,
            to_key: None,
            properties: None,
            locale: None,
            dry_run: Some(true),
        };
        let issues = validation::validate_operation_params(&params);
        assert_eq!(issues.len(), 3);
        assert_eq!(issues[0].code, "E003");
        assert_eq!(issues[1].code, "E004");
        assert_eq!(issues[2].code, "E005");
    }

    #[test]
    fn test_validate_operation_params_update_props_missing_all() {
        let params = WriteParams {
            operation: WriteOperation::UpdateProps,
            class: None,
            key: None,
            arc_class: None,
            from_key: None,
            to_key: None,
            properties: None,
            locale: None,
            dry_run: Some(true),
        };
        let issues = validation::validate_operation_params(&params);
        assert_eq!(issues.len(), 3);
        assert_eq!(issues[0].code, "E006");
        assert_eq!(issues[1].code, "E007");
        assert_eq!(issues[2].code, "E008");
    }

    #[test]
    fn test_validate_denomination_forms_missing() {
        let props = serde_json::Map::new();
        let issues = validation::validate_denomination_forms("EntityNative", &props);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].code, "W001");
    }

    #[test]
    fn test_validate_denomination_forms_incomplete() {
        let mut props = serde_json::Map::new();
        props.insert(
            "denomination_forms".to_string(),
            serde_json::json!([
                {"type": "text", "value": "code QR", "priority": 1},
                {"type": "title", "value": "Code QR", "priority": 1}
            ]),
        );

        let issues = validation::validate_denomination_forms("EntityNative", &props);
        // Missing abbrev and url
        assert_eq!(issues.len(), 2);
        assert!(issues.iter().all(|i| i.code == "W002"));
    }

    #[test]
    fn test_validate_denomination_forms_complete() {
        let mut props = serde_json::Map::new();
        props.insert(
            "denomination_forms".to_string(),
            serde_json::json!([
                {"type": "text", "value": "code QR", "priority": 1},
                {"type": "title", "value": "Code QR", "priority": 1},
                {"type": "abbrev", "value": "QR", "priority": 1},
                {"type": "url", "value": "code-qr", "priority": 1}
            ]),
        );

        let issues = validation::validate_denomination_forms("EntityNative", &props);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_validate_denomination_forms_non_entity_native() {
        let props = serde_json::Map::new();
        let issues = validation::validate_denomination_forms("SEOKeyword", &props);
        assert!(issues.is_empty()); // No check for non-EntityNative
    }

    #[test]
    fn test_validate_denomination_forms_invalid_structure() {
        let mut props = serde_json::Map::new();
        props.insert(
            "denomination_forms".to_string(),
            serde_json::json!([
                {"value": "code QR"},  // missing type
                {"type": "title"}      // missing value
            ]),
        );

        let issues = validation::validate_denomination_forms("EntityNative", &props);
        // E014 (missing type), E015 (missing value), plus W002 for missing types
        let errors: Vec<_> = issues
            .iter()
            .filter(|i| i.severity == CheckSeverity::Error)
            .collect();
        assert_eq!(errors.len(), 2);
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Required Properties Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_validate_required_properties_all_present() {
        let meta = ClassMetadata {
            name: "SEOKeyword".to_string(),
            realm: "shared".to_string(),
            layer: "knowledge".to_string(),

            required_properties: vec!["keyword".to_string(), "slug_form".to_string()],
            optional_properties: vec!["search_volume".to_string()],
            ..Default::default()
        };

        let mut props = serde_json::Map::new();
        props.insert(
            "keyword".to_string(),
            serde_json::Value::String("qr code".to_string()),
        );
        props.insert(
            "slug_form".to_string(),
            serde_json::Value::String("qr-code".to_string()),
        );

        let issues = validation::validate_required_properties(&meta, &props);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_validate_required_properties_missing() {
        let meta = ClassMetadata {
            name: "SEOKeyword".to_string(),
            realm: "shared".to_string(),
            layer: "knowledge".to_string(),

            required_properties: vec!["keyword".to_string(), "slug_form".to_string()],
            optional_properties: vec!["search_volume".to_string()],
            ..Default::default()
        };

        let mut props = serde_json::Map::new();
        props.insert(
            "keyword".to_string(),
            serde_json::Value::String("qr code".to_string()),
        );

        let issues = validation::validate_required_properties(&meta, &props);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].code, "E011");
    }

    #[test]
    fn test_validate_required_properties_skips_system_managed() {
        let meta = ClassMetadata {
            name: "SEOKeyword".to_string(),
            realm: "shared".to_string(),
            layer: "knowledge".to_string(),

            required_properties: vec![
                "key".to_string(),
                "keyword".to_string(),
                "created_at".to_string(),
                "updated_at".to_string(),
            ],
            ..Default::default()
        };

        let mut props = serde_json::Map::new();
        props.insert(
            "keyword".to_string(),
            serde_json::Value::String("qr code".to_string()),
        );

        // key, created_at, updated_at should be skipped (system-managed)
        let issues = validation::validate_required_properties(&meta, &props);
        assert!(issues.is_empty());
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Schema Context & Suggestions Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_build_schema_context() {
        let meta = ClassMetadata {
            name: "EntityNative".to_string(),
            realm: "org".to_string(),
            layer: "semantic".to_string(),

            required_properties: vec!["name".to_string()],
            optional_properties: vec!["description".to_string()],
            content: Some("Locale-specific entity content".to_string()),
            triggers: Some(vec!["native".to_string(), "locale".to_string()]),
            ..Default::default()
        };

        let ctx = validation::build_schema_context(&meta);
        assert_eq!(ctx.class_name, "EntityNative");
        assert_eq!(ctx.realm, "org");
        assert_eq!(ctx.layer, "semantic");
        assert!(ctx.content.is_some());
        assert!(ctx.triggers.is_some());
    }

    #[test]
    fn test_generate_suggestions_native_class() {
        let meta = ClassMetadata {
            name: "EntityNative".to_string(),
            realm: "org".to_string(),
            layer: "semantic".to_string(),

            ..Default::default()
        };

        let params = WriteParams {
            operation: WriteOperation::UpsertNode,
            class: Some("EntityNative".to_string()),
            key: Some("qr-code@fr-FR".to_string()),
            arc_class: None,
            from_key: None,
            to_key: None,
            properties: None,
            locale: Some("fr-FR".to_string()),
            dry_run: Some(true),
        };

        let suggestions = validation::generate_suggestions(&meta, &params, &[]);
        // Should have: FOR_LOCALE, HAS_NATIVE, denomination_forms, provenance
        assert!(suggestions.len() >= 3);
        assert!(suggestions
            .iter()
            .any(|s| s.action.contains("FOR_LOCALE")));
        assert!(suggestions
            .iter()
            .any(|s| s.action.contains("HAS_NATIVE")));
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Cypher Preview Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_generate_cypher_preview_upsert() {
        let params = WriteParams {
            operation: WriteOperation::UpsertNode,
            class: Some("SEOKeyword".to_string()),
            key: Some("seo:test@fr-FR".to_string()),
            arc_class: None,
            from_key: None,
            to_key: None,
            properties: None,
            locale: Some("fr-FR".to_string()),
            dry_run: Some(true),
        };

        let preview = validation::generate_cypher_preview(&params, None).unwrap();
        assert!(preview.contains("MERGE (n:SEOKeyword"));
        assert!(preview.contains("FOR_LOCALE"));
    }

    #[test]
    fn test_generate_cypher_preview_create_arc() {
        let params = WriteParams {
            operation: WriteOperation::CreateArc,
            class: None,
            key: None,
            arc_class: Some("TARGETS".to_string()),
            from_key: Some("a".to_string()),
            to_key: Some("b".to_string()),
            properties: None,
            locale: None,
            dry_run: Some(true),
        };

        let preview = validation::generate_cypher_preview(&params, None).unwrap();
        assert!(preview.contains("TARGETS"));
        assert!(preview.contains("from_key"));
        assert!(preview.contains("to_key"));
    }

    #[test]
    fn test_generate_cypher_preview_update_props() {
        let params = WriteParams {
            operation: WriteOperation::UpdateProps,
            class: Some("EntityNative".to_string()),
            key: Some("qr-code@fr-FR".to_string()),
            arc_class: None,
            from_key: None,
            to_key: None,
            properties: None,
            locale: None,
            dry_run: Some(true),
        };

        let preview = validation::generate_cypher_preview(&params, None).unwrap();
        assert!(preview.contains("MATCH (n:EntityNative"));
        assert!(preview.contains("updated_at"));
    }

    #[test]
    fn test_generate_cypher_preview_native_with_has_native() {
        let meta = ClassMetadata {
            name: "EntityNative".to_string(),
            realm: "org".to_string(),
            layer: "semantic".to_string(),

            ..Default::default()
        };

        let params = WriteParams {
            operation: WriteOperation::UpsertNode,
            class: Some("EntityNative".to_string()),
            key: Some("qr-code@fr-FR".to_string()),
            arc_class: None,
            from_key: None,
            to_key: None,
            properties: None,
            locale: None,
            dry_run: Some(true),
        };

        let preview = validation::generate_cypher_preview(&params, Some(&meta)).unwrap();
        assert!(preview.contains("HAS_NATIVE"));
        assert!(preview.contains("Entity"));
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Check Issue Builder Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_check_issue_builders() {
        let error = CheckIssue::error("E001", "test error")
            .with_field("class")
            .with_hint("try this");

        assert_eq!(error.severity, CheckSeverity::Error);
        assert_eq!(error.code, "E001");
        assert_eq!(error.field, Some("class".to_string()));
        assert_eq!(error.hint, Some("try this".to_string()));

        let warning = CheckIssue::warning("W001", "test warning");
        assert_eq!(warning.severity, CheckSeverity::Warning);

        let info = CheckIssue::info("I001", "test info");
        assert_eq!(info.severity, CheckSeverity::Info);
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // HAS_NATIVE Key Extraction Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_has_native_key_extraction_entity_native() {
        let key = "qr-code@fr-FR";
        let class = "EntityNative";

        assert!(class.ends_with("Native"));
        assert!(key.contains('@'));

        let entity_key = key.split('@').next().unwrap();
        assert_eq!(entity_key, "qr-code");

        let base_class = class.trim_end_matches("Native");
        assert_eq!(base_class, "Entity");
    }

    #[test]
    fn test_has_native_key_extraction_page_native() {
        let key = "homepage@es-MX";
        let class = "PageNative";

        assert!(class.ends_with("Native"));
        assert!(key.contains('@'));

        let entity_key = key.split('@').next().unwrap();
        assert_eq!(entity_key, "homepage");

        let base_class = class.trim_end_matches("Native");
        assert_eq!(base_class, "Page");
    }

    #[test]
    fn test_has_native_key_extraction_block_native() {
        let key = "head-seo-meta@ja-JP";
        let class = "BlockNative";

        assert!(class.ends_with("Native"));
        assert!(key.contains('@'));

        let entity_key = key.split('@').next().unwrap();
        assert_eq!(entity_key, "head-seo-meta");

        let base_class = class.trim_end_matches("Native");
        assert_eq!(base_class, "Block");
    }

    #[test]
    fn test_has_native_no_extraction_for_non_native() {
        let _key = "qr-code@fr-FR";
        let class = "Entity";
        assert!(!class.ends_with("Native"));
    }

    #[test]
    fn test_has_native_no_extraction_without_locale() {
        let key = "qr-code";
        let class = "EntityNative";

        assert!(class.ends_with("Native"));
        assert!(!key.contains('@'));
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Class Name Validation Integration Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_valid_class_names_accepted() {
        assert!(crate::validation::is_valid_class_name("Entity"));
        assert!(crate::validation::is_valid_class_name("EntityNative"));
        assert!(crate::validation::is_valid_class_name("SEOKeyword"));
        assert!(crate::validation::is_valid_class_name("PageNative"));
        assert!(crate::validation::is_valid_class_name("BlockNative"));
    }

    #[test]
    fn test_cypher_injection_class_names_rejected() {
        assert!(!crate::validation::is_valid_class_name(
            "Entity}DETACH DELETE n"
        ));
        assert!(!crate::validation::is_valid_class_name("Entity]->(x)"));
        assert!(!crate::validation::is_valid_class_name("a:Entity"));
        assert!(!crate::validation::is_valid_class_name("123Entity"));
    }

    #[test]
    fn test_valid_arc_names_accepted() {
        assert!(crate::validation::is_valid_arc_class_name("HAS_NATIVE"));
        assert!(crate::validation::is_valid_arc_class_name("FOR_LOCALE"));
        assert!(crate::validation::is_valid_arc_class_name("BELONGS_TO"));
        assert!(crate::validation::is_valid_arc_class_name("TARGETS"));
    }

    #[test]
    fn test_cypher_injection_arc_names_rejected() {
        assert!(!crate::validation::is_valid_arc_class_name(
            "HAS_NATIVE}RETURN"
        ));
        assert!(!crate::validation::is_valid_arc_class_name("has_native"));
        assert!(!crate::validation::is_valid_arc_class_name("HAS-NATIVE"));
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // ADR-042 Provenance Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_mcp_provenance_constant() {
        assert_eq!(operations::MCP_PROVENANCE, "mcp:novanet_write");
    }

    #[test]
    fn test_provenance_format_examples() {
        let valid_formats = [
            "seed:schema",
            "seed:immutable",
            "content:bootstrap",
            "user:manual",
            "user:studio",
            "nika:workflow:abc123",
            "mcp:novanet_write",
        ];

        for format in valid_formats {
            assert!(format.contains(':'), "Format '{}' should contain ':'", format);
            let parts: Vec<&str> = format.split(':').collect();
            assert!(!parts[0].is_empty(), "Source type should not be empty");
        }
    }
}
