// apps/studio/src/app/api/views/route.ts
import { NextResponse } from 'next/server';
import { ViewLoader } from '@novanet/core/filters';
import type { ViewRegistry, ViewCategory, ViewCategoryGroup } from '@novanet/core/filters';
import path from 'path';

// Path to YAML views in core package
const viewsDir = path.resolve(process.cwd(), '../../packages/core/models/views');

// Cache registry in memory (static between requests)
let registryCache: ViewRegistry | null = null;

// Category display names
const CATEGORY_NAMES: Record<ViewCategory, string> = {
  scope: 'Scope Layers',
  generation: 'Generation',
  knowledge: 'Knowledge',
  project: 'Project',
  mining: 'Mining',
};

// Category order for display
const CATEGORY_ORDER: ViewCategory[] = ['scope', 'generation', 'knowledge', 'project', 'mining'];

/**
 * GET /api/views
 * Returns the view registry grouped by category.
 */
export async function GET() {
  try {
    // Load registry (cached after first request)
    if (!registryCache) {
      registryCache = await ViewLoader.loadRegistry(viewsDir);
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
