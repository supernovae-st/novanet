/**
 * Animation Budget Tests
 *
 * Tests for the animation budget manager that controls
 * concurrent animation limits with priority-based eviction.
 */

import { AnimationBudgetManager } from '../system/performance/AnimationBudget';
import { EDGE_PRIORITIES } from '../system/constants';

describe('AnimationBudgetManager', () => {
  let budget: AnimationBudgetManager;

  beforeEach(() => {
    budget = new AnimationBudgetManager({
      maxConcurrent: 5,
      priorities: EDGE_PRIORITIES,
    });
  });

  describe('slot allocation', () => {
    it('should grant slots up to maxConcurrent limit', () => {
      expect(budget.requestSlot('edge-1', 'default')).toBe(true);
      expect(budget.requestSlot('edge-2', 'default')).toBe(true);
      expect(budget.requestSlot('edge-3', 'default')).toBe(true);
      expect(budget.requestSlot('edge-4', 'default')).toBe(true);
      expect(budget.requestSlot('edge-5', 'default')).toBe(true);

      const stats = budget.getStats();
      expect(stats.current).toBe(5);
      expect(stats.max).toBe(5);
    });

    it('should deny slots when at capacity with same priority', () => {
      // Fill up with default priority
      for (let i = 0; i < 5; i++) {
        budget.requestSlot(`edge-${i}`, 'default');
      }

      // Should deny new default priority
      expect(budget.requestSlot('edge-new', 'default')).toBe(false);
    });

    it('should not double-allocate the same edge', () => {
      budget.requestSlot('edge-1', 'default');
      budget.requestSlot('edge-1', 'default');
      budget.requestSlot('edge-1', 'default');

      expect(budget.getStats().current).toBe(1);
    });

    it('should update priority for existing edge', () => {
      budget.requestSlot('edge-1', 'default');
      budget.requestSlot('edge-1', 'selected'); // Upgrade priority

      expect(budget.getStats().current).toBe(1);
      expect(budget.canAnimate('edge-1')).toBe(true);
    });
  });

  describe('priority-based eviction', () => {
    it('should evict lower priority edge for higher priority request', () => {
      // Fill with default priority
      for (let i = 0; i < 5; i++) {
        budget.requestSlot(`edge-${i}`, 'default');
      }

      // Request with higher priority should evict a default
      expect(budget.requestSlot('edge-selected', 'selected')).toBe(true);
      expect(budget.canAnimate('edge-selected')).toBe(true);

      // One of the defaults should be evicted
      const stats = budget.getStats();
      expect(stats.current).toBe(5);
    });

    it('should respect priority hierarchy: selected > highlighted > connected > default', () => {
      // Fill with mixed priorities
      budget.requestSlot('edge-default', 'default');
      budget.requestSlot('edge-connected', 'connected');
      budget.requestSlot('edge-highlighted', 'highlighted');
      budget.requestSlot('edge-selected', 'selected');
      budget.requestSlot('edge-default2', 'default');

      // Try to add another selected - should evict lowest priority (default)
      expect(budget.requestSlot('edge-selected2', 'selected')).toBe(true);

      // Selected edges should still be animating
      expect(budget.canAnimate('edge-selected')).toBe(true);
      expect(budget.canAnimate('edge-selected2')).toBe(true);
    });
  });

  describe('slot release', () => {
    it('should release slot and allow new allocations', () => {
      // Fill up
      for (let i = 0; i < 5; i++) {
        budget.requestSlot(`edge-${i}`, 'default');
      }

      // Release one (no waiting edges, count drops)
      budget.releaseSlot('edge-2');
      expect(budget.getStats().current).toBe(4);

      // Now new allocation should work
      expect(budget.requestSlot('edge-new', 'default')).toBe(true);
      expect(budget.getStats().current).toBe(5);
    });

    it('should auto-promote waiting edge when slot released', () => {
      // Fill up
      for (let i = 0; i < 5; i++) {
        budget.requestSlot(`edge-${i}`, 'default');
      }

      // This goes to waiting queue
      expect(budget.requestSlot('edge-waiting', 'default')).toBe(false);
      expect(budget.getStats().waiting).toBe(1);

      // Release one - waiting edge should be auto-promoted
      budget.releaseSlot('edge-2');
      expect(budget.getStats().current).toBe(5); // Still 5 due to promotion
      expect(budget.getStats().waiting).toBe(0);
      expect(budget.canAnimate('edge-waiting')).toBe(true);
    });

    it('should handle releasing non-existent edge gracefully', () => {
      budget.requestSlot('edge-1', 'default');
      budget.releaseSlot('edge-nonexistent');

      expect(budget.getStats().current).toBe(1);
    });
  });

  describe('canAnimate', () => {
    it('should return true for allocated edges', () => {
      budget.requestSlot('edge-1', 'default');
      expect(budget.canAnimate('edge-1')).toBe(true);
    });

    it('should return false for non-allocated edges', () => {
      expect(budget.canAnimate('edge-nonexistent')).toBe(false);
    });

    it('should return false after release', () => {
      budget.requestSlot('edge-1', 'default');
      budget.releaseSlot('edge-1');
      expect(budget.canAnimate('edge-1')).toBe(false);
    });
  });

  describe('getStats', () => {
    it('should return accurate statistics', () => {
      budget.requestSlot('edge-1', 'selected');
      budget.requestSlot('edge-2', 'highlighted');
      budget.requestSlot('edge-3', 'default');

      const stats = budget.getStats();
      expect(stats.current).toBe(3);
      expect(stats.max).toBe(5);
      expect(stats.waiting).toBe(0);
      expect(stats.utilization).toBeCloseTo(0.6);
    });

    it('should track waiting edges', () => {
      // Fill up
      for (let i = 0; i < 5; i++) {
        budget.requestSlot(`edge-${i}`, 'default');
      }
      // Add one more that will wait
      budget.requestSlot('edge-waiting', 'default');

      const stats = budget.getStats();
      expect(stats.current).toBe(5);
      expect(stats.waiting).toBe(1);
    });
  });

  describe('clear', () => {
    it('should clear all allocations', () => {
      budget.requestSlot('edge-1', 'default');
      budget.requestSlot('edge-2', 'selected');

      budget.clear();

      expect(budget.getStats().current).toBe(0);
      expect(budget.canAnimate('edge-1')).toBe(false);
      expect(budget.canAnimate('edge-2')).toBe(false);
    });
  });

  describe('updatePriority', () => {
    it('should update priority for active edge', () => {
      budget.requestSlot('edge-1', 'default');
      expect(budget.canAnimate('edge-1')).toBe(true);

      budget.updatePriority('edge-1', 'selected');
      expect(budget.canAnimate('edge-1')).toBe(true);
    });

    it('should promote waiting edge when priority increases', () => {
      // Fill with default
      for (let i = 0; i < 5; i++) {
        budget.requestSlot(`edge-${i}`, 'default');
      }
      // Add waiting edge
      budget.requestSlot('edge-waiting', 'default');
      expect(budget.canAnimate('edge-waiting')).toBe(false);

      // Upgrade to selected - should evict a default
      budget.updatePriority('edge-waiting', 'selected');
      expect(budget.canAnimate('edge-waiting')).toBe(true);
    });
  });
});
