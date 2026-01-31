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
 * Design System (sidebarTokens) — shadcn-inspired compact:
 * - Row height: h-8 (32px) compact rows
 * - Row gaps: gap-2 (8px) between elements
 * - Row padding: px-2 (8px) horizontal
 * - Icon box: 16x16px (base), 40x40px (header)
 * - Badge: plain text, no pill background
 * - Section spacing: mt-4 between sections, space-y-px between rows
 * - Section indent: pl-4 (shadcn SidebarMenuSub pattern)
 */

import { memo, type ReactNode } from 'react';
import { cn } from '@/lib/utils';
import { panelClasses, gapTokens, sidebarTokens } from '@/design/tokens';
import { FilterTree } from '@/components/ui/FilterTree';
import type { CheckboxState } from '@/components/ui/TriStateCheckbox';

// =============================================================================
// TYPES
// =============================================================================

/** Individual stat pill displayed in the header */
export interface HeaderStat {
  label: string;
  value?: string | number;
  color?: string;
}

export interface SidebarContentProps {
  /** Test ID for the container */
  testId?: string;
  /** Additional class names */
  className?: string;
  /** Header configuration (optional - omit when tab bar handles identity) */
  header?: {
    icon: ReactNode;
    iconGradient?: { from: string; to: string };
    title: string;
    /** Plain subtitle text (legacy) */
    subtitle?: string;
    /** Stat pills - replaces subtitle with rich badges */
    stats?: HeaderStat[];
    /** Live status indicator: 'live' = green pulse, 'loading' = amber pulse */
    status?: 'live' | 'loading';
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
  const accentColor = header.iconGradient?.from || '#888';

  return (
    <div
      className={cn(panelClasses.container, className)}
      data-testid={testId}
      role="region"
      aria-label={header.title}
    >
      {/* Header - Premium Glassmorphism (A-E improvements) */}
      <div className="relative group/header overflow-hidden px-4 py-3.5 border-b border-white/[0.04]">
        {/* C: Gradient accent background - stronger presence */}
        {header.iconGradient && (
          <div
            className="absolute inset-0 pointer-events-none opacity-[0.07] group-hover/header:opacity-[0.12] transition-opacity duration-300"
            style={{
              background: `linear-gradient(135deg, ${header.iconGradient.from}, transparent 60%, ${header.iconGradient.to}40)`,
            }}
          />
        )}

        <div className={cn('relative flex items-center', gapTokens.spacious)}>
          {/* A: Enlarged icon box (w-10 h-10) with stronger glow + colored ring */}
          <div className="relative flex-shrink-0">
            {/* Glow - stronger opacity + wider blur */}
            {header.iconGradient && (
              <div
                className="absolute -inset-1 rounded-2xl opacity-30 blur-xl group-hover/header:opacity-50 transition-opacity duration-300"
                style={{
                  background: `linear-gradient(to bottom right, ${header.iconGradient.from}, ${header.iconGradient.to})`,
                }}
              />
            )}
            {/* D: Icon box with hover scale */}
            <div
              className={cn(
                'relative flex items-center justify-center',
                'w-10 h-10 rounded-xl',
                'shadow-lg shadow-black/30',
                'transition-transform duration-200 ease-out',
                'group-hover/header:scale-105',
              )}
              style={{
                background: header.iconGradient
                  ? `linear-gradient(to bottom right, ${header.iconGradient.from}25, ${header.iconGradient.to}25)`
                  : 'rgba(255,255,255,0.05)',
                border: `1px solid ${accentColor}30`,
                boxShadow: `0 0 12px ${accentColor}15, 0 4px 12px rgba(0,0,0,0.3)`,
              }}
            >
              {header.icon}
            </div>
          </div>

          {/* B: Title + stat pills (stronger hierarchy) */}
          <div className="flex-1 min-w-0">
            <div className={cn('flex items-center', gapTokens.default)}>
              <h2 className="text-[15px] font-semibold text-white/90 tracking-tight">
                {header.title}
              </h2>
              {/* E: Live status badge */}
              {header.status === 'live' && (
                <span className="flex items-center gap-1">
                  <span className="relative flex h-2 w-2">
                    <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75" />
                    <span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500" />
                  </span>
                </span>
              )}
              {header.status === 'loading' && (
                <span className="relative flex h-2 w-2">
                  <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-amber-400 opacity-75" />
                  <span className="relative inline-flex rounded-full h-2 w-2 bg-amber-500" />
                </span>
              )}
            </div>

            {/* B: Stat pills or legacy subtitle */}
            {header.stats && header.stats.length > 0 ? (
              <div className={cn('flex items-center mt-1', gapTokens.compact)}>
                {header.stats.map((stat, i) => (
                  <span
                    key={i}
                    className="inline-flex items-center gap-1 px-1.5 py-0.5 rounded-md text-[10px] font-medium tabular-nums"
                    style={{
                      color: stat.color || 'rgba(255,255,255,0.55)',
                      backgroundColor: stat.color ? `${stat.color}15` : 'rgba(255,255,255,0.06)',
                    }}
                  >
                    {stat.value !== undefined && (
                      <span className="font-semibold">{stat.value}</span>
                    )}
                    {stat.label}
                  </span>
                ))}
              </div>
            ) : header.subtitle ? (
              <p className="text-[10px] text-white/40 mt-0.5 truncate">
                {header.subtitle}
              </p>
            ) : null}
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
