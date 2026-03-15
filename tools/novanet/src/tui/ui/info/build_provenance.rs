//! Provenance section builder (ADR-035/ADR-042).
//!
//! Extracts `DataCategory`, `ProvenanceMeta`, and `build_provenance_section()`
//! from `mod.rs` for focused provenance rendering.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use serde_json::Value as JsonValue;

use crate::tui::palette;

use super::{COLOR_HEADER_PROVENANCE, SectionContent};
use super::super::STYLE_DIM;

// =============================================================================
// PROVENANCE HELPERS (ADR-035)
// =============================================================================

/// Data category derived from provenance.source field.
/// 6 sources mapped to 6 categories with distinct lifecycle properties.
///
///   Source           | Reseed | Backup | Editable | Color (TUI)
///   -----------------+--------+--------+----------+------------
///   seed:schema      |  Y     |  No    |  No      | Slate-500
///   seed:immutable   |  Y     |  No    |  No      | Green-500
///   seed:locale      |  Y     |  No    |  No      | Cyan-500
///   seed:content     |  Y     |  Yes   |  Yes     | Blue-500
///   runtime:nika     |  No    |  Yes   |  Yes     | Orange-500
///   runtime:mcp      |  No    |  Yes   |  Yes     | Purple-500
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum DataCategory {
    Schema,    // seed:schema - regenerable from YAML models
    Immutable, // seed:immutable - static reference data (geography, culture)
    Locale,    // seed:locale - locale knowledge atoms (expressions, patterns)
    Content,   // seed:content - bootstrap content examples (entities, pages)
    Nika,      // runtime:nika - Nika workflow output
    Mcp,       // runtime:mcp - MCP API mutation
}

impl DataCategory {
    /// Parse category from provenance source value.
    /// 6 sources with distinct lifecycle properties.
    pub(crate) fn from_source(source: &str) -> Self {
        match source {
            "seed:schema" => DataCategory::Schema,
            "seed:immutable" => DataCategory::Immutable,
            "seed:locale" => DataCategory::Locale,
            "seed:content" => DataCategory::Content,
            "runtime:nika" => DataCategory::Nika,
            "runtime:mcp" => DataCategory::Mcp,
            // Prefix-match fallbacks for partial provenance strings
            s if s.starts_with("seed:schema") => DataCategory::Schema,
            s if s.starts_with("seed:immutable") => DataCategory::Immutable,
            s if s.starts_with("seed:locale") => DataCategory::Locale,
            s if s.starts_with("seed:content") || s.starts_with("content:") => {
                DataCategory::Content
            }
            s if s.starts_with("runtime:nika") || s.starts_with("nika:") => DataCategory::Nika,
            s if s.starts_with("runtime:mcp") || s.starts_with("mcp:") => DataCategory::Mcp,
            _ => DataCategory::Mcp, // Unknown runtime sources default to MCP
        }
    }

    /// Human-readable category name (concise).
    pub(crate) fn label(&self) -> &'static str {
        match self {
            DataCategory::Schema => "Schema",
            DataCategory::Immutable => "Immutable",
            DataCategory::Locale => "Locale",
            DataCategory::Content => "Content",
            DataCategory::Nika => "Nika",
            DataCategory::Mcp => "MCP",
        }
    }

    /// Short description for the category (shown after label).
    pub(crate) fn description(&self) -> &'static str {
        match self {
            DataCategory::Schema => "regenerable from YAML",
            DataCategory::Immutable => "static reference data",
            DataCategory::Locale => "knowledge atoms",
            DataCategory::Content => "bootstrap examples",
            DataCategory::Nika => "workflow output",
            DataCategory::Mcp => "API mutation",
        }
    }

    /// Terminal icon for the category (Unicode).
    /// Consistent with visual-encoding.yaml dual-icon pattern.
    pub(crate) fn icon(&self) -> &'static str {
        match self {
            DataCategory::Schema => "◆",    // filled diamond -- structured definitions
            DataCategory::Immutable => "◇",  // outline diamond -- fixed reference
            DataCategory::Locale => "◈",     // diamond with dot -- locale-specific
            DataCategory::Content => "●",    // filled circle -- content data
            DataCategory::Nika => "▶",       // play -- workflow execution
            DataCategory::Mcp => "⚡",       // lightning -- API mutation
        }
    }

    /// Color for the category badge.
    pub(crate) fn color(&self) -> Color {
        match self {
            DataCategory::Schema => palette::SLATE_500,
            DataCategory::Immutable => palette::GREEN_500,
            DataCategory::Locale => palette::CYAN_500,
            DataCategory::Content => palette::BLUE_500,
            DataCategory::Nika => palette::ORANGE_500,
            DataCategory::Mcp => palette::PURPLE_500,
        }
    }

    /// Whether this data survives reseed (seed sources are regenerable).
    pub(crate) fn reseed_safe(&self) -> bool {
        matches!(
            self,
            DataCategory::Schema
                | DataCategory::Immutable
                | DataCategory::Locale
                | DataCategory::Content
        )
    }

    /// Whether this data needs backup (content + runtime data is unique).
    pub(crate) fn needs_backup(&self) -> bool {
        matches!(
            self,
            DataCategory::Content | DataCategory::Nika | DataCategory::Mcp
        )
    }

    /// Whether this data is editable by users.
    pub(crate) fn is_editable(&self) -> bool {
        matches!(
            self,
            DataCategory::Content | DataCategory::Nika | DataCategory::Mcp
        )
    }
}

/// Parse provenance JSON for display in the TUI info panel.
/// Unified provenance object with tagged union on `source`.
///
/// Structure depends on source:
///   seed:*       -> { source, version, file? }
///   runtime:nika -> { source, version, workflow_id?, task_id?, provider?, model?, generated_at? }
///   runtime:mcp  -> { source, version, tool?, user? }
#[derive(Default)]
pub(crate) struct ProvenanceMeta {
    pub(crate) source: Option<String>,
    pub(crate) version: Option<String>,
    // Seed fields
    pub(crate) file: Option<String>,
    // Nika fields
    pub(crate) workflow_id: Option<String>,
    pub(crate) task_id: Option<String>,
    pub(crate) provider: Option<String>,
    pub(crate) model: Option<String>,
    pub(crate) generated_at: Option<String>,
    // MCP fields
    pub(crate) tool: Option<String>,
    pub(crate) user: Option<String>,
}

impl ProvenanceMeta {
    /// Parse from provenance JSON value.
    pub(crate) fn from_json(value: &JsonValue) -> Self {
        let obj = value.as_object();
        let get_str = |key: &str| -> Option<String> {
            obj.and_then(|o| o.get(key))
                .and_then(|v| v.as_str())
                .map(String::from)
        };
        Self {
            source: get_str("source"),
            version: get_str("version"),
            file: get_str("file"),
            workflow_id: get_str("workflow_id"),
            task_id: get_str("task_id"),
            provider: get_str("provider"),
            model: get_str("model"),
            generated_at: get_str("generated_at"),
            tool: get_str("tool"),
            user: get_str("user"),
        }
    }

    /// Check if this contains Nika generation details.
    fn is_nika_generated(&self) -> bool {
        self.workflow_id.is_some() || self.task_id.is_some()
    }

    /// Check if this is an MCP mutation with extra details.
    fn is_mcp_mutation(&self) -> bool {
        self.tool.is_some() || self.user.is_some()
    }
}

/// Build provenance section content from unified provenance property.
/// Consolidated from `created_by` + `created_by_meta` into single `provenance` JSON object.
///
/// Display layout:
/// ```text
///   Source        ◆ seed:schema          (icon + source in category color)
///   Category      Schema -- regenerable from YAML
///   Version       v0.19.0
///   Lifecycle     YReseed  oBkup  lockEdit
///   File          48-page-block-qr-code.cypher
/// ```
pub(crate) fn build_provenance_section(provenance: Option<&JsonValue>) -> SectionContent<'static> {
    let mut section = SectionContent::default();

    // Parse provenance -- handle both JSON object and JSON-encoded string
    let meta = match provenance {
        Some(prov) => match prov {
            // Direct JSON object (normal case)
            JsonValue::Object(_) => ProvenanceMeta::from_json(prov),
            // JSON-encoded string (provenance stored as string in some seed files)
            JsonValue::String(s) => {
                if let Ok(parsed) = serde_json::from_str::<JsonValue>(s) {
                    ProvenanceMeta::from_json(&parsed)
                } else {
                    // Plain string, not JSON -- treat as raw source
                    ProvenanceMeta {
                        source: Some(s.clone()),
                        ..Default::default()
                    }
                }
            }
            // Null or other types
            _ => {
                section.add_line(Line::from(vec![
                    Span::styled("  Source       ", STYLE_DIM),
                    Span::styled(
                        "⚠ missing",
                        Style::default().fg(palette::YELLOW_500),
                    ),
                ]));
                return section;
            }
        },
        None => {
            section.add_line(Line::from(vec![
                Span::styled("  Source       ", STYLE_DIM),
                Span::styled(
                    "⚠ missing",
                    Style::default().fg(palette::YELLOW_500),
                ),
            ]));
            return section;
        }
    };

    // Get source from provenance.source field
    let source = match &meta.source {
        Some(s) if !s.is_empty() => s.as_str(),
        _ => {
            section.add_line(Line::from(vec![
                Span::styled("  Source       ", STYLE_DIM),
                Span::styled(
                    "⚠ missing source",
                    Style::default().fg(palette::YELLOW_500),
                ),
            ]));
            return section;
        }
    };

    let category = DataCategory::from_source(source);

    // Source line: icon + source in category color
    section.add_line(Line::from(vec![
        Span::styled("  Source       ", STYLE_DIM),
        Span::styled(
            format!("{} ", category.icon()),
            Style::default().fg(category.color()),
        ),
        Span::styled(source.to_string(), Style::default().fg(category.color())),
    ]));

    // Category line: label + description
    section.add_line(Line::from(vec![
        Span::styled("  Category     ", STYLE_DIM),
        Span::styled(
            category.label(),
            Style::default()
                .fg(category.color())
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" — {}", category.description()),
            Style::default().fg(Color::DarkGray),
        ),
    ]));

    // Version line (if present)
    if let Some(ref version) = meta.version {
        section.add_line(Line::from(vec![
            Span::styled("  Version      ", STYLE_DIM),
            Span::styled(version.clone(), Style::default().fg(Color::DarkGray)),
        ]));
    }

    // Lifecycle badges line
    let reseed_badge = if category.reseed_safe() {
        Span::styled("✓Reseed", Style::default().fg(palette::GREEN_500))
    } else {
        Span::styled("⚠Reseed", Style::default().fg(palette::RED_500))
    };

    let backup_badge = if category.needs_backup() {
        Span::styled("●Backup", Style::default().fg(palette::ORANGE_500))
    } else {
        Span::styled("○Backup", Style::default().fg(Color::DarkGray))
    };

    let edit_badge = if category.is_editable() {
        Span::styled("✎Edit", Style::default().fg(palette::BLUE_500))
    } else {
        Span::styled("🔒Edit", Style::default().fg(Color::DarkGray))
    };

    section.add_line(Line::from(vec![
        Span::styled("  Lifecycle    ", STYLE_DIM),
        reseed_badge,
        Span::styled("  ", Style::default()),
        backup_badge,
        Span::styled("  ", Style::default()),
        edit_badge,
    ]));

    // Seed source file (if present)
    if let Some(ref file) = meta.file {
        section.add_line(Line::from(vec![
            Span::styled("  File         ", STYLE_DIM),
            Span::styled(file.clone(), Style::default().fg(Color::DarkGray)),
        ]));
    }

    // If Nika-generated, show generation details
    if meta.is_nika_generated() {
        section.add_line(Line::from(Span::styled(
            "  ─── Generation Details ───",
            Style::default()
                .fg(COLOR_HEADER_PROVENANCE)
                .add_modifier(Modifier::DIM),
        )));

        if let Some(ref wf) = meta.workflow_id {
            let task_str = meta.task_id.as_deref().unwrap_or("");
            section.add_line(Line::from(vec![
                Span::styled("  Workflow     ", STYLE_DIM),
                Span::styled(wf.clone(), Style::default().fg(Color::Cyan)),
                if !task_str.is_empty() {
                    Span::styled(
                        format!(" ({})", task_str),
                        Style::default().fg(Color::DarkGray),
                    )
                } else {
                    Span::styled("", Style::default())
                },
            ]));
        }

        if let (Some(provider), Some(model)) = (&meta.provider, &meta.model) {
            section.add_line(Line::from(vec![
                Span::styled("  Provider     ", STYLE_DIM),
                Span::styled(
                    format!("{}/{}", provider, model),
                    Style::default().fg(Color::Yellow),
                ),
            ]));
        }

        if let Some(ref ts) = meta.generated_at {
            section.add_line(Line::from(vec![
                Span::styled("  Generated    ", STYLE_DIM),
                Span::styled(ts.clone(), Style::default().fg(Color::DarkGray)),
            ]));
        }
    }

    // If MCP mutation, show mutation details
    if meta.is_mcp_mutation() {
        section.add_line(Line::from(Span::styled(
            "  ─── MCP Details ───",
            Style::default()
                .fg(COLOR_HEADER_PROVENANCE)
                .add_modifier(Modifier::DIM),
        )));

        if let Some(ref tool) = meta.tool {
            section.add_line(Line::from(vec![
                Span::styled("  Tool         ", STYLE_DIM),
                Span::styled(tool.clone(), Style::default().fg(Color::Cyan)),
            ]));
        }

        if let Some(ref user) = meta.user {
            section.add_line(Line::from(vec![
                Span::styled("  User         ", STYLE_DIM),
                Span::styled(user.clone(), Style::default().fg(Color::Yellow)),
            ]));
        }
    }

    section
}
