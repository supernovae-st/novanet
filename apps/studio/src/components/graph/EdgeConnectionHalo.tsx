'use client';

/**
 * EdgeConnectionHalo - Visual indicator for nodes connected to selected edge
 *
 * v11.6.6 - Enhanced Source Pulsation:
 * - Source: Dramatic sonar-style expanding rings + directional arrow
 * - Target: Subtle inward pulse (destination marker)
 *
 * Features:
 * - Multiple concentric expanding rings (source)
 * - Directional arrow icon at source center
 * - Pulsing glow for edge connection state
 * - Source vs Target differentiation
 */

import { memo } from 'react';
import { cn } from '@/lib/utils';

export type EdgeConnectionRole = 'source' | 'target' | null;

export interface EdgeConnectionHaloProps {
  /** Role of this node in the selected edge (source/target/null) */
  role: EdgeConnectionRole;
  /** Accent color for the halo (from edge theme) */
  color: string;
  /** Additional CSS classes */
  className?: string;
}

/**
 * Edge connection halo that renders around nodes connected to selected edge
 *
 * - Source: Sonar-style expanding rings + arrow (dramatic origin marker)
 * - Target: Dashed ring with inward pulse (destination marker)
 */
export const EdgeConnectionHalo = memo(function EdgeConnectionHalo({
  role,
  color,
  className,
}: EdgeConnectionHaloProps) {
  // Performance: Early return if not connected to selected edge
  if (!role) return null;

  const isSource = role === 'source';

  if (isSource) {
    // SOURCE: Dramatic sonar-style expanding rings
    return (
      <div
        className={cn(
          'absolute inset-0 rounded-xl pointer-events-none z-10 overflow-visible',
          className
        )}
        aria-hidden="true"
      >
        {/* Base glow layer */}
        <div
          className="absolute inset-0 rounded-xl"
          style={{
            boxShadow: `
              0 0 0 3px ${color},
              0 0 30px ${color}70,
              0 0 60px ${color}40,
              0 0 100px ${color}20
            `,
          }}
        />

        {/* Expanding ring 1 - fastest */}
        <div
          className="absolute inset-[-30px] rounded-3xl animate-source-ring-1"
          style={{
            border: `5px solid ${color}`,
            boxShadow: `0 0 20px ${color}`,
          }}
        />

        {/* Expanding ring 2 - medium */}
        <div
          className="absolute inset-[-30px] rounded-3xl animate-source-ring-2"
          style={{
            border: `4px solid ${color}`,
            boxShadow: `0 0 15px ${color}`,
          }}
        />

        {/* Expanding ring 3 - slowest, largest */}
        <div
          className="absolute inset-[-30px] rounded-3xl animate-source-ring-3"
          style={{
            border: `3px solid ${color}`,
            boxShadow: `0 0 10px ${color}`,
          }}
        />

        {/* Expanding ring 4 - extra wave */}
        <div
          className="absolute inset-[-30px] rounded-3xl animate-source-ring-4"
          style={{
            border: `2px solid ${color}80`,
          }}
        />

        {/* Inner pulsing core */}
        <div
          className="absolute inset-2 rounded-lg animate-breathe-subtle"
          style={{
            boxShadow: `inset 0 0 20px ${color}60`,
            background: `radial-gradient(circle at center, ${color}15 0%, transparent 70%)`,
          }}
        />

        {/* Directional arrow icon at center - Rounded friendly chevron (z-20 to stay above rings) */}
        <div className="absolute inset-0 flex items-center justify-center z-20">
          <svg
            width="80"
            height="80"
            viewBox="0 0 64 64"
            className="animate-pulse-arrow"
            style={{
              transform: 'rotate(-45deg)',
              filter: `drop-shadow(0 0 16px ${color}) drop-shadow(0 0 32px ${color}80)`,
            }}
          >
            {/* Ultra glow layer */}
            <path
              d="M18 10 L46 32 L18 54"
              fill="none"
              stroke={color}
              strokeWidth="20"
              strokeLinecap="round"
              strokeLinejoin="round"
              opacity="0.25"
              style={{ filter: 'blur(10px)' }}
            />
            {/* Outer glow */}
            <path
              d="M18 10 L46 32 L18 54"
              fill="none"
              stroke={color}
              strokeWidth="16"
              strokeLinecap="round"
              strokeLinejoin="round"
              opacity="0.5"
              style={{ filter: 'blur(5px)' }}
            />
            {/* Primary stroke */}
            <path
              d="M18 10 L46 32 L18 54"
              fill="none"
              stroke={color}
              strokeWidth="10"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
            {/* White inner highlight */}
            <path
              d="M20 14 L44 32 L20 50"
              fill="none"
              stroke="white"
              strokeWidth="4"
              strokeLinecap="round"
              strokeLinejoin="round"
              opacity="0.95"
            />
          </svg>
        </div>
      </div>
    );
  }

  // TARGET: Subtle inward pulse
  return (
    <div
      className={cn(
        'absolute inset-0 rounded-xl pointer-events-none z-10',
        'animate-edge-target-pulse',
        className
      )}
      style={{
        boxShadow: `
          0 0 0 3px ${color}80,
          0 0 20px ${color}50,
          0 0 40px ${color}25
        `,
        border: `2px dashed ${color}60`,
      }}
      aria-hidden="true"
    />
  );
});
