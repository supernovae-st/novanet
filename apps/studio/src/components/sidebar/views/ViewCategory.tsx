'use client';

/**
 * ViewCategory - Section header for a group of views
 *
 * Displays category name with icon and optional description.
 * Groups related views in the ViewSelector.
 *
 * Design: Consistent spacing, proper grid gap
 */

import { memo } from 'react';
import { Grid3x3 } from 'lucide-react';
import { cn } from '@/lib/utils';
import { iconSizes } from '@/design/tokens';
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
    <div className={cn('space-y-3', className)}>
      {/* Category Header */}
      <div className="flex items-center gap-2.5">
        <Icon className={cn(iconSizes.md, color)} />
        <span className="text-[11px] uppercase tracking-wider font-semibold text-white/45">
          {label}
        </span>
        {viewCount !== undefined && (
          <span className="text-[10px] tabular-nums text-white/25">({viewCount})</span>
        )}
      </div>

      {/* View Cards - single column for horizontal layout */}
      <div className="flex flex-col gap-2">{children}</div>
    </div>
  );
});
