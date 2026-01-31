'use client';

/**
 * Graph2D Component - Beautiful Knowledge Graph Visualization
 *
 * Features:
 * - Turbo-style glowing animated nodes
 * - Gradient animated edges
 * - Dagre hierarchical layout
 * - Smooth pan/zoom interactions
 * - Category-based color coding
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
import { Sparkles } from 'lucide-react';
import { cn } from '@/lib/utils';
import { logger } from '@/lib/logger';
import { glassClasses, gapTokens } from '@/design/tokens';
import { EmptyState } from '@/components/ui/EmptyState';
import { useFilteredGraph, useFocusMode, useHoverHighlight, useNodeExpansion, useCenterOnNode, useSmartFitView, useContainerConstraint, useGraphInteractions, Z_INDEX } from '@/hooks';
import { useUIStore } from '@/stores/uiStore';
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
import {
  TurboNode,
  StructuralNode,
  LocaleKnowledgeNode,
  ProjectNode,
  type TurboNodeData,
  type TurboNodeType,
} from './nodes';
import { FloatingEdge, type FloatingEdgeType, EdgeVisibilityProvider } from './edges';
import { NodeContextMenu } from './NodeContextMenu';
import { GraphToolbar } from './GraphToolbar';
import type { GraphNode as GraphNodeType, GraphEdge as GraphEdgeType } from '@/types';

// Schema mode imports (Task 3.2)
import { ScopeGroupNode, SubcategoryGroupNode, SchemaNode } from './schema';
import { SchemaErrorBoundary } from './SchemaErrorBoundary';
import { applySchemaLayout } from '@/lib/schemaLayoutELK';
import { getSchemaHierarchy } from '@novanet/core/graph';
import { useFilterStore } from '@/stores/filterStore';
import type { Scope } from '@novanet/core/types';

// =============================================================================
// Node & Edge Types - MUST be defined outside component for performance
// =============================================================================

const nodeTypes = {
  // Data mode node types
  turbo: TurboNode,
  structural: StructuralNode,
  localeKnowledge: LocaleKnowledgeNode,
  project: ProjectNode,
  // Schema mode node types (Task 3.2)
  scopeGroup: ScopeGroupNode,
  subcategoryGroup: SubcategoryGroupNode,
  schemaNode: SchemaNode,
} as const;

const edgeTypes = {
  floating: FloatingEdge,
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

  // Select node type based on category
  // Categories: project, content, locale, generation, seo, geo, analytics
  let nodeType: string = 'turbo';

  // Special case: Project nodes get premium social network card style
  if (node.type === 'Project') {
    nodeType = 'project';
  } else {
    switch (config.category) {
      case 'project':
      case 'content':
      case 'generation':
        nodeType = 'structural';
        break;
      case 'locale':
        // Locale node itself uses structural, knowledge nodes use localeKnowledge
        nodeType = ['LocaleIdentity', 'LocaleVoice', 'LocaleCulture', 'LocaleMarket', 'LocaleLexicon', 'Expression'].includes(node.type)
          ? 'localeKnowledge'
          : 'structural';
        break;
      // Default to turbo for other categories (seo, geo, analytics)
    }
  }

  return {
    id: node.id,
    type: nodeType,
    position: { x: 0, y: 0 }, // Will be set by dagre layout
    data: {
      id: node.id,
      type: node.type,
      key: node.key,
      displayName: node.displayName,
      icon: nodeTypeConfigs[node.type]?.icon,
      description: node.description,
      category: config.category,
    },
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
    clearSelection,
    sidebarOpen,
    focusMode,
    dataMode, // Schema mode toggle (Task 3.2)
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
      clearSelection: state.clearSelection,
      sidebarOpen: state.sidebarOpen,
      focusMode: state.focusMode,
      dataMode: state.dataMode, // Schema mode toggle (Task 3.2)
    }))
  );

  // Filter store - collapsed state for schema mode (Task 3.2)
  const { collapsedScopes, collapsedSubcategories } = useFilterStore(
    useShallow((state) => ({
      collapsedScopes: state.collapsedScopes,
      collapsedSubcategories: state.collapsedSubcategories,
    }))
  );

  // Focus mode for selection-based dimming
  const { isNodeDimmed, isEdgeDimmed, selectedId: focusSelectedId, connectedIds } = useFocusMode(graphEdges);

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
  // prevHasSelectionRef removed - no longer tracking selection close for fitView
  const prevLayoutVersionRef = useRef(layoutVersion);

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
  // SCHEMA MODE STATE (Task 3.2)
  // =========================================================================
  // Schema mode displays the ontology hierarchy (35 node types) instead of
  // real data instances. Uses ELK layout for hierarchical grouped visualization.
  // =========================================================================
  const [schemaNodes, setSchemaNodes] = useState<ReactFlowNode[]>([]);
  const [schemaEdges, setSchemaEdges] = useState<ReactFlowEdge[]>([]);
  const [isSchemaLayouting, setIsSchemaLayouting] = useState(false);
  const [schemaLayoutError, setSchemaLayoutError] = useState<Error | null>(null);

  // Track previous dataMode to detect changes
  const prevDataModeRef = useRef(dataMode);

  // PERF: Ref for schemaNodes to avoid callback re-creation during drag
  // Callbacks use ref.current to always get latest nodes without re-running
  const schemaNodesRef = useRef(schemaNodes);
  schemaNodesRef.current = schemaNodes;

  // Z-index management for schema mode (must be before handleSchemaNodeClick)
  const {
    bringToFront: bringSchemaNodeToFront,
    setHoverZIndex: setSchemaHoverZIndex,
    resetZIndex: resetSchemaZIndex,
    bringEdgeNodesToFront: bringSchemaEdgeNodesToFront,
  } = useGraphInteractions({ setNodes: setSchemaNodes });

  // Load schema graph with ELK layout and collapsed state filtering
  // Responds to layoutDirection and layoutVersion changes (like data mode)
  const loadSchemaGraph = useCallback(async () => {
    setIsSchemaLayouting(true);
    setSchemaLayoutError(null);

    try {
      const hierarchy = getSchemaHierarchy();
      const { nodes: layoutedNodes, edges: layoutedEdges } = await applySchemaLayout(hierarchy, layoutDirection);

      // Apply collapsed state filtering (Task 3.2)
      // Build lookup maps for parent relationships
      const nodeMap = new Map(layoutedNodes.map((n) => [n.id, n]));

      const visibleNodes = layoutedNodes.filter((node) => {
        // Check if scope is collapsed
        if (node.type === 'scopeGroup') {
          const scope = node.data?.scope as Scope | undefined;
          return scope ? !collapsedScopes.includes(scope) : true;
        }

        // Check if subcategory is collapsed
        if (node.type === 'subcategoryGroup') {
          // Check parent scope first
          const parentScope = nodeMap.get(node.parentId as string);
          const parentScopeData = parentScope?.data?.scope as Scope | undefined;
          if (parentScopeData && collapsedScopes.includes(parentScopeData)) {
            return false;
          }
          const nodeScope = node.data?.scope as string | undefined;
          const nodeSubcat = node.data?.subcategory as string | undefined;
          if (nodeScope && nodeSubcat) {
            const key = `${nodeScope}-${nodeSubcat}`;
            return !collapsedSubcategories.includes(key);
          }
          return true;
        }

        // Schema nodes: check both parent subcategory and grandparent scope
        if (node.type === 'schemaNode') {
          const parentSubcat = nodeMap.get(node.parentId as string);
          if (!parentSubcat) return true;

          // Check grandparent scope
          const grandparentScope = nodeMap.get(parentSubcat.parentId as string);
          const grandparentScopeData = grandparentScope?.data?.scope as Scope | undefined;
          if (grandparentScopeData && collapsedScopes.includes(grandparentScopeData)) {
            return false;
          }

          // Check parent subcategory
          const parentScope = parentSubcat.data?.scope as string | undefined;
          const parentSubcatName = parentSubcat.data?.subcategory as string | undefined;
          if (parentScope && parentSubcatName) {
            const key = `${parentScope}-${parentSubcatName}`;
            if (collapsedSubcategories.includes(key)) {
              return false;
            }
          }
        }

        return true;
      });

      // Filter edges to only include those with visible source and target
      const visibleNodeIds = new Set(visibleNodes.map((n) => n.id));
      const visibleEdges = layoutedEdges.filter(
        (edge) => visibleNodeIds.has(edge.source) && visibleNodeIds.has(edge.target)
      );

      // Apply initial z-index based on node type and scope
      // Hierarchy: Shared(10) < Global(20) < Project(30) < Nodes(1000)
      const nodesWithZIndex = visibleNodes.map((node) => {
        let zIndex: number = Z_INDEX.BASE;

        // Scope containers: scope-{Scope}
        if (node.id.startsWith('scope-')) {
          const scope = node.id.replace('scope-', '');
          if (scope === 'Shared') zIndex = Z_INDEX.SCOPE_SHARED;
          else if (scope === 'Global') zIndex = Z_INDEX.SCOPE_GLOBAL;
          else if (scope === 'Project') zIndex = Z_INDEX.SCOPE_PROJECT;
        }
        // Subcategory containers: subcat-{Scope}-{SubcategoryName}
        else if (node.id.startsWith('subcat-')) {
          const parts = node.id.replace('subcat-', '').split('-');
          const scope = parts[0];
          if (scope === 'Shared') zIndex = Z_INDEX.SUBCAT_SHARED;
          else if (scope === 'Global') zIndex = Z_INDEX.SUBCAT_GLOBAL;
          else if (scope === 'Project') zIndex = Z_INDEX.SUBCAT_PROJECT;
        }

        return { ...node, zIndex };
      });

      setSchemaNodes(nodesWithZIndex);
      setSchemaEdges(visibleEdges);
    } catch (error) {
      console.error('[Graph2D] Schema layout failed:', error);
      setSchemaLayoutError(error as Error);
    } finally {
      setIsSchemaLayouting(false);
    }
  }, [collapsedScopes, collapsedSubcategories, layoutDirection]);

  // Load schema graph when:
  // - dataMode changes to 'schema'
  // - collapsed state changes (via loadSchemaGraph dependency)
  // - layout direction changes (via loadSchemaGraph dependency)
  // - layoutVersion changes (user clicked layout button)
  useEffect(() => {
    if (dataMode === 'schema') {
      loadSchemaGraph();
    }
    prevDataModeRef.current = dataMode;
    // eslint-disable-next-line react-hooks/exhaustive-deps -- layoutVersion forces re-layout on button click
  }, [dataMode, loadSchemaGraph, layoutVersion]);

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
    // eslint-disable-next-line react-hooks/exhaustive-deps -- setState callbacks are stable
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
    // eslint-disable-next-line react-hooks/exhaustive-deps -- schemaNodesRef.current provides latest nodes
    [containerHandleNodeDrag]
  );

  // Schema node drag stop handler - triggers container shrinking
  // PERF: Uses schemaNodesRef to avoid re-creating callback on every node move
  const handleSchemaNodeDragStop = useCallback(
    (_event: React.MouseEvent, node: ReactFlowNode) => {
      containerHandleNodeDragStop(node, schemaNodesRef.current, setSchemaNodes);
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps -- schemaNodesRef.current provides latest nodes
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

      // Bring clicked node to front (z-index)
      bringSchemaNodeToFront(node.id);

      // Check if this is a container node (scope or subcategory group)
      const isContainer = node.id.startsWith('scope-') || node.id.startsWith('subcat-');

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

          // If this is a scope container, also include subcategory children
          if (node.id.startsWith('scope-')) {
            const subcatNodes = allNodes.filter(n => n.parentId === node.id);
            for (const subcat of subcatNodes) {
              const subcatChildren = allNodes.filter(n => n.parentId === subcat.id);
              containerAndChildren.push(...subcatChildren);
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
          const finalX = internalNode.internals.positionAbsolute.x;
          const finalY = internalNode.internals.positionAbsolute.y;

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

  const handleSchemaEdgeClick = useCallback(
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
      bringSchemaEdgeNodesToFront(edge);

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
    [setSelectedEdge, bringSchemaEdgeNodesToFront, getNodes, fitView]
  );

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
  }, [graphNodes, graphEdges, layoutDirection, layoutVersion]);

  // Step 2: Apply dimming state ONLY when focus/hover changes (cheap O(n) operation)
  // Priority: Selection focus mode > Hover highlight > Normal
  const initialNodes = useMemo(() => {
    return layoutedNodes.map((node) => {
      // Focus mode dimming takes precedence over hover dimming
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
  // eslint-disable-next-line react-hooks/exhaustive-deps -- focusSelectedId/hoveredId triggers recalc when selection/hover changes
  }, [layoutedNodes, isNodeDimmed, isNodeHoverDimmed, focusSelectedId, hoveredId]);

  // =========================================================================
  // EDGE DATA - Simplified
  // =========================================================================
  // Hover state (hovered, hoverDimmed) is computed LOCALLY in FloatingEdge via
  // direct Zustand store subscription. This bypasses React Flow's broken edge
  // data update mechanism. Only focus-mode dimming still comes through data.
  // =========================================================================
  // Build set of visible node IDs for edge validation
  const visibleNodeIdSet = useMemo(() => {
    return new Set(layoutedNodes.map((n) => n.id));
  }, [layoutedNodes]);

  const initialEdges = useMemo(() => {
    // DEFENSIVE: Only include edges where both source and target nodes exist
    return graphEdges
      .filter((e) => visibleNodeIdSet.has(e.source) && visibleNodeIdSet.has(e.target))
      .map((e) => {
        const dimmed = isEdgeDimmed(e.source, e.target);

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
          },
        };
      });
  }, [graphEdges, visibleNodeIdSet, isEdgeDimmed, showEdgeLabels]);

  // React Flow state
  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

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
    // Color by scope for schema nodes
    const scope = node.data?.scope;
    if (scope === 'Project') return '#8b5cf6cc'; // violet
    if (scope === 'Global') return '#10b981cc'; // emerald
    if (scope === 'Shared') return '#f59e0bcc'; // amber
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
    setSelectedNode(null);
    setContextMenu(null);
    onPaneClick?.();
  }, [setSelectedNode, onPaneClick]);

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
      if (event.key === 'Escape' && selectedNodeId) {
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
    // Direct deps (selectedNodeId, callbacks):
    // - These SHOULD trigger re-registration because handler behavior changes
  }, [selectedNodeId, setSelectedNode, expandNode, hideNode, clearSelection]);

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
  // When in schema mode, render the hierarchical schema visualization
  // with ELK layout and group nodes. Wrapped in SchemaErrorBoundary.
  // =========================================================================
  if (dataMode === 'schema') {
    return (
      <SchemaErrorBoundary>
        <div className={cn('h-full w-full', className)} data-testid="react-flow-wrapper-schema">
          <ReactFlow
            nodes={schemaNodes}
            edges={schemaEdges}
            nodeTypes={nodeTypes}
            edgeTypes={edgeTypes}
            onNodesChange={handleSchemaNodesChange}
            onNodeDrag={handleSchemaNodeDrag}
            onNodeDragStop={handleSchemaNodeDragStop}
            onNodeClick={handleSchemaNodeClick}
            onNodeMouseEnter={handleSchemaNodeMouseEnter}
            onNodeMouseLeave={handleSchemaNodeMouseLeave}
            onEdgeClick={handleSchemaEdgeClick}
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
        <EmptyState
          icon={Sparkles}
          title="No nodes to display"
          description="Try adjusting your filters or fetching data from Neo4j. Use the preset shortcuts (1-9) to quickly filter."
          accentColor="accent-purple"
        />
      </div>
    );
  }

  return (
    <EdgeVisibilityProvider>
      <div className={cn('h-full w-full', className)} data-testid="react-flow-wrapper">
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
