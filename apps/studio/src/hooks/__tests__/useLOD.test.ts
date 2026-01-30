// src/hooks/__tests__/useLOD.test.ts
import { renderHook } from '@testing-library/react';
import { useLOD } from '../useLOD';
import { LOD_TIERS } from '@/components/graph/config/LODConfig';

// Mock React Flow's useStore hook
const mockZoom = { current: 1.0 };

jest.mock('@xyflow/react', () => ({
  useStore: jest.fn((selector) => {
    // Simulate the state shape from React Flow
    const state = { transform: [0, 0, mockZoom.current] };
    return selector(state);
  }),
}));

describe('useLOD', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    mockZoom.current = 1.0;
  });

  describe('LOD tier selection based on zoom', () => {
    it('returns ULTRA tier when zoom > 1.5', () => {
      mockZoom.current = 2.0;
      const { result } = renderHook(() => useLOD());

      expect(result.current).toEqual(LOD_TIERS.ULTRA);
      expect(result.current.particles).toBe(true);
      expect(result.current.glowLayers).toBe(3);
      expect(result.current.blurQuality).toBe('high');
    });

    it('returns ULTRA tier at zoom boundary 1.51', () => {
      mockZoom.current = 1.51;
      const { result } = renderHook(() => useLOD());

      expect(result.current).toEqual(LOD_TIERS.ULTRA);
    });

    it('returns HIGH tier when zoom is 0.8-1.5', () => {
      mockZoom.current = 1.2;
      const { result } = renderHook(() => useLOD());

      expect(result.current).toEqual(LOD_TIERS.HIGH);
      expect(result.current.particles).toBe(true);
      expect(result.current.glowLayers).toBe(2);
      expect(result.current.blurQuality).toBe('medium');
    });

    it('returns HIGH tier at zoom boundary 0.81', () => {
      mockZoom.current = 0.81;
      const { result } = renderHook(() => useLOD());

      expect(result.current).toEqual(LOD_TIERS.HIGH);
    });

    it('returns HIGH tier at zoom boundary 1.5', () => {
      mockZoom.current = 1.5;
      const { result } = renderHook(() => useLOD());

      expect(result.current).toEqual(LOD_TIERS.HIGH);
    });

    it('returns MEDIUM tier when zoom is 0.4-0.8', () => {
      mockZoom.current = 0.6;
      const { result } = renderHook(() => useLOD());

      expect(result.current).toEqual(LOD_TIERS.MEDIUM);
      expect(result.current.particles).toBe(true);
      expect(result.current.labels).toBe(true);
      expect(result.current.labelAbbreviation).toBe('short');
      expect(result.current.energyEffects).toBe('simplified');
      expect(result.current.glowLayers).toBe(1);
    });

    it('returns MEDIUM tier at zoom boundary 0.41', () => {
      mockZoom.current = 0.41;
      const { result } = renderHook(() => useLOD());

      expect(result.current).toEqual(LOD_TIERS.MEDIUM);
    });

    it('returns MEDIUM tier at zoom boundary 0.8', () => {
      mockZoom.current = 0.8;
      const { result } = renderHook(() => useLOD());

      expect(result.current).toEqual(LOD_TIERS.MEDIUM);
    });

    it('returns LOW tier when zoom is 0.2-0.4', () => {
      mockZoom.current = 0.3;
      const { result } = renderHook(() => useLOD());

      expect(result.current).toEqual(LOD_TIERS.LOW);
      expect(result.current.particles).toBe(true);
      expect(result.current.labels).toBe(true);
      expect(result.current.labelAbbreviation).toBe('initials');
      expect(result.current.energyEffects).toBe('simplified');
      expect(result.current.glowLayers).toBe(0);
      expect(result.current.blurQuality).toBe('none');
    });

    it('returns LOW tier at zoom boundary 0.21', () => {
      mockZoom.current = 0.21;
      const { result } = renderHook(() => useLOD());

      expect(result.current).toEqual(LOD_TIERS.LOW);
    });

    it('returns LOW tier at zoom boundary 0.4', () => {
      mockZoom.current = 0.4;
      const { result } = renderHook(() => useLOD());

      expect(result.current).toEqual(LOD_TIERS.LOW);
    });

    it('returns MINIMAL tier when zoom <= 0.2', () => {
      mockZoom.current = 0.1;
      const { result } = renderHook(() => useLOD());

      expect(result.current).toEqual(LOD_TIERS.MINIMAL);
      expect(result.current.particles).toBe(true);
      expect(result.current.labels).toBe(true);
      expect(result.current.labelAbbreviation).toBe('icon');
      expect(result.current.energyEffects).toBe(false);
      expect(result.current.glowLayers).toBe(0);
    });

    it('returns MINIMAL tier at zoom boundary 0.2', () => {
      mockZoom.current = 0.2;
      const { result } = renderHook(() => useLOD());

      expect(result.current).toEqual(LOD_TIERS.MINIMAL);
    });

    it('returns MINIMAL tier at very low zoom (0.05)', () => {
      mockZoom.current = 0.05;
      const { result } = renderHook(() => useLOD());

      expect(result.current).toEqual(LOD_TIERS.MINIMAL);
    });
  });

  describe('tier property verification', () => {
    it('has particles enabled at all tiers for edge visibility', () => {
      // All tiers have particles enabled for edge visibility
      mockZoom.current = 2.0;
      expect(renderHook(() => useLOD()).result.current.particles).toBe(true);

      mockZoom.current = 1.0;
      expect(renderHook(() => useLOD()).result.current.particles).toBe(true);

      mockZoom.current = 0.5;
      expect(renderHook(() => useLOD()).result.current.particles).toBe(true);

      mockZoom.current = 0.3;
      expect(renderHook(() => useLOD()).result.current.particles).toBe(true);

      mockZoom.current = 0.1;
      expect(renderHook(() => useLOD()).result.current.particles).toBe(true);
    });

    it('has labels enabled at all tiers with progressive abbreviation', () => {
      // All tiers have labels enabled with progressive abbreviation
      mockZoom.current = 2.0;
      expect(renderHook(() => useLOD()).result.current.labels).toBe(true);
      expect(renderHook(() => useLOD()).result.current.labelAbbreviation).toBe('full');

      mockZoom.current = 1.0;
      expect(renderHook(() => useLOD()).result.current.labels).toBe(true);
      expect(renderHook(() => useLOD()).result.current.labelAbbreviation).toBe('full');

      mockZoom.current = 0.5;
      expect(renderHook(() => useLOD()).result.current.labels).toBe(true);
      expect(renderHook(() => useLOD()).result.current.labelAbbreviation).toBe('short');

      mockZoom.current = 0.3;
      expect(renderHook(() => useLOD()).result.current.labels).toBe(true);
      expect(renderHook(() => useLOD()).result.current.labelAbbreviation).toBe('initials');

      mockZoom.current = 0.1;
      expect(renderHook(() => useLOD()).result.current.labels).toBe(true);
      expect(renderHook(() => useLOD()).result.current.labelAbbreviation).toBe('icon');
    });

    it('progressively reduces glowLayers as zoom decreases', () => {
      mockZoom.current = 2.0;
      expect(renderHook(() => useLOD()).result.current.glowLayers).toBe(3);

      mockZoom.current = 1.0;
      expect(renderHook(() => useLOD()).result.current.glowLayers).toBe(2);

      mockZoom.current = 0.5;
      expect(renderHook(() => useLOD()).result.current.glowLayers).toBe(1);

      mockZoom.current = 0.3;
      expect(renderHook(() => useLOD()).result.current.glowLayers).toBe(0);

      mockZoom.current = 0.1;
      expect(renderHook(() => useLOD()).result.current.glowLayers).toBe(0);
    });
  });
});
