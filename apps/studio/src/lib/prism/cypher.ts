/**
 * Cypher Language Grammar for Prism
 *
 * Based on Neo4j Cypher syntax specification.
 * Used for syntax highlighting in CodeViewer.
 *
 * @see https://neo4j.com/docs/cypher-manual/current/
 */

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type PrismType = any;

/**
 * Register Cypher language with Prism
 *
 * @param prism - Prism instance from prism-react-renderer
 */
export function registerCypher(prism: PrismType): void {
  // Skip if already registered
  if (prism.languages?.cypher) {
    return;
  }

  prism.languages.cypher = {
    // Comments
    comment: [
      {
        pattern: /\/\/.*$/m,
        greedy: true,
      },
      {
        pattern: /\/\*[\s\S]*?\*\//,
        greedy: true,
      },
    ],

    // Strings (single and double quoted)
    string: {
      pattern: /'(?:[^'\\]|\\.)*'|"(?:[^"\\]|\\.)*"/,
      greedy: true,
    },

    // Parameters: $paramName or $`param name`
    parameter: {
      pattern: /\$[a-zA-Z_][a-zA-Z0-9_]*|\$`[^`]+`/,
      alias: 'variable',
    },

    // Node labels and relationship types: :Label
    label: {
      pattern: /:[a-zA-Z_][a-zA-Z0-9_]*/,
      alias: 'class-name',
    },

    // Keywords (case-insensitive)
    keyword: {
      pattern:
        /\b(?:MATCH|OPTIONAL|WHERE|RETURN|WITH|AS|ORDER|BY|SKIP|LIMIT|CREATE|MERGE|DELETE|DETACH|SET|REMOVE|UNION|ALL|UNWIND|CALL|YIELD|CASE|WHEN|THEN|ELSE|END|AND|OR|XOR|NOT|IN|IS|NULL|TRUE|FALSE|STARTS|ENDS|CONTAINS|EXISTS|COUNT|DISTINCT|ASC|ASCENDING|DESC|DESCENDING|ON|CONSTRAINT|INDEX|USING|SCAN|PERIODIC|COMMIT|FOREACH|LOAD|CSV|FROM|HEADERS|FIELDTERMINATOR|EXPLAIN|PROFILE|SHORTESTPATH|ALLSHORTESTPATHS|REDUCE|EXTRACT|FILTER|ANY|NONE|SINGLE|COALESCE|HEAD|TAIL|LAST|SIZE|LENGTH|TYPE|LABELS|KEYS|NODES|RELATIONSHIPS|RANGE|COLLECT|SUM|AVG|MIN|MAX|PERCENTILE|PERCENTILEDISC|PERCENTILECONT|STDEV|STDEVP|PROPERTIES|TIMESTAMP|DATETIME|DATE|TIME|LOCALTIME|LOCALDATETIME|DURATION|POINT|DISTANCE)\b/i,
    },

    // Functions
    function: {
      pattern: /\b[a-zA-Z_][a-zA-Z0-9_]*(?=\s*\()/,
    },

    // Properties: .propertyName
    property: {
      pattern: /\.[a-zA-Z_][a-zA-Z0-9_]*/,
      inside: {
        punctuation: /\./,
      },
    },

    // Numbers
    number: /\b\d+(?:\.\d+)?\b/,

    // Operators
    operator:
      /\+|(?:--?|<>?)(?:-?>)?|<=?|>=?|<>|=~?|!~|:=|\*|\/|%|\^|\+|AND|OR|XOR|NOT|IN/i,

    // Punctuation
    punctuation: /[{}[\](),;.|]/,
  };

  // Alias for convenience
  prism.languages.cql = prism.languages.cypher;
}
