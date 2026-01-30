'use client';

/**
 * Divider - Consistent vertical/horizontal separator
 *
 * Replaces repeated <div className="w-px h-5 bg-white/10" /> patterns
 */

import { memo } from 'react';
import { cn } from '@/lib/utils';

interface DividerProps {
  /** Direction of the divider */
  orientation?: 'vertical' | 'horizontal';
  /** Height for vertical dividers */
  height?: 'sm' | 'md' | 'lg';
  /** Additional CSS classes */
  className?: string;
}

export const Divider = memo(function Divider({
  orientation = 'vertical',
  height = 'md',
  className,
}: DividerProps) {
  const heightStyles = {
    sm: 'h-4',
    md: 'h-5',
    lg: 'h-6',
  };

  return (
    <div
      className={cn(
        'bg-white/10 shrink-0',
        orientation === 'vertical'
          ? cn('w-px', heightStyles[height])
          : 'h-px w-full my-1',
        className
      )}
    />
  );
});
