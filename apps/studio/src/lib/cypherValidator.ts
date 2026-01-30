/**
 * Client-side Cypher Validation
 *
 * Basic syntax validation for Cypher queries before execution.
 * Catches common errors early without hitting the server.
 */

// =============================================================================
// Configuration
// =============================================================================

/** Maximum allowed query length (prevent abuse) */
const MAX_QUERY_LENGTH = 5000;

/** Minimum meaningful query length */
const MIN_QUERY_LENGTH = 10;

// =============================================================================
// Types
// =============================================================================

export interface ValidationResult {
  valid: boolean;
  error?: string;
}

// =============================================================================
// Helper Functions
// =============================================================================

/**
 * Check if backticks are properly paired (not dynamic/injection)
 */
function hasValidLabelSyntax(query: string): boolean {
  // Count backticks - must be even
  const backticks = query.match(/`/g) || [];
  if (backticks.length % 2 !== 0) {
    return false;
  }

  // Check for dynamic label construction patterns
  const suspiciousPatterns = [
    /`[^`]*\+[^`]*`/, // Dynamic concatenation inside backticks
    /`[^`]*\$[^`]*`/, // Variable interpolation inside backticks
    /`[^`]*\{[^`]*`/, // Object access inside backticks
  ];

  return !suspiciousPatterns.some((pattern) => pattern.test(query));
}

// =============================================================================
// Validation Rules
// =============================================================================

/**
 * Validate Cypher query syntax (basic checks + injection prevention)
 */
export function validateCypher(query: string): ValidationResult {
  const trimmed = query.trim();

  // Empty query
  if (!trimmed) {
    return { valid: false, error: 'Query cannot be empty' };
  }

  // Length checks
  if (trimmed.length < MIN_QUERY_LENGTH) {
    return { valid: false, error: 'Query too short' };
  }

  if (trimmed.length > MAX_QUERY_LENGTH) {
    return { valid: false, error: `Query too long (max ${MAX_QUERY_LENGTH} characters)` };
  }

  // Must start with a valid Cypher keyword
  const validStarts = ['MATCH', 'OPTIONAL', 'WITH', 'RETURN', 'UNWIND', 'CALL'];
  const upperQuery = trimmed.toUpperCase();
  const startsValid = validStarts.some((keyword) => upperQuery.startsWith(keyword));
  if (!startsValid) {
    return {
      valid: false,
      error: 'Query must start with MATCH, OPTIONAL, WITH, RETURN, UNWIND, or CALL',
    };
  }

  // Must have RETURN (read queries require RETURN)
  if (!upperQuery.includes('RETURN')) {
    return { valid: false, error: 'Read queries must include RETURN clause' };
  }

  // Check balanced parentheses
  const openParens = (trimmed.match(/\(/g) || []).length;
  const closeParens = (trimmed.match(/\)/g) || []).length;
  if (openParens !== closeParens) {
    return { valid: false, error: `Unbalanced parentheses: ${openParens} open, ${closeParens} close` };
  }

  // Check balanced brackets
  const openBrackets = (trimmed.match(/\[/g) || []).length;
  const closeBrackets = (trimmed.match(/\]/g) || []).length;
  if (openBrackets !== closeBrackets) {
    return { valid: false, error: `Unbalanced brackets: ${openBrackets} open, ${closeBrackets} close` };
  }

  // Check balanced braces
  const openBraces = (trimmed.match(/\{/g) || []).length;
  const closeBraces = (trimmed.match(/\}/g) || []).length;
  if (openBraces !== closeBraces) {
    return { valid: false, error: `Unbalanced braces: ${openBraces} open, ${closeBraces} close` };
  }

  // =========================================================================
  // Injection Attack Prevention
  // =========================================================================

  // Check for Cypher comments (can hide malicious code)
  if (trimmed.includes('//') || trimmed.includes('/*')) {
    return { valid: false, error: 'Comments are not allowed in queries' };
  }

  // Check for valid label syntax (prevent injection via backticks)
  if (trimmed.includes('`') && !hasValidLabelSyntax(trimmed)) {
    return { valid: false, error: 'Invalid label syntax detected' };
  }

  // Check for suspicious string concatenation that might bypass filters
  if (/\+\s*['"]/.test(trimmed) || /['"]\s*\+/.test(trimmed)) {
    return { valid: false, error: 'String concatenation is not allowed in queries' };
  }

  // Check for semicolons (multiple statement injection)
  if (trimmed.includes(';')) {
    return { valid: false, error: 'Multiple statements (;) are not allowed' };
  }

  // =========================================================================
  // Common Typos
  // =========================================================================

  if (upperQuery.includes('METCH')) {
    return { valid: false, error: 'Did you mean MATCH? (found METCH)' };
  }
  if (upperQuery.includes('RETRUN')) {
    return { valid: false, error: 'Did you mean RETURN? (found RETRUN)' };
  }
  if (upperQuery.includes('WHER ') || upperQuery.includes('WEHRE')) {
    return { valid: false, error: 'Did you mean WHERE?' };
  }

  // =========================================================================
  // Write Operation Blocking (client-side safety)
  // =========================================================================

  const blockedKeywords = [
    'CREATE',
    'MERGE',
    'SET',
    'DELETE',
    'REMOVE',
    'DETACH',
    'DROP',
    'INDEX',
    'CONSTRAINT',
    'LOAD CSV',
    'FOREACH',
  ];

  for (const keyword of blockedKeywords) {
    // Use word boundary check to avoid false positives (e.g., "CREATED_AT")
    const regex = new RegExp(`\\b${keyword}\\b`, 'i');
    if (regex.test(trimmed)) {
      return { valid: false, error: `Write operation "${keyword}" is not allowed` };
    }
  }

  return { valid: true };
}

/**
 * Quick validation for UI feedback (returns error message or null)
 */
export function getCypherError(query: string): string | null {
  const result = validateCypher(query);
  return result.valid ? null : result.error || 'Invalid query';
}
