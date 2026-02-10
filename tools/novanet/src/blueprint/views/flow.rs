//! Flow view — 6 data flow diagrams.

use crate::blueprint::sources::BlueprintData;

/// Render flow view with all 6 diagrams.
pub fn render(_data: &BlueprintData) -> String {
    let mut out = String::new();

    out.push_str(&render_header());
    out.push('\n');
    out.push_str(&render_ownership_flow());
    out.push('\n');
    out.push_str(&render_localization_flow());
    out.push('\n');
    out.push_str(&render_knowledge_flow());
    out.push('\n');
    out.push_str(&render_generation_flow());
    out.push('\n');
    out.push_str(&render_seo_flow());
    out.push('\n');
    out.push_str(&render_cross_realm_flow());
    out.push('\n');
    out.push_str(&render_footer());

    out
}

fn render_header() -> String {
    "╭──────────────────────────────────────────────────────────────────────────────╮\n\
     │  ◉ NOVANET FLOWS                                                             │\n\
     │                                                                              │\n\
     │  6 data flow diagrams showing how content moves through the graph            │\n\
     ╰──────────────────────────────────────────────────────────────────────────────╯".to_string()
}

fn render_ownership_flow() -> String {
    // v11.0: ProjectL10n → ProjectContent, HAS_L10N → HAS_CONTENT
    "┌──────────────────────────────────────────────────────────────────────────────┐\n\
     │  1. OWNERSHIP FLOW — Who owns what (tenant hierarchy)                        │\n\
     ├──────────────────────────────────────────────────────────────────────────────┤\n\
     │                                                                              │\n\
     │  Org                                                                      │\n\
     │    │                                                                         │\n\
     │    ├──[HAS_PROJECT]──► Project ──[HAS_CONTENT]──► ProjectContent             │\n\
     │    │                      │                                                  │\n\
     │    │                      ├──[HAS_PAGE]──► Page ──[HAS_BLOCK]──► Block       │\n\
     │    │                      │                                                  │\n\
     │    │                      └──[HAS_ENTITY]──► Entity                          │\n\
     │    │                                                                         │\n\
     │    └──[HAS_CONFIG]──► OrgConfig                                           │\n\
     │                                                                              │\n\
     │  💡 Ownership = parent controls lifecycle. Delete Project = delete Pages.    │\n\
     │                                                                              │\n\
     └──────────────────────────────────────────────────────────────────────────────┘".to_string()
}

fn render_localization_flow() -> String {
    "┌──────────────────────────────────────────────────────────────────────────────┐\n\
     │  2. LOCALIZATION FLOW — Invariant → Locale-specific content                  │\n\
     ├──────────────────────────────────────────────────────────────────────────────┤\n\
     │                                                                              │\n\
     │  ┌─────────────────────────────────────────────────────────────────────────┐ │\n\
     │  │ INVARIANT (defined 1×)              LOCALIZED (exists per locale)       │ │\n\
     │  ├─────────────────────────────────────────────────────────────────────────┤ │\n\
     │  │                                                                         │ │\n\
     │  │  Entity ─────[HAS_CONTENT]─────►  EntityContent                         │ │\n\
     │  │  (key: \"qr-generator\")            (key: \"entity:qr-generator@fr-FR\")    │ │\n\
     │  │                                   (key: \"entity:qr-generator@de-DE\")    │ │\n\
     │  │                                   (key: \"entity:qr-generator@ja-JP\")    │ │\n\
     │  │                                              ...200+ locales            │ │\n\
     │  │                                                                         │ │\n\
     │  └─────────────────────────────────────────────────────────────────────────┘ │\n\
     │                                                                              │\n\
     │  💡 NOT translation! Native generation. fr-FR content born French.           │\n\
     │                                                                              │\n\
     └──────────────────────────────────────────────────────────────────────────────┘".to_string()
}

fn render_knowledge_flow() -> String {
    "┌──────────────────────────────────────────────────────────────────────────────┐\n\
     │  3. KNOWLEDGE FLOW — How locale knowledge is structured                      │\n\
     ├──────────────────────────────────────────────────────────────────────────────┤\n\
     │                                                                              │\n\
     │  Locale (fr-FR)                                                              │\n\
     │    │                                                                         │\n\
     │    ├──[HAS_TERMS]──► TermSet ──[CONTAINS_TERM]──► Term ◊                     │\n\
     │    │                 (domain: \"tech\")              \"QR code\" → \"code QR\"     │\n\
     │    │                                               \"scan\" → \"scanner\"        │\n\
     │    │                                                                         │\n\
     │    ├──[HAS_EXPRESSIONS]──► ExpressionSet ──[CONTAINS_EXPRESSION]──► Expr     │\n\
     │    │                       (register: \"formal\")     \"Veuillez scanner...\"    │\n\
     │    │                                                                         │\n\
     │    ├──[HAS_PATTERNS]──► PatternSet ──[CONTAINS_PATTERN]──► Pattern           │\n\
     │    │                                  \"{product} vous permet de {action}\"    │\n\
     │    │                                                                         │\n\
     │    ├──[HAS_CULTURE]──► CultureSet ──[CONTAINS_CULTURE_REF]──► CultureRef     │\n\
     │    │                                 \"Politeness level: vous > tu\"           │\n\
     │    │                                                                         │\n\
     │    └──[HAS_TABOOS]──► TabooSet ──[CONTAINS_TABOO]──► Taboo                   │\n\
     │                                   \"Avoid: 'cheap', 'foreign'\"                │\n\
     │                                                                              │\n\
     │  💡 Atoms are LOCALE-NATIVE. fr-FR has 20K terms, sw-KE has 500.             │\n\
     │                                                                              │\n\
     └──────────────────────────────────────────────────────────────────────────────┘".to_string()
}

fn render_generation_flow() -> String {
    "┌──────────────────────────────────────────────────────────────────────────────┐\n\
     │  4. GENERATION FLOW — LLM pipeline from content to output                    │\n\
     ├──────────────────────────────────────────────────────────────────────────────┤\n\
     │                                                                              │\n\
     │         INPUTS                      PROCESS                    OUTPUT        │\n\
     │  ┌─────────────────┐          ┌─────────────────┐       ┌─────────────────┐  │\n\
     │  │                 │          │                 │       │                 │  │\n\
     │  │ EntityContent   │─────────►│                 │       │ PageGenerated   │  │\n\
     │  │ (semantic)      │          │   GenerationJob │──────►│ (output layer)  │  │\n\
     │  │                 │          │   ○ job trait   │       │                 │  │\n\
     │  │ Term ◊          │─────────►│                 │       │ BlockGenerated  │  │\n\
     │  │ Expression ◊    │          │   Uses:         │       │                 │  │\n\
     │  │ Pattern ◊       │─────────►│   - LLM model   │       └─────────────────┘  │\n\
     │  │                 │          │   - Instruction │              │             │\n\
     │  │ Instruction     │─────────►│   - Locale ctx  │              │             │\n\
     │  │ (prompts)       │          │                 │              ▼             │\n\
     │  │                 │          └─────────────────┘       ┌─────────────────┐  │\n\
     │  │ Page (struct)   │────────────────────────────────────│ Final render    │  │\n\
     │  │ Block (struct)  │                                    │ (HTML/JSON)     │  │\n\
     │  └─────────────────┘                                    └─────────────────┘  │\n\
     │                                                                              │\n\
     │  💡 Selective loading: LLM gets 50 relevant Terms, not 20K JSON blob.        │\n\
     │                                                                              │\n\
     └──────────────────────────────────────────────────────────────────────────────┘".to_string()
}

fn render_seo_flow() -> String {
    "┌──────────────────────────────────────────────────────────────────────────────┐\n\
     │  5. SEO FLOW — Keywords and geo-targeting                                    │\n\
     ├──────────────────────────────────────────────────────────────────────────────┤\n\
     │                                                                              │\n\
     │  SEOKeyword ◄──[HAS_SEO_KEYWORDS]── Page                                     │\n\
     │      │                                │                                      │\n\
     │      │                                ├──[HAS_GEO_QUERIES]──► GeoQuery       │\n\
     │      │                                │                                      │\n\
     │      ▼                                └──[HAS_SEARCH_INTENT]──► SearchIntent │\n\
     │  SEOCluster                                                                  │\n\
     │      │                                                                       │\n\
     │      └──[TARGETS_LOCALE]──► Locale                                           │\n\
     │                                                                              │\n\
     │  💡 SEO is tenant-specific (v11.0). Keywords for qrcode-ai.com,              │\n\
     │     not universal knowledge.                                                 │\n\
     │                                                                              │\n\
     └──────────────────────────────────────────────────────────────────────────────┘".to_string()
}

fn render_cross_realm_flow() -> String {
    "┌──────────────────────────────────────────────────────────────────────────────┐\n\
     │  6. CROSS-REALM FLOW — Shared knowledge used by tenant content               │\n\
     ├──────────────────────────────────────────────────────────────────────────────┤\n\
     │                                                                              │\n\
     │  ╔═══════════════════════════════════════════════════════════════════════╗   │\n\
     │  ║  SHARED REALM (read-only)                                             ║   │\n\
     │  ║                                                                       ║   │\n\
     │  ║  Locale ──► LocaleVoice ──► TermSet ──► Term                          ║   │\n\
     │  ║  (fr-FR)    (formal/tu)     (tech)      (\"scanner\")                   ║   │\n\
     │  ║                                                                       ║   │\n\
     │  ╚═══════════════════════════╦═══════════════════════════════════════════╝   │\n\
     │                              ║                                               │\n\
     │                    [USES_TERM]  (cross-realm arc)                            │\n\
     │                              ║                                               │\n\
     │  ╔═══════════════════════════▼═══════════════════════════════════════════╗   │\n\
     │  ║  ORG REALM (read-write)                                            ║   │\n\
     │  ║                                                                       ║   │\n\
     │  ║  EntityContent ─────────────────► uses global Terms                   ║   │\n\
     │  ║  (tenant-specific meaning)        (universal vocabulary)              ║   │\n\
     │  ║                                                                       ║   │\n\
     │  ╚═══════════════════════════════════════════════════════════════════════╝   │\n\
     │                                                                              │\n\
     │  💡 Shared = shared across all tenants. Org = your business data.         │\n\
     │                                                                              │\n\
     └──────────────────────────────────────────────────────────────────────────────┘".to_string()
}

fn render_footer() -> String {
    "╭──────────────────────────────────────────────────────────────────────────────╮\n\
     │  📖 Other views: --view=tree | arcs | content | cardinality | glossary      │\n\
     ╰──────────────────────────────────────────────────────────────────────────────╯".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow_view() {
        let root = crate::config::resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load blueprint data");

        let output = render(&data);

        assert!(output.contains("NOVANET FLOWS"), "Should have header");
        assert!(output.contains("OWNERSHIP FLOW"), "Should have ownership flow");
        assert!(output.contains("LOCALIZATION FLOW"), "Should have localization flow");
        assert!(output.contains("KNOWLEDGE FLOW"), "Should have knowledge flow");
        assert!(output.contains("GENERATION FLOW"), "Should have generation flow");
        assert!(output.contains("SEO FLOW"), "Should have SEO flow");
        assert!(output.contains("CROSS-REALM FLOW"), "Should have cross-realm flow");
    }
}
