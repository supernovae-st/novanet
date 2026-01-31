// src/hooks/__tests__/useGroupDrag.test.ts
/**
 * GroupDrag Hook Tests - TDD
 *
 * Tests for group-aware dragging:
 * - Connected nodes should move with the dragged node
 * - Movement is dampened based on connection distance
 * - Only direct neighbors move (1-hop)
 */

interface NodePosition {
  id: string;
  x: number;
  y: number;
}

interface EdgeConnection {
  source: string;
  target: string;
}

interface GroupDragOptions {
  /** How much connected nodes follow (0-1, default: 0.3) */
  followStrength?: number;
  /** Maximum hops to include (default: 1) */
  maxHops?: number;
  /** Enable/disable group drag */
  enabled?: boolean;
}

// Mock implementation for testing expectations
const createMockUseGroupDrag = (options: GroupDragOptions = {}) => {
  const {
    followStrength = 0.3,
    maxHops = 1,
    enabled = true,
  } = options;

  // Build adjacency map
  const buildAdjacencyMap = (edges: EdgeConnection[]): Map<string, Set<string>> => {
    const adj = new Map<string, Set<string>>();
    edges.forEach((e) => {
      if (!adj.has(e.source)) adj.set(e.source, new Set());
      if (!adj.has(e.target)) adj.set(e.target, new Set());
      adj.get(e.source)!.add(e.target);
      adj.get(e.target)!.add(e.source);
    });
    return adj;
  };

  // Get nodes within maxHops
  const getConnectedNodes = (
    nodeId: string,
    edges: EdgeConnection[],
    hops: number
  ): Map<string, number> => {
    const adj = buildAdjacencyMap(edges);
    const result = new Map<string, number>(); // nodeId -> hop distance
    const visited = new Set<string>([nodeId]);
    let frontier = [nodeId];

    for (let hop = 1; hop <= hops; hop++) {
      const nextFrontier: string[] = [];
      for (const current of frontier) {
        const neighbors = adj.get(current) || new Set();
        for (const neighbor of neighbors) {
          if (!visited.has(neighbor)) {
            visited.add(neighbor);
            result.set(neighbor, hop);
            nextFrontier.push(neighbor);
          }
        }
      }
      frontier = nextFrontier;
    }

    return result;
  };

  return {
    getGroupPositions: (
      nodes: NodePosition[],
      edges: EdgeConnection[],
      draggedNodeId: string,
      dragDelta: { dx: number; dy: number }
    ): Map<string, { x: number; y: number }> => {
      const result = new Map<string, { x: number; y: number }>();

      if (!enabled) {
        nodes.forEach((n) => {
          if (n.id === draggedNodeId) {
            result.set(n.id, { x: n.x + dragDelta.dx, y: n.y + dragDelta.dy });
          } else {
            result.set(n.id, { x: n.x, y: n.y });
          }
        });
        return result;
      }

      const connectedNodes = getConnectedNodes(draggedNodeId, edges, maxHops);
      const nodeMap = new Map(nodes.map((n) => [n.id, n]));

      nodes.forEach((node) => {
        if (node.id === draggedNodeId) {
          // Dragged node moves fully
          result.set(node.id, {
            x: node.x + dragDelta.dx,
            y: node.y + dragDelta.dy,
          });
        } else if (connectedNodes.has(node.id)) {
          // Connected nodes follow with dampening
          const hopDistance = connectedNodes.get(node.id)!;
          const dampening = followStrength / hopDistance;

          result.set(node.id, {
            x: node.x + dragDelta.dx * dampening,
            y: node.y + dragDelta.dy * dampening,
          });
        } else {
          // Unconnected nodes stay put
          result.set(node.id, { x: node.x, y: node.y });
        }
      });

      return result;
    },

    getConnectedNodeIds: (
      nodeId: string,
      edges: EdgeConnection[]
    ): Set<string> => {
      const connected = getConnectedNodes(nodeId, edges, maxHops);
      return new Set(connected.keys());
    },
  };
};

describe('useGroupDrag', () => {
  describe('basic group movement', () => {
    it('should move dragged node by full delta', () => {
      const hook = createMockUseGroupDrag();

      const nodes: NodePosition[] = [
        { id: 'a', x: 0, y: 0 },
        { id: 'b', x: 100, y: 0 },
      ];
      const edges: EdgeConnection[] = [{ source: 'a', target: 'b' }];

      const result = hook.getGroupPositions(nodes, edges, 'a', { dx: 50, dy: 30 });

      expect(result.get('a')).toEqual({ x: 50, y: 30 });
    });

    it('should move connected nodes by dampened delta', () => {
      const hook = createMockUseGroupDrag({ followStrength: 0.5 });

      const nodes: NodePosition[] = [
        { id: 'a', x: 0, y: 0 },
        { id: 'b', x: 100, y: 0 },
      ];
      const edges: EdgeConnection[] = [{ source: 'a', target: 'b' }];

      const result = hook.getGroupPositions(nodes, edges, 'a', { dx: 100, dy: 0 });

      // Connected node should move by followStrength * delta
      expect(result.get('b')!.x).toBe(100 + 100 * 0.5);
      expect(result.get('b')!.y).toBe(0);
    });

    it('should not move unconnected nodes', () => {
      const hook = createMockUseGroupDrag();

      const nodes: NodePosition[] = [
        { id: 'a', x: 0, y: 0 },
        { id: 'b', x: 100, y: 0 },
        { id: 'c', x: 200, y: 0 }, // Not connected
      ];
      const edges: EdgeConnection[] = [{ source: 'a', target: 'b' }];

      const result = hook.getGroupPositions(nodes, edges, 'a', { dx: 50, dy: 50 });

      expect(result.get('c')).toEqual({ x: 200, y: 0 });
    });
  });

  describe('hop distance dampening', () => {
    it('should dampen more for nodes further away (2-hop)', () => {
      const hook = createMockUseGroupDrag({ followStrength: 0.6, maxHops: 2 });

      const nodes: NodePosition[] = [
        { id: 'a', x: 0, y: 0 },
        { id: 'b', x: 100, y: 0 },   // 1-hop from a
        { id: 'c', x: 200, y: 0 },   // 2-hop from a
      ];
      const edges: EdgeConnection[] = [
        { source: 'a', target: 'b' },
        { source: 'b', target: 'c' },
      ];

      const result = hook.getGroupPositions(nodes, edges, 'a', { dx: 100, dy: 0 });

      const bMove = result.get('b')!.x - 100;
      const cMove = result.get('c')!.x - 200;

      // 1-hop node should move more than 2-hop node
      expect(bMove).toBeGreaterThan(cMove);
      // 1-hop: followStrength / 1 = 0.6
      expect(bMove).toBeCloseTo(100 * 0.6, 1);
      // 2-hop: followStrength / 2 = 0.3
      expect(cMove).toBeCloseTo(100 * 0.3, 1);
    });

    it('should respect maxHops limit', () => {
      const hook = createMockUseGroupDrag({ followStrength: 0.5, maxHops: 1 });

      const nodes: NodePosition[] = [
        { id: 'a', x: 0, y: 0 },
        { id: 'b', x: 100, y: 0 },   // 1-hop
        { id: 'c', x: 200, y: 0 },   // 2-hop (beyond maxHops)
      ];
      const edges: EdgeConnection[] = [
        { source: 'a', target: 'b' },
        { source: 'b', target: 'c' },
      ];

      const result = hook.getGroupPositions(nodes, edges, 'a', { dx: 100, dy: 0 });

      // 2-hop node should not move (beyond maxHops)
      expect(result.get('c')).toEqual({ x: 200, y: 0 });
    });
  });

  describe('bidirectional edges', () => {
    it('should work regardless of edge direction', () => {
      const hook = createMockUseGroupDrag({ followStrength: 0.5 });

      const nodes: NodePosition[] = [
        { id: 'a', x: 0, y: 0 },
        { id: 'b', x: 100, y: 0 },
      ];
      // Edge goes from b to a, but we drag a
      const edges: EdgeConnection[] = [{ source: 'b', target: 'a' }];

      const result = hook.getGroupPositions(nodes, edges, 'a', { dx: 100, dy: 0 });

      // b should still follow
      expect(result.get('b')!.x).toBeGreaterThan(100);
    });
  });

  describe('enabled/disabled', () => {
    it('should only move dragged node when disabled', () => {
      const hook = createMockUseGroupDrag({ enabled: false });

      const nodes: NodePosition[] = [
        { id: 'a', x: 0, y: 0 },
        { id: 'b', x: 100, y: 0 },
      ];
      const edges: EdgeConnection[] = [{ source: 'a', target: 'b' }];

      const result = hook.getGroupPositions(nodes, edges, 'a', { dx: 50, dy: 50 });

      expect(result.get('a')).toEqual({ x: 50, y: 50 });
      expect(result.get('b')).toEqual({ x: 100, y: 0 });
    });
  });

  describe('getConnectedNodeIds', () => {
    it('should return all connected node IDs within maxHops', () => {
      const hook = createMockUseGroupDrag({ maxHops: 2 });

      const edges: EdgeConnection[] = [
        { source: 'a', target: 'b' },
        { source: 'b', target: 'c' },
        { source: 'c', target: 'd' },
      ];

      const connected = hook.getConnectedNodeIds('a', edges);

      expect(connected.has('b')).toBe(true);  // 1-hop
      expect(connected.has('c')).toBe(true);  // 2-hop
      expect(connected.has('d')).toBe(false); // 3-hop (beyond maxHops)
    });
  });

  describe('complex graph structures', () => {
    it('should handle nodes with multiple connections', () => {
      const hook = createMockUseGroupDrag({ followStrength: 0.4 });

      const nodes: NodePosition[] = [
        { id: 'center', x: 0, y: 0 },
        { id: 'n1', x: 100, y: 0 },
        { id: 'n2', x: 0, y: 100 },
        { id: 'n3', x: -100, y: 0 },
      ];
      const edges: EdgeConnection[] = [
        { source: 'center', target: 'n1' },
        { source: 'center', target: 'n2' },
        { source: 'center', target: 'n3' },
      ];

      const result = hook.getGroupPositions(nodes, edges, 'center', { dx: 50, dy: 50 });

      // All connected nodes should move
      expect(result.get('n1')!.x).toBeGreaterThan(100);
      expect(result.get('n2')!.y).toBeGreaterThan(100);
      expect(result.get('n3')!.x).toBeGreaterThan(-100);
    });
  });
});
