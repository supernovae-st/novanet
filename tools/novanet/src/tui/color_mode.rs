//! Terminal color capability detection.
//!
//! Detects the terminal's color support level from environment variables
//! (COLORTERM, TERM) and provides a simple enum for color mode selection.

/// Terminal color capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
    /// Full 24-bit RGB support (modern terminals)
    TrueColor,
    /// 256-color xterm palette
    Color256,
    /// 16-color basic palette
    Color16,
}

impl ColorMode {
    /// Detect terminal color capability from environment.
    pub fn detect() -> Self {
        // Check COLORTERM for true color support
        if let Ok(colorterm) = std::env::var("COLORTERM") {
            if colorterm == "truecolor" || colorterm == "24bit" {
                return ColorMode::TrueColor;
            }
        }

        // Check TERM for 256-color support
        if let Ok(term) = std::env::var("TERM") {
            if term.contains("256color") || term.contains("256-color") {
                return ColorMode::Color256;
            }
        }

        // Fallback to 16-color
        ColorMode::Color16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_mode_detect_returns_valid_variant() {
        // detect() should always return a valid variant without panicking
        let mode = ColorMode::detect();
        assert!(
            matches!(mode, ColorMode::TrueColor | ColorMode::Color256 | ColorMode::Color16),
            "detect() should return a valid ColorMode variant"
        );
    }

    #[test]
    fn test_color_mode_clone_eq() {
        let mode = ColorMode::TrueColor;
        let cloned = mode;
        assert_eq!(mode, cloned);
    }
}
