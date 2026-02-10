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
  TRAIT_GLOW_INTENSITY,
  type ForceGraphNode,
  type ForceGraphLink,
} from '@/lib/graph3d';
import type { Layer, Realm, Trait } from '@novanet/core/types';
import { Graph3DLegend } from './Graph3DLegend';

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

// Cache for composite node meshes
const compositeNodeCache = new Map<string, THREE.Group>();

// Hover state tracking
const hoverScales = new Map<string, number>();

export const Graph3D = memo(function Graph3D({
  className,
  showLegend = true,
  onNodeClick,
  onNodeDoubleClick,
  onEdgeClick,
  onPaneClick,
}: Graph3DProps) {
  const fgRef = useRef<ForceGraphMethods | null>(null);
  const starfieldRef = useRef<THREE.Points | null>(null);
  const composerRef = useRef<ReturnType<typeof createEnhancedComposer> | null>(null);
  const [legendCollapsed, setLegendCollapsed] = useState(false);
  const [showHelp, setShowHelp] = useState(false);
  const [isGraphReady, setIsGraphReady] = useState(false);
  const [neighborIds, setNeighborIds] = useState<Set<string>>(new Set());
  const [highlightedLinks, setHighlightedLinks] = useState<Set<string>>(new Set());
  const [bootPhase, setBootPhase] = useState<'loading' | 'spawning' | 'ready'>('loading');
  const [selectionBurst, setSelectionBurst] = useState<string | null>(null);

  // Get filtered graph data
  const { nodes, edges } = useFilteredGraph();

  // UI store for selection
  const selectedNodeId = useUIStore((state) => state.selectedNodeId);
  const hoveredNodeId = useUIStore((state) => state.hoveredNodeId);
  const setSelectedNode = useUIStore((state) => state.setSelectedNode);
  const setHoveredNode = useUIStore((state) => state.setHoveredNode);
  const setSelectedEdge = useUIStore((state) => state.setSelectedEdge);
  const setHoveredEdge = useUIStore((state) => state.setHoveredEdge);

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
    for (const nodeId of compositeNodeCache.keys()) {
      if (!currentNodeIds.has(nodeId)) {
        const mesh = compositeNodeCache.get(nodeId);
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
        compositeNodeCache.delete(nodeId);
      }
    }

    // Clear stale hover scales
    for (const nodeId of hoverScales.keys()) {
      if (!currentNodeIds.has(nodeId)) {
        hoverScales.delete(nodeId);
      }
    }
  }, [graphData.nodes]);

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

  // Initialize post-processing bloom
  useEffect(() => {
    if (!isGraphReady || !fgRef.current) return;

    const renderer = (fgRef.current as any).renderer?.();
    const scene = fgRef.current.scene?.();
    const camera = fgRef.current.camera?.();

    if (!renderer || !scene || !camera) return;

    // Create enhanced composer (bloom + vignette) — HYPERSPACE GLOW
    const composer = createEnhancedComposer(renderer, scene, camera, {
      strength: 1.8,      // Strong bloom for hyperspace glow
      radius: 0.6,        // Wide bloom spread
      threshold: 0.1,     // LOW threshold so particles glow (was 0.3)
    }, {
      offset: 0.5,        // Cinematic vignette
      darkness: 0.4,      // Darker edges for depth
    });
    composerRef.current = composer;

    // Handle resize
    const handleResize = () => {
      if (composerRef.current) {
        updateComposerSize(composerRef.current, window.innerWidth, window.innerHeight);
      }
    };
    window.addEventListener('resize', handleResize);

    return () => {
      window.removeEventListener('resize', handleResize);
      // Dispose the composer and its passes to prevent GPU memory leaks
      if (composerRef.current) {
        composerRef.current.dispose();
      }
      composerRef.current = null;
    };
  }, [isGraphReady]);

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
      if (!link) return;
      // source/target can be string (initial) or object (after d3 simulation)
      const source = link.source as string | { id: string } | undefined;
      const target = link.target as string | { id: string } | undefined;
      if (!source || !target) return;

      // Handle both string and object forms, with null safety for .id
      const sourceId = typeof source === 'object' ? (source.id ?? '') : source;
      const targetId = typeof target === 'object' ? (target.id ?? '') : target;
      if (!sourceId || !targetId) return;

      if (sourceId === selectedNodeId || targetId === selectedNodeId) {
        neighbors.add(sourceId === selectedNodeId ? targetId : sourceId);
        links.add(`${sourceId}-${targetId}`);
      }
    });

    setNeighborIds(neighbors);
    setHighlightedLinks(links);
  }, [selectedNodeId, graphData.links]);

  // Camera preset views — adjusted for larger graph spread
  const setCameraView = useCallback((view: 'top' | 'front' | 'side' | 'reset') => {
    if (!fgRef.current?.cameraPosition) return;

    const distance = 500;  // Increased for larger nodes/spread
    const positions: Record<string, { pos: { x: number; y: number; z: number }; lookAt: { x: number; y: number; z: number } }> = {
      top: { pos: { x: 0, y: distance, z: 0 }, lookAt: { x: 0, y: 0, z: 0 } },
      front: { pos: { x: 0, y: 80, z: distance }, lookAt: { x: 0, y: 0, z: 0 } },
      side: { pos: { x: distance, y: 80, z: 0 }, lookAt: { x: 0, y: 0, z: 0 } },
      reset: { pos: { x: distance * 0.6, y: distance * 0.4, z: distance * 0.7 }, lookAt: { x: 0, y: 0, z: 0 } },
    };

    const { pos, lookAt } = positions[view];
    fgRef.current.cameraPosition(pos, lookAt, 1200);  // Smoother 1.2s animation
  }, []);

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

    // Calculate connection count for particle density
    const connectionCount = graphData.links.filter(
      l => l.source === node.id || l.target === node.id ||
           (l.source as any)?.id === node.id || (l.target as any)?.id === node.id
    ).length;

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
    const hoverScale = hoverScales.get(node.id) || 1.0;

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

    // Add selection outer glow
    if (node.id === selectedNodeId) {
      const glowGeometry = new THREE.SphereGeometry(14, 16, 16);
      const glowMaterial = new THREE.MeshBasicMaterial({
        color: 0xffffff,
        transparent: true,
        opacity: 0.12,
        side: THREE.BackSide,
        blending: THREE.AdditiveBlending,
        depthWrite: false,
      });
      const glow = new THREE.Mesh(glowGeometry, glowMaterial);
      group.add(glow);
    }

    // Add hover highlight
    if (node.id === hoveredNodeId && node.id !== selectedNodeId) {
      const hoverGeometry = new THREE.SphereGeometry(12, 12, 12);
      const hoverMaterial = new THREE.MeshBasicMaterial({
        color: parseInt(layerColor.replace('#', ''), 16),
        transparent: true,
        opacity: 0.08,
        side: THREE.BackSide,
        blending: THREE.AdditiveBlending,
        depthWrite: false,
      });
      const hoverGlow = new THREE.Mesh(hoverGeometry, hoverMaterial);
      group.add(hoverGlow);
    }

    // Store reference for interaction
    group.userData = { nodeId: node.id, nodeType: node.type };

    return group;
  }, [selectedNodeId, hoveredNodeId, selectionBurst, getNodeOpacity, getNodeScale, graphData.links]);

  // Smooth camera zoom to selected node — larger distance for bigger nodes
  const zoomToNode = useCallback((node: ForceGraphNode) => {
    if (!fgRef.current?.cameraPosition) return;

    // Get node position
    const nodePos = {
      x: node.x || 0,
      y: node.y || 0,
      z: node.z || 0,
    };

    // Calculate camera position at larger distance for bigger nodes
    const distance = 180;  // Increased for larger nodes
    const cameraPos = {
      x: nodePos.x + distance * 0.6,
      y: nodePos.y + distance * 0.4,
      z: nodePos.z + distance * 0.7,
    };

    // Smooth 1.5s cinematic animation
    fgRef.current.cameraPosition(cameraPos, nodePos, 1500);
  }, []);

  // Node click handler with zoom
  const handleNodeClick = useCallback(
    (node: ForceGraphNode) => {
      setSelectedNode(node.id);
      zoomToNode(node);

      // Trigger selection burst effect
      setSelectionBurst(node.id);
      setTimeout(() => setSelectionBurst(null), 400);

      onNodeClick?.(node.id);
    },
    [setSelectedNode, zoomToNode, onNodeClick]
  );

  // Node double-click handler
  const handleNodeDoubleClick = useCallback(
    (node: ForceGraphNode) => {
      onNodeDoubleClick?.(node.id);
    },
    [onNodeDoubleClick]
  );

  // Node hover handler with scale effect
  const handleNodeHover = useCallback(
    (node: ForceGraphNode | null, prevNode: ForceGraphNode | null) => {
      // Update hover scale
      if (prevNode) {
        hoverScales.set(prevNode.id, 1.0);
      }
      if (node) {
        hoverScales.set(node.id, 1.15);
      }

      setHoveredNode(node?.id || null);

      // Change cursor
      if (typeof document !== 'undefined') {
        document.body.style.cursor = node ? 'pointer' : 'grab';
      }
    },
    [setHoveredNode]
  );

  // Background click handler
  const handleBackgroundClick = useCallback(() => {
    setSelectedNode(null);
    setSelectedEdge(null);
    onPaneClick?.();
  }, [setSelectedNode, setSelectedEdge, onPaneClick]);

  // Link click handler
  const handleLinkClick = useCallback(
    (link: ForceGraphLink) => {
      // Extract source/target IDs (can be string or object after D3 simulation)
      const sourceId = typeof link.source === 'object'
        ? (link.source as { id?: string }).id ?? ''
        : link.source;
      const targetId = typeof link.target === 'object'
        ? (link.target as { id?: string }).id ?? ''
        : link.target;

      // Clear node selection, set edge selection
      setSelectedNode(null);
      setSelectedEdge(link.id, {
        id: link.id,
        type: link.type ?? 'UNKNOWN',
        source: sourceId,
        target: targetId,
        data: { ...link },
      });

      // Zoom camera to midpoint between nodes
      if (fgRef.current?.cameraPosition && link.source && link.target) {
        const sourceNode = graphData.nodes.find(n => n.id === sourceId);
        const targetNode = graphData.nodes.find(n => n.id === targetId);

        if (sourceNode && targetNode) {
          const midPoint = {
            x: ((sourceNode.x || 0) + (targetNode.x || 0)) / 2,
            y: ((sourceNode.y || 0) + (targetNode.y || 0)) / 2,
            z: ((sourceNode.z || 0) + (targetNode.z || 0)) / 2,
          };

          const distance = 200;
          const cameraPos = {
            x: midPoint.x + distance * 0.6,
            y: midPoint.y + distance * 0.5,
            z: midPoint.z + distance * 0.7,
          };

          fgRef.current.cameraPosition(cameraPos, midPoint, 1200);
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

  // Engine tick to mark graph as ready
  const handleEngineTick = useCallback(() => {
    if (!isGraphReady) {
      setIsGraphReady(true);
    }
  }, [isGraphReady]);

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

  // Custom emissive particle object for bloom compatibility
  const getParticleThreeObject = useCallback((link: unknown) => {
    try {
      const l = link as ForceGraphLink | undefined;
      const colorStr = l && typeof l === 'object' && 'type' in l
        ? getArcParticleConfig(String(l.type || '')).particleColor
        : '#60a5fa';

      const color = new THREE.Color(colorStr);
      const geometry = new THREE.SphereGeometry(1.5, 12, 12);

      // MeshStandardMaterial with emissive for bloom glow
      const material = new THREE.MeshStandardMaterial({
        color: color,
        emissive: color,
        emissiveIntensity: 3.0,  // High intensity to exceed bloom threshold
        transparent: true,
        opacity: 0.9,
      });

      return new THREE.Mesh(geometry, material);
    } catch {
      // Fallback: simple sphere
      const geometry = new THREE.SphereGeometry(1.5, 8, 8);
      const material = new THREE.MeshBasicMaterial({ color: 0x60a5fa });
      return new THREE.Mesh(geometry, material);
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
    <div className={cn('relative', className)}>
      <ForceGraph3D
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
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
        linkColor={getLinkColor as any}
        linkWidth={1.2}
        linkOpacity={0.35}
        linkDirectionalParticles={5}
        linkDirectionalParticleSpeed={0.004}
        linkDirectionalParticleWidth={4}
        linkDirectionalParticleColor={getLinkParticleColor as any}
        linkDirectionalParticleResolution={32}
        linkDirectionalParticleThreeObject={getParticleThreeObject as any}
        linkCurvature={getLinkCurvature as any}
        linkCurveRotation={0.5}
        nodeRelSize={8}
        backgroundColor="#050810"
        showNavInfo={false}
        enableNodeDrag={true}
        enableNavigationControls={true}
        controlType="orbit"
        warmupTicks={0}
        cooldownTicks={100}
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
