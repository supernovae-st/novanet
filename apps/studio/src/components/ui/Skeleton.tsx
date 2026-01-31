/**
 * Skeleton Loading Components
 *
 * Placeholder components for loading states.
 * Used throughout the app for consistent loading UX.
 */

import { cn } from '@/lib/utils';
import { iconSizes, gapTokens } from '@/design/tokens';

// =============================================================================
// Base Skeleton
// =============================================================================

interface SkeletonProps {
  className?: string;
}

export function Skeleton({ className }: SkeletonProps) {
  return (
    <div
      className={cn(
        'animate-pulse rounded-md bg-white/10',
        className
      )}
    />
  );
}

// =============================================================================
// Text Skeleton
// =============================================================================

interface SkeletonTextProps {
  lines?: number;
  className?: string;
}

export function SkeletonText({ lines = 3, className }: SkeletonTextProps) {
  return (
    <div className={cn('space-y-2', className)}>
      {Array.from({ length: lines }).map((_, i) => (
        <Skeleton
          key={i}
          className={cn(
            'h-4',
            i === lines - 1 ? 'w-3/4' : 'w-full'
          )}
        />
      ))}
    </div>
  );
}

// =============================================================================
// Circle Skeleton (for avatars)
// =============================================================================

interface SkeletonCircleProps {
  size?: 'sm' | 'md' | 'lg';
  className?: string;
}

export function SkeletonCircle({ size = 'md', className }: SkeletonCircleProps) {
  const sizeClasses = {
    sm: 'w-8 h-8',
    md: 'w-12 h-12',
    lg: 'w-16 h-16',
  };

  return (
    <Skeleton
      className={cn(
        'rounded-full',
        sizeClasses[size],
        className
      )}
    />
  );
}

// =============================================================================
// Card Skeleton
// =============================================================================

interface SkeletonCardProps {
  className?: string;
}

export function SkeletonCard({ className }: SkeletonCardProps) {
  return (
    <div
      className={cn(
        'rounded-xl border border-white/10 bg-white/5 p-4 space-y-4',
        className
      )}
    >
      <div className={cn('flex items-center', gapTokens.spacious)}>
        <SkeletonCircle size="sm" />
        <div className="flex-1 space-y-2">
          <Skeleton className="h-4 w-1/3" />
          <Skeleton className="h-3 w-1/2" />
        </div>
      </div>
      <SkeletonText lines={2} />
    </div>
  );
}

// =============================================================================
// Shimmer Loader (for full-page loading)
// =============================================================================

interface ShimmerLoaderProps {
  className?: string;
}

export function ShimmerLoader({ className }: ShimmerLoaderProps) {
  return (
    <div
      className={cn(
        'relative overflow-hidden rounded-lg bg-white/5',
        className
      )}
    >
      <div
        className="absolute inset-0 -translate-x-full animate-[shimmer_2s_infinite] bg-gradient-to-r from-transparent via-white/10 to-transparent"
      />
    </div>
  );
}

// =============================================================================
// Node Skeleton (for graph nodes)
// =============================================================================

interface NodeSkeletonProps {
  className?: string;
}

export function NodeSkeleton({ className }: NodeSkeletonProps) {
  return (
    <div
      className={cn(
        'rounded-xl border border-white/10 bg-white/5 p-3 w-48',
        className
      )}
    >
      <div className={cn('flex items-center mb-2', gapTokens.default)}>
        <Skeleton className="w-6 h-6 rounded" />
        <Skeleton className="h-4 flex-1" />
      </div>
      <Skeleton className="h-3 w-2/3" />
    </div>
  );
}

// =============================================================================
// Panel Skeleton (for sidebar panels)
// =============================================================================

interface PanelSkeletonProps {
  className?: string;
}

export function PanelSkeleton({ className }: PanelSkeletonProps) {
  return (
    <div className={cn('space-y-4 p-4', className)}>
      {/* Header */}
      <div className={cn('flex items-center', gapTokens.spacious)}>
        <SkeletonCircle size="md" />
        <div className="flex-1 space-y-2">
          <Skeleton className="h-5 w-1/2" />
          <Skeleton className="h-3 w-3/4" />
        </div>
      </div>

      {/* Content sections */}
      <div className="space-y-3">
        <Skeleton className="h-4 w-1/4" />
        <SkeletonText lines={3} />
      </div>

      <div className="space-y-3">
        <Skeleton className="h-4 w-1/3" />
        <SkeletonText lines={2} />
      </div>
    </div>
  );
}
