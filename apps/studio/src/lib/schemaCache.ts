/**
 * Schema Cache for Neo4j
 *
 * Provides cached schema information for AI prompt injection.
 * Cache invalidates after TTL (default: 5 minutes).
 */

import { getDriver } from './neo4j';
import { logger } from './logger';

// =============================================================================
// Configuration
// =============================================================================

/** Schema limits to prevent timeouts and token bloat */
const SCHEMA_LIMITS = {
  /** Maximum node types to fetch properties for */
  maxNodeTypes: 50,
  /** Maximum relationship types to fetch properties for */
  maxRelTypes: 30,
  /** Sample size per node type for property discovery */
  maxSampleNodesPerType: 5,
  /** Maximum properties to show per type in prompt */
  maxPropsPerType: 10,
};

/** Cache TTL in milliseconds (5 minutes) */
const CACHE_TTL_MS = 5 * 60 * 1000;

/** Error backoff in milliseconds (10 seconds) - prevents retry storms */
const ERROR_BACKOFF_MS = 10 * 1000;

// =============================================================================
// Types
// =============================================================================

export interface SchemaInfo {
  nodeTypes: string[];
  relationshipTypes: string[];
  nodeProperties: Record<string, string[]>;
  relationshipProperties: Record<string, string[]>;
  fetchedAt: number;
  /** True if returned from stale cache after fetch failure */
  isStale?: boolean;
  /** True if schema was truncated due to limits */
  isTruncated?: boolean;
}

// =============================================================================
// Cache State
// =============================================================================

let cachedSchema: SchemaInfo | null = null;
/** Mutex for concurrent fetch deduplication */
let schemaFetchPromise: Promise<SchemaInfo> | null = null;
/** Timestamp of last fetch error (for backoff) */
let lastErrorAt: number = 0;

// =============================================================================
// Schema Fetching
// =============================================================================

/**
 * Fetch fresh schema from Neo4j
 */
async function fetchSchemaFromNeo4j(): Promise<SchemaInfo> {
  const driver = getDriver();
  const session = driver.session();
  let isTruncated = false;

  try {
    // Get all node labels
    const labelsResult = await session.run('CALL db.labels()');
    const allNodeTypes = labelsResult.records.map((r) => r.get('label') as string);

    // Get all relationship types
    const relTypesResult = await session.run('CALL db.relationshipTypes()');
    const allRelTypes = relTypesResult.records.map((r) => r.get('relationshipType') as string);

    // Log if truncating
    if (allNodeTypes.length > SCHEMA_LIMITS.maxNodeTypes) {
      logger.warn(
        'SchemaCache',
        `Truncating ${allNodeTypes.length} node types to ${SCHEMA_LIMITS.maxNodeTypes}`
      );
      isTruncated = true;
    }
    if (allRelTypes.length > SCHEMA_LIMITS.maxRelTypes) {
      logger.warn(
        'SchemaCache',
        `Truncating ${allRelTypes.length} relationship types to ${SCHEMA_LIMITS.maxRelTypes}`
      );
      isTruncated = true;
    }

    const nodeTypes = allNodeTypes.slice(0, SCHEMA_LIMITS.maxNodeTypes);
    const relationshipTypes = allRelTypes.slice(0, SCHEMA_LIMITS.maxRelTypes);

    // Get node properties per label (sample a few nodes per type)
    const nodeProperties: Record<string, string[]> = {};
    for (const label of nodeTypes) {
      try {
        const propsResult = await session.run(
          `MATCH (n:\`${label}\`) WITH n LIMIT $limit UNWIND keys(n) AS key RETURN DISTINCT key`,
          { limit: SCHEMA_LIMITS.maxSampleNodesPerType }
        );
        nodeProperties[label] = propsResult.records.map((r) => r.get('key') as string);
      } catch {
        nodeProperties[label] = [];
      }
    }

    // Get relationship properties per type (sample a few relationships)
    const relationshipProperties: Record<string, string[]> = {};
    for (const relType of relationshipTypes) {
      try {
        const propsResult = await session.run(
          `MATCH ()-[r:\`${relType}\`]->() WITH r LIMIT $limit UNWIND keys(r) AS key RETURN DISTINCT key`,
          { limit: SCHEMA_LIMITS.maxSampleNodesPerType }
        );
        relationshipProperties[relType] = propsResult.records.map((r) => r.get('key') as string);
      } catch {
        relationshipProperties[relType] = [];
      }
    }

    return {
      nodeTypes: allNodeTypes, // Return all for reference
      relationshipTypes: allRelTypes,
      nodeProperties,
      relationshipProperties,
      fetchedAt: Date.now(),
      isTruncated,
    };
  } finally {
    await session.close();
  }
}

// =============================================================================
// Public API
// =============================================================================

/**
 * Get schema info (from cache if valid, otherwise fetch fresh)
 * Uses mutex to prevent concurrent fetches
 */
export async function getSchema(): Promise<SchemaInfo> {
  const now = Date.now();

  // Return cached if still valid
  if (cachedSchema && now - cachedSchema.fetchedAt < CACHE_TTL_MS) {
    logger.info('SchemaCache', 'Using cached schema');
    return cachedSchema;
  }

  // Backoff after error - prevent retry storms
  // Return stale cache during backoff period instead of hammering Neo4j
  if (lastErrorAt && now - lastErrorAt < ERROR_BACKOFF_MS) {
    logger.info('SchemaCache', 'Within error backoff period, using stale cache');
    if (cachedSchema) {
      return { ...cachedSchema, isStale: true };
    }
    // No cache available, return empty schema during backoff
    return {
      nodeTypes: [],
      relationshipTypes: [],
      nodeProperties: {},
      relationshipProperties: {},
      fetchedAt: 0,
      isStale: true,
    };
  }

  // Deduplicate concurrent fetches (mutex pattern)
  // Note: This is safe because JS is single-threaded - no await between
  // the check and assignment means no other code can run in between.
  if (schemaFetchPromise) {
    logger.info('SchemaCache', 'Waiting for existing fetch...');
    return schemaFetchPromise;
  }

  // Fetch fresh schema - assign promise immediately (synchronous, no race possible)
  logger.info('SchemaCache', 'Fetching fresh schema from Neo4j...');
  schemaFetchPromise = fetchSchemaFromNeo4j();

  try {
    cachedSchema = await schemaFetchPromise;
    lastErrorAt = 0; // Clear error state on success
    logger.info(
      'SchemaCache',
      `Schema cached: ${cachedSchema.nodeTypes.length} node types, ${cachedSchema.relationshipTypes.length} relationship types${cachedSchema.isTruncated ? ' (truncated)' : ''}`
    );
    return cachedSchema;
  } catch (error) {
    lastErrorAt = Date.now(); // Set error timestamp for backoff
    logger.error('SchemaCache', 'Failed to fetch schema', error);

    // Return stale cache if available
    if (cachedSchema) {
      logger.warn('SchemaCache', 'Returning stale cache');
      return { ...cachedSchema, isStale: true };
    }

    // Return empty schema as fallback
    return {
      nodeTypes: [],
      relationshipTypes: [],
      nodeProperties: {},
      relationshipProperties: {},
      fetchedAt: 0,
      isStale: true,
    };
  } finally {
    schemaFetchPromise = null;
  }
}

/**
 * Format schema for AI prompt injection
 */
export function formatSchemaForPrompt(schema: SchemaInfo): string {
  const lines: string[] = [];

  lines.push('## Live Database Schema');
  lines.push(`**Fetched**: ${new Date(schema.fetchedAt).toLocaleTimeString()}${schema.isStale ? ' (stale)' : ''}${schema.isTruncated ? ' (truncated)' : ''}`);
  lines.push(`**Total**: ${schema.nodeTypes.length} node types, ${schema.relationshipTypes.length} relationship types`);
  lines.push('');

  // Node types with properties (limited by SCHEMA_LIMITS)
  lines.push('### Node Types');
  const nodeTypesToShow = schema.nodeTypes.slice(0, SCHEMA_LIMITS.maxNodeTypes);
  for (const nodeType of nodeTypesToShow) {
    const props = schema.nodeProperties[nodeType];
    if (props && props.length > 0) {
      const propsToShow = props.slice(0, SCHEMA_LIMITS.maxPropsPerType);
      lines.push(`- **${nodeType}**: ${propsToShow.join(', ')}${props.length > SCHEMA_LIMITS.maxPropsPerType ? '...' : ''}`);
    } else {
      lines.push(`- **${nodeType}**`);
    }
  }

  lines.push('');

  // Relationship types
  lines.push('### Relationship Types');
  const relTypesToShow = schema.relationshipTypes.slice(0, SCHEMA_LIMITS.maxRelTypes);
  for (const relType of relTypesToShow) {
    const props = schema.relationshipProperties[relType];
    if (props && props.length > 0) {
      lines.push(`- **${relType}**: ${props.join(', ')}`);
    } else {
      lines.push(`- **${relType}**`);
    }
  }

  return lines.join('\n');
}
