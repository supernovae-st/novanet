/**
 * Context View Data API Route
 *
 * Executes view Cypher queries and returns graph data (nodes + edges).
 * Used by Context Views Action Cards in the TabbedDetailPanel footer.
 *
 * v11.6: Supports YAML-based contextual views with direct Cypher templates
 *
 * @example GET /api/views/composition/data?nodeId=xxx&nodeKey=page:homepage
 */

import { NextRequest, NextResponse } from 'next/server';
import { promises as fs } from 'fs';
import path from 'path';
import yaml from 'js-yaml';
import { executeQuery } from '@/lib/neo4j';
import { getViewQuery, getViewStatsQuery } from '@/lib/cypher/viewQueries';
import { VIEW_TYPES, type ViewId } from '@/config/viewTypes';
import { logger } from '@/lib/logger';

// =============================================================================
// YAML CONTEXTUAL VIEW TYPES
// =============================================================================

interface ContextualViewParam {
  name: string;
  type: string;
  required?: boolean;
  description?: string;
}

interface ContextualViewDef {
  id: string;
  description: string;
  category: string;
  contextual: boolean;
  applicable_types: string[];
  modes: string[];
  cypher: string;
  params?: ContextualViewParam[];
}

/**
 * Try to load a contextual view from YAML.
 */
async function loadContextualView(viewId: string): Promise<ContextualViewDef | null> {
  const contextualPath = path.join(
    process.cwd(),
    '../../packages/core/models/views/contextual',
    `${viewId}.yaml`
  );

  try {
    const content = await fs.readFile(contextualPath, 'utf-8');
    const data = yaml.load(content) as ContextualViewDef;

    if (data && data.contextual === true && data.cypher) {
      return data;
    }
    return null;
  } catch {
    return null;
  }
}

// =============================================================================
// VALIDATION
// =============================================================================

// Valid view ID pattern (alphanumeric + hyphens, no path traversal)
const VIEW_ID_REGEX = /^[a-z0-9-]+$/;

// Valid node key pattern (alphanumeric + hyphens + colons + @ for composite keys)
const NODE_KEY_REGEX = /^[a-zA-Z0-9_:@.-]+$/;

/**
 * GET /api/views/:id/data
 * Executes view Cypher query and returns graph data.
 *
 * Query parameters:
 * - nodeId: Neo4j elementId of the root node (required)
 * - nodeKey: Key of the root node (required)
 * - nodeType: Type of the root node (required)
 * - limit: Maximum nodes to return (optional, default 50)
 * - statsOnly: If true, return only stats without full data (optional)
 */
export async function GET(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  const { id } = await params;
  const startTime = Date.now();

  // Validate view ID (security: prevent directory traversal)
  if (!VIEW_ID_REGEX.test(id) || id.includes('..') || id.includes('/') || id.includes('\\')) {
    return NextResponse.json(
      { success: false, error: 'Invalid view ID format' },
      { status: 400 }
    );
  }

  try {
    // Extract query parameters
    const searchParams = request.nextUrl.searchParams;
    const nodeId = searchParams.get('nodeId');
    const nodeKey = searchParams.get('nodeKey');
    const nodeType = searchParams.get('nodeType');
    const limitParam = searchParams.get('limit');
    const statsOnly = searchParams.get('statsOnly') === 'true';

    // Validate required parameters
    if (!nodeId || !nodeKey || !nodeType) {
      return NextResponse.json(
        {
          success: false,
          error: 'Missing required parameters: nodeId, nodeKey, and nodeType are required',
        },
        { status: 400 }
      );
    }

    // Validate node key format
    if (!NODE_KEY_REGEX.test(nodeKey)) {
      return NextResponse.json(
        { success: false, error: 'Invalid nodeKey format' },
        { status: 400 }
      );
    }

    // Parse limit (default 50, max 200)
    const limit = Math.min(Math.max(parseInt(limitParam || '50', 10) || 50, 1), 200);

    const viewId = id as ViewId;

    // First, try to load YAML contextual view
    const contextualView = await loadContextualView(id);

    if (contextualView) {
      // Use YAML Cypher template
      const cypherParams: Record<string, unknown> = {
        nodeKey,
        nodeId,
        nodeType,
        limit,
      };

      logger.info('API', `/views/${id}/data (YAML contextual)`, {
        nodeKey,
        description: contextualView.description,
      });

      const result = await executeQuery(contextualView.cypher, cypherParams);

      // Get view configuration for visual styling
      const viewConfig = VIEW_TYPES[viewId];

      if (statsOnly) {
        return NextResponse.json({
          success: true,
          data: {
            stats: {
              nodeCount: result.totalNodes,
              arcCount: result.totalArcs,
            },
          },
          meta: {
            viewId,
            queryDuration: result.duration,
            requestDuration: Date.now() - startTime,
          },
        });
      }

      return NextResponse.json({
        success: true,
        data: {
          nodes: result.nodes,
          edges: result.edges,
          view: {
            id: viewId,
            label: viewConfig?.label || contextualView.id,
            style: viewConfig?.style || 'flow',
            effect: viewConfig?.effect || 'border-beam',
            transitionColor: viewConfig?.transitionColor || '#06b6d4',
          },
        },
        meta: {
          totalNodes: result.totalNodes,
          totalArcs: result.totalArcs,
          queryDuration: result.duration,
          requestDuration: Date.now() - startTime,
          description: contextualView.description,
        },
      });
    }

    // Check if view exists in our configuration (for non-contextual views)
    if (!VIEW_TYPES[viewId]) {
      return NextResponse.json(
        { success: false, error: `View '${id}' not found` },
        { status: 404 }
      );
    }

    // Build query params for hardcoded queries
    const queryParams = {
      nodeId,
      nodeKey,
      nodeType,
      limit,
    };

    if (statsOnly) {
      // Return only stats (lighter weight)
      const { cypher, params: cypherParams, description } = getViewStatsQuery(viewId, queryParams);

      logger.info('API', `/views/${id}/data (stats)`, {
        nodeKey,
        description,
      });

      const result = await executeQuery(cypher, cypherParams);

      return NextResponse.json({
        success: true,
        data: {
          stats: {
            nodeCount: result.totalNodes,
            arcCount: result.totalArcs,
          },
        },
        meta: {
          viewId,
          queryDuration: result.duration,
          requestDuration: Date.now() - startTime,
        },
      });
    }

    // Full data query (fallback to hardcoded queries)
    const { cypher, params: cypherParams, description } = getViewQuery(viewId, queryParams);

    logger.info('API', `/views/${id}/data`, {
      nodeKey,
      description,
      limit,
    });

    const result = await executeQuery(cypher, cypherParams);

    // Get view configuration for response
    const viewConfig = VIEW_TYPES[viewId];

    return NextResponse.json({
      success: true,
      data: {
        nodes: result.nodes,
        edges: result.edges,
        view: {
          id: viewId,
          label: viewConfig.label,
          style: viewConfig.style,
          effect: viewConfig.effect,
          transitionColor: viewConfig.transitionColor,
        },
      },
      meta: {
        totalNodes: result.totalNodes,
        totalArcs: result.totalArcs,
        queryDuration: result.duration,
        requestDuration: Date.now() - startTime,
        description,
      },
    });
  } catch (error) {
    logger.error('API', `/views/${id}/data error`, error);

    const errorMessage = error instanceof Error ? error.message : 'Unknown error';

    // Determine error type for appropriate status code
    let status = 500;
    if (errorMessage.includes('syntax') || errorMessage.includes('Invalid')) {
      status = 400;
    } else if (errorMessage.includes('connection') || errorMessage.includes('ECONNREFUSED')) {
      status = 503;
    }

    return NextResponse.json(
      {
        success: false,
        error: errorMessage,
        details: process.env.NODE_ENV === 'development' ? String(error) : undefined,
      },
      { status }
    );
  }
}
