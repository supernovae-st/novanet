/**
 * LOD Controller Tests
 *
 * Tests for the Level of Detail system that reduces
 * visual complexity based on distance and zoom.
 */

import {
  calculateLODTier,
  filterEffectsForLOD,
  getLODConfig,
  getLODIntensity,
  shouldAnimate,
  LODManager,
} from '../system/performance/LODController';
import type { EffectPrimitive } from '../system/types';

describe('calculateLODTier', () => {
  it('should return high for selected edges regardless of distance', () => {
    expect(calculateLODTier(1000, 1, true, false, false)).toBe('high');
    expect(calculateLODTier(5000, 0.1, true, false, false)).toBe('high');
  });

  it('should return high for hovered edges regardless of distance', () => {
    expect(calculateLODTier(1000, 1, false, true, false)).toBe('high');
    expect(calculateLODTier(5000, 0.1, false, true, false)).toBe('high');
  });

  it('should return medium for edges connected to selected node', () => {
    expect(calculateLODTier(1000, 1, false, false, true)).toBe('medium');
  });

  it('should return appropriate tier based on distance at zoom 1', () => {
    // Close - high
    expect(calculateLODTier(100, 1, false, false, false)).toBe('high');
    // Medium distance - medium
    expect(calculateLODTier(300, 1, false, false, false)).toBe('medium');
    // Far - low
    expect(calculateLODTier(700, 1, false, false, false)).toBe('low');
    // Very far - minimal
    expect(calculateLODTier(1500, 1, false, false, false)).toBe('minimal');
  });

  it('should adjust effective distance based on zoom', () => {
    // At zoom 2, distance 200 becomes effective distance 100 -> high
    expect(calculateLODTier(200, 2, false, false, false)).toBe('high');
    // At zoom 0.5, distance 100 becomes effective distance 200 -> still high (at boundary)
    // At zoom 0.25, distance 100 becomes effective distance 400 -> medium
    expect(calculateLODTier(100, 0.25, false, false, false)).toBe('medium');
  });
});

describe('filterEffectsForLOD', () => {
  const allEffects: EffectPrimitive[] = ['emit', 'particles', 'trail', 'impact', 'glow', 'zigzag'];

  it('should return all effects for high tier', () => {
    const filtered = filterEffectsForLOD(allEffects, 'high');
    expect(filtered).toEqual(allEffects);
  });

  it('should return core effects (particles, glow) for medium tier', () => {
    const filtered = filterEffectsForLOD(allEffects, 'medium');
    expect(filtered).toContain('particles');
    expect(filtered).toContain('glow');
    expect(filtered).not.toContain('emit');
    expect(filtered).not.toContain('trail');
  });

  it('should return only glow for low tier', () => {
    const filtered = filterEffectsForLOD(allEffects, 'low');
    expect(filtered).toEqual(['glow']);
  });

  it('should return empty array for minimal tier', () => {
    const filtered = filterEffectsForLOD(allEffects, 'minimal');
    expect(filtered).toEqual([]);
  });
});

describe('getLODConfig', () => {
  it('should return config for each tier', () => {
    expect(getLODConfig('high').enableGlow).toBe(true);
    expect(getLODConfig('high').maxParticles).toBe(6);

    expect(getLODConfig('medium').maxParticles).toBe(3);
    expect(getLODConfig('low').maxParticles).toBe(1);
    expect(getLODConfig('minimal').maxParticles).toBe(0);
  });
});

describe('getLODIntensity', () => {
  it('should return decreasing intensity for lower tiers', () => {
    expect(getLODIntensity('high')).toBe(1);
    expect(getLODIntensity('medium')).toBe(0.7);
    expect(getLODIntensity('low')).toBe(0.4);
    expect(getLODIntensity('minimal')).toBe(0.2);
  });
});

describe('shouldAnimate', () => {
  it('should return true for all tiers except minimal', () => {
    expect(shouldAnimate('high')).toBe(true);
    expect(shouldAnimate('medium')).toBe(true);
    expect(shouldAnimate('low')).toBe(true);
    expect(shouldAnimate('minimal')).toBe(false);
  });
});

describe('LODManager', () => {
  let manager: LODManager;

  beforeEach(() => {
    manager = new LODManager();
    manager.updateViewport({ x: 0, y: 0 }, 1);
  });

  describe('updateEdgeLOD', () => {
    it('should calculate and cache tier for edge', () => {
      const tier = manager.updateEdgeLOD(
        'edge-1',
        { x: 100, y: 0 },
        false,
        false,
        false
      );
      expect(tier).toBe('high');
      expect(manager.getTier('edge-1')).toBe('high');
    });

    it('should return high for selected edges', () => {
      const tier = manager.updateEdgeLOD(
        'edge-1',
        { x: 1000, y: 1000 },
        true,
        false,
        false
      );
      expect(tier).toBe('high');
    });
  });

  describe('getTier', () => {
    it('should return cached tier', () => {
      manager.updateEdgeLOD('edge-1', { x: 100, y: 0 }, false, false, false);
      expect(manager.getTier('edge-1')).toBe('high');
    });

    it('should return high for unknown edges', () => {
      expect(manager.getTier('unknown-edge')).toBe('high');
    });
  });

  describe('removeEdge', () => {
    it('should remove edge from tracking', () => {
      manager.updateEdgeLOD('edge-1', { x: 100, y: 0 }, false, false, false);
      manager.removeEdge('edge-1');

      // Should return default (high) after removal
      expect(manager.getTier('edge-1')).toBe('high');
    });
  });

  describe('getStats', () => {
    it('should return tier distribution', () => {
      manager.updateEdgeLOD('edge-1', { x: 100, y: 0 }, false, false, false);
      manager.updateEdgeLOD('edge-2', { x: 300, y: 0 }, false, false, false);
      manager.updateEdgeLOD('edge-3', { x: 1500, y: 0 }, false, false, false);

      const stats = manager.getStats();
      expect(stats.high).toBeGreaterThanOrEqual(1);
    });
  });

  describe('clear', () => {
    it('should remove all tracked edges', () => {
      manager.updateEdgeLOD('edge-1', { x: 100, y: 0 }, false, false, false);
      manager.updateEdgeLOD('edge-2', { x: 200, y: 0 }, false, false, false);

      manager.clear();

      const stats = manager.getStats();
      expect(stats.high + stats.medium + stats.low + stats.minimal).toBe(0);
    });
  });
});
