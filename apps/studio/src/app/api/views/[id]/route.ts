// apps/studio/src/app/api/views/[id]/route.ts
import { NextRequest, NextResponse } from 'next/server';
import { ViewLoader, CypherGenerator } from '@novanet/core/filters';

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
}

/**
 * GET /api/views/:id
 * Loads a single view definition and generates Cypher query.
 *
 * Query parameters:
 * - key: Root node key (optional, depends on view)
 * - locale: BCP 47 locale code (optional, e.g., 'fr-FR')
 * - project: Project key (optional)
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
    };

    // Validate locale format if provided
    if (viewParams.locale && !LOCALE_REGEX.test(viewParams.locale)) {
      return NextResponse.json(
        { success: false, error: 'Invalid locale format. Expected BCP 47 (e.g., fr-FR)' },
        { status: 400 }
      );
    }

    // Load view definition
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
