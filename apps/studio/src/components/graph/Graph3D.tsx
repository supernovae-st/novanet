'use client';

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
  createGeometryForLayer,
  getArcParticleConfig,
  createStarfield,
  createBloomComposer,
  updateComposerSize,
  type ForceGraphNode,
  type ForceGraphLink,
} from '@/lib/graph3d';
import { Graph3DLegend } from './Graph3DLegend';

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

// Cache geometries to avoid recreation
const geometryCache = new Map<string, THREE.BufferGeometry>();

// Cache materials to avoid recreation
const materialCache = new Map<string, THREE.Material>();

/**
 * Get or create cached geometry for a layer
 */
function getCachedGeometry(layer: string): THREE.BufferGeometry {
  if (!geometryCache.has(layer)) {
    const geometry = createGeometryForLayer(layer as Parameters<typeof createGeometryForLayer>[0]);
    geometryCache.set(layer, geometry);
  }
  return geometryCache.get(layer)!.clone();
}

/**
 * Create material based on trait with caching
 */
function createTraitMaterial(trait: string, layerColor: string): THREE.Material {
  const cacheKey = `${trait}-${layerColor}`;

  if (materialCache.has(cacheKey)) {
    return materialCache.get(cacheKey)!.clone();
  }

  let material: THREE.Material;

  switch (trait) {
    case 'localized':
      // Wireframe for localized content
      material = new THREE.MeshStandardMaterial({
        color: layerColor,
        wireframe: true,
        transparent: true,
        opacity: 0.85,
      });
      break;

    case 'knowledge':
      // Glass-like material for knowledge atoms
      material = new THREE.MeshPhysicalMaterial({
        color: layerColor,
        transparent: true,
        opacity: 0.7,
        transmission: 0.4,
        roughness: 0.15,
        metalness: 0.05,
        clearcoat: 0.3,
      });
      break;

    case 'generated':
      // Emissive glow for generated content
      material = new THREE.MeshStandardMaterial({
        color: layerColor,
        emissive: layerColor,
        emissiveIntensity: 0.5,
        transparent: true,
        opacity: 0.95,
      });
      break;

    case 'aggregated':
      // Dotted appearance for aggregated metrics
      material = new THREE.MeshStandardMaterial({
        color: layerColor,
        transparent: true,
        opacity: 0.6,
        wireframe: true,
        wireframeLinewidth: 2,
      });
      break;

    default: // invariant
      // Solid metallic for invariant definitions
      material = new THREE.MeshStandardMaterial({
        color: layerColor,
        metalness: 0.4,
        roughness: 0.5,
      });
  }

  materialCache.set(cacheKey, material);
  return material.clone();
}

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
  const composerRef = useRef<ReturnType<typeof createBloomComposer> | null>(null);
  const [legendCollapsed, setLegendCollapsed] = useState(false);
  const [isGraphReady, setIsGraphReady] = useState(false);

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

    // Create bloom composer
    const composer = createBloomComposer(renderer, scene, camera, {
      strength: 1.2,
      radius: 0.5,
      threshold: 0.7,
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

  // Custom node rendering with Three.js
  const renderNode = useCallback((node: ForceGraphNode) => {
    const group = new THREE.Group();
    const nodeSize = node.val || 4;

    // Get colors
    const layerColor = getLayerColor(node.layer);
    const realmColor = getRealmColor(node.realm);

    // Create main geometry based on layer
    const geometry = getCachedGeometry(node.layer);

    // Create material based on trait
    const material = createTraitMaterial(node.trait, layerColor);

    // Create mesh
    const mesh = new THREE.Mesh(geometry, material);

    // Apply hover scale
    const hoverScale = hoverScales.get(node.id) || 1.0;
    mesh.scale.setScalar(hoverScale);

    group.add(mesh);

    // Add realm indicator ring
    const ringSize = nodeSize * 1.3;
    const ringGeometry = new THREE.TorusGeometry(ringSize, 0.25, 8, 32);
    const ringMaterial = new THREE.MeshBasicMaterial({
      color: realmColor,
      transparent: true,
      opacity: 0.7,
    });
    const ring = new THREE.Mesh(ringGeometry, ringMaterial);
    ring.rotation.x = Math.PI / 2;
    group.add(ring);

    // Add selection glow
    if (node.id === selectedNodeId) {
      const glowGeometry = new THREE.SphereGeometry(nodeSize * 1.8, 16, 16);
      const glowMaterial = new THREE.MeshBasicMaterial({
        color: '#ffffff',
        transparent: true,
        opacity: 0.15,
        side: THREE.BackSide,
      });
      const glow = new THREE.Mesh(glowGeometry, glowMaterial);
      group.add(glow);

      // Add pulsing inner glow
      const innerGlowGeometry = new THREE.SphereGeometry(nodeSize * 1.4, 16, 16);
      const innerGlowMaterial = new THREE.MeshBasicMaterial({
        color: layerColor,
        transparent: true,
        opacity: 0.25,
      });
      const innerGlow = new THREE.Mesh(innerGlowGeometry, innerGlowMaterial);
      group.add(innerGlow);
    }

    // Add hover highlight
    if (node.id === hoveredNodeId && node.id !== selectedNodeId) {
      const hoverGeometry = new THREE.SphereGeometry(nodeSize * 1.5, 12, 12);
      const hoverMaterial = new THREE.MeshBasicMaterial({
        color: layerColor,
        transparent: true,
        opacity: 0.1,
        side: THREE.BackSide,
      });
      const hoverGlow = new THREE.Mesh(hoverGeometry, hoverMaterial);
      group.add(hoverGlow);
    }

    // Store reference for interaction
    group.userData = { nodeId: node.id, nodeType: node.type };

    return group;
  }, [selectedNodeId, hoveredNodeId]);

  // Camera zoom to node
  const zoomToNode = useCallback((node: ForceGraphNode) => {
    if (!fgRef.current?.cameraPosition) return;

    // Get node position
    const nodePos = {
      x: node.x || 0,
      y: node.y || 0,
      z: node.z || 0,
    };

    // Calculate camera position (zoom to a distance based on node size)
    const distance = (node.val || 4) * 15;
    const cameraPos = {
      x: nodePos.x + distance * 0.7,
      y: nodePos.y + distance * 0.5,
      z: nodePos.z + distance * 0.7,
    };

    // Animate camera
    fgRef.current.cameraPosition(cameraPos, nodePos, 1000);
  }, []);

  // Node click handler with zoom
  const handleNodeClick = useCallback(
    (node: ForceGraphNode) => {
      setSelectedNode(node.id);
      zoomToNode(node);
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
    const config = getArcParticleConfig(link.type);
    return config.particleColor;
  }, []);

  const getLinkWidth = useCallback((link: ForceGraphLink) => {
    const config = getArcParticleConfig(link.type);
    return config.linkWidth;
  }, []);

  const getLinkParticles = useCallback((link: ForceGraphLink) => {
    const config = getArcParticleConfig(link.type);
    return config.particles;
  }, []);

  const getLinkParticleSpeed = useCallback((link: ForceGraphLink) => {
    const config = getArcParticleConfig(link.type);
    return config.particleSpeed;
  }, []);

  const getLinkParticleWidth = useCallback((link: ForceGraphLink) => {
    const config = getArcParticleConfig(link.type);
    return config.particleWidth;
  }, []);

  const getLinkParticleColor = useCallback((link: ForceGraphLink) => {
    const config = getArcParticleConfig(link.type);
    return config.particleColor;
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
        linkOpacity={0.5}
        linkDirectionalParticles={getLinkParticles as any}
        linkDirectionalParticleSpeed={getLinkParticleSpeed as any}
        linkDirectionalParticleWidth={getLinkParticleWidth as any}
        linkDirectionalParticleColor={getLinkParticleColor as any}
        backgroundColor="#070b14"
        showNavInfo={false}
        enableNodeDrag={true}
        enableNavigationControls={true}
        controlType="orbit"
        warmupTicks={50}
        cooldownTicks={100}
      />

      {/* Stats overlay */}
      <div className="absolute bottom-4 left-4 px-3 py-2 rounded-lg bg-black/60 backdrop-blur-sm border border-white/10">
        <p className="text-xs text-white/70 font-medium">
          {graphData.nodes.length} nodes · {graphData.links.length} arcs
        </p>
        {selectedNodeId && (
          <p className="text-[10px] text-white/40 mt-1">
            Selected: {graphData.nodes.find(n => n.id === selectedNodeId)?.name || selectedNodeId}
          </p>
        )}
      </div>

      {/* Legend */}
      {showLegend && (
        <Graph3DLegend
          collapsed={legendCollapsed}
          onToggle={() => setLegendCollapsed(!legendCollapsed)}
        />
      )}
    </div>
  );
});

export default Graph3D;
