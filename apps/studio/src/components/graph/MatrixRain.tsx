'use client';

/**
 * MatrixRain - Matrix-style rain overlay effect
 *
 * Displays falling characters during Neo4j queries, providing
 * visual feedback that the system is working.
 *
 * Features:
 * - GPU-accelerated animations (translateY + opacity)
 * - Configurable character set (katakana + latin by default)
 * - Smooth fade in/out transitions
 * - Pointer-events disabled (doesn't block interaction)
 * - Respects prefers-reduced-motion
 */

import { memo, useMemo } from 'react';
import { cn } from '@/lib/utils';
import { MATRIX_CHARACTERS } from '@/hooks';

interface MatrixRainProps {
  /** Whether the rain is active */
  isActive: boolean;
  /** Opacity for fade transitions (0-1) */
  opacity?: number;
  /** Color of rain characters (CSS color) */
  color?: string;
  /** Number of columns (default: auto based on 20px per column) */
  columns?: number;
  /** Custom character set */
  characters?: string;
  /** Additional className */
  className?: string;
}

interface RainColumn {
  id: number;
  left: string;
  delay: number;
  duration: number;
  chars: string[];
}

/**
 * Generate random rain columns
 */
function generateColumns(count: number, characters: string): RainColumn[] {
  const charArray = characters.split('');
  const columns: RainColumn[] = [];

  for (let i = 0; i < count; i++) {
    // Random number of characters per column (5-15)
    const charCount = 5 + Math.floor(Math.random() * 10);
    const chars: string[] = [];

    for (let j = 0; j < charCount; j++) {
      chars.push(charArray[Math.floor(Math.random() * charArray.length)]);
    }

    columns.push({
      id: i,
      left: `${(i / count) * 100}%`,
      delay: Math.random() * 2, // 0-2s delay
      duration: 2 + Math.random() * 2, // 2-4s duration
      chars,
    });
  }

  return columns;
}

export const MatrixRain = memo(function MatrixRain({
  isActive,
  opacity = 1,
  color = 'var(--accent-color, #10b981)',
  columns: columnCount,
  characters = MATRIX_CHARACTERS,
  className,
}: MatrixRainProps) {
  // Generate columns based on count (default ~40 for full width)
  const columns = useMemo(() => {
    const count = columnCount || 40;
    return generateColumns(count, characters);
  }, [columnCount, characters]);

  if (!isActive) return null;

  return (
    <div
      className={cn(
        'absolute inset-0 pointer-events-none overflow-hidden z-50',
        'motion-reduce:hidden', // Respect reduced motion
        className
      )}
      style={{
        opacity,
        transition: 'opacity 300ms ease-out',
      }}
      aria-hidden="true"
    >
      {columns.map((column) => (
        <div
          key={column.id}
          className="absolute top-0 flex flex-col"
          style={{
            left: column.left,
            animation: `matrixFall ${column.duration}s linear infinite`,
            animationDelay: `${column.delay}s`,
          }}
        >
          {column.chars.map((char, idx) => (
            <span
              key={idx}
              className="font-mono text-sm leading-tight"
              style={{
                color,
                textShadow: `0 0 10px ${color}`,
                opacity: 1 - idx * 0.08, // Fade along column
              }}
            >
              {char}
            </span>
          ))}
        </div>
      ))}

      {/* Keyframe animation injected via style tag */}
      <style jsx>{`
        @keyframes matrixFall {
          0% {
            transform: translateY(-100%);
            opacity: 1;
          }
          90% {
            opacity: 1;
          }
          100% {
            transform: translateY(100vh);
            opacity: 0;
          }
        }
      `}</style>
    </div>
  );
});

export default MatrixRain;
