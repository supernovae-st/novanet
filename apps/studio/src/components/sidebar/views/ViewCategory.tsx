'use client';

/**
 * ViewCategory - Section header for a group of views
 *
 * Displays category name with icon and optional description.
 * Groups related views in the ViewSelector.
 */

import { memo } from 'react';
import {
  Crosshair,
  Sparkles,
  BookOpen,
  FolderKanban,
  Pickaxe,
  Grid3x3,
  type LucideIcon,
} from 'lucide-react';
import { cn } from '@/lib/utils';
import type { ViewCategory as ViewCategoryType } from '@novanet/core/filters';

// Map category IDs to icons and display names
const CATEGORY_CONFIG: Record<
  ViewCategoryType,
  { icon: LucideIcon; label: string; color: string }
> = {
  scope: {
    icon: Crosshair,
    label: 'Scope',
    color: 'text-emerald-400',
  },
  generation: {
    icon: Sparkles,
    label: 'Generation',
    color: 'text-amber-400',
  },
  knowledge: {
    icon: BookOpen,
    label: 'Knowledge',
    color: 'text-violet-400',
  },
  project: {
    icon: FolderKanban,
    label: 'Project',
    color: 'text-blue-400',
  },
  mining: {
    icon: Pickaxe,
    label: 'Mining',
    color: 'text-rose-400',
  },
};

interface ViewCategorySectionProps {
  categoryId: ViewCategoryType;
  viewCount?: number;
  className?: string;
  children: React.ReactNode;
}

export const ViewCategorySection = memo(function ViewCategorySection({
  categoryId,
  viewCount,
  className,
  children,
}: ViewCategorySectionProps) {
  const config = CATEGORY_CONFIG[categoryId] || {
    icon: Grid3x3,
    label: categoryId,
    color: 'text-white/60',
  };
  const Icon = config.icon;

  return (
    <div className={cn('space-y-2', className)}>
      {/* Category Header */}
      <div className="flex items-center gap-2 px-1">
        <Icon className={cn('w-3.5 h-3.5', config.color)} />
        <span className="text-[10px] uppercase tracking-wider font-medium text-white/40">
          {config.label}
        </span>
        {viewCount !== undefined && (
          <span className="text-[9px] text-white/25">({viewCount})</span>
        )}
      </div>

      {/* View Cards Grid */}
      <div className="grid grid-cols-3 gap-1">{children}</div>
    </div>
  );
});
