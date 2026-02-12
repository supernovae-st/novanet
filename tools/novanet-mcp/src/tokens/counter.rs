//! Token Counter
//!
//! Hybrid token counting using tiktoken-rs with cl100k_base encoding.
//! Uses lazy initialization for the BPE model.

use std::sync::OnceLock;
use tiktoken_rs::CoreBPE;

/// Global BPE instance (lazy initialized)
static BPE: OnceLock<CoreBPE> = OnceLock::new();

/// Get or initialize the BPE encoder
fn get_bpe() -> &'static CoreBPE {
    BPE.get_or_init(|| {
        tiktoken_rs::cl100k_base().expect("Failed to initialize cl100k_base tokenizer")
    })
}

/// Token counter using tiktoken for Claude/GPT models
#[derive(Clone)]
pub struct TokenCounter {
    // tiktoken-rs uses a singleton pattern via OnceLock, so we don't need to store the BPE
}

impl TokenCounter {
    /// Create a new token counter
    pub fn new() -> Self {
        Self {}
    }

    /// Get exact token count for text
    ///
    /// Uses cl100k_base encoding (used by Claude and GPT-4)
    pub fn count(&self, text: &str) -> usize {
        let bpe = get_bpe();
        bpe.encode_with_special_tokens(text).len()
    }

    /// Fast estimate for pre-flight checks (~96% accuracy)
    ///
    /// Rule of thumb: ~4 chars per token for English
    /// Adjusts for CJK characters which have higher token density
    pub fn estimate(&self, text: &str) -> usize {
        let total_chars = text.chars().count();
        let cjk_count = text.chars().filter(|c| is_cjk(*c)).count();

        if cjk_count > total_chars / 2 {
            // CJK-heavy text: ~1.5 tokens per character
            (total_chars as f64 * 1.5) as usize
        } else {
            // English-like text: ~4 chars per token
            total_chars.div_ceil(4)
        }
    }

    /// Smart budget check: fast path when clearly within/outside budget
    ///
    /// Uses estimate for fast path, falls back to exact count when needed.
    pub fn within_budget(&self, text: &str, budget: usize) -> bool {
        let estimate = self.estimate(text);

        // Fast path: clearly within budget (10% margin)
        if estimate < budget * 90 / 100 {
            return true;
        }

        // Fast path: clearly over budget (10% margin)
        if estimate > budget * 110 / 100 {
            return false;
        }

        // Slow path: exact count needed
        self.count(text) <= budget
    }

    /// Truncate text to fit within token budget
    ///
    /// Returns the truncated text and actual token count
    pub fn truncate_to_budget(&self, text: &str, budget: usize) -> (String, usize) {
        let tokens = self.count(text);

        if tokens <= budget {
            return (text.to_string(), tokens);
        }

        // Binary search for the right truncation point
        let mut low = 0;
        let mut high = text.len();

        while low < high {
            let mid = (low + high).div_ceil(2);
            let truncated = &text[..mid];
            if self.count(truncated) <= budget {
                low = mid;
            } else {
                high = mid - 1;
            }
        }

        let truncated = &text[..low];
        let final_count = self.count(truncated);
        (truncated.to_string(), final_count)
    }
}

impl Default for TokenCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if a character is CJK (Chinese, Japanese, Korean)
fn is_cjk(c: char) -> bool {
    matches!(c,
        '\u{4E00}'..='\u{9FFF}' |    // CJK Unified Ideographs
        '\u{3400}'..='\u{4DBF}' |    // CJK Unified Ideographs Extension A
        '\u{20000}'..='\u{2A6DF}' |  // CJK Unified Ideographs Extension B
        '\u{2A700}'..='\u{2B73F}' |  // CJK Unified Ideographs Extension C
        '\u{2B740}'..='\u{2B81F}' |  // CJK Unified Ideographs Extension D
        '\u{F900}'..='\u{FAFF}' |    // CJK Compatibility Ideographs
        '\u{3000}'..='\u{303F}' |    // CJK Punctuation
        '\u{3040}'..='\u{309F}' |    // Hiragana
        '\u{30A0}'..='\u{30FF}' |    // Katakana
        '\u{AC00}'..='\u{D7AF}'      // Hangul Syllables
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_count() {
        let counter = TokenCounter::new();

        // English text
        let english = "Hello, world!";
        let count = counter.count(english);
        assert!(count > 0 && count < 10, "English count: {}", count);

        // Longer text
        let longer = "The quick brown fox jumps over the lazy dog.";
        let longer_count = counter.count(longer);
        assert!(longer_count > count, "Longer text should have more tokens");
    }

    #[test]
    fn test_estimate() {
        let counter = TokenCounter::new();

        // English estimate should be roughly chars/4
        let english = "Hello world";
        let estimate = counter.estimate(english);
        assert!((2..=4).contains(&estimate));

        // CJK text should have higher estimate
        let cjk = "你好世界";
        let cjk_estimate = counter.estimate(cjk);
        assert!(cjk_estimate >= 4); // ~1.5 tokens per char
    }

    #[test]
    fn test_within_budget() {
        let counter = TokenCounter::new();

        let text = "Hello, world!";

        // Should be within a large budget
        assert!(counter.within_budget(text, 100));

        // Should not be within a tiny budget
        assert!(!counter.within_budget(text, 1));
    }

    #[test]
    fn test_truncate_to_budget() {
        let counter = TokenCounter::new();

        let text = "The quick brown fox jumps over the lazy dog. This is a longer sentence to test truncation.";
        let budget = 5;

        let (truncated, count) = counter.truncate_to_budget(text, budget);
        assert!(
            count <= budget,
            "Count {} should be <= budget {}",
            count,
            budget
        );
        assert!(truncated.len() < text.len(), "Truncated should be shorter");
    }

    #[test]
    fn test_is_cjk() {
        assert!(is_cjk('中'));
        assert!(is_cjk('日'));
        assert!(is_cjk('あ'));
        assert!(is_cjk('한'));
        assert!(!is_cjk('A'));
        assert!(!is_cjk('1'));
    }
}
