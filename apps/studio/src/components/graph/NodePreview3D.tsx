'use client';

/**
 * NodePreview3D - 3D Preview for Node Details Sidebar
 *
 * Renders an isolated Three.js canvas showing the node with:
 * - Core geometry (Layer)
 * - Orbital rings (Realm)
 * - Particle cloud
 * - Glow effects
 * - Auto-rotation (pauses on hover)
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
import type { Layer, Realm, Trait } from '@novanet/core/types';

export interface NodePreview3DProps {
  /** Node layer for geometry */
  layer: Layer;
  /** Node realm for ring count */
  realm: Realm;
  /** Node trait for animation speed */
  trait: Trait;
  /** Canvas size in pixels */
  size?: number;
  /** Enable auto-rotation */
  autoRotate?: boolean;
  /** Additional class names */
  className?: string;
}

export const NodePreview3D = memo(function NodePreview3D({
  layer,
  realm,
  trait,
  size = 180,
  autoRotate = true,
  className,
}: NodePreview3DProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const rendererRef = useRef<THREE.WebGLRenderer | null>(null);
  const sceneRef = useRef<THREE.Scene | null>(null);
  const cameraRef = useRef<THREE.PerspectiveCamera | null>(null);
  const compositeMeshesRef = useRef<CompositeNodeMeshes | null>(null);
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
    camera.position.set(20, 15, 25);
    camera.lookAt(0, 0, 0);
    cameraRef.current = camera;

    // Create renderer with transparency
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

    // Add point light for glow effect
    const pointLight = new THREE.PointLight(0xffffff, 0.5, 50);
    pointLight.position.set(-10, 10, 10);
    scene.add(pointLight);

    // Mark as initialized to trigger animation
    setIsInitialized(true);

    // Cleanup
    return () => {
      cancelAnimationFrame(animationRef.current);
      setIsInitialized(false);
      renderer.dispose();
      if (containerRef.current?.contains(renderer.domElement)) {
        containerRef.current.removeChild(renderer.domElement);
      }
    };
  }, [size]);

  // Create/update node when props change
  useEffect(() => {
    if (!sceneRef.current) return;

    // Remove old node
    if (compositeMeshesRef.current) {
      sceneRef.current.remove(compositeMeshesRef.current.group);
      disposeCompositeNode(compositeMeshesRef.current);
    }

    // Get colors
    const layerColor = getLayerColor(layer);
    const realmColor = getRealmColor(realm);

    // Create new composite node
    const composite = createCompositeNode({
      layer,
      realm,
      trait,
      layerColor,
      realmColor,
      connectionCount: 5, // Show some particles
      baseSize: 6, // Smaller for preview
    });

    compositeMeshesRef.current = composite;
    sceneRef.current.add(composite.group);
  }, [layer, realm, trait]);

  // Animation loop - using performance.now() for reliable timing
  useEffect(() => {
    // Wait for initialization
    if (!isInitialized || !rendererRef.current || !sceneRef.current || !cameraRef.current) return;

    // Initialize start time
    startTimeRef.current = performance.now();
    let lastTime = startTimeRef.current;

    const animate = () => {
      animationRef.current = requestAnimationFrame(animate);

      const now = performance.now();
      const delta = (now - lastTime) / 1000; // Convert to seconds
      const elapsed = (now - startTimeRef.current) / 1000;
      lastTime = now;

      // Animate composite node (rings rotation, particle orbit, pulse)
      if (compositeMeshesRef.current) {
        animateCompositeNode(compositeMeshesRef.current, delta, elapsed);

        // Auto-rotate the group (unless hovered) - smooth continuous rotation
        if (autoRotate && !isHovered) {
          // Use elapsed time for smooth rotation (0.5 rad/s ≈ 12.5s per full rotation)
          compositeMeshesRef.current.group.rotation.y = elapsed * 0.5;
        }
      }

      // Render
      rendererRef.current!.render(sceneRef.current!, cameraRef.current!);
    };

    animate();

    return () => {
      cancelAnimationFrame(animationRef.current);
    };
  }, [isInitialized, autoRotate, isHovered]);

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
