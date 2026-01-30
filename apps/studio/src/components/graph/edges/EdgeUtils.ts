/**
 * Edge Utility Functions
 *
 * Pure utility functions for edge rendering:
 * - formatRelationType: Format relation type for display
 * - getSmartLabel: Get smart abbreviated label based on available space
 * - getRelationIcon: Get icon for relation type
 * - getNodeIntersection: Calculate edge intersection with node bounds
 * - generateCurvedPath: Generate curved SVG path between two points
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
 * Generate a curved path between two points
 */
export function generateCurvedPath(
  source: { x: number; y: number },
  target: { x: number; y: number }
): string {
  const dx = target.x - source.x;
  const dy = target.y - source.y;
  const distance = Math.sqrt(dx * dx + dy * dy);

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
 */
export function generateReversedPath(
  source: { x: number; y: number },
  target: { x: number; y: number }
): string {
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
