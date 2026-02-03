'use client';

/**
 * MatrixExplosionOverlay - Epic Matrix Rain Easter Egg
 *
 * Triggered by clicking META badge:
 * 1. Canvas-based Matrix code rain (like the movie)
 * 2. Particle explosion from center
 * 3. Wave glow that pulses through existing UI
 * 4. Mode name reveal with typewriter effect
 *
 * Color themes per navigation mode:
 * - meta: Blue (#60a5fa)
 * - data: Emerald (#34d399)
 * - overlay: Violet (#a78bfa)
 * - query: Amber (#fbbf24)
 */

import { memo, useEffect, useRef, useState, useCallback } from 'react';
import { cn } from '@/lib/utils';
import type { NavigationMode } from '@/stores/uiStore';

// Terminal-style prefix for mode display
const MODE_PREFIX = '> mode:';

// Matrix characters - mix of katakana and symbols
const MATRIX_CHARS = 'アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン0123456789ABCDEF<>{}[]@#$%';

// Mode-specific themes
const MODE_THEMES: Record<NavigationMode, { primary: string; rgb: string; name: string }> = {
  data: {
    primary: '#34d399',
    rgb: '52, 211, 153',
    name: 'DATA',
  },
  meta: {
    primary: '#60a5fa',
    rgb: '96, 165, 250',
    name: 'META',
  },
  overlay: {
    primary: '#a78bfa',
    rgb: '167, 139, 250',
    name: 'OVERLAY',
  },
  query: {
    primary: '#fbbf24',
    rgb: '251, 191, 36',
    name: 'QUERY',
  },
};

// Particle for explosion effect
interface Particle {
  id: number;
  char: string;
  x: number;
  y: number;
  vx: number;
  vy: number;
  size: number;
  opacity: number;
  rotation: number;
}

interface MatrixExplosionOverlayProps {
  isActive: boolean;
  onComplete: () => void;
  navigationMode: NavigationMode;
}

export const MatrixExplosionOverlay = memo(function MatrixExplosionOverlay({
  isActive,
  onComplete,
  navigationMode,
}: MatrixExplosionOverlayProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const animationRef = useRef<number | null>(null);
  const [phase, setPhase] = useState<'idle' | 'rain' | 'explode' | 'fade'>('idle');
  const [particles, setParticles] = useState<Particle[]>([]);
  const [showName, setShowName] = useState(false);
  const [displayedText, setDisplayedText] = useState('');
  const [showCursor, setShowCursor] = useState(true);

  const theme = MODE_THEMES[navigationMode] || MODE_THEMES.meta;
  const fullText = `${MODE_PREFIX}${theme.name.toLowerCase()}`;

  // Matrix rain animation on canvas
  const startMatrixRain = useCallback(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Set canvas size
    const width = window.innerWidth;
    const height = window.innerHeight;
    canvas.width = width;
    canvas.height = height;

    // Rain columns
    const fontSize = 16;
    const columns = Math.ceil(width / fontSize);
    const drops: number[] = Array(columns).fill(0).map(() => Math.random() * -100);

    let frameCount = 0;
    const maxFrames = 120; // ~2 seconds at 60fps

    const animate = () => {
      frameCount++;

      // Fade out near the end
      const fadeProgress = frameCount > maxFrames - 30 ? (maxFrames - frameCount) / 30 : 1;

      // Semi-transparent background for trail effect
      ctx.fillStyle = `rgba(10, 10, 15, ${0.1 * fadeProgress})`;
      ctx.fillRect(0, 0, width, height);

      // Draw characters
      ctx.font = `${fontSize}px monospace`;

      for (let i = 0; i < columns; i++) {
        // Random character
        const char = MATRIX_CHARS[Math.floor(Math.random() * MATRIX_CHARS.length)];

        // Varying brightness
        const brightness = Math.random() > 0.9 ? 1 : 0.3 + Math.random() * 0.4;

        // Color with theme
        ctx.fillStyle = `rgba(${theme.rgb}, ${brightness * fadeProgress})`;

        // Glow effect for bright characters
        if (brightness > 0.8) {
          ctx.shadowColor = theme.primary;
          ctx.shadowBlur = 8;
        } else {
          ctx.shadowBlur = 0;
        }

        ctx.fillText(char, i * fontSize, drops[i] * fontSize);

        // Reset drop or continue falling
        if (drops[i] * fontSize > height && Math.random() > 0.975) {
          drops[i] = 0;
        }
        drops[i] += 0.5 + Math.random() * 0.5;
      }

      // Continue animation
      if (frameCount < maxFrames) {
        animationRef.current = requestAnimationFrame(animate);
      } else {
        // Clear canvas
        ctx.clearRect(0, 0, width, height);
      }
    };

    animate();
  }, [theme]);

  // Typewriter effect
  useEffect(() => {
    if (!showName) {
      setDisplayedText('');
      return;
    }

    let index = 0;
    const typeInterval = setInterval(() => {
      if (index < fullText.length) {
        setDisplayedText(fullText.slice(0, index + 1));
        index++;
      } else {
        clearInterval(typeInterval);
      }
    }, 60); // 60ms per character

    return () => clearInterval(typeInterval);
  }, [showName, fullText]);

  // Cursor blink effect
  useEffect(() => {
    if (!showName) return;

    const blinkInterval = setInterval(() => {
      setShowCursor((prev) => !prev);
    }, 530);

    return () => clearInterval(blinkInterval);
  }, [showName]);

  // Add/remove glitch class for global UI effect
  useEffect(() => {
    if (phase === 'rain' || phase === 'explode') {
      document.body.classList.add('matrix-glitch-active');
      document.documentElement.style.setProperty('--matrix-color', theme.primary);
      document.documentElement.style.setProperty('--matrix-glow', `rgba(${theme.rgb}, 0.6)`);
    } else {
      document.body.classList.remove('matrix-glitch-active');
    }
    return () => {
      document.body.classList.remove('matrix-glitch-active');
    };
  }, [phase, theme]);

  // Main animation sequence
  useEffect(() => {
    if (!isActive) {
      setPhase('idle');
      setParticles([]);
      setShowName(false);
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current);
      }
      return;
    }

    // Phase 1: Start rain
    setPhase('rain');
    startMatrixRain();

    // Phase 2: Explosion + name reveal (after 500ms)
    const explodeTimer = setTimeout(() => {
      setPhase('explode');
      setShowName(true);

      // Generate explosion particles from center
      const centerX = window.innerWidth / 2;
      const centerY = window.innerHeight / 2;

      const newParticles: Particle[] = Array.from({ length: 60 }, (_, i) => {
        const angle = (Math.PI * 2 * i) / 60 + Math.random() * 0.5;
        const speed = 6 + Math.random() * 14;
        return {
          id: i,
          char: MATRIX_CHARS[Math.floor(Math.random() * MATRIX_CHARS.length)],
          x: centerX,
          y: centerY,
          vx: Math.cos(angle) * speed,
          vy: Math.sin(angle) * speed,
          size: 18 + Math.random() * 24,
          opacity: 1,
          rotation: Math.random() * 360,
        };
      });
      setParticles(newParticles);
    }, 500);

    // Phase 3: Fade (after 2s)
    const fadeTimer = setTimeout(() => {
      setPhase('fade');
      setShowName(false);
    }, 2000);

    // Complete (after 2.5s)
    const completeTimer = setTimeout(() => {
      setPhase('idle');
      setParticles([]);
      onComplete();
    }, 2500);

    return () => {
      clearTimeout(explodeTimer);
      clearTimeout(fadeTimer);
      clearTimeout(completeTimer);
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current);
      }
    };
  }, [isActive, onComplete, startMatrixRain]);

  // Animate particles with physics
  useEffect(() => {
    if (particles.length === 0) return;

    const interval = setInterval(() => {
      setParticles((prev) =>
        prev
          .map((p) => ({
            ...p,
            x: p.x + p.vx,
            y: p.y + p.vy,
            vy: p.vy + 0.3, // gravity
            vx: p.vx * 0.98, // friction
            opacity: Math.max(0, p.opacity - 0.02),
            rotation: p.rotation + p.vx * 2,
            // Random character change
            char:
              Math.random() > 0.9
                ? MATRIX_CHARS[Math.floor(Math.random() * MATRIX_CHARS.length)]
                : p.char,
          }))
          .filter((p) => p.opacity > 0)
      );
    }, 16);

    return () => clearInterval(interval);
  }, [particles.length]);

  if (!isActive && phase === 'idle') return null;

  return (
    <div className="fixed inset-0 z-[9999] pointer-events-none overflow-hidden">
      {/* Canvas for Matrix rain */}
      <canvas
        ref={canvasRef}
        className={cn(
          'absolute inset-0 transition-opacity duration-500',
          phase === 'fade' ? 'opacity-0' : 'opacity-70'
        )}
      />

      {/* Explosion particles */}
      {particles.map((p) => (
        <span
          key={p.id}
          className="absolute font-mono font-bold pointer-events-none"
          style={{
            left: p.x,
            top: p.y,
            fontSize: p.size,
            color: theme.primary,
            opacity: p.opacity,
            transform: `translate(-50%, -50%) rotate(${p.rotation}deg)`,
            textShadow: `
              0 0 ${p.size / 2}px rgba(${theme.rgb}, 0.8),
              0 0 ${p.size}px rgba(${theme.rgb}, 0.4)
            `,
          }}
        >
          {p.char}
        </span>
      ))}

      {/* Mode name - Terminal typewriter style */}
      {showName && (
        <div className="absolute inset-0 flex items-center justify-center">
          <div
            className="relative px-6 py-4 rounded-lg"
            style={{
              background: 'rgba(0, 0, 0, 0.4)',
              backdropFilter: 'blur(8px)',
              border: `1px solid rgba(${theme.rgb}, 0.3)`,
              boxShadow: `
                0 0 30px rgba(${theme.rgb}, 0.15),
                inset 0 1px 0 rgba(255, 255, 255, 0.05)
              `,
            }}
          >
            {/* Scanline effect */}
            <div
              className="absolute inset-0 pointer-events-none rounded-lg overflow-hidden opacity-[0.03]"
              style={{
                background: 'repeating-linear-gradient(0deg, transparent, transparent 2px, rgba(255,255,255,0.1) 2px, rgba(255,255,255,0.1) 4px)',
              }}
            />

            {/* Terminal text */}
            <div className="relative flex items-center">
              <span
                className="font-mono text-2xl font-medium tracking-wide"
                style={{
                  color: theme.primary,
                  textShadow: `0 0 10px rgba(${theme.rgb}, 0.5)`,
                }}
              >
                {displayedText}
              </span>
              {/* Blinking cursor */}
              <span
                className="font-mono text-2xl font-medium ml-0.5"
                style={{
                  color: theme.primary,
                  opacity: showCursor ? 1 : 0,
                  textShadow: `0 0 10px rgba(${theme.rgb}, 0.5)`,
                  transition: 'opacity 0.1s',
                }}
              >
                _
              </span>
            </div>
          </div>
        </div>
      )}

      {/* Radial pulse on explosion */}
      {phase === 'explode' && (
        <div
          className="absolute inset-0 animate-[flashBurst_0.4s_ease-out_forwards]"
          style={{
            background: `radial-gradient(circle at center, rgba(${theme.rgb}, 0.3) 0%, transparent 60%)`,
          }}
        />
      )}
    </div>
  );
});
