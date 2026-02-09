/**
 * Fuzzy Search Utilities
 *
 * Smart fuzzy matching with:
 * - Exact substring matching (highest score)
 * - Acronym matching (e.g., "gfm" matches "Graph Focus Mode")
 * - Word prefix matching (e.g., "foc" matches "Focus Mode")
 * - Character-by-character fuzzy matching
 *
 * Extracted from CommandPalette for reuse across the app.
 */

// =============================================================================
// TYPES
// =============================================================================

export interface FuzzyMatch {
  /** Whether the query matches the text */
  match: boolean;
  /** Score for ranking (higher = better match) */
  score: number;
  /** Indices of matched characters in the text */
  matchedIndices: number[];
}

export interface FuzzySearchOptions {
  /** Enable acronym matching (default: true) */
  acronymMatch?: boolean;
  /** Enable word prefix matching (default: true) */
  prefixMatch?: boolean;
  /** Minimum score to consider a match (default: 0) */
  threshold?: number;
}

// =============================================================================
// SCORING CONSTANTS
// =============================================================================

const SCORE = {
  /** Exact match bonus */
  EXACT: 100,
  /** Acronym match bonus */
  ACRONYM: 80,
  /** Word prefix match bonus */
  WORD_PREFIX: 70,
  /** Character match base */
  CHAR_MATCH: 10,
  /** Consecutive character bonus */
  CONSECUTIVE: 5,
  /** Match at start of word bonus */
  WORD_START: 3,
} as const;

// =============================================================================
// HELPERS
// =============================================================================

/**
 * Extract word boundaries from text
 * Returns indices where new words start
 */
function getWordBoundaries(text: string): number[] {
  const boundaries: number[] = [0];
  for (let i = 1; i < text.length; i++) {
    const char = text[i];
    const prevChar = text[i - 1];
    // Word boundary: space, underscore, hyphen, or lowercase to uppercase
    if (
      prevChar === ' ' ||
      prevChar === '_' ||
      prevChar === '-' ||
      (prevChar === prevChar.toLowerCase() && char === char.toUpperCase())
    ) {
      boundaries.push(i);
    }
  }
  return boundaries;
}

/**
 * Get acronym from text (first letter of each word)
 */
function getAcronym(text: string): string {
  const boundaries = getWordBoundaries(text);
  return boundaries.map((i) => text[i].toLowerCase()).join('');
}

// =============================================================================
// MAIN FUNCTIONS
// =============================================================================

/**
 * Smart fuzzy match with multiple matching strategies
 */
export function fuzzyMatch(
  query: string,
  text: string,
  options: FuzzySearchOptions = {}
): FuzzyMatch {
  const {
    acronymMatch = true,
    prefixMatch = true,
    threshold = 0,
  } = options;

  if (!query) {
    return { match: true, score: 0, matchedIndices: [] };
  }

  const lowerQuery = query.toLowerCase();
  const lowerText = text.toLowerCase();
  const matchedIndices: number[] = [];

  // Strategy 1: Exact substring match (highest priority)
  const exactIndex = lowerText.indexOf(lowerQuery);
  if (exactIndex !== -1) {
    for (let i = 0; i < lowerQuery.length; i++) {
      matchedIndices.push(exactIndex + i);
    }
    // Score based on position (earlier = better)
    const positionBonus = Math.max(0, 50 - exactIndex);
    return {
      match: true,
      score: SCORE.EXACT + positionBonus,
      matchedIndices,
    };
  }

  // Strategy 2: Acronym match (e.g., "gfm" -> "Graph Focus Mode")
  if (acronymMatch && lowerQuery.length <= 6) {
    const acronym = getAcronym(text);
    if (acronym.includes(lowerQuery)) {
      const boundaries = getWordBoundaries(text);
      const acronymIndex = acronym.indexOf(lowerQuery);
      for (let i = 0; i < lowerQuery.length; i++) {
        matchedIndices.push(boundaries[acronymIndex + i]);
      }
      return {
        match: true,
        score: SCORE.ACRONYM,
        matchedIndices,
      };
    }
  }

  // Strategy 3: Word prefix match (e.g., "foc" -> "Focus Mode")
  if (prefixMatch) {
    const words = text.split(/[\s_-]+/);
    let currentIndex = 0;

    for (const word of words) {
      if (word.toLowerCase().startsWith(lowerQuery)) {
        for (let i = 0; i < lowerQuery.length; i++) {
          matchedIndices.push(currentIndex + i);
        }
        return {
          match: true,
          score: SCORE.WORD_PREFIX,
          matchedIndices,
        };
      }
      currentIndex += word.length + 1; // +1 for separator
    }
  }

  // Strategy 4: Character-by-character fuzzy match
  let queryIdx = 0;
  let score = 0;
  let consecutiveBonus = 0;
  const wordBoundaries = new Set(getWordBoundaries(text));

  for (let i = 0; i < lowerText.length && queryIdx < lowerQuery.length; i++) {
    if (lowerText[i] === lowerQuery[queryIdx]) {
      matchedIndices.push(i);
      score += SCORE.CHAR_MATCH + consecutiveBonus;

      // Bonus for matching at word start
      if (wordBoundaries.has(i)) {
        score += SCORE.WORD_START;
      }

      consecutiveBonus += SCORE.CONSECUTIVE;
      queryIdx++;
    } else {
      consecutiveBonus = 0;
    }
  }

  const isMatch = queryIdx === lowerQuery.length && score >= threshold;

  return {
    match: isMatch,
    score: isMatch ? score : 0,
    matchedIndices: isMatch ? matchedIndices : [],
  };
}
