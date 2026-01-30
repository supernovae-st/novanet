/**
 * NodeStyles - Memoized Style Factory
 *
 * Caches CSSProperties objects by their input parameters to prevent
 * unnecessary object allocations on every render. This reduces GC pressure
 * and allows React to skip re-renders via referential equality checks.
 *
 * Problem: TurboNode had ~13 inline style objects, creating 247,000
 * object allocations per frame with 19k nodes.
 *
 * Solution: Return the same object reference for identical inputs.
 *
 * @example
 * // First call creates and caches the style object
 * const style1 = getNodeContainerStyle(200, 100, false, false);
 *
 * // Second call with same inputs returns the cached reference
 * const style2 = getNodeContainerStyle(200, 100, false, false);
 *
 * style1 === style2 // true (same reference)
 */

import type { CSSProperties } from 'react';

// Style caches - Map provides O(1) lookup
const containerCache = new Map<string, CSSProperties>();
const headerCache = new Map<string, CSSProperties>();
const contentCache = new Map<string, CSSProperties>();
const gradientBorderCache = new Map<string, CSSProperties>();
const iconCache = new Map<string, CSSProperties>();
const iconContainerCache = new Map<string, CSSProperties>();
const handleCache = new Map<string, CSSProperties>();
const badgeCache = new Map<string, CSSProperties>();
const innerCardCache = new Map<string, CSSProperties>();

/**
 * Creates a unique cache key from input arguments.
 * Uses delimiter to prevent collision between similar numeric values.
 * e.g., (20, 100) vs (201, 0) would collide with naive concatenation.
 */
function cacheKey(...args: (string | number | boolean)[]): string {
  return args.join('|');
}

/**
 * Gets a memoized container style object.
 *
 * @param width - Node width in pixels
 * @param height - Node height in pixels
 * @param isDimmed - Whether the node is dimmed (focus/hover mode)
 * @param isSelected - Whether the node is currently selected
 * @returns Cached CSSProperties object
 */
export function getNodeContainerStyle(
  width: number,
  height: number,
  isDimmed: boolean,
  isSelected: boolean
): CSSProperties {
  const key = cacheKey(width, height, isDimmed, isSelected);

  if (!containerCache.has(key)) {
    containerCache.set(key, {
      width,
      height,
      opacity: isDimmed ? 0.3 : 1,
      transform: isSelected ? 'scale(1.02)' : undefined,
      transition: 'opacity 0.2s ease, transform 0.2s ease',
    });
  }

  return containerCache.get(key)!;
}

/**
 * Gets a memoized header style object.
 *
 * @param primaryColor - The primary color for the header background
 * @param isHovered - Whether the node is currently hovered
 * @returns Cached CSSProperties object
 */
export function getNodeHeaderStyle(
  primaryColor: string,
  isHovered: boolean
): CSSProperties {
  const key = cacheKey(primaryColor, isHovered);

  if (!headerCache.has(key)) {
    headerCache.set(key, {
      backgroundColor: primaryColor,
      opacity: isHovered ? 1 : 0.9,
    });
  }

  return headerCache.get(key)!;
}

/**
 * Gets a memoized content style object.
 *
 * @param isDimmed - Whether the content is dimmed
 * @returns Cached CSSProperties object
 */
export function getNodeContentStyle(isDimmed: boolean): CSSProperties {
  const key = cacheKey(isDimmed);

  if (!contentCache.has(key)) {
    contentCache.set(key, {
      opacity: isDimmed ? 0.5 : 1,
    });
  }

  return contentCache.get(key)!;
}

// =============================================================================
// TurboNode-specific Style Factories
// =============================================================================

/**
 * Gets a memoized gradient border style for TurboNode.
 *
 * @param primaryColor - Primary color for gradient
 * @param secondaryColor - Secondary color for gradient
 * @param isSelected - Whether the node is selected
 * @param isHovered - Whether the node is hovered
 * @returns Cached CSSProperties object
 */
export function getGradientBorderStyle(
  primaryColor: string,
  secondaryColor: string,
  isSelected: boolean,
  isHovered: boolean
): CSSProperties {
  const key = cacheKey(primaryColor, secondaryColor, isSelected, isHovered);

  if (!gradientBorderCache.has(key)) {
    gradientBorderCache.set(key, {
      background: isSelected
        ? `linear-gradient(135deg, ${primaryColor}, ${secondaryColor}, ${primaryColor})`
        : isHovered
          ? `linear-gradient(135deg, ${primaryColor}, ${secondaryColor})`
          : `linear-gradient(135deg, ${primaryColor}, ${secondaryColor}90)`,
      boxShadow: isSelected
        ? `0 0 40px 8px ${primaryColor}70, 0 0 80px 16px ${primaryColor}40, 0 0 120px 24px ${primaryColor}20`
        : isHovered
          ? `0 0 30px 6px ${primaryColor}50, 0 0 60px 12px ${primaryColor}25`
          : `0 0 20px 4px ${primaryColor}40, 0 0 40px 8px ${primaryColor}20`,
    });
  }

  return gradientBorderCache.get(key)!;
}

/**
 * Gets a memoized icon style for TurboNode.
 *
 * @param color - Icon color
 * @param isSelected - Whether the node is selected
 * @returns Cached CSSProperties object
 */
export function getIconStyle(
  color: string,
  isSelected: boolean
): CSSProperties {
  const key = cacheKey(color, isSelected);

  if (!iconCache.has(key)) {
    iconCache.set(key, {
      color,
      filter: `drop-shadow(0 0 ${isSelected ? '10px' : '6px'} ${color}70)`,
    });
  }

  return iconCache.get(key)!;
}

/**
 * Gets a memoized icon container gradient style.
 *
 * @param primaryColor - Primary color for gradient
 * @param secondaryColor - Secondary color for gradient
 * @returns Cached CSSProperties object
 */
export function getIconContainerStyle(
  primaryColor: string,
  secondaryColor: string
): CSSProperties {
  const key = cacheKey(primaryColor, secondaryColor);

  if (!iconContainerCache.has(key)) {
    iconContainerCache.set(key, {
      background: `linear-gradient(135deg, ${primaryColor}25, ${secondaryColor}15)`,
    });
  }

  return iconContainerCache.get(key)!;
}

/**
 * Gets a memoized handle style for React Flow handles.
 *
 * @param primaryColor - Handle color
 * @param isSelected - Whether the node is selected
 * @param isSolid - Whether handle is solid (target) or hollow (source)
 * @returns Cached CSSProperties object
 */
export function getHandleStyle(
  primaryColor: string,
  isSelected: boolean,
  isSolid: boolean
): CSSProperties {
  const key = cacheKey(primaryColor, isSelected, isSolid);

  if (!handleCache.has(key)) {
    handleCache.set(key, {
      backgroundColor: isSolid ? primaryColor : 'transparent',
      borderColor: primaryColor,
      boxShadow: isSelected ? `0 0 8px ${primaryColor}` : undefined,
    });
  }

  return handleCache.get(key)!;
}

/**
 * Gets a memoized badge/label style.
 *
 * @param primaryColor - Primary color for background/border
 * @returns Cached CSSProperties object
 */
export function getBadgeStyle(primaryColor: string): CSSProperties {
  const key = cacheKey(primaryColor);

  if (!badgeCache.has(key)) {
    badgeCache.set(key, {
      background: `${primaryColor}15`,
      borderColor: `${primaryColor}40`,
    });
  }

  return badgeCache.get(key)!;
}

/**
 * Gets a memoized category badge style.
 *
 * @param primaryColor - Primary color
 * @returns Cached CSSProperties object
 */
export function getCategoryBadgeStyle(primaryColor: string): CSSProperties {
  const key = cacheKey('category', primaryColor);

  if (!badgeCache.has(key)) {
    badgeCache.set(key, {
      background: `${primaryColor}15`,
      borderColor: `${primaryColor}35`,
      color: primaryColor,
    });
  }

  return badgeCache.get(key)!;
}

/**
 * Gets a memoized status dot style.
 *
 * @param primaryColor - Dot color
 * @returns Cached CSSProperties object
 */
export function getStatusDotStyle(primaryColor: string): CSSProperties {
  const key = cacheKey('status', primaryColor);

  if (!badgeCache.has(key)) {
    badgeCache.set(key, {
      background: primaryColor,
      boxShadow: `0 0 8px ${primaryColor}`,
    });
  }

  return badgeCache.get(key)!;
}

/**
 * Gets a memoized inner card background style.
 *
 * @param isSelected - Whether the node is selected
 * @returns Cached CSSProperties object
 */
export function getInnerCardStyle(isSelected: boolean): CSSProperties {
  const key = cacheKey(isSelected);

  if (!innerCardCache.has(key)) {
    innerCardCache.set(key, {
      backgroundColor: isSelected ? '#1a1a24' : '#18181f',
    });
  }

  return innerCardCache.get(key)!;
}

// =============================================================================
// Static Styles (no parameters, constant references)
// =============================================================================

/**
 * Static style for root container transition.
 * Single object reference, never recreated.
 */
export const ROOT_TRANSITION_STYLE: CSSProperties = {
  transition: 'transform 200ms ease-out, opacity 200ms ease-out',
};

/**
 * Static style for display name container.
 * Single object reference, never recreated.
 */
export const DISPLAY_NAME_CONTAINER_STYLE: CSSProperties = {
  background: 'rgba(255, 255, 255, 0.05)',
  borderColor: 'rgba(255, 255, 255, 0.1)',
};

// =============================================================================
// Cache Management
// =============================================================================

/**
 * Clears all style caches.
 *
 * Use cases:
 * - Testing (clear between tests for isolation)
 * - Memory management (if cache grows too large)
 * - Theme changes (invalidate cached styles)
 */
export function clearStyleCaches(): void {
  containerCache.clear();
  headerCache.clear();
  contentCache.clear();
  gradientBorderCache.clear();
  iconCache.clear();
  iconContainerCache.clear();
  handleCache.clear();
  badgeCache.clear();
  innerCardCache.clear();
}

/**
 * Returns the current cache sizes for debugging/monitoring.
 */
export function getStyleCacheStats(): {
  container: number;
  header: number;
  content: number;
  gradientBorder: number;
  icon: number;
  iconContainer: number;
  handle: number;
  badge: number;
  innerCard: number;
  total: number;
} {
  const total = containerCache.size + headerCache.size + contentCache.size +
    gradientBorderCache.size + iconCache.size + iconContainerCache.size +
    handleCache.size + badgeCache.size + innerCardCache.size;

  return {
    container: containerCache.size,
    header: headerCache.size,
    content: contentCache.size,
    gradientBorder: gradientBorderCache.size,
    icon: iconCache.size,
    iconContainer: iconContainerCache.size,
    handle: handleCache.size,
    badge: badgeCache.size,
    innerCard: innerCardCache.size,
    total,
  };
}
