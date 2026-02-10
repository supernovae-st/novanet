//! Glossary view — concept definitions.

use crate::blueprint::ascii::{truncate, pad_right};
use crate::blueprint::sources::BlueprintData;

/// Render glossary view.
pub fn render(data: &BlueprintData) -> String {
    let mut out = String::new();

    out.push_str("╭──────────────────────────────────────────────────────────────────────────────╮\n");
    out.push_str("│  ◉ NOVANET GLOSSARY                                                         │\n");
    out.push_str("│                                                                              │\n");
    out.push_str("│  Definitions of core concepts in the NovaNet meta-graph                      │\n");
    out.push_str("╰──────────────────────────────────────────────────────────────────────────────╯\n\n");

    // Realms
    out.push_str("┌──────────────────────────────────────────────────────────────────────────────┐\n");
    out.push_str("│  REALM                                                                       │\n");
    out.push_str("├──────────────────────────────────────────────────────────────────────────────┤\n");
    out.push_str("│  WHERE a node lives. Determines ownership and access control.                │\n");
    out.push_str("│                                                                              │\n");
    for realm in &data.taxonomy.node_realms {
        let layers: Vec<_> = realm.layers.iter().map(|l| l.key.as_str()).collect();
        out.push_str(&format!(
            "│  ◉ {:<10} — {}│\n",
            realm.key,
            pad_right(&realm.llm_context, 60)
        ));
        out.push_str(&format!(
            "│               Layers: {}│\n",
            pad_right(&layers.join(", "), 57)
        ));
    }
    out.push_str("└──────────────────────────────────────────────────────────────────────────────┘\n\n");

    // Layers
    out.push_str("┌──────────────────────────────────────────────────────────────────────────────┐\n");
    out.push_str("│  LAYER                                                                       │\n");
    out.push_str("├──────────────────────────────────────────────────────────────────────────────┤\n");
    out.push_str("│  WHAT category a node belongs to. Functional grouping within a realm.        │\n");
    out.push_str("│                                                                              │\n");
    for realm in &data.taxonomy.node_realms {
        out.push_str(&format!("│  {} {}:{}│\n",
            if realm.key == "shared" { "◉" } else { "◎" },
            realm.key.to_uppercase(),
            " ".repeat(80 - 6 - realm.key.len())
        ));
        for layer in &realm.layers {
            out.push_str(&format!(
                "│    {} {:<18} — {}│\n",
                layer.emoji,
                layer.key,
                pad_right(&truncate(&layer.llm_context, 45), 45)
            ));
        }
    }
    out.push_str("└──────────────────────────────────────────────────────────────────────────────┘\n\n");

    // Traits
    out.push_str("┌──────────────────────────────────────────────────────────────────────────────┐\n");
    out.push_str("│  TRAIT                                                                       │\n");
    out.push_str("├──────────────────────────────────────────────────────────────────────────────┤\n");
    out.push_str("│  HOW a node behaves with respect to locales. Determines locale handling.     │\n");
    out.push_str("│                                                                              │\n");
    for trait_def in &data.taxonomy.node_traits {
        let symbol = match trait_def.key.as_str() {
            "invariant" => "■",
            "localized" => "□",
            "knowledge" => "◊",
            "derived" => "◇",
            "job" => "○",
            _ => "?",
        };
        out.push_str(&format!(
            "│  {} {:<10} — {}│\n",
            symbol,
            trait_def.key,
            pad_right(&truncate(&trait_def.llm_context, 55), 55)
        ));
    }
    out.push_str("└──────────────────────────────────────────────────────────────────────────────┘\n\n");

    // Arc Families
    out.push_str("┌──────────────────────────────────────────────────────────────────────────────┐\n");
    out.push_str("│  ARC FAMILY                                                                  │\n");
    out.push_str("├──────────────────────────────────────────────────────────────────────────────┤\n");
    out.push_str("│  WHAT type of relationship an arc represents. Functional grouping of arcs.   │\n");
    out.push_str("│                                                                              │\n");
    for family in &data.taxonomy.arc_families {
        let arrow = match family.key.as_str() {
            "ownership" => "→",
            "localization" => "⇢",
            "semantic" => "⇄",
            "generation" => "⇉",
            "mining" => "⇶",
            _ => "→",
        };
        out.push_str(&format!(
            "│  {} {:<14} — {}│\n",
            arrow,
            family.key,
            pad_right(&truncate(&family.llm_context, 50), 50)
        ));
    }
    out.push_str("└──────────────────────────────────────────────────────────────────────────────┘\n\n");

    // Key Patterns
    out.push_str("┌──────────────────────────────────────────────────────────────────────────────┐\n");
    out.push_str("│  KEY PATTERNS                                                                │\n");
    out.push_str("├──────────────────────────────────────────────────────────────────────────────┤\n");
    out.push_str("│  Node keys follow consistent patterns based on trait:                        │\n");
    out.push_str("│                                                                              │\n");
    out.push_str("│  ■ Invariant:  {kind-key}                    → \"homepage\", \"qr-generator\"    │\n");
    out.push_str("│  □ Localized:  {kind}:{invariant}@{locale}   → \"entity:qr-gen@fr-FR\"         │\n");
    out.push_str("│  ◊ Knowledge:  {locale}:{domain}:{key}       → \"fr-FR:tech:scanner\"          │\n");
    out.push_str("│  ◇ Derived:    {kind}:{invariant}@{locale}   → \"page:home@fr-FR\"             │\n");
    out.push_str("│  ○ Job:        {type}:{target}:{timestamp}   → \"gen:page-home:2025...\"       │\n");
    out.push_str("└──────────────────────────────────────────────────────────────────────────────┘\n\n");

    // Core Principle
    out.push_str("┌──────────────────────────────────────────────────────────────────────────────┐\n");
    out.push_str("│  CORE PRINCIPLE: Generation, NOT Translation                                 │\n");
    out.push_str("├──────────────────────────────────────────────────────────────────────────────┤\n");
    out.push_str("│                                                                              │\n");
    out.push_str("│  ❌ WRONG:  Source → Translate → Target                                      │\n");
    out.push_str("│  ✓ RIGHT:  Entity (invariant) → Generate natively → EntityContent (local)    │\n");
    out.push_str("│                                                                              │\n");
    out.push_str("│  Content is BORN in each locale, not translated from a source language.      │\n");
    out.push_str("│  fr-FR content is authentically French, not English-translated-to-French.    │\n");
    out.push_str("│                                                                              │\n");
    out.push_str("└──────────────────────────────────────────────────────────────────────────────┘");

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glossary_view() {
        let root = crate::config::resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load blueprint data");

        let output = render(&data);

        assert!(output.contains("NOVANET GLOSSARY"), "Should have header");
        assert!(output.contains("REALM"), "Should have realm section");
        assert!(output.contains("LAYER"), "Should have layer section");
        assert!(output.contains("TRAIT"), "Should have trait section");
        assert!(output.contains("ARC FAMILY"), "Should have arc family section");
        assert!(output.contains("Generation, NOT Translation"), "Should have core principle");
    }
}
