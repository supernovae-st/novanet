'use client';

/**
 * MatrixTransition - 3-phase Matrix rain transition effect
 *
 * Phases:
 * 1. Dissolve (300ms): Current graph pixelates into Matrix characters
 * 2. Rain (400ms): Matrix rain with view-type color
 * 3. Materialize (300ms): New nodes appear from characters
 *
 * Total duration: 1000ms
 */

import { memo, useEffect, useState, useRef, useCallback } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { cn } from '@/lib/utils';
import type { ViewId } from '@/config/viewTypes';
import { VIEW_TYPES } from '@/config/viewTypes';

// =============================================================================
// TYPES
// =============================================================================

type TransitionPhase = 'idle' | 'dissolve' | 'rain' | 'materialize';

interface MatrixTransitionProps {
  isActive: boolean;
  viewId: ViewId | null;
  onComplete?: () => void;
  className?: string;
}

interface MatrixChar {
  char: string;
  x: number;
  y: number;
  speed: number;
  opacity: number;
}

// =============================================================================
// CONSTANTS
// =============================================================================

const MATRIX_CHARS = 'ァアィイゥウェエォオカガキギクグケゲコゴサザシジスズセゼソゾタダチヂッツヅテデトドナニヌネノハバパヒビピフブプヘベペホボポマミムメモャヤュユョヨラリルレロヮワヰヱヲンヴヵヶ0123456789';
const DISSOLVE_DURATION = 300;
const RAIN_DURATION = 400;
const MATERIALIZE_DURATION = 300;

// =============================================================================
// MATRIX RAIN CANVAS
// =============================================================================

const MatrixRainCanvas = memo(function MatrixRainCanvas({
  color,
  phase,
}: {
  color: string;
  phase: TransitionPhase;
}) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const animationRef = useRef<number | undefined>(undefined);
  const charsRef = useRef<MatrixChar[]>([]);

  // Initialize characters
  const initChars = useCallback((width: number, height: number) => {
    const columns = Math.ceil(width / 14);
    const chars: MatrixChar[] = [];

    for (let i = 0; i < columns; i++) {
      chars.push({
        char: MATRIX_CHARS[Math.floor(Math.random() * MATRIX_CHARS.length)],
        x: i * 14,
        y: Math.random() * height * -1,
        speed: 5 + Math.random() * 10,
        opacity: 0.3 + Math.random() * 0.7,
      });
    }

    charsRef.current = chars;
  }, []);

  // Animation loop
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Set canvas size
    const resize = () => {
      canvas.width = canvas.offsetWidth * window.devicePixelRatio;
      canvas.height = canvas.offsetHeight * window.devicePixelRatio;
      ctx.scale(window.devicePixelRatio, window.devicePixelRatio);
      initChars(canvas.offsetWidth, canvas.offsetHeight);
    };

    resize();
    window.addEventListener('resize', resize);

    // Initialize with initial dimensions
    initChars(canvas.offsetWidth, canvas.offsetHeight);

    // Animation
    const animate = () => {
      if (phase !== 'rain') {
        animationRef.current = requestAnimationFrame(animate);
        return;
      }

      ctx.fillStyle = 'rgba(0, 0, 0, 0.05)';
      ctx.fillRect(0, 0, canvas.offsetWidth, canvas.offsetHeight);

      ctx.font = '14px monospace';

      charsRef.current.forEach((char) => {
        // Random char change
        if (Math.random() > 0.95) {
          char.char = MATRIX_CHARS[Math.floor(Math.random() * MATRIX_CHARS.length)];
        }

        // Draw with color
        ctx.fillStyle = color + Math.floor(char.opacity * 255).toString(16).padStart(2, '0');
        ctx.fillText(char.char, char.x, char.y);

        // Move down
        char.y += char.speed;

        // Reset when off screen
        if (char.y > canvas.offsetHeight) {
          char.y = -20;
          char.speed = 5 + Math.random() * 10;
        }
      });

      animationRef.current = requestAnimationFrame(animate);
    };

    animationRef.current = requestAnimationFrame(animate);

    return () => {
      window.removeEventListener('resize', resize);
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current);
      }
    };
  }, [color, phase, initChars]);

  return (
    <canvas
      ref={canvasRef}
      className="absolute inset-0 w-full h-full pointer-events-none"
    />
  );
});

// =============================================================================
// MAIN COMPONENT
// =============================================================================

export const MatrixTransition = memo(function MatrixTransition({
  isActive,
  viewId,
  onComplete,
  className,
}: MatrixTransitionProps) {
  const [phase, setPhase] = useState<TransitionPhase>('idle');

  // Get transition color from view config
  const color = viewId ? VIEW_TYPES[viewId]?.transitionColor ?? '#22c55e' : '#22c55e';

  // Run transition phases
  useEffect(() => {
    if (!isActive) {
      setPhase('idle');
      return;
    }

    // Phase 1: Dissolve
    setPhase('dissolve');

    const dissolveTimer = setTimeout(() => {
      // Phase 2: Rain
      setPhase('rain');

      const rainTimer = setTimeout(() => {
        // Phase 3: Materialize
        setPhase('materialize');

        const materializeTimer = setTimeout(() => {
          setPhase('idle');
          onComplete?.();
        }, MATERIALIZE_DURATION);

        return () => clearTimeout(materializeTimer);
      }, RAIN_DURATION);

      return () => clearTimeout(rainTimer);
    }, DISSOLVE_DURATION);

    return () => clearTimeout(dissolveTimer);
  }, [isActive, onComplete]);

  if (phase === 'idle') {
    return null;
  }

  return (
    <AnimatePresence>
      <motion.div
        className={cn(
          'fixed inset-0 z-50 pointer-events-none overflow-hidden',
          className
        )}
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
      >
        {/* Phase 1: Dissolve - dark overlay with pixelation */}
        {phase === 'dissolve' && (
          <motion.div
            className="absolute inset-0 bg-black"
            initial={{ opacity: 0 }}
            animate={{ opacity: 0.9 }}
            transition={{ duration: DISSOLVE_DURATION / 1000 }}
          >
            {/* Scanline effect */}
            <div
              className="absolute inset-0"
              style={{
                background: `repeating-linear-gradient(
                  0deg,
                  transparent,
                  transparent 2px,
                  rgba(0, 0, 0, 0.3) 2px,
                  rgba(0, 0, 0, 0.3) 4px
                )`,
              }}
            />
          </motion.div>
        )}

        {/* Phase 2: Rain - Matrix characters falling */}
        {phase === 'rain' && (
          <motion.div
            className="absolute inset-0 bg-black"
            initial={{ opacity: 0.9 }}
            animate={{ opacity: 1 }}
          >
            <MatrixRainCanvas color={color} phase={phase} />

            {/* Glow overlay */}
            <div
              className="absolute inset-0"
              style={{
                background: `radial-gradient(circle at center, ${color}20, transparent 70%)`,
              }}
            />
          </motion.div>
        )}

        {/* Phase 3: Materialize - fade out with scale */}
        {phase === 'materialize' && (
          <motion.div
            className="absolute inset-0 bg-black"
            initial={{ opacity: 1 }}
            animate={{ opacity: 0 }}
            transition={{ duration: MATERIALIZE_DURATION / 1000, ease: 'easeOut' }}
          >
            {/* Center burst effect */}
            <motion.div
              className="absolute inset-0 flex items-center justify-center"
              initial={{ scale: 1, opacity: 0.5 }}
              animate={{ scale: 2, opacity: 0 }}
              transition={{ duration: MATERIALIZE_DURATION / 1000 }}
            >
              <div
                className="w-32 h-32 rounded-full"
                style={{
                  background: `radial-gradient(circle, ${color}, transparent)`,
                  filter: 'blur(20px)',
                }}
              />
            </motion.div>
          </motion.div>
        )}
      </motion.div>
    </AnimatePresence>
  );
});

export default MatrixTransition;
