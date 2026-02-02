//! Filter build command: `novanet filter build`.
//!
//! Reads a JSON filter spec from stdin, resolves it to Cypher via the meta-graph
//! query builder, and writes the result to stdout. Designed for Studio subprocess
//! integration: `echo '{"realms":["project"]}' | novanet filter build`

use std::io::Read;

use crate::cypher;
use crate::facets::FacetFilter;

/// Read JSON from stdin, build faceted Cypher, write to stdout.
pub fn run_filter_build() -> crate::Result<()> {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .map_err(crate::NovaNetError::Io)?;

    let input = input.trim();
    if input.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "filter build: no JSON provided on stdin".to_string(),
        ));
    }

    let filter = FacetFilter::from_json(input)?;
    let stmt = cypher::filter_build_query(&filter);

    // Output inlined Cypher to stdout (ready for Studio to execute)
    println!("{}", stmt.render_inline());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_build_empty_json_uses_fallback() {
        // Directly test the query building (stdin test would be integration)
        let filter = FacetFilter::from_json(r#"{"realms":[]}"#).unwrap();
        let stmt = cypher::filter_build_query(&filter);
        assert!(stmt.cypher.contains("NOT n:Meta"));
    }

    #[test]
    fn filter_build_with_facets() {
        let filter =
            FacetFilter::from_json(r#"{"realms":["project"],"layers":["structure"]}"#).unwrap();
        let stmt = cypher::filter_build_query(&filter);
        assert!(stmt.cypher.contains("IN_REALM"));
        assert!(stmt.cypher.contains("IN_LAYER"));
        let inlined = stmt.render_inline();
        assert!(inlined.contains("'project'"));
        assert!(inlined.contains("'structure'"));
    }
}
