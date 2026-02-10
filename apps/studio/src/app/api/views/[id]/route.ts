// apps/studio/src/app/api/views/[id]/route.ts
import { NextRequest, NextResponse } from 'next/server';
import { ViewLoader, CypherGenerator } from '@novanet/core/filters';
import { promises as fs } from 'fs';
import path from 'path';
import yaml from 'js-yaml';

// ============================================================================
// VALIDATION
// ============================================================================

// Valid view ID pattern (alphanumeric + hyphens, no path traversal)
const VIEW_ID_REGEX = /^[a-z0-9-]+$/;

// Valid locale pattern (BCP 47 - permissive to support variants like zh-Hans-CN)
const LOCALE_REGEX = /^[a-z]{2,3}(-[A-Za-z]{2,4})?(-[A-Z]{2})?(-[a-z0-9]+)*$/;

interface ViewParams {
  key?: string;
  locale?: string;
  project?: string;
  nodeKey?: string;
}

// ============================================================================
// CONTEXTUAL VIEW TYPES
// ============================================================================

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

// ============================================================================
// HELPERS
// ============================================================================

/**
 * Try to load a contextual view from YAML.
 * Contextual views have direct Cypher templates instead of declarative includes.
 */
async function loadContextualView(viewId: string): Promise<ContextualViewDef | null> {
  // Look for contextual view in packages/core/models/views/contextual/
  const contextualPath = path.join(
    process.cwd(),
    '../../packages/core/models/views/contextual',
    `${viewId}.yaml`
  );

  try {
    const content = await fs.readFile(contextualPath, 'utf-8');
    const data = yaml.load(content) as ContextualViewDef;

    // Validate it's a contextual view
    if (data && data.contextual === true && data.cypher) {
      return data;
    }
    return null;
  } catch {
    // File doesn't exist or parse error - not a contextual view
    return null;
  }
}

/**
 * Substitute parameters in a Cypher template.
 */
function substituteCypherParams(
  cypher: string,
  params: ViewParams,
  viewParams?: ContextualViewParam[]
): { query: string; params: Record<string, unknown> } {
  // Build params object from view params
  const cypherParams: Record<string, unknown> = {};

  if (viewParams) {
    for (const p of viewParams) {
      if (p.name === 'nodeKey' && (params.key || params.nodeKey)) {
        cypherParams.nodeKey = params.key || params.nodeKey;
      } else if (p.name === 'locale' && params.locale) {
        cypherParams.locale = params.locale;
      } else if (p.name === 'project' && params.project) {
        cypherParams.project = params.project;
      }
    }
  } else {
    // Default param mapping for contextual views
    if (params.key || params.nodeKey) {
      cypherParams.nodeKey = params.key || params.nodeKey;
    }
    if (params.locale) {
      cypherParams.locale = params.locale;
    }
    if (params.project) {
      cypherParams.project = params.project;
    }
  }

  return {
    query: cypher.trim(),
    params: cypherParams,
  };
}

/**
 * GET /api/views/:id
 * Loads a single view definition and generates Cypher query.
 *
 * Query parameters:
 * - key: Root node key (optional, depends on view)
 * - locale: BCP 47 locale code (optional, e.g., 'fr-FR')
 * - project: Project key (optional)
 * - nodeKey: Alias for key (used by contextual views)
 */
export async function GET(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  const { id } = await params;

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
    const viewParams: ViewParams = {
      key: searchParams.get('key') || undefined,
      locale: searchParams.get('locale') || undefined,
      project: searchParams.get('project') || undefined,
      nodeKey: searchParams.get('nodeKey') || undefined,
    };

    // Validate locale format if provided
    if (viewParams.locale && !LOCALE_REGEX.test(viewParams.locale)) {
      return NextResponse.json(
        { success: false, error: 'Invalid locale format. Expected BCP 47 (e.g., fr-FR)' },
        { status: 400 }
      );
    }

    // First, try to load as a contextual view (direct Cypher template)
    const contextualView = await loadContextualView(id);
    if (contextualView) {
      const cypher = substituteCypherParams(
        contextualView.cypher,
        viewParams,
        contextualView.params
      );

      return NextResponse.json({
        success: true,
        data: {
          view: {
            id: contextualView.id,
            name: contextualView.id.replace(/-/g, ' ').replace(/\b\w/g, c => c.toUpperCase()),
            description: contextualView.description,
            contextual: true,
            applicableTypes: contextualView.applicable_types,
          },
          cypher,
          params: viewParams,
        },
      });
    }

    // Fall back to declarative view (uses NovaNetFilter + CypherGenerator)
    const view = await ViewLoader.loadView(id);

    // Convert to filter and generate Cypher
    const filter = ViewLoader.toFilter(view, viewParams);
    const cypher = CypherGenerator.generate(filter);

    return NextResponse.json({
      success: true,
      data: {
        view,
        cypher,
        params: viewParams,
      },
    });
  } catch (error: unknown) {
    console.error(`Failed to load view '${id}':`, error);

    // Handle file not found
    if (error && typeof error === 'object' && 'code' in error && error.code === 'ENOENT') {
      return NextResponse.json(
        { success: false, error: `View '${id}' not found` },
        { status: 404 }
      );
    }

    return NextResponse.json(
      {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to load view',
      },
      { status: 500 }
    );
  }
}
