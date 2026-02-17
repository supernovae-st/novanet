/**
 * Edge Utility Functions
 *
 * Pure utility functions for edge rendering:
 * - formatRelationType: Format relation type for display
 * - getSmartLabel: Get smart abbreviated label based on available space
 * - getRelationIcon: Get icon for relation type
 * - getNodeIntersection: Calculate edge intersection with node bounds
 * - generateCurvedPath: Generate curved SVG path between two points
 * - getRandomDelay: Generate deterministic random delay based on edge ID
 */

/**
 * Format relation type for display
 */
export function formatRelationType(type: string): string {
  return type
    .replace(/_/g, ' ')
    .toLowerCase()
    .replace(/\b\w/g, (c) => c.toUpperCase());
}

/**
 * Get shortened version of relation type (e.g., HAS_CONCEPT -> Has Con)
 */
function getShortLabel(type: string): string {
  const parts = type.split('_');
  if (parts.length === 1) {
    // Single word: capitalize first letter, lowercase rest, truncate
    const word = type.toLowerCase();
    return word.charAt(0).toUpperCase() + word.slice(1, 6);
  }
  // Multi-word: format each part nicely, truncate after first
  return parts
    .map((word, i) => {
      const lower = word.toLowerCase();
      const truncated = i === 0 ? lower : lower.slice(0, 3);
      return truncated.charAt(0).toUpperCase() + truncated.slice(1);
    })
    .join(' ');
}

/**
 * Simple arrow indicators for relation direction (no emojis)
 * Used only at very low zoom (icon-only mode)
 */
const RELATION_ARROWS: Record<string, string> = {
  DEFAULT: '→',
};

/**
 * Get simple arrow for relation type (no emojis)
 */
export function getRelationIcon(): string {
  return RELATION_ARROWS.DEFAULT;
}

/**
 * Get smart label based on available space (edge length)
 *
 * @param relationType - The relationship type (e.g., "HAS_CONCEPT")
 * @param edgeLength - Length of the edge in pixels
 * @returns Object with icon and text for the label
 */
export function getSmartLabel(
  relationType: string,
  edgeLength: number
): { icon: string; text: string } {
  const icon = getRelationIcon();

  // Simple space-based logic: use edge length directly
  // Labels scale inversely with zoom, so actual available space = edgeLength
  // Only abbreviate for very short edges
  let text: string;

  if (edgeLength > 80) {
    // Plenty of room - show full formatted text
    text = formatRelationType(relationType);
  } else if (edgeLength > 40) {
    // Medium space - show short version
    text = getShortLabel(relationType);
  } else {
    // Very short edge - just show icon, no text
    text = '';
  }

  return { icon, text };
}

/**
 * Calculate the intersection point between a line and an ellipse
 */
export function getNodeIntersection(
  nodeCenter: { x: number; y: number },
  nodeWidth: number,
  nodeHeight: number,
  targetPoint: { x: number; y: number },
  padding: number = 8
): { x: number; y: number } {
  const dx = targetPoint.x - nodeCenter.x;
  const dy = targetPoint.y - nodeCenter.y;

  if (dx === 0 && dy === 0) {
    return { x: nodeCenter.x, y: nodeCenter.y };
  }

  const angle = Math.atan2(dy, dx);
  const halfWidth = nodeWidth / 2 + padding;
  const halfHeight = nodeHeight / 2 + padding;

  const cosAngle = Math.cos(angle);
  const sinAngle = Math.sin(angle);

  const t = Math.sqrt(1 / (
    (cosAngle * cosAngle) / (halfWidth * halfWidth) +
    (sinAngle * sinAngle) / (halfHeight * halfHeight)
  ));

  return {
    x: nodeCenter.x + t * cosAngle,
    y: nodeCenter.y + t * sinAngle,
  };
}

/**
 * Check if a position has valid (non-NaN, finite) coordinates
 */
export function isValidPosition(pos: { x: number; y: number }): boolean {
  return (
    typeof pos.x === 'number' &&
    typeof pos.y === 'number' &&
    Number.isFinite(pos.x) &&
    Number.isFinite(pos.y)
  );
}

/**
 * Generate a curved path between two points
 * Returns empty string if positions are invalid to prevent SVG NaN errors
 */
export function generateCurvedPath(
  source: { x: number; y: number },
  target: { x: number; y: number }
): string {
  // Validate positions to prevent NaN in SVG path
  if (!isValidPosition(source) || !isValidPosition(target)) {
    return '';
  }

  const dx = target.x - source.x;
  const dy = target.y - source.y;
  const distance = Math.sqrt(dx * dx + dy * dy);

  // Handle zero distance - return straight line to prevent NaN from division
  if (distance === 0) {
    return `M ${source.x} ${source.y} L ${target.x} ${target.y}`;
  }

  const curveOffset = Math.min(distance * 0.25, 60);
  const perpX = -dy / distance;
  const perpY = dx / distance;

  const midX = (source.x + target.x) / 2;
  const midY = (source.y + target.y) / 2;

  const direction = (source.x + source.y) > (target.x + target.y) ? 1 : -1;
  const controlX = midX + perpX * curveOffset * direction * 0.5;
  const controlY = midY + perpY * curveOffset * direction * 0.5;

  return `M ${source.x} ${source.y} Q ${controlX} ${controlY} ${target.x} ${target.y}`;
}

/**
 * Generate REVERSED curved path (for animation flowing toward dependency)
 * Animation particles will flow from source → target visually
 * Returns empty string if positions are invalid to prevent SVG NaN errors
 */
export function generateReversedPath(
  source: { x: number; y: number },
  target: { x: number; y: number }
): string {
  // Validate positions to prevent NaN in SVG path
  if (!isValidPosition(source) || !isValidPosition(target)) {
    return '';
  }

  // Swap source and target for reversed animation direction
  const dx = source.x - target.x;
  const dy = source.y - target.y;
  const distance = Math.sqrt(dx * dx + dy * dy);

  if (distance === 0) {
    return `M ${target.x} ${target.y} L ${source.x} ${source.y}`;
  }

  const curveOffset = Math.min(distance * 0.25, 60);
  const perpX = -dy / distance;
  const perpY = dx / distance;

  const midX = (source.x + target.x) / 2;
  const midY = (source.y + target.y) / 2;

  const direction = (target.x + target.y) > (source.x + source.y) ? 1 : -1;
  const controlX = midX + perpX * curveOffset * direction * 0.5;
  const controlY = midY + perpY * curveOffset * direction * 0.5;

  // Path goes from TARGET to SOURCE (reversed)
  return `M ${target.x} ${target.y} Q ${controlX} ${controlY} ${source.x} ${source.y}`;
}

// =============================================================================
// Parallel Edge Path Generation (v11.6.1)
// =============================================================================

/**
 * Generate a curved path with perpendicular offset for parallel edges
 *
 * When multiple edges connect the same two nodes, each edge needs a
 * different curve to prevent overlap. This function generates paths
 * with varying perpendicular offsets.
 *
 * IMPORTANT: The perpendicular vector must be computed from a CANONICAL
 * direction (not the actual edge direction) to ensure edges A→B and B→A
 * get consistent offsets. Otherwise, their perpendicular vectors would
 * be opposite, causing offsets to cancel out and edges to overlap.
 *
 * @param source - Source point coordinates
 * @param target - Target point coordinates
 * @param index - Index of this edge in the parallel group (0-based)
 * @param total - Total number of parallel edges
 * @returns SVG path string, or empty string if positions invalid
 *
 * @see docs/plans/2026-02-10-arc-animation-system-v2-design.md
 */
export function generateParallelPath(
  source: { x: number; y: number },
  target: { x: number; y: number },
  index: number,
  total: number
): string {
  // Validate positions
  if (!isValidPosition(source) || !isValidPosition(target)) {
    return '';
  }

  // Single edge - use standard curved path
  if (total === 1) {
    return generateCurvedPath(source, target);
  }

  // CRITICAL FIX: Use CANONICAL direction for perpendicular calculation
  // This ensures edges A→B and B→A use the same perpendicular vector,
  // so their offsets don't cancel out when going opposite directions.
  // Canonical = always from "smaller" point to "larger" point (x, then y)
  const isSourceCanonicallyFirst =
    source.x < target.x || (source.x === target.x && source.y < target.y);
  const canonicalStart = isSourceCanonicallyFirst ? source : target;
  const canonicalEnd = isSourceCanonicallyFirst ? target : source;

  const dx = canonicalEnd.x - canonicalStart.x;
  const dy = canonicalEnd.y - canonicalStart.y;
  const distance = Math.sqrt(dx * dx + dy * dy);

  // Handle zero distance
  if (distance === 0) {
    return `M ${source.x} ${source.y} L ${target.x} ${target.y}`;
  }

  // Perpendicular unit vector (consistent for A→B and B→A)
  const perpX = -dy / distance;
  const perpY = dx / distance;

  // Calculate offset from center line
  // v11.6.1: Use mathematical spacing for beautiful equal distribution
  // For total=2: offsets are -1, +1 (not -0.5, +0.5)
  // For total=3: offsets are -1, 0, +1
  // For total=4: offsets are -1.5, -0.5, +0.5, +1.5
  const normalizedOffset = (index - (total - 1) / 2);

  // v11.6.1: Much larger base offset for dramatic, beautiful curves
  const baseOffset = 80;
  const maxSpread = 300;

  // Scale offset by total edges so spacing stays consistent regardless of count
  // 2 edges: each gets ±80px
  // 3 edges: -80px, 0, +80px
  // 4 edges: -120px, -40px, +40px, +120px
  const offsetPixels = normalizedOffset * baseOffset;
  const clampedOffset = Math.max(-maxSpread, Math.min(maxSpread, offsetPixels));

  // Midpoint with perpendicular offset
  const midX = (source.x + target.x) / 2 + perpX * clampedOffset;
  const midY = (source.y + target.y) / 2 + perpY * clampedOffset;

  // v11.6.1: Always add some curve, even for center edge (index in middle)
  // This prevents the middle edge from being a straight line
  const baseCurve = 30; // Minimum curve for all edges
  const curveBoost = Math.abs(clampedOffset) * 0.3 + baseCurve;
  const direction = (canonicalStart.x + canonicalStart.y) > (canonicalEnd.x + canonicalEnd.y) ? 1 : -1;
  const controlX = midX + perpX * curveBoost * direction;
  const controlY = midY + perpY * curveBoost * direction;

  return `M ${source.x} ${source.y} Q ${controlX} ${controlY} ${target.x} ${target.y}`;
}

/**
 * Generate REVERSED parallel path (for labels when text needs to be flipped)
 *
 * Same offset logic as generateParallelPath but path goes target → source
 * so that SVG textPath renders text in readable direction.
 *
 * @param source - Source point coordinates
 * @param target - Target point coordinates
 * @param index - Index of this edge in the parallel group (0-based)
 * @param total - Total number of parallel edges
 * @returns SVG path string, or empty string if positions invalid
 */
export function generateReversedParallelPath(
  source: { x: number; y: number },
  target: { x: number; y: number },
  index: number,
  total: number
): string {
  // Validate positions
  if (!isValidPosition(source) || !isValidPosition(target)) {
    return '';
  }

  // Single edge - use standard reversed path
  if (total === 1) {
    return generateReversedPath(source, target);
  }

  // Use same CANONICAL direction for perpendicular calculation as forward path
  const isSourceCanonicallyFirst =
    source.x < target.x || (source.x === target.x && source.y < target.y);
  const canonicalStart = isSourceCanonicallyFirst ? source : target;
  const canonicalEnd = isSourceCanonicallyFirst ? target : source;

  const dx = canonicalEnd.x - canonicalStart.x;
  const dy = canonicalEnd.y - canonicalStart.y;
  const distance = Math.sqrt(dx * dx + dy * dy);

  // Handle zero distance
  if (distance === 0) {
    return `M ${target.x} ${target.y} L ${source.x} ${source.y}`;
  }

  // Perpendicular unit vector (consistent for A→B and B→A)
  const perpX = -dy / distance;
  const perpY = dx / distance;

  // Same offset calculation as forward path (must match exactly)
  const normalizedOffset = (index - (total - 1) / 2);
  const baseOffset = 80;
  const maxSpread = 300;
  const offsetPixels = normalizedOffset * baseOffset;
  const clampedOffset = Math.max(-maxSpread, Math.min(maxSpread, offsetPixels));

  // Midpoint with perpendicular offset
  const midX = (source.x + target.x) / 2 + perpX * clampedOffset;
  const midY = (source.y + target.y) / 2 + perpY * clampedOffset;

  // Always add some curve, even for center edge
  const baseCurve = 30;
  const curveBoost = Math.abs(clampedOffset) * 0.3 + baseCurve;
  const direction = (canonicalStart.x + canonicalStart.y) > (canonicalEnd.x + canonicalEnd.y) ? 1 : -1;
  const controlX = midX + perpX * curveBoost * direction;
  const controlY = midY + perpY * curveBoost * direction;

  // Path goes from TARGET to SOURCE (reversed for text direction)
  return `M ${target.x} ${target.y} Q ${controlX} ${controlY} ${source.x} ${source.y}`;
}

/**
 * Get the midpoint of an SVG quadratic bezier path
 *
 * @param path - SVG path string (M ... Q ... format)
 * @returns Midpoint coordinates, or null if path invalid
 */
export function getPathMidpoint(
  path: string
): { x: number; y: number } | null {
  // Parse M x1 y1 Q cx cy x2 y2
  const match = path.match(/M\s*([\d.-]+)\s+([\d.-]+)\s+Q\s*([\d.-]+)\s+([\d.-]+)\s+([\d.-]+)\s+([\d.-]+)/);

  if (!match) {
    return null;
  }

  const [, x1Str, y1Str, cxStr, cyStr, x2Str, y2Str] = match;
  const x1 = parseFloat(x1Str);
  const y1 = parseFloat(y1Str);
  const cx = parseFloat(cxStr);
  const cy = parseFloat(cyStr);
  const x2 = parseFloat(x2Str);
  const y2 = parseFloat(y2Str);

  // Quadratic bezier at t=0.5
  // B(0.5) = (1-0.5)²P0 + 2(1-0.5)(0.5)P1 + 0.5²P2
  // B(0.5) = 0.25*P0 + 0.5*P1 + 0.25*P2
  const t = 0.5;
  const mt = 1 - t;

  return {
    x: mt * mt * x1 + 2 * mt * t * cx + t * t * x2,
    y: mt * mt * y1 + 2 * mt * t * cy + t * t * y2,
  };
}

// =============================================================================
// Animation Timing Utilities (v11.6.3)
// =============================================================================

/**
 * Generate a deterministic random delay based on edge ID.
 *
 * Uses simple string hashing to produce consistent delays for the same edge,
 * preventing synchronized animation starts across all edges.
 *
 * @param edgeId - Unique edge identifier
 * @param maxDelay - Maximum delay in seconds (default: 5)
 * @returns Delay in seconds (0 to maxDelay)
 *
 * @example
 * ```tsx
 * const delay = getRandomDelay(edgeId, 10); // 0-10s delay
 * <animateMotion begin={`${delay}s`} ... />
 * ```
 */
export function getRandomDelay(edgeId: string, maxDelay: number = 5): number {
  let hash = 0;
  for (let i = 0; i < edgeId.length; i++) {
    const char = edgeId.charCodeAt(i);
    hash = ((hash << 5) - hash) + char;
    hash = hash & hash; // Convert to 32-bit integer
  }
  return Math.abs(hash % 1000) / 1000 * maxDelay;
}
