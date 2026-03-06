//! Actionable error hints for NovaNet MCP Server
//!
//! Provides user-friendly hints that help diagnose and resolve common errors.

/// Get actionable hint for an error message
pub fn get_hint(error_msg: &str) -> String {
    let lower = error_msg.to_lowercase();

    // Cypher errors
    if lower.contains("syntaxerror") || lower.contains("syntax error") {
        return "Hint: Check your Cypher syntax.\n\
             Common issues:\n\
             - Missing quotes around string values\n\
             - Incorrect relationship direction (use --> or <--)\n\
             - Undefined variables in RETURN clause\n\
             Try: EXPLAIN {your_query} to validate syntax"
            .to_string();
    }

    if lower.contains("unknown function") {
        return "Hint: Unknown Cypher function.\n\
             - Check APOC is installed: CALL apoc.help('function_name')\n\
             - Verify function spelling and case\n\
             - Some functions require APOC plugin"
            .to_string();
    }

    if lower.contains("variable") && lower.contains("not defined") {
        return "Hint: Undefined variable in query.\n\
             - Ensure variable is declared in MATCH/WITH clause\n\
             - Check variable spelling matches exactly\n\
             - Variables are case-sensitive"
            .to_string();
    }

    // Connection errors
    if lower.contains("connection refused") || lower.contains("connect error") {
        return "Hint: Cannot connect to Neo4j.\n\
             1. Verify Neo4j is running: neo4j status\n\
             2. Check NEO4J_URI environment variable (default: bolt://localhost:7687)\n\
             3. Verify credentials: NEO4J_USER and NEO4J_PASSWORD\n\
             4. Check firewall allows port 7687"
            .to_string();
    }

    if lower.contains("authentication") || lower.contains("unauthorized") {
        return "Hint: Authentication failed.\n\
             - Check NEO4J_USER (default: neo4j)\n\
             - Check NEO4J_PASSWORD\n\
             - Reset password: neo4j-admin set-initial-password <password>"
            .to_string();
    }

    // Resource errors
    if lower.contains("timeout") || lower.contains("timed out") {
        return "Hint: Query timed out.\n\
             - Add LIMIT to reduce result set\n\
             - Use indexes: CREATE INDEX FOR (n:Label) ON (n.property)\n\
             - Profile query: PROFILE {your_query}\n\
             - Consider pagination with SKIP/LIMIT"
            .to_string();
    }

    if lower.contains("memory") || lower.contains("heap") {
        return "Hint: Memory limit exceeded.\n\
             - Reduce result set with LIMIT\n\
             - Use streaming: CALL { ... } IN TRANSACTIONS\n\
             - Increase Neo4j heap: dbms.memory.heap.max_size"
            .to_string();
    }

    // Schema class not found (more specific, must come before generic "not found")
    if lower.contains("schema") && lower.contains("not found") {
        return "Hint: Unknown class name.\n\
             - Use novanet_introspect to list available classes\n\
             - Check spelling and case (PascalCase)\n\
             - Common classes: Entity, EntityNative, SEOKeyword, Term"
            .to_string();
    }

    // Arc endpoint not found (more specific, must come before generic "not found")
    if lower.contains("arc endpoint") && lower.contains("not found") {
        return "Hint: Arc source or target doesn't exist.\n\
             - Create the node first with upsert_node operation\n\
             - Verify the key spelling\n\
             - Use novanet_search to find existing nodes"
            .to_string();
    }

    // Entity not found (generic fallback)
    if lower.contains("not found") || lower.contains("no such") {
        return "Hint: Entity not found.\n\
             - Verify entity key exists: MATCH (e:Entity {key: 'key'}) RETURN e\n\
             - Check for typos in key/label names\n\
             - Use novanet_search to find similar entities"
            .to_string();
    }

    // Schema errors
    if lower.contains("constraint") {
        return "Hint: Constraint violation.\n\
             - Check unique constraints: SHOW CONSTRAINTS\n\
             - Verify property values don't duplicate existing data\n\
             - Use MERGE instead of CREATE for idempotent operations"
            .to_string();
    }

    // Invalid tool (for batch operations)
    if lower.contains("invalid tool") {
        return "Hint: Invalid tool name.\n\
             Valid tools: novanet_query, novanet_describe, novanet_search,\n\
             novanet_traverse, novanet_assemble, novanet_atoms,\n\
             novanet_generate, novanet_introspect, novanet_batch,\n\
             novanet_cache_stats, novanet_cache_invalidate, novanet_write,\n\
             novanet_check, novanet_audit"
            .to_string();
    }

    // Write operation errors
    if lower.contains("trait") && lower.contains("not writable") {
        return "Hint: Write permission denied.\n\
             Only these traits allow writes:\n\
             - authored: Human-written content (EntityNative, PageNative)\n\
             - imported: External data (SEOKeyword, GeoTrend)\n\
             - generated: LLM-generated (BlockNative)\n\
             - retrieved: Discovered knowledge (Term, Expression)\n\
             \n\
             'defined' trait is READ-ONLY (Entity, Page, Block, Locale)."
            .to_string();
    }

    if lower.contains("slug") && lower.contains("locked") {
        return "Hint: Slug is immutable after deployment.\n\
             - Create a URL redirect instead of changing the slug\n\
             - Or set slug_locked: false first (requires admin access)\n\
             - ADR-030: BlockNative:head-seo-meta owns the slug"
            .to_string();
    }

    if lower.contains("singleton") && lower.contains("is_slug_source") {
        return "Hint: Only one SEOKeyword can be the slug source.\n\
             - The existing is_slug_source arc will be demoted\n\
             - Use rank: 'primary' for the new slug source\n\
             - Previous keyword becomes rank: 'secondary'"
            .to_string();
    }

    if lower.contains("missing required property") {
        return "Hint: Required property missing.\n\
             - Use novanet_introspect target='class' name='ClassName' to see required properties\n\
             - Check property names are spelled correctly"
            .to_string();
    }

    // Default hint
    "Hint: Unexpected error occurred.\n\
         - Check NovaNet logs: tail -f ~/.novanet/logs/mcp.log\n\
         - Verify Neo4j health: novanet doctor\n\
         - Report issue: https://github.com/supernovae-st/novanet/issues"
        .to_string()
}

/// Wrap error message with hint
pub fn with_hint(error: &str) -> String {
    format!("{}\n\n{}", error, get_hint(error))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hint_for_cypher_syntax() {
        let hint = get_hint("Neo.ClientError.Statement.SyntaxError");
        assert!(hint.contains("Check your Cypher syntax"));
    }

    #[test]
    fn test_hint_for_cypher_syntax_lowercase() {
        let hint = get_hint("syntax error in query");
        assert!(hint.contains("Check your Cypher syntax"));
    }

    #[test]
    fn test_hint_for_unknown_function() {
        let hint = get_hint("Unknown function 'apoc.foo.bar'");
        assert!(hint.contains("Unknown Cypher function"));
        assert!(hint.contains("APOC"));
    }

    #[test]
    fn test_hint_for_undefined_variable() {
        let hint = get_hint("Variable `x` not defined");
        assert!(hint.contains("Undefined variable"));
        assert!(hint.contains("MATCH/WITH"));
    }

    #[test]
    fn test_hint_for_connection_error() {
        let hint = get_hint("connection refused");
        assert!(hint.contains("NEO4J_URI"));
        assert!(hint.contains("7687"));
    }

    #[test]
    fn test_hint_for_connect_error() {
        let hint = get_hint("Connect error: timeout");
        assert!(hint.contains("Cannot connect to Neo4j"));
    }

    #[test]
    fn test_hint_for_not_found() {
        let hint = get_hint("Entity not found: qr-code");
        assert!(hint.contains("Verify entity key"));
        assert!(hint.contains("novanet_search"));
    }

    #[test]
    fn test_hint_for_no_such() {
        let hint = get_hint("No such node with key 'missing'");
        assert!(hint.contains("Entity not found"));
    }

    #[test]
    fn test_hint_for_timeout() {
        let hint = get_hint("Query timed out after 30s");
        assert!(hint.contains("LIMIT"));
        assert!(hint.contains("PROFILE"));
    }

    #[test]
    fn test_hint_for_timed_out() {
        let hint = get_hint("Operation timed out");
        assert!(hint.contains("Query timed out"));
    }

    #[test]
    fn test_hint_for_authentication() {
        let hint = get_hint("Authentication failed");
        assert!(hint.contains("NEO4J_USER"));
        assert!(hint.contains("NEO4J_PASSWORD"));
    }

    #[test]
    fn test_hint_for_unauthorized() {
        let hint = get_hint("Unauthorized access");
        assert!(hint.contains("Authentication failed"));
    }

    #[test]
    fn test_hint_for_memory() {
        let hint = get_hint("Out of memory");
        assert!(hint.contains("Memory limit"));
        assert!(hint.contains("LIMIT"));
    }

    #[test]
    fn test_hint_for_heap() {
        let hint = get_hint("heap space exceeded");
        assert!(hint.contains("Memory limit"));
        assert!(hint.contains("heap.max_size"));
    }

    #[test]
    fn test_hint_for_constraint() {
        let hint = get_hint("Constraint violation on Entity.key");
        assert!(hint.contains("MERGE"));
        assert!(hint.contains("SHOW CONSTRAINTS"));
    }

    #[test]
    fn test_hint_for_invalid_tool() {
        let hint = get_hint("Invalid tool: foo_bar");
        assert!(hint.contains("novanet_query"));
        assert!(hint.contains("novanet_introspect"));
    }

    #[test]
    fn test_hint_default() {
        let hint = get_hint("some unknown error xyz");
        assert!(hint.contains("Unexpected error"));
        assert!(hint.contains("novanet doctor"));
    }

    #[test]
    fn test_with_hint() {
        let result = with_hint("connection refused");
        assert!(result.contains("connection refused"));
        assert!(result.contains("NEO4J_URI"));
        // Verify the error comes before the hint
        let error_pos = result.find("connection refused").unwrap();
        let hint_pos = result.find("NEO4J_URI").unwrap();
        assert!(error_pos < hint_pos);
    }

    #[test]
    fn test_with_hint_preserves_original_message() {
        let original = "Custom error: something went wrong";
        let result = with_hint(original);
        assert!(result.starts_with(original));
    }

    #[test]
    fn test_hint_case_insensitive() {
        let hint1 = get_hint("CONNECTION REFUSED");
        let hint2 = get_hint("connection refused");
        let hint3 = get_hint("Connection Refused");

        // All should match the same hint
        assert!(hint1.contains("Cannot connect to Neo4j"));
        assert!(hint2.contains("Cannot connect to Neo4j"));
        assert!(hint3.contains("Cannot connect to Neo4j"));
    }

    // Write-specific hint tests (Task 1.2)
    #[test]
    fn test_hint_for_trait_not_writable() {
        let hint = get_hint("Class 'Entity' has trait 'defined' which is not writable");
        assert!(hint.contains("authored"));
        assert!(hint.contains("imported"));
    }

    #[test]
    fn test_hint_for_slug_locked() {
        let hint = get_hint("Slug is locked on 'block:head-seo-meta@fr-FR'");
        assert!(hint.contains("redirect"));
    }

    #[test]
    fn test_hint_for_singleton_violation() {
        let hint = get_hint("Singleton violation: Only one arc can have 'is_slug_source'");
        assert!(hint.contains("is_slug_source"));
    }

    #[test]
    fn test_hint_for_schema_not_found() {
        let hint = get_hint("Schema class not found: 'FooBar'");
        assert!(hint.contains("novanet_introspect"));
    }
}
