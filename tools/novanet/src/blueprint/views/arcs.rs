//! Arcs view — all arcs grouped by family.

use crate::blueprint::ascii::{arc_family_arrow, truncate};
use crate::blueprint::sources::BlueprintData;
use crate::parsers::arcs::ArcFamily;

/// Render arcs view.
pub fn render(data: &BlueprintData) -> String {
    let mut out = String::new();

    out.push_str(
        "╭──────────────────────────────────────────────────────────────────────────────╮\n",
    );
    out.push_str(
        "│  ◉ NOVANET ARCS                                                             │\n",
    );
    out.push_str(
        "│                                                                              │\n",
    );
    out.push_str(
        "│  All arc kinds grouped by family                                             │\n",
    );
    out.push_str(
        "╰──────────────────────────────────────────────────────────────────────────────╯\n\n",
    );

    let by_family = data.arcs_by_family();

    let families = [
        (ArcFamily::Ownership, "ownership", "Parent-child hierarchy"),
        (
            ArcFamily::Localization,
            "localization",
            "Invariant↔localized links",
        ),
        (ArcFamily::Semantic, "semantic", "Meaning connections"),
        (ArcFamily::Generation, "generation", "LLM pipeline"),
        (ArcFamily::Mining, "mining", "Knowledge extraction"),
    ];

    for (family_enum, family_key, description) in families {
        let arrow = arc_family_arrow(family_key);
        let arcs = by_family
            .get(&family_enum)
            .map(|v| v.as_slice())
            .unwrap_or(&[]);

        out.push_str(&format!(
            "┌──────────────────────────────────────────────────────────────────────────────┐\n\
             │  {} {} ({} arcs) — {}{}│\n\
             ├──────────────────────────────────────────────────────────────────────────────┤\n",
            arrow,
            family_key.to_uppercase(),
            arcs.len(),
            description,
            " ".repeat(
                80 - 20 - family_key.len() - description.len() - arcs.len().to_string().len()
            )
        ));

        if arcs.is_empty() {
            out.push_str("│  (no arcs in this family)                                                    │\n");
        } else {
            // Sort arcs by name
            let mut sorted_arcs: Vec<_> = arcs.iter().collect();
            sorted_arcs.sort_by_key(|a| &a.arc_type);

            for arc in sorted_arcs {
                let source_str = arc.source.labels().join("|");
                let target_str = arc.target.labels().join("|");
                let cardinality = format!(" [{}]", arc.cardinality);

                let line = format!(
                    "{}  {} → {}{}",
                    arc.arc_type,
                    truncate(&source_str, 20),
                    truncate(&target_str, 20),
                    cardinality
                );

                out.push_str(&format!(
                    "│  {}{}│\n",
                    line,
                    " ".repeat(80 - 4 - visible_len(&line))
                ));
            }
        }

        out.push_str(
            "└──────────────────────────────────────────────────────────────────────────────┘\n\n",
        );
    }

    // Summary
    out.push_str(
        "───────────────────────────────────────────────────────────────────────────────\n",
    );
    out.push_str(&format!("TOTAL: {} arc kinds\n", data.arc_count()));

    out
}

/// Get visible length (simple version).
fn visible_len(s: &str) -> usize {
    s.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arcs_view() {
        let root = crate::config::resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load blueprint data");

        let output = render(&data);

        assert!(output.contains("NOVANET ARCS"), "Should have header");
        assert!(output.contains("OWNERSHIP"), "Should have ownership family");
        assert!(
            output.contains("LOCALIZATION"),
            "Should have localization family"
        );
        assert!(output.contains("SEMANTIC"), "Should have semantic family");
        assert!(
            output.contains("GENERATION"),
            "Should have generation family"
        );
        assert!(output.contains("MINING"), "Should have mining family");
        assert!(output.contains("TOTAL:"), "Should have total");
    }
}
