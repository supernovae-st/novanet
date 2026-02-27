'use client';

 

/**
 * Graph2D Component - Beautiful Knowledge Graph Visualization
 *
 * Features:
 * - Turbo-style glowing animated nodes
 * - Gradient animated edges
 * - Dagre hierarchical layout
 * - Smooth pan/zoom interactions
 * - Layer-based color coding
 */

import { useCallback, useMemo, useEffect, useState, useRef, memo } from 'react';
import {
  ReactFlow,
  Background,
  MiniMap,
  useNodesState,
  useEdgesState,
  useReactFlow,
  applyNodeChanges,
  type NodeMouseHandler,
  type EdgeMouseHandler,
  type Node as ReactFlowNode,
  type Edge as ReactFlowEdge,
  type NodeChange,
  ConnectionMode,
  BackgroundVariant,
  ReactFlowProvider,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';

import { useShallow } from 'zustand/react/shallow';
import { cn } from '@/lib/utils';
import { logger } from '@/lib/logger';
import { glassClasses, gapTokens } from '@/design/tokens';
import { GraphEmptyState } from './GraphEmptyState';
import { useFilteredGraph, useFocusMode, useHoverHighlight, useNodeExpansion, useCenterOnNode, useSmartFitView, useContainerConstraint, useGraphInteractions, Z_INDEX, useMagneticData } from '@/hooks';
import { useUIStore } from '@/stores/uiStore';
import { useAnimationStore } from '@/stores/animationStore';
import { useGraphStore } from '@/stores/graphStore';
import { NODE_TYPE_CONFIG, nodeTypeConfigs } from '@/config/nodeTypes';
import { MINIMAP_HEIGHT, TOOLBAR_BOTTOM_OFFSET } from '@/config/layoutConstants';
import {
  DAGRE_CONFIG,
  FORCE_CONFIG,
  RADIAL_CONFIG,
  COMBINED_LAYOUT_CONFIG,
  INITIAL_POSITION_CONFIG,
  GRAPH_ANIMATION,
} from '@/config/layoutConfig';
import { applyDagreLayout } from '@/lib/layout';
import { createForceSimulation, runSimulationSync, applyForcePositions } from '@/lib/forceSimulation';
import { extractLocaleFromKey } from '@/lib/localeUtils';
import {
  REALM_COLORS,
  REALM_DISPLAY_NAMES,
  LAYER_COLORS,
  LAYER_DISPLAY_NAMES,
  type RealmKey,
  type LayerKey,
} from '@/design/colors/generated';
import {
  TurboNode,
  StructuralNode,
  SharedLayerNode,
  LocaleNode,
  ClassNode,
  RealmNode,
  ProjectNode,
  RealmAttractorNode,
  LayerAttractorNode,
  type TurboNodeData,
  type TurboNodeType,
} from './nodes';
import { FloatingEdge, MagneticEdge, type FloatingEdgeType, EdgeVisibilityProvider, useEdgeVisibilityStore, useParallelEdges, getEdgeIndexInGroup } from './edges';
import { NodeContextMenu } from './NodeContextMenu';
import { GraphToolbar } from './GraphToolbar';
import type { GraphNode as GraphNodeType, GraphEdge as GraphEdgeType } from '@/types';

// Schema mode imports (Task 3.2)
import { SchemaNode, SchemaBadgeNode } from './schema';
import { SchemaErrorBoundary } from './SchemaErrorBoundary';
import { applySchemaLayout } from '@/lib/schemaLayoutELK';
import { getSchemaHierarchy } from '@novanet/core/graph';
// v9.5: useFilterStore and Realm type removed - no longer filtering by collapsed state

// =============================================================================
// Node & Edge Types - MUST be defined outside component for performance
// =============================================================================

const nodeTypes = {
  // Data mode node types
  turbo: TurboNode,
  structural: StructuralNode,
  sharedLayer: SharedLayerNode,
  locale: LocaleNode,  // v0.13.0: "Passport Élégant" design for Locale nodes
  classNode: ClassNode,  // v0.13.0: "Holographic Blueprint" design for Class nodes
  realmNode: RealmNode,  // v0.13.1: "Orbital Gateway" design for Realm nodes (shared, org)
  project: ProjectNode,
  // Schema mode node types (Task 3.2)
  schemaNode: SchemaNode,
  // v11.8: Compact badge for Realm & Layer in schema view
  schemaBadge: SchemaBadgeNode,
  // Magnetic grouping attractor nodes (Task 10)
  realmAttractor: RealmAttractorNode,
  layerAttractor: LayerAttractorNode,
} as const;

const edgeTypes = {
  floating: FloatingEdge,
  // Magnetic grouping edge type (Task 10)
  magnetic: MagneticEdge,
} as const;

// =============================================================================
// Props
// =============================================================================

export interface Graph2DProps {
  /** Additional class names */
  className?: string;
  /** Show minimap */
  showMinimap?: boolean;
  /** Show controls */
  showControls?: boolean;
  /** Callback when a node is clicked */
  onNodeClick?: (nodeId: string) => void;
  /** Callback when a node is double-clicked (expand) */
  onNodeDoubleClick?: (nodeId: string) => void;
  /** Callback when background is clicked */
  onPaneClick?: () => void;
}

// =============================================================================
// Transform Functions
// =============================================================================

/**
 * Transform GraphNode to TurboNode
 * Selects node type based on category for differentiated design
 */
function toTurboNode(node: GraphNodeType): TurboNodeType {
  const config = NODE_TYPE_CONFIG[node.type] || NODE_TYPE_CONFIG.Project;

  // Select node type based on layer (v9)
  let nodeType: string = 'turbo';

  // Cast to string once for schema meta-type comparisons
  // (Realm/Layer/NodeClass/ArcClass are not in the NodeType enum)
  const nodeTypeStr = node.type as string;

  // Special case: Project nodes get premium social network card style
  if (nodeTypeStr === 'Project') {
    nodeType = 'project';
  // v0.13.0: Locale nodes get "Passport Élégant" design
  } else if (nodeTypeStr === 'Locale') {
    nodeType = 'locale';
  // v0.13.0: Realm gets "Orbital Gateway" design, Layer gets compact badge
  } else if (nodeTypeStr === 'Realm') {
    nodeType = 'realmNode';
  } else if (nodeTypeStr === 'Layer') {
    nodeType = 'schemaBadge';
  // v0.13.0: NodeClass and ArcClass get "Holographic Blueprint" design
  } else if (['NodeClass', 'ArcClass', 'ArcFamily'].includes(nodeTypeStr)) {
    nodeType = 'classNode';
  // v0.13.1: shared/config nodes get SharedLayerNode (not StructuralNode)
  // EntityCategory width=420px, SEOKeywordFormat width=175px defined in SharedLayerNode
  } else if (['EntityCategory', 'SEOKeywordFormat'].includes(nodeTypeStr)) {
    nodeType = 'sharedLayer';
  } else {
    switch (config.layer) {
      case 'foundation':
      case 'structure':
      case 'semantic':
      case 'instruction':
      case 'output':
      case 'config':
        nodeType = 'structural';
        break;
      case 'locale':
      case 'geography':
      case 'knowledge':
        nodeType = 'sharedLayer';
        break;
      // Default to turbo for seo, geo layers
    }
  }

  // Extract locale from composite key pattern: {type}:{key}@{locale}
  // Applies to: *Native nodes, Knowledge atoms, Locale layer nodes
  const locale = extractLocaleFromKey(node.key);

  // v0.13.0: Build data object based on node type
  // SchemaBadgeNode (Realm/Layer) requires specific fields: metaType, label, color, realmKey/layerKey
  let data: TurboNodeData;

  // Use nodeTypeStr (already cast above) for schema meta-type comparisons
  if (nodeTypeStr === 'Realm') {
    // Realm nodes: metaType='realm', color from REALM_COLORS
    const realmKey = node.key as RealmKey;
    data = {
      id: node.id,
      type: node.type,
      key: node.key,
      displayName: node.displayName,
      // SchemaBadgeNode specific fields
      label: node.displayName || REALM_DISPLAY_NAMES[realmKey] || node.key,
      description: node.description || `${REALM_DISPLAY_NAMES[realmKey] || node.key} realm`,
      metaType: 'realm',
      color: REALM_COLORS[realmKey]?.color ?? '#2aa198',
      realmKey,
    };
  } else if (nodeTypeStr === 'Layer') {
    // Layer nodes: metaType='layer', color from LAYER_COLORS
    const layerKey = node.key as LayerKey;
    data = {
      id: node.id,
      type: node.type,
      key: node.key,
      displayName: node.displayName,
      // SchemaBadgeNode specific fields
      label: node.displayName || LAYER_DISPLAY_NAMES[layerKey] || node.key,
      description: node.description || `${LAYER_DISPLAY_NAMES[layerKey] || node.key} layer`,
      metaType: 'layer',
      color: LAYER_COLORS[layerKey]?.color ?? '#64748b',
      layerKey,
    };
  } else {
    // Default data structure for all other node types
    // Spread node.data to include Neo4j properties like entity_key, locale_key, etc.
    data = {
      id: node.id,
      type: node.type,
      key: node.key,
      displayName: node.displayName,
      icon: nodeTypeConfigs[node.type]?.icon,
      description: node.description,
      category: config.layer,
      locale,
      // Include all other Neo4j properties (entity_key, locale_key, benefits, etc.)
      ...(node.data || {}),
    };
  }

  return {
    id: node.id,
    type: nodeType,
    position: { x: 0, y: 0 }, // Will be set by dagre layout
    data,
  };
}

/**
 * Transform GraphEdge to FloatingEdge
 * Uses floating edge type for knowledge-graph style connections from any direction
 */
function toFloatingEdge(edge: GraphEdgeType): FloatingEdgeType {
  return {
    id: edge.id,
    source: edge.source,
    target: edge.target,
    type: 'floating',
    data: {
      relationType: edge.type,
      animated: true,
    },
  };
}

// =============================================================================
// Component
// =============================================================================

function Graph2DInner({
  className,
  showMinimap = true,
  showControls = true,
  onNodeClick,
  onNodeDoubleClick,
  onPaneClick,
}: Graph2DProps) {
  // Get filtered graph data
  const { nodes: graphNodes, edges: graphEdges } = useFilteredGraph();

  // UI store - useShallow prevents re-renders when unchanged
  // NOTE: hoveredEdgeId is NOT subscribed here - FloatingEdge subscribes directly
  // to avoid React Flow's broken edge data update mechanism
  const {
    minimapVisible,
    showEdgeLabels,
    layoutDirection,
    layoutVersion,
    setSelectedNode,
    setSelectedEdge,
    setHoveredNode,
    setHoveredEdge,
    setHoveredConnections,
    selectedNodeId,
    selectedEdgeId,
    clearSelection,
    sidebarOpen,
    focusMode,
  } = useUIStore(
    useShallow((state) => ({
      minimapVisible: state.minimapVisible,
      showEdgeLabels: state.showEdgeLabels,
      layoutDirection: state.layoutDirection,
      layoutVersion: state.layoutVersion,
      setSelectedNode: state.setSelectedNode,
      setSelectedEdge: state.setSelectedEdge,
      setHoveredNode: state.setHoveredNode,
      setHoveredEdge: state.setHoveredEdge,
      setHoveredConnections: state.setHoveredConnections,
      selectedNodeId: state.selectedNodeId,
      selectedEdgeId: state.selectedEdgeId,
      clearSelection: state.clearSelection,
      sidebarOpen: state.sidebarOpen,
      focusMode: state.focusMode,
      layoutMode: state.layoutMode, // Magnetic grouping toggle
    }))
  );

  // v12: Detect schema mode from loaded nodes (node IDs have 'schema-' prefix)
  const isSchemaMode = useMemo(() => {
    if (graphNodes.length === 0) return false;
    const schemaNodeCount = graphNodes.filter(n => n.id.startsWith('schema-')).length;
    return schemaNodeCount > 0 && schemaNodeCount === graphNodes.length;
  }, [graphNodes]);

  // Filter store - v9.5: collapsed state no longer needed (pure graph nodes)
  // Kept for potential future use with meta-node filtering

  // Animation store - Matrix transition state
  const transitionPhase = useAnimationStore((state) => state.transitionPhase);

  // Compute graph opacity based on transition phase (dissolve = fade out, reform = fade in)
  const graphOpacity = transitionPhase === 'dissolve' || transitionPhase === 'fetch' ? 0 : 1;

  // =========================================================================
  // MAGNETIC LAYOUT DATA (organizing principles)
  // =========================================================================
  // When layoutMode is 'magnetic', fetch Realm/Layer as attractor nodes
  const { data: magneticData, isMagneticMode } = useMagneticData();

  // Focus mode for selection-based dimming (supports both node and edge selection)
  const {
    isNodeDimmed,
    isEdgeDimmed,
    selectedId: focusSelectedId,
    selectedEdgeId: focusSelectedEdgeId,
    hoveredEdgeId: focusHoveredEdgeId,
    connectedIds,
  } = useFocusMode(graphEdges);

  // Hover highlight for connection-based dimming (lighter than focus mode)
  // NOTE: Edge hover state (isEdgeHoverDimmed) is now computed locally in FloatingEdge
  // via direct store subscription, bypassing React Flow's broken edge data updates
  const { isNodeHoverDimmed, hoveredId, connectedIds: hoverConnectedIds } = useHoverHighlight(graphEdges);

  // Sync hover-connected node IDs to store for direct subscription in node components
  // This bypasses React Flow's broken data update mechanism (same pattern as FloatingEdge)
  useEffect(() => {
    setHoveredConnections(hoverConnectedIds);
  }, [hoverConnectedIds, setHoveredConnections]);

  // Node expansion (double-click to expand neighbors)
  const { expandNode } = useNodeExpansion();

  // React Flow instance for programmatic control (centering, zooming, fitting)
  const { setCenter, getZoom, getNodes, getInternalNode, fitView } = useReactFlow();

  // Center on node with offset compensation for UI panels
  const { centerOnNode } = useCenterOnNode();

  // Smart fitView with dynamic insets for UI state changes
  const { smartFitView } = useSmartFitView();

  // Graph store for hide action
  const hideNode = useGraphStore((state) => state.hideNode);

  // Track current index for Tab cycling through connected nodes
  const cycleIndexRef = useRef(0);

  // Track applied layout version to detect real layout changes vs data-only updates
  // This prevents position overwrites during drag when only dimming state changes
  const appliedLayoutVersionRef = useRef(layoutVersion);

  // ==========================================================================
  // Refs for keyboard handler - "useLatestRef" pattern
  // Updated synchronously during render to always have latest values.
  // This avoids re-attaching keyboard listener when these values change,
  // preventing memory leaks from frequent event listener re-registration.
  // The keyboard handler reads from ref.current, not the closure value.
  // ==========================================================================
  const connectedIdsRef = useRef(connectedIds);
  connectedIdsRef.current = connectedIds;

  // Refs for values used in keyboard handler - prevents memory leaks
  // These are updated after state definitions below
  const nodesRef = useRef<TurboNodeType[]>([]);
  const getZoomRef = useRef(getZoom);
  const setCenterRef = useRef(setCenter);

  // Refs to track previous UI state for auto-fitView on changes
  const prevSidebarOpenRef = useRef(sidebarOpen);
  const prevFocusModeRef = useRef(focusMode);
  // Track selection state to fitView when aside panel closes (selection cleared)
  const prevHasSelectionRef = useRef<boolean>(false);
  const prevLayoutVersionRef = useRef(layoutVersion);

  // Track magnetic mode state for fitView on data arrival / mode exit
  const prevMagneticDataRef = useRef(magneticData);

  // Track if initial fitView has been performed
  const initialFitDoneRef = useRef(false);

  // PERF: Ref for smartFitView to avoid effect re-registration
  // Effects use ref.current to always get latest callback without re-running
  const smartFitViewRef = useRef(smartFitView);
  smartFitViewRef.current = smartFitView;

  // Context menu state (right-click on nodes)
  const [contextMenu, setContextMenu] = useState<{
    nodeId: string;
    position: { x: number; y: number };
  } | null>(null);

  // =========================================================================
  // EDGE BUNDLING STATE (v11.6.1)
  // =========================================================================
  // Tracks which bundled edge groups (4+ parallel edges) are expanded
  const [expandedBundles, setExpandedBundles] = useState<Set<string>>(new Set());

  // Callback for bundle hover
  const _handleBundleHover = useCallback((bundleKey: string, expanded: boolean) => {
    setExpandedBundles((prev) => {
      const next = new Set(prev);
      if (expanded) {
        next.add(bundleKey);
      } else {
        next.delete(bundleKey);
      }
      return next;
    });
  }, []);

  // =========================================================================
  // SCHEMA MODE STATE (Task 3.2)
  // =========================================================================
  // v12: Schema mode is detected from loaded nodes (schema- prefix), not
  // from navigation mode. Uses ELK layout for hierarchical grouped visualization.
  // =========================================================================
  const [schemaNodes, setSchemaNodes] = useState<ReactFlowNode[]>([]);
  const [schemaArcs, setSchemaEdges] = useState<ReactFlowEdge[]>([]);
  const [isSchemaLayouting, setIsSchemaLayouting] = useState(false);
  const [, setSchemaLayoutError] = useState<Error | null>(null);

  // PERF: Ref for schemaNodes to avoid callback re-creation during drag
  // Callbacks use ref.current to always get latest nodes without re-running
  const schemaNodesRef = useRef(schemaNodes);
  schemaNodesRef.current = schemaNodes;

  // Z-index management for schema mode (must be before handleSchemaNodeClick)
  const {
    bringToFront: bringSchemaNodeToFront,
    setHoverZIndex: setSchemaHoverZIndex,
    resetZIndex: resetSchemaZIndex,
    bringEdgeNodesToFront: bringSchemaArcNodesToFront,
  } = useGraphInteractions({ setNodes: setSchemaNodes });

  // Load schema graph with hierarchical layout (v9.5)
  // Generates pure graph nodes (Realm, Layer, Class) with edges
  // Responds to layoutDirection and layoutVersion changes (like data mode)
  const loadSchemaGraph = useCallback(async () => {
    setIsSchemaLayouting(true);
    setSchemaLayoutError(null);

    try {
      const hierarchy = getSchemaHierarchy();
      const { nodes: layoutedNodes, edges: layoutedEdges } = await applySchemaLayout(hierarchy, layoutDirection);

      // v9.5: No container filtering needed - all nodes are regular graph nodes
      // Apply z-index based on schema type (Realm < Layer < Class)
      const nodesWithZIndex = layoutedNodes.map((node) => {
        let zIndex: number = Z_INDEX.BASE;

        // Schema grouping nodes get lower z-index so Class nodes appear on top
        const metaType = node.data?.metaType as string | undefined;
        if (metaType === 'realm') {
          zIndex = Z_INDEX.REALM_ORG; // Realms at base level
        } else if (metaType === 'layer') {
          zIndex = Z_INDEX.LAYER_ORG; // Layers above realms
        }
        // Class nodes (metaType === 'class' or undefined) use BASE (highest)

        return { ...node, zIndex };
      });

      setSchemaNodes(nodesWithZIndex);
      setSchemaEdges(layoutedEdges);
    } catch (error) {
      console.error('[Graph2D] Schema layout failed:', error);
      setSchemaLayoutError(error as Error);
    } finally {
      setIsSchemaLayouting(false);
    }
  }, [layoutDirection]);

  // v12: Load schema graph when isSchemaMode is detected from loaded nodes
  // (schema nodes have 'schema-' prefix in their IDs)
  useEffect(() => {
    if (isSchemaMode) {
      loadSchemaGraph();
    }
  }, [isSchemaMode, loadSchemaGraph, layoutVersion]);

  // =========================================================================
  // SCHEMA NODE DRAG HANDLERS (Task 1 & 3: Schema Node Dragging + Constraints)
  // =========================================================================
  // Handle schema node position changes during drag operations.
  // Uses applyNodeChanges from React Flow for smooth position updates.
  // Container constraint hook manages dynamic container resizing.
  // =========================================================================
  const handleSchemaNodesChange = useCallback(
    (changes: NodeChange<ReactFlowNode>[]) => {
      setSchemaNodes((nds) => applyNodeChanges(changes, nds));
    },
     
    []
  );

  // Container constraint hook for dynamic container resizing (Task 3)
  const { handleNodeDrag: containerHandleNodeDrag, handleNodeDragStop: containerHandleNodeDragStop } =
    useContainerConstraint();

  // Schema node drag handler - calls container constraint hook
  // PERF: Uses schemaNodesRef to avoid re-creating callback on every node move
  const handleSchemaNodeDrag = useCallback(
    (_event: React.MouseEvent, node: ReactFlowNode) => {
      containerHandleNodeDrag(node, schemaNodesRef.current, setSchemaNodes);
    },
     
    [containerHandleNodeDrag]
  );

  // Schema node drag stop handler - triggers container shrinking
  // PERF: Uses schemaNodesRef to avoid re-creating callback on every node move
  const handleSchemaNodeDragStop = useCallback(
    (_event: React.MouseEvent, node: ReactFlowNode) => {
      containerHandleNodeDragStop(node, schemaNodesRef.current, setSchemaNodes);
    },
     
    [containerHandleNodeDragStop]
  );

  // =========================================================================
  // SCHEMA CLICK HANDLERS (Task 4: Click Interactions for Schema Details)
  // =========================================================================
  // Clicking schema nodes/edges shows their details in the right panel.
  // Uses uiStore selectedNodeId/selectedEdgeId to drive the panel.
  // =========================================================================
  const handleSchemaNodeClick = useCallback(
    (_event: React.MouseEvent, node: ReactFlowNode) => {
      // Set selected node for schema details panel
      // Note: Schema nodes have different data structure than data nodes
      setSelectedNode(node.id);

      // Update selected state on all schema nodes
      // Required because we use useState instead of useNodesState
      setSchemaNodes((nodes) =>
        nodes.map((n) => ({
          ...n,
          selected: n.id === node.id,
        }))
      );

      // Bring clicked node to front (z-index)
      bringSchemaNodeToFront(node.id);

      // Check if this is a container node (realm or layer group)
      const isContainer = node.id.startsWith('realm-') || node.id.startsWith('layer-');

      // Small delay to ensure state is fully propagated before view adjustment
      // This ensures the panel width is accounted for in the calculation
      setTimeout(() => {
        if (isContainer) {
          // CONTAINER: FitView to show the entire container and its children
          // Get all nodes that are children of this container
          const allNodes = getNodes();
          const containerAndChildren = allNodes.filter(
            n => n.id === node.id || n.parentId === node.id
          );

          // If this is a realm container, also include layer children
          if (node.id.startsWith('realm-')) {
            const layerNodes = allNodes.filter(n => n.parentId === node.id);
            for (const layerNode of layerNodes) {
              const layerChildren = allNodes.filter(n => n.parentId === layerNode.id);
              containerAndChildren.push(...layerChildren);
            }
          }

          // FitView to the container and all its children with padding
          fitView({
            nodes: containerAndChildren,
            duration: GRAPH_ANIMATION.FIT_VIEW_DURATION,
            padding: 0.15,
            maxZoom: 1.5,
            minZoom: 0.1,
          });
        } else {
          // LEAF NODE: Center on node (same behavior as data mode)
          const nodeWidth = node.measured?.width ?? 180;
          const nodeHeight = node.measured?.height ?? 90;

          // Use getInternalNode for reliable access to positionAbsolute
          // This is the official React Flow API for nested/grouped nodes
          const internalNode = getInternalNode(node.id);

          if (!internalNode) {
            // Fallback: use original node if not found in React Flow
            centerOnNode(
              node.position.x,
              node.position.y,
              nodeWidth,
              nodeHeight,
              { zoom: GRAPH_ANIMATION.NODE_DOUBLE_CLICK_ZOOM, duration: GRAPH_ANIMATION.FIT_VIEW_DURATION }
            );
            return;
          }

          // Get measured dimensions from internal node
          const finalWidth = internalNode.measured?.width ?? nodeWidth;
          const finalHeight = internalNode.measured?.height ?? nodeHeight;

          // CRITICAL: Use positionAbsolute for nested nodes (inside group containers)
          // node.position is relative to parent, positionAbsolute is canvas-absolute
          // Defensive: internals/positionAbsolute may be undefined during initialization
          const finalX = internalNode.internals?.positionAbsolute?.x ?? node.position.x;
          const finalY = internalNode.internals?.positionAbsolute?.y ?? node.position.y;

          centerOnNode(
            finalX,
            finalY,
            finalWidth,
            finalHeight,
            { zoom: GRAPH_ANIMATION.NODE_DOUBLE_CLICK_ZOOM, duration: GRAPH_ANIMATION.FIT_VIEW_DURATION }
          );
        }
      }, 50);
    },
    [setSelectedNode, centerOnNode, getInternalNode, getNodes, fitView, bringSchemaNodeToFront]
  );

  const handleSchemaArcClick = useCallback(
    (_event: React.MouseEvent, edge: ReactFlowEdge) => {
      // Set selected edge with data for schema relation details panel
      // Extract relation type from edge data (schema edges store it in data.relationType)
      const edgeData = edge.data as { relationType?: string } | undefined;
      setSelectedEdge(edge.id, {
        id: edge.id,
        type: edgeData?.relationType ?? edge.id.split('_')[0] ?? 'UNKNOWN',
        source: edge.source,
        target: edge.target,
        data: edgeData,
      });

      // Bring source and target nodes to front (z-index)
      bringSchemaArcNodesToFront(edge);

      // FitView to show both source and target nodes
      setTimeout(() => {
        const allNodes = getNodes();
        const edgeNodes = allNodes.filter(
          n => n.id === edge.source || n.id === edge.target
        );
        if (edgeNodes.length > 0) {
          fitView({
            nodes: edgeNodes,
            duration: GRAPH_ANIMATION.FIT_VIEW_DURATION,
            padding: 0.2,
            maxZoom: 1.5,
            minZoom: 0.1,
          });
        }
      }, 50);
    },
    [setSelectedEdge, bringSchemaArcNodesToFront, getNodes, fitView]
  );

  // =========================================================================
  // NODE TYPE → LAYER MAPPING (from Neo4j via useMagneticData)
  // =========================================================================
  // Comes from OF_CLASS + IN_LAYER relationships in Neo4j, seeded from YAML. (v0.12.0: was OF_KIND)
  // No hardcoded maps - all data flows: YAML → Neo4j → API → here.
  const nodeTypeToLayer = magneticData?.nodeTypeMapping ?? {};

  // PERFORMANCE: Split layout (expensive) from dimming (cheap)
  // Step 1: Compute layout ONLY when graph data or layout direction changes (expensive)
  const layoutedNodes = useMemo(() => {
    const turboNodes = graphNodes.map((n) => toTurboNode(n));
    const floatingEdges = graphEdges.map((e) => toFloatingEdge(e));

    // Fallback grid layout if algorithm fails
    const applyFallbackLayout = (nodes: TurboNodeType[]): TurboNodeType[] => {
      const cols = Math.ceil(Math.sqrt(nodes.length));
      const spacing = INITIAL_POSITION_CONFIG.FALLBACK_GRID_SPACING;
      return nodes.map((node, i) => ({
        ...node,
        position: {
          x: (i % cols) * spacing,
          y: Math.floor(i / cols) * spacing,
        },
      }));
    };

    try {
      // =====================================================================
      // MAGNETIC MODE: Position data nodes around Realm/Layer attractors
      // =====================================================================
      if (isMagneticMode && magneticData) {
        // Fixed realm positions (triangular arrangement)
        const realmPositions: Record<string, { x: number; y: number }> = {
          project: { x: 0, y: 0 },
          global: { x: 2000, y: 0 },
          shared: { x: 1000, y: 1500 },
        };

        // Layer → Realm mapping
        const layerToRealm: Record<string, string> = {};
        for (const sub of magneticData.layers) {
          layerToRealm[sub.key] = sub.realmKey;
        }

        // Compute layer positions (circle around realm)
        const layerPositions: Record<string, { x: number; y: number }> = {};
        const layersByRealm = new Map<string, typeof magneticData.layers>();
        for (const sub of magneticData.layers) {
          const list = layersByRealm.get(sub.realmKey) || [];
          list.push(sub);
          layersByRealm.set(sub.realmKey, list);
        }
        for (const [realmKey, subs] of layersByRealm) {
          const realmPos = realmPositions[realmKey] || { x: 0, y: 0 };
          const radius = 500;
          subs.forEach((sub, i) => {
            const angle = (2 * Math.PI * i) / subs.length - Math.PI / 2;
            // Use realm-qualified key to avoid collisions (e.g., shared/config vs org/config)
            const layerPosKey = `${sub.realmKey}-${sub.key}`;
            layerPositions[layerPosKey] = {
              x: realmPos.x + radius * Math.cos(angle),
              y: realmPos.y + radius * Math.sin(angle),
            };
          });
        }

        // Create attractor nodes (Realm and Layer)
        const attractorNodes: TurboNodeType[] = [];

        // Realm attractor nodes
        for (const realm of magneticData.realms) {
          const pos = realmPositions[realm.key];
          // typeCount = how many nodeTypes belong to this realm (static, from schema)
          const typeCount = Object.values(nodeTypeToLayer)
            .filter(lk => layerToRealm[lk] === realm.key).length;
          // loadedCount = how many loaded instances belong to this realm (dynamic)
          const loadedCount = turboNodes.filter(n => {
            const lk = nodeTypeToLayer[n.data.type];
            return layerToRealm[lk] === realm.key;
          }).length;
          attractorNodes.push({
            id: `realm-${realm.key}`,
            type: 'realmAttractor',
            position: pos,
            data: {
              id: `realm-${realm.key}`,
              type: 'Realm',
              key: realm.key,
              label: realm.displayName,
              displayName: realm.displayName,
              emoji: realm.emoji,
              color: realm.color,
              typeCount,
              loadedCount,
              category: 'project',
            },
          } as unknown as TurboNodeType);
        }

        // Layer attractor nodes
        // Use realm-qualified IDs to avoid duplicates (e.g., shared/config vs org/config)
        for (const sub of magneticData.layers) {
          const layerPosKey = `${sub.realmKey}-${sub.key}`;
          const pos = layerPositions[layerPosKey];
          const parentRealm = magneticData.realms.find(s => s.key === sub.realmKey);
          // typeCount = how many nodeTypes map to this layer (static)
          const subTypeCount = Object.values(nodeTypeToLayer)
            .filter(v => v === sub.key).length;
          // loadedCount = how many loaded instances belong to this layer (dynamic)
          const subLoadedCount = turboNodes
            .filter(n => nodeTypeToLayer[n.data.type] === sub.key).length;
          const layerId = `layer-${sub.realmKey}-${sub.key}`;
          attractorNodes.push({
            id: layerId,
            type: 'layerAttractor',
            position: pos,
            data: {
              id: layerId,
              type: 'Layer',
              key: sub.key,
              label: sub.displayName,
              displayName: sub.displayName,
              emoji: sub.emoji,
              realmKey: sub.realmKey,
              color: parentRealm?.color || '#666',
              typeCount: subTypeCount,
              loadedCount: subLoadedCount,
              category: 'project',
            },
          } as unknown as TurboNodeType);
        }

        // Position data nodes near their layer (with seeded jitter)
        let seed = 12345;
        const seededRandom = () => {
          seed = (seed * 1103515245 + 12345) & 0x7fffffff;
          return seed / 0x7fffffff;
        };

        // Create nodeType → realm-qualified layer key mapping
        // (needed because layerPositions now uses qualified keys like "shared-config")
        const nodeTypeToQualifiedLayerKey: Record<string, string> = {};
        for (const [nodeType, layerKey] of Object.entries(nodeTypeToLayer)) {
          const layer = magneticData.layers.find(l => l.key === layerKey);
          if (layer) {
            nodeTypeToQualifiedLayerKey[nodeType] = `${layer.realmKey}-${layer.key}`;
          }
        }

        const magneticDataNodes = turboNodes.map(node => {
          const qualifiedLayerKey = nodeTypeToQualifiedLayerKey[node.data.type] || 'org-foundation';
          const layerPos = layerPositions[qualifiedLayerKey] || { x: 0, y: 0 };
          return {
            ...node,
            position: {
              x: layerPos.x + (seededRandom() - 0.5) * 400,
              y: layerPos.y + (seededRandom() - 0.5) * 400,
            },
          };
        });

        // Combine attractor nodes with data nodes
        return [...attractorNodes, ...magneticDataNodes];
      }

      // Determine dagre direction based on layoutDirection
      const dagreDir = layoutDirection === 'LR' ? 'LR' : 'TB';

      // For pure force layout, skip dagre and start with random positions
      if (layoutDirection === 'force') {
        // Spread nodes in a larger spiral before force simulation
        const randomPositioned = turboNodes.map((node, i) => ({
          ...node,
          position: {
            x: Math.cos(i * INITIAL_POSITION_CONFIG.SPIRAL_ANGLE_INCREMENT) * INITIAL_POSITION_CONFIG.SPIRAL_RADIUS + Math.random() * INITIAL_POSITION_CONFIG.SPIRAL_JITTER,
            y: Math.sin(i * INITIAL_POSITION_CONFIG.SPIRAL_ANGLE_INCREMENT) * INITIAL_POSITION_CONFIG.SPIRAL_RADIUS + Math.random() * INITIAL_POSITION_CONFIG.SPIRAL_JITTER,
          },
        }));

        const simulation = createForceSimulation(randomPositioned, floatingEdges, {
          chargeStrength: FORCE_CONFIG.CHARGE_STRENGTH,
          linkDistance: FORCE_CONFIG.LINK_DISTANCE,
          clusterByCategory: true,
          collisionRadius: FORCE_CONFIG.COLLISION_RADIUS,
          centerStrength: FORCE_CONFIG.CENTER_STRENGTH,
          alphaDecay: FORCE_CONFIG.ALPHA_DECAY,
          velocityDecay: FORCE_CONFIG.VELOCITY_DECAY,
          nodeCount: graphNodes.length,
        });

        const positions = runSimulationSync(simulation, FORCE_CONFIG.ITERATIONS);
        return applyForcePositions(randomPositioned, positions);
      }

      // For radial layout, arrange in concentric circles with better spacing
      if (layoutDirection === 'radial') {
        const centerX = 0;
        const centerY = 0;
        // Larger radius for better spacing
        const radius = Math.max(RADIAL_CONFIG.MIN_RADIUS, graphNodes.length * RADIAL_CONFIG.RADIUS_PER_NODE);
        const angleStep = (2 * Math.PI) / Math.max(turboNodes.length, 1);

        return turboNodes.map((node, i) => ({
          ...node,
          position: {
            x: centerX + radius * Math.cos(i * angleStep - Math.PI / 2),
            y: centerY + radius * Math.sin(i * angleStep - Math.PI / 2),
          },
        }));
      }

      // Step 1: Apply dagre layout for initial hierarchical positioning
      const dagrePositioned = applyDagreLayout(turboNodes, floatingEdges, {
        direction: dagreDir,
        ranksep: DAGRE_CONFIG.RANK_SEP,
        nodesep: DAGRE_CONFIG.NODE_SEP,
        nodeWidth: DAGRE_CONFIG.NODE_WIDTH,
        nodeHeight: DAGRE_CONFIG.NODE_HEIGHT,
      });

      // For pure dagre (TB/LR), skip force simulation
      if (layoutDirection === 'TB' || layoutDirection === 'LR') {
        return dagrePositioned;
      }

      // Default 'dagre' mode: Apply force simulation for physics-based refinement
      const simulation = createForceSimulation(dagrePositioned, floatingEdges, {
        chargeStrength: COMBINED_LAYOUT_CONFIG.CHARGE_STRENGTH,
        linkDistance: COMBINED_LAYOUT_CONFIG.LINK_DISTANCE,
        clusterByCategory: true,
        collisionRadius: COMBINED_LAYOUT_CONFIG.COLLISION_RADIUS,
        centerStrength: COMBINED_LAYOUT_CONFIG.CENTER_STRENGTH,
        alphaDecay: COMBINED_LAYOUT_CONFIG.ALPHA_DECAY,
        velocityDecay: COMBINED_LAYOUT_CONFIG.VELOCITY_DECAY,
        nodeCount: graphNodes.length,
      });

      const positions = runSimulationSync(simulation, COMBINED_LAYOUT_CONFIG.ITERATIONS);
      return applyForcePositions(dagrePositioned, positions);
    } catch (error) {
      // Layout algorithm failed - use fallback grid layout
      logger.error('Graph2D', 'Layout computation failed, using fallback grid', error);
      return applyFallbackLayout(turboNodes);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps -- layoutVersion forces recalculation even when direction unchanged
  }, [graphNodes, graphEdges, layoutDirection, layoutVersion, isMagneticMode, magneticData]);

  // DEFENSIVE: Filter out any nodes with invalid positions (prevents React Flow crashes)
  const validLayoutedNodes = useMemo(() => {
    return layoutedNodes.filter((node) => {
      const hasValidPosition = node.position &&
        typeof node.position.x === 'number' && !isNaN(node.position.x) &&
        typeof node.position.y === 'number' && !isNaN(node.position.y);
      if (!hasValidPosition) {
        logger.warn('Graph2D', 'Filtered node with invalid position', { nodeId: node.id });
      }
      return hasValidPosition;
    });
  }, [layoutedNodes]);

  // Step 2: Apply dimming state ONLY when focus/hover changes (cheap O(n) operation)
  // Priority: Selection focus mode > Edge focus mode > Hover highlight > Normal
  const initialNodes = useMemo(() => {
    return validLayoutedNodes.map((node) => {
      // Focus mode dimming takes precedence over hover dimming
      // This now includes edge selection/hover (isNodeDimmed checks both)
      const focusDimmed = isNodeDimmed(node.id);
      // Hover dimming is lighter (25% opacity vs 15% for focus)
      const hoverDimmed = isNodeHoverDimmed(node.id);

      return {
        ...node,
        data: {
          ...node.data,
          // Apply dimming: focus mode takes precedence
          dimmed: focusDimmed,
          // Additional lighter dimming for hover (only when no focus mode)
          hoverDimmed: !focusDimmed && hoverDimmed,
        },
      };
    });
  // eslint-disable-next-line react-hooks/exhaustive-deps -- selection/hover triggers recalc
  }, [validLayoutedNodes, isNodeDimmed, isNodeHoverDimmed, focusSelectedId, focusSelectedEdgeId, focusHoveredEdgeId, hoveredId]);

  // =========================================================================
  // EDGE DATA - Simplified
  // =========================================================================
  // Hover state (hovered, hoverDimmed) is computed LOCALLY in FloatingEdge via
  // direct Zustand store subscription. This bypasses React Flow's broken edge
  // data update mechanism. Only focus-mode dimming still comes through data.
  // =========================================================================
  // Build set of visible node IDs for edge validation
  const visibleNodeIdSet = useMemo(() => {
    return new Set(validLayoutedNodes.map((n) => n.id));
  }, [validLayoutedNodes]);

  // Memoize visible graph edges for parallel detection (v11.6.1)
  const visibleGraphEdges = useMemo(() => {
    return graphEdges.filter((e) => visibleNodeIdSet.has(e.source) && visibleNodeIdSet.has(e.target));
  }, [graphEdges, visibleNodeIdSet]);

  // Detect parallel edges (v11.6.1)
  // Cast to Edge[] since useParallelEdges only uses id/source/target
  const parallelEdgeGroups = useParallelEdges(visibleGraphEdges as unknown as ReactFlowEdge[]);

  const initialEdges = useMemo(() => {
    // DEFENSIVE: Only include edges where both source and target nodes exist
    const businessEdges = visibleGraphEdges
      .map((e) => {
        const dimmed = isEdgeDimmed(e.source, e.target);

        // Get parallel edge info from groups (v11.6.1)
        const parallelInfo = getEdgeIndexInGroup(
          { id: e.id, source: e.source, target: e.target } as ReactFlowEdge,
          parallelEdgeGroups.groups
        );

        // Check if this edge is part of a bundled group (4+ edges)
        const [sortedA, sortedB] = [e.source, e.target].sort();
        const bundleKey = `${sortedA}::${sortedB}`;
        const group = parallelEdgeGroups.groups.get(bundleKey);
        const isBundledEdge = group?.isBundled && !expandedBundles.has(bundleKey);

        // Skip bundled edges when collapsed (v11.6.1)
        // TODO: Add BundledEdgeOverlay component to render collapsed bundles
        // For now, bundled edges (4+) are hidden when collapsed, shown when expanded via hover
        if (isBundledEdge) {
          return null;
        }

        return {
          id: e.id,
          source: e.source,
          target: e.target,
          type: 'floating' as const,
          data: {
            relationType: e.type,
            dimmed, // Focus mode dimming only
            animated: !dimmed, // Animation controlled by focus dimming (hover dimming handled locally)
            showLabel: showEdgeLabels,
            // Parallel edge offset info (v11.6.1)
            parallelIndex: parallelInfo?.index,
            parallelTotal: parallelInfo?.total,
          },
        };
      })
      .filter((e): e is NonNullable<typeof e> => e !== null);

    // In magnetic mode, add structural edges (Realm→Layer, Node→Layer)
    if (isMagneticMode && magneticData) {
      const magneticEdges: Array<(typeof businessEdges)[number] | FloatingEdgeType> = [];

      // Realm → Layer edges (HAS_LAYER)
      // Use realm-qualified layer IDs to avoid duplicates
      for (const sub of magneticData.layers) {
        const layerId = `layer-${sub.realmKey}-${sub.key}`;
        magneticEdges.push({
          id: `edge-realm-${sub.realmKey}-to-${sub.key}`,
          source: `realm-${sub.realmKey}`,
          target: layerId,
          type: 'floating' as const,
          data: {
            relationType: 'HAS_LAYER',
            dimmed: false,
            animated: false,
            showLabel: showEdgeLabels,
          },
        });
      }

      // Build nodeType → full layer ID mapping
      const nodeTypeToLayerId: Record<string, string> = {};
      for (const [nodeType, layerKey] of Object.entries(nodeTypeToLayer)) {
        const layer = magneticData.layers.find(l => l.key === layerKey);
        if (layer) {
          nodeTypeToLayerId[nodeType] = `layer-${layer.realmKey}-${layer.key}`;
        }
      }

      // Data node → Layer edges (OF_CLASS) - faint magnetic edges (v0.12.0: was OF_KIND)
      for (const node of graphNodes) {
        const layerId = nodeTypeToLayerId[node.type];
        if (layerId) {
          magneticEdges.push({
            id: `edge-${node.id}-in-${layerId}`,
            source: node.id,
            target: layerId,
            type: 'magnetic',
            data: {
              relationType: 'OF_CLASS',
              dimmed: false,
              animated: false,
              showLabel: false,
            },
          } as unknown as FloatingEdgeType);
        }
      }

      return [...businessEdges, ...magneticEdges];
    }

    return businessEdges;
  }, [visibleGraphEdges, graphNodes, parallelEdgeGroups, expandedBundles, isEdgeDimmed, showEdgeLabels, isMagneticMode, magneticData]);

  // React Flow state
  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

  // ==========================================================================
  // PERFORMANCE: Update edge count for simplified effects mode
  // ==========================================================================
  // Track total edge count for performance decisions (simplified effects, disabled animations)
  const setTotalEdgeCount = useEdgeVisibilityStore((state) => state.setTotalEdgeCount);
  useEffect(() => {
    setTotalEdgeCount(initialEdges.length);
  }, [initialEdges.length, setTotalEdgeCount]);

  // ==========================================================================
  // Z-INDEX MANAGEMENT - Data mode (schema mode hook is above, near setSchemaNodes)
  // ==========================================================================
  // Brings clicked/hovered nodes to front to handle overlapping nodes
  const {
    bringToFront: bringDataNodeToFront,
    setHoverZIndex: setDataHoverZIndex,
    resetZIndex: resetDataZIndex,
    bringEdgeNodesToFront: bringDataEdgeNodesToFront,
  } = useGraphInteractions({ setNodes: setNodes as React.Dispatch<React.SetStateAction<ReactFlowNode[]>> });

  // Keep refs updated for keyboard handler (avoids re-registering event listeners)
  nodesRef.current = nodes;
  getZoomRef.current = getZoom;
  setCenterRef.current = setCenter;

  // ============================================================================
  // CRITICAL: Smart node sync that preserves user-dragged positions
  // ============================================================================
  // Problem: `initialNodes` recalculates when hoveredId changes (for dimming).
  // During drag, mouse moves trigger hover → initialNodes recalc → setNodes
  // overwrites the dragged node's position with the original layout position.
  //
  // Solution: Only do full position sync when layout ACTUALLY changes.
  // For data-only changes (dimming), preserve current positions.
  // ============================================================================
  useEffect(() => {
    const layoutChanged = appliedLayoutVersionRef.current !== layoutVersion;

    if (layoutChanged) {
      // New layout applied (user pressed Shift+H/V/D/R/F) - full sync
      appliedLayoutVersionRef.current = layoutVersion;
      setNodes(initialNodes);
      return;
    }

    // Layout unchanged - check if nodes structurally changed
    setNodes((currentNodes) => {
      // Build lookup map for O(1) access
      const initialNodeMap = new Map(initialNodes.map((n) => [n.id, n]));

      // Check if node IDs match (same structure)
      const idsMatch =
        currentNodes.length === initialNodes.length &&
        currentNodes.every((n) => initialNodeMap.has(n.id));

      if (!idsMatch) {
        // Structural change (different nodes) - full sync needed
        return initialNodes;
      }

      // Same nodes, same layout - ONLY update data (dimming), PRESERVE positions
      // This is the key fix: positions come from currentNodes (may be user-dragged)
      // while data comes from initialNodes (fresh dimming state)
      return currentNodes.map((node) => ({
        ...node,
        data: initialNodeMap.get(node.id)!.data,
      }));
    });
  }, [initialNodes, setNodes, layoutVersion]);

  // =========================================================================
  // EDGE SYNC - Simplified
  // =========================================================================
  // Hover state changes no longer require edge re-sync (handled in FloatingEdge).
  // This effect only runs when:
  // - Graph data changes (graphEdges)
  // - Focus mode dimming changes (isEdgeDimmed)
  // - Label visibility changes (showEdgeLabels)
  // =========================================================================
  useEffect(() => {
    setEdges(initialEdges);
  }, [initialEdges, setEdges]);

  // ==========================================================================
  // PERF: Memoized MiniMap nodeColor callbacks to avoid re-creating functions
  // ==========================================================================
  const schemaMinimapNodeColor = useCallback((node: ReactFlowNode) => {
    // Color by realm for schema nodes (v0.12.0: shared/org)
    const realm = node.data?.realm;
    if (realm === 'shared') return '#10b981cc'; // emerald - universal knowledge
    if (realm === 'org') return '#8b5cf6cc'; // violet - org-specific
    return '#666';
  }, []);

  const dataMinimapNodeColor = useCallback((node: ReactFlowNode) => {
    const config = NODE_TYPE_CONFIG[(node.data as TurboNodeData)?.type];
    // Softer colors with transparency for dots
    return config?.color ? `${config.color}cc` : '#666';
  }, []);

  // ==========================================================================
  // Click Handling
  // - Click: Select node + open panel + zoom/center (same as schema mode)
  // - Double-click: Zoom/center + expand neighbors
  // ==========================================================================
  const handleNodeClick: NodeMouseHandler<TurboNodeType> = useCallback(
    (_, node) => {
      // Click: select, open panel, zoom/center
      setSelectedNode(node.id);

      // Bring clicked node to front (z-index)
      bringDataNodeToFront(node.id);

      const nodeWidth = node.measured?.width ?? DAGRE_CONFIG.NODE_WIDTH;
      const nodeHeight = node.measured?.height ?? DAGRE_CONFIG.NODE_HEIGHT;

      // Small delay to ensure state is fully propagated before centering
      // This ensures the panel width is accounted for in the center calculation
      setTimeout(() => {
        centerOnNode(
          node.position.x,
          node.position.y,
          nodeWidth,
          nodeHeight,
          { zoom: GRAPH_ANIMATION.NODE_DOUBLE_CLICK_ZOOM, duration: GRAPH_ANIMATION.FIT_VIEW_DURATION }
        );
      }, 50);

      // Forward to external callback
      onNodeClick?.(node.id);
    },
    [setSelectedNode, centerOnNode, onNodeClick, bringDataNodeToFront]
  );

  const handleNodeDoubleClick: NodeMouseHandler<TurboNodeType> = useCallback(
    (_, node) => {
      // Double-click: zoom/center + expand neighbors (NO panel opening)
      // Panel opening is exclusive to ⌘+Click
      const nodeWidth = node.measured?.width ?? DAGRE_CONFIG.NODE_WIDTH;
      const nodeHeight = node.measured?.height ?? DAGRE_CONFIG.NODE_HEIGHT;

      requestAnimationFrame(() => {
        centerOnNode(
          node.position.x,
          node.position.y,
          nodeWidth,
          nodeHeight,
          { zoom: GRAPH_ANIMATION.NODE_DOUBLE_CLICK_ZOOM, duration: GRAPH_ANIMATION.FIT_VIEW_DURATION }
        );
      });

      // Expand neighbors
      onNodeDoubleClick?.(node.id);
    },
    [centerOnNode, onNodeDoubleClick]
  );

  const handleEdgeClick: EdgeMouseHandler<FloatingEdgeType> = useCallback(
    (_, edge) => {
      // Set selected edge with data for details panel
      setSelectedEdge(edge.id, {
        id: edge.id,
        type: edge.data?.relationType ?? 'UNKNOWN',
        source: edge.source,
        target: edge.target,
        data: edge.data,
      });

      // Bring source and target nodes to front (z-index)
      bringDataEdgeNodesToFront(edge);

      // FitView to show both source and target nodes
      setTimeout(() => {
        const edgeNodes = nodes.filter(
          n => n.id === edge.source || n.id === edge.target
        );
        if (edgeNodes.length > 0) {
          fitView({
            nodes: edgeNodes,
            duration: GRAPH_ANIMATION.FIT_VIEW_DURATION,
            padding: 0.2,
            maxZoom: 1.5,
            minZoom: 0.1,
          });
        }
      }, 50);
    },
    [setSelectedEdge, bringDataEdgeNodesToFront, nodes, fitView]
  );

  // Edge hover handlers
  // Generic type to work with both data edges and schema edges
  const handleEdgeMouseEnter = useCallback(
    (_: React.MouseEvent, edge: { id: string }) => {
      setHoveredEdge(edge.id);
    },
    [setHoveredEdge]
  );

  const handleEdgeMouseLeave = useCallback(
    () => {
      setHoveredEdge(null);
    },
    [setHoveredEdge]
  );

  // Node hover handlers for connection highlighting + z-index management
  // Separate handlers for data and schema modes to use correct z-index functions

  // DATA MODE hover handlers
  const handleDataNodeMouseEnter = useCallback(
    (_: React.MouseEvent, node: { id: string }) => {
      setHoveredNode(node.id);
      setDataHoverZIndex(node.id);
    },
    [setHoveredNode, setDataHoverZIndex]
  );

  const handleDataNodeMouseLeave = useCallback(
    (_: React.MouseEvent, node: { id: string }) => {
      setHoveredNode(null);
      resetDataZIndex(node.id);
    },
    [setHoveredNode, resetDataZIndex]
  );

  // SCHEMA MODE hover handlers
  const handleSchemaNodeMouseEnter = useCallback(
    (_: React.MouseEvent, node: { id: string }) => {
      setHoveredNode(node.id);
      setSchemaHoverZIndex(node.id);
    },
    [setHoveredNode, setSchemaHoverZIndex]
  );

  const handleSchemaNodeMouseLeave = useCallback(
    (_: React.MouseEvent, node: { id: string }) => {
      setHoveredNode(null);
      resetSchemaZIndex(node.id);
    },
    [setHoveredNode, resetSchemaZIndex]
  );

  const handlePaneClick = useCallback(() => {
    // v11.6.3: Use clearSelection to clear both node and edge selection
    // This exits focus mode when clicking on the canvas background
    clearSelection();
    setContextMenu(null);

    // Deselect all schema nodes when clicking on pane
    // Required because we use useState instead of useNodesState for schema mode
    // v12: Check if schemaNodes exist instead of navigationMode
    if (schemaNodes.length > 0) {
      setSchemaNodes((nodes) =>
        nodes.map((n) => ({ ...n, selected: false }))
      );
    }

    onPaneClick?.();
  }, [clearSelection, onPaneClick, schemaNodes.length]);

  // Right-click context menu handler
  const handleNodeContextMenu: NodeMouseHandler<TurboNodeType> = useCallback(
    (event, node) => {
      event.preventDefault();
      setContextMenu({
        nodeId: node.id,
        position: { x: event.clientX, y: event.clientY },
      });
    },
    []
  );

  // Close context menu
  const handleCloseContextMenu = useCallback(() => {
    setContextMenu(null);
  }, []);

  // ==========================================================================
  // Keyboard Navigation (Neo4j Browser style)
  // Tab: cycle through connected nodes
  // Shift+Tab: cycle inverse direction
  // Enter: expand node (like double-click)
  // Delete/Backspace: hide node
  // F: fit view to all nodes
  // Escape: clear selection
  // ==========================================================================
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      // Skip if focus is in an input element
      const target = event.target as HTMLElement;
      if (
        target.tagName === 'INPUT' ||
        target.tagName === 'TEXTAREA' ||
        target.isContentEditable
      ) {
        return;
      }

      // Tab: cycle through connected nodes
      if (event.key === 'Tab' && selectedNodeId) {
        event.preventDefault();
        const connected = Array.from(connectedIdsRef.current);
        if (connected.length === 0) return;

        // Determine direction
        const direction = event.shiftKey ? -1 : 1;

        // Update cycle index
        cycleIndexRef.current = (cycleIndexRef.current + direction + connected.length) % connected.length;

        // Select the next connected node
        setSelectedNode(connected[cycleIndexRef.current]);
        return;
      }

      // Enter: expand selected node (like double-click) with centering
      if (event.key === 'Enter' && selectedNodeId) {
        event.preventDefault();
        // Find the node to get its position for centering (use ref for latest nodes)
        const selectedNode = nodesRef.current.find((n) => n.id === selectedNodeId);
        if (selectedNode) {
          const currentZoom = getZoomRef.current();
          const targetZoom = Math.max(currentZoom, GRAPH_ANIMATION.DEFAULT_CENTER_ZOOM);
          setCenterRef.current(selectedNode.position.x, selectedNode.position.y, {
            zoom: targetZoom,
            duration: GRAPH_ANIMATION.FIT_VIEW_DURATION,
          });
        }
        expandNode(selectedNodeId);
        return;
      }

      // Delete or Backspace: hide selected node
      if ((event.key === 'Delete' || event.key === 'Backspace') && selectedNodeId) {
        event.preventDefault();
        hideNode(selectedNodeId);
        setSelectedNode(null);
        cycleIndexRef.current = 0;
        return;
      }

      // F: fit view (global shortcut, like Neo4j Browser)
      if (event.key === 'f' && !event.metaKey && !event.ctrlKey && !event.shiftKey) {
        event.preventDefault();
        smartFitViewRef.current({ duration: GRAPH_ANIMATION.FIT_VIEW_DURATION });
        return;
      }

      // Escape: clear selection (handled here for graph-specific context)
      // Note: Global escape handler in useKeyboardShortcuts handles dialogs
      // v11.6.3: Also handle edge selection to exit edge focus mode
      if (event.key === 'Escape' && (selectedNodeId || focusSelectedEdgeId)) {
        event.preventDefault();
        clearSelection();
        setContextMenu(null);
        cycleIndexRef.current = 0;
        return;
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
    // Dependencies: Only include values that SHOULD trigger re-registration.
    //
    // Refs pattern (connectedIdsRef, nodesRef, getZoomRef, setCenterRef):
    // - Refs are updated during the RENDER phase (before effects run)
    // - React guarantees: render → ref updates → effect cleanup → effect setup
    // - So when this effect runs, refs already contain fresh values
    // - This avoids re-registering the listener on every data change
    //
    // Direct deps (selectedNodeId, focusSelectedEdgeId, callbacks):
    // - These SHOULD trigger re-registration because handler behavior changes
  }, [selectedNodeId, focusSelectedEdgeId, setSelectedNode, expandNode, hideNode, clearSelection]);

  // Reset cycle index when selection changes externally
  useEffect(() => {
    cycleIndexRef.current = 0;
  }, [selectedNodeId]);

  // ==========================================================================
  // Auto-FitView on UI State Changes
  // Trigger smooth fitView when sidebar toggles, focus mode changes, panel closes,
  // or layout changes (ensures nodes stay visible after layout recalculation)
  // Note: Don't trigger when panel OPENS (that's handled by centerOnNode in double-click)
  // ==========================================================================
  useEffect(() => {
    // Check if any relevant state changed
    const sidebarChanged = prevSidebarOpenRef.current !== sidebarOpen;
    const focusModeChanged = prevFocusModeRef.current !== focusMode;
    const layoutChanged = prevLayoutVersionRef.current !== layoutVersion;

    // Update refs
    prevSidebarOpenRef.current = sidebarOpen;
    prevFocusModeRef.current = focusMode;
    prevLayoutVersionRef.current = layoutVersion;

    // Trigger fitView on relevant changes (not when panel opens or selection clears)
    // selectionClosed removed: clicking outside shouldn't cause jarring dezoom
    if (sidebarChanged || focusModeChanged || layoutChanged) {
      // Small delay to let CSS animations start
      const timeoutId = setTimeout(() => {
        smartFitViewRef.current({ duration: GRAPH_ANIMATION.FIT_VIEW_DURATION });
      }, GRAPH_ANIMATION.UI_CHANGE_DELAY);

      return () => clearTimeout(timeoutId);
    }
    // PERF: smartFitView accessed via ref to avoid effect re-registration

  }, [sidebarOpen, focusMode, layoutVersion]);

  // ==========================================================================
  // Magnetic Mode FitView
  // Refit camera when magnetic data arrives or when leaving magnetic mode.
  // Separate from auto-fitView because magnetic data loads async (~20ms).
  // ==========================================================================
  useEffect(() => {
    const dataJustArrived = !prevMagneticDataRef.current && magneticData && isMagneticMode;
    const leftMagnetic = prevMagneticDataRef.current && !isMagneticMode;
    prevMagneticDataRef.current = magneticData;

    if (dataJustArrived || leftMagnetic) {
      const timeoutId = setTimeout(() => {
        smartFitViewRef.current({ duration: GRAPH_ANIMATION.FIT_VIEW_DURATION });
      }, GRAPH_ANIMATION.UI_CHANGE_DELAY);
      return () => clearTimeout(timeoutId);
    }
    // PERF: smartFitView accessed via ref to avoid effect re-registration
  }, [isMagneticMode, magneticData]);

  // ==========================================================================
  // Navigation Mode FitView
  // Refit camera when switching between data/meta modes.
  // Triggered after transition completes (reform phase ends).
  // ==========================================================================
  const prevTransitionPhaseRef = useRef(transitionPhase);
  useEffect(() => {
    const transitionJustEnded = prevTransitionPhaseRef.current === 'reform' && transitionPhase === null;
    prevTransitionPhaseRef.current = transitionPhase;

    if (transitionJustEnded) {
      // Fit view after mode transition completes
      const timeoutId = setTimeout(() => {
        smartFitViewRef.current({ duration: GRAPH_ANIMATION.FIT_VIEW_DURATION });
      }, 50); // Small delay to ensure nodes are rendered
      return () => clearTimeout(timeoutId);
    }
    // PERF: smartFitView accessed via ref to avoid effect re-registration
  }, [transitionPhase]);

  // ==========================================================================
  // Selection Close FitView
  // Refit camera when aside panel closes (selection cleared).
  // Provides better UX by centering content after closing detail panel.
  // ==========================================================================
  useEffect(() => {
    const hasSelection = selectedNodeId !== null || selectedEdgeId !== null;

    // If selection was present and is now cleared, fit view
    if (prevHasSelectionRef.current && !hasSelection) {
      const timeoutId = setTimeout(() => {
        smartFitViewRef.current({ duration: GRAPH_ANIMATION.FIT_VIEW_DURATION });
      }, GRAPH_ANIMATION.UI_CHANGE_DELAY);
      return () => clearTimeout(timeoutId);
    }

    // Update ref for next render
    prevHasSelectionRef.current = hasSelection;
    // PERF: smartFitView accessed via ref to avoid effect re-registration
  }, [selectedNodeId, selectedEdgeId]);

  // ==========================================================================
  // Initial FitView on Mount
  // Use smartFitView with proper insets instead of ReactFlow's default fitView
  // ==========================================================================
  useEffect(() => {
    // Only run once on mount when we have nodes
    if (initialFitDoneRef.current || nodes.length === 0) return;

    // Small delay to ensure layout is complete
    const timeoutId = setTimeout(() => {
      smartFitViewRef.current({ duration: 0 }); // No animation on initial load
      initialFitDoneRef.current = true;
    }, GRAPH_ANIMATION.INITIAL_FIT_DELAY);

    return () => clearTimeout(timeoutId);
    // PERF: smartFitView accessed via ref to avoid effect re-registration
     
  }, [nodes.length]);

  // =========================================================================
  // SCHEMA MODE RENDER (Task 3.2)
  // =========================================================================
  // v12: Schema mode is detected from loaded nodes (schema- prefix), not
  // from navigation mode. Renders hierarchical schema visualization
  // with ELK layout and group nodes. Wrapped in SchemaErrorBoundary.
  // =========================================================================
  if (isSchemaMode) {
    return (
      <SchemaErrorBoundary>
        <div
          className={cn('h-full w-full transition-opacity duration-400 ease-out', className)}
          style={{ opacity: graphOpacity }}
          data-testid="react-flow-wrapper-schema"
        >
          <ReactFlow
            nodes={schemaNodes}
            edges={schemaArcs}
            nodeTypes={nodeTypes}
            edgeTypes={edgeTypes}
            onNodesChange={handleSchemaNodesChange}
            onNodeDrag={handleSchemaNodeDrag}
            onNodeDragStop={handleSchemaNodeDragStop}
            onNodeClick={handleSchemaNodeClick}
            onNodeMouseEnter={handleSchemaNodeMouseEnter}
            onNodeMouseLeave={handleSchemaNodeMouseLeave}
            onEdgeClick={handleSchemaArcClick}
            onEdgeMouseEnter={handleEdgeMouseEnter}
            onEdgeMouseLeave={handleEdgeMouseLeave}
            onPaneClick={handlePaneClick}
            connectionMode={ConnectionMode.Loose}
            fitView
            fitViewOptions={{ padding: 0.2 }}
            minZoom={0.05}
            maxZoom={2}
            proOptions={{
              hideAttribution: true,
            }}
            // Schema mode: Interactive (Tasks 1, 3, 4)
            nodesDraggable={true}
            nodesConnectable={false}
            elementsSelectable={true}
            selectNodesOnDrag={false}
            panOnScroll={true}
            zoomOnScroll={true}
            zoomOnPinch={true}
            // Accessibility (Context7 best practices)
            nodesFocusable={true}
            edgesFocusable={true}
            // Style
            colorMode="dark"
          >
            {/* Background - subtle dot grid */}
            <Background
              variant={BackgroundVariant.Dots}
              color="rgba(255, 255, 255, 0.03)"
              gap={24}
              size={1}
            />

            {/* Minimap for schema mode */}
            {showMinimap && minimapVisible && (
              <MiniMap
                className={cn(
                  glassClasses.heavy,
                  '[&_.react-flow__minimap-edge]:hidden'
                )}
                style={{ height: MINIMAP_HEIGHT }}
                nodeColor={schemaMinimapNodeColor}
                nodeStrokeWidth={0}
                nodeBorderRadius={4}
                maskColor="rgba(0, 0, 0, 0.85)"
                pannable
                zoomable
                position="bottom-right"
              />
            )}

            {/* GraphToolbar - positioned above minimap */}
            {showControls && (
              <div
                className="absolute z-10 right-3"
                style={{
                  bottom: showMinimap && minimapVisible ? `${TOOLBAR_BOTTOM_OFFSET}px` : '12px',
                }}
              >
                <GraphToolbar />
              </div>
            )}
          </ReactFlow>

          {/* Loading indicator during ELK layout */}
          {isSchemaLayouting && (
            <div
              className={cn(
                'absolute inset-0 flex items-center justify-center',
                glassClasses.medium,
                'z-50'
              )}
              data-testid="schema-loading-indicator"
            >
              <div className={cn('flex flex-col items-center', gapTokens.spacious)}>
                <div className="w-8 h-8 border-2 border-white/30 border-t-white/90 rounded-full animate-spin" />
                <span className="text-sm text-white/70">Layouting schema...</span>
              </div>
            </div>
          )}

        </div>
      </SchemaErrorBoundary>
    );
  }

  // =========================================================================
  // DATA MODE RENDER (default)
  // =========================================================================
  // Empty state for data mode
  if (graphNodes.length === 0) {
    return (
      <div className={cn('h-full', className)}>
        <GraphEmptyState />
      </div>
    );
  }

  return (
    <EdgeVisibilityProvider>
      <div
        className={cn('h-full w-full transition-opacity duration-400 ease-out', className)}
        style={{ opacity: graphOpacity }}
        data-testid="react-flow-wrapper"
      >
        <ReactFlow
          nodes={nodes}
          edges={edges}
          onNodesChange={onNodesChange}
          onEdgesChange={onEdgesChange}
          onNodeClick={handleNodeClick}
          onNodeDoubleClick={handleNodeDoubleClick}
          onNodeContextMenu={handleNodeContextMenu}
          onNodeMouseEnter={handleDataNodeMouseEnter}
          onNodeMouseLeave={handleDataNodeMouseLeave}
          onEdgeClick={handleEdgeClick}
          onEdgeMouseEnter={handleEdgeMouseEnter}
          onEdgeMouseLeave={handleEdgeMouseLeave}
          onPaneClick={handlePaneClick}
          nodeTypes={nodeTypes}
          edgeTypes={edgeTypes}
          connectionMode={ConnectionMode.Loose}
          // NOTE: fitView is disabled - we use smartFitView on mount for proper insets
          minZoom={0.05}
          maxZoom={2}
          defaultEdgeOptions={{
            type: 'floating',
          }}
          proOptions={{
            hideAttribution: true,
          }}
          // Interaction
          nodesDraggable={true}
          nodesConnectable={false}
          elementsSelectable={true}
          selectNodesOnDrag={false}
          panOnScroll={true}
          zoomOnScroll={true}
          zoomOnPinch={true}
          // Accessibility (Context7 best practices)
          nodesFocusable={true}
          edgesFocusable={true}
          // Style
          colorMode="dark"
        >
          {/* Background - subtle dot grid */}
          <Background
            variant={BackgroundVariant.Dots}
            color="rgba(255, 255, 255, 0.03)"
            gap={24}
            size={1}
          />


          {/* ═══════════════════════════════════════════════════════════════════════
              BOTTOM-RIGHT PANEL: GraphToolbar + Minimap
              Positioned as a cohesive unit with toolbar above minimap
              ═══════════════════════════════════════════════════════════════════════ */}

          {/* ═══════════════════════════════════════════════════════════════════════
              MINIMAP - Glass style matching toolbar
              ═══════════════════════════════════════════════════════════════════════ */}
          {showMinimap && minimapVisible && (
            <MiniMap
              className={cn(
                glassClasses.heavy,
                // Hide edges for cleaner minimal look
                '[&_.react-flow__minimap-edge]:hidden'
              )}
              style={{ height: MINIMAP_HEIGHT }}
              nodeColor={dataMinimapNodeColor}
              nodeStrokeWidth={0}
              nodeBorderRadius={50}
              maskColor="rgba(0, 0, 0, 0.85)"
              pannable
              zoomable
              position="bottom-right"
            />
          )}

          {/* GraphToolbar - Positioned above minimap */}
          {showControls && (
            <div
              className="absolute z-10 right-3"
              style={{
                bottom: showMinimap && minimapVisible ? `${TOOLBAR_BOTTOM_OFFSET}px` : '12px',
              }}
            >
              <GraphToolbar />
            </div>
          )}

        </ReactFlow>

        {/* Context Menu */}
        {contextMenu && (
          <NodeContextMenu
            nodeId={contextMenu.nodeId}
            position={contextMenu.position}
            onClose={handleCloseContextMenu}
          />
        )}
      </div>
    </EdgeVisibilityProvider>
  );
}

/**
 * Graph2D with ReactFlowProvider wrapper
 */
export const Graph2D = memo(function Graph2D(props: Graph2DProps) {
  return (
    <ReactFlowProvider>
      <Graph2DInner {...props} />
    </ReactFlowProvider>
  );
});
