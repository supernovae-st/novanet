// apps/studio/src/app/api/views/[id]/route.ts
// v0.12.5: Unified view system - all views from views.yaml (single source of truth)
import { NextRequest, NextResponse } from 'next/server';
import { ViewLoader } from '@novanet/core/filters';

// ============================================================================
// VALIDATION
// ============================================================================

// Valid view ID pattern (alphanumeric + hyphens, no path traversal)
const VIEW_ID_REGEX = /^[a-z0-9-]+$/;

// Valid locale pattern (BCP 47 - permissive to support variants like zh-Hans-CN)
const LOCALE_REGEX = /^[a-z]{2,3}(-[A-Za-z]{2,4})?(-[A-Z]{2})?(-[a-z0-9]+)*$/;

// View ID aliases for backward compatibility (v0.12.5)
const VIEW_ID_ALIASES: Record<string, string> = {
  'complete-graph': 'schema-complete',
  'block-generation': 'gen-block',
  'page-generation': 'gen-page',
  'gen-context': 'gen-page',
  'data-locales': 'data-locales',
  'data-project': 'data-project',
  'ctx-neighbors': 'ctx-neighbors',
  'ctx-entity': 'ctx-entity',
};

interface ViewParams {
  key?: string;
  locale?: string;
  project?: string;
  nodeKey?: string;
  realm?: string;
  kind?: string;
  arcKind?: string;
}

/**
 * GET /api/views/:id
 * Loads a single view from _registry.yaml and returns Cypher query.
 *
 * Query parameters:
 * - key: Root node key (alias for nodeKey)
 * - nodeKey: Root node key for contextual views
 * - locale: BCP 47 locale code (e.g., 'fr-FR')
 * - project: Project key
 * - realm: Realm filter (shared | org)
 * - kind: Node kind name (for overlay-kind-instances)
 * - arcKind: Arc kind name (for overlay-arc-analysis)
 */
export async function GET(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  const { id: rawId } = await params;

  // Validate view ID (security: prevent directory traversal)
  if (!VIEW_ID_REGEX.test(rawId) || rawId.includes('..') || rawId.includes('/') || rawId.includes('\\')) {
    return NextResponse.json(
      { success: false, error: 'Invalid view ID format' },
      { status: 400 }
    );
  }

  // Apply alias for standardized view IDs
  const id = VIEW_ID_ALIASES[rawId] || rawId;

  try {
    // Extract query parameters
    const searchParams = request.nextUrl.searchParams;
    const viewParams: ViewParams = {
      key: searchParams.get('key') || undefined,
      nodeKey: searchParams.get('nodeKey') || undefined,
      locale: searchParams.get('locale') || undefined,
      project: searchParams.get('project') || undefined,
      realm: searchParams.get('realm') || undefined,
      kind: searchParams.get('kind') || undefined,
      arcKind: searchParams.get('arcKind') || undefined,
    };

    // Validate locale format if provided
    if (viewParams.locale && !LOCALE_REGEX.test(viewParams.locale)) {
      return NextResponse.json(
        { success: false, error: 'Invalid locale format. Expected BCP 47 (e.g., fr-FR)' },
        { status: 400 }
      );
    }

    // Load view from unified _registry.yaml
    const view = await ViewLoader.getViewById(id);
    if (!view) {
      return NextResponse.json(
        { success: false, error: `View '${id}' not found` },
        { status: 404 }
      );
    }

    // Build Cypher params from view params
    const cypherParams: Record<string, unknown> = {};

    // nodeKey (used by contextual views)
    if (viewParams.key || viewParams.nodeKey) {
      cypherParams.nodeKey = viewParams.key || viewParams.nodeKey;
    }

    // Other params
    if (viewParams.locale) cypherParams.locale = viewParams.locale;
    if (viewParams.project) cypherParams.project = viewParams.project;
    if (viewParams.realm) cypherParams.realm = viewParams.realm;
    if (viewParams.kind) cypherParams.kind = viewParams.kind;
    if (viewParams.arcKind) cypherParams.arcKind = viewParams.arcKind;

    // Get Cypher from view (embedded in _registry.yaml)
    if (!view.cypher) {
      return NextResponse.json(
        { success: false, error: `View '${id}' has no Cypher query` },
        { status: 500 }
      );
    }

    return NextResponse.json({
      success: true,
      data: {
        view: {
          id: view.id,
          name: view.name,
          description: view.description,
          category: view.category,
          icon: view.icon,
          color: view.color,
          rootType: view.root_type,
          contextual: view.contextual || false,
          applicableTypes: view.applicable_types || [],
        },
        cypher: {
          query: view.cypher.trim(),
          params: cypherParams,
        },
        params: viewParams,
      },
    });
  } catch (error: unknown) {
    console.error(`Failed to load view '${id}':`, error);

    return NextResponse.json(
      {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to load view',
      },
      { status: 500 }
    );
  }
}
