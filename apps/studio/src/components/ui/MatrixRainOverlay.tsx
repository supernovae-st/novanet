'use client';

/**
 * MatrixRainOverlay - Full-screen Matrix rain animation
 *
 * Used during Data ↔ Schema mode transitions.
 * Uses tsparticles with WebGL for smooth performance.
 */

import { useCallback, useEffect, useState } from 'react';
import Particles, { initParticlesEngine } from '@tsparticles/react';
import { loadSlim } from '@tsparticles/slim';
import type { Engine, ISourceOptions } from '@tsparticles/engine';
import { cn } from '@/lib/utils';

interface MatrixRainOverlayProps {
  /** Whether the overlay is visible */
  visible: boolean;
  /** Callback when the transition completes */
  onComplete?: () => void;
  /** Transition phase for opacity control */
  phase?: 'dissolve' | 'fetch' | 'reform' | null;
}

// Matrix rain particle configuration
const MATRIX_OPTIONS: ISourceOptions = {
  fullScreen: false,
  background: {
    color: 'transparent',
  },
  fpsLimit: 60,
  particles: {
    number: {
      value: 150,
      density: {
        enable: true,
      },
    },
    color: {
      value: ['#10b981', '#34d399', '#6ee7b7', '#a7f3d0'],
    },
    shape: {
      type: 'char',
      options: {
        char: {
          value: 'アイウエオカキクケコサシスセソタチツテトナニヌネノ0123456789'.split(''),
          font: 'monospace',
          weight: '400',
        },
      },
    },
    opacity: {
      value: { min: 0.1, max: 0.9 },
      animation: {
        enable: true,
        speed: 1,
        sync: false,
      },
    },
    size: {
      value: { min: 10, max: 18 },
    },
    move: {
      enable: true,
      direction: 'bottom',
      speed: { min: 8, max: 25 },
      straight: true,
      outModes: {
        default: 'out',
        top: 'none',
      },
    },
    // Glow effect
    shadow: {
      enable: true,
      color: '#10b981',
      blur: 10,
    },
  },
  detectRetina: true,
};

export function MatrixRainOverlay({ visible, phase }: MatrixRainOverlayProps) {
  const [init, setInit] = useState(false);

  // Initialize tsparticles engine once
  useEffect(() => {
    initParticlesEngine(async (engine: Engine) => {
      await loadSlim(engine);
    }).then(() => {
      setInit(true);
    });
  }, []);

  const particlesLoaded = useCallback(async () => {
    // Particles loaded callback
  }, []);

  // Determine opacity based on phase
  const opacity = phase === 'dissolve' || phase === 'fetch' ? 1 : phase === 'reform' ? 0 : 0;

  if (!init || !visible) return null;

  return (
    <div
      className={cn(
        'absolute inset-0 z-40 pointer-events-none',
        'transition-opacity duration-400 ease-out'
      )}
      style={{ opacity }}
      aria-hidden="true"
    >
      <Particles
        id="matrix-rain"
        className="w-full h-full"
        particlesLoaded={particlesLoaded}
        options={MATRIX_OPTIONS}
      />
    </div>
  );
}
