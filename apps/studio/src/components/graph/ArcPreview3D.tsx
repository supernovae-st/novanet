'use client';

/**
 * ArcPreview3D - 3D Preview for Arc Details Sidebar
 *
 * Renders an isolated Three.js canvas showing:
 * - Two mini composite nodes (source and target)
 * - Animated arc with particle flow between them
 * - Glow effects
 * - Auto-rotation
 */

import { memo, useRef, useEffect, useState, useCallback } from 'react';
import * as THREE from 'three';
import { cn } from '@/lib/utils';
import {
  createCompositeNode,
  animateCompositeNode,
  disposeCompositeNode,
  type CompositeNodeMeshes,
} from '@/lib/graph3d/nodeComposite';
import { getLayerColor, getRealmColor } from '@/lib/graph3d/colorPalette';
import { getArcParticleConfig, detectArcFamily } from '@/lib/graph3d/arcParticles';
import type { Layer, Realm, Trait } from '@novanet/core/types';

export interface ArcPreview3DProps {
  /** Arc type (e.g., "HAS_PAGE", "USES_ENTITY") */
  arcType: string;
  /** Source node info */
  source: {
    layer: Layer;
    realm: Realm;
    trait: Trait;
  };
  /** Target node info */
  target: {
    layer: Layer;
    realm: Realm;
    trait: Trait;
  };
  /** Canvas size in pixels */
  size?: number;
  /** Additional class names */
  className?: string;
}

export const ArcPreview3D = memo(function ArcPreview3D({
  arcType,
  source,
  target,
  size = 180,
  className,
}: ArcPreview3DProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const rendererRef = useRef<THREE.WebGLRenderer | null>(null);
  const sceneRef = useRef<THREE.Scene | null>(null);
  const cameraRef = useRef<THREE.PerspectiveCamera | null>(null);
  const sourceNodeRef = useRef<CompositeNodeMeshes | null>(null);
  const targetNodeRef = useRef<CompositeNodeMeshes | null>(null);
  const arcLineRef = useRef<THREE.Line | null>(null);
  const particlesRef = useRef<THREE.Points | null>(null);
  const animationRef = useRef<number>(0);
  const startTimeRef = useRef<number>(0);

  const [isHovered, setIsHovered] = useState(false);
  const [isInitialized, setIsInitialized] = useState(false);

  // Initialize Three.js scene
  useEffect(() => {
    if (!containerRef.current) return;

    // Create scene
    const scene = new THREE.Scene();
    sceneRef.current = scene;

    // Create camera
    const camera = new THREE.PerspectiveCamera(45, 1, 0.1, 1000);
    camera.position.set(0, 20, 40);
    camera.lookAt(0, 0, 0);
    cameraRef.current = camera;

    // Create renderer
    const renderer = new THREE.WebGLRenderer({
      antialias: true,
      alpha: true,
    });
    renderer.setSize(size, size);
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    renderer.setClearColor(0x000000, 0);
    renderer.toneMapping = THREE.ACESFilmicToneMapping;
    renderer.toneMappingExposure = 1.2;
    rendererRef.current = renderer;

    containerRef.current.appendChild(renderer.domElement);

    // Add ambient light
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.4);
    scene.add(ambientLight);

    // Add directional light
    const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
    directionalLight.position.set(10, 20, 15);
    scene.add(directionalLight);

    // Mark as initialized to trigger animation
    setIsInitialized(true);

    return () => {
      cancelAnimationFrame(animationRef.current);
      setIsInitialized(false);

      // Dispose all objects before renderer
      if (sceneRef.current) {
        if (sourceNodeRef.current) {
          sceneRef.current.remove(sourceNodeRef.current.group);
          disposeCompositeNode(sourceNodeRef.current);
          sourceNodeRef.current = null;
        }
        if (targetNodeRef.current) {
          sceneRef.current.remove(targetNodeRef.current.group);
          disposeCompositeNode(targetNodeRef.current);
          targetNodeRef.current = null;
        }
        if (arcLineRef.current) {
          sceneRef.current.remove(arcLineRef.current);
          arcLineRef.current.geometry.dispose();
          (arcLineRef.current.material as THREE.Material).dispose();
          arcLineRef.current = null;
        }
        if (particlesRef.current) {
          sceneRef.current.remove(particlesRef.current);
          particlesRef.current.geometry.dispose();
          (particlesRef.current.material as THREE.Material).dispose();
          particlesRef.current = null;
        }
      }

      renderer.dispose();

      // Safe DOM cleanup - container may be unmounted
      try {
        if (containerRef.current?.contains(renderer.domElement)) {
          containerRef.current.removeChild(renderer.domElement);
        }
      } catch {
        // Component already unmounted, ignore
      }
    };
  }, [size]);

  // Create nodes and arc
  useEffect(() => {
    const scene = sceneRef.current;
    if (!scene) return;

    // Cleanup old objects with explicit null reset
    if (sourceNodeRef.current) {
      scene.remove(sourceNodeRef.current.group);
      disposeCompositeNode(sourceNodeRef.current);
      sourceNodeRef.current = null;
    }
    if (targetNodeRef.current) {
      scene.remove(targetNodeRef.current.group);
      disposeCompositeNode(targetNodeRef.current);
      targetNodeRef.current = null;
    }
    if (arcLineRef.current) {
      scene.remove(arcLineRef.current);
      arcLineRef.current.geometry.dispose();
      (arcLineRef.current.material as THREE.Material).dispose();
      arcLineRef.current = null;
    }
    if (particlesRef.current) {
      scene.remove(particlesRef.current);
      particlesRef.current.geometry.dispose();
      (particlesRef.current.material as THREE.Material).dispose();
      particlesRef.current = null;
    }

    // Node positions
    const sourcePos = new THREE.Vector3(-12, 0, 0);
    const targetPos = new THREE.Vector3(12, 0, 0);

    // Create source node
    const sourceNode = createCompositeNode({
      layer: source.layer,
      realm: source.realm,
      trait: source.trait,
      layerColor: getLayerColor(source.layer),
      realmColor: getRealmColor(source.realm),
      connectionCount: 1,
      baseSize: 4,
    });
    sourceNode.group.position.copy(sourcePos);
    sourceNodeRef.current = sourceNode;
    scene.add(sourceNode.group);

    // Create target node
    const targetNode = createCompositeNode({
      layer: target.layer,
      realm: target.realm,
      trait: target.trait,
      layerColor: getLayerColor(target.layer),
      realmColor: getRealmColor(target.realm),
      connectionCount: 1,
      baseSize: 4,
    });
    targetNode.group.position.copy(targetPos);
    targetNodeRef.current = targetNode;
    scene.add(targetNode.group);

    // Get arc config
    const arcConfig = getArcParticleConfig(arcType);
    const arcColor = new THREE.Color(arcConfig.particleColor);

    // Create arc line
    const lineGeometry = new THREE.BufferGeometry().setFromPoints([sourcePos, targetPos]);
    const lineMaterial = new THREE.LineBasicMaterial({
      color: arcColor,
      transparent: true,
      opacity: 0.4,
      blending: THREE.AdditiveBlending,
    });
    const line = new THREE.Line(lineGeometry, lineMaterial);
    arcLineRef.current = line;
    scene.add(line);

    // Create particles for the arc
    const particleCount = 30;
    const positions = new Float32Array(particleCount * 3);
    for (let i = 0; i < particleCount; i++) {
      const t = i / particleCount;
      positions[i * 3] = sourcePos.x + (targetPos.x - sourcePos.x) * t;
      positions[i * 3 + 1] = 0;
      positions[i * 3 + 2] = 0;
    }

    const particleGeometry = new THREE.BufferGeometry();
    particleGeometry.setAttribute('position', new THREE.BufferAttribute(positions, 3));

    const particleMaterial = new THREE.PointsMaterial({
      color: arcColor,
      size: 2,
      transparent: true,
      opacity: 0.9,
      blending: THREE.AdditiveBlending,
      depthWrite: false,
    });

    const particles = new THREE.Points(particleGeometry, particleMaterial);
    particlesRef.current = particles;
    scene.add(particles);
  }, [arcType, source, target]);

  // Animation loop - using performance.now() for reliable timing
  useEffect(() => {
    // Wait for initialization
    if (!isInitialized || !rendererRef.current || !sceneRef.current || !cameraRef.current) return;

    startTimeRef.current = performance.now();
    let lastTime = startTimeRef.current;

    const animate = () => {
      animationRef.current = requestAnimationFrame(animate);

      const now = performance.now();
      const delta = (now - lastTime) / 1000; // Convert to seconds
      const elapsed = (now - startTimeRef.current) / 1000;
      lastTime = now;

      // Animate nodes
      if (sourceNodeRef.current) {
        animateCompositeNode(sourceNodeRef.current, delta, elapsed);
      }
      if (targetNodeRef.current) {
        animateCompositeNode(targetNodeRef.current, delta, elapsed);
      }

      // Animate particles flowing along arc
      if (particlesRef.current) {
        const positions = particlesRef.current.geometry.attributes.position.array as Float32Array;
        const count = positions.length / 3;

        for (let i = 0; i < count; i++) {
          // Move particle along x-axis using elapsed time for consistent speed
          const particlePhase = (elapsed * 0.8 + i / count) % 1; // 0.8 = speed
          positions[i * 3] = -12 + particlePhase * 24; // -12 to +12

          // Add slight y oscillation
          positions[i * 3 + 1] = Math.sin(elapsed * 3 + i * 0.5) * 0.5;
        }

        particlesRef.current.geometry.attributes.position.needsUpdate = true;
      }

      // Auto-rotate scene (unless hovered) - smooth continuous rotation
      if (!isHovered && sceneRef.current) {
        sceneRef.current.rotation.y = elapsed * 0.4;
      }

      rendererRef.current!.render(sceneRef.current!, cameraRef.current!);
    };

    animate();

    return () => {
      cancelAnimationFrame(animationRef.current);
    };
  }, [isInitialized, isHovered]);

  const handleMouseEnter = useCallback(() => setIsHovered(true), []);
  const handleMouseLeave = useCallback(() => setIsHovered(false), []);

  return (
    <div
      ref={containerRef}
      className={cn(
        'relative rounded-xl overflow-hidden',
        'bg-gradient-to-b from-slate-900/50 to-black/50',
        'border border-white/10',
        className
      )}
      style={{ width: size, height: size }}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
    />
  );
});
