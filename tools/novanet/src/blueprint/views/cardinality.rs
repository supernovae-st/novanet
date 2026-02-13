//! Cardinality view — 1:1, 1:N, N:M relationship constraints.

use crate::blueprint::sources::BlueprintData;
use crate::parsers::arcs::Cardinality;

/// Render cardinality view.
pub fn render(data: &BlueprintData) -> String {
    let mut out = String::new();

    out.push_str(
        "╭──────────────────────────────────────────────────────────────────────────────╮\n",
    );
    out.push_str(
        "│  ◉ NOVANET CARDINALITY                                                      │\n",
    );
    out.push_str(
        "│                                                                              │\n",
    );
    out.push_str(
        "│  Relationship constraints: 1:1, 1:N, N:M                                     │\n",
    );
    out.push_str(
        "╰──────────────────────────────────────────────────────────────────────────────╯\n\n",
    );

    let by_cardinality = data.arcs_by_cardinality();

    // 1:1 relationships
    out.push_str(
        "┌──────────────────────────────────────────────────────────────────────────────┐\n",
    );
    let one_to_one = by_cardinality
        .get(&Cardinality::OneToOne)
        .map(|v| v.len())
        .unwrap_or(0);
    out.push_str(&format!(
        "│  1:1 (One-to-One) — Exclusive pairing                          ({} arcs)    │\n",
        one_to_one
    ));
    out.push_str(
        "├──────────────────────────────────────────────────────────────────────────────┤\n",
    );
    if let Some(arcs) = by_cardinality.get(&Cardinality::OneToOne) {
        for arc in arcs.iter().take(10) {
            let source = arc.source.labels().join("|");
            let target = arc.target.labels().join("|");
            out.push_str(&format!(
                "│  {} ════════════════ {}{}│\n",
                pad_right(&source, 25),
                pad_right(&target, 25),
                " ".repeat(80 - 58)
            ));
            out.push_str(&format!(
                "│       {}{}│\n",
                arc.arc_type,
                " ".repeat(80 - 8 - arc.arc_type.len())
            ));
        }
        if arcs.len() > 10 {
            out.push_str(&format!(
                "│  ... and {} more{}│\n",
                arcs.len() - 10,
                " ".repeat(80 - 18 - (arcs.len() - 10).to_string().len())
            ));
        }
    } else {
        out.push_str(
            "│  (none)                                                                      │\n",
        );
    }
    out.push_str(
        "│                                                                              │\n",
    );
    out.push_str(
        "│  ═══ means exactly one on each side (per locale for authored nodes)           │\n",
    );
    out.push_str(
        "└──────────────────────────────────────────────────────────────────────────────┘\n\n",
    );

    // 1:N relationships
    out.push_str(
        "┌──────────────────────────────────────────────────────────────────────────────┐\n",
    );
    let one_to_many = by_cardinality
        .get(&Cardinality::OneToMany)
        .map(|v| v.len())
        .unwrap_or(0);
    out.push_str(&format!(
        "│  1:N (One-to-Many) — Parent owns children                       ({} arcs)   │\n",
        one_to_many
    ));
    out.push_str(
        "├──────────────────────────────────────────────────────────────────────────────┤\n",
    );
    if let Some(arcs) = by_cardinality.get(&Cardinality::OneToMany) {
        for arc in arcs.iter().take(10) {
            let source = arc.source.labels().join("|");
            let target = arc.target.labels().join("|");
            out.push_str(&format!(
                "│  {} ────────────────► {} (many){}│\n",
                pad_right(&source, 20),
                pad_right(&target, 20),
                " ".repeat(80 - 60)
            ));
            out.push_str(&format!(
                "│       {}{}│\n",
                arc.arc_type,
                " ".repeat(80 - 8 - arc.arc_type.len())
            ));
        }
        if arcs.len() > 10 {
            out.push_str(&format!(
                "│  ... and {} more{}│\n",
                arcs.len() - 10,
                " ".repeat(80 - 18 - (arcs.len() - 10).to_string().len())
            ));
        }
    } else {
        out.push_str(
            "│  (none)                                                                      │\n",
        );
    }
    out.push_str(
        "│                                                                              │\n",
    );
    out.push_str(
        "│  ───► means one parent, many children (ownership semantics)                  │\n",
    );
    out.push_str(
        "└──────────────────────────────────────────────────────────────────────────────┘\n\n",
    );

    // N:M relationships
    out.push_str(
        "┌──────────────────────────────────────────────────────────────────────────────┐\n",
    );
    let many_to_many = by_cardinality
        .get(&Cardinality::ManyToMany)
        .map(|v| v.len())
        .unwrap_or(0);
    out.push_str(&format!(
        "│  N:M (Many-to-Many) — Flexible associations                     ({} arcs)   │\n",
        many_to_many
    ));
    out.push_str(
        "├──────────────────────────────────────────────────────────────────────────────┤\n",
    );
    if let Some(arcs) = by_cardinality.get(&Cardinality::ManyToMany) {
        for arc in arcs.iter().take(10) {
            let source = arc.source.labels().join("|");
            let target = arc.target.labels().join("|");
            out.push_str(&format!(
                "│  {} ◄────────────────► {}{}│\n",
                pad_right(&source, 22),
                pad_right(&target, 22),
                " ".repeat(80 - 60)
            ));
            out.push_str(&format!(
                "│       {}{}│\n",
                arc.arc_type,
                " ".repeat(80 - 8 - arc.arc_type.len())
            ));
        }
        if arcs.len() > 10 {
            out.push_str(&format!(
                "│  ... and {} more{}│\n",
                arcs.len() - 10,
                " ".repeat(80 - 18 - (arcs.len() - 10).to_string().len())
            ));
        }
    } else {
        out.push_str(
            "│  (none)                                                                      │\n",
        );
    }
    out.push_str(
        "│                                                                              │\n",
    );
    out.push_str(
        "│  ◄───► means many on both sides (junction table semantics)                   │\n",
    );
    out.push_str(
        "└──────────────────────────────────────────────────────────────────────────────┘\n\n",
    );

    // Summary
    let total = one_to_one + one_to_many + many_to_many;
    let with_cardinality = total;
    let without = data.arc_count() - with_cardinality;

    out.push_str(
        "───────────────────────────────────────────────────────────────────────────────\n",
    );
    out.push_str("SUMMARY\n");
    out.push_str(&format!(
        "├── 1:1 arcs:  {} ({:.0}%)\n",
        one_to_one,
        pct(one_to_one, total)
    ));
    out.push_str(&format!(
        "├── 1:N arcs:  {} ({:.0}%)\n",
        one_to_many,
        pct(one_to_many, total)
    ));
    out.push_str(&format!(
        "├── N:M arcs:  {} ({:.0}%)\n",
        many_to_many,
        pct(many_to_many, total)
    ));
    out.push_str(&format!("└── Unspecified: {}\n", without));

    out
}

fn pct(value: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        (value as f64 / total as f64) * 100.0
    }
}

/// Pad string to width with spaces on the right.
fn pad_right(s: &str, width: usize) -> String {
    if s.len() >= width {
        s[..width].to_string()
    } else {
        format!("{}{}", s, " ".repeat(width - s.len()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cardinality_view() {
        let root = crate::config::resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load blueprint data");

        let output = render(&data);

        assert!(output.contains("NOVANET CARDINALITY"), "Should have header");
        assert!(output.contains("1:1"), "Should have 1:1 section");
        assert!(output.contains("1:N"), "Should have 1:N section");
        assert!(output.contains("N:M"), "Should have N:M section");
        assert!(output.contains("SUMMARY"), "Should have summary");
    }
}
