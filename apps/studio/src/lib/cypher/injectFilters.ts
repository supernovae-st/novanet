/**
 * Cypher Filter Injection - Query-First Architecture
 *
 * Injects filter values directly into Cypher queries so that:
 * 1. What's displayed in QueryPill = what's executed
 * 2. Copy-paste to Neo4j Browser works directly
 * 3. No hidden state or post-query filtering
 */

export interface CypherFilters {
  /** Max nodes to return - injects/replaces LIMIT clause */
  displayLimit?: number;
  /** Locale key filter - injects WHERE clause for locale-aware queries */
  localeKey?: string;
}

/**
 * Inject filters into a Cypher query
 *
 * Rules:
 * - displayLimit: replaces existing LIMIT or adds one at the end
 * - localeKey: only injected for queries that reference Locale nodes
 *
 * @param query - Original Cypher query
 * @param filters - Filters to inject
 * @returns Modified query with filters applied
 */
export function injectFilters(query: string, filters: CypherFilters): string {
  let result = query.trim();

  // Remove trailing semicolon for processing (will add back if present)
  const hadSemicolon = result.endsWith(';');
  if (hadSemicolon) {
    result = result.slice(0, -1).trim();
  }

  // 1. Handle displayLimit - replace or add LIMIT clause
  if (filters.displayLimit !== undefined && filters.displayLimit > 0) {
    // Check if query already has LIMIT (anywhere in query, not just at end)
    const limitRegex = /\bLIMIT\s+\d+/i;
    if (limitRegex.test(result)) {
      // Replace the first LIMIT found (primary limiting factor)
      result = result.replace(limitRegex, `LIMIT ${filters.displayLimit}`);
    } else {
      // Add LIMIT at the end
      result = `${result}\nLIMIT ${filters.displayLimit}`;
    }
  }

  // 2. Handle localeKey - substitute parameters OR inject WHERE clause
  // v0.12.5: Also substitute $locale and $nodeKey for Locale-typed queries
  if (filters.localeKey && filters.localeKey !== 'world') {
    // First, try direct parameter substitution for $locale and $nodeKey
    // This handles queries like: MATCH (l:Locale {key: $locale}) or {key: $nodeKey}
    const hasLocaleParam = /\$locale\b/.test(result);
    const hasNodeKeyWithLocale = /\([\w]+:Locale\s*\{key:\s*\$nodeKey\}/.test(result);

    if (hasLocaleParam) {
      // Replace $locale parameter with actual value
      result = result.replace(/\$locale\b/g, `'${filters.localeKey}'`);
    }

    if (hasNodeKeyWithLocale) {
      // Replace $nodeKey when it's used with a Locale node
      result = result.replace(
        /(\([\w]+:Locale\s*\{key:\s*)\$nodeKey(\})/g,
        `$1'${filters.localeKey}'$2`
      );
    }

    // If no parameter substitution happened, try WHERE clause injection
    if (!hasLocaleParam && !hasNodeKeyWithLocale) {
      // Check if query references Locale nodes
      const hasLocaleRef = /\b(Locale|:Locale)\b/i.test(result);
      if (hasLocaleRef) {
        // Find a good place to inject the WHERE clause
        // Look for patterns like (l:Locale) or (locale:Locale)
        const localeVarMatch = result.match(/\((\w+):Locale\)/i);
        if (localeVarMatch) {
          const localeVar = localeVarMatch[1];
          // Check if there's already a WHERE clause
          const whereRegex = /\bWHERE\b/i;
          if (whereRegex.test(result)) {
            // Add to existing WHERE with AND
            result = result.replace(
              whereRegex,
              `WHERE ${localeVar}.key = "${filters.localeKey}" AND`
            );
          } else {
            // Find MATCH...RETURN pattern and inject WHERE after MATCH block
            // This is a simplified approach - inject after the locale match
            const matchEndPattern = new RegExp(
              `(\\(${localeVar}:Locale\\)[^)]*(?:\\)|\\]))`,
              'i'
            );
            const matchEnd = result.match(matchEndPattern);
            if (matchEnd) {
              const insertPos = result.indexOf(matchEnd[0]) + matchEnd[0].length;
              // Find next MATCH or OPTIONAL MATCH or WITH or RETURN
              const nextClauseMatch = result.slice(insertPos).match(/\b(MATCH|OPTIONAL|WITH|RETURN)\b/i);
              if (nextClauseMatch) {
                const clausePos = insertPos + (nextClauseMatch.index || 0);
                result =
                  result.slice(0, clausePos) +
                  `WHERE ${localeVar}.key = "${filters.localeKey}"\n` +
                  result.slice(clausePos);
              }
            }
          }
        }
      }
    }
  }

  // Add back semicolon if original had one
  if (hadSemicolon) {
    result = result + ';';
  }

  return result;
}

/**
 * Extract current LIMIT value from a query
 */
export function extractLimit(query: string): number | null {
  const match = query.match(/\bLIMIT\s+(\d+)/i);
  return match ? parseInt(match[1], 10) : null;
}

/**
 * Check if a query is locale-aware (references Locale nodes)
 */
export function isLocaleAware(query: string): boolean {
  return /\b(Locale|:Locale)\b/i.test(query);
}
