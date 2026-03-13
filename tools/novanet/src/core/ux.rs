//! Shared UX helpers for polished CLI output.
//!
//! Provides consistent visual patterns across all CLI commands:
//! - **Banners**: Header boxes with title, description, and metadata
//! - **Steps**: Narrated step-by-step progress with status indicators
//! - **Summaries**: Result boxes with key metrics
//! - **Next steps**: Actionable hints for the next command

use colored::Colorize;

// =============================================================================
// BANNER
// =============================================================================

/// Print a command header banner with title, description, and key-value metadata.
///
/// ```text
/// ┌─────────────────────────────────────────────────────────────────────┐
/// │  NOVANET DATA EXPORT                                                │
/// │  Export Neo4j data nodes to YAML for version control                │
/// │                                                                     │
/// │  Source: bolt://localhost:7687 (neo4j)                               │
/// │  Output: ~/.novanet/export/                                         │
/// └─────────────────────────────────────────────────────────────────────┘
/// ```
pub fn print_banner(title: &str, description: &str, metadata: &[(&str, String)]) {
    // Calculate width from content
    let content_lines: Vec<String> = {
        let mut lines = vec![title.to_string(), description.to_string()];
        if !metadata.is_empty() {
            lines.push(String::new()); // blank separator
            for (label, value) in metadata {
                lines.push(format!("{label}: {value}"));
            }
        }
        lines
    };

    let max_content = content_lines
        .iter()
        .map(|l| visible_len(l))
        .max()
        .unwrap_or(40);
    let width = (max_content + 6).max(60); // padding + minimum

    eprintln!();
    eprintln!("  {}", border_top(width));

    for line in &content_lines {
        if line.is_empty() {
            eprintln!("  {}", border_empty(width));
        } else {
            eprintln!("  {}", border_line(line, width));
        }
    }

    eprintln!("  {}", border_bottom(width));
    eprintln!();
}

// =============================================================================
// STEP INDICATORS
// =============================================================================

/// Print a completed step with a check mark.
pub fn step_ok(label: &str, detail: &str) {
    eprintln!(
        "  {} {}  {}",
        "OK".green().bold(),
        label.bold(),
        detail.dimmed()
    );
}

/// Print a completed step with count alignment.
pub fn step_ok_count(label: &str, count: usize, suffix: &str) {
    eprintln!(
        "  {} {:>16}  {:>6} {}",
        "OK".green().bold(),
        label.bold(),
        count.to_string().cyan(),
        suffix.dimmed()
    );
}

/// Print a skipped step.
pub fn step_skip(label: &str, reason: &str) {
    eprintln!(
        "  {} {}  {}",
        "--".dimmed(),
        label.dimmed(),
        reason.dimmed()
    );
}

/// Print a warning step.
pub fn step_warn(label: &str, detail: &str) {
    eprintln!(
        "  {} {}  {}",
        "!!".yellow().bold(),
        label.bold(),
        detail.yellow()
    );
}

// =============================================================================
// SUMMARY BOX
// =============================================================================

/// Print a summary box with a status icon and key metrics.
///
/// ```text
/// ┌─────────────────────────────────────────────────────────────────────┐
/// │  Export complete -- 1,816 nodes -> 6 files                          │
/// │  Output: ~/.novanet/export/                                         │
/// │  Checkpoint saved (use --incremental next time for delta)           │
/// └─────────────────────────────────────────────────────────────────────┘
/// ```
pub fn print_summary_box(lines: &[String]) {
    let max_content = lines
        .iter()
        .map(|l| visible_len(l))
        .max()
        .unwrap_or(40);
    let width = (max_content + 6).max(60);

    eprintln!();
    eprintln!("  {}", border_top(width));
    for line in lines {
        eprintln!("  {}", border_line(line, width));
    }
    eprintln!("  {}", border_bottom(width));
}

// =============================================================================
// NEXT STEPS
// =============================================================================

/// Print a "next step" hint pointing to the next command in the workflow.
pub fn print_next_step(hint: &str, command: &str) {
    eprintln!();
    eprintln!(
        "  {} {} {}",
        "Next:".bold(),
        hint,
        command.cyan()
    );
    eprintln!();
}

// =============================================================================
// FLOW DIAGRAM
// =============================================================================

/// Print the data management workflow diagram.
/// Shows the 3-step pipeline with the current step highlighted.
///
/// ```text
///   HOW IT WORKS
///   Your content lives in a Neo4j database (running in Docker).
///   These commands let you save, check, and version-control that content.
///
///   ┌──────────┐       ┌──────────┐       ┌──────────┐
///   │ Database │──1──> │  Local   │──3──> │   Git    │
///   │ (Neo4j)  │       │  backup  │       │  repo    │
///   └──────────┘       └──────────┘       └──────────┘
///        ^                   │
///        └────── 2. check ───┘
///
///   1. export   Save database content to local YAML files
///   2. diff     Check what changed since last export
///   3. promote  Copy local files to git for version control
/// ```
pub fn print_data_flow(active_step: Option<u8>) {
    eprintln!();
    eprintln!("  {}", "HOW IT WORKS".bold());
    eprintln!(
        "  {}",
        "Your content lives in a Neo4j database (running in Docker).".dimmed()
    );
    eprintln!(
        "  {}",
        "These commands save, check, and version-control that content.".dimmed()
    );
    eprintln!();

    // Simple 3-box diagram
    eprintln!(
        "  {}       {}       {}",
        "Database".bold(),
        "Local backup".bold(),
        "Git repo".bold(),
    );
    eprintln!(
        "  {}  ──1──>  {}  ──3──>  {}",
        "(Neo4j)  ".dimmed(),
        "(~/.novanet/) ".dimmed(),
        "(private-data/)".dimmed(),
    );
    eprintln!(
        "  {}  <──2──  {}",
        "         ".dimmed(),
        "              ".dimmed(),
    );
    eprintln!();

    // Legend with active step highlighted
    let labels: [(u8, &str, &str); 3] = [
        (1, "1. export ", "Save database content to local files"),
        (2, "2. diff   ", "Check what changed since last export"),
        (3, "3. promote", "Copy local files to git for version control"),
    ];

    for (step, label, desc) in &labels {
        if active_step == Some(*step) {
            eprintln!("  {} {}  {}", ">>".cyan().bold(), label.cyan().bold(), desc);
        } else {
            eprintln!("     {}  {}", label.dimmed(), desc.dimmed());
        }
    }
    eprintln!();
}

/// Print a simpler, cleaner flow diagram for command banners.
/// Shows source → destination with human labels.
pub fn print_flow_arrow(from_label: &str, from_detail: &str, to_label: &str, to_detail: &str) {
    eprintln!(
        "  {} {}  --->  {} {}",
        from_label.bold(),
        format!("({from_detail})").dimmed(),
        to_label.bold(),
        format!("({to_detail})").dimmed()
    );
    eprintln!();
}

// =============================================================================
// DRY RUN NOTICE
// =============================================================================

/// Print a dry-run notice under the banner.
pub fn print_dry_run_notice() {
    eprintln!(
        "  {}",
        "(dry run -- no files will be written)".dimmed()
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
    let mut result = String::with_capacity(s.len() + s.len() / 3);
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
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

fn border_line(content: &str, width: usize) -> String {
    let content_len = visible_len(content);
    let padding = if width > content_len + 4 {
        width - content_len - 4
    } else {
        1
    };
    format!(
        "\u{2502}  {}{}  \u{2502}",
        content,
        " ".repeat(padding)
    )
}

fn border_empty(width: usize) -> String {
    format!("\u{2502}{}  \u{2502}", " ".repeat(width - 2))
}

/// Get the visible length of a string (ignoring ANSI escape sequences).
fn visible_len(s: &str) -> usize {
    // Strip ANSI escape sequences for length calculation
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
        // Simulate colored text: \x1b[32mhello\x1b[0m
        assert_eq!(visible_len("\x1b[32mhello\x1b[0m"), 5);
    }

    #[test]
    fn border_top_creates_box() {
        let top = border_top(20);
        assert!(top.starts_with('\u{250c}'));
        assert!(top.ends_with('\u{2510}'));
    }

    #[test]
    fn border_line_pads_content() {
        let line = border_line("test", 20);
        assert!(line.starts_with('\u{2502}'));
        assert!(line.ends_with('\u{2502}'));
        assert!(line.contains("test"));
    }

    // Smoke tests: just verify they don't panic
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
    fn step_ok_smoke() {
        step_ok("Entity", "42 nodes");
    }

    #[test]
    fn step_skip_smoke() {
        step_skip("Entity", "0 nodes");
    }
}
