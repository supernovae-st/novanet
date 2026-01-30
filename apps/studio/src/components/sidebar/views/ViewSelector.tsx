'use client';

/**
 * ViewSelector - YAML Views selector with category grouping
 *
 * Replaces the hardcoded ViewPresetSelector with dynamic views
 * loaded from @novanet/core via the API.
 *
 * Features:
 * - Grid layout grouped by category (scope, generation, knowledge, project, mining)
 * - Keyboard shortcuts 1-9 for quick access
 * - URL state sync via viewStore
 * - Loading and error states
 *
 * @see /api/views for the registry endpoint
 * @see useViewStore for state management
 */

import { memo, useCallback, useEffect, useMemo } from 'react';
import { useShallow } from 'zustand/react/shallow';
import {
  Grid3x3,
  Loader2,
  AlertCircle,
  RefreshCw,
  Sparkles,
  BookOpen,
  FolderKanban,
  Pickaxe,
  Eye,
  type LucideIcon,
} from 'lucide-react';
import { cn } from '@/lib/utils';
import { useViewStore, type ViewParams } from '@/stores/viewStore';
import type { ViewRegistryEntry } from '@novanet/core/filters';
import { ViewCard } from './ViewCard';
import { ViewCategorySection } from './ViewCategory';

// Map view IDs to icons (fallback to Eye for unknown views)
const VIEW_ICONS: Record<string, LucideIcon> = {
  'complete-graph': Grid3x3,
  'block-generation': Sparkles,
  'concept-generation': Sparkles,
  'page-generation': Sparkles,
  'knowledge-graph': BookOpen,
  'locale-knowledge': BookOpen,
  'seo-mining': Pickaxe,
  'geo-mining': Pickaxe,
  'project-scope': FolderKanban,
  'project-overview': FolderKanban,
};

// Keyboard shortcut mapping (1-9 keys map to first 9 views in order)
const SHORTCUT_KEYS = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

interface ViewSelectorProps {
  className?: string;
  onSelect?: (viewId: string, params?: ViewParams) => void;
}

export const ViewSelector = memo(function ViewSelector({
  className,
  onSelect,
}: ViewSelectorProps) {
  const {
    categories,
    activeViewId,
    loading,
    executing,
    error,
    loadRegistry,
    executeView,
  } = useViewStore(
    useShallow((state) => ({
      categories: state.categories,
      activeViewId: state.activeViewId,
      loading: state.loading,
      executing: state.executing,
      error: state.error,
      loadRegistry: state.loadRegistry,
      executeView: state.executeView,
    }))
  );

  // Load registry on mount
  useEffect(() => {
    loadRegistry();
  }, [loadRegistry]);

  // Handle view selection - executes the view's Cypher query
  const handleSelect = useCallback(
    (view: ViewRegistryEntry) => {
      executeView(view.id);
      onSelect?.(view.id);
    },
    [executeView, onSelect]
  );

  // Flatten views for shortcut indexing (memoized for performance)
  const allViews = useMemo(
    () => categories.flatMap((cat) => cat.views),
    [categories]
  );

  // Register keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Ignore if typing in an input
      if (
        e.target instanceof HTMLInputElement ||
        e.target instanceof HTMLTextAreaElement
      ) {
        return;
      }

      const index = SHORTCUT_KEYS.indexOf(e.key);
      if (index !== -1 && index < allViews.length) {
        e.preventDefault();
        handleSelect(allViews[index]);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [allViews, handleSelect]);

  // Get shortcut for a view (based on its position in flattened list)
  const getShortcut = (viewId: string): string | undefined => {
    const index = allViews.findIndex((v) => v.id === viewId);
    return index !== -1 && index < SHORTCUT_KEYS.length
      ? SHORTCUT_KEYS[index]
      : undefined;
  };

  // Get icon for a view
  const getIcon = (viewId: string): LucideIcon => {
    return VIEW_ICONS[viewId] || Eye;
  };

  // Loading state
  if (loading && categories.length === 0) {
    return (
      <div className={cn('space-y-1', className)}>
        <div className="flex items-center gap-2 px-3 py-2 text-xs text-white/40">
          <Loader2 className="w-3.5 h-3.5 animate-spin" />
          <span className="uppercase tracking-wider font-medium">Loading Views...</span>
        </div>
      </div>
    );
  }

  // Error state
  if (error) {
    return (
      <div className={cn('space-y-2 px-3 py-2', className)}>
        <div className="flex items-center gap-2 text-xs text-red-400">
          <AlertCircle className="w-3.5 h-3.5" />
          <span>Failed to load views</span>
        </div>
        <button
          onClick={() => loadRegistry()}
          className="flex items-center gap-1.5 text-[10px] text-white/40 hover:text-white/60 transition-colors"
        >
          <RefreshCw className="w-3 h-3" />
          Retry
        </button>
      </div>
    );
  }

  // Empty state (no views in registry)
  if (categories.length === 0) {
    return (
      <div className={cn('space-y-1', className)}>
        <div className="flex items-center gap-2 px-3 py-2 text-xs text-white/40">
          <Grid3x3 className="w-3.5 h-3.5" />
          <span className="uppercase tracking-wider font-medium">No Views Available</span>
        </div>
      </div>
    );
  }

  return (
    <div className={cn('space-y-4', className)}>
      {/* Header */}
      <div className="flex items-center gap-2 px-3 py-2 text-xs text-white/40">
        {executing ? (
          <Loader2 className="w-3.5 h-3.5 animate-spin text-novanet-400" />
        ) : (
          <Grid3x3 className="w-3.5 h-3.5" />
        )}
        <span className="uppercase tracking-wider font-medium">YAML Views</span>
        <span className="text-white/25 ml-auto">{allViews.length} views</span>
      </div>

      {/* Categories */}
      <div className="space-y-4 px-2">
        {categories.map((category) => (
          <ViewCategorySection
            key={category.id}
            categoryId={category.id}
            viewCount={category.views.length}
          >
            {category.views.map((view) => (
              <ViewCard
                key={view.id}
                id={view.id}
                name={view.description || view.id}
                description={view.description}
                shortcut={getShortcut(view.id)}
                icon={getIcon(view.id)}
                isActive={activeViewId === view.id}
                onClick={() => handleSelect(view)}
              />
            ))}
          </ViewCategorySection>
        ))}
      </div>
    </div>
  );
});
