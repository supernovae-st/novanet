'use client';

/* eslint-disable @typescript-eslint/no-explicit-any -- Three.js/ForceGraph library interop requires any casts */

/**
 * Graph3D Component - 3D Knowledge Graph Visualization
 *
 * Features:
 * - Force-directed 3D layout with react-force-graph-3d
 * - Custom node geometries by Layer classification (9 shapes)
 * - Materials by Trait (solid, wireframe, glass, emissive, points)
 * - Realm indicator rings (outline color)
 * - Particle effects on arcs by ArcFamily
 * - Interactive hover with scale animation
 * - Click-to-zoom camera animation
 * - Visual encoding legend
 */

import { memo, useCallback, useRef, useMemo, useState, useEffect } from 'react';
import dynamic from 'next/dynamic';
import * as THREE from 'three';
import { useShallow } from 'zustand/react/shallow';
import { cn } from '@/lib/utils';
import { useFilteredGraph } from '@/hooks';
import { useUIStore } from '@/stores/uiStore';
import { GraphEmptyState } from './GraphEmptyState';
import {
  transformGraphData,
  filterValidData,
  getLayerColor,
  getRealmColor,
  getArcParticleConfig,
  createStarfield,
  createEnhancedComposer,
  updateComposerSize,
  createCompositeNode,
  getBloomConfigForQuality,
  ArcLODManager,
  detectArcFamily,
  type ForceGraphNode,
  type ForceGraphLink,
  type BloomQualityLevel,
  type ArcFamily,
} from '@/lib/graph3d';
import type { Layer, Realm, Trait } from '@novanet/core/types';
import { Graph3DLegend } from './Graph3DLegend';
import { GRAPH_ANIMATION } from '@/config/layoutConfig';

// ─────────────────────────────────────────────────────────────────────────────
// Type Guards for ForceGraph Link Endpoints
// ─────────────────────────────────────────────────────────────────────────────

/**
 * Link endpoint can be string (before D3 simulation) or object (after simulation)
 */
type LinkEndpoint = string | { id?: string } | undefined | null;

/**
 * Safely extract node ID from a link endpoint.
 * D3 force simulation mutates source/target from string to object with id property.
 *
 * @param endpoint - Link source or target (string | { id?: string } | undefined | null)
 * @returns Node ID string, or empty string if invalid
 */
function getNodeIdFromLinkEndpoint(endpoint: LinkEndpoint): string {
  if (!endpoint) return '';
  if (typeof endpoint === 'string') return endpoint;
  if (typeof endpoint === 'object' && 'id' in endpoint) {
    return endpoint.id ?? '';
  }
  return '';
}

/**
 * Type guard to validate a ForceGraphLink has valid source and target IDs.
 * Returns true if both endpoints resolve to non-empty strings.
 */
function isValidForceGraphLink(
  link: ForceGraphLink | null | undefined
): link is ForceGraphLink & { source: LinkEndpoint; target: LinkEndpoint } {
  if (!link) return false;
  const sourceId = getNodeIdFromLinkEndpoint(link.source as LinkEndpoint);
  const targetId = getNodeIdFromLinkEndpoint(link.target as LinkEndpoint);
  return sourceId !== '' && targetId !== '';
}

/**
 * Parse hex color string to integer with fallback.
 * Handles both '#RRGGBB' and 'RRGGBB' formats.
 *
 * @param color - Hex color string (e.g., '#60a5fa' or '60a5fa')
 * @param fallback - Fallback integer value if parsing fails (default: 0x60a5fa)
 * @returns Parsed integer or fallback
 */
function parseHexColor(color: string, fallback: number = 0x60a5fa): number {
  if (!color || typeof color !== 'string') return fallback;

  // Remove leading # if present
  const hex = color.startsWith('#') ? color.slice(1) : color;

  // Validate hex format (3, 6, or 8 characters)
  if (!/^[0-9a-fA-F]{3}$|^[0-9a-fA-F]{6}$|^[0-9a-fA-F]{8}$/.test(hex)) {
    return fallback;
  }

  // Expand 3-char hex to 6-char
  const fullHex = hex.length === 3
    ? hex.split('').map(c => c + c).join('')
    : hex.slice(0, 6); // Ignore alpha channel if 8-char

  const parsed = parseInt(fullHex, 16);
  return Number.isNaN(parsed) ? fallback : parsed;
}

// Layer Z-axis positions for visual separation
const LAYER_Z_POSITIONS: Record<string, number> = {
  config: 0,
  locale: 30,
  geography: 60,
  knowledge: 90,
  foundation: 130,
  structure: 170,
  semantic: 210,
  instruction: 250,
  output: 290,
};

// Realm X-axis offsets
const REALM_X_OFFSETS: Record<string, number> = {
  shared: -60,
  org: 60,
};

// Force graph ref type (minimal typing for dynamic import)
interface ForceGraphMethods {
  cameraPosition: (
    position?: { x: number; y: number; z: number },
    lookAt?: { x: number; y: number; z: number },
    transitionMs?: number
  ) => { x: number; y: number; z: number };
  scene: () => THREE.Scene;
  camera: () => THREE.Camera;
  renderer: () => THREE.WebGLRenderer;
}

// Dynamically import ForceGraph3D to avoid SSR issues
const ForceGraph3D = dynamic(() => import('react-force-graph-3d'), {
  ssr: false,
  loading: () => (
    <div className="absolute inset-0 flex items-center justify-center bg-slate-900">
      <div className="flex flex-col items-center gap-4">
        <div className="w-8 h-8 border-2 border-white/20 border-t-white/60 rounded-full animate-spin" />
        <p className="text-white/50 text-sm">Loading 3D graph...</p>
      </div>
    </div>
  ),
});

export interface Graph3DProps {
  /** Additional class names */
  className?: string;
  /** Show minimap */
  showMinimap?: boolean;
  /** Show controls */
  showControls?: boolean;
  /** Show legend */
  showLegend?: boolean;
  /** Callback when a node is clicked */
  onNodeClick?: (nodeId: string) => void;
  /** Callback when a node is double-clicked (expand) */
  onNodeDoubleClick?: (nodeId: string) => void;
  /** Callback when an edge is clicked */
  onEdgeClick?: (edgeId: string) => void;
  /** Callback when background is clicked */
  onPaneClick?: () => void;
}

export const Graph3D = memo(function Graph3D({
  className,
  showLegend = true,
  onNodeClick,
  onNodeDoubleClick,
  onEdgeClick,
  onPaneClick,
}: Graph3DProps) {
  const fgRef = useRef<ForceGraphMethods | null>(null);
  const containerRef = useRef<HTMLDivElement | null>(null);
  const starfieldRef = useRef<THREE.Points | null>(null);
  const composerRef = useRef<ReturnType<typeof createEnhancedComposer> | null>(null);
  const arcLODManagerRef = useRef<ArcLODManager | null>(null);

  // Instance-level caches (not global) to prevent memory leaks
  const compositeNodeCacheRef = useRef(new Map<string, THREE.Group>());
  const hoverScalesRef = useRef(new Map<string, number>());
  // Track previous selection to detect when aside panel closes (selection cleared)
  const prevHasSelectionRef = useRef<boolean>(false);

  // Cached geometries and materials to avoid allocation per render
  const selectionGlowGeometryRef = useRef<THREE.SphereGeometry | null>(null);
  const selectionGlowMaterialRef = useRef<THREE.MeshBasicMaterial | null>(null);
  const hoverGlowGeometryRef = useRef<THREE.SphereGeometry | null>(null);
  const hoverGlowMaterialsRef = useRef(new Map<string, THREE.MeshBasicMaterial>());
  // Cached particle geometries and materials (keyed by color)
  const particleGeometryRef = useRef<THREE.SphereGeometry | null>(null);
  const particleMaterialsRef = useRef(new Map<string, THREE.MeshStandardMaterial>());
  const [legendCollapsed, setLegendCollapsed] = useState(false);
  const [showHelp, setShowHelp] = useState(false);
  const [isGraphReady, setIsGraphReady] = useState(false);
  const [neighborIds, setNeighborIds] = useState<Set<string>>(new Set());
  const [highlightedLinks, setHighlightedLinks] = useState<Set<string>>(new Set());
  const [bootPhase, setBootPhase] = useState<'loading' | 'spawning' | 'ready'>('loading');
  const [selectionBurst, setSelectionBurst] = useState<string | null>(null);

  // Get filtered graph data
  const { nodes, edges } = useFilteredGraph();

  // UI store for selection - combined into 2 subscriptions instead of 6
  // State selector with useShallow for object comparison (re-renders only when values change)
  const { selectedNodeId, selectedEdgeId, hoveredNodeId } = useUIStore(
    useShallow((state) => ({
      selectedNodeId: state.selectedNodeId,
      selectedEdgeId: state.selectedEdgeId,
      hoveredNodeId: state.hoveredNodeId,
    }))
  );
  // Actions selector (stable reference - actions never change, useShallow ensures stable object)
  const { setSelectedNode, setHoveredNode, setSelectedEdge, setHoveredEdge } = useUIStore(
    useShallow((state) => ({
      setSelectedNode: state.setSelectedNode,
      setHoveredNode: state.setHoveredNode,
      setSelectedEdge: state.setSelectedEdge,
      setHoveredEdge: state.setHoveredEdge,
    }))
  );

  // Bloom quality from store (persisted user preference)
  const bloomQuality = useUIStore((state) => state.bloomQuality) as BloomQualityLevel;

  // Transform data for force graph
  const graphData = useMemo(() => {
    const transformed = transformGraphData(nodes, edges);
    return filterValidData(transformed);
  }, [nodes, edges]);

  // Clear global caches when graph data changes to prevent stale references
  useEffect(() => {
    // Get current node IDs
    const currentNodeIds = new Set(graphData.nodes.map((n) => n.id));

    // Remove cached meshes for nodes that no longer exist
    const cache = compositeNodeCacheRef.current;
    for (const nodeId of cache.keys()) {
      if (!currentNodeIds.has(nodeId)) {
        const mesh = cache.get(nodeId);
        if (mesh) {
          mesh.traverse((child) => {
            if (child instanceof THREE.Mesh) {
              child.geometry?.dispose();
              if (child.material instanceof THREE.Material) {
                child.material.dispose();
              }
            }
          });
        }
        cache.delete(nodeId);
      }
    }

    // Clear stale hover scales
    const scales = hoverScalesRef.current;
    for (const nodeId of scales.keys()) {
      if (!currentNodeIds.has(nodeId)) {
        scales.delete(nodeId);
      }
    }
  }, [graphData.nodes]);

  // Cleanup ALL cached meshes on unmount to prevent memory leaks
  useEffect(() => {
    const cacheRef = compositeNodeCacheRef;

    return () => {
      // Dispose all cached geometries and materials
      for (const mesh of cacheRef.current.values()) {
        mesh.traverse((child) => {
          if (child instanceof THREE.Mesh) {
            child.geometry?.dispose();
            if (child.material instanceof THREE.Material) {
              child.material.dispose();
            }
          }
        });
      }
      cacheRef.current.clear();
      hoverScalesRef.current.clear();

      // Dispose cached glow geometries and materials
      selectionGlowGeometryRef.current?.dispose();
      selectionGlowMaterialRef.current?.dispose();
      hoverGlowGeometryRef.current?.dispose();
      for (const material of hoverGlowMaterialsRef.current.values()) {
        material.dispose();
      }
      hoverGlowMaterialsRef.current.clear();

      // Dispose cached particle geometries and materials
      particleGeometryRef.current?.dispose();
      for (const material of particleMaterialsRef.current.values()) {
        material.dispose();
      }
      particleMaterialsRef.current.clear();
    };
  }, []);

  // Add starfield to scene when graph is ready
  useEffect(() => {
    if (!isGraphReady || !fgRef.current) return;

    const scene = fgRef.current.scene?.();
    if (!scene) return;

    // Add starfield if not already added
    if (!starfieldRef.current) {
      const starfield = createStarfield(800, 400);
      starfieldRef.current = starfield;
      scene.add(starfield);
    }

    return () => {
      if (starfieldRef.current && scene) {
        scene.remove(starfieldRef.current);
        starfieldRef.current = null;
      }
    };
  }, [isGraphReady]);

  // Initialize arc effects manager
  useEffect(() => {
    if (!isGraphReady || !fgRef.current) return;

    const scene = fgRef.current.scene?.();
    if (!scene) return;

    // Create manager if not exists
    if (!arcLODManagerRef.current) {
      arcLODManagerRef.current = new ArcLODManager();
      scene.add(arcLODManagerRef.current.getScene());
    }

    // Populate arcs
    const manager = arcLODManagerRef.current;
    manager.clear();

    graphData.links.forEach((link) => {
      if (!isValidForceGraphLink(link)) return;

      const sourceId = getNodeIdFromLinkEndpoint(link.source as LinkEndpoint);
      const targetId = getNodeIdFromLinkEndpoint(link.target as LinkEndpoint);
      const sourceNode = graphData.nodes.find(n => n.id === sourceId);
      const targetNode = graphData.nodes.find(n => n.id === targetId);

      if (!sourceNode || !targetNode) return;

      const sourcePos = new THREE.Vector3(
        sourceNode.x ?? 0,
        sourceNode.y ?? 0,
        sourceNode.z ?? 0
      );
      const targetPos = new THREE.Vector3(
        targetNode.x ?? 0,
        targetNode.y ?? 0,
        targetNode.z ?? 0
      );

      const family = detectArcFamily(link.type ?? '') as ArcFamily;
      manager.addArc(link.id, family, sourceId, targetId, sourcePos, targetPos);
    });

    return () => {
      if (arcLODManagerRef.current) {
        scene.remove(arcLODManagerRef.current.getScene());
        arcLODManagerRef.current.dispose();
        arcLODManagerRef.current = null;
      }
    };
  }, [isGraphReady, graphData.links, graphData.nodes]);

  // Continuous animation loop for shader uniforms (independent of D3 simulation)
  useEffect(() => {
    if (!isGraphReady || !fgRef.current || !arcLODManagerRef.current) return;

    let animationFrameId: number;
    let lastTime = performance.now();

    const animate = () => {
      try {
        const currentTime = performance.now();
        const time = currentTime * 0.001;
        const deltaTime = (currentTime - lastTime) * 0.001;
        lastTime = currentTime;

        // Update shader uniforms (animations)
        if (arcLODManagerRef.current && fgRef.current) {
          const camera = fgRef.current.camera?.();
          if (camera?.position) {
            arcLODManagerRef.current.update(camera, time, deltaTime);
          }
        }
      } catch (err) {
        console.warn('[Graph3D] Animation loop error:', err);
      }

      animationFrameId = requestAnimationFrame(animate);
    };

    // Start animation loop
    animationFrameId = requestAnimationFrame(animate);

    return () => {
      cancelAnimationFrame(animationFrameId);
    };
  }, [isGraphReady]);

  // Initialize post-processing bloom with WebGL context loss handling
  // Re-runs when bloomQuality or node count changes significantly
  useEffect(() => {
    if (!isGraphReady || !fgRef.current) return;

    const renderer = (fgRef.current as any).renderer?.();
    const scene = fgRef.current.scene?.();
    const camera = fgRef.current.camera?.();

    if (!renderer || !scene || !camera) return;

    // Get canvas for WebGL context event listeners
    const canvas = renderer.domElement;

    // Calculate adaptive bloom config based on quality setting and node count
    const nodeCount = graphData.nodes.length;
    const bloomConfig = getBloomConfigForQuality(bloomQuality, nodeCount);

    // Log performance decision for debugging
    if (bloomConfig === null) {
      console.info('[Graph3D] Bloom disabled (quality: %s, nodes: %d)', bloomQuality, nodeCount);
    } else {
      console.info('[Graph3D] Bloom enabled (quality: %s, nodes: %d, strength: %.1f)',
        bloomQuality, nodeCount, bloomConfig.strength);
    }

    // WebGL context loss handler — graceful degradation
    const handleContextLost = (event: Event) => {
      event.preventDefault(); // Allow context to be restored
      console.warn('[Graph3D] WebGL context lost — pausing rendering');

      // Dispose composer to free resources
      if (composerRef.current) {
        composerRef.current.dispose();
        composerRef.current = null;
      }
    };

    // WebGL context restore handler — recreate resources
    const handleContextRestored = () => {
      console.info('[Graph3D] WebGL context restored — recreating resources');

      // Recreate composer after context restore (if bloom is enabled)
      if (bloomConfig) {
        const newComposer = createEnhancedComposer(renderer, scene, camera, bloomConfig, {
          offset: 0.5,
          darkness: 0.4,
        });
        composerRef.current = newComposer;
      }

      // Clear caches - cached meshes have invalid GPU handles after restore
      compositeNodeCacheRef.current.clear();
    };

    canvas.addEventListener('webglcontextlost', handleContextLost);
    canvas.addEventListener('webglcontextrestored', handleContextRestored);

    // Only create composer if bloom is enabled
    if (bloomConfig) {
      // Create enhanced composer (bloom + vignette) with adaptive config
      const composer = createEnhancedComposer(renderer, scene, camera, bloomConfig, {
        offset: 0.5,        // Cinematic vignette
        darkness: 0.4,      // Darker edges for depth
      });
      composerRef.current = composer;
    } else {
      // Bloom disabled - ensure no stale composer
      if (composerRef.current) {
        composerRef.current.dispose();
        composerRef.current = null;
      }
    }

    // Handle resize
    const handleResize = () => {
      if (composerRef.current) {
        updateComposerSize(composerRef.current, window.innerWidth, window.innerHeight);
      }
    };
    window.addEventListener('resize', handleResize);

    return () => {
      canvas.removeEventListener('webglcontextlost', handleContextLost);
      canvas.removeEventListener('webglcontextrestored', handleContextRestored);
      window.removeEventListener('resize', handleResize);
      // Dispose the composer and its passes to prevent GPU memory leaks
      if (composerRef.current) {
        composerRef.current.dispose();
      }
      composerRef.current = null;
    };
  }, [isGraphReady, bloomQuality, graphData.nodes.length]);

  // Configure camera controls for constrained orbit
  useEffect(() => {
    if (!isGraphReady || !fgRef.current) return;

    const controls = (fgRef.current as any).controls?.();
    if (!controls) return;

    // Constrain orbit to prevent disorientation
    controls.minPolarAngle = Math.PI * 0.15;  // Don't go fully overhead
    controls.maxPolarAngle = Math.PI * 0.85;  // Don't go fully underneath
    controls.minDistance = 100;                // Prevent clipping into larger nodes
    controls.maxDistance = 1200;               // Allow zooming out for large spread
    controls.enableDamping = true;             // Smooth deceleration
    controls.dampingFactor = 0.06;             // Smoother damping

    // Cleanup: dispose OrbitControls to prevent stale event listeners
    return () => {
      controls.dispose?.();
    };
  }, [isGraphReady]);

  // ResizeObserver to handle container size changes (e.g., when detail panel opens/closes)
  // This fixes the centering issue where Three.js doesn't detect flex layout changes
  useEffect(() => {
    if (!containerRef.current || !fgRef.current) return;

    const resizeObserver = new ResizeObserver((entries) => {
      for (const entry of entries) {
        const { width, height } = entry.contentRect;
        if (width === 0 || height === 0) continue;

        // Force ForceGraph3D to resize its renderer
        const renderer = fgRef.current?.renderer?.();
        const camera = fgRef.current?.camera?.() as THREE.PerspectiveCamera | undefined;

        if (renderer && camera) {
          renderer.setSize(width, height);
          camera.aspect = width / height;
          camera.updateProjectionMatrix();

          // Also update post-processing composer if active
          if (composerRef.current) {
            updateComposerSize(composerRef.current, width, height);
          }
        }
      }
    });

    resizeObserver.observe(containerRef.current);

    return () => {
      resizeObserver.disconnect();
    };
  }, [isGraphReady]);

  // Configure D3 forces for layer/realm positioning
  useEffect(() => {
    if (!isGraphReady || !fgRef.current) return;

    const fg = fgRef.current as any;

    // VERY strong charge repulsion for hyperspace spread
    fg.d3Force?.('charge')?.strength?.(-500);

    // Add layer Z-positioning force
    fg.d3Force?.('z')?.strength?.(0.4);

    // Large link distance for breathing room between big nodes
    fg.d3Force?.('link')?.distance?.(120);

    // Reheat simulation to apply new forces
    fg.d3ReheatSimulation?.();
  }, [isGraphReady]);

  // Boot phase - skip animation, go directly to ready
  useEffect(() => {
    if (!isGraphReady || bootPhase !== 'loading') return;
    setBootPhase('ready');
  }, [isGraphReady, bootPhase]);

  // Compute neighbors and highlighted links when selection changes
  useEffect(() => {
    if (!selectedNodeId) {
      setNeighborIds(new Set());
      setHighlightedLinks(new Set());
      return;
    }

    const neighbors = new Set<string>();
    const links = new Set<string>();

    graphData.links.forEach((link) => {
      // Use type guard to validate link has valid source/target
      if (!isValidForceGraphLink(link)) return;

      // Use helper to extract IDs safely from either string or object form
      const sourceId = getNodeIdFromLinkEndpoint(link.source as LinkEndpoint);
      const targetId = getNodeIdFromLinkEndpoint(link.target as LinkEndpoint);

      if (sourceId === selectedNodeId || targetId === selectedNodeId) {
        neighbors.add(sourceId === selectedNodeId ? targetId : sourceId);
        links.add(`${sourceId}-${targetId}`);
      }
    });

    setNeighborIds(neighbors);
    setHighlightedLinks(links);
  }, [selectedNodeId, graphData.links]);

  // ==========================================================================
  // CAMERA CONTROLS
  // ==========================================================================

  // Calculate bounding box of all nodes for zoomToFit
  const calculateNodeBounds = useCallback(() => {
    const nodes = graphData.nodes;
    if (nodes.length === 0) return null;

    let minX = Infinity, maxX = -Infinity;
    let minY = Infinity, maxY = -Infinity;
    let minZ = Infinity, maxZ = -Infinity;

    for (const node of nodes) {
      const x = typeof node.x === 'number' ? node.x : 0;
      const y = typeof node.y === 'number' ? node.y : 0;
      const z = typeof node.z === 'number' ? node.z : 0;

      minX = Math.min(minX, x);
      maxX = Math.max(maxX, x);
      minY = Math.min(minY, y);
      maxY = Math.max(maxY, y);
      minZ = Math.min(minZ, z);
      maxZ = Math.max(maxZ, z);
    }

    // Center of bounding box
    const center = {
      x: (minX + maxX) / 2,
      y: (minY + maxY) / 2,
      z: (minZ + maxZ) / 2,
    };

    // Size of bounding box (diagonal)
    const size = Math.sqrt(
      Math.pow(maxX - minX, 2) +
      Math.pow(maxY - minY, 2) +
      Math.pow(maxZ - minZ, 2)
    );

    return { center, size, bounds: { minX, maxX, minY, maxY, minZ, maxZ } };
  }, [graphData.nodes]);

  // Zoom to fit all nodes in view with smooth animation
  const zoomToFit = useCallback((duration: number = GRAPH_ANIMATION.FIT_VIEW_DURATION_3D) => {
    if (!fgRef.current?.cameraPosition) return;

    const bounds = calculateNodeBounds();
    if (!bounds) return;

    const { center, size } = bounds;

    // Calculate camera distance based on graph size (with padding)
    const distance = Math.max(size * 1.5, GRAPH_ANIMATION.EDGE_FOCUS_DISTANCE_3D);

    // Isometric-style camera position
    const cameraPos = {
      x: center.x + distance * 0.6,
      y: center.y + distance * 0.5,
      z: center.z + distance * 0.6,
    };

    // Animate camera with smooth easing
    fgRef.current.cameraPosition(cameraPos, center, duration);
  }, [calculateNodeBounds]);

  // Camera preset views — now uses calculated bounds for 'reset'
  const setCameraView = useCallback((view: 'top' | 'front' | 'side' | 'reset') => {
    if (!fgRef.current?.cameraPosition) return;

    // For reset, use zoomToFit to center on actual graph content
    if (view === 'reset') {
      zoomToFit(GRAPH_ANIMATION.RESET_DURATION_3D);  // Fast animation for responsiveness
      return;
    }

    // Calculate bounds for other views too
    const bounds = calculateNodeBounds();
    const center = bounds?.center ?? { x: 0, y: 0, z: 0 };
    const distance = bounds ? Math.max(bounds.size * 1.2, 300) : 500;

    const positions: Record<string, { pos: { x: number; y: number; z: number } }> = {
      top: { pos: { x: center.x, y: center.y + distance, z: center.z } },
      front: { pos: { x: center.x, y: center.y + 80, z: center.z + distance } },
      side: { pos: { x: center.x + distance, y: center.y + 80, z: center.z } },
    };

    const { pos } = positions[view];
    fgRef.current.cameraPosition(pos, center, GRAPH_ANIMATION.NODE_FOCUS_DURATION_3D);
  }, [calculateNodeBounds, zoomToFit]);

  // Auto-zoom to fit when aside panel closes (selection cleared)
  // This provides a better UX when closing the detail panel
  useEffect(() => {
    const hasSelection = selectedNodeId !== null || selectedEdgeId !== null;

    // If selection was present and is now cleared, reset camera to fit all nodes
    if (prevHasSelectionRef.current && !hasSelection) {
      // Small delay to let the panel animation complete
      const timerId = setTimeout(() => {
        setCameraView('reset');
      }, 100);
      return () => clearTimeout(timerId);
    }

    // Update ref for next render
    prevHasSelectionRef.current = hasSelection;
  }, [selectedNodeId, selectedEdgeId, setCameraView]);

  // Keyboard shortcuts handler
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Don't trigger if typing in input
      if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return;

      switch (e.key.toLowerCase()) {
        case '?':
        case 'h':
          e.preventDefault();
          setShowHelp(prev => !prev);
          break;
        case 'l':
          e.preventDefault();
          setLegendCollapsed(prev => !prev);
          break;
        case 't':
          e.preventDefault();
          setCameraView('top');
          break;
        case 'f':
          e.preventDefault();
          setCameraView('front');
          break;
        case 's':
          e.preventDefault();
          setCameraView('side');
          break;
        case 'r':
          e.preventDefault();
          setCameraView('reset');
          break;
        case 'escape':
          setShowHelp(false);
          setSelectedNode(null);
          break;
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [setCameraView, setSelectedNode]);

  // Calculate node opacity based on selection context
  const getNodeOpacity = useCallback((nodeId: string): number => {
    if (!selectedNodeId) return 1.0;  // No selection = all visible
    if (nodeId === selectedNodeId) return 1.0;  // Selected = full
    if (neighborIds.has(nodeId)) return 0.7;  // Neighbors = visible
    return 0.15;  // Others = ghosted
  }, [selectedNodeId, neighborIds]);

  // Calculate node scale based on selection context
  const getNodeScale = useCallback((nodeId: string): number => {
    if (!selectedNodeId) return 1.0;
    if (nodeId === selectedNodeId) return 1.3;  // Selected = larger
    if (neighborIds.has(nodeId)) return 1.0;  // Neighbors = normal
    return 0.7;  // Others = smaller
  }, [selectedNodeId, neighborIds]);

  // Custom node rendering with Three.js — ATOMIC/GALAXY STYLE
  // Core + Orbital Rings + Particle Cloud
  const renderNode = useCallback((node: ForceGraphNode) => {
    // Get colors
    const layerColor = getLayerColor(node.layer);
    const realmColor = getRealmColor(node.realm);

    // Use pre-computed connection count (O(1) instead of O(m) filter per node)
    const connectionCount = node.connectionCount ?? 0;

    // Create composite node (Core + Rings + Particles)
    const composite = createCompositeNode({
      layer: node.layer as Layer,
      realm: node.realm as Realm,
      trait: node.trait as Trait,
      layerColor,
      realmColor,
      connectionCount,
      baseSize: 8,
    });

    const { group, core, rings, particles } = composite;

    // Apply focus+context
    const contextOpacity = getNodeOpacity(node.id);
    const contextScale = getNodeScale(node.id);
    const hoverScale = hoverScalesRef.current.get(node.id) || 1.0;

    // Scale the entire group
    group.scale.setScalar(contextScale * hoverScale);

    // Adjust opacity based on context
    if (core.material instanceof THREE.MeshPhysicalMaterial) {
      core.material.opacity = Math.min(core.material.opacity, contextOpacity);

      // Boost emissive for selected/burst nodes
      if (node.id === selectionBurst) {
        core.material.emissiveIntensity = 5.0;
      } else if (node.id === selectedNodeId) {
        core.material.emissiveIntensity = 2.5;
      }
    }

    // Adjust ring opacity
    rings.forEach((ring) => {
      if (ring.material instanceof THREE.MeshBasicMaterial) {
        ring.material.opacity *= contextOpacity;
      }
    });

    // Adjust particle opacity
    if (particles.material instanceof THREE.PointsMaterial) {
      particles.material.opacity *= contextOpacity;
    }

    // Add selection outer glow (using cached geometry/material)
    if (node.id === selectedNodeId) {
      // Lazy-create cached geometry
      if (!selectionGlowGeometryRef.current) {
        selectionGlowGeometryRef.current = new THREE.SphereGeometry(14, 16, 16);
      }
      if (!selectionGlowMaterialRef.current) {
        selectionGlowMaterialRef.current = new THREE.MeshBasicMaterial({
          color: 0xffffff,
          transparent: true,
          opacity: 0.12,
          side: THREE.BackSide,
          blending: THREE.AdditiveBlending,
          depthWrite: false,
        });
      }
      const glow = new THREE.Mesh(selectionGlowGeometryRef.current, selectionGlowMaterialRef.current);
      group.add(glow);
    }

    // Add hover highlight (using cached geometry, color-keyed materials)
    if (node.id === hoveredNodeId && node.id !== selectedNodeId) {
      // Lazy-create cached geometry
      if (!hoverGlowGeometryRef.current) {
        hoverGlowGeometryRef.current = new THREE.SphereGeometry(12, 12, 12);
      }
      // Get or create material for this layer color
      let hoverMaterial = hoverGlowMaterialsRef.current.get(layerColor);
      if (!hoverMaterial) {
        hoverMaterial = new THREE.MeshBasicMaterial({
          color: parseHexColor(layerColor),
          transparent: true,
          opacity: 0.08,
          side: THREE.BackSide,
          blending: THREE.AdditiveBlending,
          depthWrite: false,
        });
        hoverGlowMaterialsRef.current.set(layerColor, hoverMaterial);
      }
      const hoverGlow = new THREE.Mesh(hoverGlowGeometryRef.current, hoverMaterial);
      group.add(hoverGlow);
    }

    // Store reference for interaction
    group.userData = { nodeId: node.id, nodeType: node.type };

    return group;
  }, [selectedNodeId, hoveredNodeId, selectionBurst, getNodeOpacity, getNodeScale]);

  // Smooth camera zoom to selected node — larger distance for bigger nodes
  const zoomToNode = useCallback((node: ForceGraphNode | null | undefined) => {
    try {
      if (!fgRef.current?.cameraPosition || !node) return;

      // Skip zoom if node doesn't have valid coordinates yet
      // This can happen if D3 simulation hasn't positioned the node yet
      if (node.x === undefined && node.y === undefined && node.z === undefined) {
        console.debug('[Graph3D] Skipping zoom - node not positioned yet:', node.id);
        return;
      }

      // Extra defensive: ensure coordinates are numbers
      const nx = typeof node.x === 'number' ? node.x : 0;
      const ny = typeof node.y === 'number' ? node.y : 0;
      const nz = typeof node.z === 'number' ? node.z : 0;

      // Get node position - this is where the camera will look
      const nodePos = { x: nx, y: ny, z: nz };

      // Calculate camera position at fixed distance from node
      const distance = GRAPH_ANIMATION.NODE_FOCUS_DISTANCE_3D;
      const cameraPos = {
        x: nodePos.x + distance * 0.6,
        y: nodePos.y + distance * 0.4,
        z: nodePos.z + distance * 0.7,
      };

      // Smooth cinematic animation (unified timing with 2D)
      // Note: The canvas container handles panel offset via flex layout
      // No manual offset needed - camera looks directly at node position
      fgRef.current.cameraPosition(cameraPos, nodePos, GRAPH_ANIMATION.NODE_FOCUS_DURATION_3D);
    } catch (err) {
      console.warn('[Graph3D] zoomToNode error:', err);
    }
  }, []);

  // Node click handler with zoom (defensive null check)
  const handleNodeClick = useCallback(
    (node: ForceGraphNode | null | undefined) => {
      try {
        if (!node?.id) return;  // Defensive: node may be undefined from library

        setSelectedNode(node.id);
        zoomToNode(node);

        // Trigger selection burst effect
        setSelectionBurst(node.id);
        setTimeout(() => setSelectionBurst(null), 400);

        onNodeClick?.(node.id);
      } catch (err) {
        console.warn('[Graph3D] handleNodeClick error:', err);
      }
    },
    [setSelectedNode, zoomToNode, onNodeClick]
  );

  // Node double-click handler (defensive null check)
  const handleNodeDoubleClick = useCallback(
    (node: ForceGraphNode | null | undefined) => {
      if (!node?.id) return;
      onNodeDoubleClick?.(node.id);
    },
    [onNodeDoubleClick]
  );

  // ==========================================================================
  // CURSOR MANAGEMENT
  // ==========================================================================

  // Track if user is dragging (for cursor state)
  const isDraggingRef = useRef(false);

  // Update cursor based on current state
  const updateCursor = useCallback((hovering: boolean) => {
    if (typeof document === 'undefined') return;

    if (hovering) {
      document.body.style.cursor = 'pointer';
    } else if (isDraggingRef.current) {
      document.body.style.cursor = 'grabbing';
    } else {
      document.body.style.cursor = 'grab';
    }
  }, []);

  // Handle mouse down/up for grabbing cursor
  useEffect(() => {
    if (!containerRef.current) return;

    const container = containerRef.current;

    const handleMouseDown = (e: MouseEvent) => {
      // Only set grabbing if not clicking on a node (check hover state)
      const freshHover = useUIStore.getState().hoveredNodeId;
      if (!freshHover && e.button === 0) {
        isDraggingRef.current = true;
        document.body.style.cursor = 'grabbing';
      }
    };

    const handleMouseUp = () => {
      if (isDraggingRef.current) {
        isDraggingRef.current = false;
        const freshHover = useUIStore.getState().hoveredNodeId;
        updateCursor(!!freshHover);
      }
    };

    // Also handle mouse leave to reset cursor
    const handleMouseLeave = () => {
      isDraggingRef.current = false;
      document.body.style.cursor = 'default';
    };

    container.addEventListener('mousedown', handleMouseDown);
    window.addEventListener('mouseup', handleMouseUp);
    container.addEventListener('mouseleave', handleMouseLeave);

    return () => {
      container.removeEventListener('mousedown', handleMouseDown);
      window.removeEventListener('mouseup', handleMouseUp);
      container.removeEventListener('mouseleave', handleMouseLeave);
    };
  }, [updateCursor]);

  // Node hover handler with scale effect (defensive null checks)
  const handleNodeHover = useCallback(
    (node: ForceGraphNode | null | undefined, prevNode: ForceGraphNode | null | undefined) => {
      // Update hover scale (using ref)
      const scales = hoverScalesRef.current;
      if (prevNode?.id) {
        scales.set(prevNode.id, 1.0);
      }
      if (node?.id) {
        scales.set(node.id, 1.15);
      }

      setHoveredNode(node?.id ?? null);

      // Update cursor (only if not dragging)
      if (!isDraggingRef.current) {
        updateCursor(!!node);
      }
    },
    [setHoveredNode, updateCursor]
  );

  // Background click handler - deselect and reset view
  // Only triggers if we're not hovering a node (prevents conflict with onNodeClick)
  const handleBackgroundClick = useCallback(() => {
    // Guard: Read FRESH hover state directly from store (not stale closure)
    // This fixes the "double-click to select" bug in 3D view
    // React batches state updates, so captured hoveredNodeId may be stale
    const freshHoveredNodeId = useUIStore.getState().hoveredNodeId;
    if (freshHoveredNodeId) {
      return;
    }

    setSelectedNode(null);
    setSelectedEdge(null);
    setCameraView('reset');  // Reset camera to default isometric view
    onPaneClick?.();
  }, [setSelectedNode, setSelectedEdge, setCameraView, onPaneClick]);

  // Link click handler
  const handleLinkClick = useCallback(
    (link: ForceGraphLink) => {
      // Use helper to extract source/target IDs safely (handles string or object form)
      const sourceId = getNodeIdFromLinkEndpoint(link.source as LinkEndpoint);
      const targetId = getNodeIdFromLinkEndpoint(link.target as LinkEndpoint);

      // Clear node selection, set edge selection
      setSelectedNode(null);
      setSelectedEdge(link.id, {
        id: link.id,
        type: link.type ?? 'UNKNOWN',
        source: sourceId,
        target: targetId,
        data: { ...link },
      });

      // Zoom camera to midpoint between nodes (only if valid link)
      if (fgRef.current?.cameraPosition && isValidForceGraphLink(link)) {
        const sourceNode = graphData.nodes.find(n => n.id === sourceId);
        const targetNode = graphData.nodes.find(n => n.id === targetId);

        if (sourceNode && targetNode) {
          const midPoint = {
            x: ((sourceNode.x || 0) + (targetNode.x || 0)) / 2,
            y: ((sourceNode.y || 0) + (targetNode.y || 0)) / 2,
            z: ((sourceNode.z || 0) + (targetNode.z || 0)) / 2,
          };

          const distance = GRAPH_ANIMATION.EDGE_FOCUS_DISTANCE_3D;
          const cameraPos = {
            x: midPoint.x + distance * 0.6,
            y: midPoint.y + distance * 0.5,
            z: midPoint.z + distance * 0.7,
          };

          fgRef.current.cameraPosition(cameraPos, midPoint, GRAPH_ANIMATION.EDGE_FOCUS_DURATION_3D);
        }
      }

      onEdgeClick?.(link.id);
    },
    [setSelectedNode, setSelectedEdge, graphData.nodes, onEdgeClick]
  );

  // Link hover handler
  const handleLinkHover = useCallback(
    (link: ForceGraphLink | null) => {
      setHoveredEdge(link?.id ?? null);

      // Change cursor
      if (typeof document !== 'undefined') {
        document.body.style.cursor = link ? 'pointer' : 'grab';
      }
    },
    [setHoveredEdge]
  );

  // Engine tick to mark graph as ready and sync arc positions with D3 simulation
  const handleEngineTick = useCallback(() => {
    try {
      if (!isGraphReady) {
        setIsGraphReady(true);
      }

      // Update arc positions from D3 simulation (only during simulation, not for animations)
      if (arcLODManagerRef.current) {
        graphData.links.forEach((link) => {
          if (!isValidForceGraphLink(link)) return;

          const sourceId = getNodeIdFromLinkEndpoint(link.source as LinkEndpoint);
          const targetId = getNodeIdFromLinkEndpoint(link.target as LinkEndpoint);
          const sourceNode = graphData.nodes.find(n => n.id === sourceId);
          const targetNode = graphData.nodes.find(n => n.id === targetId);

          if (!sourceNode || !targetNode) return;

          // Extra defensive: ensure coordinates are numbers
          const sourcePos = new THREE.Vector3(
            typeof sourceNode.x === 'number' ? sourceNode.x : 0,
            typeof sourceNode.y === 'number' ? sourceNode.y : 0,
            typeof sourceNode.z === 'number' ? sourceNode.z : 0
          );
          const targetPos = new THREE.Vector3(
            typeof targetNode.x === 'number' ? targetNode.x : 0,
            typeof targetNode.y === 'number' ? targetNode.y : 0,
            typeof targetNode.z === 'number' ? targetNode.z : 0
          );

          arcLODManagerRef.current?.updateArcPositions(link.id, sourcePos, targetPos);
        });
      }
    } catch (err) {
      console.warn('[Graph3D] handleEngineTick error:', err);
    }
  }, [isGraphReady, graphData.links, graphData.nodes]);

  // Link styling callbacks - ULTRA DEFENSIVE
  const getLinkColor = useCallback((link: unknown) => {
    try {
      const l = link as ForceGraphLink | undefined;
      if (!l || typeof l !== 'object' || !('type' in l)) return '#1e3a5f';
      const config = getArcParticleConfig(String(l.type || ''));
      return config.linkColor;
    } catch {
      return '#1e3a5f';
    }
  }, []);

  const getLinkWidth = useCallback((link: unknown) => {
    try {
      const l = link as ForceGraphLink | undefined;
      if (!l || typeof l !== 'object') return 0.3;
      const config = getArcParticleConfig(String((l as any).type || ''));
      return config.linkWidth;
    } catch {
      return 0.3;
    }
  }, []);

  const getLinkOpacity = useCallback((link: unknown) => {
    try {
      const l = link as ForceGraphLink | undefined;
      if (!l || typeof l !== 'object') return 0.15;
      const config = getArcParticleConfig(String((l as any).type || ''));
      return config.linkOpacity;
    } catch {
      return 0.15;
    }
  }, []);

  const getLinkParticles = useCallback((_link: unknown) => {
    // Return fixed number of particles - simple and safe
    return 4;
  }, []);

  const getLinkParticleSpeed = useCallback((_link: unknown) => {
    return 0.003;  // Slow speed so particles are visible
  }, []);

  const getLinkParticleWidth = useCallback((_link: unknown) => {
    return 3;  // Fixed width
  }, []);

  const getLinkParticleColor = useCallback((link: unknown) => {
    try {
      const l = link as ForceGraphLink | undefined;
      if (!l || typeof l !== 'object') return '#60a5fa';
      const config = getArcParticleConfig(String((l as any).type || ''));
      return config.particleColor;
    } catch {
      return '#60a5fa';
    }
  }, []);

  const getLinkCurvature = useCallback((link: unknown) => {
    try {
      const l = link as ForceGraphLink | undefined;
      if (!l || typeof l !== 'object') return 0;
      const config = getArcParticleConfig(String((l as any).type || ''));
      return config.curvature;
    } catch {
      return 0;
    }
  }, []);

  // Custom emissive particle object for bloom compatibility (with geometry caching)
  const getParticleThreeObject = useCallback((link: unknown) => {
    try {
      const l = link as ForceGraphLink | undefined;
      const colorStr = l && typeof l === 'object' && 'type' in l
        ? getArcParticleConfig(String(l.type || '')).particleColor
        : '#60a5fa';

      // Lazy-create cached geometry (shared by all particles)
      if (!particleGeometryRef.current) {
        particleGeometryRef.current = new THREE.SphereGeometry(1.5, 12, 12);
      }

      // Get or create material for this color
      let material = particleMaterialsRef.current.get(colorStr);
      if (!material) {
        const color = new THREE.Color(colorStr);
        material = new THREE.MeshStandardMaterial({
          color: color,
          emissive: color,
          emissiveIntensity: 3.0,  // High intensity to exceed bloom threshold
          transparent: true,
          opacity: 0.9,
        });
        particleMaterialsRef.current.set(colorStr, material);
      }

      return new THREE.Mesh(particleGeometryRef.current, material);
    } catch {
      // Fallback: use cached geometry if available
      if (!particleGeometryRef.current) {
        particleGeometryRef.current = new THREE.SphereGeometry(1.5, 8, 8);
      }
      let fallbackMaterial = particleMaterialsRef.current.get('fallback');
      if (!fallbackMaterial) {
        fallbackMaterial = new THREE.MeshStandardMaterial({ color: 0x60a5fa }) as any;
        particleMaterialsRef.current.set('fallback', fallbackMaterial as THREE.MeshStandardMaterial);
      }
      return new THREE.Mesh(particleGeometryRef.current, fallbackMaterial);
    }
  }, []);

  // Empty state - context-aware diagnostics
  if (graphData.nodes.length === 0) {
    return (
      <div className={cn('h-full bg-slate-900', className)}>
        <GraphEmptyState />
      </div>
    );
  }

  return (
    <div ref={containerRef} className={cn('relative', className)}>
      <ForceGraph3D
         
        ref={fgRef as any}
        graphData={graphData as any}
        nodeThreeObject={renderNode as any}
        nodeLabel={((node: ForceGraphNode) =>
          `<div style="background: rgba(0,0,0,0.8); padding: 4px 8px; border-radius: 4px; font-size: 11px;">
            <strong>${node.name}</strong><br/>
            <span style="color: rgba(255,255,255,0.6)">${node.type} · ${node.layer}</span>
          </div>`) as any
        }
        nodeThreeObjectExtend={false}
        onNodeClick={handleNodeClick as any}
        onNodeRightClick={handleNodeDoubleClick as any}
        onNodeHover={handleNodeHover as any}
        onLinkClick={handleLinkClick as any}
        onLinkHover={handleLinkHover as any}
        onBackgroundClick={handleBackgroundClick}
        onEngineTick={handleEngineTick}
        // Custom arc effects - disable built-in link rendering
        linkDirectionalParticles={0}
        linkVisibility={false}
        nodeRelSize={8}
        backgroundColor="#050810"
        showNavInfo={false}
        enableNodeDrag={false}  // Disabled: DragControls crash on rapid clicks (Three.js bug)
        enableNavigationControls={true}
        controlType="orbit"
        warmupTicks={100}
        cooldownTicks={200}
        d3AlphaDecay={0.04}
        d3VelocityDecay={0.5}
      />

      {/* Legend */}
      {showLegend && (
        <Graph3DLegend
          collapsed={legendCollapsed}
          onToggle={() => setLegendCollapsed(!legendCollapsed)}
        />
      )}

      {/* Keyboard shortcuts hint */}
      <div className="absolute top-4 right-4 px-3 py-2 rounded-lg bg-white/10 backdrop-blur-md border border-white/20">
        <p className="text-xs text-white/80 font-medium">Press <kbd className="px-1.5 py-0.5 mx-1 bg-white/20 rounded text-white">?</kbd> for help</p>
      </div>

      {/* Help overlay */}
      {showHelp && (
        <div
          className="absolute inset-0 flex items-center justify-center bg-black/70 backdrop-blur-sm z-50"
          onClick={() => setShowHelp(false)}
        >
          <div
            className="bg-slate-900/95 border border-white/20 rounded-xl p-6 max-w-md shadow-2xl"
            onClick={(e) => e.stopPropagation()}
          >
            <h2 className="text-lg font-bold text-white mb-4 flex items-center gap-2">
              <span className="text-2xl">🎮</span> Keyboard Shortcuts
            </h2>
            <div className="space-y-3 text-sm">
              <div className="grid grid-cols-2 gap-x-6 gap-y-2">
                <div className="text-white/50">Navigation</div>
                <div></div>

                <kbd className="px-2 py-0.5 bg-white/10 rounded text-white/80 text-xs font-mono">T</kbd>
                <span className="text-white/70">Top view</span>

                <kbd className="px-2 py-0.5 bg-white/10 rounded text-white/80 text-xs font-mono">F</kbd>
                <span className="text-white/70">Front view</span>

                <kbd className="px-2 py-0.5 bg-white/10 rounded text-white/80 text-xs font-mono">S</kbd>
                <span className="text-white/70">Side view</span>

                <kbd className="px-2 py-0.5 bg-white/10 rounded text-white/80 text-xs font-mono">R</kbd>
                <span className="text-white/70">Reset view (isometric)</span>

                <div className="col-span-2 border-t border-white/10 my-2"></div>

                <div className="text-white/50">UI Controls</div>
                <div></div>

                <kbd className="px-2 py-0.5 bg-white/10 rounded text-white/80 text-xs font-mono">L</kbd>
                <span className="text-white/70">Toggle legend</span>

                <kbd className="px-2 py-0.5 bg-white/10 rounded text-white/80 text-xs font-mono">?</kbd>
                <span className="text-white/70">Show/hide help</span>

                <kbd className="px-2 py-0.5 bg-white/10 rounded text-white/80 text-xs font-mono">Esc</kbd>
                <span className="text-white/70">Deselect / Close</span>

                <div className="col-span-2 border-t border-white/10 my-2"></div>

                <div className="text-white/50">Mouse</div>
                <div></div>

                <span className="text-white/60">Drag</span>
                <span className="text-white/70">Rotate view</span>

                <span className="text-white/60">Scroll</span>
                <span className="text-white/70">Zoom in/out</span>

                <span className="text-white/60">Right-drag</span>
                <span className="text-white/70">Pan view</span>

                <span className="text-white/60">Click node</span>
                <span className="text-white/70">Select + zoom</span>
              </div>
            </div>
            <button
              onClick={() => setShowHelp(false)}
              className="mt-4 w-full py-2 bg-white/10 hover:bg-white/20 rounded-lg text-white/70 text-sm transition-colors"
            >
              Close (Esc)
            </button>
          </div>
        </div>
      )}
    </div>
  );
});

export default Graph3D;
