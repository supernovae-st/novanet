// stores/schemaStore.ts
// v12: Schema Discovery Store for Unified View System
//
// Provides:
// - Database schema discovery (61 node labels, 156 relationship types)
// - Taxonomy metadata merging (colors, icons, layers from CLASS_TAXONOMY)
// - Current query counts (updated after each query execution)
// - Realm > Layer > Class hierarchy for Schema Explorer Panel

import { create } from 'zustand';
import { immer } from 'zustand/middleware/immer';
import { NODE_TYPES, CLASS_TAXONOMY, type NodeType, type Realm, type Layer, type Trait } from '@novanet/core/types';
import { LAYERS_ICONS, REALMS_ICONS, getClassIcon } from '@novanet/core/graph';
import { logger } from '@/lib/logger';

// ============================================================================
// TYPES
// ============================================================================

/**
 * Enriched node type with taxonomy metadata and current query count.
 */
export interface EnrichedNodeType {
  /** Node type name (e.g., 'Entity', 'Page') */
  name: NodeType;
  /** Realm (shared | org) */
  realm: Realm;
  /** Layer (config | locale | geography | knowledge | foundation | structure | semantic | instruction | output) */
  layer: Layer;
  /** Trait (defined | authored | imported | generated | retrieved) v11.8: ADR-024 */
  trait: Trait;
  /** Lucide icon name */
  icon: string;
  /** Layer color (hex) */
  color: string;
  /** Count in current query results (0 = exists but not in view) */
  count: number;
  /** Whether this type is included in current filter (not excluded) */
  isActive: boolean;
}

/**
 * Enriched relationship type with family metadata and current query count.
 */
export interface EnrichedRelationType {
  /** Relationship type name (e.g., 'HAS_PAGE', 'USES_ENTITY') */
  name: string;
  /** Arc family (ownership | localization | semantic | generation | mining) */
  family: string;
  /** Family color (hex) */
  color: string;
  /** Count in current query results */
  count: number;
  /** Whether this type is included in current filter */
  isActive: boolean;
}

/**
 * Layer group containing node types.
 */
export interface LayerGroup {
  /** Layer key */
  layer: Layer;
  /** Layer display name */
  displayName: string;
  /** Layer icon (Lucide name) */
  icon: string;
  /** Layer color (hex) */
  color: string;
  /** Node types in this layer */
  nodeTypes: EnrichedNodeType[];
  /** Total count across all node types in this layer */
  totalCount: number;
}

/**
 * Realm group containing layers.
 */
export interface RealmGroup {
  /** Realm key */
  realm: Realm;
  /** Realm display name */
  displayName: string;
  /** Realm icon (Lucide name) */
  icon: string;
  /** Realm color (hex) */
  color: string;
  /** Layers in this realm */
  layers: LayerGroup[];
  /** Total node type count in this realm */
  nodeTypeCount: number;
  /** Total instance count across all node types in this realm */
  totalCount: number;
}

/**
 * Database schema (raw from Neo4j).
 */
interface DbSchema {
  nodeLabels: string[];
  relationshipTypes: string[];
}

// ============================================================================
// LAYER COLORS (from taxonomy.yaml)
// ============================================================================

const LAYER_COLORS: Record<Layer, string> = {
  config: '#64748b',      // slate
  locale: '#64748b',      // slate
  geography: '#10b981',   // emerald
  knowledge: '#8b5cf6',   // violet
  foundation: '#3b82f6',  // blue
  structure: '#06b6d4',   // cyan
  semantic: '#f97316',    // orange
  instruction: '#22c55e', // green
  output: '#ec4899',      // pink
};

const LAYER_DISPLAY_NAMES: Record<Layer, string> = {
  config: 'Config',
  locale: 'Locale',
  geography: 'Geography',
  knowledge: 'Knowledge',
  foundation: 'Foundation',
  structure: 'Structure',
  semantic: 'Semantic',
  instruction: 'Instruction',
  output: 'Output',
};

const REALM_COLORS: Record<Realm, string> = {
  shared: '#2aa198',
  org: '#6c71c4',
};

const REALM_DISPLAY_NAMES: Record<Realm, string> = {
  shared: 'Shared',
  org: 'Organization',
};

// Arc family colors (from taxonomy.yaml)
const ARC_FAMILY_COLORS: Record<string, string> = {
  ownership: '#3b82f6',     // blue
  localization: '#f59e0b',  // amber
  semantic: '#8b5cf6',      // violet
  generation: '#ec4899',    // pink
  mining: '#10b981',        // emerald
};

// ============================================================================
// STORE STATE & ACTIONS
// ============================================================================

interface SchemaStoreState {
  // Database schema (fetched once at boot)
  dbSchema: DbSchema | null;
  isSchemaLoaded: boolean;
  isLoading: boolean;
  error: string | null;

  // Enriched types (merged with taxonomy)
  enrichedNodeTypes: EnrichedNodeType[];
  enrichedRelTypes: EnrichedRelationType[];

  // Hierarchical structure for Schema Explorer Panel
  realmGroups: RealmGroup[];

  // Current query counts
  nodeTypeCounts: Record<string, number>;
  relTypeCounts: Record<string, number>;

  // Filter state (excluded types)
  excludedNodeTypes: Set<string>;
  excludedRelTypes: Set<string>;
}

interface SchemaStoreActions {
  // Schema loading
  loadSchema: () => Promise<void>;

  // Count updates (called after each query)
  updateCounts: (nodeTypeCounts: Record<string, number>, relTypeCounts: Record<string, number>) => void;

  // Filter actions
  toggleNodeType: (name: string) => void;
  toggleRelType: (name: string) => void;
  setNodeTypeActive: (name: string, active: boolean) => void;
  setRelTypeActive: (name: string, active: boolean) => void;
  selectAllNodeTypes: () => void;
  selectNoNodeTypes: () => void;
  selectAllRelTypes: () => void;
  selectNoRelTypes: () => void;
  invertNodeTypeSelection: () => void;
  invertRelTypeSelection: () => void;

  // Getters
  getNodeType: (name: string) => EnrichedNodeType | undefined;
  getRelType: (name: string) => EnrichedRelationType | undefined;
  getNodeTypesByLayer: (layer: Layer) => EnrichedNodeType[];
  getNodeTypesByRealm: (realm: Realm) => EnrichedNodeType[];

  // Filter export (for query modification)
  getExcludedNodeTypes: () => string[];
  getExcludedRelTypes: () => string[];
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/**
 * Build enriched node types from CLASS_TAXONOMY with counts.
 */
function buildEnrichedNodeTypes(
  counts: Record<string, number>,
  excludedTypes: Set<string>
): EnrichedNodeType[] {
  return NODE_TYPES.map((name) => {
    const meta = CLASS_TAXONOMY[name];
    return {
      name,
      realm: meta.realm,
      layer: meta.layer,
      trait: meta.trait,
      icon: getClassIcon(name),
      color: LAYER_COLORS[meta.layer],
      count: counts[name] ?? 0,
      isActive: !excludedTypes.has(name),
    };
  });
}

/**
 * Build hierarchical realm > layer > type structure.
 */
function buildRealmGroups(enrichedTypes: EnrichedNodeType[]): RealmGroup[] {
  const realms: Realm[] = ['shared', 'org'];
  const sharedLayers: Layer[] = ['config', 'locale', 'geography', 'knowledge'];
  const orgLayers: Layer[] = ['config', 'foundation', 'structure', 'semantic', 'instruction', 'output'];

  return realms.map((realm) => {
    const layers = realm === 'shared' ? sharedLayers : orgLayers;
    const layerGroups: LayerGroup[] = layers.map((layer) => {
      const nodeTypes = enrichedTypes.filter(
        (t) => t.realm === realm && t.layer === layer
      );
      return {
        layer,
        displayName: LAYER_DISPLAY_NAMES[layer],
        icon: LAYERS_ICONS[layer]?.web ?? 'circle',
        color: LAYER_COLORS[layer],
        nodeTypes,
        totalCount: nodeTypes.reduce((sum, t) => sum + t.count, 0),
      };
    }).filter((lg) => lg.nodeTypes.length > 0); // Filter out empty layers

    const allNodeTypes = enrichedTypes.filter((t) => t.realm === realm);
    return {
      realm,
      displayName: REALM_DISPLAY_NAMES[realm],
      icon: REALMS_ICONS[realm]?.web ?? 'circle',
      color: REALM_COLORS[realm],
      layers: layerGroups,
      nodeTypeCount: allNodeTypes.length,
      totalCount: allNodeTypes.reduce((sum, t) => sum + t.count, 0),
    };
  });
}

/**
 * Determine arc family from relationship type name.
 * Heuristic based on naming conventions.
 */
function getArcFamily(relType: string): string {
  // Ownership family patterns
  if (relType.startsWith('HAS_') || relType.startsWith('BELONGS_TO') ||
      relType.endsWith('_OF') || relType.startsWith('CONTAINS_')) {
    return 'ownership';
  }
  // Localization family
  if (relType.startsWith('FOR_LOCALE') || relType.startsWith('FOR_COUNTRY') ||
      relType.includes('_LOCALE') || relType.includes('_CULTURE')) {
    return 'localization';
  }
  // Generation family
  if (relType.startsWith('HAS_INSTRUCTION') || relType.startsWith('HAS_NATIVE') ||
      relType.startsWith('COMPILED_') || relType.includes('_GENERATED')) {
    return 'generation';
  }
  // Mining family (SEO/GEO)
  if (relType.startsWith('TARGETS') || relType.startsWith('MONITORS') ||
      relType.includes('_SEO') || relType.includes('_GEO')) {
    return 'mining';
  }
  // Semantic family (default for remaining)
  if (relType.startsWith('USES_') || relType.startsWith('REFERENCES_') ||
      relType.startsWith('SEMANTIC_') || relType.startsWith('LINKS_')) {
    return 'semantic';
  }
  // Default to ownership (most common)
  return 'ownership';
}

/**
 * Build enriched relationship types with family detection.
 */
function buildEnrichedRelTypes(
  relTypes: string[],
  counts: Record<string, number>,
  excludedTypes: Set<string>
): EnrichedRelationType[] {
  return relTypes.map((name) => {
    const family = getArcFamily(name);
    return {
      name,
      family,
      color: ARC_FAMILY_COLORS[family] ?? '#64748b',
      count: counts[name] ?? 0,
      isActive: !excludedTypes.has(name),
    };
  });
}

// ============================================================================
// STORE
// ============================================================================

export const useSchemaStore = create<SchemaStoreState & SchemaStoreActions>()(
  immer((set, get) => ({
    // Initial state
    dbSchema: null,
    isSchemaLoaded: false,
    isLoading: false,
    error: null,
    enrichedNodeTypes: buildEnrichedNodeTypes({}, new Set()),
    enrichedRelTypes: [],
    realmGroups: buildRealmGroups(buildEnrichedNodeTypes({}, new Set())),
    nodeTypeCounts: {},
    relTypeCounts: {},
    excludedNodeTypes: new Set(),
    excludedRelTypes: new Set(),

    // Load schema from database
    loadSchema: async () => {
      if (get().isSchemaLoaded) {
        logger.debug('SchemaStore', 'Schema already loaded, skipping');
        return;
      }

      set({ isLoading: true, error: null });
      logger.debug('SchemaStore', 'Loading database schema...');

      try {
        // Fetch schema from API (existing endpoint)
        const res = await fetch('/api/graph/schema');
        const json = await res.json();

        if (!json.success) {
          throw new Error(json.error || 'Failed to load schema');
        }

        // Extract just the label/type names from the response
        const dbSchema: DbSchema = {
          nodeLabels: (json.data.nodeLabels ?? []).map((l: { label: string }) => l.label),
          relationshipTypes: (json.data.relationshipTypes ?? []).map((r: { type: string }) => r.type),
        };

        // Also update initial counts from schema API (total DB counts)
        const dbNodeCounts: Record<string, number> = {};
        const dbRelCounts: Record<string, number> = {};
        for (const item of json.data.nodeLabels ?? []) {
          dbNodeCounts[item.label] = item.count;
        }
        for (const item of json.data.relationshipTypes ?? []) {
          dbRelCounts[item.type] = item.count;
        }

        // Build enriched types with initial DB counts
        const enrichedNodeTypes = buildEnrichedNodeTypes(
          dbNodeCounts,
          get().excludedNodeTypes
        );
        const enrichedRelTypes = buildEnrichedRelTypes(
          dbSchema.relationshipTypes,
          dbRelCounts,
          get().excludedRelTypes
        );
        const realmGroups = buildRealmGroups(enrichedNodeTypes);

        set((state) => {
          state.dbSchema = dbSchema;
          state.enrichedNodeTypes = enrichedNodeTypes;
          state.enrichedRelTypes = enrichedRelTypes;
          state.realmGroups = realmGroups;
          state.nodeTypeCounts = dbNodeCounts;
          state.relTypeCounts = dbRelCounts;
          state.isSchemaLoaded = true;
          state.isLoading = false;
        });

        logger.info('SchemaStore', 'Schema loaded', {
          nodeLabels: dbSchema.nodeLabels.length,
          relationshipTypes: dbSchema.relationshipTypes.length,
        });
      } catch (error) {
        const message = error instanceof Error ? error.message : 'Unknown error';
        set({ error: message, isLoading: false });
        logger.error('SchemaStore', 'Failed to load schema', { error: message });
      }
    },

    // Update counts after query execution
    updateCounts: (nodeTypeCounts, relTypeCounts) => {
      set((state) => {
        state.nodeTypeCounts = nodeTypeCounts;
        state.relTypeCounts = relTypeCounts;

        // Rebuild enriched types with new counts
        state.enrichedNodeTypes = buildEnrichedNodeTypes(
          nodeTypeCounts,
          state.excludedNodeTypes
        );
        state.enrichedRelTypes = buildEnrichedRelTypes(
          state.dbSchema?.relationshipTypes ?? [],
          relTypeCounts,
          state.excludedRelTypes
        );
        state.realmGroups = buildRealmGroups(state.enrichedNodeTypes);
      });

      logger.debug('SchemaStore', 'Counts updated', {
        nodeTypes: Object.keys(nodeTypeCounts).length,
        relTypes: Object.keys(relTypeCounts).length,
      });
    },

    // Toggle node type filter
    toggleNodeType: (name) => {
      set((state) => {
        if (state.excludedNodeTypes.has(name)) {
          state.excludedNodeTypes.delete(name);
        } else {
          state.excludedNodeTypes.add(name);
        }

        // Update enriched types
        state.enrichedNodeTypes = buildEnrichedNodeTypes(
          state.nodeTypeCounts,
          state.excludedNodeTypes
        );
        state.realmGroups = buildRealmGroups(state.enrichedNodeTypes);
      });
    },

    // Toggle relationship type filter
    toggleRelType: (name) => {
      set((state) => {
        if (state.excludedRelTypes.has(name)) {
          state.excludedRelTypes.delete(name);
        } else {
          state.excludedRelTypes.add(name);
        }

        // Update enriched types
        state.enrichedRelTypes = buildEnrichedRelTypes(
          state.dbSchema?.relationshipTypes ?? [],
          state.relTypeCounts,
          state.excludedRelTypes
        );
      });
    },

    // Set node type active state explicitly
    setNodeTypeActive: (name, active) => {
      set((state) => {
        if (active) {
          state.excludedNodeTypes.delete(name);
        } else {
          state.excludedNodeTypes.add(name);
        }

        state.enrichedNodeTypes = buildEnrichedNodeTypes(
          state.nodeTypeCounts,
          state.excludedNodeTypes
        );
        state.realmGroups = buildRealmGroups(state.enrichedNodeTypes);
      });
    },

    // Set relationship type active state explicitly
    setRelTypeActive: (name, active) => {
      set((state) => {
        if (active) {
          state.excludedRelTypes.delete(name);
        } else {
          state.excludedRelTypes.add(name);
        }

        state.enrichedRelTypes = buildEnrichedRelTypes(
          state.dbSchema?.relationshipTypes ?? [],
          state.relTypeCounts,
          state.excludedRelTypes
        );
      });
    },

    // Select all node types
    selectAllNodeTypes: () => {
      set((state) => {
        state.excludedNodeTypes = new Set();
        state.enrichedNodeTypes = buildEnrichedNodeTypes(
          state.nodeTypeCounts,
          state.excludedNodeTypes
        );
        state.realmGroups = buildRealmGroups(state.enrichedNodeTypes);
      });
    },

    // Deselect all node types
    selectNoNodeTypes: () => {
      set((state) => {
        state.excludedNodeTypes = new Set(NODE_TYPES);
        state.enrichedNodeTypes = buildEnrichedNodeTypes(
          state.nodeTypeCounts,
          state.excludedNodeTypes
        );
        state.realmGroups = buildRealmGroups(state.enrichedNodeTypes);
      });
    },

    // Select all relationship types
    selectAllRelTypes: () => {
      set((state) => {
        state.excludedRelTypes = new Set();
        state.enrichedRelTypes = buildEnrichedRelTypes(
          state.dbSchema?.relationshipTypes ?? [],
          state.relTypeCounts,
          state.excludedRelTypes
        );
      });
    },

    // Deselect all relationship types
    selectNoRelTypes: () => {
      set((state) => {
        const allRelTypes = state.dbSchema?.relationshipTypes ?? [];
        state.excludedRelTypes = new Set(allRelTypes);
        state.enrichedRelTypes = buildEnrichedRelTypes(
          allRelTypes,
          state.relTypeCounts,
          state.excludedRelTypes
        );
      });
    },

    // Invert node type selection
    invertNodeTypeSelection: () => {
      set((state) => {
        const newExcluded = new Set<string>();
        for (const nodeType of NODE_TYPES) {
          if (!state.excludedNodeTypes.has(nodeType)) {
            newExcluded.add(nodeType);
          }
        }
        state.excludedNodeTypes = newExcluded;
        state.enrichedNodeTypes = buildEnrichedNodeTypes(
          state.nodeTypeCounts,
          state.excludedNodeTypes
        );
        state.realmGroups = buildRealmGroups(state.enrichedNodeTypes);
      });
    },

    // Invert relationship type selection
    invertRelTypeSelection: () => {
      set((state) => {
        const allRelTypes = state.dbSchema?.relationshipTypes ?? [];
        const newExcluded = new Set<string>();
        for (const relType of allRelTypes) {
          if (!state.excludedRelTypes.has(relType)) {
            newExcluded.add(relType);
          }
        }
        state.excludedRelTypes = newExcluded;
        state.enrichedRelTypes = buildEnrichedRelTypes(
          allRelTypes,
          state.relTypeCounts,
          state.excludedRelTypes
        );
      });
    },

    // Get enriched node type by name
    getNodeType: (name) => {
      return get().enrichedNodeTypes.find((t) => t.name === name);
    },

    // Get enriched relationship type by name
    getRelType: (name) => {
      return get().enrichedRelTypes.find((t) => t.name === name);
    },

    // Get node types by layer
    getNodeTypesByLayer: (layer) => {
      return get().enrichedNodeTypes.filter((t) => t.layer === layer);
    },

    // Get node types by realm
    getNodeTypesByRealm: (realm) => {
      return get().enrichedNodeTypes.filter((t) => t.realm === realm);
    },

    // Get excluded node types for query modification
    getExcludedNodeTypes: () => {
      return Array.from(get().excludedNodeTypes);
    },

    // Get excluded relationship types for query modification
    getExcludedRelTypes: () => {
      return Array.from(get().excludedRelTypes);
    },
  }))
);

// ============================================================================
// SELECTORS (for useShallow)
// ============================================================================

export const selectEnrichedNodeTypes = (state: SchemaStoreState) => state.enrichedNodeTypes;
export const selectEnrichedRelTypes = (state: SchemaStoreState) => state.enrichedRelTypes;
export const selectRealmGroups = (state: SchemaStoreState) => state.realmGroups;
export const selectIsSchemaLoaded = (state: SchemaStoreState) => state.isSchemaLoaded;
export const selectIsLoading = (state: SchemaStoreState) => state.isLoading;
export const selectNodeTypeCounts = (state: SchemaStoreState) => state.nodeTypeCounts;
export const selectRelTypeCounts = (state: SchemaStoreState) => state.relTypeCounts;
