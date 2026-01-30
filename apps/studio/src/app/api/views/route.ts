// apps/studio/src/app/api/views/route.ts
import { NextResponse } from 'next/server';
import { ViewLoader } from '@novanet/core/filters';
import type { ViewRegistry, ViewCategoryGroup } from '@novanet/core/filters';
import path from 'path';
import fs from 'fs';

// ============================================================================
// PATH RESOLUTION (works in dev + production + monorepo)
// ============================================================================

function resolveViewsDir(): string {
  // Priority 1: Environment variable (for production/custom deployments)
  if (process.env.NOVANET_VIEWS_DIR) {
    return process.env.NOVANET_VIEWS_DIR;
  }

  // Priority 2: Monorepo structure detection
  const candidates = [
    // From apps/studio (dev)
    path.resolve(process.cwd(), '../../packages/core/models/views'),
    // From monorepo root
    path.resolve(process.cwd(), 'packages/core/models/views'),
    // From worktree
    path.resolve(process.cwd(), '../packages/core/models/views'),
  ];

  for (const candidate of candidates) {
    if (fs.existsSync(path.join(candidate, '_registry.yaml'))) {
      return candidate;
    }
  }

  // Fallback (will error but with clear message)
  throw new Error(
    'Could not locate views directory. Set NOVANET_VIEWS_DIR environment variable.'
  );
}

// Lazy-initialized path (computed once on first request)
let viewsDir: string | null = null;
function getViewsDir(): string {
  if (!viewsDir) {
    viewsDir = resolveViewsDir();
  }
  return viewsDir;
}

// Cache registry in memory (static between requests)
// Note: Cache is lost on serverless cold starts (Vercel, Lambda)
let registryCache: ViewRegistry | null = null;

// Import centralized category configuration
import { CATEGORY_NAMES, CATEGORY_ORDER } from '@/config/viewCategories';

/**
 * GET /api/views
 * Returns the view registry grouped by category.
 */
export async function GET() {
  try {
    // Load registry (cached after first request)
    if (!registryCache) {
      registryCache = await ViewLoader.loadRegistry(getViewsDir());
    }

    // Group views by category
    const categories: ViewCategoryGroup[] = CATEGORY_ORDER.map(categoryId => ({
      id: categoryId,
      name: CATEGORY_NAMES[categoryId],
      views: registryCache!.views.filter(v => v.category === categoryId),
    }));

    return NextResponse.json({
      success: true,
      data: {
        registry: registryCache,
        categories,
      },
    });
  } catch (error) {
    console.error('Failed to load view registry:', error);
    return NextResponse.json(
      {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to load view registry',
      },
      { status: 500 }
    );
  }
}
