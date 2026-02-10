'use client';

/**
 * useMatrixRain - Matrix rain animation state management
 *
 * Controls the Matrix-style rain effect that plays during Neo4j queries.
 * Provides start/stop controls and animation configuration.
 *
 * Features:
 * - Auto-stop after configurable duration
 * - Smooth fade-in/fade-out transitions
 * - Performance-optimized (will-change, GPU acceleration)
 */

import { useState, useCallback, useRef, useEffect } from 'react';

export interface MatrixRainConfig {
  /** Duration of rain effect in ms (default: auto-stop disabled) */
  duration?: number;
  /** Fade out duration in ms (default: 300) */
  fadeOutDuration?: number;
  /** Character set for rain (default: katakana + latin) */
  characters?: string;
  /** Number of rain columns (default: auto based on container width) */
  columns?: number;
  /** Color of rain characters (default: accent color) */
  color?: string;
}

export interface MatrixRainState {
  /** Whether rain is currently active */
  isActive: boolean;
  /** Whether rain is fading out */
  isFadingOut: boolean;
  /** Current opacity (0-1) for fade transitions */
  opacity: number;
}

export interface UseMatrixRainResult {
  /** Current state of the rain effect */
  state: MatrixRainState;
  /** Start the rain effect */
  start: () => void;
  /** Stop the rain effect (with fade out) */
  stop: () => void;
  /** Immediately stop without fade */
  forceStop: () => void;
}

// Default katakana + latin characters for Matrix effect
const DEFAULT_CHARACTERS =
  'アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン' +
  'ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789@#$%^&*()';

const DEFAULT_FADE_DURATION = 300;

/**
 * Hook for managing Matrix rain animation state
 *
 * @example
 * ```tsx
 * function GraphCanvas() {
 *   const { state, start, stop } = useMatrixRain();
 *
 *   // Start rain when query begins
 *   useEffect(() => {
 *     if (isQuerying) start();
 *     else stop();
 *   }, [isQuerying]);
 *
 *   return (
 *     <>
 *       <ReactFlow />
 *       {state.isActive && <MatrixRainOverlay opacity={state.opacity} />}
 *     </>
 *   );
 * }
 * ```
 */
export function useMatrixRain(config: MatrixRainConfig = {}): UseMatrixRainResult {
  const { duration, fadeOutDuration = DEFAULT_FADE_DURATION } = config;

  const [state, setState] = useState<MatrixRainState>({
    isActive: false,
    isFadingOut: false,
    opacity: 0,
  });

  const fadeTimeoutRef = useRef<NodeJS.Timeout | null>(null);
  const durationTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  // Cleanup timeouts on unmount
  useEffect(() => {
    return () => {
      if (fadeTimeoutRef.current) clearTimeout(fadeTimeoutRef.current);
      if (durationTimeoutRef.current) clearTimeout(durationTimeoutRef.current);
    };
  }, []);

  const start = useCallback(() => {
    // Clear any pending timeouts
    if (fadeTimeoutRef.current) clearTimeout(fadeTimeoutRef.current);
    if (durationTimeoutRef.current) clearTimeout(durationTimeoutRef.current);

    // Start rain
    setState({
      isActive: true,
      isFadingOut: false,
      opacity: 1,
    });

    // Auto-stop after duration if specified
    if (duration && duration > 0) {
      durationTimeoutRef.current = setTimeout(() => {
        stop();
      }, duration);
    }
  }, [duration]);

  const stop = useCallback(() => {
    // Start fade out
    setState((prev) => ({
      ...prev,
      isFadingOut: true,
      opacity: 0,
    }));

    // Complete stop after fade
    fadeTimeoutRef.current = setTimeout(() => {
      setState({
        isActive: false,
        isFadingOut: false,
        opacity: 0,
      });
    }, fadeOutDuration);
  }, [fadeOutDuration]);

  const forceStop = useCallback(() => {
    // Clear any pending timeouts
    if (fadeTimeoutRef.current) clearTimeout(fadeTimeoutRef.current);
    if (durationTimeoutRef.current) clearTimeout(durationTimeoutRef.current);

    // Immediately stop
    setState({
      isActive: false,
      isFadingOut: false,
      opacity: 0,
    });
  }, []);

  return {
    state,
    start,
    stop,
    forceStop,
  };
}

export { DEFAULT_CHARACTERS };
export default useMatrixRain;
