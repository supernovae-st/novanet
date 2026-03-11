//! Facet filter parsing and resolution.
//!
//! Parses comma-separated CLI flags or JSON (stdin) into a typed `FacetFilter`.
//! Used by `cypher.rs` to build faceted Cypher queries and by `novanet filter build`
//! for the Studio subprocess integration.

use serde::{Deserialize, Serialize};

/// Parsed facet filters from CLI flags or JSON stdin.
///
/// Each field holds zero or more values. Empty = no restriction on that axis.
/// Multiple values within a facet are OR-combined; facets are AND-combined.
///
/// Example: `realms=["shared","org"], layers=["knowledge"]`
///   → Classes that are (IN_REALM shared OR org) AND (IN_LAYER knowledge)
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct FacetFilter {
    #[serde(default)]
    pub realms: Vec<String>,
    #[serde(default)]
    pub layers: Vec<String>,
    // v0.17.3 (ADR-036): trait_filters removed - traits no longer in schema
    #[serde(default)]
    pub arc_families: Vec<String>,
    #[serde(default)]
    pub classes: Vec<String>,
}

impl FacetFilter {
    /// Parse from comma-separated CLI flag values.
    ///
    /// ```
    /// use novanet::facets::FacetFilter;
    /// let f = FacetFilter::from_cli(
    ///     Some("shared,org"), Some("knowledge"), None, None,
    /// );
    /// assert_eq!(f.realms, vec!["shared", "org"]);
    /// assert_eq!(f.layers, vec!["knowledge"]);
    /// ```
    #[must_use]
    pub fn from_cli(
        realm: Option<&str>,
        layer: Option<&str>,
        // v0.17.3 (ADR-036): trait_filter param removed
        arc_family: Option<&str>,
        class: Option<&str>,
    ) -> Self {
        Self {
            realms: parse_csv(realm),
            layers: parse_csv(layer),
            arc_families: parse_csv(arc_family),
            classes: parse_csv(class),
        }
    }

    /// Parse from JSON string (for `novanet filter build` stdin).
    pub fn from_json(json: &str) -> crate::Result<Self> {
        serde_json::from_str(json)
            .map_err(|e| crate::NovaNetError::Validation(format!("invalid filter JSON: {e}")))
    }

    /// Whether all facets are empty (no filters active).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.realms.is_empty()
            && self.layers.is_empty()
            // v0.17.3 (ADR-036): trait_filters removed
            && self.arc_families.is_empty()
            && self.classes.is_empty()
    }

    /// Count of active facet axes (0–4).
    /// v0.17.3 (ADR-036): Was 0-5, now 0-4 (trait_filters removed)
    #[must_use]
    pub fn active_count(&self) -> usize {
        [
            !self.realms.is_empty(),
            !self.layers.is_empty(),
            // v0.17.3 (ADR-036): trait_filters removed
            !self.arc_families.is_empty(),
            !self.classes.is_empty(),
        ]
        .iter()
        .filter(|&&v| v)
        .count()
    }
}

/// Split a comma-separated string into trimmed, non-empty values.
fn parse_csv(input: Option<&str>) -> Vec<String> {
    match input {
        Some(s) if !s.is_empty() => s
            .split(',')
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
            .collect(),
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_csv_none() {
        assert!(parse_csv(None).is_empty());
    }

    #[test]
    fn parse_csv_empty() {
        assert!(parse_csv(Some("")).is_empty());
    }

    #[test]
    fn parse_csv_single() {
        assert_eq!(parse_csv(Some("shared")), vec!["shared"]);
    }

    #[test]
    fn parse_csv_multiple_trimmed() {
        // v11.2: 2 realms (shared, org) - renamed from global, tenant
        assert_eq!(parse_csv(Some(" shared , org ")), vec!["shared", "org"]);
    }

    #[test]
    fn parse_csv_trailing_comma() {
        assert_eq!(parse_csv(Some("shared,")), vec!["shared"]);
    }

    #[test]
    fn from_cli_mixed() {
        // v0.17.3 (ADR-036): trait_filter param removed
        let f = FacetFilter::from_cli(
            Some("shared,org"),
            Some("knowledge"),
            None,
            Some("Locale,Expression"),
        );
        assert_eq!(f.realms, vec!["shared", "org"]);
        assert_eq!(f.layers, vec!["knowledge"]);
        assert!(f.arc_families.is_empty());
        assert_eq!(f.classes, vec!["Locale", "Expression"]);
        assert!(!f.is_empty());
        assert_eq!(f.active_count(), 3);
    }

    #[test]
    fn from_cli_empty_is_empty() {
        // v0.17.3 (ADR-036): trait_filter param removed
        let f = FacetFilter::from_cli(None, None, None, None);
        assert!(f.is_empty());
        assert_eq!(f.active_count(), 0);
    }

    #[test]
    fn from_json_full() {
        // v0.17.3 (ADR-036): traits removed from JSON
        let json = r#"{
            "realms": ["shared"],
            "layers": ["knowledge", "config"],
            "arc_families": [],
            "classes": []
        }"#;
        let f = FacetFilter::from_json(json).unwrap();
        assert_eq!(f.realms, vec!["shared"]);
        assert_eq!(f.layers, vec!["knowledge", "config"]);
        assert!(f.arc_families.is_empty());
        assert!(f.classes.is_empty());
    }

    #[test]
    fn from_json_minimal() {
        let json = r#"{"realms": ["org"]}"#;
        let f = FacetFilter::from_json(json).unwrap();
        assert_eq!(f.realms, vec!["org"]);
        assert!(f.layers.is_empty());
    }

    #[test]
    fn from_json_invalid() {
        assert!(FacetFilter::from_json("not json").is_err());
    }

    #[test]
    fn roundtrip_json() {
        // v0.17.3 (ADR-036): trait_filter param removed
        let original =
            FacetFilter::from_cli(Some("shared"), Some("knowledge"), Some("mining"), None);
        let json = serde_json::to_string(&original).unwrap();
        let parsed = FacetFilter::from_json(&json).unwrap();
        assert_eq!(original, parsed);
    }
}
