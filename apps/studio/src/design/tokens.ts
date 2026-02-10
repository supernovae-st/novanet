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
  /** 20px - Modal grids, spacious layouts */
  xl: 'gap-5',
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
 * Icon button pattern tokens - unified icon button styling
 *
 * Consolidates repeating icon button patterns across the codebase.
 */
export const iconButtonClasses = {
  /** Ghost variant - default icon button (no background) */
  ghost: 'p-1.5 rounded-lg transition-colors duration-150 text-white/50 hover:text-white/70 hover:bg-white/[0.06]',
  /** Action variant - with success feedback */
  action: 'p-1.5 rounded-lg transition-colors duration-150 text-white/50 hover:text-white/70 hover:bg-white/[0.06] active:text-emerald-400 active:bg-emerald-500/20',
  /** Close variant - for dismiss/close buttons */
  close: 'w-8 h-8 rounded-lg flex items-center justify-center text-white/40 hover:text-white/80 hover:bg-white/[0.06] transition-colors duration-150',
  /** Copy variant - with copied state */
  copy: 'p-1.5 rounded-lg opacity-50 group-hover:opacity-100 transition duration-150 text-white/60 hover:text-white/90 hover:bg-white/[0.06]',
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
  /** Border opacities - Unified 5-level scale */
  border: {
    subtle: 'white/[0.06]',   // 6%  - Panel dividers, very light
    light: 'white/10',        // 10% - Card borders, default
    medium: 'white/[0.12]',   // 12% - Glass borders, modal borders
    strong: 'white/[0.15]',   // 15% - Hover, elevated, floating
    heavy: 'white/20',        // 20% - Active, pressed, high emphasis
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
  /** 20px - Large icons (headers, actions) */
  lg: 'w-5 h-5',
  /** 24px - Extra large icons (feature icons, hero) */
  xl: 'w-6 h-6',
} as const;

// ============================================================================
// NODE CARD SIZES - Unified sizing for graph node components
// ============================================================================

/**
 * Node card size tokens - Centralized sizing for graph nodes
 *
 * Source of truth for card dimensions across all node components.
 * Maps size categories to width/height, and node types to categories.
 *
 * Usage:
 * ```ts
 * const { width, height } = nodeCardSizes.getByType('Page');
 * // or direct access
 * const size = nodeCardSizes.sizes.lg; // { width: 240, height: 120 }
 * ```
 */
export const nodeCardSizes = {
  /** Size categories - from extra-small to extra-large */
  sizes: {
    /** 160×80 - Knowledge atoms (Term, Expression, SEOKeyword) */
    xs: { width: 160, height: 80 },
    /** 180×90 - Compact nodes (ContentSlot, Style, Culture) */
    sm: { width: 180, height: 90 },
    /** 200×100 - Standard nodes (Block, EntityContent) */
    md: { width: 200, height: 100 },
    /** 220×110 - Medium-large (Locale, BrandIdentity) */
    lg: { width: 220, height: 110 },
    /** 240×120 - Large structural (Page, Entity, Layer attractors) */
    xl: { width: 240, height: 120 },
    /** 280×140 - Premium (Project, OrgConfig, Realm attractors) */
    '2xl': { width: 280, height: 140 },
  },

  /** Node type to size category mapping */
  byType: {
    // Extra-large (2xl) - Premium nodes
    Project: '2xl',
    OrgConfig: '2xl',
    realmAttractor: '2xl',

    // Large (xl) - Structural nodes
    Page: 'xl',
    Entity: 'xl',
    layerAttractor: 'xl',

    // Medium-large (lg)
    Locale: 'lg',
    BrandIdentity: 'lg',
    PageGenerated: 'lg',

    // Medium (md) - Standard nodes
    Block: 'md',
    EntityContent: 'md',
    Continent: 'md',
    PageType: 'md',
    BlockType: 'md',
    ContentSlot: 'md',

    // Small (sm) - Compact nodes
    Style: 'sm',
    Culture: 'sm',
    BlockPrompt: 'sm',
    Region: 'sm',
    Country: 'sm',

    // Extra-small (xs) - Knowledge atoms
    Term: 'xs',
    Expression: 'xs',
    Pattern: 'xs',
    CultureRef: 'xs',
    Taboo: 'xs',
    AudienceTrait: 'xs',
    TermSet: 'xs',
    ExpressionSet: 'xs',
    PatternSet: 'xs',
    CultureSet: 'xs',
    TabooSet: 'xs',
    AudienceSet: 'xs',
    CategorySet: 'xs',
    SEOKeyword: 'xs',
    GEOQuery: 'xs',
  } as Record<string, keyof typeof nodeCardSizes.sizes>,

  /** Get size by category key */
  get(sizeKey: 'xs' | 'sm' | 'md' | 'lg' | 'xl' | '2xl') {
    return this.sizes[sizeKey];
  },

  /** Get size by node type (with fallback to md) */
  getByType(type: string): { width: number; height: number } {
    const sizeKey = this.byType[type];
    return this.sizes[sizeKey] || this.sizes.md;
  },

  /** Get width only (for backward compat with getCardWidth functions) */
  getWidth(type: string): number {
    return this.getByType(type).width;
  },
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

  /** Panel body - scrollable content (px-3 matches toolbar padding for alignment) */
  body: 'flex-1 overflow-y-auto scrollbar-thin px-3',

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
  // Border opacity levels - Unified 5-level scale (matches opacity.border)
  border: {
    subtle: 'rgba(255, 255, 255, 0.06)',  // 6%  - Panel dividers
    light: 'rgba(255, 255, 255, 0.10)',   // 10% - Card borders
    medium: 'rgba(255, 255, 255, 0.12)',  // 12% - Glass, modal borders
    strong: 'rgba(255, 255, 255, 0.15)',  // 15% - Hover, elevated
    heavy: 'rgba(255, 255, 255, 0.20)',   // 20% - Active, pressed
    active: 'rgba(255, 255, 255, 0.30)',  // 30% - High emphasis (special)
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
 *
 * Border scale (unified with opacity.border):
 *   subtle=6%, light=10%, medium=12%, strong=15%, heavy=20%
 */
export const glassClasses = {
  subtle: 'bg-[hsl(240,8%,5%)] backdrop-blur-sm border border-white/[0.06]',
  light: 'bg-[hsl(240,8%,5%)] backdrop-blur-md border border-white/10',
  medium: 'bg-[hsl(240,6%,8%)] backdrop-blur-xl border border-white/[0.12]',
  heavy: 'bg-[hsl(240,5%,12%)] backdrop-blur-xl border border-white/[0.15]',
  floating: 'bg-[hsl(240,6%,8%)] backdrop-blur-xl border border-white/[0.15] rounded-2xl shadow-2xl shadow-black/50 ring-1 ring-white/4 ring-inset',
  /** Modal variant - fully opaque dark background for better perf and consistency */
  modal: 'bg-[#0d0d12] border border-white/[0.12] rounded-2xl shadow-2xl shadow-black/60',

  // Hover variants - each moves up one level in the opacity scale
  subtleHover: 'hover:bg-white/[0.04] hover:border-white/10',
  lightHover: 'hover:bg-white/[0.06] hover:border-white/[0.12]',
  mediumHover: 'hover:bg-white/[0.08] hover:border-white/[0.15]',
  heavyHover: 'hover:bg-white/[0.12] hover:border-white/20',
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
  /** Modal header - title area with close button (standard: 16px/12px) */
  header: 'flex items-center justify-between px-4 py-3 border-b border-white/[0.08]',
  /** Modal header - large picker modals (24px/16px) */
  headerLg: 'flex items-center justify-between px-6 py-4 border-b border-white/[0.06]',
  /** Modal body - scrollable content area */
  body: 'overflow-y-auto',
  /** Modal body - large picker modals with padding */
  bodyLg: 'overflow-y-auto p-6',
  /** Modal footer - action buttons area (standard: 16px/12px) */
  footer: 'px-4 py-3 border-t border-white/[0.08]',
  /** Modal footer - large picker modals (24px/12px) */
  footerLg: 'px-6 py-3 border-t border-white/[0.06]',
  /** Close button style */
  closeButton: 'w-8 h-8 rounded-lg flex items-center justify-center text-white/40 hover:text-white/80 hover:bg-white/[0.06] transition-colors duration-150',
} as const;

// ============================================================================
// OVERLAY / COMMAND PALETTE DESIGN SYSTEM
// ============================================================================

/**
 * Overlay design tokens - Unified system for command palette–style modals
 *
 * Used by: CommandPalette (⌘K), AiSearchOverlay (⌘J), KeyboardHelpPanel (?)
 *
 * All three modals share:
 * - Vertically centered positioning
 * - Raycast-style width: max-w-2xl (672px)
 * - Inline search header (icon + transparent input + shortcut badge)
 * - Category section headers (uppercase, muted)
 * - Dark footer bar with keyboard hints
 * - 65vh max body height
 * - animate-scale-in entrance
 */
export const overlayClasses = {
  /** Container position - centered both horizontally and vertically */
  position: 'items-center justify-center',
  /** Modal size - Raycast-style: max-w-2xl = 672px */
  size: 'lg' as const,
  /** Max width class for direct use (shared with pickers) */
  maxWidth: 'max-w-2xl',
  /** Search header row - generous padding, stronger border */
  searchHeader: 'flex items-center px-4 sm:px-5 py-3.5 sm:py-4 border-b border-white/[0.10]',
  /** Search input - transparent inline, larger text for bold presence */
  searchInput: [
    'flex-1 bg-transparent text-white placeholder-white/35',
    'text-base sm:text-lg outline-none border-none ring-0',
    'focus:outline-none focus:ring-0',
  ].join(' '),
  /** Category/section header in command list */
  sectionHeader: 'px-3 py-2.5 text-[11px] font-semibold text-white/45 uppercase tracking-widest',
  /** Command row - base layout with generous sizing */
  rowBase: [
    'w-full flex items-center px-3 sm:px-4 py-2.5 sm:py-3 rounded-xl',
    'transition-all duration-150',
    'outline-none ring-0 focus:outline-none focus:ring-0',
  ].join(' '),
  /** Row idle/hover state */
  rowIdle: 'hover:bg-white/[0.04] border border-transparent',
  /** Row selected/active state - bold novanet accent with glow */
  rowSelected: 'bg-novanet-500/15 border border-novanet-500/30 shadow-[0_0_20px_rgba(139,92,246,0.12)]',
  /** Row icon container - base (larger for bold layout) */
  rowIconBase: 'w-9 h-9 rounded-lg flex items-center justify-center shrink-0 transition-colors duration-150',
  /** Row icon idle */
  rowIconIdle: 'bg-white/[0.05] text-white/50',
  /** Row icon selected - stronger accent */
  rowIconSelected: 'bg-novanet-500/25 text-novanet-300',
  /** Unified body maxHeight - generous for bold centered modal */
  bodyMaxHeight: '65vh',
  /** Footer - subtle dark bar with top border */
  footer: 'px-4 sm:px-5 py-3 bg-black/25 border-t border-white/[0.06]',
  /** Footer content - centered keyboard hint row */
  footerContent: 'flex items-center justify-center text-xs text-white/40',
  /** Entrance animation - Raycast-style spring */
  animation: 'animate-overlay-enter',
  /** Body content staggered entrance */
  contentAnimation: 'animate-overlay-content',
} as const;

// ============================================================================
// PICKER MODAL DESIGN SYSTEM
// ============================================================================

/**
 * Picker design tokens - Unified system for grid-based picker modals
 *
 * Used by: ViewPicker, LocalePicker, ProjectPicker
 *
 * All three pickers share:
 * - Full-width centered layout with consistent sizing
 * - Icon box + title + subtitle header
 * - Inline search bar
 * - Grid of cards with keyboard navigation
 * - Keyboard hints footer
 * - Raycast-style spring animation
 */
export const pickerClasses = {
  /** Container - full-screen centered with padding */
  container: 'fixed inset-0 z-50 flex items-center justify-center p-4',
  /** Backdrop - Raycast-style blur ramp */
  backdrop: 'fixed inset-0 bg-black/60 animate-overlay-backdrop',
  /** Modal shell - glass morphism with spring entrance */
  shell: [
    'relative w-full overflow-hidden flex flex-col rounded-2xl',
    'bg-[#0d0d12] border border-white/[0.12] shadow-2xl shadow-black/60',
    'animate-overlay-enter',
  ].join(' '),
  /** Default size - Raycast-style: max-w-2xl = 672px (same as overlays) */
  sizeDefault: 'max-w-2xl',
  /** Large size for grids with many items (3 columns) */
  sizeLarge: 'max-w-4xl',
  /** Compact size for simpler pickers */
  sizeCompact: 'max-w-xl',
  /** Max height */
  maxHeight: 'max-h-[80vh]',
  /** Header row - icon box + title + close button */
  header: 'flex items-center justify-between px-5 sm:px-6 py-4 border-b border-white/[0.08]',
  /** Header icon box */
  headerIconBox: 'w-10 h-10 rounded-xl border border-white/[0.10] flex items-center justify-center',
  /** Header title */
  headerTitle: 'text-base font-semibold text-white',
  /** Header subtitle */
  headerSubtitle: 'text-xs text-white/40 mt-0.5',
  /** Close button */
  closeButton: 'p-2 rounded-lg hover:bg-white/10 transition-colors text-white/50 hover:text-white',
  /** Search bar row */
  searchBar: 'flex items-center gap-3 px-5 sm:px-6 py-3.5 border-b border-white/[0.08]',
  /** Search input (shared with overlayClasses pattern) */
  searchInput: 'flex-1 bg-transparent text-white placeholder-white/35 text-base outline-none border-none ring-0 focus:outline-none focus:ring-0',
  /** Grid container - min-h-0 enables flex shrink for scrolling */
  grid: 'flex-1 min-h-0 overflow-y-auto scrollbar-thin p-5 sm:p-6 animate-overlay-content',
  /** Card base */
  cardBase: [
    'flex flex-col items-center justify-center p-4 sm:p-5 rounded-xl',
    'gap-2 border transition-all duration-150 relative',
    'hover:scale-[1.02] active:scale-[0.98]',
  ].join(' '),
  /** Card idle state */
  cardIdle: 'bg-white/[0.02] border-white/[0.08] hover:bg-white/[0.05] hover:border-white/[0.15] text-white/90 hover:text-white',
  /** Card focused state (keyboard navigation) */
  cardFocused: 'bg-white/[0.06] border-white/20 text-white',
  /** Card "all" variant (dashed border) */
  cardAll: 'border-dashed border-white/15',
  /** Card entrance animation */
  cardAnimation: 'animate-picker-card',
  /** Empty state */
  emptyState: 'text-center py-12 text-white/40',
  /** Footer bar - count + keyboard hints */
  footer: 'px-5 sm:px-6 py-3 border-t border-white/[0.06] bg-black/25',
  /** Footer content layout */
  footerContent: 'flex items-center justify-between text-xs text-white/50',
} as const;

/** Max cards that receive stagger delay (first 6 get 30ms increments) */
const MAX_STAGGER = 6;

/**
 * Get stagger class for picker card animation
 * Cards 0-5 get incremental delay (30ms each), cards 6+ animate immediately
 */
export function getCardStagger(index: number): string {
  if (index < 0 || index >= MAX_STAGGER) return '';
  return `stagger-${index + 1}`;
}

// ============================================================================
// UNIFIED SIDEBAR DESIGN SYSTEM
// ============================================================================

/**
 * Sidebar design tokens - Unified system for all sidebar tabs
 *
 * shadcn/ui sidebar-inspired compact layout:
 * - Row height: h-8 (32px) - shadcn SidebarMenuButton standard
 * - Row gap: gap-2 (8px) between elements
 * - Row padding: px-2 (8px) horizontal
 * - Icon: w-4 h-4 (16px) inline, no box
 * - Badge: plain text, no pill
 * - Row radius: rounded-md (6px)
 * - Section indent: pl-4 (16px) - shadcn SidebarMenuSub
 *
 * Used by: Schema Browser, Views, Nodes, Relationships tabs
 */
export const sidebarTokens = {
  // ─────────────────────────────────────────────────────────────────────────
  // ROW - shadcn compact: h-8, px-2, gap-2, rounded-md
  // ─────────────────────────────────────────────────────────────────────────
  row: {
    /** Base row - minimal, compact */
    base: [
      'group relative w-full flex items-center',
      'h-8 px-2 gap-2 rounded-md',
      'bg-transparent',
      'transition-colors duration-100',
      'hover:bg-white/[0.04]',
      'focus-visible:outline-none focus-visible:bg-white/[0.06]',
      'active:bg-white/[0.06]',
    ].join(' '),
    /** Selected state - subtle background only, no border accent */
    selected: 'bg-white/[0.06]',
    /** Disabled state */
    disabled: 'opacity-50 cursor-not-allowed pointer-events-none',
  },

  // ─────────────────────────────────────────────────────────────────────────
  // ICON BOX - Inline 14px, matches iconSizes.sm
  // ─────────────────────────────────────────────────────────────────────────
  iconBox: {
    /** Standard: 14x14px inline icon (matches iconSizes.sm) */
    base: 'flex-shrink-0 flex items-center justify-center w-3.5 h-3.5 transition-colors duration-100',
    /** Small: 12x12px */
    sm: 'flex-shrink-0 flex items-center justify-center w-3 h-3 transition-colors duration-100',
    /** Large: 40x40px (panel headers only) */
    lg: 'flex-shrink-0 flex items-center justify-center w-10 h-10 rounded-xl transition-colors duration-100',
  },

  // ─────────────────────────────────────────────────────────────────────────
  // CHECKBOX - Compact 14px
  // ─────────────────────────────────────────────────────────────────────────
  checkbox: {
    /** Checkbox: 14x14px - proportional to h-8 rows */
    base: 'w-3.5 h-3.5 rounded border flex items-center justify-center transition-colors duration-150 flex-shrink-0',
    /** Unchecked state */
    unchecked: 'border-white/25 bg-transparent',
    /** Checked state (color set via style) */
    checked: 'border-transparent',
    /** Spacer when checkbox is hidden */
    spacer: 'w-3.5 flex-shrink-0',
  },

  // ─────────────────────────────────────────────────────────────────────────
  // LABEL - text-sm standard
  // ─────────────────────────────────────────────────────────────────────────
  label: {
    /** Row label text */
    base: 'text-sm font-medium transition-colors duration-150 flex-1 text-left truncate',
    /** Selected state */
    selected: 'text-white',
    /** Unselected state */
    unselected: 'text-white/60 group-hover:text-white/80',
    /** Section label (category header) */
    section: 'text-[11px] font-semibold uppercase tracking-wider',
  },

  // ─────────────────────────────────────────────────────────────────────────
  // BADGE - Plain text, no pill background
  // ─────────────────────────────────────────────────────────────────────────
  badge: {
    /** Count: plain text, right-aligned */
    count: 'text-[10px] tabular-nums flex-shrink-0 transition-colors duration-150',
    /** Count selected */
    countSelected: 'text-white/60',
    /** Count unselected */
    countUnselected: 'text-white/30 group-hover:text-white/50',
    /** Shortcut badge: plain colored text, same weight as count */
    shortcut: 'text-[10px] font-medium tabular-nums flex-shrink-0 transition-colors duration-150',
  },

  // ─────────────────────────────────────────────────────────────────────────
  // SECTION - Compact with shadcn indent
  // ─────────────────────────────────────────────────────────────────────────
  section: {
    /** Section container - tighter spacing between sections */
    container: 'mt-4 first:mt-0',
    /** Section header row - compact */
    header: [
      'flex items-center w-full py-1 px-2 gap-2',
      'rounded-md',
      'transition-colors duration-100',
      'hover:bg-white/[0.03]',
    ].join(' '),
    /** Section content - shadcn SidebarMenuSub indent (pl-4), minimal row gap */
    content: 'mt-0.5 pl-4 space-y-px overflow-hidden transition-all duration-150',
    /** Content expanded state */
    contentExpanded: 'max-h-[2000px] opacity-100',
    /** Content collapsed state */
    contentCollapsed: 'max-h-0 opacity-0',
    /** Section count text */
    count: 'text-[10px] tabular-nums text-white/40',
  },

  // ─────────────────────────────────────────────────────────────────────────
  // CHEVRON - Standard 16px
  // ─────────────────────────────────────────────────────────────────────────
  chevron: {
    /** Chevron icon */
    base: 'w-4 h-4 text-white/30 transition-transform duration-150 ml-auto',
    /** Collapsed state (rotated) */
    collapsed: '-rotate-90',
  },

  // ─────────────────────────────────────────────────────────────────────────
  // TREE - Tight section spacing
  // ─────────────────────────────────────────────────────────────────────────
  tree: {
    /** Tree container - compact */
    container: 'space-y-1 pb-3',
  },

  // ─────────────────────────────────────────────────────────────────────────
  // PROGRESS BAR - For data counts
  // ─────────────────────────────────────────────────────────────────────────
  progressBar: {
    /** Progress bar container */
    container: 'w-12 flex-shrink-0',
  },

  // ─────────────────────────────────────────────────────────────────────────
  // HEADER - Action bar with execute button
  // ─────────────────────────────────────────────────────────────────────────
  header: {
    /** Header container */
    container: 'flex items-center justify-between px-2 mb-2',
    /** Title container */
    title: 'flex items-center gap-2',
    /** Selection badge */
    selectionBadge: 'px-1.5 py-0.5 rounded-full bg-emerald-500/20 text-emerald-400 text-[10px] font-semibold animate-in fade-in duration-200',
    /** Execute button base */
    executeButton: 'p-1.5 rounded-md transition duration-200',
    /** Execute button enabled */
    executeEnabled: 'text-emerald-400 hover:text-emerald-300 hover:bg-emerald-500/10 hover:scale-110',
    /** Execute button disabled */
    executeDisabled: 'text-white/40 cursor-not-allowed',
  },
} as const;

// Re-export v9 visual encoding (ADR-014)
// Colors generated from organizing-principles.yaml (source of truth)
export {
  // Types
  type ColorTokens,
  type RealmKey,
  type LayerKey,
  type TraitKey,
  type ArcFamilyKey,
  // Realm colors (replaces realmAccents)
  REALM_COLORS,
  REALM_DISPLAY_NAMES,
  getRealmColor,
  // Layer colors
  LAYER_COLORS,
  LAYER_DISPLAY_NAMES,
  getLayerColor,
  // Trait colors
  TRAIT_COLORS,
  TRAIT_DISPLAY_NAMES,
  getTraitColor,
  // Arc family colors
  ARC_FAMILY_COLORS,
  ARC_FAMILY_DISPLAY_NAMES,
  getArcFamilyColor,
  // Utility
  getFacetHex,
} from './colors';
export { TRAIT_STYLES, getTraitStyle, getTraitBorderCSS } from './traitStyles';

// ============================================================================
// EXPORT ALL
// ============================================================================

export const tokens = {
  // Core design tokens
  spacing,
  radius,
  shadows,
  zIndex,
  durations,
  easing,
  transitions,
  controls,
  opacity,
  defaultColors,
  iconSizes,
  nodeCardSizes,
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
  overlayClasses,
  pickerClasses,
  // Sidebar design system
  sidebarTokens,
  iconButtonClasses,
} as const;

export default tokens;
