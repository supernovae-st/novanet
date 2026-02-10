// apps/studio/src/app/api/views/route.ts
import { NextResponse } from 'next/server';
import { ViewLoader } from '@novanet/core/filters';
import type { ViewRegistry, ViewCategoryGroup } from '@novanet/core/filters';
import { CATEGORY_NAMES, CATEGORY_ORDER } from '@/config/viewCategories';

// Cache registry in memory (static between requests)
// Note: Cache is lost on serverless cold starts (Vercel, Lambda)
let registryCache: ViewRegistry | null = null;

/**
 * GET /api/views
 * Returns the view registry grouped by category.
 */
export async function GET() {
  try {
    // Load registry (cached after first request)
    if (!registryCache) {
      registryCache = await ViewLoader.loadRegistry();
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
