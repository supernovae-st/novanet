'use client';

/**
 * ViewCategory - Section header for a group of views
 *
 * Displays category name with icon and optional description.
 * Groups related views in the ViewSelector.
 */

import { memo } from 'react';
import { Grid3x3 } from 'lucide-react';
import { cn } from '@/lib/utils';
import type { ViewCategory as ViewCategoryType } from '@novanet/core/filters';
import { VIEW_CATEGORIES } from '@/config/viewCategories';

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
  const config = VIEW_CATEGORIES[categoryId];
  const Icon = config?.icon || Grid3x3;
  const label = config?.label || categoryId;
  const color = config?.color || 'text-white/60';

  return (
    <div className={cn('space-y-2', className)}>
      {/* Category Header */}
      <div className="flex items-center gap-2 px-1">
        <Icon className={cn('w-3.5 h-3.5', color)} />
        <span className="text-[10px] uppercase tracking-wider font-medium text-white/40">
          {label}
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
