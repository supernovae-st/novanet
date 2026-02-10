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
import { Sparkles } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useFilteredGraph } from '@/hooks';
import { useUIStore } from '@/stores/uiStore';
import { EmptyState } from '@/components/ui/EmptyState';
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
import type { Layer, NodeRealm, NodeTrait } from '@novanet/core/types';
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

  // Transform data for force graph
  const graphData = useMemo(() => {
    const transformed = transformGraphData(nodes, edges);
    return filterValidData(transformed);
  }, [nodes, edges]);

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
      threshold: 0.3,     // More elements glow
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

  // Galaxy boot animation - nodes spiral in from center
  useEffect(() => {
    if (!isGraphReady || bootPhase !== 'loading' || graphData.nodes.length === 0) return;

    setBootPhase('spawning');

    // Animate nodes from center with stagger
    // fx/fy/fz are D3 force properties added at runtime
    graphData.nodes.forEach((node, index) => {
      const n = node as ForceGraphNode & { fx?: number; fy?: number; fz?: number };
      // Start all nodes at center
      n.fx = 0;
      n.fy = 0;
      n.fz = 0;

      // Release with stagger
      const delay = index * 30;  // 30ms stagger
      setTimeout(() => {
        n.fx = undefined;
        n.fy = undefined;
        n.fz = undefined;
      }, delay);
    });

    // Mark as ready after all nodes released
    const totalDelay = graphData.nodes.length * 30 + 1500;
    setTimeout(() => setBootPhase('ready'), totalDelay);
  }, [isGraphReady, bootPhase, graphData.nodes]);

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

      const sourceId = typeof source === 'object' ? source.id : source;
      const targetId = typeof target === 'object' ? target.id : target;

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
      realm: node.realm as NodeRealm,
      trait: node.trait as NodeTrait,
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
    onPaneClick?.();
  }, [setSelectedNode, onPaneClick]);

  // Engine tick to mark graph as ready
  const handleEngineTick = useCallback(() => {
    if (!isGraphReady) {
      setIsGraphReady(true);
    }
  }, [isGraphReady]);

  // Link styling callbacks
  const getLinkColor = useCallback((link: ForceGraphLink) => {
    if (!link?.type) return '#60a5fa';
    const config = getArcParticleConfig(link.type);
    return config.particleColor;
  }, []);

  const getLinkWidth = useCallback((link: ForceGraphLink) => {
    if (!link) return 0;
    const config = getArcParticleConfig(link.type);
    const baseWidth = config.linkWidth;

    // Highlight connected links
    const source = link.source as string | { id: string } | undefined;
    const target = link.target as string | { id: string } | undefined;
    if (!source || !target) return baseWidth;

    const sourceId = typeof source === 'object' ? source.id : source;
    const targetId = typeof target === 'object' ? target.id : target;
    const linkKey = `${sourceId}-${targetId}`;

    if (highlightedLinks.has(linkKey)) {
      return baseWidth * 3;  // 3x wider when highlighted
    }

    return selectedNodeId ? baseWidth * 0.5 : baseWidth;  // Dim when selection exists
  }, [highlightedLinks, selectedNodeId]);

  const getLinkOpacity = useCallback((link: ForceGraphLink) => {
    if (!link) return 0.3;
    const source = link.source as string | { id: string } | undefined;
    const target = link.target as string | { id: string } | undefined;
    if (!source || !target) return 0.3;

    const sourceId = typeof source === 'object' ? source.id : source;
    const targetId = typeof target === 'object' ? target.id : target;
    const linkKey = `${sourceId}-${targetId}`;

    if (!selectedNodeId) return 0.5;  // Default opacity
    if (highlightedLinks.has(linkKey)) return 0.9;  // Highlighted
    return 0.1;  // Dimmed
  }, [highlightedLinks, selectedNodeId]);

  const getLinkParticles = useCallback((link: ForceGraphLink) => {
    if (!link?.type) return 6;
    const config = getArcParticleConfig(link.type);
    return config.particles;
  }, []);

  const getLinkParticleSpeed = useCallback((link: ForceGraphLink) => {
    if (!link?.type) return 0.005;
    const config = getArcParticleConfig(link.type);
    return config.particleSpeed;
  }, []);

  const getLinkParticleWidth = useCallback((link: ForceGraphLink) => {
    if (!link?.type) return 15;
    const config = getArcParticleConfig(link.type);
    return config.particleWidth;
  }, []);

  const getLinkParticleColor = useCallback((link: ForceGraphLink) => {
    if (!link?.type) return '#60a5fa';
    const config = getArcParticleConfig(link.type);
    return config.particleColor;
  }, []);

  const getLinkCurvature = useCallback((link: ForceGraphLink) => {
    if (!link?.type) return 0;
    const config = getArcParticleConfig(link.type);
    return config.curvature;
  }, []);

  // Empty state - unified with Graph2D
  if (graphData.nodes.length === 0) {
    return (
      <div className={cn('h-full bg-slate-900', className)}>
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
        onBackgroundClick={handleBackgroundClick}
        onEngineTick={handleEngineTick}
        linkColor={getLinkColor as any}
        linkWidth={getLinkWidth as any}
        linkOpacity={getLinkOpacity as any}
        linkDirectionalParticles={getLinkParticles as any}
        linkDirectionalParticleSpeed={getLinkParticleSpeed as any}
        linkDirectionalParticleWidth={getLinkParticleWidth as any}
        linkDirectionalParticleColor={getLinkParticleColor as any}
        linkDirectionalParticleResolution={16}
        linkCurvature={getLinkCurvature as any}
        linkCurveRotation={0.5}
        nodeRelSize={8}
        backgroundColor="#050810"
        showNavInfo={false}
        enableNodeDrag={true}
        enableNavigationControls={true}
        controlType="orbit"
        warmupTicks={80}
        cooldownTicks={150}
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
