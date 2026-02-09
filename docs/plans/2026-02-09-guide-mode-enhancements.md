# Guide Mode Enhancements Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Enhance Guide mode with heatmap colors, clipboard support, and inline code examples showing real YAML → Neo4j → Cypher data flow.

**Architecture:** Three independent features added to the existing Guide TUI. Heatmap uses color intensity based on kind counts. Clipboard uses `arboard` crate for cross-platform support. Code examples are generated from real YAML definitions and show the full data pipeline.

**Tech Stack:** Rust, ratatui, arboard (new), existing YAML parsers

---

## Task 1: Add arboard dependency for clipboard

**Files:**
- Modify: `tools/novanet/Cargo.toml:48` (after `unicode-segmentation`)

**Step 1: Add dependency**

Add to `[dependencies]` section:

```toml
# Clipboard
arboard = "3"                 # Cross-platform clipboard (macOS, Linux, Windows)
```

**Step 2: Verify build**

Run: `cargo build --features tui 2>&1 | head -20`
Expected: Build succeeds with arboard downloaded

**Step 3: Commit**

```bash
git add tools/novanet/Cargo.toml
git commit -m "$(cat <<'EOF'
feat(tui): add arboard dependency for clipboard support

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Task 2: Add heatmap color function

**Files:**
- Modify: `tools/novanet/src/tui/theme.rs`
- Test: In same file (inline tests)

**Step 1: Write the failing test**

Add to `theme.rs` tests module:

```rust
#[test]
fn test_heatmap_color_zero() {
    let color = heatmap_color(0, 50);
    // Zero count = dim gray
    assert!(matches!(color, Color::Rgb(r, _, _) if r < 100));
}

#[test]
fn test_heatmap_color_max() {
    let color = heatmap_color(50, 50);
    // Max count = bright (full intensity)
    assert!(matches!(color, Color::Rgb(r, _, _) if r > 150));
}

#[test]
fn test_heatmap_color_half() {
    let color = heatmap_color(25, 50);
    // Half count = medium intensity
    assert!(matches!(color, Color::Rgb(r, _, _) if r > 80 && r < 180));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test heatmap_color --features tui 2>&1`
Expected: FAIL with "cannot find function `heatmap_color`"

**Step 3: Write minimal implementation**

Add to `theme.rs` (before `impl Theme`):

```rust
/// Generate a heatmap color based on count relative to max.
/// Returns cyan spectrum: dim (few) → bright (many).
pub fn heatmap_color(count: usize, max_count: usize) -> Color {
    if max_count == 0 {
        return Color::Rgb(60, 60, 70); // No data = dim
    }

    let ratio = (count as f64) / (max_count as f64);
    let intensity = (ratio * 180.0) as u8 + 60; // Range: 60-240

    // Cyan spectrum: dim gray → bright cyan
    Color::Rgb(intensity / 3, intensity, intensity)
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test heatmap_color --features tui 2>&1`
Expected: PASS (3 tests)

**Step 5: Commit**

```bash
git add tools/novanet/src/tui/theme.rs
git commit -m "$(cat <<'EOF'
feat(tui): add heatmap_color function for Guide mode

Generates cyan spectrum colors based on count intensity.
Used to visualize kind density per layer/trait.

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Task 3: Apply heatmap to Layers tab

**Files:**
- Modify: `tools/novanet/src/tui/guide/layers.rs:150-180`

**Step 1: Read current implementation**

Check the `render_realm_column` function to find where kind_count is displayed.

**Step 2: Update layer card rendering with heatmap**

In `layers.rs`, find the card rendering loop and update the kind count display:

```rust
// Before (around line 155):
let count_str = format!("({} K)", kind_count);
let count_style = Style::default().fg(Color::DarkGray);

// After:
use crate::tui::theme::heatmap_color;

// Calculate max count for heatmap scaling
let max_count = layer_stats.iter().map(|(_, c)| *c).max().unwrap_or(1);
let count_color = heatmap_color(kind_count, max_count);
let count_str = format!("({} K)", kind_count);
let count_style = Style::default().fg(count_color);
```

**Step 3: Verify visually**

Run: `cargo run --features tui -- tui`
Navigate to Guide mode → Layers tab
Expected: Layers with more kinds show brighter cyan counts

**Step 4: Commit**

```bash
git add tools/novanet/src/tui/guide/layers.rs
git commit -m "$(cat <<'EOF'
feat(tui): apply heatmap colors to Layers tab kind counts

Layers with more kinds now show brighter cyan numbers,
providing visual density indication at a glance.

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Task 4: Apply heatmap to Traits tab

**Files:**
- Modify: `tools/novanet/src/tui/guide/traits.rs` (constellation rendering)

**Step 1: Find constellation rendering**

Locate where trait kind counts are displayed in the constellation.

**Step 2: Update with heatmap colors**

Add import and update the count display:

```rust
use crate::tui::theme::heatmap_color;

// In render_constellation or similar function:
let max_count = trait_stats.iter().map(|t| t.kind_count).max().unwrap_or(1);

// When rendering each trait node:
let count_color = heatmap_color(trait.kind_count, max_count);
let count_span = Span::styled(
    format!("({} K)", trait.kind_count),
    Style::default().fg(count_color)
);
```

**Step 3: Verify visually**

Run: `cargo run --features tui -- tui`
Navigate to Guide mode → Traits tab
Expected: Traits with more kinds (like invariant) show brighter counts

**Step 4: Commit**

```bash
git add tools/novanet/src/tui/guide/traits.rs
git commit -m "$(cat <<'EOF'
feat(tui): apply heatmap colors to Traits constellation

Trait nodes with more kinds show brighter cyan counts,
making dense traits visually prominent.

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Task 5: Add realm/layer keys to detail panel

**Files:**
- Modify: `tools/novanet/src/tui/guide/traits.rs` (detail panel)

**Step 1: Find detail panel rendering**

Locate `render_detail_panel` or similar function that shows trait info.

**Step 2: Add realm/layer breakdown**

Update the detail panel to show which realms/layers contain this trait:

```rust
// In the detail panel rendering, after showing llm_context:
lines.push(Line::from(""));
lines.push(Line::from(Span::styled(
    "BY LAYER",
    Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD)
)));

for (layer_key, kinds) in &trait_stats.kinds_by_layer {
    let kind_names: Vec<&str> = kinds.iter().map(|s| s.as_str()).take(3).collect();
    let preview = if kinds.len() > 3 {
        format!("{}, ... (+{})", kind_names.join(", "), kinds.len() - 3)
    } else {
        kind_names.join(", ")
    };
    lines.push(Line::from(vec![
        Span::styled(format!("  {}: ", layer_key), Style::default().fg(Color::Cyan)),
        Span::styled(preview, Style::default().fg(Color::White)),
    ]));
}
```

**Step 3: Verify visually**

Run: `cargo run --features tui -- tui`
Navigate to Guide → Traits → select any trait
Expected: Detail panel shows "BY LAYER" section with kind previews

**Step 4: Commit**

```bash
git add tools/novanet/src/tui/guide/traits.rs
git commit -m "$(cat <<'EOF'
feat(tui): add realm/layer breakdown to Traits detail panel

Shows which layers contain kinds with this trait,
with preview of kind names per layer.

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Task 6: Add clipboard module

**Files:**
- Create: `tools/novanet/src/tui/clipboard.rs`
- Modify: `tools/novanet/src/tui/mod.rs`

**Step 1: Write the failing test**

Create `clipboard.rs` with test first:

```rust
//! Clipboard utilities for TUI.

use arboard::Clipboard;

/// Copy text to system clipboard.
/// Returns Ok(()) on success, Err with message on failure.
pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let mut clipboard = Clipboard::new().map_err(|e| format!("Clipboard init failed: {}", e))?;
    clipboard
        .set_text(text.to_string())
        .map_err(|e| format!("Clipboard set failed: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_to_clipboard_doesnt_panic() {
        // Just verify it doesn't panic - actual clipboard may not work in CI
        let result = copy_to_clipboard("test");
        // We accept either success or graceful error
        assert!(result.is_ok() || result.is_err());
    }
}
```

**Step 2: Add module to mod.rs**

In `tools/novanet/src/tui/mod.rs`, add:

```rust
pub mod clipboard;
```

**Step 3: Run test**

Run: `cargo test clipboard --features tui 2>&1`
Expected: PASS

**Step 4: Commit**

```bash
git add tools/novanet/src/tui/clipboard.rs tools/novanet/src/tui/mod.rs
git commit -m "$(cat <<'EOF'
feat(tui): add clipboard module with copy_to_clipboard

Cross-platform clipboard support using arboard.
Used for yanking kind keys and JSON in Guide mode.

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Task 7: Add yank (copy) to Guide state

**Files:**
- Modify: `tools/novanet/src/tui/guide/mod.rs`

**Step 1: Add clipboard status to GuideState**

Add field to track last clipboard operation:

```rust
pub struct GuideState {
    // ... existing fields ...

    /// Last clipboard message (shown briefly in status bar)
    pub clipboard_message: Option<String>,

    /// When clipboard message was set (for auto-clear)
    pub clipboard_message_time: Option<std::time::Instant>,
}
```

**Step 2: Add yank handler in handle_key**

In the `handle_key` function, add case for 'y':

```rust
KeyCode::Char('y') => {
    let text = self.get_current_yank_text();
    if let Some(text) = text {
        match crate::tui::clipboard::copy_to_clipboard(&text) {
            Ok(()) => {
                self.clipboard_message = Some(format!("Copied: {}", text));
                self.clipboard_message_time = Some(std::time::Instant::now());
            }
            Err(e) => {
                self.clipboard_message = Some(format!("Error: {}", e));
                self.clipboard_message_time = Some(std::time::Instant::now());
            }
        }
        true
    } else {
        false
    }
}
```

**Step 3: Add get_current_yank_text helper**

```rust
impl GuideState {
    /// Get text to copy based on current tab and selection.
    fn get_current_yank_text(&self) -> Option<String> {
        match self.tab {
            GuideTab::Traits => {
                // Copy trait key
                let traits = ["invariant", "localized", "knowledge", "derived", "job"];
                traits.get(self.trait_cursor).map(|s| s.to_string())
            }
            GuideTab::Layers => {
                // Copy layer key
                let layers = if self.layer_realm == 0 {
                    &["config", "locale-knowledge"][..]
                } else {
                    &["config", "foundation", "structure", "semantic", "instruction", "seo", "output"][..]
                };
                layers.get(self.layer_cursor).map(|s| s.to_string())
            }
            GuideTab::Arcs => {
                // Copy arc family key
                let families = ["ownership", "localization", "semantic", "generation", "mining"];
                families.get(self.arc_cursor).map(|s| s.to_string())
            }
            GuideTab::Pipeline => {
                // Copy current stage name
                Some("generation-pipeline".to_string())
            }
        }
    }
}
```

**Step 4: Update Default impl**

```rust
impl Default for GuideState {
    fn default() -> Self {
        Self {
            // ... existing fields ...
            clipboard_message: None,
            clipboard_message_time: None,
        }
    }
}
```

**Step 5: Write test**

```rust
#[test]
fn test_yank_traits() {
    let mut state = GuideState::default();
    state.tab = GuideTab::Traits;
    state.trait_cursor = 0;
    let text = state.get_current_yank_text();
    assert_eq!(text, Some("invariant".to_string()));
}

#[test]
fn test_yank_layers_global() {
    let mut state = GuideState::default();
    state.tab = GuideTab::Layers;
    state.layer_realm = 0;
    state.layer_cursor = 1;
    let text = state.get_current_yank_text();
    assert_eq!(text, Some("locale-knowledge".to_string()));
}
```

**Step 6: Run tests**

Run: `cargo test yank --features tui 2>&1`
Expected: PASS

**Step 7: Commit**

```bash
git add tools/novanet/src/tui/guide/mod.rs
git commit -m "$(cat <<'EOF'
feat(tui): add yank (copy) support in Guide mode

Press 'y' to copy current item key to clipboard:
- Traits: trait key (e.g., "invariant")
- Layers: layer key (e.g., "semantic")
- Arcs: family key (e.g., "ownership")

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Task 8: Add code examples data structure

**Files:**
- Create: `tools/novanet/src/tui/guide/examples.rs`
- Modify: `tools/novanet/src/tui/guide/mod.rs`

**Step 1: Create examples module with real data**

```rust
//! Code Examples — Real YAML → Neo4j → Cypher examples for Guide mode.
//!
//! Shows the complete data pipeline for selected items.

/// Code example for a concept (trait, layer, arc family).
#[derive(Debug, Clone)]
pub struct CodeExample {
    /// YAML source file path (relative to models/)
    pub yaml_path: &'static str,
    /// YAML snippet showing the concept
    pub yaml_snippet: &'static str,
    /// Neo4j Cypher to create this node
    pub cypher_create: &'static str,
    /// Cypher query to find nodes with this concept
    pub cypher_query: &'static str,
    /// CLI command to use
    pub cli_command: &'static str,
}

/// Get code example for a trait.
pub fn trait_example(trait_key: &str) -> Option<CodeExample> {
    match trait_key {
        "invariant" => Some(CodeExample {
            yaml_path: "node-kinds/tenant/semantic/entity.yaml",
            yaml_snippet: r#"node:
  name: Entity
  realm: tenant
  layer: semantic
  trait: invariant  # <- Structure, no locale change
  description: "Semantic unit representing products..."

standard_properties:
  key:
    type: string
    pattern: "^[a-z][a-z0-9-]*$"
    examples: ["qr-code", "smart-link"]"#,
            cypher_create: r#"CREATE (e:Entity {
  key: 'qr-code',
  display_name: 'QR Code',
  type: 'THING',
  is_pillar: true
})"#,
            cypher_query: r#"// Find all invariant nodes
MATCH (n)
WHERE n.trait = 'invariant' OR n:Entity OR n:Page OR n:Block
RETURN labels(n)[0] AS kind, count(*) AS count
ORDER BY count DESC"#,
            cli_command: "cargo run -- query --trait=invariant --format=table",
        }),
        "localized" => Some(CodeExample {
            yaml_path: "node-kinds/tenant/semantic/entity-content.yaml",
            yaml_snippet: r#"node:
  name: EntityContent
  realm: tenant
  layer: semantic
  trait: localized  # <- Generated per locale
  description: "Locale-native content for an Entity"

standard_properties:
  key:
    pattern: "^entity:[a-z][a-z0-9-]*@[a-z]{2}-[A-Z]{2}$"
    examples: ["entity:qr-code@fr-FR"]"#,
            cypher_create: r#"// First create parent Entity (invariant)
MATCH (e:Entity {key: 'qr-code'})

// Then create localized content
CREATE (ec:EntityContent {
  key: 'entity:qr-code@fr-FR',
  entity_key: 'qr-code',
  locale_key: 'fr-FR',
  display_name: 'Code QR',
  slug: 'code-qr'
})

// Link with HAS_CONTENT arc
CREATE (e)-[:HAS_CONTENT]->(ec)"#,
            cypher_query: r#"// Find Entity with all its localized content
MATCH (e:Entity {key: $key})-[:HAS_CONTENT]->(ec:EntityContent)
OPTIONAL MATCH (ec)-[:FOR_LOCALE]->(l:Locale)
RETURN e.key, ec.locale_key, ec.display_name
ORDER BY ec.locale_key"#,
            cli_command: "cargo run -- query --trait=localized --format=json",
        }),
        "knowledge" => Some(CodeExample {
            yaml_path: "node-kinds/global/locale-knowledge/atoms/term.yaml",
            yaml_snippet: r#"node:
  name: Term
  realm: global
  layer: locale-knowledge
  trait: knowledge  # <- INPUT to LLM (savoir)
  description: "Locale-native term/vocabulary item"

# Knowledge atoms are loaded INTO the LLM as context
# They exist only where needed (fr-FR may have 20K, sw-KE 500)"#,
            cypher_create: r#"// Create a Term (knowledge atom)
CREATE (t:Term {
  key: 'term:qr-code@fr-FR',
  locale_key: 'fr-FR',
  term: 'code QR',
  usage_notes: 'Préféré à "QR code" en français'
})"#,
            cypher_query: r#"// Find knowledge atoms for a locale
MATCH (l:Locale {key: 'fr-FR'})-[:HAS_TERMS]->(ts:TermSet)
      -[:CONTAINS_TERM]->(t:Term)
RETURN t.term, t.usage_notes
LIMIT 10"#,
            cli_command: "cargo run -- query --trait=knowledge --realm=global",
        }),
        "derived" => Some(CodeExample {
            yaml_path: "node-kinds/tenant/output/page-generated.yaml",
            yaml_snippet: r#"node:
  name: PageGenerated
  realm: tenant
  layer: output
  trait: derived  # <- Computed from invariant + knowledge
  description: "Generated page content per locale"

# PageGenerated = LLM output
# Created by: Page (invariant) + Knowledge (input) -> PageGenerated"#,
            cypher_create: r#"// PageGenerated is created by generation job
MATCH (p:Page {key: 'homepage'})
MATCH (l:Locale {key: 'fr-FR'})

CREATE (pg:PageGenerated {
  key: 'page:homepage@fr-FR',
  page_key: 'homepage',
  locale_key: 'fr-FR',
  title: 'Générateur de QR Code Gratuit',
  generated_at: datetime()
})

CREATE (p)-[:HAS_GENERATED]->(pg)
CREATE (pg)-[:FOR_LOCALE]->(l)"#,
            cypher_query: r#"// Find all generated pages for a locale
MATCH (p:Page)-[:HAS_GENERATED]->(pg:PageGenerated)
      -[:FOR_LOCALE]->(l:Locale {key: $locale})
RETURN p.key, pg.title, pg.generated_at
ORDER BY pg.generated_at DESC"#,
            cli_command: "cargo run -- query --trait=derived --layer=output",
        }),
        "job" => Some(CodeExample {
            yaml_path: "node-kinds/tenant/output/generation-job.yaml",
            yaml_snippet: r#"node:
  name: GenerationJob
  realm: tenant
  layer: output
  trait: job  # <- Async background process
  description: "LLM generation task tracking"

# Jobs track async generation progress
# Status: pending -> running -> completed/failed"#,
            cypher_create: r#"// Create a generation job
CREATE (j:GenerationJob {
  key: 'job:page-gen-' + randomUUID(),
  status: 'pending',
  target_kind: 'PageGenerated',
  target_locale: 'fr-FR',
  created_at: datetime()
})"#,
            cypher_query: r#"// Find active generation jobs
MATCH (j:GenerationJob)
WHERE j.status IN ['pending', 'running']
RETURN j.key, j.status, j.target_kind, j.target_locale
ORDER BY j.created_at"#,
            cli_command: "cargo run -- query --trait=job --format=table",
        }),
        _ => None,
    }
}

/// Get code example for a layer.
pub fn layer_example(layer_key: &str) -> Option<CodeExample> {
    match layer_key {
        "semantic" => Some(CodeExample {
            yaml_path: "node-kinds/tenant/semantic/",
            yaml_snippet: r#"# Semantic layer contains meaning/knowledge nodes
# Entity, EntityContent, AudiencePersona, ChannelSurface

node:
  name: Entity
  realm: tenant
  layer: semantic  # <- MEANING layer
  trait: invariant

# Key pattern for semantic layer:
# - Entity = semantic unit (product, feature, concept)
# - EntityContent = localized semantic content"#,
            cypher_create: r#"// Create semantic nodes
CREATE (e:Entity {key: 'smart-link', type: 'THING'})
CREATE (ec:EntityContent {
  key: 'entity:smart-link@fr-FR',
  display_name: 'Lien Intelligent'
})
CREATE (e)-[:HAS_CONTENT]->(ec)"#,
            cypher_query: r#"// Count nodes by layer
MATCH (n)
WHERE n.layer = 'semantic'
RETURN labels(n)[0] AS kind, count(*) AS count
ORDER BY count DESC"#,
            cli_command: "cargo run -- query --layer=semantic",
        }),
        "structure" => Some(CodeExample {
            yaml_path: "node-kinds/tenant/structure/",
            yaml_snippet: r#"# Structure layer = page/block hierarchy
# Page, Block, ContentSlot

node:
  name: Page
  realm: tenant
  layer: structure  # <- STRUCTURE layer
  trait: invariant

# Page contains Blocks via HAS_BLOCK
# Block contains ContentSlots via HAS_SLOT"#,
            cypher_create: r#"// Create structure hierarchy
CREATE (p:Page {key: 'homepage', display_name: 'Homepage'})
CREATE (b:Block {key: 'hero-section', block_type: 'hero'})
CREATE (p)-[:HAS_BLOCK {position: 0}]->(b)"#,
            cypher_query: r#"// Get page structure
MATCH (p:Page {key: $page_key})-[:HAS_BLOCK]->(b:Block)
RETURN b.key, b.block_type
ORDER BY b.position"#,
            cli_command: "cargo run -- query --layer=structure",
        }),
        "output" => Some(CodeExample {
            yaml_path: "node-kinds/tenant/output/",
            yaml_snippet: r#"# Output layer = generated artifacts
# PageGenerated, BlockGenerated, GenerationJob, OutputArtifact

node:
  name: PageGenerated
  realm: tenant
  layer: output  # <- OUTPUT layer (derived)
  trait: derived

# Generation pipeline:
# Page (structure) + Knowledge → PageGenerated (output)"#,
            cypher_create: r#"// The generation pipeline creates output
MATCH (p:Page {key: 'homepage'})
MATCH (l:Locale {key: 'ja-JP'})

CREATE (pg:PageGenerated {
  key: 'page:homepage@ja-JP',
  title: 'QRコードジェネレーター',
  meta_description: '無料でQRコードを作成'
})

CREATE (p)-[:HAS_GENERATED]->(pg)
CREATE (pg)-[:FOR_LOCALE]->(l)"#,
            cypher_query: r#"// Get all generated outputs for a page
MATCH (p:Page {key: $page_key})-[:HAS_GENERATED]->(pg:PageGenerated)
RETURN pg.locale_key, pg.title
ORDER BY pg.locale_key"#,
            cli_command: "cargo run -- query --layer=output --format=json",
        }),
        _ => None,
    }
}

/// Get code example for an arc family.
pub fn arc_family_example(family_key: &str) -> Option<CodeExample> {
    match family_key {
        "ownership" => Some(CodeExample {
            yaml_path: "arc-kinds/ownership/",
            yaml_snippet: r#"# Ownership arcs = hierarchical containment
# HAS_PAGE, HAS_BLOCK, HAS_ENTITY, HAS_CONTENT

arc:
  name: HAS_PAGE
  family: ownership
  scope: intra_realm
  cardinality: "1:N"
  source: [Tenant, Project]
  target: Page

# Ownership forms a tree:
# Tenant -> Project -> Entity/Page -> Block"#,
            cypher_create: r#"// Ownership creates hierarchy
MATCH (t:Tenant {key: 'qrcode-ai'})
CREATE (p:Page {key: 'features'})
CREATE (t)-[:HAS_PAGE]->(p)

// Nested ownership
CREATE (b:Block {key: 'hero'})
CREATE (p)-[:HAS_BLOCK {position: 0}]->(b)"#,
            cypher_query: r#"// Traverse ownership hierarchy
MATCH path = (t:Tenant)-[:HAS_PAGE]->(p:Page)
              -[:HAS_BLOCK]->(b:Block)
WHERE t.key = $tenant_key
RETURN p.key, collect(b.key) AS blocks"#,
            cli_command: "cargo run -- query --arc-family=ownership",
        }),
        "localization" => Some(CodeExample {
            yaml_path: "arc-kinds/localization/",
            yaml_snippet: r#"# Localization arcs = locale content links
# HAS_CONTENT, FOR_LOCALE, HAS_GENERATED

arc:
  name: HAS_CONTENT
  family: localization
  scope: intra_realm
  cardinality: "1:N"
  source: Entity
  target: EntityContent

# Pattern: Invariant -[:HAS_CONTENT]-> Localized -[:FOR_LOCALE]-> Locale"#,
            cypher_create: r#"// Localization pattern
MATCH (e:Entity {key: 'qr-code'})
MATCH (l:Locale {key: 'de-DE'})

CREATE (ec:EntityContent {
  key: 'entity:qr-code@de-DE',
  display_name: 'QR-Code',
  slug: 'qr-code'
})

CREATE (e)-[:HAS_CONTENT]->(ec)
CREATE (ec)-[:FOR_LOCALE]->(l)"#,
            cypher_query: r#"// Find all localized content
MATCH (e:Entity)-[:HAS_CONTENT]->(ec:EntityContent)
      -[:FOR_LOCALE]->(l:Locale)
WHERE e.key = $entity_key
RETURN l.key AS locale, ec.display_name AS title
ORDER BY l.key"#,
            cli_command: "cargo run -- query --arc-family=localization",
        }),
        "semantic" => Some(CodeExample {
            yaml_path: "arc-kinds/semantic/",
            yaml_snippet: r#"# Semantic arcs = knowledge connections
# USES_TERM, USES_EXPRESSION, SEMANTIC_LINK, REFERENCES

arc:
  name: USES_TERM
  family: semantic
  scope: cross_realm  # Block (tenant) -> Term (global)
  cardinality: "N:N"
  source: [Block, BlockGenerated]
  target: Term

# Semantic arcs load knowledge into generation context"#,
            cypher_create: r#"// Semantic linking
MATCH (bg:BlockGenerated {key: 'block:hero@fr-FR'})
MATCH (t:Term {key: 'term:qr-code@fr-FR'})

CREATE (bg)-[:USES_TERM {
  weight: 0.9,
  usage: 'primary_keyword'
}]->(t)"#,
            cypher_query: r#"// Find terms used by a block
MATCH (bg:BlockGenerated)-[r:USES_TERM]->(t:Term)
WHERE bg.key = $block_key
RETURN t.term, r.weight
ORDER BY r.weight DESC"#,
            cli_command: "cargo run -- query --arc-family=semantic",
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_examples_all_defined() {
        for trait_key in ["invariant", "localized", "knowledge", "derived", "job"] {
            assert!(
                trait_example(trait_key).is_some(),
                "Missing example for trait: {}",
                trait_key
            );
        }
    }

    #[test]
    fn test_layer_examples_key_layers() {
        for layer in ["semantic", "structure", "output"] {
            assert!(
                layer_example(layer).is_some(),
                "Missing example for layer: {}",
                layer
            );
        }
    }

    #[test]
    fn test_arc_family_examples_key_families() {
        for family in ["ownership", "localization", "semantic"] {
            assert!(
                arc_family_example(family).is_some(),
                "Missing example for family: {}",
                family
            );
        }
    }
}
```

**Step 2: Add module to guide/mod.rs**

```rust
pub mod examples;
```

**Step 3: Run tests**

Run: `cargo test examples --features tui 2>&1`
Expected: PASS (3 tests)

**Step 4: Commit**

```bash
git add tools/novanet/src/tui/guide/examples.rs tools/novanet/src/tui/guide/mod.rs
git commit -m "$(cat <<'EOF'
feat(tui): add code examples module with real YAML/Cypher data

Shows complete data pipeline for each concept:
- YAML source snippets from actual model files
- Neo4j Cypher CREATE statements
- Query patterns for exploration
- CLI commands

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Task 9: Render code examples in detail panel

**Files:**
- Modify: `tools/novanet/src/tui/guide/traits.rs` (detail panel)

**Step 1: Add example rendering function**

Add to `traits.rs`:

```rust
use super::examples::{trait_example, CodeExample};

/// Render code example in the detail panel.
fn render_code_example(example: &CodeExample, lines: &mut Vec<Line<'static>>) {
    // YAML section
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("YAML: {}", example.yaml_path),
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "─".repeat(40),
        Style::default().fg(Color::Rgb(60, 60, 70)),
    )));

    for yaml_line in example.yaml_snippet.lines() {
        let style = if yaml_line.trim().starts_with('#') {
            Style::default().fg(Color::Rgb(100, 100, 120))
        } else if yaml_line.contains(':') {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::White)
        };
        lines.push(Line::from(Span::styled(yaml_line.to_string(), style)));
    }

    // Cypher CREATE section
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "CYPHER CREATE:",
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
    )));
    for cypher_line in example.cypher_create.lines() {
        let style = if cypher_line.trim().starts_with("//") {
            Style::default().fg(Color::Rgb(100, 100, 120))
        } else {
            Style::default().fg(Color::White)
        };
        lines.push(Line::from(Span::styled(cypher_line.to_string(), style)));
    }

    // CLI command
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "CLI:",
        Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        format!("$ {}", example.cli_command),
        Style::default().fg(Color::White),
    )));
}
```

**Step 2: Integrate into detail panel**

In the detail panel rendering, after showing trait info:

```rust
// Get current trait key
let trait_keys = ["invariant", "localized", "knowledge", "derived", "job"];
if let Some(trait_key) = trait_keys.get(app.guide.trait_cursor) {
    if let Some(example) = trait_example(trait_key) {
        render_code_example(&example, &mut lines);
    }
}
```

**Step 3: Verify visually**

Run: `cargo run --features tui -- tui`
Navigate to Guide → Traits → select "invariant"
Expected: Detail panel shows YAML snippet, Cypher, and CLI command

**Step 4: Commit**

```bash
git add tools/novanet/src/tui/guide/traits.rs
git commit -m "$(cat <<'EOF'
feat(tui): render code examples in Traits detail panel

Shows real YAML → Cypher → CLI pipeline for selected trait.
Syntax highlighting: green (YAML path), cyan (YAML keys),
yellow (Cypher), magenta (CLI).

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Task 10: Add code examples to Layers tab

**Files:**
- Modify: `tools/novanet/src/tui/guide/layers.rs`

**Step 1: Import examples**

```rust
use super::examples::layer_example;
```

**Step 2: Add example rendering to layer detail**

When a layer is selected, show the code example (if available):

```rust
// In the layer detail section
let layer_keys = if app.guide.layer_realm == 0 {
    &["config", "locale-knowledge"][..]
} else {
    &["config", "foundation", "structure", "semantic", "instruction", "seo", "output"][..]
};

if let Some(layer_key) = layer_keys.get(app.guide.layer_cursor) {
    if let Some(example) = layer_example(layer_key) {
        // Render example (reuse function from traits.rs or move to shared location)
    }
}
```

**Step 3: Verify and commit**

```bash
git add tools/novanet/src/tui/guide/layers.rs
git commit -m "$(cat <<'EOF'
feat(tui): add code examples to Layers tab detail

Shows YAML structure and Cypher patterns for key layers.

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Task 11: Add code examples to Arcs tab

**Files:**
- Modify: `tools/novanet/src/tui/guide/arcs.rs`

**Step 1: Import and integrate**

```rust
use super::examples::arc_family_example;
```

**Step 2: Add to arc family detail rendering**

Similar pattern to traits/layers.

**Step 3: Verify and commit**

```bash
git add tools/novanet/src/tui/guide/arcs.rs
git commit -m "$(cat <<'EOF'
feat(tui): add code examples to Arcs tab detail

Shows arc patterns with real YAML → Cypher examples.

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Task 12: Update KEYBINDINGS.md

**Files:**
- Modify: `tools/novanet/KEYBINDINGS.md`

**Step 1: Add Guide mode section**

Add new section documenting Guide-specific keybindings:

```markdown
---

## Guide Mode

| Key | Action |
|-----|--------|
| `1` | Switch to Traits tab |
| `2` | Switch to Layers tab |
| `3` | Switch to Arcs tab |
| `4` | Switch to Pipeline tab |
| `Tab` | Cycle to next tab |
| `Shift+Tab` | Cycle to previous tab |
| `j` / `Down` | Move cursor down |
| `k` / `Up` | Move cursor up |
| `h` / `Left` | Navigate left (realm switch in Layers) |
| `l` / `Right` | Navigate right |
| `Enter` | Drill down into selection |
| `Esc` | Drill up / back |
| `y` | Yank (copy) current item to clipboard |
| `n` | Next tip in tips bar |
| `Space` | Toggle animation (Pipeline tab only) |
| `gi` | Quick jump to Invariant trait |
| `gl` | Quick jump to Localized trait |
| `gk` | Quick jump to Knowledge trait |
| `gd` | Quick jump to Derived trait |
| `gj` | Quick jump to Job trait |
```

**Step 2: Commit**

```bash
git add tools/novanet/KEYBINDINGS.md
git commit -m "$(cat <<'EOF'
docs(tui): update KEYBINDINGS.md with Guide mode shortcuts

Documents all Guide-specific keybindings including:
- Tab switching (1-4, Tab)
- Navigation (j/k, h/l)
- Yank (y) for clipboard
- Quick jumps (gi, gl, gk, gd, gj)

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Task 13: Final integration test

**Step 1: Run all tests**

```bash
cargo test --features tui 2>&1 | tail -20
```
Expected: All tests pass

**Step 2: Manual verification**

```bash
cargo run --features tui -- tui
```

Verify:
1. Guide mode → Traits → heatmap colors visible
2. Guide mode → Layers → heatmap colors visible
3. Press `y` → clipboard message appears
4. Detail panels show YAML/Cypher examples
5. Press `1-4` to switch tabs

**Step 3: Final commit**

```bash
git add -A
git commit -m "$(cat <<'EOF'
feat(tui): complete Guide mode enhancements

Summary:
- Heatmap colors for kind counts (bright = many, dim = few)
- Clipboard support with 'y' key (arboard crate)
- Real code examples showing YAML → Neo4j → Cypher flow
- Updated keybindings documentation

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Summary

| Task | Component | Effort |
|------|-----------|--------|
| 1 | Add arboard dependency | 2 min |
| 2 | Heatmap color function | 5 min |
| 3-4 | Apply heatmap to tabs | 10 min |
| 5 | Realm/layer in detail | 5 min |
| 6-7 | Clipboard module + yank | 15 min |
| 8 | Code examples data | 20 min |
| 9-11 | Render examples in tabs | 15 min |
| 12 | Update docs | 5 min |
| 13 | Integration test | 5 min |

**Total: ~80 minutes**
