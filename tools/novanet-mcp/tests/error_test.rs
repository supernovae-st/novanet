//! Tests for error module
//!
//! Tests error variants, helper constructors, message formatting, and MCP conversion.

use novanet_mcp::error::Error;
use rmcp::ErrorData as McpError;

// =============================================================================
// Helper Constructor Tests
// =============================================================================

#[test]
fn test_not_found_error_creation() {
    let err = Error::not_found("qr-code");
    let msg = err.to_string();

    assert!(msg.contains("qr-code"));
    assert!(msg.contains("not found"));
}

#[test]
fn test_not_found_with_string_key() {
    let key = String::from("homepage");
    let err = Error::not_found(key);
    let msg = err.to_string();

    assert!(msg.contains("homepage"));
}

#[test]
fn test_token_budget_exceeded_error() {
    let err = Error::token_budget_exceeded(150_000, 100_000);
    let msg = err.to_string();

    assert!(msg.contains("150000"));
    assert!(msg.contains("100000"));
    assert!(msg.contains("budget"));
}

#[test]
fn test_write_not_allowed_error() {
    let err = Error::write_not_allowed("CREATE");
    let msg = err.to_string();

    assert!(msg.contains("CREATE"));
    assert!(msg.contains("not allowed"));
}

#[test]
fn test_invalid_cypher_error() {
    let err = Error::invalid_cypher("Query contains DELETE keyword");
    let msg = err.to_string();

    assert!(msg.contains("DELETE"));
    assert!(msg.contains("Invalid"));
}

// =============================================================================
// Error Message Format Tests
// =============================================================================

#[test]
fn test_config_error_message() {
    let err = Error::Config("Missing required environment variable".to_string());
    let msg = err.to_string();

    assert!(msg.contains("Configuration error"));
    assert!(msg.contains("Missing required"));
}

#[test]
fn test_pool_error_message() {
    let err = Error::Pool("Connection pool exhausted".to_string());
    let msg = err.to_string();

    assert!(msg.contains("pool"));
    assert!(msg.contains("exhausted"));
}

#[test]
fn test_mcp_protocol_error_message() {
    let err = Error::Mcp("Invalid JSON-RPC request".to_string());
    let msg = err.to_string();

    assert!(msg.contains("MCP"));
    assert!(msg.contains("JSON-RPC"));
}

#[test]
fn test_internal_error_message() {
    let err = Error::Internal("Unexpected state".to_string());
    let msg = err.to_string();

    assert!(msg.contains("Internal"));
    assert!(msg.contains("Unexpected"));
}

// =============================================================================
// MCP Error Code Mapping Tests
// =============================================================================

const RESOURCE_NOT_FOUND: i32 = -32001;
const INVALID_PARAMS: i32 = -32602;
const INTERNAL_ERROR: i32 = -32603;

#[test]
fn test_not_found_maps_to_resource_not_found_code() {
    let err = Error::not_found("entity-key");
    let mcp_err: McpError = err.into();

    assert_eq!(mcp_err.code.0, RESOURCE_NOT_FOUND);
    assert!(mcp_err.message.contains("entity-key"));
}

#[test]
fn test_invalid_cypher_maps_to_invalid_params() {
    let err = Error::invalid_cypher("Contains forbidden keyword");
    let mcp_err: McpError = err.into();

    assert_eq!(mcp_err.code.0, INVALID_PARAMS);
}

#[test]
fn test_write_not_allowed_maps_to_invalid_params() {
    let err = Error::write_not_allowed("DELETE");
    let mcp_err: McpError = err.into();

    assert_eq!(mcp_err.code.0, INVALID_PARAMS);
}

#[test]
fn test_token_budget_exceeded_maps_to_invalid_params() {
    let err = Error::token_budget_exceeded(200_000, 100_000);
    let mcp_err: McpError = err.into();

    assert_eq!(mcp_err.code.0, INVALID_PARAMS);
}

#[test]
fn test_config_error_maps_to_internal_error() {
    let err = Error::Config("Missing password".to_string());
    let mcp_err: McpError = err.into();

    assert_eq!(mcp_err.code.0, INTERNAL_ERROR);
}

#[test]
fn test_pool_error_maps_to_internal_error() {
    let err = Error::Pool("Timeout".to_string());
    let mcp_err: McpError = err.into();

    assert_eq!(mcp_err.code.0, INTERNAL_ERROR);
}

#[test]
fn test_internal_error_maps_to_internal_error_code() {
    let err = Error::Internal("Unknown error".to_string());
    let mcp_err: McpError = err.into();

    assert_eq!(mcp_err.code.0, INTERNAL_ERROR);
}

// =============================================================================
// Error Debug Formatting Tests
// =============================================================================

#[test]
fn test_error_debug_format() {
    let err = Error::not_found("test-key");
    let debug = format!("{:?}", err);

    assert!(debug.contains("NotFound"));
    assert!(debug.contains("test-key"));
}

#[test]
fn test_error_display_is_human_readable() {
    let err = Error::token_budget_exceeded(50000, 40000);
    let display = format!("{}", err);

    // Display should be human-readable, not Debug format
    assert!(!display.contains("TokenBudgetExceeded {"));
    assert!(display.contains("exceeded"));
}

// =============================================================================
// From Implementation Tests
// =============================================================================

#[test]
fn test_serde_json_error_converts() {
    // Create a serde_json error by parsing invalid JSON
    let result: std::result::Result<serde_json::Value, serde_json::Error> =
        serde_json::from_str("not valid json");

    let json_err = result.unwrap_err();
    let err: Error = json_err.into();

    let msg = err.to_string();
    assert!(msg.contains("Serialization"));
}

// =============================================================================
// Edge Case Tests
// =============================================================================

#[test]
fn test_empty_key_not_found() {
    let err = Error::not_found("");
    let msg = err.to_string();

    // Should handle empty string gracefully
    assert!(msg.contains("not found"));
}

#[test]
fn test_unicode_in_error_message() {
    let err = Error::not_found("エンティティ-キー");
    let msg = err.to_string();

    assert!(msg.contains("エンティティ-キー"));
}

#[test]
fn test_very_long_error_message() {
    let long_reason = "x".repeat(10_000);
    let err = Error::invalid_cypher(long_reason.clone());
    let msg = err.to_string();

    // Should handle long messages
    assert!(msg.len() > 10_000);
}

#[test]
fn test_special_characters_in_error() {
    let err = Error::invalid_cypher("Contains ' and \" and \n newline");
    let msg = err.to_string();

    assert!(msg.contains("'"));
    assert!(msg.contains("\""));
}
