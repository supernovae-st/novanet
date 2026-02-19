'use client';

/**
 * RealmOrbitalCardContent - "Cyberpunk Galaxy Gateway" design for Realm nodes
 *
 * ULTRA Premium Visual Design:
 * - Matrix digital rain canvas background
 * - 6+ animated aurora blobs with enhanced motion
 * - CRT scanline overlay effect
 * - Glitch text animation on hover
 * - Shooting star particles with trails
 * - Electric turbulence SVG border with displacement filter
 * - Pulsing neon glow effects
 * - Hex data stream scrolling
 * - Holographic noise texture with color shifting
 * - Terminal typing effect for description
 * - 3D tilt effect on mouse movement
 *
 * Used for: Realm nodes (shared, org) - top-level hierarchy
 *
 * Visual Encoding:
 * - Shared realm: Cyan/Teal (#2aa198) aurora
 * - Org realm: Violet/Purple (#6c71c4) aurora
 */

import { memo, useMemo, useRef, useCallback, useState, useEffect } from 'react';
import { motion, useMotionValue, useSpring, useTransform, AnimatePresence } from 'framer-motion';
import { cn } from '@/lib/utils';
import type { CardContext } from '../CardShell';
import {
  REALM_COLORS,
  LAYER_COLORS,
  type RealmKey,
  type LayerKey,
} from '@/design/colors/generated';

// =============================================================================
// Types
// =============================================================================

export interface LayerDistributionItem {
  layer: string;
  count: number;
  percentage: number;
}

export interface RealmNodeData {
  id: string;
  type: string;
  key: 'shared' | 'org';
  displayName: string;
  description: string;
  nodeCount: number;
  layerDistribution: LayerDistributionItem[];
}

export interface RealmOrbitalCardProps extends CardContext {
  data: RealmNodeData;
  orbitalConfig?: {
    particleCount?: number;
    rotationSpeed?: number;
    particleSize?: number;
  };
}

// =============================================================================
// Constants
// =============================================================================

const CARD_WIDTH = 420;
const CARD_HEIGHT = 320;
const BORDER_RADIUS = 32;

// Cyberpunk-enhanced realm palettes
const REALM_PALETTES = {
  shared: {
    primary: '#2aa198',
    secondary: '#14b8a6',
    accent: '#06b6d4',
    neon: '#00fff7',
    glow: 'rgba(42, 161, 152, 0.6)',
    matrix: '#00ff88',
  },
  org: {
    primary: '#6c71c4',
    secondary: '#8b5cf6',
    accent: '#a855f7',
    neon: '#ff00ff',
    glow: 'rgba(108, 113, 196, 0.6)',
    matrix: '#ff44ff',
  },
};

// Matrix characters
const MATRIX_CHARS = 'アァカサタナハマヤャラワガザダバパイィキシチニヒミリヰギジヂビピウゥクスツヌフムユュルグズブヅプエェケセテネヘメレヱゲゼデベペオォコソトノホモヨョロヲゴゾドボポヴッン0123456789ABCDEF';

// =============================================================================
// Helper Functions
// =============================================================================

const hexToRgb = (hex: string): string => {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result
    ? `${parseInt(result[1], 16)}, ${parseInt(result[2], 16)}, ${parseInt(result[3], 16)}`
    : '42, 161, 152';
};

// =============================================================================
// Matrix Digital Rain Canvas
// =============================================================================

const MatrixRain = memo(function MatrixRain({
  palette,
  isActive,
}: {
  palette: typeof REALM_PALETTES.shared;
  isActive: boolean;
}) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const columnsRef = useRef<number[]>([]);
  const animationRef = useRef<number>(0);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const fontSize = 10;
    const columns = Math.floor(CARD_WIDTH / fontSize);
    columnsRef.current = Array(columns).fill(1);

    const opacity = isActive ? 0.15 : 0.08;

    const draw = () => {
      ctx.fillStyle = `rgba(0, 0, 0, 0.05)`;
      ctx.fillRect(0, 0, CARD_WIDTH, CARD_HEIGHT);

      ctx.fillStyle = palette.matrix + (isActive ? 'cc' : '66');
      ctx.font = `${fontSize}px monospace`;

      columnsRef.current.forEach((y, i) => {
        const char = MATRIX_CHARS[Math.floor(Math.random() * MATRIX_CHARS.length)];
        const x = i * fontSize;
        ctx.fillText(char, x, y * fontSize);

        if (y * fontSize > CARD_HEIGHT && Math.random() > 0.975) {
          columnsRef.current[i] = 0;
        }
        columnsRef.current[i]++;
      });

      animationRef.current = requestAnimationFrame(draw);
    };

    draw();

    return () => {
      cancelAnimationFrame(animationRef.current);
    };
  }, [palette.matrix, isActive]);

  return (
    <canvas
      ref={canvasRef}
      width={CARD_WIDTH}
      height={CARD_HEIGHT}
      className="absolute inset-0 rounded-[32px] opacity-60 mix-blend-screen pointer-events-none"
      style={{ zIndex: 1 }}
    />
  );
});

// =============================================================================
// Enhanced Aurora Mesh Background (6 blobs)
// =============================================================================

const AuroraMesh = memo(function AuroraMesh({
  palette,
  isActive,
}: {
  palette: typeof REALM_PALETTES.shared;
  isActive: boolean;
}) {
  const blobConfigs = useMemo(() => [
    // Large primary blob
    { size: '70%', top: '-25%', left: '-15%', color: palette.primary, opacity: isActive ? '50' : '30', duration: 18, blur: 'blur-3xl' },
    // Medium secondary blob
    { size: '55%', top: '45%', right: '-20%', left: 'auto', color: palette.secondary, opacity: isActive ? '45' : '25', duration: 14, blur: 'blur-2xl' },
    // Small accent blob
    { size: '40%', bottom: '-15%', left: '25%', top: 'auto', color: palette.accent, opacity: isActive ? '40' : '20', duration: 11, blur: 'blur-xl' },
    // Neon blob 1
    { size: '30%', top: '10%', right: '5%', left: 'auto', color: palette.neon, opacity: isActive ? '35' : '15', duration: 9, blur: 'blur-2xl' },
    // Neon blob 2
    { size: '35%', bottom: '20%', left: '-5%', top: 'auto', color: palette.neon, opacity: isActive ? '30' : '12', duration: 13, blur: 'blur-xl' },
    // Center glow blob
    { size: '50%', top: '25%', left: '25%', color: palette.primary, opacity: isActive ? '25' : '10', duration: 16, blur: 'blur-3xl' },
  ], [palette, isActive]);

  return (
    <div className="absolute inset-0 overflow-hidden rounded-[32px]">
      {/* Deep space base gradient */}
      <div
        className="absolute inset-0"
        style={{
          background: `
            radial-gradient(ellipse 100% 80% at 50% 120%, ${palette.primary}20, transparent 60%),
            radial-gradient(ellipse 80% 100% at 0% 50%, ${palette.secondary}10, transparent 50%),
            radial-gradient(ellipse 80% 100% at 100% 50%, ${palette.accent}10, transparent 50%)
          `,
        }}
      />

      {/* Animated blobs */}
      {blobConfigs.map((config, i) => (
        <motion.div
          key={i}
          className={`absolute rounded-full mix-blend-screen filter ${config.blur}`}
          style={{
            width: config.size,
            height: config.size,
            top: config.top,
            left: config.left,
            right: config.right,
            bottom: config.bottom,
            background: `radial-gradient(circle, ${config.color}${config.opacity}, transparent 70%)`,
          }}
          animate={{
            x: [0, 40 + i * 10, -(30 + i * 8), 0],
            y: [0, -(30 + i * 12), 50 + i * 5, 0],
            scale: [1, 1.1 + i * 0.02, 0.9, 1],
          }}
          transition={{
            duration: config.duration,
            repeat: Infinity,
            repeatType: 'reverse',
            ease: 'easeInOut',
            delay: i * 0.5,
          }}
        />
      ))}

      {/* Holographic noise texture */}
      <motion.div
        className="absolute inset-0 opacity-[0.04]"
        style={{
          backgroundImage: `url("data:image/svg+xml,%3Csvg viewBox='0 0 256 256' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noise'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.8' numOctaves='4' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noise)'/%3E%3C/svg%3E")`,
        }}
        animate={{
          opacity: [0.03, 0.06, 0.03],
        }}
        transition={{
          duration: 3,
          repeat: Infinity,
          ease: 'easeInOut',
        }}
      />
    </div>
  );
});

// =============================================================================
// CRT Scanlines Effect
// =============================================================================

const Scanlines = memo(function Scanlines({ isActive }: { isActive: boolean }) {
  return (
    <div
      className="absolute inset-0 rounded-[32px] pointer-events-none overflow-hidden"
      style={{
        background: `repeating-linear-gradient(
          0deg,
          rgba(0, 0, 0, 0) 0px,
          rgba(0, 0, 0, 0) 1px,
          rgba(0, 0, 0, ${isActive ? 0.15 : 0.08}) 1px,
          rgba(0, 0, 0, ${isActive ? 0.15 : 0.08}) 2px
        )`,
        zIndex: 25,
      }}
    />
  );
});

// =============================================================================
// Electric Turbulence Border (SVG Filter)
// =============================================================================

const ElectricBorder = memo(function ElectricBorder({
  palette,
  isActive,
  uniqueId,
}: {
  palette: typeof REALM_PALETTES.shared;
  isActive: boolean;
  uniqueId: string;
}) {
  return (
    <svg
      className="absolute inset-0 w-full h-full pointer-events-none"
      style={{ zIndex: 10 }}
    >
      <defs>
        {/* Turbulence filter for electric effect */}
        <filter id={`turbulence-${uniqueId}`} x="-20%" y="-20%" width="140%" height="140%">
          <feTurbulence
            type="turbulence"
            baseFrequency="0.02"
            numOctaves="3"
            result="noise"
            seed="1"
          >
            <animate
              attributeName="baseFrequency"
              values="0.02;0.04;0.02"
              dur="4s"
              repeatCount="indefinite"
            />
          </feTurbulence>
          <feDisplacementMap
            in="SourceGraphic"
            in2="noise"
            scale={isActive ? 4 : 2}
            xChannelSelector="R"
            yChannelSelector="B"
          />
        </filter>

        {/* Animated gradient */}
        <linearGradient id={`borderGrad-${uniqueId}`} x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stopColor={palette.neon} stopOpacity={isActive ? 1 : 0.5}>
            <animate attributeName="stopColor" values={`${palette.neon};${palette.accent};${palette.primary};${palette.neon}`} dur="3s" repeatCount="indefinite" />
          </stop>
          <stop offset="50%" stopColor={palette.primary} stopOpacity={isActive ? 0.8 : 0.3}>
            <animate attributeName="stopColor" values={`${palette.primary};${palette.neon};${palette.accent};${palette.primary}`} dur="3s" repeatCount="indefinite" />
          </stop>
          <stop offset="100%" stopColor={palette.accent} stopOpacity={isActive ? 1 : 0.5}>
            <animate attributeName="stopColor" values={`${palette.accent};${palette.primary};${palette.neon};${palette.accent}`} dur="3s" repeatCount="indefinite" />
          </stop>
        </linearGradient>

        {/* Glow filter */}
        <filter id={`glow-${uniqueId}`}>
          <feGaussianBlur stdDeviation={isActive ? 6 : 3} result="coloredBlur" />
          <feMerge>
            <feMergeNode in="coloredBlur" />
            <feMergeNode in="coloredBlur" />
            <feMergeNode in="SourceGraphic" />
          </feMerge>
        </filter>
      </defs>

      {/* Main border with turbulence */}
      <g filter={isActive ? `url(#turbulence-${uniqueId})` : undefined}>
        <rect
          x="1"
          y="1"
          width={CARD_WIDTH - 2}
          height={CARD_HEIGHT - 2}
          rx={BORDER_RADIUS}
          ry={BORDER_RADIUS}
          fill="none"
          stroke={`url(#borderGrad-${uniqueId})`}
          strokeWidth={isActive ? 4 : 2.5}
          strokeDasharray={isActive ? '16 8' : '12 6'}
          filter={`url(#glow-${uniqueId})`}
        >
          <animate
            attributeName="stroke-dashoffset"
            values="0;-48"
            dur="1.5s"
            repeatCount="indefinite"
          />
        </rect>
      </g>

      {/* Inner glow line */}
      <rect
        x="6"
        y="6"
        width={CARD_WIDTH - 12}
        height={CARD_HEIGHT - 12}
        rx={BORDER_RADIUS - 5}
        ry={BORDER_RADIUS - 5}
        fill="none"
        stroke={palette.neon}
        strokeWidth={1}
        strokeOpacity={isActive ? 0.5 : 0.2}
      />
    </svg>
  );
});

// =============================================================================
// Shooting Stars
// =============================================================================

const ShootingStars = memo(function ShootingStars({
  palette,
  isActive,
}: {
  palette: typeof REALM_PALETTES.shared;
  isActive: boolean;
}) {
  const stars = useMemo(() =>
    Array.from({ length: isActive ? 8 : 4 }, (_, i) => ({
      id: i,
      startX: Math.random() * 100,
      startY: Math.random() * 30,
      duration: 1.5 + Math.random() * 2,
      delay: Math.random() * 5,
      size: 1 + Math.random() * 2,
      angle: 30 + Math.random() * 30,
    })),
    [isActive]
  );

  return (
    <div className="absolute inset-0 overflow-hidden rounded-[32px] pointer-events-none" style={{ zIndex: 5 }}>
      {stars.map((star) => (
        <motion.div
          key={star.id}
          className="absolute"
          style={{
            left: `${star.startX}%`,
            top: `${star.startY}%`,
            width: 40 + star.size * 20,
            height: star.size,
            background: `linear-gradient(90deg, transparent, ${palette.neon}, ${palette.primary})`,
            borderRadius: '50%',
            transform: `rotate(${star.angle}deg)`,
            boxShadow: `0 0 ${star.size * 4}px ${palette.neon}`,
          }}
          initial={{ x: 0, y: 0, opacity: 0 }}
          animate={{
            x: [0, 200],
            y: [0, 150],
            opacity: [0, 1, 0],
          }}
          transition={{
            duration: star.duration,
            delay: star.delay,
            repeat: Infinity,
            repeatDelay: 3 + Math.random() * 4,
            ease: 'easeOut',
          }}
        />
      ))}
    </div>
  );
});

// =============================================================================
// Hex Data Stream
// =============================================================================

const HexDataStream = memo(function HexDataStream({
  palette,
  isActive,
}: {
  palette: typeof REALM_PALETTES.shared;
  isActive: boolean;
}) {
  const [hexData, setHexData] = useState<string[]>([]);

  useEffect(() => {
    if (!isActive) return;

    const generateHex = () => {
      const newHex = Array.from({ length: 8 }, () =>
        Math.floor(Math.random() * 256).toString(16).padStart(2, '0').toUpperCase()
      ).join(' ');
      setHexData((prev) => [newHex, ...prev.slice(0, 4)]);
    };

    const interval = setInterval(generateHex, 500);
    generateHex();

    return () => clearInterval(interval);
  }, [isActive]);

  if (!isActive) return null;

  return (
    <div
      className="absolute bottom-12 right-4 text-[8px] font-mono opacity-40 text-right"
      style={{ color: palette.matrix, zIndex: 20 }}
    >
      <AnimatePresence mode="popLayout">
        {hexData.map((hex, i) => (
          <motion.div
            key={hex + i}
            initial={{ opacity: 0, x: 10 }}
            animate={{ opacity: 1 - i * 0.2, x: 0 }}
            exit={{ opacity: 0, x: -10 }}
            transition={{ duration: 0.3 }}
          >
            {hex}
          </motion.div>
        ))}
      </AnimatePresence>
    </div>
  );
});

// =============================================================================
// Clean Title with Subtle Glow (replaced heavy glitch effect)
// =============================================================================

const RealmTitle = memo(function RealmTitle({
  text,
  palette,
  isActive,
  className,
}: {
  text: string;
  palette: typeof REALM_PALETTES.shared;
  isActive: boolean;
  className?: string;
}) {
  return (
    <motion.h2
      className={cn('relative font-bold', className)}
      style={{
        color: '#ffffff',
        textShadow: isActive
          ? `0 0 20px ${palette.neon}80, 0 0 40px ${palette.primary}60`
          : `0 0 10px ${palette.primary}40`,
      }}
      animate={isActive ? {
        textShadow: [
          `0 0 20px ${palette.neon}80, 0 0 40px ${palette.primary}60`,
          `0 0 30px ${palette.neon}90, 0 0 50px ${palette.primary}70`,
          `0 0 20px ${palette.neon}80, 0 0 40px ${palette.primary}60`,
        ],
      } : {}}
      transition={{ duration: 2, repeat: Infinity, ease: 'easeInOut' }}
    >
      {text}
    </motion.h2>
  );
});

// =============================================================================
// Portal Icon with Enhanced Concentric Rings
// =============================================================================

const PortalIcon = memo(function PortalIcon({
  palette,
  isActive,
}: {
  palette: typeof REALM_PALETTES.shared;
  isActive: boolean;
}) {
  return (
    <div className="relative w-16 h-16">
      {/* Outer glow */}
      <motion.div
        className="absolute inset-[-8px] rounded-full"
        style={{
          background: `radial-gradient(circle, ${palette.neon}30, transparent 70%)`,
        }}
        animate={{
          scale: [1, 1.2, 1],
          opacity: [0.5, 0.8, 0.5],
        }}
        transition={{ duration: 2, repeat: Infinity, ease: 'easeInOut' }}
      />

      {/* Outer ring - slow rotation */}
      <motion.div
        className="absolute inset-0 rounded-full"
        style={{
          border: `2px solid ${palette.neon}${isActive ? 'cc' : '60'}`,
          boxShadow: isActive ? `0 0 20px ${palette.neon}, inset 0 0 20px ${palette.neon}40` : 'none',
        }}
        animate={{ rotate: 360 }}
        transition={{ duration: 20, repeat: Infinity, ease: 'linear' }}
      />

      {/* Second ring - opposite direction */}
      <motion.div
        className="absolute inset-1 rounded-full"
        style={{
          border: `1.5px solid ${palette.primary}${isActive ? '90' : '50'}`,
        }}
        animate={{ rotate: -360 }}
        transition={{ duration: 12, repeat: Infinity, ease: 'linear' }}
      />

      {/* Middle ring - dashed */}
      <motion.div
        className="absolute inset-2 rounded-full"
        style={{
          border: `1.5px dashed ${palette.secondary}${isActive ? '80' : '40'}`,
        }}
        animate={{ rotate: -360 }}
        transition={{ duration: 15, repeat: Infinity, ease: 'linear' }}
      />

      {/* Inner ring */}
      <motion.div
        className="absolute inset-3 rounded-full"
        style={{
          border: `1px solid ${palette.accent}${isActive ? '70' : '35'}`,
        }}
        animate={{ rotate: 360 }}
        transition={{ duration: 8, repeat: Infinity, ease: 'linear' }}
      />

      {/* Core - pulsing glow */}
      <motion.div
        className="absolute inset-4 rounded-full"
        style={{
          background: `radial-gradient(circle, ${palette.neon}, ${palette.primary})`,
          boxShadow: `0 0 ${isActive ? 40 : 20}px ${palette.neon}`,
        }}
        animate={{
          scale: [1, 1.2, 1],
          opacity: [0.8, 1, 0.8],
        }}
        transition={{
          duration: 1.5,
          repeat: Infinity,
          ease: 'easeInOut',
        }}
      />

      {/* Center bright dot */}
      <motion.div
        className="absolute inset-[22px] rounded-full bg-white"
        style={{
          boxShadow: `0 0 15px white, 0 0 30px ${palette.neon}`,
        }}
        animate={{
          opacity: [0.9, 1, 0.9],
        }}
        transition={{
          duration: 0.5,
          repeat: Infinity,
          ease: 'easeInOut',
        }}
      />
    </div>
  );
});

// =============================================================================
// Shimmer Effect with Holographic Rainbow
// =============================================================================

const ShimmerOverlay = memo(function ShimmerOverlay({
  isHovered,
  palette,
}: {
  isHovered: boolean;
  palette: typeof REALM_PALETTES.shared;
}) {
  return (
    <motion.div
      className="absolute inset-0 pointer-events-none rounded-[32px] overflow-hidden"
      style={{ zIndex: 15 }}
    >
      {/* Main shimmer sweep */}
      <motion.div
        className="absolute h-full w-[60%]"
        style={{
          background: `linear-gradient(90deg,
            transparent 0%,
            ${palette.neon}15 30%,
            rgba(255,255,255,0.15) 50%,
            ${palette.accent}15 70%,
            transparent 100%)`,
          transform: 'skewX(-20deg)',
        }}
        initial={{ left: '-150%' }}
        animate={{ left: isHovered ? '150%' : '-150%' }}
        transition={{ duration: 0.8, ease: 'easeInOut' }}
      />

      {/* Secondary rainbow shimmer */}
      {isHovered && (
        <motion.div
          className="absolute h-full w-[40%]"
          style={{
            background: `linear-gradient(90deg,
              transparent,
              rgba(255,0,0,0.05),
              rgba(255,255,0,0.05),
              rgba(0,255,0,0.05),
              rgba(0,255,255,0.05),
              rgba(255,0,255,0.05),
              transparent)`,
            transform: 'skewX(-20deg)',
          }}
          initial={{ left: '-100%' }}
          animate={{ left: '150%' }}
          transition={{ duration: 1.2, ease: 'easeInOut', delay: 0.3 }}
        />
      )}
    </motion.div>
  );
});

// =============================================================================
// Layer Pills with Neon Enhancement
// =============================================================================

const LayerPill = memo(function LayerPill({
  layer,
  count,
  palette,
}: {
  layer: string;
  count: number;
  palette: typeof REALM_PALETTES.shared;
}) {
  const layerColor = LAYER_COLORS[layer as LayerKey]?.color ?? palette.primary;

  return (
    <motion.div
      className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-[10px] font-medium"
      style={{
        background: `${layerColor}15`,
        border: `1px solid ${layerColor}50`,
        color: layerColor,
        textShadow: `0 0 10px ${layerColor}`,
      }}
      whileHover={{
        scale: 1.08,
        boxShadow: `0 0 20px ${layerColor}60, inset 0 0 10px ${layerColor}20`,
      }}
      transition={{ type: 'spring', stiffness: 400, damping: 20 }}
    >
      <span className="opacity-90">{layer}</span>
      <span
        className="px-1.5 py-0.5 rounded-full text-[9px] font-bold"
        style={{ background: `${layerColor}40` }}
      >
        {count}
      </span>
    </motion.div>
  );
});

// =============================================================================
// Enhanced Floating Particles
// =============================================================================

const FloatingParticles = memo(function FloatingParticles({
  palette,
  count = 20,
  isActive,
}: {
  palette: typeof REALM_PALETTES.shared;
  count?: number;
  isActive: boolean;
}) {
  const particles = useMemo(() =>
    Array.from({ length: count }, (_, i) => ({
      id: i,
      x: Math.random() * 100,
      y: Math.random() * 100,
      size: 1 + Math.random() * 4,
      duration: 10 + Math.random() * 15,
      delay: Math.random() * 5,
      type: Math.random() > 0.7 ? 'star' : 'dot',
    })),
    [count]
  );

  return (
    <div className="absolute inset-0 overflow-hidden rounded-[32px] pointer-events-none" style={{ zIndex: 3 }}>
      {particles.map((p) => (
        <motion.div
          key={p.id}
          className={p.type === 'star' ? 'absolute' : 'absolute rounded-full'}
          style={{
            width: p.size,
            height: p.size,
            left: `${p.x}%`,
            top: `${p.y}%`,
            background: p.type === 'star'
              ? `conic-gradient(from 0deg, transparent, ${palette.neon}, transparent)`
              : palette.neon,
            boxShadow: `0 0 ${p.size * 3}px ${palette.neon}`,
            clipPath: p.type === 'star' ? 'polygon(50% 0%, 61% 35%, 98% 35%, 68% 57%, 79% 91%, 50% 70%, 21% 91%, 32% 57%, 2% 35%, 39% 35%)' : undefined,
          }}
          animate={{
            y: [0, -40 - p.size * 5, 0],
            x: [0, 20 + p.size * 3, -20 - p.size * 3, 0],
            opacity: isActive ? [0.4, 0.9, 0.4] : [0.2, 0.5, 0.2],
            rotate: p.type === 'star' ? [0, 360] : undefined,
            scale: [1, 1.2, 1],
          }}
          transition={{
            duration: p.duration,
            delay: p.delay,
            repeat: Infinity,
            ease: 'easeInOut',
          }}
        />
      ))}
    </div>
  );
});

// =============================================================================
// L-Shaped Corner Brackets (Tech/Cyberpunk Style) - SVG version for clean rendering
// =============================================================================

const TechCornerBrackets = memo(function TechCornerBrackets({
  palette,
  isActive,
  uniqueId,
}: {
  palette: typeof REALM_PALETTES.shared;
  isActive: boolean;
  uniqueId: string;
}) {
  const bracketLength = 16;
  const strokeWidth = 1.5;
  const offset = 10;

  return (
    <svg
      className="absolute inset-0 w-full h-full pointer-events-none"
      style={{ zIndex: 50 }}
      viewBox={`0 0 ${CARD_WIDTH} ${CARD_HEIGHT}`}
    >
      <defs>
        <filter id={`bracketGlow-${uniqueId}`}>
          <feGaussianBlur stdDeviation={isActive ? 3 : 1.5} result="blur" />
          <feMerge>
            <feMergeNode in="blur" />
            <feMergeNode in="SourceGraphic" />
          </feMerge>
        </filter>
      </defs>

      <g
        stroke={palette.neon}
        strokeWidth={strokeWidth}
        fill="none"
        strokeLinecap="square"
        filter={`url(#bracketGlow-${uniqueId})`}
        opacity={isActive ? 0.9 : 0.5}
      >
        {/* Top-left L */}
        <path d={`M ${offset} ${offset + bracketLength} L ${offset} ${offset} L ${offset + bracketLength} ${offset}`} />

        {/* Top-right L */}
        <path d={`M ${CARD_WIDTH - offset - bracketLength} ${offset} L ${CARD_WIDTH - offset} ${offset} L ${CARD_WIDTH - offset} ${offset + bracketLength}`} />

        {/* Bottom-left L */}
        <path d={`M ${offset} ${CARD_HEIGHT - offset - bracketLength} L ${offset} ${CARD_HEIGHT - offset} L ${offset + bracketLength} ${CARD_HEIGHT - offset}`} />

        {/* Bottom-right L */}
        <path d={`M ${CARD_WIDTH - offset - bracketLength} ${CARD_HEIGHT - offset} L ${CARD_WIDTH - offset} ${CARD_HEIGHT - offset} L ${CARD_WIDTH - offset} ${CARD_HEIGHT - offset - bracketLength}`} />
      </g>
    </svg>
  );
});

// =============================================================================
// Grid Icon in Rounded Square (Left side element)
// =============================================================================

const GridIcon = memo(function GridIcon({
  palette,
  isActive,
}: {
  palette: typeof REALM_PALETTES.shared;
  isActive: boolean;
}) {
  return (
    <motion.div
      className="relative w-14 h-14 rounded-xl flex items-center justify-center"
      style={{
        background: `linear-gradient(135deg, ${palette.primary}25, ${palette.secondary}15)`,
        border: `1.5px solid ${palette.neon}${isActive ? '80' : '40'}`,
        boxShadow: isActive
          ? `0 0 20px ${palette.neon}40, inset 0 0 15px ${palette.primary}20`
          : `inset 0 0 10px ${palette.primary}10`,
      }}
      animate={isActive ? {
        boxShadow: [
          `0 0 20px ${palette.neon}40, inset 0 0 15px ${palette.primary}20`,
          `0 0 30px ${palette.neon}60, inset 0 0 20px ${palette.primary}30`,
          `0 0 20px ${palette.neon}40, inset 0 0 15px ${palette.primary}20`,
        ],
      } : {}}
      transition={{ duration: 2, repeat: Infinity, ease: 'easeInOut' }}
    >
      {/* 3x3 Grid */}
      <div className="grid grid-cols-3 gap-1">
        {Array.from({ length: 9 }).map((_, i) => (
          <motion.div
            key={i}
            className="w-2.5 h-2.5 rounded-sm"
            style={{
              background: i === 4
                ? palette.neon
                : `${palette.primary}${isActive ? '90' : '60'}`,
              boxShadow: i === 4 && isActive ? `0 0 8px ${palette.neon}` : 'none',
            }}
            animate={isActive ? {
              opacity: [0.6, 1, 0.6],
              scale: i === 4 ? [1, 1.1, 1] : [1, 1.05, 1],
            } : {}}
            transition={{
              duration: 1.5,
              delay: i * 0.1,
              repeat: Infinity,
              ease: 'easeInOut',
            }}
          />
        ))}
      </div>
    </motion.div>
  );
});

// =============================================================================
// Badge Component (for REALM, LAYER style badges)
// =============================================================================

const TechBadge = memo(function TechBadge({
  label,
  value,
  icon,
  palette,
  isActive,
  variant = 'default',
}: {
  label: string;
  value?: string | number;
  icon?: React.ReactNode;
  palette: typeof REALM_PALETTES.shared;
  isActive: boolean;
  variant?: 'default' | 'primary' | 'accent';
}) {
  const variantColors = {
    default: { bg: palette.primary, border: palette.primary, text: palette.neon },
    primary: { bg: palette.neon, border: palette.neon, text: '#000' },
    accent: { bg: palette.accent, border: palette.accent, text: '#fff' },
  };
  const colors = variantColors[variant];

  return (
    <motion.div
      className="inline-flex items-center gap-1.5 px-2 py-1 rounded-md"
      style={{
        background: `${colors.bg}15`,
        border: `1px solid ${colors.border}${isActive ? '70' : '40'}`,
        boxShadow: isActive ? `0 0 10px ${colors.bg}30` : 'none',
      }}
      whileHover={{ scale: 1.05 }}
    >
      {icon && (
        <span style={{ color: colors.border, opacity: 0.9 }}>
          {icon}
        </span>
      )}
      <span
        className="text-[10px] font-bold uppercase tracking-wider font-mono"
        style={{
          color: colors.border,
          textShadow: isActive ? `0 0 8px ${colors.border}` : 'none',
        }}
      >
        {label}
      </span>
      {value !== undefined && (
        <span
          className="text-[10px] font-bold font-mono px-1.5 py-0.5 rounded"
          style={{
            background: `${colors.border}30`,
            color: colors.text,
          }}
        >
          {value}
        </span>
      )}
    </motion.div>
  );
});

// =============================================================================
// Main Component
// =============================================================================

export const RealmOrbitalCardContent = memo(function RealmOrbitalCardContent({
  data,
  colors,
  selected,
  isHovered,
}: RealmOrbitalCardProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const [mousePosition, setMousePosition] = useState({ x: 0.5, y: 0.5 });
  const uniqueId = useMemo(() => Math.random().toString(36).substr(2, 9), []);

  // Safe defaults
  const nodeCount = data.nodeCount ?? 0;
  const layerDistribution = data.layerDistribution ?? [];
  const description = data.description ?? '';
  const realmKey = (data.key ?? 'shared') as 'shared' | 'org';

  // Get palette
  const palette = REALM_PALETTES[realmKey] ?? REALM_PALETTES.shared;
  const isActive = selected || isHovered;

  // 3D Tilt effect
  const mouseX = useMotionValue(0.5);
  const mouseY = useMotionValue(0.5);

  const rotateX = useSpring(useTransform(mouseY, [0, 1], [12, -12]), { stiffness: 300, damping: 30 });
  const rotateY = useSpring(useTransform(mouseX, [0, 1], [-12, 12]), { stiffness: 300, damping: 30 });

  const handleMouseMove = useCallback((e: React.MouseEvent) => {
    if (!containerRef.current) return;
    const rect = containerRef.current.getBoundingClientRect();
    const x = (e.clientX - rect.left) / rect.width;
    const y = (e.clientY - rect.top) / rect.height;
    mouseX.set(x);
    mouseY.set(y);
    setMousePosition({ x, y });
  }, [mouseX, mouseY]);

  const handleMouseLeave = useCallback(() => {
    mouseX.set(0.5);
    mouseY.set(0.5);
    setMousePosition({ x: 0.5, y: 0.5 });
  }, [mouseX, mouseY]);

  // Spotlight style
  const spotlightStyle = useMemo(() => ({
    background: isActive
      ? `radial-gradient(circle at ${mousePosition.x * 100}% ${mousePosition.y * 100}%, ${palette.neon}25 0%, transparent 50%)`
      : 'none',
  }), [isActive, mousePosition, palette.neon]);

  return (
    <motion.div
      ref={containerRef}
      className="relative cursor-pointer"
      style={{
        width: CARD_WIDTH,
        height: CARD_HEIGHT,
        borderRadius: BORDER_RADIUS,
        perspective: 1200,
        transformStyle: 'preserve-3d',
      }}
      onMouseMove={handleMouseMove}
      onMouseLeave={handleMouseLeave}
    >
      <motion.div
        className="relative w-full h-full"
        style={{
          rotateX: isActive ? rotateX : 0,
          rotateY: isActive ? rotateY : 0,
          transformStyle: 'preserve-3d',
          borderRadius: BORDER_RADIUS,
          background: 'linear-gradient(135deg, rgba(10,10,15,0.98), rgba(5,5,10,0.99))',
          boxShadow: isActive
            ? `0 30px 60px -15px rgba(0,0,0,0.6),
               0 0 80px ${palette.neon}40,
               0 0 120px ${palette.primary}20,
               inset 0 1px 0 rgba(255,255,255,0.1)`
            : `0 15px 40px -10px rgba(0,0,0,0.5),
               0 0 40px ${palette.primary}15`,
        }}
        whileHover={{ scale: 1.02 }}
        transition={{ type: 'spring', stiffness: 300, damping: 25 }}
      >
        {/* Background effects */}
        <MatrixRain palette={palette} isActive={isActive} />
        <AuroraMesh palette={palette} isActive={isActive} />
        <ElectricBorder palette={palette} isActive={isActive} uniqueId={uniqueId} />
        <ShootingStars palette={palette} isActive={isActive} />
        <FloatingParticles palette={palette} count={25} isActive={isActive} />
        <Scanlines isActive={isActive} />
        <ShimmerOverlay isHovered={isHovered ?? false} palette={palette} />
        <TechCornerBrackets palette={palette} isActive={isActive} uniqueId={uniqueId} />
        <HexDataStream palette={palette} isActive={isActive} />

        {/* Spotlight */}
        <div
          className="absolute inset-0 rounded-[32px] pointer-events-none transition-all duration-300"
          style={{ ...spotlightStyle, zIndex: 16 }}
        />

        {/* Content */}
        <div className="relative z-30 p-8 h-full flex flex-col">
          {/* Header */}
          <div className="flex items-start justify-between mb-4">
            <div className="flex items-center gap-4">
              <PortalIcon palette={palette} isActive={isActive} />
              <div>
                <motion.div
                  className="text-[10px] font-bold uppercase tracking-[0.3em] mb-1 font-mono"
                  style={{
                    color: palette.neon,
                    textShadow: `0 0 10px ${palette.neon}`,
                  }}
                  initial={{ opacity: 0, y: -10 }}
                  animate={{ opacity: 1, y: 0 }}
                >
                  {'<REALM/>'}
                </motion.div>
                <RealmTitle
                  text={data.displayName}
                  palette={palette}
                  isActive={isActive}
                  className="text-3xl"
                />
              </div>
            </div>

            {/* Stats badge */}
            <motion.div
              className="flex flex-col items-end"
              initial={{ opacity: 0, scale: 0.8 }}
              animate={{ opacity: 1, scale: 1 }}
              transition={{ delay: 0.2 }}
            >
              <motion.div
                className="text-2xl font-bold font-mono"
                style={{
                  color: palette.neon,
                  textShadow: `0 0 20px ${palette.neon}`,
                }}
                animate={isActive ? {
                  textShadow: [`0 0 20px ${palette.neon}`, `0 0 40px ${palette.neon}`, `0 0 20px ${palette.neon}`],
                } : {}}
                transition={{ duration: 1.5, repeat: Infinity }}
              >
                {nodeCount}
              </motion.div>
              <div
                className="text-[9px] text-white/50 uppercase tracking-wider font-mono"
                style={{ textShadow: `0 0 5px ${palette.primary}` }}
              >
                ::CLASSES
              </div>
            </motion.div>
          </div>

          {/* Description */}
          {description && (
            <motion.p
              className="text-sm text-white/60 leading-relaxed mb-4 max-w-[80%] font-mono"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              transition={{ delay: 0.2 }}
              style={{ textShadow: '0 0 5px rgba(0,0,0,0.5)' }}
            >
              {'> '}{description}
            </motion.p>
          )}

          {/* Spacer */}
          <div className="flex-1" />

          {/* Layer distribution */}
          {layerDistribution.length > 0 && (
            <motion.div
              className="flex flex-wrap gap-2"
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.3 }}
            >
              {layerDistribution.slice(0, 6).map((item, index) => (
                <motion.div
                  key={item.layer}
                  initial={{ opacity: 0, scale: 0.8, y: 10 }}
                  animate={{ opacity: 1, scale: 1, y: 0 }}
                  transition={{ delay: 0.3 + index * 0.08 }}
                >
                  <LayerPill
                    layer={item.layer}
                    count={item.count}
                    palette={palette}
                  />
                </motion.div>
              ))}
            </motion.div>
          )}

          {/* Bottom gradient line with pulse */}
          <motion.div
            className="absolute bottom-0 left-8 right-8 h-[2px] rounded-full"
            style={{
              background: `linear-gradient(90deg, transparent, ${palette.neon}, ${palette.primary}, ${palette.neon}, transparent)`,
              boxShadow: `0 0 10px ${palette.neon}`,
            }}
            initial={{ scaleX: 0, opacity: 0 }}
            animate={{
              scaleX: 1,
              opacity: 1,
              boxShadow: isActive
                ? [`0 0 10px ${palette.neon}`, `0 0 25px ${palette.neon}`, `0 0 10px ${palette.neon}`]
                : `0 0 10px ${palette.neon}`,
            }}
            transition={{
              scaleX: { delay: 0.4, duration: 0.6 },
              opacity: { delay: 0.4, duration: 0.6 },
              boxShadow: { duration: 2, repeat: Infinity, ease: 'easeInOut' },
            }}
          />
        </div>
      </motion.div>
    </motion.div>
  );
});
