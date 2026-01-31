/**
 * NovaNet Design Tokens
 *
 * Centralized design system tokens for consistent styling across the app.
 * These tokens align with the CSS variables defined in globals.css
 */

// ============================================================================
// SPACING
// ============================================================================

export const spacing = {
  px: '1px',
  0: '0',
  0.5: '0.125rem', // 2px
  1: '0.25rem',    // 4px
  1.5: '0.375rem', // 6px
  2: '0.5rem',     // 8px
  2.5: '0.625rem', // 10px
  3: '0.75rem',    // 12px
  4: '1rem',       // 16px
  5: '1.25rem',    // 20px
  6: '1.5rem',     // 24px
  8: '2rem',       // 32px
  10: '2.5rem',    // 40px
  12: '3rem',      // 48px
  16: '4rem',      // 64px
} as const;

// ============================================================================
// BORDER RADIUS
// ============================================================================

export const radius = {
  none: '0',
  sm: '0.375rem',   // 6px
  md: '0.5rem',     // 8px
  DEFAULT: '0.75rem', // 12px - Our primary radius
  lg: '1rem',       // 16px
  xl: '1.25rem',    // 20px
  '2xl': '1.5rem',  // 24px
  full: '9999px',
} as const;

// ============================================================================
// SHADOWS
// ============================================================================

export const shadows = {
  none: 'none',
  sm: '0 1px 2px 0 rgb(0 0 0 / 0.3)',
  DEFAULT: '0 1px 3px 0 rgb(0 0 0 / 0.3), 0 1px 2px -1px rgb(0 0 0 / 0.3)',
  md: '0 4px 6px -1px rgb(0 0 0 / 0.3), 0 2px 4px -2px rgb(0 0 0 / 0.3)',
  lg: '0 10px 15px -3px rgb(0 0 0 / 0.3), 0 4px 6px -4px rgb(0 0 0 / 0.3)',
  xl: '0 20px 25px -5px rgb(0 0 0 / 0.3), 0 8px 10px -6px rgb(0 0 0 / 0.3)',
  '2xl': '0 25px 50px -12px rgb(0 0 0 / 0.5)',
  // Glass-specific shadows
  glass: '0 25px 50px -12px rgba(0, 0, 0, 0.8), 0 0 0 1px rgba(255, 255, 255, 0.05) inset',
  glow: (color: string) => `0 0 20px ${color}`,
  glowLg: (color: string) => `0 0 40px ${color}, 0 0 80px ${color}`,
} as const;

// ============================================================================
// Z-INDEX SCALE
// ============================================================================

export const zIndex = {
  hide: -1,
  auto: 'auto',
  base: 0,
  dropdown: 100,
  sticky: 200,
  fixed: 300,
  modalBackdrop: 400,
  modal: 450,
  popover: 500,
  tooltip: 600,
  toast: 700,
  overlay: 9999,
} as const;

// ============================================================================
// TIMING & EASING (defined first so transitions can reference)
// ============================================================================

export const durations = {
  faster: 100,  // Micro-interactions (button press feedback)
  fast: 150,
  normal: 200,
  slow: 300,
  slower: 500,
} as const;

export const easing = {
  /** Standard easing for most transitions */
  default: 'ease-out',
  /** Overshoot bounce for playful interactions (buttons, toggles) */
  spring: 'cubic-bezier(0.34, 1.56, 0.64, 1)',
  /** Smooth deceleration for entrances */
  out: 'cubic-bezier(0, 0, 0.2, 1)',
  /** Acceleration for exits */
  in: 'cubic-bezier(0.4, 0, 1, 1)',
  /** Symmetric for reversible animations */
  inOut: 'cubic-bezier(0.4, 0, 0.2, 1)',
} as const;

// Composed transitions (duration + easing) - DRY: references primitives
export const transitions = {
  // Standard transitions (ease-out)
  faster: `${durations.faster}ms ${easing.default}`,
  fast: `${durations.fast}ms ${easing.default}`,
  normal: `${durations.normal}ms ${easing.default}`,
  slow: `${durations.slow}ms ${easing.default}`,
  slower: `${durations.slower}ms ${easing.inOut}`,
  // Spring transitions (bouncy overshoot) - for buttons, toggles
  springFaster: `${durations.faster}ms ${easing.spring}`,
  springFast: `${durations.fast}ms ${easing.spring}`,
  springNormal: `${durations.normal}ms ${easing.spring}`,
} as const;

// ============================================================================
// INTERACTIVE CONTROLS (buttons, toolbars, action bars)
// ============================================================================

export const controls = {
  /** Standard button size (px) - use for toolbar buttons, icon buttons */
  buttonSize: 44,
  buttonSizePx: '44px',
  /** Icon size for Lucide icons (px) */
  iconSize: 18,
  iconSizePx: '18px',
  /** Gap between grouped buttons (px) */
  gap: 8,
  gapPx: '8px',
} as const;

// ============================================================================
// SEMANTIC GAP TOKENS - Consistent spacing by purpose
// ============================================================================

/**
 * Gap tokens - semantic naming for flex/grid gaps
 *
 * Usage: Replace hardcoded `gap-X` with semantic tokens
 */
export const gapTokens = {
  /** 4px - Icon pairs, tight badges */
  tight: 'gap-1',
  /** 6px - Buttons, filter rows, compact lists */
  compact: 'gap-1.5',
  /** 8px - Default for most flex layouts */
  default: 'gap-2',
  /** 10px - Filter sections, elevated panels */
  comfortable: 'gap-2.5',
  /** 12px - Headers, section content */
  spacious: 'gap-3',
  /** 16px - Large sections, panel spacing */
  large: 'gap-4',
} as const;

// ============================================================================
// SEMANTIC PADDING TOKENS - Consistent padding by purpose
// ============================================================================

/**
 * Padding tokens - semantic naming for padding patterns
 */
export const paddingTokens = {
  /** Compact buttons, badges */
  compact: 'px-2.5 py-1',
  /** Standard buttons, filter rows */
  standard: 'px-3 py-1.5',
  /** Large buttons, panel sections */
  large: 'px-4 py-2',
  /** Panel headers, elevated content */
  panel: 'px-4 py-3',
  /** Spacious headers (modals) */
  spacious: 'px-4 py-4',
} as const;

// ============================================================================
// COMPONENT PATTERN TOKENS - Ready-to-use class combinations
// ============================================================================

/**
 * Badge/pill pattern tokens - unified badge styling
 */
export const badgeClasses = {
  /** Compact badge - counts, status */
  compact: 'inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[10px] font-medium',
  /** Standard badge - labels, tags */
  standard: 'inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg text-xs font-medium',
  /** Large badge - prominent status */
  large: 'inline-flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm font-medium',
} as const;

/**
 * Icon button pattern tokens - unified icon button styling
 *
 * Consolidates repeating icon button patterns across the codebase.
 */
export const iconButtonClasses = {
  /** Ghost variant - default icon button (no background) */
  ghost: 'p-1.5 rounded-lg transition-all duration-150 text-white/50 hover:text-white/70 hover:bg-white/[0.06]',
  /** Action variant - with success feedback */
  action: 'p-1.5 rounded-lg transition-all duration-150 text-white/50 hover:text-white/70 hover:bg-white/[0.06] active:text-emerald-400 active:bg-emerald-500/20',
  /** Close variant - for dismiss/close buttons */
  close: 'w-8 h-8 rounded-lg flex items-center justify-center text-white/40 hover:text-white/80 hover:bg-white/[0.06] transition-colors duration-150',
  /** Copy variant - with copied state */
  copy: 'p-1.5 rounded-lg opacity-50 group-hover:opacity-100 transition-all duration-150 text-white/60 hover:text-white/90 hover:bg-white/[0.06]',
} as const;

/**
 * Section header tokens - unified section styling
 */
export const sectionHeaderClasses = {
  /** Standard section header */
  standard: 'flex items-center gap-2 text-xs uppercase tracking-wider font-semibold text-white/40',
  /** Elevated section header (with background) */
  elevated: 'flex items-center gap-2.5 px-4 py-3 text-xs uppercase tracking-wider font-semibold text-white/40 border-b border-white/[0.06]',
} as const;

// ============================================================================
// OPACITY PRIMITIVES - White opacity scale for consistent transparency
// ============================================================================

/**
 * White opacity scale - semantic naming for consistent transparency
 *
 * Usage: Replace hardcoded `white/[0.xx]` with these tokens
 * Example: `text-white/40` → `opacity.text.muted`
 */
export const opacity = {
  /** Background opacities */
  bg: {
    subtle: 'white/[0.03]',   // Hover states, very subtle
    light: 'white/[0.04]',    // Light backgrounds
    medium: 'white/[0.06]',   // Card backgrounds, selected states
    strong: 'white/[0.08]',   // Elevated elements
    heavy: 'white/[0.10]',    // Strong emphasis
    intense: 'white/[0.15]',  // Maximum emphasis
  },
  /** Border opacities */
  border: {
    subtle: 'white/[0.06]',   // Panel dividers
    light: 'white/[0.08]',    // Card borders
    medium: 'white/[0.12]',   // Modal borders
    strong: 'white/[0.15]',   // Active borders
    heavy: 'white/[0.20]',    // High emphasis borders
  },
  /** Text opacities */
  text: {
    disabled: 'white/[0.20]', // Disabled text
    muted: 'white/[0.30]',    // Very muted (counts, hints)
    subtle: 'white/[0.40]',   // Secondary text
    secondary: 'white/[0.60]',// Body text, labels
    primary: 'white/[0.80]',  // Important text
    strong: 'white/[0.90]',   // Headings
    full: 'white',            // Maximum contrast
  },
} as const;

// ============================================================================
// GLOW EFFECTS - Consistent glow patterns for selected/active states
// ============================================================================

/**
 * Glow effect tokens for UI elements
 *
 * Usage: Replace hardcoded boxShadow glow values with these functions
 * Example: `boxShadow: glowEffects.row(color)` or `boxShadow: glowEffects.novanet.card`
 */
export const glowEffects = {
  /** Dynamic color glows - use with item's accent color */
  row: (color: string) => `0 0 20px ${color}20, 0 4px 12px rgba(0,0,0,0.3)`,
  iconBox: (color: string) => `0 0 12px ${color}30`,
  checkbox: (color: string) => `0 0 8px ${color}40`,
  section: (color: string) => `0 0 12px ${color}10`,

  /** Static novanet brand glows - for brand-themed elements, buttons, etc. */
  novanet: {
    /** novanet-500 = #8b5cf6 (violet) */
    card: '0 0 20px rgba(139, 92, 246, 0.15), 0 4px 12px rgba(0,0,0,0.3)',
    iconBox: '0 0 12px rgba(139, 92, 246, 0.30)',
    subtle: '0 0 8px rgba(139, 92, 246, 0.20)',
  },
} as const;

// ============================================================================
// DEFAULT COLORS - Fallback colors for components
// ============================================================================

/**
 * Default/fallback colors for components when no color prop is provided
 */
export const defaultColors = {
  /** Neutral gray for unspecified categories */
  neutral: '#8b8b8b',
  /** Muted text color */
  muted: '#6b7280',
} as const;

// ============================================================================
// ICON SIZES - Consistent icon dimensions
// ============================================================================

/**
 * Icon size tokens - Tailwind class pairs for width/height
 *
 * Usage: Replace `w-4 h-4` patterns with `iconSizes.md`
 */
export const iconSizes = {
  /** 12px - Tiny icons (badges, compact UI) */
  xs: 'w-3 h-3',
  /** 14px - Small icons (inline, filters) */
  sm: 'w-3.5 h-3.5',
  /** 16px - Medium icons (default, buttons) */
  md: 'w-4 h-4',
  /** 18px - Large icons (headers, actions) */
  lg: 'w-4.5 h-4.5',
  /** 20px - Extra large icons (feature icons) */
  xl: 'w-5 h-5',
  /** 24px - Hero icons (empty states) */
  '2xl': 'w-6 h-6',
} as const;

// ============================================================================
// PANEL CLASSES - Consistent sidebar/panel styling
// ============================================================================

/**
 * Panel design tokens - Unified sidebar/panel system
 *
 * Based on NodeLabelsSection gold standard:
 * - Consistent padding: px-4 py-3/4
 * - Border: border-white/[0.06]
 * - Headers: icon box + title + subtitle
 */
export const panelClasses = {
  /** Panel container */
  container: 'flex flex-col h-full',

  /** Panel header - icon box + title pattern */
  header: 'px-4 py-4 border-b border-white/[0.06]',
  headerContent: 'flex items-center gap-3',
  headerIconBox: 'w-9 h-9 rounded-xl bg-gradient-to-br from-white/10 to-white/5 border border-white/10 flex items-center justify-center',
  headerIcon: 'w-4 h-4 text-white/60',
  headerTitle: 'text-sm font-medium text-white/90',
  headerSubtitle: 'text-[11px] text-white/40',

  /** Panel body - scrollable content */
  body: 'flex-1 overflow-y-auto scrollbar-thin p-3',

  /** Panel footer - stats/actions */
  footer: 'px-4 py-3 border-t border-white/[0.06]',
  footerText: 'text-[11px] text-white/40 text-center',

  /** Section divider */
  divider: 'border-b border-white/[0.06]',

  /** Action bar - compact button row */
  actionBar: 'flex items-center justify-between px-1 py-1.5 mb-2',
  actionButton: 'text-[10px] text-white/40 hover:text-white/60 transition-colors',

  /** Collapsible section header */
  sectionHeader: 'flex items-center justify-between px-4 py-3 cursor-pointer hover:bg-white/[0.02] transition-colors',
  sectionTitle: 'text-xs font-medium text-white/70',
  sectionCount: 'text-xs text-white/40',
} as const;

// ============================================================================
// GLASS MORPHISM - Premium Nika-Inspired System
// ============================================================================

export const glass = {
  // Surface colors - 4-step hierarchy (0%, 5%, 8%, 12%, 16%)
  surface: {
    0: 'hsl(0, 0%, 0%)',             // #000000 - Background
    1: 'hsl(240, 8%, 5%)',           // #0D0D10 - Cards, sidebars
    2: 'hsl(240, 6%, 8%)',           // #141418 - Elevated panels
    3: 'hsl(240, 5%, 12%)',          // #1C1C22 - Hover states
    4: 'hsl(240, 5%, 16%)',          // #24242C - Active/pressed
    // Legacy aliases
    base: 'hsl(0, 0%, 0%)',
    hover: 'hsl(240, 5%, 12%)',
  },
  // Background opacity levels (for overlays)
  bg: {
    light: 'rgba(13, 13, 16, 0.6)',
    medium: 'rgba(13, 13, 16, 0.8)',
    heavy: 'rgba(13, 13, 16, 0.95)',
  },
  // Border opacity levels - Calibrated for contrast
  border: {
    subtle: 'rgba(255, 255, 255, 0.08)',
    light: 'rgba(255, 255, 255, 0.12)',
    medium: 'rgba(255, 255, 255, 0.15)',
    heavy: 'rgba(255, 255, 255, 0.20)',
    active: 'rgba(255, 255, 255, 0.30)',
  },
  // Blur intensity
  blur: {
    sm: '8px',
    md: '12px',
    lg: '16px',
    xl: '24px',
  },
  // Inner highlights for depth
  highlight: {
    subtle: 'rgba(255, 255, 255, 0.04)',
    medium: 'rgba(255, 255, 255, 0.06)',
    strong: 'rgba(255, 255, 255, 0.10)',
  },
} as const;

// ============================================================================
// TYPOGRAPHY
// ============================================================================

export const typography = {
  fontFamily: {
    sans: 'var(--font-geist-sans), system-ui, -apple-system, sans-serif',
    mono: 'var(--font-geist-mono), ui-monospace, monospace',
  },
  fontSize: {
    xs: ['0.75rem', { lineHeight: '1rem' }],      // 12px
    sm: ['0.875rem', { lineHeight: '1.25rem' }],  // 14px
    base: ['1rem', { lineHeight: '1.5rem' }],     // 16px
    lg: ['1.125rem', { lineHeight: '1.75rem' }],  // 18px
    xl: ['1.25rem', { lineHeight: '1.75rem' }],   // 20px
    '2xl': ['1.5rem', { lineHeight: '2rem' }],    // 24px
    '3xl': ['1.875rem', { lineHeight: '2.25rem' }], // 30px
    '4xl': ['2.25rem', { lineHeight: '2.5rem' }],   // 36px
  },
  fontWeight: {
    normal: '400',
    medium: '500',
    semibold: '600',
    bold: '700',
  },
} as const;

// ============================================================================
// SEMANTIC COLORS (CSS variable references)
// ============================================================================

export const colors = {
  // Base
  background: 'hsl(var(--background))',
  foreground: 'hsl(var(--foreground))',

  // Primary - NovaNet brand
  primary: {
    DEFAULT: 'hsl(var(--primary))',
    foreground: 'hsl(var(--primary-foreground))',
  },

  // Secondary
  secondary: {
    DEFAULT: 'hsl(var(--secondary))',
    foreground: 'hsl(var(--secondary-foreground))',
  },

  // Muted
  muted: {
    DEFAULT: 'hsl(var(--muted))',
    foreground: 'hsl(var(--muted-foreground))',
  },

  // Accent
  accent: {
    DEFAULT: 'hsl(var(--accent))',
    foreground: 'hsl(var(--accent-foreground))',
  },

  // Semantic
  destructive: {
    DEFAULT: 'hsl(var(--destructive))',
    foreground: 'hsl(var(--destructive-foreground))',
  },
  success: {
    DEFAULT: 'hsl(var(--success))',
    foreground: 'hsl(var(--success-foreground))',
  },
  warning: {
    DEFAULT: 'hsl(var(--warning))',
    foreground: 'hsl(var(--warning-foreground))',
  },

  // Border & Input
  border: 'hsl(var(--border))',
  input: 'hsl(var(--input))',
  ring: 'hsl(var(--ring))',
} as const;

// ============================================================================
// TAILWIND CLASS HELPERS
// ============================================================================

/**
 * Glass morphism utility classes - Premium Nika-Inspired
 */
export const glassClasses = {
  subtle: 'bg-[hsl(240,8%,5%)] backdrop-blur-sm border border-white/8',
  light: 'bg-[hsl(240,8%,5%)] backdrop-blur-md border border-white/12',
  medium: 'bg-[hsl(240,6%,8%)] backdrop-blur-xl border border-white/12',
  heavy: 'bg-[hsl(240,5%,12%)] backdrop-blur-xl border border-white/15',
  floating: 'bg-[hsl(240,6%,8%)] backdrop-blur-xl border border-white/15 rounded-2xl shadow-2xl shadow-black/50 ring-1 ring-white/4 ring-inset',
  /** Modal variant - fully opaque dark background for better perf and consistency */
  modal: 'bg-[#0d0d12] border border-white/[0.12] rounded-2xl shadow-2xl shadow-black/60',

  // Hover variants for interactive elements
  subtleHover: 'hover:bg-white/[0.04] hover:border-white/[0.08]',
  lightHover: 'hover:bg-white/[0.06] hover:border-white/[0.10]',
  mediumHover: 'hover:bg-white/[0.08] hover:border-white/[0.12]',
  heavyHover: 'hover:bg-white/[0.12] hover:border-white/[0.15]',
} as const;

/**
 * Shadow utility classes - Tailwind shadow presets with black opacity
 */
export const shadowTokens = {
  sm: 'shadow-sm shadow-black/10',
  md: 'shadow-md shadow-black/20',
  lg: 'shadow-lg shadow-black/30',
  xl: 'shadow-xl shadow-black/40',
} as const;

/**
 * Modal/Dialog classes - Unified modal system
 *
 * Best practices:
 * - Opaque background (no backdrop-blur on content for performance)
 * - Focus trap for accessibility (WCAG 2.1 AA)
 * - Body scroll lock when open
 * - Escape key to close
 *
 * Note: z-index uses Tailwind z-50 for legacy compatibility.
 * New Modal component uses zIndex.modal (450) via inline styles.
 */
export const modalClasses = {
  /** Backdrop overlay - darkens and blurs background */
  backdrop: 'fixed inset-0 bg-black/70 backdrop-blur-sm',
  /** Modal container - centers content */
  container: 'fixed inset-0 flex items-center justify-center',
  /** Modal content - the actual dialog box */
  content: 'bg-[#0d0d12] border border-white/[0.12] rounded-2xl shadow-2xl shadow-black/60',
  /** Modal header - title area with close button */
  header: 'flex items-center justify-between px-4 py-3 border-b border-white/[0.08]',
  /** Modal body - scrollable content area */
  body: 'overflow-y-auto',
  /** Modal footer - action buttons area */
  footer: 'px-4 py-3 border-t border-white/[0.08]',
  /** Close button style */
  closeButton: 'w-8 h-8 rounded-lg flex items-center justify-center text-white/40 hover:text-white/80 hover:bg-white/[0.06] transition-colors duration-150',
} as const;

/**
 * Common button variant classes - HIGH CONTRAST
 */
export const buttonClasses = {
  base: 'inline-flex items-center justify-center rounded-lg font-medium transition-all duration-200',
  ghost: 'text-white/60 hover:bg-white/10 hover:text-white',
  outline: 'border border-white/20 bg-white/8 hover:bg-white/15 text-white/90',
  primary: 'bg-primary text-primary-foreground hover:bg-primary/90 shadow-lg shadow-primary/30',
} as const;

/**
 * Text opacity utilities - Premium contrast levels
 *
 * Based on research from Nika Draft, Linear, Raycast:
 * - Primary: 95% (headings, important content)
 * - Secondary: 80% (body text, readable)
 * - Muted: 65% (captions, hints)
 * - Disabled: 45% (non-interactive)
 */
export const textOpacity = {
  disabled: 'text-white/45',
  muted: 'text-white/65',
  secondary: 'text-white/80',
  primary: 'text-white/95',
  full: 'text-white',
} as const;

// ============================================================================
// FILTER TREE - Unified hierarchical filter design
// ============================================================================

/**
 * Filter tree design tokens - Unified system for Schema Browser & Data View
 *
 * Design principles (based on Data View style):
 * - Flat tree structure with border-left connectors
 * - Tri-state checkboxes for hierarchical selection
 * - Chevrons for expand/collapse
 * - Category-colored icons
 * - Counts aligned right
 * - Progress bars (optional, for data counts)
 *
 * Variants:
 * - default: Standard filter tree (Schema Browser)
 * - data: With progress bars (Data View / Node filters)
 */
export const filterTreeClasses = {
  /** Container for the filter tree */
  container: 'space-y-2',

  /** Section header row (category level) - Premium style (matches FilterSection) */
  sectionHeader: [
    'flex items-center gap-2.5',
    'py-1 rounded-xl',
    'transition-all duration-200',
    'cursor-pointer select-none',
  ].join(' '),

  /** Section content (nested items) - full width like Views */
  sectionContent: 'mt-1 space-y-2.5 overflow-hidden transition-all duration-300',
  sectionContentExpanded: 'max-h-[800px] opacity-100 py-2',
  sectionContentCollapsed: 'max-h-0 opacity-0',

  /** Individual row (item level) - Premium frosted glass (matches FilterCard) */
  row: [
    'group w-full flex items-center gap-3',
    'h-12 px-3.5 rounded-xl', // 48px height for WCAG tap targets
    // Frosted glass base
    'backdrop-blur-md',
    'ring-1 ring-inset ring-white/[0.06]',
    'bg-white/[0.03]',
    // Transitions
    'transition-all duration-200',
    // Hover
    'hover:bg-white/[0.06] hover:ring-white/[0.10]',
    // Focus
    'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-novanet-500/60 focus-visible:ring-offset-1 focus-visible:ring-offset-black/50',
  ].join(' '),
  rowSelected: 'bg-white/[0.08] ring-white/[0.12] shadow-lg shadow-black/20',
  rowDisabled: 'opacity-50 cursor-not-allowed',

  /** Chevron for expand/collapse */
  chevron: 'w-3.5 h-3.5 text-white/40 transition-transform duration-200',
  chevronCollapsed: '-rotate-90',

  /** Checkbox container - matches TriStateCheckbox for consistency */
  checkbox: 'w-4 h-4 rounded border-2 flex items-center justify-center transition-all duration-200 flex-shrink-0',
  checkboxUnchecked: 'border-white/20 bg-transparent',
  checkboxChecked: 'border-transparent', // Color set dynamically via style prop

  /** Label text - truncate to prevent wrapping */
  label: 'text-[13px] font-medium transition-colors duration-200 flex-1 text-left truncate',
  labelSelected: 'text-white',
  labelUnselected: 'text-white/70 group-hover:text-white/90',

  /** Section label (category) - Uppercase accent */
  sectionLabel: 'text-[11px] font-bold uppercase tracking-wider', // Color set dynamically

  /** Count display - Pill style (matches FilterCard) */
  count: 'text-[11px] font-semibold tabular-nums transition-all duration-200 flex-shrink-0 min-w-[32px] text-center py-1 px-2.5 rounded-full',
  countSelected: 'text-white/90 bg-white/[0.12]',
  countUnselected: 'text-white/50 bg-white/[0.05] group-hover:bg-white/[0.08] group-hover:text-white/70',
  countMuted: 'text-white/40',

  /** Progress bar container (data variant) - compact to allow full labels */
  progressBar: 'w-10 flex-shrink-0',

  /** Header with total and execute button */
  header: 'flex items-center justify-between px-1 mb-2',
  headerTitle: 'flex items-center gap-2',
  headerBadge: 'px-1.5 py-0.5 rounded-full bg-emerald-500/20 text-emerald-400 text-[10px] font-semibold animate-in fade-in duration-200',

  /** Execute button */
  executeButton: 'p-1.5 rounded-lg transition-all duration-200',
  executeButtonEnabled: 'text-emerald-400 hover:text-emerald-300 hover:bg-emerald-500/10 hover:scale-110',
  executeButtonDisabled: 'text-white/40 cursor-not-allowed',
} as const;

/**
 * Scope-specific accent colors - used for both Schema and Data modes
 */
export const scopeAccents = {
  project: {
    color: '#8b5cf6', // violet-500
    bg: 'bg-violet-500/20',
    text: 'text-violet-400',
    border: 'border-violet-500/30',
  },
  global: {
    color: '#10b981', // emerald-500
    bg: 'bg-emerald-500/20',
    text: 'text-emerald-400',
    border: 'border-emerald-500/30',
  },
  shared: {
    color: '#f59e0b', // amber-500
    bg: 'bg-amber-500/20',
    text: 'text-amber-400',
    border: 'border-amber-500/30',
  },
} as const;

// ============================================================================
// EXPORT ALL
// ============================================================================

export const tokens = {
  // Core design tokens
  spacing,
  radius,
  shadows,
  shadowTokens,
  zIndex,
  durations,
  easing,
  transitions,
  controls,
  opacity,
  glowEffects,
  defaultColors,
  iconSizes,
  // Semantic spacing tokens
  gapTokens,
  paddingTokens,
  // Component class tokens
  panelClasses,
  glass,
  typography,
  colors,
  glassClasses,
  modalClasses,
  buttonClasses,
  textOpacity,
  filterTreeClasses,
  scopeAccents,
  badgeClasses,
  iconButtonClasses,
  sectionHeaderClasses,
} as const;

export default tokens;
