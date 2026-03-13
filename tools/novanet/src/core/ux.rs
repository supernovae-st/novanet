//! Shared UX helpers for polished CLI output.
//!
//! Provides consistent visual patterns across all CLI commands:
//! - **Banners**: Header boxes with colored title, description, and metadata
//! - **Steps**: Narrated step-by-step progress with status indicators
//! - **Summaries**: Result boxes with key metrics and status-colored borders
//! - **Next steps**: Actionable hints with distinct call-to-action styling
//! - **Flow diagrams**: Compact horizontal diagram of the 2-command pipeline
//! - **Class icons**: Unicode glyphs per node type (◆ Entity, ▣ Page, etc.)

use colored::Colorize;

// =============================================================================
// BANNER
// =============================================================================

/// Print a command header banner with colored title, description, and metadata.
///
/// ```text
/// ┌─────────────────────────────────────────────────────────────────────┐
/// │  NOVANET DATA BACKUP                                                │
/// │  Save database content to version-controlled files                  │
/// │                                                                     │
/// │  From     Database (Neo4j in Docker)                                │
/// │  To       YAML files (private-data/data/)                           │
/// └─────────────────────────────────────────────────────────────────────┘
/// ```
pub fn print_banner(title: &str, description: &str, metadata: &[(&str, String)]) {
    // Build content lines with colored parts
    let title_colored = title.cyan().bold().to_string();
    let desc_colored = description.dimmed().to_string();

    // Find the longest label for alignment
    let label_width = metadata
        .iter()
        .map(|(l, _)| l.len())
        .max()
        .unwrap_or(0);

    let metadata_lines: Vec<String> = metadata
        .iter()
        .map(|(label, value)| {
            format!(
                "{}  {}",
                format!("{label:<label_width$}").dimmed(),
                value
            )
        })
        .collect();

    // Calculate width from visible content
    let mut all_visible: Vec<usize> = vec![
        visible_len(title),
        visible_len(description),
    ];
    for (label, value) in metadata {
        all_visible.push(label.len() + 2 + visible_len(value));
    }

    let max_content = all_visible.into_iter().max().unwrap_or(40);
    let width = (max_content + 6).max(60);

    eprintln!();
    eprintln!("  {}", border_top(width).dimmed());
    eprintln!("  {}", border_line_raw(&title_colored, visible_len(title), width));
    eprintln!("  {}", border_line_raw(&desc_colored, visible_len(description), width));

    if !metadata.is_empty() {
        eprintln!("  {}", border_empty(width));
        for line in &metadata_lines {
            eprintln!("  {}", border_line_raw(line, visible_len(line), width));
        }
    }

    eprintln!("  {}", border_bottom(width).dimmed());
    eprintln!();
}

// =============================================================================
// STEP INDICATORS
// =============================================================================

/// Print a completed step with a green check mark.
pub fn step_ok(label: &str, detail: &str) {
    eprintln!(
        "  {} {}  {}",
        "✓".green().bold(),
        label.bold(),
        detail.dimmed()
    );
}

/// Print a completed step with a count (right-aligned).
pub fn step_ok_count(label: &str, count: usize, suffix: &str) {
    let count_str = fmt_count(count);
    eprintln!(
        "  {} {:<20}  {} {}",
        "✓".green().bold(),
        label.bold(),
        count_str.cyan().bold(),
        suffix.dimmed()
    );
}

/// Print a skipped step.
pub fn step_skip(label: &str, reason: &str) {
    eprintln!(
        "  {} {}  {}",
        "·".dimmed(),
        label.dimmed(),
        reason.dimmed()
    );
}

/// Print a warning step.
pub fn step_warn(label: &str, detail: &str) {
    eprintln!(
        "  {} {}  {}",
        "!".yellow().bold(),
        label.yellow().bold(),
        detail.yellow()
    );
}

// =============================================================================
// SUMMARY BOX
// =============================================================================

/// Print a success summary box (green border).
pub fn print_summary_ok(lines: &[String]) {
    print_summary_colored(lines, SummaryStyle::Ok);
}

/// Print a warning summary box (yellow border).
pub fn print_summary_warn(lines: &[String]) {
    print_summary_colored(lines, SummaryStyle::Warn);
}

/// Print a summary box with default style (for backward compat).
pub fn print_summary_box(lines: &[String]) {
    print_summary_colored(lines, SummaryStyle::Default);
}

enum SummaryStyle {
    Ok,
    Warn,
    Default,
}

fn print_summary_colored(lines: &[String], style: SummaryStyle) {
    let max_content = lines
        .iter()
        .map(|l| visible_len(l))
        .max()
        .unwrap_or(40);
    let width = (max_content + 6).max(60);

    let (top, bottom, pipe) = match style {
        SummaryStyle::Ok => (
            format!("  {}{}{}", "┌".green(), "─".repeat(width).green(), "┐".green()),
            format!("  {}{}{}", "└".green(), "─".repeat(width).green(), "┘".green()),
            "│".green().to_string(),
        ),
        SummaryStyle::Warn => (
            format!("  {}{}{}", "┌".yellow(), "─".repeat(width).yellow(), "┐".yellow()),
            format!("  {}{}{}", "└".yellow(), "─".repeat(width).yellow(), "┘".yellow()),
            "│".yellow().to_string(),
        ),
        SummaryStyle::Default => (
            format!("  {}", border_top(width).dimmed()),
            format!("  {}", border_bottom(width).dimmed()),
            "│".dimmed().to_string(),
        ),
    };

    eprintln!();
    eprintln!("{top}");
    for line in lines {
        let content_len = visible_len(line);
        let padding = if width > content_len + 4 {
            width - content_len - 4
        } else {
            1
        };
        eprintln!(
            "  {}  {}{}  {}",
            pipe,
            line,
            " ".repeat(padding),
            pipe
        );
    }
    eprintln!("{bottom}");
}

// =============================================================================
// NEXT STEPS
// =============================================================================

/// Print a "next step" hint with a prominent arrow and colored command.
pub fn print_next_step(hint: &str, command: &str) {
    eprintln!();
    eprintln!(
        "  {} {} {}",
        "→".cyan().bold(),
        hint.dimmed(),
        command.cyan().bold()
    );
    eprintln!();
}

// =============================================================================
// FLOW DIAGRAM
// =============================================================================

/// Print a compact horizontal flow diagram for the 2-command data pipeline.
///
/// ```text
///   DATA MANAGEMENT
///
///   ┌──────────┐  ──backup──>  ┌──────────────┐
///   │ Database │  <──status──  │  YAML Backup  │
///   └──────────┘               └──────────────┘
///
///   ▸ backup   Save database content to YAML files
///     status   Check what changed since last backup
/// ```
pub fn print_data_flow(active_step: Option<u8>) {
    eprintln!();
    eprintln!("  {}", "DATA MANAGEMENT".bold());
    eprintln!(
        "  {}",
        "Two commands keep your YAML backup in sync with Neo4j.".dimmed()
    );
    eprintln!();

    // Compact horizontal diagram
    let db_box_t = format!("{}{}{}",   "┌".cyan().dimmed(), "────────────".cyan().dimmed(), "┐".cyan().dimmed());
    let db_box_m = format!("{} {} {}",  "│".cyan().dimmed(), " Database ".bold(), "│".cyan().dimmed());
    let db_box_b = format!("{}{}{}",   "└".cyan().dimmed(), "────────────".cyan().dimmed(), "┘".cyan().dimmed());

    let yaml_box_t = format!("{}{}{}",  "┌".green().dimmed(), "──────────────".green().dimmed(), "┐".green().dimmed());
    let yaml_box_m = format!("{} {} {}","│".green().dimmed(), " YAML Backup ".bold(), "│".green().dimmed());
    let yaml_box_b = format!("{}{}{}",  "└".green().dimmed(), "──────────────".green().dimmed(), "┘".green().dimmed());

    let arrow_fwd = format!(" {} ", "──backup──▸".cyan());
    let arrow_rev = format!(" {} ", "◂──status──".yellow());

    eprintln!("   {}              {}", db_box_t, yaml_box_t);
    eprintln!("   {}{}{}",           db_box_m, arrow_fwd, yaml_box_m);
    eprintln!("   {}{}{}",           db_box_b, arrow_rev, yaml_box_b);
    eprintln!();

    // Legend
    let labels = [
        (1, "backup", "Save database content to YAML files"),
        (2, "status", "Check what changed since last backup"),
    ];

    for (step, label, desc) in &labels {
        if active_step == Some(*step) {
            eprintln!(
                "   {} {:<9} {}",
                "▸".cyan().bold(),
                label.cyan().bold(),
                desc
            );
        } else {
            eprintln!(
                "     {:<9} {}",
                label.dimmed(),
                desc.dimmed()
            );
        }
    }
    eprintln!();
}

/// Print a simpler flow arrow for command banners.
pub fn print_flow_arrow(from_label: &str, from_detail: &str, to_label: &str, to_detail: &str) {
    eprintln!(
        "  {} {}  {}  {} {}",
        from_label.bold(),
        format!("({from_detail})").dimmed(),
        "───▸".cyan().bold(),
        to_label.bold(),
        format!("({to_detail})").dimmed()
    );
    eprintln!();
}

// =============================================================================
// CLASS ICONS
// =============================================================================

/// Return a Unicode icon for a node class name.
#[must_use]
pub fn class_icon(class: &str) -> &'static str {
    match class {
        "Entity"       => "◆",
        "EntityNative" => "◇",
        "Page"         => "▣",
        "PageNative"   => "▢",
        "Block"        => "▪",
        "BlockNative"  => "▫",
        "Project"      => "★",
        "Brand"        => "◎",
        _              => "●",
    }
}

/// Return a class label with its icon (plain, no ANSI).
#[must_use]
pub fn class_label(class: &str) -> String {
    let icon = class_icon(class);
    format!("{icon} {class}")
}

// =============================================================================
// STATUS INDICATORS
// =============================================================================

/// Print a status line for a class with colored indicator.
///
/// - Clean: green checkmark + dimmed detail
/// - Drift: yellow triangle + detail passed through as-is (caller controls color)
pub fn step_class_status(class: &str, detail: &str, is_clean: bool) {
    let icon = class_icon(class);
    if is_clean {
        eprintln!(
            "  {} {} {:<16} {}",
            "✓".green().bold(),
            icon.green(),
            class.green(),
            detail.dimmed()
        );
    } else {
        // detail is pre-colored by fmt_diff_counts(), don't re-wrap
        eprintln!(
            "  {} {} {:<16} {}",
            "△".yellow().bold(),
            icon.yellow(),
            class.yellow().bold(),
            detail
        );
    }
}

/// Print a colored summary bar showing sync vs drift percentage.
///
/// ```text
///   ████████████████░░░░  80% in sync  (160/200 nodes)
/// ```
pub fn print_sync_bar(in_sync: usize, total: usize) {
    if total == 0 {
        return;
    }

    let pct = (in_sync as f64 / total as f64 * 100.0) as usize;
    let bar_width = 24;
    let filled = (in_sync * bar_width) / total.max(1);
    let empty = bar_width - filled;

    let bar = format!(
        "{}{}",
        "█".repeat(filled),
        "░".repeat(empty),
    );

    let bar_colored = if pct == 100 {
        bar.green().bold()
    } else if pct >= 90 {
        bar.green()
    } else if pct >= 50 {
        bar.yellow()
    } else {
        bar.red()
    };

    let pct_str = if pct == 100 {
        format!("{}% in sync", pct).green().bold().to_string()
    } else {
        format!("{}% in sync", pct).to_string()
    };

    eprintln!(
        "  {}  {}  ({}/{})",
        bar_colored,
        pct_str,
        fmt_count(in_sync),
        fmt_count(total),
    );
}

/// Format colored diff counts: +N / ~N / -N in green/yellow/red.
#[must_use]
pub fn fmt_diff_counts(added: usize, modified: usize, removed: usize) -> String {
    let mut parts = Vec::new();
    if added > 0 {
        parts.push(format!("+{added}").green().bold().to_string());
    }
    if modified > 0 {
        parts.push(format!("~{modified}").yellow().bold().to_string());
    }
    if removed > 0 {
        parts.push(format!("-{removed}").red().bold().to_string());
    }
    if parts.is_empty() {
        "in sync".green().to_string()
    } else {
        parts.join(" ")
    }
}

/// Print a section header with a colored underline.
pub fn print_section(title: &str) {
    eprintln!();
    eprintln!("  {} {}", "│".cyan().dimmed(), title.bold());
}

// =============================================================================
// DRY RUN NOTICE
// =============================================================================

/// Print a dry-run notice under the banner.
pub fn print_dry_run_notice() {
    eprintln!(
        "  {} {}",
        "▸".yellow().bold(),
        "DRY RUN — no files will be written".yellow().bold()
    );
    eprintln!();
}

// =============================================================================
// FORMAT HELPERS
// =============================================================================

/// Format a number with thousands separators (1816 -> "1,816").
#[must_use]
pub fn fmt_count(n: usize) -> String {
    if n < 1_000 {
        return n.to_string();
    }
    let s = n.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();
    let commas = (len - 1) / 3;
    let mut result = String::with_capacity(len + commas);
    for (i, &b) in bytes.iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push(',');
        }
        result.push(b as char);
    }
    result
}

/// Format a path, replacing home dir with ~ for readability.
#[must_use]
pub fn fmt_path(path: &std::path::Path) -> String {
    let display = path.display().to_string();
    if let Some(home) = dirs::home_dir() {
        let home_str = home.display().to_string();
        if display.starts_with(&home_str) {
            return display.replacen(&home_str, "~", 1);
        }
    }
    display
}

// =============================================================================
// BOX DRAWING HELPERS
// =============================================================================

fn border_top(width: usize) -> String {
    format!("\u{250c}{}\u{2510}", "\u{2500}".repeat(width))
}

fn border_bottom(width: usize) -> String {
    format!("\u{2514}{}\u{2518}", "\u{2500}".repeat(width))
}

/// Build a border line with explicit visible length (for pre-colored content).
fn border_line_raw(content: &str, content_visible_len: usize, width: usize) -> String {
    let padding = if width > content_visible_len + 4 {
        width - content_visible_len - 4
    } else {
        1
    };
    format!(
        "{}  {}{}  {}",
        "\u{2502}".dimmed(),
        content,
        " ".repeat(padding),
        "\u{2502}".dimmed()
    )
}

fn border_empty(width: usize) -> String {
    format!(
        "{}{}  {}",
        "\u{2502}".dimmed(),
        " ".repeat(width - 2),
        "\u{2502}".dimmed()
    )
}

/// Get the visible length of a string (ignoring ANSI escape sequences).
fn visible_len(s: &str) -> usize {
    let mut len = 0;
    let mut in_escape = false;
    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else {
            len += 1;
        }
    }
    len
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_count_small() {
        assert_eq!(fmt_count(0), "0");
        assert_eq!(fmt_count(42), "42");
        assert_eq!(fmt_count(999), "999");
    }

    #[test]
    fn fmt_count_thousands() {
        assert_eq!(fmt_count(1_000), "1,000");
        assert_eq!(fmt_count(1_816), "1,816");
        assert_eq!(fmt_count(19_817), "19,817");
        assert_eq!(fmt_count(1_000_000), "1,000,000");
    }

    #[test]
    fn fmt_path_replaces_home() {
        if let Some(home) = dirs::home_dir() {
            let path = home.join(".novanet").join("export");
            let formatted = fmt_path(&path);
            assert!(formatted.starts_with("~/.novanet"));
            assert!(!formatted.contains(&home.display().to_string()));
        }
    }

    #[test]
    fn fmt_path_absolute_unchanged() {
        let path = std::path::Path::new("/tmp/novanet-export");
        assert_eq!(fmt_path(path), "/tmp/novanet-export");
    }

    #[test]
    fn visible_len_plain() {
        assert_eq!(visible_len("hello"), 5);
        assert_eq!(visible_len(""), 0);
    }

    #[test]
    fn visible_len_with_ansi() {
        assert_eq!(visible_len("\x1b[32mhello\x1b[0m"), 5);
    }

    #[test]
    fn border_top_creates_box() {
        let top = border_top(20);
        assert!(top.starts_with('\u{250c}'));
        assert!(top.ends_with('\u{2510}'));
    }

    // Smoke tests: verify they don't panic
    #[test]
    fn print_banner_smoke() {
        print_banner(
            "TEST BANNER",
            "A test description",
            &[("Key", "Value".to_string())],
        );
    }

    #[test]
    fn print_summary_box_smoke() {
        print_summary_box(&["Line 1".to_string(), "Line 2".to_string()]);
    }

    #[test]
    fn print_summary_ok_smoke() {
        print_summary_ok(&["All good".to_string()]);
    }

    #[test]
    fn print_summary_warn_smoke() {
        print_summary_warn(&["Drift detected".to_string()]);
    }

    #[test]
    fn step_ok_smoke() {
        step_ok("Entity", "42 nodes");
    }

    #[test]
    fn step_skip_smoke() {
        step_skip("Entity", "0 nodes");
    }

    #[test]
    fn print_data_flow_smoke() {
        print_data_flow(None);
        print_data_flow(Some(1));
        print_data_flow(Some(2));
    }

    #[test]
    fn class_icon_known() {
        assert_eq!(class_icon("Entity"), "◆");
        assert_eq!(class_icon("EntityNative"), "◇");
        assert_eq!(class_icon("Page"), "▣");
        assert_eq!(class_icon("PageNative"), "▢");
        assert_eq!(class_icon("Block"), "▪");
        assert_eq!(class_icon("BlockNative"), "▫");
        assert_eq!(class_icon("Project"), "★");
    }

    #[test]
    fn class_icon_unknown_fallback() {
        assert_eq!(class_icon("CustomThing"), "●");
    }

    #[test]
    fn class_label_format() {
        let label = class_label("Entity");
        assert!(label.contains("◆"));
        assert!(label.contains("Entity"));
    }

    #[test]
    fn fmt_diff_counts_empty() {
        let s = fmt_diff_counts(0, 0, 0);
        assert!(s.contains("in sync"));
    }

    #[test]
    fn fmt_diff_counts_with_values() {
        let s = fmt_diff_counts(3, 1, 2);
        // Should contain colored +3, ~1, -2 (ANSI codes present)
        assert!(s.contains("+3") || s.contains("3"));
    }

    #[test]
    fn step_class_status_smoke() {
        step_class_status("Entity", "42 nodes", true);
        step_class_status("Page", "+3 ~1 -2", false);
    }

    #[test]
    fn print_sync_bar_smoke() {
        print_sync_bar(18, 24);
        print_sync_bar(24, 24);
        print_sync_bar(2, 24);
        print_sync_bar(0, 0);
    }

    #[test]
    fn print_section_smoke() {
        print_section("Test Section");
    }

    #[test]
    fn print_next_step_smoke() {
        print_next_step("run", "novanet data status");
    }

    #[test]
    fn print_flow_arrow_smoke() {
        print_flow_arrow("Database", "Neo4j", "Files", "YAML");
    }
}
