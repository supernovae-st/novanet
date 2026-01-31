'use client';

/**
 * EmptyState - Unified empty/loading/error state component
 *
 * Replaces duplicated patterns across QueryResultsTabs, DatabaseInfoPanel,
 * LocalePicker, ProjectPicker, and other components.
 *
 * Design: Linear-dark style with icon box + title + description
 */

import { memo, type ReactNode, type ComponentType } from 'react';
import { Loader2 } from 'lucide-react';
import { cn } from '@/lib/utils';

// =============================================================================
// TYPES
// =============================================================================

type EmptyStateVariant = 'default' | 'loading' | 'error';

interface EmptyStateProps {
  /** Icon component to display */
  icon: ComponentType<{ className?: string }>;
  /** Main title text */
  title: string;
  /** Optional description text */
  description?: string;
  /** Visual variant */
  variant?: EmptyStateVariant;
  /** Custom accent color (Tailwind color class without prefix, e.g., 'accent-blue') */
  accentColor?: string;
  /** Optional action button */
  action?: ReactNode;
  /** Additional className for the container */
  className?: string;
  /** Size variant */
  size?: 'sm' | 'md' | 'lg';
}

// =============================================================================
// VARIANT STYLES
// =============================================================================

const variantStyles: Record<EmptyStateVariant, {
  iconBg: string;
  iconBorder: string;
  iconColor: string;
  titleColor: string;
}> = {
  default: {
    // opacity.bg.strong (0.08) + opacity.border.medium (0.12)
    iconBg: 'bg-white/[0.08]',
    iconBorder: 'border-white/[0.12]',
    iconColor: 'text-white/30',
    // opacity.text.secondary (0.70)
    titleColor: 'text-white/70',
  },
  loading: {
    iconBg: 'bg-accent-blue/15',
    iconBorder: 'border-accent-blue/30',
    iconColor: 'text-accent-blue',
    titleColor: 'text-white/80',
  },
  error: {
    iconBg: 'bg-accent-red/15',
    iconBorder: 'border-accent-red/30',
    iconColor: 'text-accent-red',
    titleColor: 'text-accent-red',
  },
};

const sizeStyles = {
  sm: {
    container: 'gap-3',
    iconBox: 'w-10 h-10 rounded-xl',
    icon: 'w-5 h-5',
    title: 'text-xs',
    description: 'text-[10px]',
  },
  md: {
    container: 'gap-4',
    iconBox: 'w-14 h-14 rounded-xl',
    icon: 'w-7 h-7',
    title: 'text-sm',
    description: 'text-xs',
  },
  lg: {
    container: 'gap-5',
    iconBox: 'w-16 h-16 rounded-2xl',
    icon: 'w-8 h-8',
    title: 'text-base',
    description: 'text-xs',
  },
};

// =============================================================================
// COMPONENT
// =============================================================================

export const EmptyState = memo(function EmptyState({
  icon: Icon,
  title,
  description,
  variant = 'default',
  accentColor,
  action,
  className,
  size = 'lg',
}: EmptyStateProps) {
  const styles = variantStyles[variant];
  const sizes = sizeStyles[size];

  // Custom accent color overrides
  const iconBg = accentColor ? `bg-${accentColor}/15` : styles.iconBg;
  const iconBorder = accentColor ? `border-${accentColor}/30` : styles.iconBorder;
  const iconColor = accentColor ? `text-${accentColor}` : styles.iconColor;

  const isLoading = variant === 'loading';

  return (
    <div
      className={cn(
        'flex flex-col items-center justify-center h-full',
        sizes.container,
        className
      )}
    >
      {/* Icon box */}
      <div className="relative">
        <div
          className={cn(
            'flex items-center justify-center border',
            sizes.iconBox,
            iconBg,
            iconBorder
          )}
        >
          {isLoading ? (
            <Loader2 className={cn(sizes.icon, iconColor, 'animate-spin')} />
          ) : (
            <Icon className={cn(sizes.icon, iconColor)} />
          )}
        </div>

        {/* Loading pulse ring */}
        {isLoading && (
          <div
            className={cn(
              'absolute inset-0 rounded-2xl animate-ping opacity-30',
              iconBg
            )}
            style={{ animationDuration: '1.5s' }}
          />
        )}
      </div>

      {/* Text */}
      <div className="text-center max-w-xs">
        <p className={cn('font-medium', sizes.title, styles.titleColor)}>
          {title}
        </p>
        {description && (
          <p className={cn('text-white/40 mt-1', sizes.description)}>
            {description}
          </p>
        )}
      </div>

      {/* Action */}
      {action && <div className="mt-2">{action}</div>}
    </div>
  );
});

// =============================================================================
// CONVENIENCE EXPORTS
// =============================================================================

export interface LoadingStateProps {
  title?: string;
  description?: string;
  accentColor?: string;
  className?: string;
  size?: 'sm' | 'md' | 'lg';
}

/** Loading state shorthand */
export const LoadingState = memo(function LoadingState({
  title = 'Loading...',
  description,
  accentColor = 'accent-blue',
  className,
  size = 'lg',
}: LoadingStateProps) {
  return (
    <EmptyState
      icon={Loader2}
      title={title}
      description={description}
      variant="loading"
      accentColor={accentColor}
      className={className}
      size={size}
    />
  );
});
