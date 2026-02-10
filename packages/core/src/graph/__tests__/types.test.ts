// packages/core/src/graph/__tests__/types.test.ts
// Tests for graph types — v11.3.0 (11 layers, 61 nodes)
import { describe, it, expect } from 'vitest';
import type { SchemaNode, SchemaArc, RealmDefinition } from '../types';
import type { Layer } from '../../types/nodes';

describe('graph/types', () => {
  it('should export SchemaNode interface', () => {
    const node: SchemaNode = {
      id: 'schema-Project',
      nodeType: 'Project',
      realm: 'org',
      layer: 'foundation',
      label: 'Project',
      description: 'Project node',
      trait: 'invariant',
    };
    expect(node.nodeType).toBe('Project');
  });

  it('should export SchemaArc interface', () => {
    const arc: SchemaArc = {
      id: 'schema-arc-0',
      relationType: 'HAS_PAGE',
      sourceType: 'Project',
      targetType: 'Page',
      label: 'HAS_PAGE',
      description: 'Project contains pages',
      cardinality: '1:N',
    };
    expect(arc.relationType).toBe('HAS_PAGE');
  });

  it('should export Layer type with all values', () => {
    // v11.3: 11 layers (3 shared + 8 org)
    const layers: Layer[] = [
      'locale', 'geography', 'knowledge',  // shared realm (3)
      'config', 'foundation', 'structure', 'semantic', 'instruction', 'seo', 'geo', 'output',  // org realm (8)
    ];
    expect(layers).toHaveLength(11);
  });

  it('should export RealmDefinition interface', () => {
    const realmDef: RealmDefinition = {
      realm: 'org',
      label: 'TENANT',
      icon: '📦',
      description: 'Tenant-specific content and structure',
      layers: {} as RealmDefinition['layers'],
    };
    expect(realmDef.realm).toBe('org');
    expect(realmDef.label).toBe('TENANT');
  });

  it('should allow optional properties on SchemaNode', () => {
    const node: SchemaNode = {
      id: 'schema-Locale',
      nodeType: 'Locale',
      realm: 'shared',
      layer: 'locale',
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
