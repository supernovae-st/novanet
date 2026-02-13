// packages/core/src/graph/__tests__/design-system-coherence.test.ts
// Comprehensive tests for design system coherence — v0.12.0
// Validates taxonomy, visual encoding, and TypeScript consistency
//
// Philosophy: v0 clean architecture — no backward compatibility, no legacy patterns

import { describe, it, expect } from 'vitest';
import { readFileSync } from 'fs';
import { join } from 'path';
import { parse as parseYaml } from 'yaml';

import { REALM_HIERARCHY } from '../hierarchy';
import { NODE_LAYERS, getNodeTypesByRealmAndLayer } from '../layers';
import { NODE_TYPES, CLASS_TAXONOMY } from '../../types/nodes';
import type { Realm, Layer } from '../../types/nodes';

// =============================================================================
// CONSTANTS — v11.6.0 Source of Truth
// =============================================================================

const V11_6_ARCHITECTURE = {
  // 2 realms
  realms: ['shared', 'org'] as const,

  // 10 layers total (4 shared + 6 org)
  layers: {
    shared: ['config', 'locale', 'geography', 'knowledge'] as const,
    org: ['config', 'semantic', 'foundation', 'structure', 'instruction', 'output'] as const,
  },

  // 5 traits
  traits: ['defined', 'authored', 'imported', 'generated', 'retrieved'] as const,

  // 5 arc families
  arcFamilies: ['ownership', 'localization', 'semantic', 'generation', 'mining'] as const,

  // Node counts (v0.12.0)
  nodeCounts: {
    total: 59,
    shared: 39,
    org: 20,
    byLayer: {
      // shared layers
      'shared/config': 3,
      'shared/locale': 6,
      'shared/geography': 6,
      'shared/knowledge': 24,
      // org layers
      'org/config': 1,
      'org/semantic': 4,
      'org/foundation': 3,
      'org/structure': 3,
      'org/instruction': 6,
      'org/output': 3,
    },
  },
} as const;

// =============================================================================
// Helper Functions
// =============================================================================

function loadYaml<T>(relativePath: string): T {
  const root = join(__dirname, '../../../../..');
  const content = readFileSync(join(root, relativePath), 'utf-8');
  return parseYaml(content) as T;
}

interface TaxonomyYaml {
  version: string;
  node_realms: Array<{
    key: string;
    display_name: string;
    color: string;
    layers: Array<{
      key: string;
      display_name: string;
      color: string;
    }>;
  }>;
  node_traits: Array<{
    key: string;
    display_name: string;
    color: string;
    border_style: string;
  }>;
  arc_families: Array<{
    key: string;
    display_name: string;
    color: string;
    stroke_style: string;
  }>;
}

interface VisualEncodingYaml {
  icons: {
    realms: Record<string, { web: string; terminal: string }>;
    layers: Record<string, { web: string; terminal: string }>;
    traits: Record<string, { web: string; terminal: string }>;
    arc_families: Record<string, { web: string; terminal: string }>;
  };
}

// =============================================================================
// Realm Tests
// =============================================================================

describe('Design System Coherence: Realms', () => {
  const taxonomy = loadYaml<TaxonomyYaml>('packages/core/models/taxonomy.yaml');

  it('should have exactly 2 realms', () => {
    expect(taxonomy.node_realms).toHaveLength(2);
    expect(Object.keys(REALM_HIERARCHY)).toHaveLength(2);
  });

  it('should have matching realm keys in YAML and TypeScript', () => {
    const yamlRealms = taxonomy.node_realms.map(r => r.key).sort();
    const tsRealms = Object.keys(REALM_HIERARCHY).sort();

    expect(yamlRealms).toEqual(['org', 'shared']);
    expect(tsRealms).toEqual(['org', 'shared']);
  });

  it('should have correct realm definitions', () => {
    for (const realm of V11_6_ARCHITECTURE.realms) {
      const yamlRealm = taxonomy.node_realms.find(r => r.key === realm);
      const tsRealm = REALM_HIERARCHY[realm as Realm];

      expect(yamlRealm).toBeDefined();
      expect(tsRealm).toBeDefined();
      expect(yamlRealm!.display_name).toBeTruthy();
      expect(yamlRealm!.color).toMatch(/^#[0-9a-f]{6}$/i);
    }
  });

  it('should NOT have deprecated realm names', () => {
    const yamlRealmKeys = taxonomy.node_realms.map(r => r.key);
    const deprecatedNames = ['global', 'tenant', 'project', 'organization'];

    for (const deprecated of deprecatedNames) {
      expect(yamlRealmKeys).not.toContain(deprecated);
    }
  });
});

// =============================================================================
// Layer Tests
// =============================================================================

describe('Design System Coherence: Layers', () => {
  const taxonomy = loadYaml<TaxonomyYaml>('packages/core/models/taxonomy.yaml');

  it('should have correct layer structure per realm', () => {
    const sharedRealm = taxonomy.node_realms.find(r => r.key === 'shared');
    const orgRealm = taxonomy.node_realms.find(r => r.key === 'org');

    expect(sharedRealm!.layers.map(l => l.key).sort()).toEqual(
      [...V11_6_ARCHITECTURE.layers.shared].sort()
    );
    expect(orgRealm!.layers.map(l => l.key).sort()).toEqual(
      [...V11_6_ARCHITECTURE.layers.org].sort()
    );
  });

  it('should have total of 10 layers (4 shared + 6 org)', () => {
    const sharedLayers = V11_6_ARCHITECTURE.layers.shared.length;
    const orgLayers = V11_6_ARCHITECTURE.layers.org.length;

    expect(sharedLayers).toBe(4);
    expect(orgLayers).toBe(6);
    expect(sharedLayers + orgLayers).toBe(10);
  });

  it('should NOT have deprecated layers', () => {
    const allLayers = taxonomy.node_realms.flatMap(r => r.layers.map(l => l.key));
    const deprecatedLayers = ['seo', 'geo', 'locale-knowledge'];

    for (const deprecated of deprecatedLayers) {
      expect(allLayers).not.toContain(deprecated);
    }
  });

  it('should have Locale in shared/config (not shared/locale)', () => {
    // v11.5: Locale is a DEFINITION (invariant) not a SETTING (knowledge)
    expect(NODE_LAYERS.Locale).toBe('config');
    expect(CLASS_TAXONOMY.Locale.layer).toBe('config');
    expect(CLASS_TAXONOMY.Locale.trait).toBe('defined');
  });

  it('should have SEO/GEO in shared/knowledge (not org)', () => {
    // v11.5: SEO/GEO moved from org to shared/knowledge (Knowledge Atoms pattern)
    const seoGeoNodes = ['SEOKeyword', 'SEOKeywordMetrics', 'SEOKeywordSet', 'GEOQuery', 'GEOQuerySet', 'GEOAnswer'];

    for (const node of seoGeoNodes) {
      expect(NODE_LAYERS[node as keyof typeof NODE_LAYERS]).toBe('knowledge');
      expect(CLASS_TAXONOMY[node as keyof typeof CLASS_TAXONOMY].realm).toBe('shared');
    }
  });
});

// =============================================================================
// Trait Tests
// =============================================================================

describe('Design System Coherence: Traits', () => {
  const taxonomy = loadYaml<TaxonomyYaml>('packages/core/models/taxonomy.yaml');

  it('should have exactly 5 traits', () => {
    expect(taxonomy.node_traits).toHaveLength(5);
  });

  it('should have correct trait definitions', () => {
    const yamlTraits = taxonomy.node_traits.map(t => t.key).sort();
    const expectedTraits = [...V11_6_ARCHITECTURE.traits].sort();

    expect(yamlTraits).toEqual(expectedTraits);
  });

  it('should NOT have deprecated traits', () => {
    const yamlTraitKeys = taxonomy.node_traits.map(t => t.key);
    // v0.12.0: old names (invariant, localized, knowledge, aggregated) are deprecated
    const deprecatedTraits = ['derived', 'job', 'invariant', 'localized', 'knowledge', 'aggregated'];

    for (const deprecated of deprecatedTraits) {
      expect(yamlTraitKeys).not.toContain(deprecated);
    }
  });

  it('should have correct border styles for each trait', () => {
    const expectedBorders: Record<string, string> = {
      defined: 'solid',
      authored: 'dashed',
      imported: 'dotted',
      generated: 'double',
      retrieved: 'dotted',
    };

    for (const trait of taxonomy.node_traits) {
      expect(trait.border_style).toBe(expectedBorders[trait.key]);
    }
  });

  it('should use generated trait for LLM output nodes', () => {
    const generatedNodes = ['PageGenerated', 'BlockGenerated', 'OutputArtifact', 'PromptArtifact'];

    for (const node of generatedNodes) {
      expect(CLASS_TAXONOMY[node as keyof typeof CLASS_TAXONOMY].trait).toBe('generated');
    }
  });

  it('should use aggregated trait for metrics nodes', () => {
    const aggregatedNodes = ['SEOKeywordMetrics', 'GEOAnswer'];

    for (const node of aggregatedNodes) {
      expect(CLASS_TAXONOMY[node as keyof typeof CLASS_TAXONOMY].trait).toBe('retrieved');
    }
  });
});

// =============================================================================
// Arc Family Tests
// =============================================================================

describe('Design System Coherence: Arc Families', () => {
  const taxonomy = loadYaml<TaxonomyYaml>('packages/core/models/taxonomy.yaml');

  it('should have exactly 5 arc families', () => {
    expect(taxonomy.arc_families).toHaveLength(5);
  });

  it('should have correct arc family definitions', () => {
    const yamlFamilies = taxonomy.arc_families.map(f => f.key).sort();
    const expectedFamilies = [...V11_6_ARCHITECTURE.arcFamilies].sort();

    expect(yamlFamilies).toEqual(expectedFamilies);
  });

  it('should have valid colors for all arc families', () => {
    for (const family of taxonomy.arc_families) {
      expect(family.color).toMatch(/^#[0-9a-f]{6}$/i);
    }
  });

  it('should have correct stroke styles for arc families', () => {
    const expectedStrokes: Record<string, string> = {
      ownership: 'solid',
      localization: 'dashed',
      semantic: 'dotted',
      generation: 'solid',
      mining: 'dashed',
    };

    for (const family of taxonomy.arc_families) {
      expect(family.stroke_style).toBe(expectedStrokes[family.key]);
    }
  });
});

// =============================================================================
// Node Count Tests
// =============================================================================

describe('Design System Coherence: Node Counts', () => {
  it('should have exactly 59 total nodes', () => {
    expect(NODE_TYPES).toHaveLength(59);
    expect(Object.keys(NODE_LAYERS)).toHaveLength(59);
    expect(Object.keys(CLASS_TAXONOMY)).toHaveLength(59);
  });

  it('should have correct node distribution by realm', () => {
    const sharedNodes = NODE_TYPES.filter(t => CLASS_TAXONOMY[t].realm === 'shared');
    const orgNodes = NODE_TYPES.filter(t => CLASS_TAXONOMY[t].realm === 'org');

    expect(sharedNodes).toHaveLength(V11_6_ARCHITECTURE.nodeCounts.shared);
    expect(orgNodes).toHaveLength(V11_6_ARCHITECTURE.nodeCounts.org);
  });

  it('should have correct node distribution by layer', () => {
    for (const [layerKey, expectedCount] of Object.entries(V11_6_ARCHITECTURE.nodeCounts.byLayer)) {
      const [realm, layer] = layerKey.split('/') as [Realm, Layer];
      const nodes = getNodeTypesByRealmAndLayer(realm, layer);

      expect(nodes).toHaveLength(expectedCount);
    }
  });

  it('should have no duplicate node names', () => {
    const nodeSet = new Set(NODE_TYPES);
    expect(nodeSet.size).toBe(NODE_TYPES.length);
  });

  it('should have consistent node count across all sources', () => {
    const nodeTypesCount = NODE_TYPES.length;
    const nodeLayersCount = Object.keys(NODE_LAYERS).length;
    const classificationCount = Object.keys(CLASS_TAXONOMY).length;

    expect(nodeTypesCount).toBe(nodeLayersCount);
    expect(nodeLayersCount).toBe(classificationCount);
  });
});

// =============================================================================
// Visual Encoding Tests
// =============================================================================

describe('Design System Coherence: Visual Encoding', () => {
  const visualEncoding = loadYaml<VisualEncodingYaml>('packages/core/models/visual-encoding.yaml');

  it('should have icons for all realms', () => {
    for (const realm of V11_6_ARCHITECTURE.realms) {
      expect(visualEncoding.icons.realms[realm]).toBeDefined();
      expect(visualEncoding.icons.realms[realm].web).toBeTruthy();
      expect(visualEncoding.icons.realms[realm].terminal).toBeTruthy();
    }
  });

  it('should have icons for all layers', () => {
    const allLayers = [
      ...V11_6_ARCHITECTURE.layers.shared,
      ...V11_6_ARCHITECTURE.layers.org.filter(l => l !== 'config'), // config already in shared
    ];

    for (const layer of allLayers) {
      expect(visualEncoding.icons.layers[layer]).toBeDefined();
      expect(visualEncoding.icons.layers[layer].web).toBeTruthy();
      expect(visualEncoding.icons.layers[layer].terminal).toBeTruthy();
    }
  });

  it('should have icons for all traits', () => {
    for (const trait of V11_6_ARCHITECTURE.traits) {
      expect(visualEncoding.icons.traits[trait]).toBeDefined();
      expect(visualEncoding.icons.traits[trait].web).toBeTruthy();
      expect(visualEncoding.icons.traits[trait].terminal).toBeTruthy();
    }
  });

  it('should have icons for all arc families', () => {
    for (const family of V11_6_ARCHITECTURE.arcFamilies) {
      expect(visualEncoding.icons.arc_families[family]).toBeDefined();
      expect(visualEncoding.icons.arc_families[family].web).toBeTruthy();
      expect(visualEncoding.icons.arc_families[family].terminal).toBeTruthy();
    }
  });

  it('should NOT have deprecated icons', () => {
    const deprecatedKeys = ['global', 'tenant', 'derived', 'job', 'seo', 'geo', 'locale-knowledge'];

    for (const deprecated of deprecatedKeys) {
      expect(visualEncoding.icons.realms[deprecated]).toBeUndefined();
      expect(visualEncoding.icons.layers[deprecated]).toBeUndefined();
      expect(visualEncoding.icons.traits[deprecated]).toBeUndefined();
    }
  });
});

// =============================================================================
// Deprecated Terms Tests
// =============================================================================

describe('Design System Coherence: No Deprecated Terms', () => {
  it('should NOT use deprecated realm names in NODE_TYPES', () => {
    const nodeNames = NODE_TYPES.join(' ');

    // Deprecated realm terminology
    expect(nodeNames).not.toContain('Global');
    expect(nodeNames).not.toContain('Tenant');
    expect(nodeNames).not.toContain('Organization'); // Use OrgConfig
  });

  it('should NOT use deprecated L10n suffix', () => {
    const nodeNames = NODE_TYPES.join(' ');

    // v10.9: L10n suffix deprecated
    expect(nodeNames).not.toContain('L10n');
    expect(NODE_TYPES).not.toContain('EntityL10n');
    expect(NODE_TYPES).not.toContain('PageL10n');
    expect(NODE_TYPES).not.toContain('BlockL10n');
    expect(NODE_TYPES).not.toContain('ProjectL10n');
  });

  it('should use Content suffix for localized semantic nodes', () => {
    expect(NODE_TYPES).toContain('EntityContent');
    expect(NODE_TYPES).toContain('ProjectContent');
  });

  it('should use Generated suffix for output nodes', () => {
    expect(NODE_TYPES).toContain('PageGenerated');
    expect(NODE_TYPES).toContain('BlockGenerated');
  });

  it('should NOT have job-related nodes', () => {
    // v11.2: Job trait and nodes removed
    const jobNodes = ['GenerationJob', 'SEOMiningRun', 'EvaluationSignal'];

    for (const jobNode of jobNodes) {
      expect(NODE_TYPES).not.toContain(jobNode);
    }
  });
});

// =============================================================================
// CLASS_TAXONOMY Consistency Tests
// =============================================================================

describe('Design System Coherence: CLASS_TAXONOMY Consistency', () => {
  it('should have valid realm for all nodes', () => {
    for (const nodeType of NODE_TYPES) {
      const meta = CLASS_TAXONOMY[nodeType];
      expect(V11_6_ARCHITECTURE.realms).toContain(meta.realm);
    }
  });

  it('should have valid layer for all nodes', () => {
    const allLayers = [
      ...V11_6_ARCHITECTURE.layers.shared,
      ...V11_6_ARCHITECTURE.layers.org,
    ];

    for (const nodeType of NODE_TYPES) {
      const meta = CLASS_TAXONOMY[nodeType];
      expect(allLayers).toContain(meta.layer);
    }
  });

  it('should have valid trait for all nodes', () => {
    for (const nodeType of NODE_TYPES) {
      const meta = CLASS_TAXONOMY[nodeType];
      expect(V11_6_ARCHITECTURE.traits).toContain(meta.trait);
    }
  });

  it('should have consistent layer between NODE_LAYERS and CLASS_TAXONOMY', () => {
    for (const nodeType of NODE_TYPES) {
      expect(NODE_LAYERS[nodeType]).toBe(CLASS_TAXONOMY[nodeType].layer);
    }
  });
});
