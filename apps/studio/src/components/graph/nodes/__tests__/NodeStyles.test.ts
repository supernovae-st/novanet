/**
 * NodeStyles Factory Tests
 *
 * Verifies memoization behavior - same inputs should return
 * the exact same object reference to prevent unnecessary re-renders
 */

import {
  getNodeContainerStyle,
  getNodeHeaderStyle,
  getNodeContentStyle,
  clearStyleCaches,
} from '../NodeStyles';

describe('NodeStyles factory', () => {
  // Clear caches before each test for isolation
  beforeEach(() => {
    clearStyleCaches();
  });

  describe('getNodeContainerStyle', () => {
    it('should return same object for same inputs (memoized)', () => {
      const style1 = getNodeContainerStyle(200, 100, false, false);
      const style2 = getNodeContainerStyle(200, 100, false, false);

      // Same reference, not just equal values
      expect(style1).toBe(style2);
    });

    it('should return different objects for different inputs', () => {
      const style1 = getNodeContainerStyle(200, 100, false, false);
      const style2 = getNodeContainerStyle(200, 100, true, false);

      expect(style1).not.toBe(style2);
    });

    it('should apply dimmed opacity of 0.3', () => {
      const style = getNodeContainerStyle(200, 100, true, false);
      expect(style.opacity).toBe(0.3);
    });

    it('should apply normal opacity of 1 when not dimmed', () => {
      const style = getNodeContainerStyle(200, 100, false, false);
      expect(style.opacity).toBe(1);
    });

    it('should apply scale transform when selected', () => {
      const style = getNodeContainerStyle(200, 100, false, true);
      expect(style.transform).toBe('scale(1.02)');
    });

    it('should not apply transform when not selected', () => {
      const style = getNodeContainerStyle(200, 100, false, false);
      expect(style.transform).toBeUndefined();
    });

    it('should include correct width and height', () => {
      const style = getNodeContainerStyle(280, 140, false, false);
      expect(style.width).toBe(280);
      expect(style.height).toBe(140);
    });

    it('should include transition property', () => {
      const style = getNodeContainerStyle(200, 100, false, false);
      expect(style.transition).toBe('opacity 0.2s ease, transform 0.2s ease');
    });
  });

  describe('getNodeHeaderStyle', () => {
    it('should return same object for same inputs (memoized)', () => {
      const style1 = getNodeHeaderStyle('#5E6AD2', false);
      const style2 = getNodeHeaderStyle('#5E6AD2', false);

      expect(style1).toBe(style2);
    });

    it('should return different objects for different colors', () => {
      const style1 = getNodeHeaderStyle('#5E6AD2', false);
      const style2 = getNodeHeaderStyle('#10b981', false);

      expect(style1).not.toBe(style2);
    });

    it('should return different objects for different hover states', () => {
      const style1 = getNodeHeaderStyle('#5E6AD2', false);
      const style2 = getNodeHeaderStyle('#5E6AD2', true);

      expect(style1).not.toBe(style2);
    });

    it('should apply full opacity when hovered', () => {
      const style = getNodeHeaderStyle('#5E6AD2', true);
      expect(style.opacity).toBe(1);
    });

    it('should apply 0.9 opacity when not hovered', () => {
      const style = getNodeHeaderStyle('#5E6AD2', false);
      expect(style.opacity).toBe(0.9);
    });

    it('should set backgroundColor to primaryColor', () => {
      const style = getNodeHeaderStyle('#5E6AD2', false);
      expect(style.backgroundColor).toBe('#5E6AD2');
    });
  });

  describe('getNodeContentStyle', () => {
    it('should return same object for same inputs (memoized)', () => {
      const style1 = getNodeContentStyle(false);
      const style2 = getNodeContentStyle(false);

      expect(style1).toBe(style2);
    });

    it('should return different objects for different dimmed states', () => {
      const style1 = getNodeContentStyle(false);
      const style2 = getNodeContentStyle(true);

      expect(style1).not.toBe(style2);
    });

    it('should apply 0.5 opacity when dimmed', () => {
      const style = getNodeContentStyle(true);
      expect(style.opacity).toBe(0.5);
    });

    it('should apply full opacity when not dimmed', () => {
      const style = getNodeContentStyle(false);
      expect(style.opacity).toBe(1);
    });
  });

  describe('clearStyleCaches', () => {
    it('should clear all caches', () => {
      // Populate caches
      const containerStyle1 = getNodeContainerStyle(200, 100, false, false);
      const headerStyle1 = getNodeHeaderStyle('#5E6AD2', false);
      const contentStyle1 = getNodeContentStyle(false);

      // Clear caches
      clearStyleCaches();

      // Get new styles - should be new objects
      const containerStyle2 = getNodeContainerStyle(200, 100, false, false);
      const headerStyle2 = getNodeHeaderStyle('#5E6AD2', false);
      const contentStyle2 = getNodeContentStyle(false);

      // After clearing, new objects should be created (different references)
      // But with same values
      expect(containerStyle2).not.toBe(containerStyle1);
      expect(containerStyle2).toEqual(containerStyle1);

      expect(headerStyle2).not.toBe(headerStyle1);
      expect(headerStyle2).toEqual(headerStyle1);

      expect(contentStyle2).not.toBe(contentStyle1);
      expect(contentStyle2).toEqual(contentStyle1);
    });
  });

  describe('cache key uniqueness', () => {
    it('should differentiate between similar numeric combinations', () => {
      // These could collide with naive string concatenation
      const style1 = getNodeContainerStyle(20, 100, false, false);
      const style2 = getNodeContainerStyle(201, 0, false, false);
      const style3 = getNodeContainerStyle(2, 10, false, false);

      expect(style1).not.toBe(style2);
      expect(style1).not.toBe(style3);
      expect(style2).not.toBe(style3);
    });
  });
});
