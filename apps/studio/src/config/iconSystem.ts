/**
 * NovaNet Icon Design System
 *
 * A semantic icon system aligned with NovaNet Core concepts.
 * All icons use lucide-react for consistency.
 *
 * Design Principles:
 * 1. Semantic Meaning - Icons represent concepts, not just visuals
 * 2. Consistency - Same concept = same icon everywhere
 * 3. Color Coding - Categories have consistent color themes
 * 4. Hierarchy - Primary icons are larger/bolder than secondary
 */

import {
  Atom,
  GitBranch,
  Database,
  Network,
  Play,
  RefreshCw,
  RotateCcw,
  Copy,
  Check,
  X,
  ChevronDown,
  ChevronRight,
  ArrowRight,
  Loader2,
  AlertCircle,
  AlertTriangle,
  Search,
  Code,
  Table2,
  FileJson,
  Pencil,
  Trash2,
  Send,
  Keyboard,
  Command,
  Mouse,
  Expand,
  Minimize2,
  Settings,
  Eye,
  EyeOff,
  Plus,
  Bookmark,
  Hash,
  Link2,
  Crosshair,
  Clock,
} from 'lucide-react';

// =============================================================================
// SEMANTIC ICON CATEGORIES
// =============================================================================

/**
 * Graph Entity Icons
 * Core concepts for graph database visualization
 */
export const GRAPH_ICONS = {
  /** Node/Vertex - Atom represents interconnected entities */
  node: Atom,
  /** Relationship/Edge - GitBranch represents connections */
  relationship: GitBranch,
  /** Link/Connection - Link2 for edge connections */
  link: Link2,
  /** Database - The whole database concept */
  database: Database,
  /** Network - Full graph/network view */
  network: Network,
} as const;

/**
 * Action Icons
 * User interactions and operations
 */
export const ACTION_ICONS = {
  /** Execute/Run a query or action */
  execute: Play,
  /** Refresh/Reload data */
  refresh: RefreshCw,
  /** Reset/Clear - rotates counter-clockwise */
  reset: RotateCcw,
  /** Copy to clipboard */
  copy: Copy,
  /** Close/Dismiss */
  close: X,
  /** Expand/Maximize */
  expand: Expand,
  /** Collapse/Minimize */
  collapse: Minimize2,
  /** Edit/Modify */
  edit: Pencil,
  /** Delete/Remove */
  delete: Trash2,
  /** Send/Submit */
  send: Send,
  /** Search/Find */
  search: Search,
  /** Add/Create new */
  add: Plus,
  /** Save/Bookmark */
  save: Bookmark,
  /** Show */
  show: Eye,
  /** Hide */
  hide: EyeOff,
  /** Settings/Configure */
  settings: Settings,
  /** Target/Focus on specific element */
  target: Crosshair,
} as const;

/**
 * Status Icons
 * Feedback and state indicators
 */
export const STATUS_ICONS = {
  /** Loading/Processing */
  loading: Loader2,
  /** Success/Complete */
  success: Check,
  /** Error/Failed */
  error: AlertCircle,
  /** Warning/Caution */
  warning: AlertTriangle,
  /** Time/Duration */
  time: Clock,
} as const;

/**
 * Navigation Icons
 * UI navigation and structure
 */
export const NAV_ICONS = {
  /** Expand section */
  chevronDown: ChevronDown,
  /** Collapse section / Navigate right */
  chevronRight: ChevronRight,
  /** Navigate/Flow right (arrows) */
  arrowRight: ArrowRight,
  /** Keyboard shortcuts */
  keyboard: Keyboard,
  /** Command key (Mac) */
  command: Command,
  /** Mouse/Pointer interactions */
  mouse: Mouse,
} as const;

/**
 * Content Type Icons
 * Different data/view formats
 */
export const CONTENT_ICONS = {
  /** Source code */
  code: Code,
  /** Table/Grid view */
  table: Table2,
  /** JSON data */
  json: FileJson,
  /** ID/Identifier/Hash */
  id: Hash,
} as const;

// =============================================================================
// COLOR THEMES
// =============================================================================

/**
 * Semantic color themes for icon categories
 * Consistent across the entire application
 */
export const ICON_COLORS = {
  /** Nodes - Emerald green (growth, entities) */
  node: {
    primary: '#10b981',    // emerald-500
    light: '#34d399',      // emerald-400
    muted: 'text-emerald-400',
    mutedHover: 'text-emerald-300',
    bg: 'bg-emerald-500/20',
    border: 'border-emerald-500/30',
  },
  /** Relationships - Violet purple (connections, links) */
  relationship: {
    primary: '#8b5cf6',    // violet-500
    light: '#a78bfa',      // violet-400
    muted: 'text-violet-400',
    mutedHover: 'text-violet-300',
    bg: 'bg-violet-500/20',
    border: 'border-violet-500/30',
  },
  /** AI Features - Amber/Gold (magic, intelligence) */
  ai: {
    primary: '#f59e0b',    // amber-500
    light: '#fbbf24',      // amber-400
    muted: 'text-amber-400',
    mutedHover: 'text-amber-300',
    bg: 'bg-amber-500/20',
    border: 'border-amber-500/30',
  },
  /** Database/System - NovaNet brand blue */
  system: {
    primary: '#3b82f6',    // blue-500 / novanet
    light: '#60a5fa',      // blue-400
    muted: 'text-novanet-400',
    mutedHover: 'text-novanet-300',
    bg: 'bg-novanet-500/20',
    border: 'border-novanet-500/30',
  },
  /** Success - Green */
  success: {
    primary: '#22c55e',    // green-500
    muted: 'text-green-400',
    bg: 'bg-green-500/20',
  },
  /** Error - Red */
  error: {
    primary: '#ef4444',    // red-500
    muted: 'text-red-400',
    bg: 'bg-red-500/20',
  },
  /** Warning - Yellow */
  warning: {
    primary: '#eab308',    // yellow-500
    muted: 'text-yellow-400',
    bg: 'bg-yellow-500/20',
  },
  /** Neutral - White/Gray for general UI */
  neutral: {
    primary: '#ffffff',
    muted: 'text-white/40',
    mutedHover: 'text-white/60',
  },
} as const;

// =============================================================================
// ICON SIZE PRESETS
// =============================================================================

/**
 * Consistent icon sizes across the app
 */
export const ICON_SIZES = {
  /** Extra small - inline text, badges */
  xs: 'w-3 h-3',
  /** Small - buttons, list items */
  sm: 'w-3.5 h-3.5',
  /** Medium - default for most uses */
  md: 'w-4 h-4',
  /** Large - headers, hero elements */
  lg: 'w-5 h-5',
  /** Extra large - feature highlights */
  xl: 'w-6 h-6',
} as const;
