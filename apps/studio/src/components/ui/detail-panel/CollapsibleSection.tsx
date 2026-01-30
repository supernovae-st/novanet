'use client';

/**
 * CollapsibleSection - Unified expandable section component
 *
 * Features:
 * - Smooth CSS grid animation
 * - Optional icon and badge count
 * - Consistent styling across detail panels
 */

import { memo, type ReactNode } from 'react';
import { cn } from '@/lib/utils';
import { NAV_ICONS } from '@/config/iconSystem';

const ChevronRight = NAV_ICONS.chevronRight;

export interface CollapsibleSectionProps {
  title: string;
  icon?: ReactNode;
  count?: number;
  isExpanded: boolean;
  onToggle: () => void;
  children: ReactNode;
  className?: string;
}

export const CollapsibleSection = memo(function CollapsibleSection({
  title,
  icon,
  count,
  isExpanded,
  onToggle,
  children,
  className,
}: CollapsibleSectionProps) {
  return (
    <div className={cn('border-b border-white/12', className)}>
      <button
        onClick={onToggle}
        aria-expanded={isExpanded}
        aria-label={`${isExpanded ? 'Collapse' : 'Expand'} ${title} section`}
        className="w-full flex items-center gap-2 px-5 py-3.5 hover:bg-white/6 transition-colors"
      >
        <ChevronRight
          className={cn(
            'w-4 h-4 text-white/60 transition-transform duration-200',
            isExpanded && 'rotate-90'
          )}
        />
        {icon && <span className="text-white/60">{icon}</span>}
        <span className="text-xs font-semibold text-white/85 uppercase tracking-wider">
          {title}
        </span>
        {count !== undefined && (
          <span className="ml-auto px-2 py-0.5 rounded-full text-[10px] font-medium bg-white/10 text-white/80">
            {count}
          </span>
        )}
      </button>

      {/* Smooth accordion with CSS grid animation */}
      <div
        className={cn(
          'grid transition-[grid-template-rows] duration-200 ease-out',
          isExpanded ? 'grid-rows-[1fr]' : 'grid-rows-[0fr]'
        )}
      >
        <div className="overflow-hidden">
          <div className="px-5 pb-4">{children}</div>
        </div>
      </div>
    </div>
  );
});
