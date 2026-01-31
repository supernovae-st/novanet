'use client';

/**
 * useMagneticSimulation - d3-force simulation for magnetic grouping
 *
 * Forces applied:
 * 1. Attraction to subcategory (via IN_SUBCATEGORY)
 * 2. Repulsion between same-type nodes
 * 3. Collision detection
 * 4. Center gravity (weak)
 */

import { useCallback, useRef, useEffect } from 'react';
import {
  forceSimulation,
  forceLink,
  forceManyBody,
  forceCollide,
  forceX,
  forceY,
  type Simulation,
  type SimulationNodeDatum,
  type SimulationLinkDatum,
} from 'd3-force';
import type { Node } from '@xyflow/react';

interface SimNode extends SimulationNodeDatum {
  id: string;
  fx?: number | null;
  fy?: number | null;
  isAttractor?: boolean;
}

interface SimLink extends SimulationLinkDatum<SimNode> {
  source: string | SimNode;
  target: string | SimNode;
  strength?: number;
}

export interface UseMagneticSimulationOptions {
  /** Strength of attraction to subcategory (0-1) */
  attractionStrength?: number;
  /** Strength of repulsion between nodes */
  repulsionStrength?: number;
  /** Node collision radius */
  collisionRadius?: number;
  /** Whether simulation is running */
  enabled?: boolean;
}

export function useMagneticSimulation(
  nodes: Node[],
  edges: Array<{ source: string; target: string; type?: string }>,
  options: UseMagneticSimulationOptions = {}
) {
  const {
    attractionStrength = 0.3,
    repulsionStrength = -100,
    collisionRadius = 60,
    enabled = true,
  } = options;

  const simulationRef = useRef<Simulation<SimNode, SimLink> | null>(null);
  const nodesRef = useRef<Map<string, SimNode>>(new Map());

  const updatePositions = useCallback((callback: (positions: Map<string, { x: number; y: number }>) => void) => {
    if (!simulationRef.current) return;

    const positions = new Map<string, { x: number; y: number }>();
    nodesRef.current.forEach((node, id) => {
      if (node.x !== undefined && node.y !== undefined) {
        positions.set(id, { x: node.x, y: node.y });
      }
    });
    callback(positions);
  }, []);

  const initSimulation = useCallback(() => {
    // Convert React Flow nodes to simulation nodes
    const simNodes: SimNode[] = nodes.map(n => ({
      id: n.id,
      x: n.position.x,
      y: n.position.y,
      // Fix attractor positions (scopes and subcategories)
      fx: n.type?.includes('Attractor') ? n.position.x : null,
      fy: n.type?.includes('Attractor') ? n.position.y : null,
      isAttractor: n.type?.includes('Attractor'),
    }));

    // Store in ref for position updates
    nodesRef.current.clear();
    simNodes.forEach(n => nodesRef.current.set(n.id, n));

    // Convert edges to simulation links
    // IN_SUBCATEGORY edges have stronger attraction
    const simLinks: SimLink[] = edges
      .filter(e => e.type === 'magnetic' || e.type === 'IN_SUBCATEGORY')
      .map(e => ({
        source: e.source,
        target: e.target,
        strength: attractionStrength,
      }));

    // Create simulation
    const simulation = forceSimulation<SimNode>(simNodes)
      .force('link', forceLink<SimNode, SimLink>(simLinks)
        .id(d => d.id)
        .distance(150)
        .strength(d => d.strength || attractionStrength)
      )
      .force('charge', forceManyBody<SimNode>()
        .strength(d => d.isAttractor ? 0 : repulsionStrength)
      )
      .force('collide', forceCollide<SimNode>(collisionRadius))
      .force('x', forceX<SimNode>().strength(0.01))
      .force('y', forceY<SimNode>().strength(0.01))
      .alphaDecay(0.02)
      .velocityDecay(0.3);

    simulationRef.current = simulation;

    return simulation;
  }, [nodes, edges, attractionStrength, repulsionStrength, collisionRadius]);

  const startSimulation = useCallback(() => {
    if (!enabled) return;

    const simulation = initSimulation();
    simulation.alpha(1).restart();
  }, [enabled, initSimulation]);

  const stopSimulation = useCallback(() => {
    simulationRef.current?.stop();
  }, []);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      simulationRef.current?.stop();
    };
  }, []);

  return {
    startSimulation,
    stopSimulation,
    updatePositions,
    simulation: simulationRef.current,
  };
}
