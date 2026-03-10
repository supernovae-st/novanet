// ============================================================================
// MIGRATION 072: Fix Slugification Schema Drift
// Date: 2026-03-09
// Version: v0.17.3
// Purpose: Add missing required properties to Slugification nodes
//
// Schema requires: key, display_name, description, llm_context, created_at,
//                  updated_at, slug_rule, preserve_diacritics, unicode_normalization,
//                  min_length, max_length, separator, lowercase, preserve_numbers,
//                  stop_words
// Seed has: key, display_name, description, slug_rule, stopwords (wrong name),
//           llm_context, created_at, updated_at
//
// Mapping:
//   - preserve_diacritics ← derived from slug_rule
//   - unicode_normalization ← 'NFC' (default)
//   - min_length ← 3 (default)
//   - max_length ← 80 (default)
//   - separator ← '-' (default)
//   - lowercase ← derived from slug_rule (false for native_script)
//   - preserve_numbers ← true (default)
//   - stop_words ← stopwords (rename)
// ============================================================================

// Step 1: Add preserve_diacritics based on slug_rule
MATCH (s:Slugification)
WHERE s.preserve_diacritics IS NULL
SET s.preserve_diacritics = CASE s.slug_rule
  WHEN 'latin_preserve' THEN true
  WHEN 'native_script' THEN true
  WHEN 'latin_strip' THEN false
  WHEN 'transliterate' THEN false
  ELSE false
END
RETURN count(s) AS slugification_preserve_diacritics_fixed;

// Step 2: Add unicode_normalization default
MATCH (s:Slugification)
WHERE s.unicode_normalization IS NULL
SET s.unicode_normalization = 'NFC'
RETURN count(s) AS slugification_unicode_normalization_fixed;

// Step 3: Add min_length default
MATCH (s:Slugification)
WHERE s.min_length IS NULL
SET s.min_length = 3
RETURN count(s) AS slugification_min_length_fixed;

// Step 4: Add max_length default
MATCH (s:Slugification)
WHERE s.max_length IS NULL
SET s.max_length = 80
RETURN count(s) AS slugification_max_length_fixed;

// Step 5: Add separator default
MATCH (s:Slugification)
WHERE s.separator IS NULL
SET s.separator = '-'
RETURN count(s) AS slugification_separator_fixed;

// Step 6: Add lowercase based on slug_rule
// Scripts like Arabic, Hebrew don't have case
MATCH (s:Slugification)
WHERE s.lowercase IS NULL
SET s.lowercase = CASE s.slug_rule
  WHEN 'native_script' THEN false
  ELSE true
END
RETURN count(s) AS slugification_lowercase_fixed;

// Step 7: Add preserve_numbers default
MATCH (s:Slugification)
WHERE s.preserve_numbers IS NULL
SET s.preserve_numbers = true
RETURN count(s) AS slugification_preserve_numbers_fixed;

// Step 8: Rename stopwords → stop_words (copy to new property)
MATCH (s:Slugification)
WHERE s.stopwords IS NOT NULL AND s.stop_words IS NULL
SET s.stop_words = s.stopwords
RETURN count(s) AS slugification_stop_words_copied;

// Step 9: Remove old stopwords property
MATCH (s:Slugification)
WHERE s.stopwords IS NOT NULL
REMOVE s.stopwords
RETURN count(s) AS slugification_stopwords_removed;

// Verification query
MATCH (s:Slugification)
WHERE s.preserve_diacritics IS NOT NULL
  AND s.unicode_normalization IS NOT NULL
  AND s.min_length IS NOT NULL
  AND s.max_length IS NOT NULL
  AND s.separator IS NOT NULL
  AND s.lowercase IS NOT NULL
  AND s.preserve_numbers IS NOT NULL
RETURN count(s) AS slugifications_now_valid;
