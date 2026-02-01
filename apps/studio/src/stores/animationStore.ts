/**
 * Animation Registry Store
 *
 * Tracks animated edges and enforces the maxConcurrent limit
 * from DEFAULT_ANIMATION_BUDGET to prevent animation overload.
 *
 * Also manages animation settings:
 * - Animation mode: Full / Reduced / Off
 * - Intensity level (0-100%)
 * - Individual effect toggles
 * - Accessibility (prefers-reduced-motion)
 *
 * This store is critical for performance optimization:
 * - Limits concurrent animations to prevent browser strain
 * - Provides O(1) lookup for animation eligibility
 * - Enables coordinated animation management across all edges
 */

import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { DEFAULT_ANIMATION_BUDGET } from '@/components/graph/edges/system/constants';

/** Animation mode levels */
export type AnimationMode = 'full' | 'reduced' | 'off';

/** Mode transition phases */
export type TransitionPhase = 'dissolve' | 'fetch' | 'reform' | null;

/** Data mode for transitions */
export type DataMode = 'data' | 'schema';

/** Animation settings state */
interface AnimationSettings {
  /** Current animation mode */
  mode: AnimationMode;
  /** Global intensity multiplier (0-100, maps to 0-2 range) */
  intensity: number;
  /** Enable edge particle effects (EMIT → TRAVEL → IMPACT) */
  edgeParticles: boolean;
  /** Enable node effects (hover, selected, orbit) */
  nodeEffects: boolean;
  /** Enable ambient glow effects */
  ambientGlow: boolean;
  /** Respect prefers-reduced-motion media query */
  respectReducedMotion: boolean;
}

interface AnimationStore {
  // ===== EDGE REGISTRATION =====
  /** Set of currently registered edge IDs */
  registeredEdges: Set<string>;
  /** Count of active animated edges */
  activeCount: number;

  /**
   * Register an edge for animation.
   * Will only register if under maxAnimatedEdges limit.
   */
  registerEdge: (id: string) => void;

  /**
   * Unregister an edge from animation.
   * Frees up a slot for another edge to animate.
   */
  unregisterEdge: (id: string) => void;

  /**
   * Check if an edge is currently allowed to animate.
   * Returns true only if the edge is registered.
   */
  canAnimate: (id: string) => boolean;

  /**
   * Reset the store to initial state.
   * Used for cleanup and testing.
   */
  reset: () => void;

  // ===== ANIMATION SETTINGS =====
  /** Animation settings */
  settings: AnimationSettings;

  /**
   * Set animation mode (Full / Reduced / Off)
   */
  setMode: (mode: AnimationMode) => void;

  /**
   * Cycle through animation modes: Full → Reduced → Off → Full
   * Keyboard shortcut: Shift+E
   */
  cycleMode: () => void;

  /**
   * Set global intensity (0-100)
   */
  setIntensity: (intensity: number) => void;

  /**
   * Toggle individual effect categories
   */
  toggleEdgeParticles: () => void;
  toggleNodeEffects: () => void;
  toggleAmbientGlow: () => void;
  toggleReducedMotion: () => void;

  /**
   * Get effective intensity (0-2 range, adjusted by mode)
   * Returns 0 if mode is 'off' or reduced-motion is active
   */
  getEffectiveIntensity: () => number;

  /**
   * Check if animations should be shown based on mode and settings
   */
  shouldAnimate: () => boolean;

  /**
   * Reset settings to defaults
   */
  resetSettings: () => void;

  // ===== MODE TRANSITION =====
  /** Whether a mode transition is in progress */
  isTransitioning: boolean;
  /** Current phase of the transition */
  transitionPhase: TransitionPhase;
  /** Target mode we're transitioning to */
  targetMode: DataMode | null;

  /**
   * Start a transition to the specified mode
   */
  startTransition: (mode: DataMode) => void;

  /**
   * Update the transition phase
   */
  setTransitionPhase: (phase: TransitionPhase) => void;

  /**
   * End the transition and reset state
   */
  endTransition: () => void;
}

const DEFAULT_SETTINGS: AnimationSettings = {
  mode: 'full',
  intensity: 70, // 70% default
  edgeParticles: true,
  nodeEffects: true,
  ambientGlow: true,
  respectReducedMotion: true,
};

/**
 * Helper to check prefers-reduced-motion
 * Only runs on client side
 */
const prefersReducedMotion = (): boolean => {
  if (typeof window === 'undefined') return false;
  return window.matchMedia('(prefers-reduced-motion: reduce)').matches;
};

export const useAnimationStore = create<AnimationStore>()(
  persist(
    (set, get) => ({
      // ===== EDGE REGISTRATION STATE =====
      registeredEdges: new Set<string>(),
      activeCount: 0,

      // ===== ANIMATION SETTINGS STATE =====
      settings: DEFAULT_SETTINGS,

      // ===== EDGE REGISTRATION ACTIONS =====
      registerEdge: (id: string) => {
        const { registeredEdges, activeCount, shouldAnimate } = get();

        // Don't register if animations are disabled
        if (!shouldAnimate()) return;

        // Already registered - no action needed
        if (registeredEdges.has(id)) return;

        // At capacity - cannot register more
        if (activeCount >= DEFAULT_ANIMATION_BUDGET.maxConcurrent) return;

        // Register the edge
        set(state => ({
          registeredEdges: new Set(state.registeredEdges).add(id),
          activeCount: state.activeCount + 1,
        }));
      },

      unregisterEdge: (id: string) => {
        const { registeredEdges } = get();

        // Not registered - no action needed
        if (!registeredEdges.has(id)) return;

        // Create new Set without the edge
        const newSet = new Set(registeredEdges);
        newSet.delete(id);

        set({
          registeredEdges: newSet,
          activeCount: newSet.size,
        });
      },

      canAnimate: (id: string) => {
        const { registeredEdges, shouldAnimate } = get();
        return shouldAnimate() && registeredEdges.has(id);
      },

      reset: () => {
        set({
          registeredEdges: new Set(),
          activeCount: 0,
        });
      },

      // ===== ANIMATION SETTINGS ACTIONS =====
      setMode: (mode: AnimationMode) => {
        set(state => ({
          settings: { ...state.settings, mode },
        }));
      },

      cycleMode: () => {
        const { settings } = get();
        const modes: AnimationMode[] = ['full', 'reduced', 'off'];
        const currentIndex = modes.indexOf(settings.mode);
        const nextIndex = (currentIndex + 1) % modes.length;
        set(state => ({
          settings: { ...state.settings, mode: modes[nextIndex] },
        }));
      },

      setIntensity: (intensity: number) => {
        const clamped = Math.max(0, Math.min(100, intensity));
        set(state => ({
          settings: { ...state.settings, intensity: clamped },
        }));
      },

      toggleEdgeParticles: () => {
        set(state => ({
          settings: { ...state.settings, edgeParticles: !state.settings.edgeParticles },
        }));
      },

      toggleNodeEffects: () => {
        set(state => ({
          settings: { ...state.settings, nodeEffects: !state.settings.nodeEffects },
        }));
      },

      toggleAmbientGlow: () => {
        set(state => ({
          settings: { ...state.settings, ambientGlow: !state.settings.ambientGlow },
        }));
      },

      toggleReducedMotion: () => {
        set(state => ({
          settings: { ...state.settings, respectReducedMotion: !state.settings.respectReducedMotion },
        }));
      },

      getEffectiveIntensity: () => {
        const { settings } = get();

        // Respect reduced-motion preference
        if (settings.respectReducedMotion && prefersReducedMotion()) {
          return 0;
        }

        // Mode-based intensity
        switch (settings.mode) {
          case 'off':
            return 0;
          case 'reduced':
            // Reduced mode caps at 50% of set intensity
            return (settings.intensity / 100) * 0.5;
          case 'full':
          default:
            // Map 0-100 to 0-2 range
            return (settings.intensity / 100) * 2;
        }
      },

      shouldAnimate: () => {
        const { settings } = get();

        // Respect reduced-motion preference
        if (settings.respectReducedMotion && prefersReducedMotion()) {
          return false;
        }

        return settings.mode !== 'off';
      },

      resetSettings: () => {
        set({ settings: DEFAULT_SETTINGS });
      },

      // ===== MODE TRANSITION STATE & ACTIONS =====
      isTransitioning: false,
      transitionPhase: null,
      targetMode: null,

      startTransition: (mode: DataMode) => {
        set({
          isTransitioning: true,
          transitionPhase: 'dissolve',
          targetMode: mode,
        });
      },

      setTransitionPhase: (phase: TransitionPhase) => {
        set({ transitionPhase: phase });
      },

      endTransition: () => {
        set({
          isTransitioning: false,
          transitionPhase: null,
          targetMode: null,
        });
      },
    }),
    {
      name: 'novanet-animation-settings',
      // Only persist settings, not the registered edges
      partialize: (state) => ({ settings: state.settings }),
      // Merge persisted state with initial state
      merge: (persisted, current) => ({
        ...current,
        settings: (persisted as { settings?: AnimationSettings })?.settings || DEFAULT_SETTINGS,
      }),
    }
  )
);

// ===== SELECTORS =====
export const selectIsTransitioning = (state: AnimationStore) => state.isTransitioning;
export const selectTransitionPhase = (state: AnimationStore) => state.transitionPhase;
export const selectTargetMode = (state: AnimationStore) => state.targetMode;
