'use client';

/**
 * MatrixExplosionOverlay - Matrix Rain Easter Egg
 *
 * Architecture:
 * - State machine for animation phases (IDLE → RUNNING → FADING → IDLE)
 * - Single requestAnimationFrame loop for all animations
 * - Debounced activation to prevent rapid-click bugs
 * - Object pooling for particles (zero GC during animation)
 * - Direct DOM manipulation (no React re-renders during animation)
 *
 * Performance:
 * - GPU-only operations (opacity, transform)
 * - Pre-allocated particle pool
 * - Single composite layer for canvas
 */

import { memo, useEffect, useRef, useCallback } from 'react';
import type { NavigationMode } from '@/stores/uiStore';

// ============================================================================
// Configuration
// ============================================================================

const CONFIG = {
  // Characters
  matrixChars: 'アイウエオカキクケコ0123456789<>{}[]',

  // Timing (ms)
  debounceMs: 100,
  promptDelayMs: 350,
  promptFadeInMs: 300,
  promptDurationMs: 1200,
  promptFadeOutMs: 400,
  rainFadeInMs: 250,
  rainFadeOutMs: 600,
  totalDurationMs: 2600,

  // Canvas
  fontSize: 18,
  trailOpacity: 0.12,
  maxCanvasOpacity: 0.55,

  // Particles
  particleCount: 30,
  particleMinSize: 14,
  particleMaxSize: 30,
  particleMinSpeed: 5,
  particleMaxSpeed: 15,
  particleGravity: 0.3,
  particleFriction: 0.98,
  particleDecay: 0.97,

  // Typewriter
  charDelayMs: 45,
} as const;

const THEMES: Record<NavigationMode, { primary: string; rgb: string; name: string }> = {
  data: { primary: '#34d399', rgb: '52, 211, 153', name: 'data' },
  meta: { primary: '#60a5fa', rgb: '96, 165, 250', name: 'meta' },
  overlay: { primary: '#a78bfa', rgb: '167, 139, 250', name: 'overlay' },
  query: { primary: '#fbbf24', rgb: '251, 191, 36', name: 'query' },
};

// ============================================================================
// Types
// ============================================================================

type AnimationPhase = 'idle' | 'running' | 'fading';

interface Particle {
  active: boolean;
  x: number;
  y: number;
  vx: number;
  vy: number;
  size: number;
  opacity: number;
  char: string;
  element: HTMLSpanElement | null;
}

interface AnimationController {
  phase: AnimationPhase;
  startTime: number;
  canvasOpacity: number;
  promptOpacity: number;
  textIndex: number;
  particles: Particle[];
  rafId: number;
  drops: number[];
}

// ============================================================================
// Utilities
// ============================================================================

const easeOutCubic = (t: number) => 1 - Math.pow(1 - t, 3);
const easeInCubic = (t: number) => t * t * t;
const clamp = (v: number, min: number, max: number) => Math.min(Math.max(v, min), max);
const randomChar = () => CONFIG.matrixChars[Math.floor(Math.random() * CONFIG.matrixChars.length)];

function lerp(start: number, end: number, progress: number, easing: (t: number) => number): number {
  return start + (end - start) * easing(clamp(progress, 0, 1));
}

// Pre-allocate particle pool
function createParticlePool(): Particle[] {
  return Array.from({ length: CONFIG.particleCount }, () => ({
    active: false,
    x: 0,
    y: 0,
    vx: 0,
    vy: 0,
    size: 0,
    opacity: 0,
    char: '',
    element: null,
  }));
}

function activateParticles(particles: Particle[], centerX: number, centerY: number): void {
  particles.forEach((p, i) => {
    const angle = (Math.PI * 2 * i) / particles.length + Math.random() * 0.3;
    const speed = CONFIG.particleMinSpeed + Math.random() * (CONFIG.particleMaxSpeed - CONFIG.particleMinSpeed);

    p.active = true;
    p.x = centerX;
    p.y = centerY;
    p.vx = Math.cos(angle) * speed;
    p.vy = Math.sin(angle) * speed;
    p.size = CONFIG.particleMinSize + Math.random() * (CONFIG.particleMaxSize - CONFIG.particleMinSize);
    p.opacity = 0.9;
    p.char = randomChar();
  });
}

function updateParticle(p: Particle): void {
  if (!p.active) return;

  p.x += p.vx;
  p.y += p.vy;
  p.vy += CONFIG.particleGravity;
  p.vx *= CONFIG.particleFriction;
  p.opacity *= CONFIG.particleDecay;

  if (p.opacity < 0.02) {
    p.active = false;
  }
}

// ============================================================================
// Component
// ============================================================================

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
  // Refs for DOM elements
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const promptRef = useRef<HTMLDivElement>(null);
  const textRef = useRef<HTMLSpanElement>(null);
  const particlesContainerRef = useRef<HTMLDivElement>(null);

  // Animation controller (mutable, no re-renders)
  const ctrlRef = useRef<AnimationController>({
    phase: 'idle',
    startTime: 0,
    canvasOpacity: 0,
    promptOpacity: 0,
    textIndex: 0,
    particles: createParticlePool(),
    rafId: 0,
    drops: [],
  });

  // Debounce ref
  const lastActivationRef = useRef(0);

  const theme = THEMES[navigationMode] || THEMES.meta;
  const fullText = `> ${theme.name}`;

  // Initialize canvas
  const initCanvas = useCallback(() => {
    const canvas = canvasRef.current;
    if (!canvas) return null;

    const ctx = canvas.getContext('2d', { alpha: true });
    if (!ctx) return null;

    const dpr = window.devicePixelRatio || 1;
    const width = window.innerWidth;
    const height = window.innerHeight;

    canvas.width = width * dpr;
    canvas.height = height * dpr;
    canvas.style.width = `${width}px`;
    canvas.style.height = `${height}px`;
    ctx.scale(dpr, dpr);

    // Initialize drops
    const columns = Math.ceil(width / CONFIG.fontSize);
    ctrlRef.current.drops = Array.from({ length: columns }, () => Math.random() * -30);

    return { ctx, width, height };
  }, []);

  // Create particle DOM elements
  const initParticleElements = useCallback(() => {
    const container = particlesContainerRef.current;
    if (!container) return;

    // Remove existing children safely
    while (container.firstChild) {
      container.removeChild(container.firstChild);
    }

    // Create elements for pool
    ctrlRef.current.particles.forEach((p) => {
      const span = document.createElement('span');
      span.className = 'absolute font-mono font-bold';
      span.style.cssText = 'opacity: 0; will-change: transform, opacity;';
      container.appendChild(span);
      p.element = span;
    });
  }, []);

  // Main animation loop
  const animate = useCallback((canvasCtx: CanvasRenderingContext2D, width: number, height: number) => {
    const ctrl = ctrlRef.current;
    const now = performance.now();
    const elapsed = now - ctrl.startTime;

    // === Phase transitions ===
    if (ctrl.phase === 'running' && elapsed >= CONFIG.promptDurationMs + CONFIG.promptDelayMs) {
      ctrl.phase = 'fading';
    }

    // === Canvas opacity ===
    let targetCanvasOpacity = 0;

    if (ctrl.phase === 'running') {
      // Fade in
      const fadeInProgress = elapsed / CONFIG.rainFadeInMs;
      targetCanvasOpacity = lerp(0, CONFIG.maxCanvasOpacity, fadeInProgress, easeOutCubic);
    } else if (ctrl.phase === 'fading') {
      // Fade out
      const fadeStart = CONFIG.promptDurationMs + CONFIG.promptDelayMs;
      const fadeProgress = (elapsed - fadeStart) / CONFIG.rainFadeOutMs;
      targetCanvasOpacity = lerp(CONFIG.maxCanvasOpacity, 0, fadeProgress, easeInCubic);
    }

    ctrl.canvasOpacity = clamp(targetCanvasOpacity, 0, CONFIG.maxCanvasOpacity);

    if (canvasRef.current) {
      canvasRef.current.style.opacity = String(ctrl.canvasOpacity);
    }

    // === Render matrix rain ===
    if (ctrl.canvasOpacity > 0) {
      canvasCtx.fillStyle = `rgba(10, 10, 15, ${CONFIG.trailOpacity})`;
      canvasCtx.fillRect(0, 0, width, height);

      canvasCtx.font = `${CONFIG.fontSize}px monospace`;

      for (let i = 0; i < ctrl.drops.length; i++) {
        const brightness = Math.random() > 0.9 ? 1 : 0.3 + Math.random() * 0.4;
        canvasCtx.fillStyle = `rgba(${theme.rgb}, ${brightness})`;
        canvasCtx.fillText(randomChar(), i * CONFIG.fontSize, ctrl.drops[i] * CONFIG.fontSize);

        if (ctrl.drops[i] * CONFIG.fontSize > height && Math.random() > 0.95) {
          ctrl.drops[i] = 0;
        }
        ctrl.drops[i] += 0.6 + Math.random() * 0.4;
      }
    }

    // === Prompt opacity ===
    let targetPromptOpacity = 0;

    if (ctrl.phase === 'running' && elapsed >= CONFIG.promptDelayMs) {
      const promptElapsed = elapsed - CONFIG.promptDelayMs;
      if (promptElapsed < CONFIG.promptFadeInMs) {
        targetPromptOpacity = lerp(0, 1, promptElapsed / CONFIG.promptFadeInMs, easeOutCubic);
      } else {
        targetPromptOpacity = 1;
      }
    } else if (ctrl.phase === 'fading') {
      const fadeStart = CONFIG.promptDurationMs + CONFIG.promptDelayMs;
      const fadeProgress = (elapsed - fadeStart) / CONFIG.promptFadeOutMs;
      targetPromptOpacity = lerp(1, 0, fadeProgress, easeInCubic);
    }

    ctrl.promptOpacity = clamp(targetPromptOpacity, 0, 1);

    if (promptRef.current) {
      promptRef.current.style.opacity = String(ctrl.promptOpacity);
    }

    // === Typewriter ===
    if (ctrl.phase === 'running' && elapsed >= CONFIG.promptDelayMs) {
      const typeElapsed = elapsed - CONFIG.promptDelayMs;
      const targetIndex = Math.min(
        Math.floor(typeElapsed / CONFIG.charDelayMs),
        fullText.length
      );

      if (targetIndex !== ctrl.textIndex) {
        ctrl.textIndex = targetIndex;
        if (textRef.current) {
          textRef.current.textContent = fullText.slice(0, targetIndex);
        }
      }
    }

    // === Particles ===
    const particlesStarted = elapsed >= CONFIG.promptDelayMs;

    if (particlesStarted) {
      // Activate particles on first frame after delay
      if (ctrl.particles[0] && !ctrl.particles[0].active && ctrl.phase === 'running') {
        activateParticles(ctrl.particles, width / 2, height / 2);
      }

      // Update and render
      ctrl.particles.forEach((p) => {
        updateParticle(p);

        if (p.element) {
          if (p.active) {
            p.element.textContent = p.char;
            p.element.style.cssText = `
              position: absolute;
              left: ${p.x}px;
              top: ${p.y}px;
              font-size: ${p.size}px;
              color: ${theme.primary};
              opacity: ${p.opacity};
              transform: translate(-50%, -50%);
              text-shadow: 0 0 6px rgba(${theme.rgb}, 0.5);
              will-change: transform, opacity;
            `;
          } else {
            p.element.style.opacity = '0';
          }
        }
      });
    }

    // === Continue or complete ===
    if (elapsed < CONFIG.totalDurationMs) {
      ctrl.rafId = requestAnimationFrame(() => animate(canvasCtx, width, height));
    } else {
      // Animation complete
      ctrl.phase = 'idle';
      ctrl.canvasOpacity = 0;
      ctrl.promptOpacity = 0;
      ctrl.textIndex = 0;
      ctrl.particles.forEach((p) => {
        p.active = false;
        if (p.element) p.element.style.opacity = '0';
      });

      if (canvasRef.current) canvasRef.current.style.opacity = '0';
      if (promptRef.current) promptRef.current.style.opacity = '0';
      if (textRef.current) textRef.current.textContent = '';

      // Clear canvas
      canvasCtx.clearRect(0, 0, width, height);

      onComplete();
    }
  }, [theme, fullText, onComplete]);

  // Start animation
  const startAnimation = useCallback(() => {
    const canvasData = initCanvas();
    if (!canvasData) return;

    initParticleElements();

    const ctrl = ctrlRef.current;
    ctrl.phase = 'running';
    ctrl.startTime = performance.now();
    ctrl.canvasOpacity = 0;
    ctrl.promptOpacity = 0;
    ctrl.textIndex = 0;
    ctrl.particles.forEach((p) => (p.active = false));

    // Show container
    if (containerRef.current) {
      containerRef.current.style.display = 'block';
    }

    animate(canvasData.ctx, canvasData.width, canvasData.height);
  }, [initCanvas, initParticleElements, animate]);

  // Stop animation
  const stopAnimation = useCallback(() => {
    const ctrl = ctrlRef.current;

    cancelAnimationFrame(ctrl.rafId);
    ctrl.phase = 'idle';
    ctrl.canvasOpacity = 0;
    ctrl.promptOpacity = 0;

    if (canvasRef.current) canvasRef.current.style.opacity = '0';
    if (promptRef.current) promptRef.current.style.opacity = '0';
    if (textRef.current) textRef.current.textContent = '';
    if (containerRef.current) containerRef.current.style.display = 'none';

    ctrl.particles.forEach((p) => {
      p.active = false;
      if (p.element) p.element.style.opacity = '0';
    });
  }, []);

  // Handle activation changes
  useEffect(() => {
    const ctrl = ctrlRef.current;
    const now = Date.now();

    if (isActive) {
      // Debounce rapid clicks
      if (now - lastActivationRef.current < CONFIG.debounceMs) {
        return;
      }

      // Don't restart if already running
      if (ctrl.phase !== 'idle') {
        return;
      }

      lastActivationRef.current = now;
      startAnimation();
    } else {
      // Only stop if we're running (allows natural completion)
      if (ctrl.phase !== 'idle') {
        stopAnimation();
      }
    }

    return () => {
      if (ctrl.phase !== 'idle') {
        cancelAnimationFrame(ctrl.rafId);
      }
    };
  }, [isActive, startAnimation, stopAnimation]);

  return (
    <div
      ref={containerRef}
      className="fixed inset-0 z-[9999] pointer-events-none overflow-hidden"
      style={{ display: 'none' }}
    >
      {/* Matrix rain */}
      <canvas
        ref={canvasRef}
        className="absolute inset-0"
        style={{ opacity: 0 }}
      />

      {/* Particles */}
      <div ref={particlesContainerRef} className="absolute inset-0" />

      {/* Terminal prompt */}
      <div
        ref={promptRef}
        className="absolute inset-0 flex items-center justify-center"
        style={{ opacity: 0 }}
      >
        <div
          className="px-5 py-3 rounded-md"
          style={{
            background: 'rgba(0, 0, 0, 0.6)',
            border: `1px solid rgba(${theme.rgb}, 0.3)`,
            boxShadow: `0 0 30px rgba(${theme.rgb}, 0.15)`,
          }}
        >
          <span
            className="font-mono text-lg tracking-wider"
            style={{
              color: theme.primary,
              textShadow: `0 0 8px rgba(${theme.rgb}, 0.4)`,
            }}
          >
            <span ref={textRef} />
            <span
              className="inline-block w-2 h-4 ml-1 align-middle animate-pulse"
              style={{ background: theme.primary }}
            />
          </span>
        </div>
      </div>
    </div>
  );
});
