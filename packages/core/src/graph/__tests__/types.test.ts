// packages/core/src/graph/__tests__/types.test.ts
import { describe, it, expect } from 'vitest';
import type { SchemaNode, SchemaEdge, Subcategory, ScopeDefinition } from '../types';

describe('graph/types', () => {
  it('should export SchemaNode interface', () => {
    const node: SchemaNode = {
      id: 'schema-Project',
      nodeType: 'Project',
      scope: 'Project',
      subcategory: 'foundation',
      label: 'Project',
      description: 'Project node',
      behavior: 'invariant',
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

  it('should export Subcategory type with all values', () => {
    const subcats: Subcategory[] = [
      'foundation', 'structure', 'semantic', 'instruction', 'output',
      'config', 'knowledge',
      'seo', 'geo'
    ];
    expect(subcats).toHaveLength(9);
  });

  it('should export ScopeDefinition interface', () => {
    const scopeDef: ScopeDefinition = {
      scope: 'Project',
      label: 'PROJECT',
      icon: '📦',
      description: 'Project-specific content and structure',
      subcategories: {} as ScopeDefinition['subcategories'],
    };
    expect(scopeDef.scope).toBe('Project');
    expect(scopeDef.label).toBe('PROJECT');
  });

  it('should allow optional properties on SchemaNode', () => {
    const node: SchemaNode = {
      id: 'schema-Locale',
      nodeType: 'Locale',
      scope: 'Global',
      subcategory: 'config',
      label: 'Locale',
      description: 'Locale configuration',
      behavior: 'invariant',
      icon: '🌍',
      color: '#10b981',
    };
    expect(node.icon).toBe('🌍');
    expect(node.color).toBe('#10b981');
  });
});
