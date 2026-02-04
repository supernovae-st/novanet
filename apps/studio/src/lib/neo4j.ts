/**
 * Neo4j Client Library
 *
 * Provides connection pooling, query execution, and result transformation
 * for the novanet-visualizer application.
 *
 * @see https://neo4j.com/docs/javascript-manual/current/
 */

import neo4j, {
  Driver,
  Session,
  Node as Neo4jNode,
  Relationship as Neo4jRelationship,
  Integer,
} from 'neo4j-driver';
import type { GraphNode, GraphEdge, NodeType, RelationType } from '@/types';
import { MAX_NODES, EXPAND_QUERY_LIMIT } from '@/config/constants';
import { logger } from '@/lib/logger';

// =============================================================================
// Configuration
// =============================================================================

const NEO4J_URI = process.env.NEO4J_URI || 'bolt://localhost:7687';
const NEO4J_USER = process.env.NEO4J_USER || 'neo4j';

// SECURITY: Require password from environment - no hardcoded fallback
function getRequiredEnv(name: string): string {
  const value = process.env[name];
  if (!value) {
    throw new Error(
      `${name} environment variable is required. ` +
      'Set it in .env.local for development or in your deployment environment.'
    );
  }
  return value;
}

const NEO4J_PASSWORD = getRequiredEnv('NEO4J_PASSWORD');

/**
 * Query timeout in milliseconds (30 seconds)
 */
const QUERY_TIMEOUT = 30000;

// MAX_NODES imported from @/config/constants

/**
 * Retry configuration
 */
const RETRY_CONFIG = {
  maxRetries: 3,
  initialDelayMs: 500,
  maxDelayMs: 5000,
};

// =============================================================================
// Driver Singleton
// =============================================================================

let driver: Driver | null = null;

/**
 * Get or create the Neo4j driver instance (singleton pattern)
 */
export function getDriver(): Driver {
  if (!driver) {
    driver = neo4j.driver(NEO4J_URI, neo4j.auth.basic(NEO4J_USER, NEO4J_PASSWORD), {
      maxConnectionPoolSize: 50,
      connectionAcquisitionTimeout: 10000,
      connectionTimeout: 5000,
      logging: {
        level: process.env.NODE_ENV === 'development' ? 'info' : 'warn',
        logger: (level, message) => {
          if (level === 'error') {
            logger.error('Neo4j', message);
          } else {
            logger.info('Neo4j', message);
          }
        },
      },
    });
  }
  return driver;
}

/**
 * Sleep helper for retry delays
 */
function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

/**
 * Calculate exponential backoff delay with jitter
 */
function getRetryDelay(attempt: number): number {
  const delay = Math.min(
    RETRY_CONFIG.initialDelayMs * Math.pow(2, attempt),
    RETRY_CONFIG.maxDelayMs
  );
  // Add jitter (±25%)
  return delay * (0.75 + Math.random() * 0.5);
}

/**
 * Check if error is retryable (transient failures)
 */
function isRetryableError(error: unknown): boolean {
  if (error instanceof Error) {
    const message = error.message.toLowerCase();
    return (
      message.includes('connection') ||
      message.includes('timeout') ||
      message.includes('socket') ||
      message.includes('econnrefused') ||
      message.includes('econnreset') ||
      message.includes('service unavailable')
    );
  }
  return false;
}

/**
 * Verify the Neo4j connection is working
 * @returns true if connection is valid
 * @throws Error if connection fails
 */
export async function verifyConnection(): Promise<boolean> {
  const d = getDriver();
  try {
    await d.verifyConnectivity();
    return true;
  } catch (error) {
    logger.error('Neo4j', 'Connection verification failed', error);
    return false;
  }
}

/**
 * Execute with retry logic for transient failures
 */
async function withRetry<T>(
  operation: () => Promise<T>,
  operationName: string
): Promise<T> {
  let lastError: Error | null = null;

  for (let attempt = 0; attempt <= RETRY_CONFIG.maxRetries; attempt++) {
    try {
      return await operation();
    } catch (error) {
      lastError = error instanceof Error ? error : new Error(String(error));

      if (attempt < RETRY_CONFIG.maxRetries && isRetryableError(error)) {
        const delay = getRetryDelay(attempt);
        logger.warn(
          'Neo4j',
          `${operationName} failed (attempt ${attempt + 1}/${RETRY_CONFIG.maxRetries + 1}), retrying in ${Math.round(delay)}ms...`,
          lastError.message
        );
        await sleep(delay);
      } else {
        throw lastError;
      }
    }
  }

  throw lastError || new Error(`${operationName} failed after ${RETRY_CONFIG.maxRetries + 1} attempts`);
}

// =============================================================================
// Type Transformations
// =============================================================================

/**
 * Convert Neo4j Integer to JavaScript number
 */
function toNumber(value: Integer | number | undefined): number {
  if (value === undefined) return 0;
  if (typeof value === 'number') return value;
  return value.toNumber();
}

/**
 * Convert Neo4j Node to GraphNode
 */
export function transformNode(node: Neo4jNode): GraphNode {
  const labels = node.labels;
  const props = node.properties;

  // Get the primary label (node type)
  const type = (labels[0] || 'Unknown') as NodeType;

  // Extract standard properties
  const key = props.key?.toString() || props.id?.toString() || node.elementId;
  const displayName =
    props.display_name?.toString() ||
    props.name?.toString() ||
    props.label?.toString() ||
    key;

  return {
    id: node.elementId,
    type,
    key,
    displayName,
    description: props.description?.toString(),
    llmContext: props.llm_context?.toString(),
    createdAt: props.created_at?.toString(),
    updatedAt: props.updated_at?.toString(),
    // Include all other properties in a generic data field
    data: Object.fromEntries(
      Object.entries(props).filter(
        ([k]) =>
          !['key', 'display_name', 'name', 'label', 'description', 'llm_context', 'created_at', 'updated_at'].includes(k)
      )
    ),
  };
}

/**
 * Convert Neo4j Relationship to GraphEdge
 */
export function transformRelationship(rel: Neo4jRelationship): GraphEdge {
  return {
    id: rel.elementId,
    source: rel.startNodeElementId,
    target: rel.endNodeElementId,
    type: rel.type as RelationType,
    data: rel.properties,
  };
}

// =============================================================================
// Query Execution
// =============================================================================

export interface QueryResult {
  nodes: GraphNode[];
  edges: GraphEdge[];
  totalNodes: number;
  totalArcs: number;
  duration: number;
}

export interface QueryOptions {
  /** Node types to include (empty = all) */
  nodeTypes?: NodeType[];
  /** Locale filter (e.g., 'fr-FR') */
  locale?: string;
  /** Maximum depth for traversal */
  depth?: number;
  /** Maximum nodes to return */
  limit?: number;
  /** Search query for node properties */
  search?: string;
}

/**
 * Execute a read query and transform results to GraphNode/GraphEdge
 * Includes automatic retry for transient connection failures
 */
export async function executeQuery(
  cypher: string,
  params: Record<string, unknown> = {},
  options: { timeout?: number; retries?: boolean } = {}
): Promise<QueryResult> {
  const runQuery = async (): Promise<QueryResult> => {
    const d = getDriver();
    const session: Session = d.session({
      defaultAccessMode: neo4j.session.READ,
    });

    const startTime = Date.now();

    try {
      const result = await session.run(cypher, params, {
        timeout: options.timeout || QUERY_TIMEOUT,
      });

    const nodes: GraphNode[] = [];
    const edges: GraphEdge[] = [];
    const nodeIds = new Set<string>();
    const edgeIds = new Set<string>();

    // Process all records
    for (const record of result.records) {
      // Extract nodes and relationships from each record
      for (const key of record.keys) {
        const value = record.get(key);

        if (value === null || value === undefined) continue;

        // Handle Node
        if (value instanceof neo4j.types.Node) {
          if (!nodeIds.has(value.elementId)) {
            nodeIds.add(value.elementId);
            nodes.push(transformNode(value));
          }
        }
        // Handle Relationship
        else if (value instanceof neo4j.types.Relationship) {
          if (!edgeIds.has(value.elementId)) {
            edgeIds.add(value.elementId);
            edges.push(transformRelationship(value));
          }
        }
        // Handle Path
        else if (value instanceof neo4j.types.Path) {
          // Extract all nodes from path
          for (const segment of value.segments) {
            if (!nodeIds.has(segment.start.elementId)) {
              nodeIds.add(segment.start.elementId);
              nodes.push(transformNode(segment.start));
            }
            if (!nodeIds.has(segment.end.elementId)) {
              nodeIds.add(segment.end.elementId);
              nodes.push(transformNode(segment.end));
            }
            if (!edgeIds.has(segment.relationship.elementId)) {
              edgeIds.add(segment.relationship.elementId);
              edges.push(transformRelationship(segment.relationship));
            }
          }
        }
        // Handle arrays (e.g., collect())
        else if (Array.isArray(value)) {
          for (const item of value) {
            if (item instanceof neo4j.types.Node && !nodeIds.has(item.elementId)) {
              nodeIds.add(item.elementId);
              nodes.push(transformNode(item));
            } else if (item instanceof neo4j.types.Relationship && !edgeIds.has(item.elementId)) {
              edgeIds.add(item.elementId);
              edges.push(transformRelationship(item));
            }
          }
        }
      }
    }

      const duration = Date.now() - startTime;

      return {
        nodes,
        edges,
        totalNodes: nodes.length,
        totalArcs: edges.length,
        duration,
      };
    } finally {
      await session.close();
    }
  };

  // Use retry wrapper for resilience (default: enabled)
  if (options.retries !== false) {
    return withRetry(runQuery, 'executeQuery');
  }
  return runQuery();
}

// =============================================================================
// Graph Queries
// =============================================================================

/**
 * Fetch graph data with filters
 *
 * @description Fetches nodes and edges from Neo4j with optional filtering.
 * Only returns nodes that have at least one relationship (mimics Neo4j Browser behavior).
 * Orphan nodes (nodes without connections) are excluded from results.
 *
 * @note The returned node count may exceed `limit` because related nodes (m) are
 * included even if they weren't in the initial limited set.
 *
 * @param options - Query options (nodeTypes, locale, search, limit)
 * @returns Promise<QueryResult> - Nodes and edges with metadata
 */
export async function fetchGraphData(options: QueryOptions = {}): Promise<QueryResult> {
  const { nodeTypes = [], locale, limit = MAX_NODES, search } = options;

  // Build dynamic Cypher query
  const cypherParts: string[] = [];
  const params: Record<string, unknown> = {
    // Ensure limit is an integer (Neo4j requires integer for LIMIT)
    limit: neo4j.int(Math.min(limit, MAX_NODES)),
  };

  // Determine if filtering by node types
  const hasNodeTypeFilter = nodeTypes.length > 0;

  // Base match clause
  if (hasNodeTypeFilter) {
    // Filter by specific node types
    cypherParts.push(`MATCH (n) WHERE any(label IN labels(n) WHERE label IN $nodeTypes)`);
    params.nodeTypes = nodeTypes;
  } else {
    cypherParts.push(`MATCH (n)`);
  }

  // Locale filter
  if (locale) {
    const localeClause = hasNodeTypeFilter ? 'AND' : 'WHERE';
    cypherParts.push(`${localeClause} (n.locale_code = $locale OR (n:Locale AND n.code = $locale))`);
    params.locale = locale;
  }

  // Search filter
  if (search) {
    const searchClause = (hasNodeTypeFilter || locale) ? 'AND' : 'WHERE';
    cypherParts.push(
      `${searchClause} (n.key CONTAINS $search OR n.display_name CONTAINS $search OR n.description CONTAINS $search)`
    );
    params.search = search;
  }

  // Return with relationships - use MATCH (not OPTIONAL MATCH) to exclude orphan nodes
  // This mimics Neo4j Browser behavior where only connected nodes are displayed
  if (hasNodeTypeFilter) {
    // Filter related nodes by same node types
    cypherParts.push(`
      WITH n LIMIT $limit
      MATCH (n)-[r]-(m)
      WHERE any(label IN labels(m) WHERE label IN $nodeTypes)
      RETURN n, r, m
    `);
  } else {
    // No node type filter - return all related nodes
    cypherParts.push(`
      WITH n LIMIT $limit
      MATCH (n)-[r]-(m)
      RETURN n, r, m
    `);
  }

  const cypher = cypherParts.join('\n');

  return executeQuery(cypher, params);
}

/**
 * Get graph statistics (node counts by type)
 * Uses retry logic for transient failures and timeout for long-running queries
 */
export async function fetchGraphStats(): Promise<Record<string, number>> {
  return withRetry(async () => {
    const cypher = `
      CALL db.labels() YIELD label
      CALL {
        WITH label
        MATCH (n)
        WHERE label IN labels(n)
        RETURN count(n) AS count
      }
      RETURN label, count
      ORDER BY count DESC
    `;

    const d = getDriver();
    const session: Session = d.session({ defaultAccessMode: neo4j.session.READ });

    try {
      const result = await session.run(cypher, {}, { timeout: QUERY_TIMEOUT });
      const stats: Record<string, number> = {};

      for (const record of result.records) {
        const label = record.get('label') as string;
        const count = toNumber(record.get('count'));
        stats[label] = count;
      }

      return stats;
    } finally {
      await session.close();
    }
  }, 'fetchGraphStats');
}

/**
 * Fetch neighbors of a specific node (for expansion)
 * Used by double-click expand feature (Neo4j Browser style)
 *
 * @param nodeId - The elementId of the node to expand
 * @param limit - Maximum number of neighbors to return (default: 50)
 * @returns Promise<QueryResult> - Neighboring nodes and connecting edges
 */
export async function fetchNodeNeighbors(
  nodeId: string,
  limit: number = EXPAND_QUERY_LIMIT
): Promise<QueryResult> {
  const cypher = `
    MATCH (n)-[r]-(m)
    WHERE elementId(n) = $nodeId
    RETURN n, r, m
    LIMIT $limit
  `;

  return executeQuery(cypher, {
    nodeId,
    limit: neo4j.int(limit),
  });
}

/**
 * Execute a custom Cypher query (from AI chat or UI interactions)
 * No automatic LIMIT is added - the caller controls the query fully.
 */
export async function executeCustomQuery(
  cypher: string,
  params: Record<string, unknown> = {}
): Promise<QueryResult> {
  // Security: Only allow read queries
  // Block write operations and potentially dangerous procedures
  const normalizedCypher = cypher.trim().toUpperCase();
  const blockedKeywords = [
    // Write operations
    'CREATE', 'MERGE', 'SET', 'DELETE', 'REMOVE', 'DETACH',
    // Procedures (APOC, etc. can modify data)
    'CALL ',
    // File operations
    'LOAD CSV',
    // Loop constructs (can contain writes)
    'FOREACH',
    // Schema modifications
    'DROP', 'INDEX', 'CONSTRAINT',
  ];

  if (blockedKeywords.some((keyword) => normalizedCypher.includes(keyword))) {
    throw new Error('Write operations and procedures are not allowed. Use read-only queries.');
  }

  return executeQuery(cypher, params, { timeout: QUERY_TIMEOUT });
}
