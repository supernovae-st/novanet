'use client';

/**
 * MouseSpotlight - Interactive spotlight that follows cursor position
 *
 * Creates a radial gradient spotlight that tracks mouse movement,
 * revealing content underneath with a premium lighting effect.
 *
 * Inspired by Aceternity UI and Vercel's gradient hover effects.
 *
 * @example
 * ```tsx
 * <MouseSpotlight
 *   color="#8b5cf6"
 *   size={200}
 *   intensity="medium"
 * />
 * ```
 */

import { memo, useState, useCallback, useRef } from 'react';
import { motion, useMotionValue, useSpring } from 'motion/react';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';

// =============================================================================
// Types
// =============================================================================

export interface MouseSpotlightProps {
  /** Spotlight color */
  color: string;
  /** Spotlight radius in pixels */
  size?: number;
  /** Whether the element is selected */
  selected?: boolean;
  /** Whether the element is hovered */
  isHovered?: boolean;
  /** Performance configuration */
  performanceConfig?: PerformanceConfig;
  /** Intensity: subtle, medium, intense */
  intensity?: 'subtle' | 'medium' | 'intense';
  /** Border radius to match parent */
  borderRadius?: number;
}

// =============================================================================
// Constants
// =============================================================================

const INTENSITY_CONFIG = {
  subtle: { opacity: 0.15, blur: 40 },
  medium: { opacity: 0.25, blur: 60 },
  intense: { opacity: 0.4, blur: 80 },
};

// =============================================================================
// Component
// =============================================================================

export const MouseSpotlight = memo(function MouseSpotlight({
  color,
  size = 150,
  selected = false,
  isHovered = false,
  performanceConfig,
  intensity = 'medium',
  borderRadius = 16,
}: MouseSpotlightProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const containerRef = useRef<HTMLDivElement>(null);
  const [isActive, setIsActive] = useState(false);

  // Raw mouse position
  const mouseX = useMotionValue(0);
  const mouseY = useMotionValue(0);

  // Smooth spring animation for the spotlight
  const springConfig = { damping: 25, stiffness: 200 };
  const spotlightX = useSpring(mouseX, springConfig);
  const spotlightY = useSpring(mouseY, springConfig);

  // Get intensity config
  const config = INTENSITY_CONFIG[intensity];
  const currentSize = selected ? size * 1.3 : isHovered ? size * 1.1 : size;
  const currentOpacity = selected ? config.opacity * 1.5 : isHovered ? config.opacity * 1.2 : config.opacity;

  // Mouse handlers
  const handleMouseMove = useCallback(
    (e: React.MouseEvent<HTMLDivElement>) => {
      if (!containerRef.current || !animationsEnabled) return;
      const rect = containerRef.current.getBoundingClientRect();
      mouseX.set(e.clientX - rect.left);
      mouseY.set(e.clientY - rect.top);
    },
    [mouseX, mouseY, animationsEnabled]
  );

  const handleMouseEnter = useCallback(() => {
    setIsActive(true);
  }, []);

  const handleMouseLeave = useCallback(() => {
    setIsActive(false);
  }, []);

  if (!animationsEnabled) {
    return null;
  }

  return (
    <div
      ref={containerRef}
      className="absolute inset-0 pointer-events-auto overflow-hidden"
      style={{ borderRadius }}
      onMouseMove={handleMouseMove}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
    >
      {/* Spotlight gradient */}
      <motion.div
        className="absolute pointer-events-none"
        style={{
          width: currentSize,
          height: currentSize,
          x: spotlightX,
          y: spotlightY,
          translateX: '-50%',
          translateY: '-50%',
          background: `radial-gradient(circle, ${color} 0%, ${color}40 30%, transparent 70%)`,
          filter: `blur(${config.blur}px)`,
          opacity: isActive ? currentOpacity : 0,
        }}
        animate={{
          scale: isActive ? 1 : 0.8,
        }}
        transition={{
          scale: { duration: 0.3 },
          opacity: { duration: 0.2 },
        }}
      />

      {/* Secondary spotlight (selected only) */}
      {selected && isActive && (
        <motion.div
          className="absolute pointer-events-none"
          style={{
            width: currentSize * 1.5,
            height: currentSize * 1.5,
            x: spotlightX,
            y: spotlightY,
            translateX: '-50%',
            translateY: '-50%',
            background: `radial-gradient(circle, ${color}20 0%, transparent 60%)`,
            filter: `blur(${config.blur * 1.5}px)`,
          }}
          animate={{
            scale: [1, 1.2, 1],
            opacity: [0.3, 0.5, 0.3],
          }}
          transition={{
            duration: 2,
            repeat: Infinity,
            ease: 'easeInOut',
          }}
        />
      )}

      {/* Highlight ring at cursor position */}
      {isActive && (
        <motion.div
          className="absolute pointer-events-none rounded-full"
          style={{
            width: 4,
            height: 4,
            x: spotlightX,
            y: spotlightY,
            translateX: '-50%',
            translateY: '-50%',
            backgroundColor: color,
            boxShadow: `0 0 10px ${color}, 0 0 20px ${color}60`,
          }}
          animate={{
            scale: [1, 1.5, 1],
            opacity: [0.8, 1, 0.8],
          }}
          transition={{
            duration: 1,
            repeat: Infinity,
            ease: 'easeInOut',
          }}
        />
      )}
    </div>
  );
});
