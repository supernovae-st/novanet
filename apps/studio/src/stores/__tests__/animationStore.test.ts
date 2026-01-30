/**
 * Animation Store Tests
 *
 * Tests for the animation registry store that enforces
 * maxConcurrent limit for performance optimization.
 */

import { useAnimationStore } from '../animationStore';
import { DEFAULT_ANIMATION_BUDGET } from '@/components/graph/edges/system/constants';

const MAX_CONCURRENT = DEFAULT_ANIMATION_BUDGET.maxConcurrent; // 50

// Mock window.matchMedia for prefers-reduced-motion check
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: jest.fn().mockImplementation((query: string) => ({
    matches: false, // Default: no reduced motion preference
    media: query,
    onchange: null,
    addListener: jest.fn(),
    removeListener: jest.fn(),
    addEventListener: jest.fn(),
    removeEventListener: jest.fn(),
    dispatchEvent: jest.fn(),
  })),
});

describe('animationStore', () => {
  beforeEach(() => {
    useAnimationStore.getState().reset();
    useAnimationStore.getState().resetSettings();
  });

  it('should register edges up to maxConcurrent limit', () => {
    // Register more edges than the limit allows
    for (let i = 0; i < MAX_CONCURRENT + 50; i++) {
      useAnimationStore.getState().registerEdge(`edge-${i}`);
    }

    const state = useAnimationStore.getState();
    expect(state.activeCount).toBe(MAX_CONCURRENT);
    expect(state.canAnimate('edge-10')).toBe(true);
    expect(state.canAnimate(`edge-${MAX_CONCURRENT + 10}`)).toBe(false);
  });

  it('should allow animation after unregister', () => {
    // Fill up to limit
    for (let i = 0; i < MAX_CONCURRENT; i++) {
      useAnimationStore.getState().registerEdge(`edge-${i}`);
    }

    expect(useAnimationStore.getState().canAnimate('edge-new')).toBe(false);

    useAnimationStore.getState().unregisterEdge('edge-10');
    useAnimationStore.getState().registerEdge('edge-new');

    expect(useAnimationStore.getState().canAnimate('edge-new')).toBe(true);
  });

  it('should not register the same edge twice', () => {
    useAnimationStore.getState().registerEdge('edge-1');
    useAnimationStore.getState().registerEdge('edge-1');
    useAnimationStore.getState().registerEdge('edge-1');

    expect(useAnimationStore.getState().activeCount).toBe(1);
  });

  it('should handle unregister of non-existent edge gracefully', () => {
    useAnimationStore.getState().registerEdge('edge-1');
    useAnimationStore.getState().unregisterEdge('edge-nonexistent');

    expect(useAnimationStore.getState().activeCount).toBe(1);
  });

  it('should reset all state correctly', () => {
    for (let i = 0; i < 50; i++) {
      useAnimationStore.getState().registerEdge(`edge-${i}`);
    }

    useAnimationStore.getState().reset();

    const state = useAnimationStore.getState();
    expect(state.activeCount).toBe(0);
    expect(state.canAnimate('edge-0')).toBe(false);
  });

  it('should update activeCount correctly on unregister', () => {
    useAnimationStore.getState().registerEdge('edge-1');
    useAnimationStore.getState().registerEdge('edge-2');
    useAnimationStore.getState().registerEdge('edge-3');

    expect(useAnimationStore.getState().activeCount).toBe(3);

    useAnimationStore.getState().unregisterEdge('edge-2');

    expect(useAnimationStore.getState().activeCount).toBe(2);
  });

  it('should track registered edges correctly', () => {
    useAnimationStore.getState().registerEdge('edge-1');
    useAnimationStore.getState().registerEdge('edge-2');

    const state = useAnimationStore.getState();
    expect(state.registeredEdges.has('edge-1')).toBe(true);
    expect(state.registeredEdges.has('edge-2')).toBe(true);
    expect(state.registeredEdges.has('edge-3')).toBe(false);
  });
});
