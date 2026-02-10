/**
 * NodeConfig Tests (v10.5.0)
 *
 * Tests for pre-computed node configuration lookup tables.
 * These tables provide O(1) access to node sizes and colors,
 * replacing runtime computation with simple property access.
 */

import {
  NODE_SIZES,
  NODE_COLORS,
  DEFAULT_NODE_SIZE,
  DEFAULT_NODE_COLORS,
  getNodeConfig,
  type NodeSize,
  type NodeColors,
} from '../NodeConfig';
import type { NodeType } from '@novanet/core/types';
import { NODE_TYPES } from '@novanet/core/types';

// All 62 node types in v11.2.0 (from Core - Single Source of Truth, 3 job nodes removed)
const ALL_NODE_TYPES: NodeType[] = [...NODE_TYPES];

describe('NodeConfig', () => {
  describe('NODE_SIZES lookup table', () => {
    it('should have predefined sizes for all 62 node types', () => {
      expect(Object.keys(NODE_SIZES)).toHaveLength(62);
      ALL_NODE_TYPES.forEach((type) => {
        expect(NODE_SIZES[type]).toBeDefined();
        expect(NODE_SIZES[type].width).toBeGreaterThan(0);
        expect(NODE_SIZES[type].height).toBeGreaterThan(0);
      });
    });

    it('should have width and height as numbers', () => {
      ALL_NODE_TYPES.forEach((type) => {
        const size = NODE_SIZES[type];
        expect(typeof size.width).toBe('number');
        expect(typeof size.height).toBe('number');
      });
    });

    it('should have larger sizes for important nodes (Project, Page, Entity)', () => {
      expect(NODE_SIZES.Project.width).toBeGreaterThanOrEqual(240);
      expect(NODE_SIZES.Page.width).toBeGreaterThanOrEqual(200);
      expect(NODE_SIZES.Entity.width).toBeGreaterThanOrEqual(200);
    });

    it('should have smaller sizes for auxiliary nodes (ExpressionSet, SEOKeywordMetrics)', () => {
      expect(NODE_SIZES.ExpressionSet.width).toBeLessThanOrEqual(160);
      expect(NODE_SIZES.SEOKeywordMetrics.width).toBeLessThanOrEqual(180);
    });
  });

  describe('NODE_COLORS lookup table', () => {
    it('should have predefined colors for all 62 node types', () => {
      expect(Object.keys(NODE_COLORS)).toHaveLength(62);
      ALL_NODE_TYPES.forEach((type) => {
        expect(NODE_COLORS[type]).toBeDefined();
        expect(NODE_COLORS[type].primary).toBeDefined();
        expect(NODE_COLORS[type].secondary).toBeDefined();
        expect(NODE_COLORS[type].tertiary).toBeDefined();
        expect(NODE_COLORS[type].glow).toBeDefined();
      });
    });

    it('should have valid hex color format for all colors', () => {
      const hexColorRegex = /^#[0-9A-Fa-f]{6}([0-9A-Fa-f]{2})?$/;

      ALL_NODE_TYPES.forEach((type) => {
        const colors = NODE_COLORS[type];
        expect(colors.primary).toMatch(hexColorRegex);
        expect(colors.secondary).toMatch(hexColorRegex);
        expect(colors.tertiary).toMatch(hexColorRegex);
        expect(colors.glow).toMatch(hexColorRegex);
      });
    });

    it('should have distinct colors for different categories', () => {
      // Project category (violet) vs Content category (amber)
      expect(NODE_COLORS.Project.primary).not.toBe(NODE_COLORS.Entity.primary);
      // Locale category (green) vs SEO category (red)
      expect(NODE_COLORS.Locale.primary).not.toBe(NODE_COLORS.SEOKeyword.primary);
    });
  });

  describe('DEFAULT_NODE_SIZE', () => {
    it('should have valid default size', () => {
      expect(DEFAULT_NODE_SIZE.width).toBeGreaterThan(0);
      expect(DEFAULT_NODE_SIZE.height).toBeGreaterThan(0);
    });
  });

  describe('DEFAULT_NODE_COLORS', () => {
    it('should have all required color properties', () => {
      expect(DEFAULT_NODE_COLORS.primary).toBeDefined();
      expect(DEFAULT_NODE_COLORS.secondary).toBeDefined();
      expect(DEFAULT_NODE_COLORS.tertiary).toBeDefined();
      expect(DEFAULT_NODE_COLORS.glow).toBeDefined();
    });
  });

  describe('getNodeConfig', () => {
    it('should return combined size and colors for known types', () => {
      ALL_NODE_TYPES.forEach((type) => {
        const config = getNodeConfig(type);
        expect(config.size).toEqual(NODE_SIZES[type]);
        expect(config.colors).toEqual(NODE_COLORS[type]);
      });
    });

    it('should return default config for unknown types', () => {
      const config = getNodeConfig('UnknownType' as NodeType);
      expect(config.size).toEqual(DEFAULT_NODE_SIZE);
      expect(config.colors).toEqual(DEFAULT_NODE_COLORS);
    });

    it('should return the same reference for repeated calls (memoization)', () => {
      const config1 = getNodeConfig('Project');
      const config2 = getNodeConfig('Project');
      expect(config1).toBe(config2);
    });

    it('should have O(1) lookup performance', () => {
      // This is a structural test - lookup tables are O(1) by design
      // The fact that NODE_SIZES and NODE_COLORS are plain objects
      // guarantees O(1) property access
      const start = performance.now();
      for (let i = 0; i < 10000; i++) {
        getNodeConfig(ALL_NODE_TYPES[i % ALL_NODE_TYPES.length]);
      }
      const duration = performance.now() - start;
      // Should complete 10k lookups in under 50ms (very conservative)
      expect(duration).toBeLessThan(50);
    });
  });

  describe('Type exports', () => {
    it('should export NodeSize type with correct shape', () => {
      const size: NodeSize = { width: 200, height: 100 };
      expect(size.width).toBe(200);
      expect(size.height).toBe(100);
    });

    it('should export NodeColors type with correct shape', () => {
      const colors: NodeColors = {
        primary: '#8b5cf6',
        secondary: '#6366f1',
        tertiary: '#a78bfa',
        glow: '#8b5cf640',
      };
      expect(colors.primary).toBe('#8b5cf6');
      expect(colors.glow).toBe('#8b5cf640');
    });
  });
});
