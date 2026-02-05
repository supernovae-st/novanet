// src/lib/__tests__/forceSimulation.test.ts
/**
 * Force Simulation Tests - TDD for Nova Layout Improvements
 *
 * Tests for:
 * 1. Spacious layout (+100% spacing)
 * 2. Spacing presets (compact, normal, spacious)
 * 3. Minimum distances between nodes
 * 4. Category clustering
 */

// Mock d3-force before imports
jest.mock('d3-force', () => {
  const mockSimulation = {
    nodes: jest.fn().mockReturnThis(),
    force: jest.fn().mockReturnThis(),
    alphaDecay: jest.fn().mockReturnThis(),
    velocityDecay: jest.fn().mockReturnThis(),
    alphaMin: jest.fn().mockReturnThis(),
    alpha: jest.fn().mockReturnValue(0.001),
    tick: jest.fn(),
    stop: jest.fn(),
  };

  return {
    forceSimulation: jest.fn(() => mockSimulation),
    forceLink: jest.fn(() => ({
      id: jest.fn().mockReturnThis(),
      distance: jest.fn().mockReturnThis(),
      strength: jest.fn().mockReturnThis(),
    })),
    forceManyBody: jest.fn(() => ({
      strength: jest.fn().mockReturnThis(),
      distanceMin: jest.fn().mockReturnThis(),
      distanceMax: jest.fn().mockReturnThis(),
      theta: jest.fn().mockReturnThis(),
    })),
    forceCenter: jest.fn(() => ({
      strength: jest.fn().mockReturnThis(),
    })),
    forceCollide: jest.fn(() => ({
      radius: jest.fn().mockReturnThis(),
      strength: jest.fn().mockReturnThis(),
      iterations: jest.fn().mockReturnThis(),
    })),
    forceX: jest.fn(() => ({
      x: jest.fn().mockReturnThis(),
      strength: jest.fn().mockReturnThis(),
    })),
    forceY: jest.fn(() => ({
      y: jest.fn().mockReturnThis(),
      strength: jest.fn().mockReturnThis(),
    })),
  };
});

import type { Node } from '@xyflow/react';
// Import only the pure functions we can test without d3
import {
  getCategoryCenter,
  applyForcePositions,
  SPACING_PRESETS,
  DEFAULT_SPACING_PRESET,
  type SpacingPreset,
} from '../forceSimulation';

// Mock getNodeDimensions
jest.mock('../layout', () => ({
  getNodeDimensions: (type: string) => {
    const dims: Record<string, { width: number; height: number }> = {
      Project: { width: 280, height: 140 },
      Entity: { width: 240, height: 120 },
      Locale: { width: 220, height: 110 },
      default: { width: 200, height: 100 },
    };
    return dims[type] || dims.default;
  },
}));

describe('forceSimulation', () => {
  describe('spacing parameters', () => {
    describe('SPACIOUS preset (+100% spacing)', () => {
      it('should have charge strength 2x stronger than normal', () => {
        expect(SPACING_PRESETS.spacious.chargeStrength).toBe(-2400);
        expect(SPACING_PRESETS.spacious.chargeStrength).toBe(SPACING_PRESETS.normal.chargeStrength * 2);
      });

      it('should have link distance 2x larger than normal', () => {
        expect(SPACING_PRESETS.spacious.linkDistance).toBe(640);
        expect(SPACING_PRESETS.spacious.linkDistance).toBe(SPACING_PRESETS.normal.linkDistance * 2);
      });

      it('should have collision radius significantly larger than normal', () => {
        expect(SPACING_PRESETS.spacious.collisionRadius).toBeGreaterThanOrEqual(3.5);
        expect(SPACING_PRESETS.spacious.collisionRadius).toBeGreaterThan(SPACING_PRESETS.normal.collisionRadius * 1.5);
      });

      it('should be the default spacing preset', () => {
        expect(DEFAULT_SPACING_PRESET).toBe('spacious');
      });
    });
  });

  describe('category clustering', () => {
    it('should place project nodes in left region', () => {
      const center = getCategoryCenter('project', 1);
      expect(center.x).toBeLessThan(0);
    });

    it('should place locale nodes in bottom region', () => {
      const center = getCategoryCenter('locale', 1);
      expect(center.y).toBeGreaterThan(0);
    });

    it('should spread category centers by scale factor', () => {
      const scale1 = getCategoryCenter('project', 1);
      const scale2 = getCategoryCenter('project', 2);

      expect(Math.abs(scale2.x)).toBe(Math.abs(scale1.x) * 2);
    });
  });

  describe('applyForcePositions', () => {
    it('should update node positions from simulation results', () => {
      const nodes: Node[] = [
        { id: 'a', position: { x: 0, y: 0 }, data: {} },
        { id: 'b', position: { x: 0, y: 0 }, data: {} },
      ];

      const positions = new Map([
        ['a', { x: 100, y: 200 }],
        ['b', { x: 300, y: 400 }],
      ]);

      const result = applyForcePositions(nodes, positions);

      expect(result[0].position).toEqual({ x: 100, y: 200 });
      expect(result[1].position).toEqual({ x: 300, y: 400 });
    });

    it('should preserve nodes not in positions map', () => {
      const nodes: Node[] = [
        { id: 'a', position: { x: 50, y: 50 }, data: {} },
      ];

      const positions = new Map<string, { x: number; y: number }>();

      const result = applyForcePositions(nodes, positions);

      expect(result[0].position).toEqual({ x: 50, y: 50 });
    });
  });
});

describe('spacing presets', () => {
  // Tests for the exported SPACING_PRESETS from forceSimulation.ts

  it('should have compact preset with reduced spacing', () => {
    expect(SPACING_PRESETS.compact.linkDistance).toBeLessThan(SPACING_PRESETS.normal.linkDistance);
    expect(Math.abs(SPACING_PRESETS.compact.chargeStrength)).toBeLessThan(
      Math.abs(SPACING_PRESETS.normal.chargeStrength)
    );
  });

  it('should have spacious preset with 2x normal spacing', () => {
    expect(SPACING_PRESETS.spacious.linkDistance).toBe(SPACING_PRESETS.normal.linkDistance * 2);
    expect(SPACING_PRESETS.spacious.chargeStrength).toBe(SPACING_PRESETS.normal.chargeStrength * 2);
  });

  it('should have collision radius scale proportionally', () => {
    expect(SPACING_PRESETS.spacious.collisionRadius).toBeGreaterThan(
      SPACING_PRESETS.normal.collisionRadius * 1.5
    );
  });

  it('should have all three presets defined', () => {
    const presets: SpacingPreset[] = ['compact', 'normal', 'spacious'];
    presets.forEach((preset) => {
      expect(SPACING_PRESETS[preset]).toBeDefined();
      expect(SPACING_PRESETS[preset].chargeStrength).toBeLessThan(0);
      expect(SPACING_PRESETS[preset].linkDistance).toBeGreaterThan(0);
      expect(SPACING_PRESETS[preset].collisionRadius).toBeGreaterThan(0);
    });
  });
});
