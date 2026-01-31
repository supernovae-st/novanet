// src/lib/schemaLayouts/magnetic.ts
/**
 * Magnetic Layout - Force-directed with Neo4j-driven grouping
 *
 * Unlike other layouts, this one:
 * 1. Displays Scope/Subcategory as VISIBLE nodes (not containers)
 * 2. Uses IN_SUBCATEGORY relationships for magnetic attraction
 * 3. Runs d3-force simulation with custom forces
 *
 * Visual structure:
 *
 *           Scope:project
 *          / \
 *         /   \ HAS_SUBCATEGORY
 *        /     \
 *   Sub:semantic  Sub:structure
 *    /\              /\
 *   o o o          o o o   <- attracted via IN_SUBCATEGORY
 */

import type { Node, Edge } from '@xyflow/react';
import type { SchemaLayoutResult } from './types';
import { NODE_WIDTH, NODE_HEIGHT, NODE_GAP } from './types';

export interface MagneticLayoutInput {
  // Organizing principle nodes from Neo4j
  scopes: Array<{
    key: string;
    displayName: string;
    emoji: string;
    color: string;
  }>;
  subcategories: Array<{
    key: string;
    displayName: string;
    emoji: string;
    scopeKey: string;  // Parent scope
  }>;
  // Instance nodes with their subcategory
  instances: Array<{
    id: string;
    label: string;
    nodeType: string;
    subcategoryKey: string;
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
 * Places subcategories around their scope in a circle
 * Places instances near their subcategory
 */
function computeInitialPositions(input: MagneticLayoutInput): Map<string, { x: number; y: number }> {
  const positions = new Map<string, { x: number; y: number }>();

  // Scope positions (triangular arrangement)
  const scopePositions: Record<string, { x: number; y: number }> = {
    project: { x: 0, y: 0 },
    global: { x: 2000, y: 0 },
    shared: { x: 1000, y: 1500 },
  };

  // Place scopes
  for (const scope of input.scopes) {
    const pos = scopePositions[scope.key] || { x: 0, y: 0 };
    positions.set(`scope-${scope.key}`, pos);
  }

  // Place subcategories around their scope
  const subcatsByScope = new Map<string, typeof input.subcategories>();
  for (const sub of input.subcategories) {
    const list = subcatsByScope.get(sub.scopeKey) || [];
    list.push(sub);
    subcatsByScope.set(sub.scopeKey, list);
  }

  for (const [scopeKey, subs] of subcatsByScope) {
    const scopePos = positions.get(`scope-${scopeKey}`) || { x: 0, y: 0 };
    const radius = 400;

    subs.forEach((sub, i) => {
      const angle = (2 * Math.PI * i) / subs.length - Math.PI / 2;
      positions.set(`subcat-${sub.key}`, {
        x: scopePos.x + radius * Math.cos(angle),
        y: scopePos.y + radius * Math.sin(angle),
      });
    });
  }

  // Place instances near their subcategory (with jitter)
  // Use seeded random for consistent initial positions
  let seed = 12345;
  const seededRandom = () => {
    seed = (seed * 1103515245 + 12345) & 0x7fffffff;
    return seed / 0x7fffffff;
  };

  for (const instance of input.instances) {
    const subcatPos = positions.get(`subcat-${instance.subcategoryKey}`) || { x: 0, y: 0 };
    positions.set(instance.id, {
      x: subcatPos.x + (seededRandom() - 0.5) * 300,
      y: subcatPos.y + (seededRandom() - 0.5) * 300,
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

  // Create Scope nodes (large, prominent)
  for (const scope of input.scopes) {
    const pos = positions.get(`scope-${scope.key}`)!;
    nodes.push({
      id: `scope-${scope.key}`,
      type: 'scopeAttractor',  // New node type
      position: pos,
      data: {
        key: scope.key,
        label: scope.displayName,
        emoji: scope.emoji,
        color: scope.color,
        nodeCount: input.instances.filter(i =>
          input.subcategories.find(s => s.key === i.subcategoryKey)?.scopeKey === scope.key
        ).length,
      },
    });
  }

  // Create Subcategory nodes (medium size)
  for (const sub of input.subcategories) {
    const pos = positions.get(`subcat-${sub.key}`)!;
    const scope = input.scopes.find(s => s.key === sub.scopeKey);

    nodes.push({
      id: `subcat-${sub.key}`,
      type: 'subcategoryAttractor',  // New node type
      position: pos,
      data: {
        key: sub.key,
        label: sub.displayName,
        emoji: sub.emoji,
        scopeKey: sub.scopeKey,
        color: scope?.color || '#666',
        nodeCount: input.instances.filter(i => i.subcategoryKey === sub.key).length,
      },
    });

    // Edge from Scope to Subcategory (HAS_SUBCATEGORY)
    edges.push({
      id: `edge-scope-${sub.scopeKey}-to-${sub.key}`,
      source: `scope-${sub.scopeKey}`,
      target: `subcat-${sub.key}`,
      type: 'floating',
      data: { relationType: 'HAS_SUBCATEGORY' },
    });
  }

  // Create instance nodes
  for (const instance of input.instances) {
    const pos = positions.get(instance.id)!;
    const sub = input.subcategories.find(s => s.key === instance.subcategoryKey);
    const scope = input.scopes.find(s => s.key === sub?.scopeKey);

    nodes.push({
      id: instance.id,
      type: 'schemaNode',  // Reuse existing node type
      position: pos,
      draggable: true,
      style: { width: NODE_WIDTH, height: NODE_HEIGHT },
      data: {
        nodeType: instance.nodeType,
        label: instance.label,
        scope: scope?.key,
        subcategory: instance.subcategoryKey,
        ...instance.properties,
      },
    });

    // Edge from instance to Subcategory (IN_SUBCATEGORY) - rendered faintly
    edges.push({
      id: `edge-${instance.id}-to-subcat-${instance.subcategoryKey}`,
      source: instance.id,
      target: `subcat-${instance.subcategoryKey}`,
      type: 'magnetic',  // New edge type (faint, dashed)
      data: { relationType: 'IN_SUBCATEGORY' },
    });
  }

  // Add business relationships between instances
  for (const rel of input.relationships) {
    // Skip IN_SUBCATEGORY (already added above)
    if (rel.type === 'IN_SUBCATEGORY') continue;

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
