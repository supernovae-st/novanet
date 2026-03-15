//! Utility functions for YAML panel.
//!
//! Provides text formatting helpers: word wrapping, timestamp formatting,
//! and path abbreviation.

/// Format a Unix timestamp as a human-readable date string.
/// Returns "YYYY-MM-DD HH:MM" format or the original number if not a valid timestamp.
pub(super) fn format_timestamp(timestamp: i64) -> String {
    // Neo4j timestamps can be in seconds or milliseconds
    // Heuristic: if > 10_000_000_000, it's milliseconds
    let secs = if timestamp > 10_000_000_000 {
        timestamp / 1000
    } else {
        timestamp
    };

    // Only format positive timestamps (valid Unix time)
    if secs < 0 {
        return timestamp.to_string();
    }

    let total_secs = secs as u64;
    let days = total_secs / 86400;
    let remaining = total_secs % 86400;
    let hours = remaining / 3600;
    let minutes = (remaining % 3600) / 60;

    // Simple year/month/day calculation (approximate, good enough for display)
    // Starting from 1970-01-01
    let mut year = 1970u32;
    let mut remaining_days = days;

    loop {
        let days_in_year = if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
            366
        } else {
            365
        };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }

    let is_leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days_in_month = [
        31,
        if is_leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];

    let mut month = 1u32;
    for &d in &days_in_month {
        if remaining_days < d {
            break;
        }
        remaining_days -= d;
        month += 1;
    }
    let day = remaining_days + 1;

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        year, month, day, hours, minutes
    )
}

/// Abbreviate a YAML path to show only the last 3 segments.
/// Example: "packages/core/models/node-classes/org/semantic/entity-native.yaml"
///       -> "org/semantic/entity-native.yaml"
pub(super) fn abbreviate_yaml_path(path: &str) -> String {
    let segments: Vec<&str> = path.split('/').collect();
    if segments.len() <= 3 {
        path.to_string()
    } else {
        segments[segments.len() - 3..].join("/")
    }
}

/// Word-wrap a string at word boundaries.
pub(super) fn word_wrap(text: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 {
        return vec![text.to_string()];
    }

    let mut result = Vec::new();

    for paragraph in text.split('\n') {
        if paragraph.is_empty() {
            result.push(String::new());
            continue;
        }

        let mut current_line = String::new();

        for word in paragraph.split_whitespace() {
            if current_line.is_empty() {
                current_line = word.to_string();
            } else if current_line.chars().count() + 1 + word.chars().count() <= max_width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                result.push(current_line);
                current_line = word.to_string();
            }
        }

        if !current_line.is_empty() {
            result.push(current_line);
        }
    }

    if result.is_empty() {
        result.push(String::new());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // format_timestamp tests
    // =========================================================================

    #[test]
    fn test_format_timestamp_unix_epoch() {
        let result = format_timestamp(0);
        assert_eq!(result, "1970-01-01 00:00");
    }

    #[test]
    fn test_format_timestamp_known_date() {
        let result = format_timestamp(1705322200);
        assert_eq!(result, "2024-01-15 12:36");
    }

    #[test]
    fn test_format_timestamp_milliseconds() {
        let result = format_timestamp(1705322200000);
        assert_eq!(result, "2024-01-15 12:36");
    }

    #[test]
    fn test_format_timestamp_negative() {
        let result = format_timestamp(-1000);
        assert_eq!(result, "-1000");
    }

    // =========================================================================
    // word_wrap tests
    // =========================================================================

    #[test]
    fn test_word_wrap_short_text() {
        let result = word_wrap("hello world", 50);
        assert_eq!(result, vec!["hello world"]);
    }

    #[test]
    fn test_word_wrap_long_text() {
        let result = word_wrap("the quick brown fox jumps over the lazy dog", 20);
        assert_eq!(
            result,
            vec!["the quick brown fox", "jumps over the lazy", "dog"]
        );
    }

    #[test]
    fn test_word_wrap_with_newlines() {
        let result = word_wrap("first line\nsecond line", 50);
        assert_eq!(result, vec!["first line", "second line"]);
    }

    #[test]
    fn test_word_wrap_empty_string() {
        let result = word_wrap("", 50);
        assert_eq!(result, vec![""]);
    }

    #[test]
    fn test_word_wrap_zero_width() {
        let result = word_wrap("hello", 0);
        assert_eq!(result, vec!["hello"]);
    }

    #[test]
    fn test_word_wrap_single_long_word() {
        let result = word_wrap("supercalifragilisticexpialidocious", 10);
        assert_eq!(result, vec!["supercalifragilisticexpialidocious"]);
    }

    #[test]
    fn test_word_wrap_utf8() {
        let result = word_wrap("你好 世界 很高兴见到你", 10);
        assert!(!result.is_empty());
        assert!(result.iter().all(|line| line.chars().count() <= 12));
    }
}
