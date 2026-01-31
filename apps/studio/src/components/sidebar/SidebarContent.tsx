'use client';

/**
 * SidebarContent - Unified sidebar content component
 *
 * Provides consistent skeleton for all sidebar tabs:
 * - Schema Browser
 * - Views
 * - Nodes
 * - Relationships
 *
 * Structure: container → header → toolbar? → body (FilterTree) → footer?
 *
 * Design System (sidebarTokens):
 * - Row height: h-11 (44px) - WCAG 2.1 AA touch targets
 * - Row gaps: gap-3 (12px) between elements
 * - Row padding: px-3 (12px) horizontal
 * - Icon box: 32x32px (base), 40x40px (header)
 * - Badge: min-w-7 (28px) pill shape
 * - Section spacing: space-y-2
 */

import { memo, type ReactNode } from 'react';
import { cn } from '@/lib/utils';
import { panelClasses, gapTokens, sidebarTokens } from '@/design/tokens';
import { FilterTree } from '@/components/ui/FilterTree';
import type { CheckboxState } from '@/components/ui/TriStateCheckbox';

// =============================================================================
// TYPES
// =============================================================================

export interface SidebarContentProps {
  /** Test ID for the container */
  testId?: string;
  /** Additional class names */
  className?: string;
  /** Header configuration */
  header: {
    icon: ReactNode;
    iconGradient?: { from: string; to: string };
    title: string;
    subtitle?: string;
    action?: ReactNode;
  };
  /** Toolbar content between header and body (e.g., AI search, tabs) */
  toolbar?: ReactNode;
  /** Footer content (optional) */
  footer?: ReactNode;
  /** Children - typically FilterTree.Root with sections */
  children: ReactNode;
}

export interface SidebarSectionProps {
  /** Section ID */
  id: string;
  /** Section label */
  label: string;
  /** Section icon */
  icon: ReactNode;
  /** Accent color (hex) */
  color: string;
  /** Tri-state checkbox state */
  checkboxState?: CheckboxState;
  /** Callback when checkbox is clicked */
  onCheckboxClick?: () => void;
  /** Show checkbox (default: true) */
  showCheckbox?: boolean;
  /** Total count for the section */
  count?: number;
  /** Default expanded state */
  defaultExpanded?: boolean;
  /** Children - typically FilterTree.Row items */
  children: ReactNode;
}

export interface SidebarRowProps {
  /** Item ID */
  id: string;
  /** Item label */
  label: string;
  /** Item icon */
  icon: ReactNode;
  /** Accent color (hex) */
  color: string;
  /** Whether selected */
  isSelected: boolean;
  /** Toggle callback */
  onToggle: () => void;
  /** Item count */
  count?: number;
  /** Show checkbox (default: true) */
  showCheckbox?: boolean;
  /** Keyboard shortcut */
  shortcut?: string;
}

// =============================================================================
// SIDEBAR CONTENT (Container)
// =============================================================================

export const SidebarContent = memo(function SidebarContent({
  testId,
  className,
  header,
  toolbar,
  footer,
  children,
}: SidebarContentProps) {
  return (
    <div
      className={cn(panelClasses.container, className)}
      data-testid={testId}
      role="region"
      aria-label={header.title}
    >
      {/* Header - Premium Glassmorphism */}
      <div className={cn('relative', panelClasses.header)}>
        {/* Background gradient */}
        <div
          className="absolute inset-0 pointer-events-none opacity-5"
          style={{
            background: header.iconGradient
              ? `linear-gradient(to bottom right, ${header.iconGradient.from}, transparent, ${header.iconGradient.to})`
              : undefined,
          }}
        />

        <div className={cn('relative flex items-center', gapTokens.spacious)}>
          {/* Icon box with glow - uses sidebarTokens.iconBox.lg (40x40px) */}
          <div className="relative">
            {header.iconGradient && (
              <div
                className="absolute inset-0 rounded-2xl opacity-20 blur-lg"
                style={{
                  background: `linear-gradient(to bottom right, ${header.iconGradient.from}, ${header.iconGradient.to})`,
                }}
              />
            )}
            <div
              className={cn(
                'relative',
                sidebarTokens.iconBox.lg, // Unified: 40x40px, rounded-xl
                'border border-white/10 shadow-lg shadow-black/20'
              )}
              style={{
                background: header.iconGradient
                  ? `linear-gradient(to bottom right, ${header.iconGradient.from}20, ${header.iconGradient.to}20)`
                  : 'rgba(255,255,255,0.05)',
              }}
            >
              {header.icon}
            </div>
          </div>

          {/* Title and subtitle */}
          <div className="flex-1 min-w-0">
            <h2 className="text-sm font-semibold text-white tracking-tight">
              {header.title}
            </h2>
            {header.subtitle && (
              <p className="text-[10px] text-white/40 mt-0.5 truncate">
                {header.subtitle}
              </p>
            )}
          </div>

          {/* Action button (optional) */}
          {header.action}
        </div>
      </div>

      {/* Toolbar - AI search, tabs, etc (optional) */}
      {toolbar}

      {/* Body - Scrollable content with unified padding */}
      <div className={panelClasses.body}>
        {children}
      </div>

      {/* Footer (optional) */}
      {footer && (
        <div className={cn(panelClasses.footer, 'bg-black/20')}>
          {footer}
        </div>
      )}
    </div>
  );
});

// =============================================================================
// SIDEBAR SECTION (Category Level)
// =============================================================================

/**
 * Re-export FilterTree.Section with standardized props
 * Use this instead of FilterTree.Section directly for consistency
 */
export const SidebarSection = memo(function SidebarSection({
  id,
  label,
  icon,
  color,
  checkboxState = 'none',
  onCheckboxClick,
  showCheckbox = true,
  count,
  defaultExpanded = true,
  children,
}: SidebarSectionProps) {
  return (
    <FilterTree.Section
      id={id}
      label={label}
      icon={icon}
      color={color}
      checkboxState={checkboxState}
      onCheckboxClick={onCheckboxClick}
      showCheckbox={showCheckbox}
      count={count}
      defaultExpanded={defaultExpanded}
    >
      {children}
    </FilterTree.Section>
  );
});

// =============================================================================
// SIDEBAR ROW (Item Level)
// =============================================================================

/**
 * Re-export FilterTree.Row with standardized props
 * Use this instead of FilterTree.Row directly for consistency
 */
export const SidebarRow = memo(function SidebarRow({
  id,
  label,
  icon,
  color,
  isSelected,
  onToggle,
  count,
  showCheckbox = true,
  shortcut,
}: SidebarRowProps) {
  return (
    <FilterTree.Row
      id={id}
      label={label}
      icon={icon}
      color={color}
      isSelected={isSelected}
      onToggle={onToggle}
      count={count}
      showCheckbox={showCheckbox}
      shortcut={shortcut}
    />
  );
});

// =============================================================================
// SIDEBAR ROOT (FilterTree wrapper)
// =============================================================================

export interface SidebarTreeProps {
  /** Show progress bars on rows */
  showProgressBars?: boolean;
  /** Maximum count for progress bars */
  maxCount?: number;
  /** Disable all interactions */
  disabled?: boolean;
  /** Children - SidebarSection components */
  children: ReactNode;
  /** Additional class names */
  className?: string;
}

/**
 * Wrapper around FilterTree.Root with standardized props
 */
export const SidebarTree = memo(function SidebarTree({
  showProgressBars = false,
  maxCount = 100,
  disabled = false,
  children,
  className,
}: SidebarTreeProps) {
  return (
    <FilterTree.Root
      showProgressBars={showProgressBars}
      maxCount={maxCount}
      disabled={disabled}
      className={className}
    >
      {children}
    </FilterTree.Root>
  );
});

// =============================================================================
// COMPOUND EXPORT
// =============================================================================

export const Sidebar = {
  Content: SidebarContent,
  Tree: SidebarTree,
  Section: SidebarSection,
  Row: SidebarRow,
};
