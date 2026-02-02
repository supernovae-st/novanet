// packages/core/src/graph/__tests__/types.test.ts
// Tests for graph types — v9.0.0
import { describe, it, expect } from 'vitest';
import type { SchemaNode, SchemaEdge, RealmDefinition } from '../types';
import type { Layer } from '../../types/nodes';

describe('graph/types', () => {
  it('should export SchemaNode interface', () => {
    const node: SchemaNode = {
      id: 'schema-Project',
      nodeType: 'Project',
      realm: 'project',
      layer: 'foundation',
      label: 'Project',
      description: 'Project node',
      trait: 'invariant',
    };
    expect(node.nodeType).toBe('Project');
  });

  it('should export SchemaEdge interface', () => {
    const edge: SchemaEdge = {
      id: 'schema-edge-0',
      relationType: 'HAS_PAGE',
      sourceType: 'Project',
      targetType: 'Page',
      label: 'HAS_PAGE',
      description: 'Project contains pages',
      cardinality: '1:N',
    };
    expect(edge.relationType).toBe('HAS_PAGE');
  });

  it('should export Layer type with all values', () => {
    const layers: Layer[] = [
      'foundation', 'structure', 'semantic', 'instruction', 'output',
      'config', 'knowledge',
      'seo', 'geo'
    ];
    expect(layers).toHaveLength(9);
  });

  it('should export RealmDefinition interface', () => {
    const realmDef: RealmDefinition = {
      realm: 'project',
      label: 'PROJECT',
      icon: '📦',
      description: 'Project-specific content and structure',
      layers: {} as RealmDefinition['layers'],
    };
    expect(realmDef.realm).toBe('project');
    expect(realmDef.label).toBe('PROJECT');
  });

  it('should allow optional properties on SchemaNode', () => {
    const node: SchemaNode = {
      id: 'schema-Locale',
      nodeType: 'Locale',
      realm: 'global',
      layer: 'config',
      label: 'Locale',
      description: 'Locale configuration',
      trait: 'invariant',
      icon: '🌍',
      color: '#10b981',
    };
    expect(node.icon).toBe('🌍');
    expect(node.color).toBe('#10b981');
  });
});
