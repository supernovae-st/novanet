'use client';

/**
 * ArcPreview3DSimple - Simple 3D Arc Preview using React Three Fiber
 *
 * Renders two nodes connected by an animated arc with particles
 */

import { memo, useRef, useMemo } from 'react';
import { Canvas, useFrame } from '@react-three/fiber';
import { OrbitControls } from '@react-three/drei';
import * as THREE from 'three';
import type { Layer, Realm, Trait } from '@novanet/core/types';
import { LAYER_COLORS, REALM_COLORS, ARC_FAMILY_COLORS } from '@/lib/graph3d/colorPalette';
import { detectArcFamily } from '@/lib/graph3d/arcParticles';

interface ArcPreview3DSimpleProps {
  arcType: string;
  source: { layer: Layer; realm: Realm; trait: Trait };
  target: { layer: Layer; realm: Realm; trait: Trait };
  size?: number;
  className?: string;
}

// Mini node for source/target
function MiniNode({ position, layer, realm }: {
  position: [number, number, number];
  layer: Layer;
  realm: Realm;
}) {
  const meshRef = useRef<THREE.Mesh>(null);
  const ringRef = useRef<THREE.Mesh>(null);
  const layerColor = LAYER_COLORS[layer] || '#6366f1';
  const realmColor = REALM_COLORS[realm] || '#6366f1';

  useFrame((_, delta) => {
    if (meshRef.current) {
      meshRef.current.rotation.y += delta * 0.5;
    }
    if (ringRef.current) {
      ringRef.current.rotation.z += delta * 0.8;
    }
  });

  return (
    <group position={position}>
      <mesh ref={meshRef}>
        <dodecahedronGeometry args={[1.2]} />
        <meshPhysicalMaterial
          color={layerColor}
          metalness={0.3}
          roughness={0.2}
          emissive={layerColor}
          emissiveIntensity={0.4}
          transparent
          opacity={0.95}
        />
      </mesh>
      <mesh ref={ringRef} rotation={[Math.PI / 2, 0, 0]}>
        <torusGeometry args={[1.8, 0.1, 8, 32]} />
        <meshBasicMaterial
          color={realmColor}
          transparent
          opacity={0.4}
          blending={THREE.AdditiveBlending}
          depthWrite={false}
        />
      </mesh>
    </group>
  );
}

// Animated arc line with particles
function ArcLine({ arcType }: { arcType: string }) {
  const pointsRef = useRef<THREE.Points>(null);

  const arcFamily = detectArcFamily(arcType);
  const arcColor = ARC_FAMILY_COLORS[arcFamily as keyof typeof ARC_FAMILY_COLORS] || '#6366f1';

  // Create particle positions
  const particleCount = 20;
  const particlePositions = useMemo(() => {
    const positions = new Float32Array(particleCount * 3);
    for (let i = 0; i < particleCount; i++) {
      const t = i / particleCount;
      positions[i * 3] = -5 + t * 10; // x: -5 to 5
      positions[i * 3 + 1] = 0;
      positions[i * 3 + 2] = 0;
    }
    return positions;
  }, []);

  // Create line object imperatively to avoid JSX type conflicts
  const lineObject = useMemo(() => {
    const geometry = new THREE.BufferGeometry();
    const points = [new THREE.Vector3(-5, 0, 0), new THREE.Vector3(5, 0, 0)];
    geometry.setFromPoints(points);
    const material = new THREE.LineBasicMaterial({
      color: arcColor,
      transparent: true,
      opacity: 0.4,
      blending: THREE.AdditiveBlending,
    });
    return new THREE.Line(geometry, material);
  }, [arcColor]);

  // Create particle geometry
  const particleGeometry = useMemo(() => {
    const geometry = new THREE.BufferGeometry();
    geometry.setAttribute('position', new THREE.BufferAttribute(particlePositions, 3));
    return geometry;
  }, [particlePositions]);

  useFrame(({ clock }) => {
    if (pointsRef.current) {
      const positions = pointsRef.current.geometry.attributes.position.array as Float32Array;
      const time = clock.getElapsedTime();

      for (let i = 0; i < particleCount; i++) {
        const phase = (time * 0.8 + i / particleCount) % 1;
        positions[i * 3] = -5 + phase * 10;
        positions[i * 3 + 1] = Math.sin(time * 3 + i * 0.5) * 0.3;
      }

      pointsRef.current.geometry.attributes.position.needsUpdate = true;
    }
  });

  return (
    <>
      <primitive object={lineObject} />
      <points ref={pointsRef} geometry={particleGeometry}>
        <pointsMaterial
          color={arcColor}
          size={0.4}
          transparent
          opacity={0.9}
          blending={THREE.AdditiveBlending}
          depthWrite={false}
          sizeAttenuation
        />
      </points>
    </>
  );
}

// Main scene content
function SceneContent({ arcType, source, target }: {
  arcType: string;
  source: { layer: Layer; realm: Realm };
  target: { layer: Layer; realm: Realm };
}) {
  const groupRef = useRef<THREE.Group>(null);

  useFrame((_, delta) => {
    if (groupRef.current) {
      groupRef.current.rotation.y += delta * 0.15;
    }
  });

  return (
    <group ref={groupRef}>
      <MiniNode position={[-5, 0, 0]} layer={source.layer} realm={source.realm} />
      <MiniNode position={[5, 0, 0]} layer={target.layer} realm={target.realm} />
      <ArcLine arcType={arcType} />
    </group>
  );
}

export const ArcPreview3DSimple = memo(function ArcPreview3DSimple({
  arcType,
  source,
  target,
  size = 100,
  className,
}: ArcPreview3DSimpleProps) {
  return (
    <div
      className={className}
      style={{
        width: size,
        height: size,
        borderRadius: '0.75rem',
        overflow: 'hidden',
        background: 'linear-gradient(to bottom, rgba(15, 23, 42, 0.5), rgba(0, 0, 0, 0.5))',
        border: '1px solid rgba(255, 255, 255, 0.1)',
      }}
    >
      <Canvas
        camera={{ position: [0, 8, 16], fov: 45 }}
        gl={{ antialias: true, alpha: true }}
        style={{ background: 'transparent' }}
      >
        <ambientLight intensity={0.4} />
        <directionalLight position={[10, 20, 15]} intensity={0.8} />

        <SceneContent arcType={arcType} source={source} target={target} />

        <OrbitControls
          enableZoom={false}
          enablePan={false}
          autoRotate
          autoRotateSpeed={0.8}
        />
      </Canvas>
    </div>
  );
});

export default ArcPreview3DSimple;
