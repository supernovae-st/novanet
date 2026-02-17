'use client';

/**
 * PerformanceContext - Adaptive performance tiers for card visual effects
 *
 * Controls effect complexity based on visible node count:
 * - ULTRA (0-20 nodes):   All effects enabled, spring animations
 * - HIGH (21-50 nodes):   Most effects, no particles/matrixRain
 * - MEDIUM (51-100):      Essential effects only
 * - LOW (101-200):        Minimal effects, no animations
 * - MINIMAL (200+):       Static rendering, no effects
 */

import { createContext, useContext, useMemo, type ReactNode } from 'react';

// =============================================================================
// Types
// =============================================================================

export type PerformanceTier = 'ULTRA' | 'HIGH' | 'MEDIUM' | 'LOW' | 'MINIMAL';

export interface PerformanceConfig {
  tier: PerformanceTier;
  effects: {
    techCorners: boolean;
    scanlines: boolean;
    gridPattern: boolean;
    shimmer: boolean;
    matrixRain: boolean;
    outerGlow: boolean;
    particles: boolean;
    glassmorphism: boolean;
  };
  animation: {
    enabled: boolean;
    duration: 'fast' | 'normal' | 'slow' | 'none';
    spring: boolean;
  };
}

// =============================================================================
// Constants
// =============================================================================

const TIER_THRESHOLDS = {
  ULTRA: 20,
  HIGH: 50,
  MEDIUM: 100,
  LOW: 200,
  MINIMAL: Infinity,
} as const;

const TIER_CONFIGS: Record<PerformanceTier, PerformanceConfig> = {
  ULTRA: {
    tier: 'ULTRA',
    effects: {
      techCorners: true,
      scanlines: true,
      gridPattern: true,
      shimmer: true,
      matrixRain: true,
      outerGlow: true,
      particles: true,
      glassmorphism: true,
    },
    animation: { enabled: true, duration: 'normal', spring: true },
  },
  HIGH: {
    tier: 'HIGH',
    effects: {
      techCorners: true,
      scanlines: true,
      gridPattern: true,
      shimmer: true,
      matrixRain: false,
      outerGlow: true,
      particles: false,
      glassmorphism: true,
    },
    animation: { enabled: true, duration: 'fast', spring: true },
  },
  MEDIUM: {
    tier: 'MEDIUM',
    effects: {
      techCorners: true,
      scanlines: false,
      gridPattern: true,
      shimmer: false,
      matrixRain: false,
      outerGlow: true,
      particles: false,
      glassmorphism: false,
    },
    animation: { enabled: true, duration: 'fast', spring: false },
  },
  LOW: {
    tier: 'LOW',
    effects: {
      techCorners: true,
      scanlines: false,
      gridPattern: false,
      shimmer: false,
      matrixRain: false,
      outerGlow: false,
      particles: false,
      glassmorphism: false,
    },
    animation: { enabled: false, duration: 'none', spring: false },
  },
  MINIMAL: {
    tier: 'MINIMAL',
    effects: {
      techCorners: false,
      scanlines: false,
      gridPattern: false,
      shimmer: false,
      matrixRain: false,
      outerGlow: false,
      particles: false,
      glassmorphism: false,
    },
    animation: { enabled: false, duration: 'none', spring: false },
  },
};

// =============================================================================
// Helpers
// =============================================================================

function getTierFromNodeCount(count: number): PerformanceTier {
  if (count <= TIER_THRESHOLDS.ULTRA) return 'ULTRA';
  if (count <= TIER_THRESHOLDS.HIGH) return 'HIGH';
  if (count <= TIER_THRESHOLDS.MEDIUM) return 'MEDIUM';
  if (count <= TIER_THRESHOLDS.LOW) return 'LOW';
  return 'MINIMAL';
}

// =============================================================================
// Context
// =============================================================================

interface PerformanceContextValue {
  tier: PerformanceTier;
  config: PerformanceConfig;
  nodeCount: number;
}

const PerformanceContext = createContext<PerformanceContextValue | null>(null);

// =============================================================================
// Provider
// =============================================================================

interface PerformanceProviderProps {
  children: ReactNode;
  nodeCount: number;
  overrideTier?: PerformanceTier;
}

export function PerformanceProvider({
  children,
  nodeCount,
  overrideTier,
}: PerformanceProviderProps) {
  const value = useMemo(() => {
    const tier = overrideTier ?? getTierFromNodeCount(nodeCount);
    return {
      tier,
      config: TIER_CONFIGS[tier],
      nodeCount,
    };
  }, [nodeCount, overrideTier]);

  return (
    <PerformanceContext.Provider value={value}>
      {children}
    </PerformanceContext.Provider>
  );
}

// =============================================================================
// Hook
// =============================================================================

export function usePerformance(): PerformanceContextValue {
  const context = useContext(PerformanceContext);
  if (!context) {
    throw new Error('usePerformance must be used within PerformanceProvider');
  }
  return context;
}

// =============================================================================
// Exports for testing/configuration
// =============================================================================

export { TIER_CONFIGS, TIER_THRESHOLDS, getTierFromNodeCount };
