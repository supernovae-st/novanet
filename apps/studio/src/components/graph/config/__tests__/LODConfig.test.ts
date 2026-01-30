/**
 * LODConfig Tests - Task 3.1 Performance Optimization
 *
 * Verifies Level of Detail (LOD) configuration for zoom-based performance scaling.
 * Part of Phase 3: Advanced Performance Optimizations for Graph Ultra-Optimization Plan.
 */
import {
  LOD_TIERS,
  getLODTier,
  type LODTier,
  type LODTierName,
} from '../LODConfig';

describe('LODConfig', () => {
  describe('getLODTier zoom thresholds', () => {
    it('should return ULTRA tier for zoom > 1.5', () => {
      expect(getLODTier(1.6)).toBe(LOD_TIERS.ULTRA);
      expect(getLODTier(2.0)).toBe(LOD_TIERS.ULTRA);
      expect(getLODTier(3.0)).toBe(LOD_TIERS.ULTRA);
    });

    it('should return HIGH tier for zoom 0.8-1.5', () => {
      expect(getLODTier(1.5)).toBe(LOD_TIERS.HIGH);
      expect(getLODTier(1.0)).toBe(LOD_TIERS.HIGH);
      expect(getLODTier(0.81)).toBe(LOD_TIERS.HIGH);
    });

    it('should return MEDIUM tier for zoom 0.4-0.8', () => {
      expect(getLODTier(0.8)).toBe(LOD_TIERS.MEDIUM);
      expect(getLODTier(0.6)).toBe(LOD_TIERS.MEDIUM);
      expect(getLODTier(0.41)).toBe(LOD_TIERS.MEDIUM);
    });

    it('should return LOW tier for zoom 0.2-0.4', () => {
      expect(getLODTier(0.4)).toBe(LOD_TIERS.LOW);
      expect(getLODTier(0.3)).toBe(LOD_TIERS.LOW);
      expect(getLODTier(0.21)).toBe(LOD_TIERS.LOW);
    });

    it('should return MINIMAL tier for zoom < 0.2', () => {
      expect(getLODTier(0.2)).toBe(LOD_TIERS.MINIMAL);
      expect(getLODTier(0.1)).toBe(LOD_TIERS.MINIMAL);
      expect(getLODTier(0.05)).toBe(LOD_TIERS.MINIMAL);
    });

    it('should handle edge case zoom = 0', () => {
      expect(getLODTier(0)).toBe(LOD_TIERS.MINIMAL);
    });

    it('should handle very high zoom values', () => {
      expect(getLODTier(10)).toBe(LOD_TIERS.ULTRA);
      expect(getLODTier(100)).toBe(LOD_TIERS.ULTRA);
    });
  });

  describe('MINIMAL tier configuration', () => {
    it('should have minimal effects but keep labels/particles for visibility', () => {
      const minimal = LOD_TIERS.MINIMAL;
      // Particles and labels always enabled for edge visibility
      expect(minimal.particles).toBe(true);
      expect(minimal.labels).toBe(true);
      expect(minimal.labelAbbreviation).toBe('icon');
      // Visual effects disabled for performance
      expect(minimal.energyEffects).toBe(false);
      expect(minimal.glowLayers).toBe(0);
      expect(minimal.blurQuality).toBe('none');
    });
  });

  describe('LOD_TIERS structure', () => {
    it('should have all 5 tiers defined', () => {
      const tierNames: LODTierName[] = ['ULTRA', 'HIGH', 'MEDIUM', 'LOW', 'MINIMAL'];
      tierNames.forEach(name => {
        expect(LOD_TIERS[name]).toBeDefined();
      });
    });

    it('should have ULTRA tier with maximum effects', () => {
      const ultra = LOD_TIERS.ULTRA;
      expect(ultra.particles).toBe(true);
      expect(ultra.energyEffects).toBe(true);
      expect(ultra.glowLayers).toBe(3);
      expect(ultra.labels).toBe(true);
      expect(ultra.blurQuality).toBe('high');
    });

    it('should have HIGH tier with good effects', () => {
      const high = LOD_TIERS.HIGH;
      expect(high.particles).toBe(true);
      expect(high.energyEffects).toBe(true);
      expect(high.glowLayers).toBe(2);
      expect(high.labels).toBe(true);
      expect(high.blurQuality).toBe('medium');
    });

    it('should have MEDIUM tier with simplified effects but labels/particles enabled', () => {
      const medium = LOD_TIERS.MEDIUM;
      expect(medium.particles).toBe(true);
      expect(medium.energyEffects).toBe('simplified');
      expect(medium.glowLayers).toBe(1);
      expect(medium.labels).toBe(true);
      expect(medium.labelAbbreviation).toBe('short');
      expect(medium.blurQuality).toBe('low');
    });

    it('should have LOW tier with minimal effects but labels/particles enabled', () => {
      const low = LOD_TIERS.LOW;
      expect(low.particles).toBe(true);
      expect(low.energyEffects).toBe('simplified');
      expect(low.glowLayers).toBe(0);
      expect(low.labels).toBe(true);
      expect(low.labelAbbreviation).toBe('initials');
      expect(low.blurQuality).toBe('none');
    });
  });

  describe('tier progression', () => {
    it('should progressively reduce glowLayers from ULTRA to MINIMAL', () => {
      expect(LOD_TIERS.ULTRA.glowLayers).toBeGreaterThan(LOD_TIERS.HIGH.glowLayers);
      expect(LOD_TIERS.HIGH.glowLayers).toBeGreaterThan(LOD_TIERS.MEDIUM.glowLayers);
      expect(LOD_TIERS.MEDIUM.glowLayers).toBeGreaterThan(LOD_TIERS.LOW.glowLayers);
      expect(LOD_TIERS.LOW.glowLayers).toBe(LOD_TIERS.MINIMAL.glowLayers);
    });

    it('should keep particles enabled at all tiers for edge visibility', () => {
      expect(LOD_TIERS.ULTRA.particles).toBe(true);
      expect(LOD_TIERS.HIGH.particles).toBe(true);
      expect(LOD_TIERS.MEDIUM.particles).toBe(true);
      expect(LOD_TIERS.LOW.particles).toBe(true);
      expect(LOD_TIERS.MINIMAL.particles).toBe(true);
    });

    it('should keep labels enabled at all tiers with progressive abbreviation', () => {
      expect(LOD_TIERS.ULTRA.labels).toBe(true);
      expect(LOD_TIERS.HIGH.labels).toBe(true);
      expect(LOD_TIERS.MEDIUM.labels).toBe(true);
      expect(LOD_TIERS.LOW.labels).toBe(true);
      expect(LOD_TIERS.MINIMAL.labels).toBe(true);
      // Abbreviation progresses from full to icon
      expect(LOD_TIERS.ULTRA.labelAbbreviation).toBe('full');
      expect(LOD_TIERS.HIGH.labelAbbreviation).toBe('full');
      expect(LOD_TIERS.MEDIUM.labelAbbreviation).toBe('short');
      expect(LOD_TIERS.LOW.labelAbbreviation).toBe('initials');
      expect(LOD_TIERS.MINIMAL.labelAbbreviation).toBe('icon');
    });
  });

  describe('type safety', () => {
    it('should return a valid LODTier object for any zoom value', () => {
      const testZooms = [0, 0.1, 0.3, 0.5, 1.0, 2.0, 5.0];
      testZooms.forEach(zoom => {
        const tier: LODTier = getLODTier(zoom);
        expect(typeof tier.particles).toBe('boolean');
        expect(typeof tier.glowLayers).toBe('number');
        expect(typeof tier.labels).toBe('boolean');
        expect(['high', 'medium', 'low', 'none']).toContain(tier.blurQuality);
        expect([true, false, 'simplified']).toContain(tier.energyEffects);
      });
    });
  });
});
