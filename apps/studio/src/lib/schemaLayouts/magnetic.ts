// src/lib/schemaLayouts/magnetic.ts
/**
 * Magnetic Layout - Force-directed with Neo4j-driven grouping
 *
 * Unlike other layouts, this one:
 * 1. Displays Realm/Layer as VISIBLE nodes (not containers)
 * 2. Uses OF_KIND relationships for magnetic attraction
 * 3. Runs d3-force simulation with custom forces
 *
 * Visual structure:
 *
 *           Realm:project
 *          / \
 *         /   \ HAS_LAYER
 *        /     \
 *   Layer:semantic  Layer:structure
 *    /\              /\
 *   o o o          o o o   <- attracted via OF_KIND
 */

import type { Node, Edge } from '@xyflow/react';
import type { SchemaLayoutResult } from './types';
import { NODE_WIDTH, NODE_HEIGHT, NODE_GAP } from './types';

export interface MagneticLayoutInput {
  // Organizing principle nodes from Neo4j
  realms: Array<{
    key: string;
    displayName: string;
    emoji: string;
    color: string;
  }>;
  layers: Array<{
    key: string;
    displayName: string;
    emoji: string;
    realmKey: string;  // Parent realm
  }>;
  // Instance nodes with their layer
  instances: Array<{
    id: string;
    label: string;
    nodeType: string;
    layerKey: string;
    // Original Neo4j properties
    properties: Record<string, unknown>;
  }>;
  // Relationships between nodes
  relationships: Array<{
    source: string;
    target: string;
    type: string;
  }>;
}

/**
 * Initial positions before simulation
 * Places layers around their realm in a circle
 * Places instances near their layer
 */
function computeInitialPositions(input: MagneticLayoutInput): Map<string, { x: number; y: number }> {
  const positions = new Map<string, { x: number; y: number }>();

  // Realm positions (triangular arrangement)
  const realmPositions: Record<string, { x: number; y: number }> = {
    project: { x: 0, y: 0 },
    global: { x: 2000, y: 0 },
    shared: { x: 1000, y: 1500 },
  };

  // Place realms
  for (const realm of input.realms) {
    const pos = realmPositions[realm.key] || { x: 0, y: 0 };
    positions.set(`realm-${realm.key}`, pos);
  }

  // Place layers around their realm
  const layersByRealm = new Map<string, typeof input.layers>();
  for (const sub of input.layers) {
    const list = layersByRealm.get(sub.realmKey) || [];
    list.push(sub);
    layersByRealm.set(sub.realmKey, list);
  }

  for (const [realmKey, subs] of layersByRealm) {
    const realmPos = positions.get(`realm-${realmKey}`) || { x: 0, y: 0 };
    const radius = 400;

    subs.forEach((sub, i) => {
      const angle = (2 * Math.PI * i) / subs.length - Math.PI / 2;
      positions.set(`layer-${sub.key}`, {
        x: realmPos.x + radius * Math.cos(angle),
        y: realmPos.y + radius * Math.sin(angle),
      });
    });
  }

  // Place instances near their layer (with jitter)
  // Use seeded random for consistent initial positions
  let seed = 12345;
  const seededRandom = () => {
    seed = (seed * 1103515245 + 12345) & 0x7fffffff;
    return seed / 0x7fffffff;
  };

  for (const instance of input.instances) {
    const layerPos = positions.get(`layer-${instance.layerKey}`) || { x: 0, y: 0 };
    positions.set(instance.id, {
      x: layerPos.x + (seededRandom() - 0.5) * 300,
      y: layerPos.y + (seededRandom() - 0.5) * 300,
    });
  }

  return positions;
}

/**
 * Apply magnetic layout
 * Returns nodes and edges for React Flow
 */
export function applyMagneticLayout(input: MagneticLayoutInput): SchemaLayoutResult {
  const nodes: Node[] = [];
  const edges: Edge[] = [];

  const positions = computeInitialPositions(input);

  // Create Realm nodes (large, prominent)
  for (const realm of input.realms) {
    const pos = positions.get(`realm-${realm.key}`)!;
    nodes.push({
      id: `realm-${realm.key}`,
      type: 'realmAttractor',  // New node type
      position: pos,
      data: {
        key: realm.key,
        label: realm.displayName,
        emoji: realm.emoji,
        color: realm.color,
        nodeCount: input.instances.filter(i =>
          input.layers.find(s => s.key === i.layerKey)?.realmKey === realm.key
        ).length,
      },
    });
  }

  // Create Layer nodes (medium size)
  for (const sub of input.layers) {
    const pos = positions.get(`layer-${sub.key}`)!;
    const realmNode = input.realms.find(s => s.key === sub.realmKey);

    nodes.push({
      id: `layer-${sub.key}`,
      type: 'layerAttractor',  // New node type
      position: pos,
      data: {
        key: sub.key,
        label: sub.displayName,
        emoji: sub.emoji,
        realmKey: sub.realmKey,
        color: realmNode?.color || '#666',
        nodeCount: input.instances.filter(i => i.layerKey === sub.key).length,
      },
    });

    // Edge from Realm to Layer (HAS_LAYER)
    edges.push({
      id: `edge-realm-${sub.realmKey}-to-${sub.key}`,
      source: `realm-${sub.realmKey}`,
      target: `layer-${sub.key}`,
      type: 'floating',
      data: { relationType: 'HAS_LAYER' },
    });
  }

  // Create instance nodes
  for (const instance of input.instances) {
    const pos = positions.get(instance.id)!;
    const sub = input.layers.find(s => s.key === instance.layerKey);
    const realmNode = input.realms.find(s => s.key === sub?.realmKey);

    nodes.push({
      id: instance.id,
      type: 'schemaNode',  // Reuse existing node type
      position: pos,
      draggable: true,
      style: { width: NODE_WIDTH, height: NODE_HEIGHT },
      data: {
        nodeType: instance.nodeType,
        label: instance.label,
        realm: realmNode?.key,
        layer: instance.layerKey,
        ...instance.properties,
      },
    });

    // Edge from instance to Layer (OF_KIND) - rendered faintly
    edges.push({
      id: `edge-${instance.id}-to-layer-${instance.layerKey}`,
      source: instance.id,
      target: `layer-${instance.layerKey}`,
      type: 'magnetic',  // New edge type (faint, dashed)
      data: { relationType: 'OF_KIND' },
    });
  }

  // Add business relationships between instances
  for (const rel of input.relationships) {
    // Skip OF_KIND (already added above)
    if (rel.type === 'OF_KIND') continue;

    edges.push({
      id: `edge-${rel.source}-${rel.type}-${rel.target}`,
      source: rel.source,
      target: rel.target,
      type: 'floating',
      data: { relationType: rel.type },
    });
  }

  return { nodes, edges };
}
