/**
 * QueryLibrary - Pre-made Cypher queries for common workflows
 *
 * Each query represents a useful view of the NovaNet graph.
 * Clicking a query executes it against Neo4j.
 */

export interface QueryPreset {
  /** Unique identifier */
  id: string;
  /** Display name */
  name: string;
  /** Brief description */
  description: string;
  /** Emoji icon */
  icon: string;
  /** Cypher query to execute */
  cypher: string;
  /** Category for grouping */
  category: 'workflow' | 'exploration' | 'analytics';
}

const QUERY_LIMIT = 200;

export const QUERY_LIBRARY: QueryPreset[] = [
  // === WORKFLOW QUERIES ===
  {
    id: 'content-pipeline',
    name: 'Content Pipeline',
    description: 'Page → Block → Output structure',
    icon: '📄',
    category: 'workflow',
    cypher: `MATCH (p:Page)-[:HAS_BLOCK]->(b:Block)-[:OF_TYPE]->(bt:BlockType)
OPTIONAL MATCH (b)-[:HAS_OUTPUT]->(bo:BlockOutput)
OPTIONAL MATCH (p)-[:HAS_OUTPUT]->(po:PageOutput)
RETURN p, b, bt, bo, po LIMIT ${QUERY_LIMIT}`,
  },
  {
    id: 'locale-knowledge',
    name: 'Locale Knowledge',
    description: 'Locale → Identity / Voice / Culture',
    icon: '🌍',
    category: 'workflow',
    cypher: `MATCH (l:Locale)
OPTIONAL MATCH (l)-[:HAS_IDENTITY]->(li:LocaleIdentity)
OPTIONAL MATCH (l)-[:HAS_VOICE]->(lv:LocaleVoice)
OPTIONAL MATCH (l)-[:HAS_CULTURE]->(lc:LocaleCulture)
OPTIONAL MATCH (l)-[:HAS_MARKET]->(lm:LocaleMarket)
OPTIONAL MATCH (l)-[:HAS_LEXICON]->(ll:LocaleLexicon)
RETURN l, li, lv, lc, lm, ll LIMIT ${QUERY_LIMIT}`,
  },
  {
    id: 'concept-network',
    name: 'Concept Network',
    description: 'Concepts with semantic links',
    icon: '🧠',
    category: 'workflow',
    cypher: `MATCH (c:Concept)
OPTIONAL MATCH (c)-[sl:SEMANTIC_LINK]-(c2:Concept)
OPTIONAL MATCH (c)-[:HAS_L10N]->(cl:ConceptL10n)
RETURN c, sl, c2, cl LIMIT ${QUERY_LIMIT}`,
  },
  {
    id: 'project-structure',
    name: 'Project Structure',
    description: 'Project → Brand → L10n',
    icon: '🏢',
    category: 'workflow',
    cypher: `MATCH (p:Project)
OPTIONAL MATCH (p)-[:HAS_BRAND_IDENTITY]->(bi:BrandIdentity)
OPTIONAL MATCH (p)-[:HAS_L10N]->(pl:ProjectL10n)
OPTIONAL MATCH (p)-[:HAS_PAGE]->(pg:Page)
RETURN p, bi, pl, pg LIMIT ${QUERY_LIMIT}`,
  },
  {
    id: 'generation-pipeline',
    name: 'Generation Pipeline',
    description: 'Prompts → Rules → Outputs',
    icon: '🤖',
    category: 'workflow',
    cypher: `MATCH (b:Block)
OPTIONAL MATCH (b)-[:HAS_PROMPT]->(bp:BlockPrompt)
OPTIONAL MATCH (b)-[:HAS_RULES]->(br:BlockRules)
OPTIONAL MATCH (b)-[:HAS_OUTPUT]->(bo:BlockOutput)
RETURN b, bp, br, bo LIMIT ${QUERY_LIMIT}`,
  },

  // === EXPLORATION QUERIES ===
  {
    id: 'seo-keywords',
    name: 'SEO Keywords',
    description: 'Keywords with volume & difficulty',
    icon: '🔍',
    category: 'exploration',
    cypher: `MATCH (s:SEOKeyword)
OPTIONAL MATCH (s)<-[:TARGETS_SEO]-(c:Concept)
OPTIONAL MATCH (s)-[:FOR_LOCALE]->(l:Locale)
RETURN s, c, l ORDER BY s.volume DESC LIMIT ${QUERY_LIMIT}`,
  },
  {
    id: 'geo-seeds',
    name: 'GEO Seeds',
    description: 'AI optimization seeds',
    icon: '🎯',
    category: 'exploration',
    cypher: `MATCH (g:GEOSeed)
OPTIONAL MATCH (g)<-[:TARGETS_GEO]-(c:Concept)
OPTIONAL MATCH (g)-[:FOR_LOCALE]->(l:Locale)
RETURN g, c, l LIMIT ${QUERY_LIMIT}`,
  },
  {
    id: 'expressions',
    name: 'Marketing Expressions',
    description: 'Locale-specific expressions',
    icon: '💬',
    category: 'exploration',
    cypher: `MATCH (ll:LocaleLexicon)-[:HAS_EXPRESSION]->(e:Expression)
MATCH (l:Locale)-[:HAS_LEXICON]->(ll)
RETURN l, ll, e LIMIT ${QUERY_LIMIT}`,
  },

  // === ANALYTICS QUERIES ===
  {
    id: 'content-gaps',
    name: 'Content Gaps',
    description: 'SEO/GEO gaps to fill',
    icon: '📊',
    category: 'analytics',
    cypher: `MATCH (sv:SEOVariation {content_gap: true})
OPTIONAL MATCH (sv)<-[:HAS_VARIATION]-(sk:SEOKeyword)
RETURN sv, sk ORDER BY sv.volume DESC LIMIT ${QUERY_LIMIT}`,
  },
  {
    id: 'recent-outputs',
    name: 'Recent Outputs',
    description: 'Latest generated content',
    icon: '🕐',
    category: 'analytics',
    cypher: `MATCH (bo:BlockOutput)
OPTIONAL MATCH (bo)<-[:HAS_OUTPUT]-(b:Block)
RETURN bo, b ORDER BY bo.generated_at DESC LIMIT 50`,
  },
];

/**
 * Generate query to fetch nodes by label
 */
export function queryByLabel(label: string): string {
  return `MATCH (n:${label}) RETURN n LIMIT ${QUERY_LIMIT}`;
}

/**
 * Generate query to fetch nodes by label with relationships
 */
export function queryByLabelWithRels(label: string): string {
  return `MATCH (n:${label})-[r]-(m) RETURN n, r, m LIMIT ${QUERY_LIMIT}`;
}
