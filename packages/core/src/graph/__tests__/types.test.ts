// packages/core/src/graph/__tests__/types.test.ts
// Tests for graph types — v11.7.0 (10 layers, 60 nodes)
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
      trait: 'defined',
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
    // v11.4: 10 layers (4 shared + 6 org) — seo/geo removed
    const layers: Layer[] = [
      'config', 'locale', 'geography', 'knowledge',  // shared realm (4)
      'foundation', 'structure', 'semantic', 'instruction', 'output',  // org realm (6) — config shared
    ];
    expect(layers).toHaveLength(9);  // 9 unique (config appears in both)
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
      trait: 'defined',
      icon: '🌍',
      color: '#10b981',
    };
    expect(node.icon).toBe('🌍');
    expect(node.color).toBe('#10b981');
  });
});
