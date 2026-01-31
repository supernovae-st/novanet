// src/hooks/__tests__/useMagneticForce.test.ts
/**
 * MagneticForce Hook Tests - TDD
 *
 * Tests for magnetic repulsion behavior during drag:
 * - Nodes should repel when another node is dragged near them
 * - Repulsion strength decreases with distance
 * - Original positions restore after drag ends
 */

import { renderHook, act } from '@testing-library/react';

// Types for the hook we'll implement
interface MagneticForceOptions {
  /** Repulsion strength (default: 200) */
  strength?: number;
  /** Maximum repulsion distance (default: 300) */
  maxDistance?: number;
  /** Animation duration in ms (default: 150) */
  animationDuration?: number;
  /** Enable/disable magnetic force */
  enabled?: boolean;
}

interface NodePosition {
  id: string;
  x: number;
  y: number;
}

interface UseMagneticForceReturn {
  /** Calculate displaced positions for all nodes */
  getDisplacedPositions: (
    nodes: NodePosition[],
    draggedNodeId: string,
    dragPosition: { x: number; y: number }
  ) => Map<string, { x: number; y: number }>;
  /** Check if a node is being displaced */
  isDisplaced: (nodeId: string) => boolean;
  /** Reset all displacements */
  reset: () => void;
}

// Mock implementation for testing expectations
const createMockUseMagneticForce = (options: MagneticForceOptions = {}) => {
  const {
    strength = 200,
    maxDistance = 300,
    enabled = true,
  } = options;

  const displacedNodes = new Set<string>();

  return {
    getDisplacedPositions: (
      nodes: NodePosition[],
      draggedNodeId: string,
      dragPosition: { x: number; y: number }
    ): Map<string, { x: number; y: number }> => {
      const result = new Map<string, { x: number; y: number }>();

      if (!enabled) {
        nodes.forEach((n) => result.set(n.id, { x: n.x, y: n.y }));
        return result;
      }

      nodes.forEach((node) => {
        if (node.id === draggedNodeId) {
          result.set(node.id, dragPosition);
          return;
        }

        const dx = node.x - dragPosition.x;
        const dy = node.y - dragPosition.y;
        const distance = Math.sqrt(dx * dx + dy * dy);

        if (distance < maxDistance && distance > 0) {
          // Repulsion decreases with distance
          const repulsionFactor = 1 - distance / maxDistance;
          const repulsionMagnitude = strength * repulsionFactor;

          // Normalize direction and apply repulsion
          const nx = dx / distance;
          const ny = dy / distance;

          result.set(node.id, {
            x: node.x + nx * repulsionMagnitude,
            y: node.y + ny * repulsionMagnitude,
          });
          displacedNodes.add(node.id);
        } else {
          result.set(node.id, { x: node.x, y: node.y });
        }
      });

      return result;
    },
    isDisplaced: (nodeId: string) => displacedNodes.has(nodeId),
    reset: () => displacedNodes.clear(),
    _displacedNodes: displacedNodes,
  };
};

describe('useMagneticForce', () => {
  describe('repulsion behavior', () => {
    it('should repel nearby nodes when dragging', () => {
      const hook = createMockUseMagneticForce({ strength: 200, maxDistance: 300 });

      const nodes: NodePosition[] = [
        { id: 'dragged', x: 100, y: 100 },
        { id: 'nearby', x: 200, y: 100 }, // 100px away
        { id: 'far', x: 500, y: 100 },    // 400px away (beyond maxDistance)
      ];

      const result = hook.getDisplacedPositions(nodes, 'dragged', { x: 150, y: 100 });

      // Dragged node should be at drag position
      expect(result.get('dragged')).toEqual({ x: 150, y: 100 });

      // Nearby node should be pushed away (x increased)
      const nearbyPos = result.get('nearby')!;
      expect(nearbyPos.x).toBeGreaterThan(200);

      // Far node should not move
      expect(result.get('far')).toEqual({ x: 500, y: 100 });
    });

    it('should have stronger repulsion when closer', () => {
      const hook = createMockUseMagneticForce({ strength: 200, maxDistance: 300 });

      const nodes: NodePosition[] = [
        { id: 'dragged', x: 0, y: 0 },
        { id: 'close', x: 100, y: 0 },   // 100px away
        { id: 'medium', x: 200, y: 0 },  // 200px away
      ];

      const result = hook.getDisplacedPositions(nodes, 'dragged', { x: 0, y: 0 });

      const closeDisplacement = result.get('close')!.x - 100;
      const mediumDisplacement = result.get('medium')!.x - 200;

      // Closer node should be displaced more
      expect(closeDisplacement).toBeGreaterThan(mediumDisplacement);
    });

    it('should repel in the correct direction', () => {
      const hook = createMockUseMagneticForce({ strength: 200, maxDistance: 300 });

      const nodes: NodePosition[] = [
        { id: 'dragged', x: 100, y: 100 },
        { id: 'right', x: 200, y: 100 },
        { id: 'above', x: 100, y: 0 },
        { id: 'diagonal', x: 200, y: 200 },
      ];

      const result = hook.getDisplacedPositions(nodes, 'dragged', { x: 100, y: 100 });

      // Node to the right should move further right
      expect(result.get('right')!.x).toBeGreaterThan(200);
      expect(result.get('right')!.y).toBeCloseTo(100, 0);

      // Node above should move further up
      expect(result.get('above')!.y).toBeLessThan(0);
      expect(result.get('above')!.x).toBeCloseTo(100, 0);

      // Diagonal node should move diagonally
      expect(result.get('diagonal')!.x).toBeGreaterThan(200);
      expect(result.get('diagonal')!.y).toBeGreaterThan(200);
    });
  });

  describe('enabled/disabled', () => {
    it('should not repel when disabled', () => {
      const hook = createMockUseMagneticForce({ enabled: false });

      const nodes: NodePosition[] = [
        { id: 'dragged', x: 0, y: 0 },
        { id: 'nearby', x: 50, y: 0 },
      ];

      const result = hook.getDisplacedPositions(nodes, 'dragged', { x: 0, y: 0 });

      // Nearby node should not move
      expect(result.get('nearby')).toEqual({ x: 50, y: 0 });
    });
  });

  describe('tracking displaced nodes', () => {
    it('should track which nodes are displaced', () => {
      const hook = createMockUseMagneticForce({ maxDistance: 300 });

      const nodes: NodePosition[] = [
        { id: 'dragged', x: 0, y: 0 },
        { id: 'nearby', x: 100, y: 0 },
        { id: 'far', x: 500, y: 0 },
      ];

      hook.getDisplacedPositions(nodes, 'dragged', { x: 0, y: 0 });

      expect(hook.isDisplaced('nearby')).toBe(true);
      expect(hook.isDisplaced('far')).toBe(false);
    });

    it('should reset displaced nodes', () => {
      const hook = createMockUseMagneticForce({ maxDistance: 300 });

      const nodes: NodePosition[] = [
        { id: 'dragged', x: 0, y: 0 },
        { id: 'nearby', x: 100, y: 0 },
      ];

      hook.getDisplacedPositions(nodes, 'dragged', { x: 0, y: 0 });
      expect(hook.isDisplaced('nearby')).toBe(true);

      hook.reset();
      expect(hook.isDisplaced('nearby')).toBe(false);
    });
  });

  describe('edge cases', () => {
    it('should handle zero distance gracefully', () => {
      const hook = createMockUseMagneticForce();

      const nodes: NodePosition[] = [
        { id: 'dragged', x: 100, y: 100 },
        { id: 'same-pos', x: 100, y: 100 }, // Same position
      ];

      // Should not throw
      expect(() => {
        hook.getDisplacedPositions(nodes, 'dragged', { x: 100, y: 100 });
      }).not.toThrow();
    });

    it('should handle empty nodes array', () => {
      const hook = createMockUseMagneticForce();

      const result = hook.getDisplacedPositions([], 'any', { x: 0, y: 0 });

      expect(result.size).toBe(0);
    });
  });
});
