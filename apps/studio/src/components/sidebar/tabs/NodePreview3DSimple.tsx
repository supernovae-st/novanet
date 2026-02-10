'use client';

/**
 * NodePreview3DSimple - Simple 3D Preview using React Three Fiber
 *
 * Renders a rotating composite node with:
 * - Core geometry based on Layer
 * - Orbital rings based on Realm
 * - Glow effects
 */

import { memo, useRef } from 'react';
import { Canvas, useFrame } from '@react-three/fiber';
import { OrbitControls } from '@react-three/drei';
import * as THREE from 'three';
import type { Layer, Realm, Trait } from '@novanet/core/types';
import { LAYER_COLORS, REALM_COLORS } from '@/lib/graph3d/colorPalette';

interface NodePreview3DSimpleProps {
  layer: Layer;
  realm: Realm;
  trait: Trait;
  size?: number;
  className?: string;
}

// Core geometry based on layer
function CoreMesh({ layer, color }: { layer: Layer; color: string }) {
  const meshRef = useRef<THREE.Mesh>(null);

  useFrame((_, delta) => {
    if (meshRef.current) {
      meshRef.current.rotation.y += delta * 0.3;
    }
  });

  const geometry = (() => {
    switch (layer) {
      case 'config':
        return <octahedronGeometry args={[1.8]} />;
      case 'locale':
        return <sphereGeometry args={[1.7, 24, 24]} />;
      case 'geography':
        return <icosahedronGeometry args={[1.8]} />;
      case 'knowledge':
        return <icosahedronGeometry args={[1.7]} />;
      case 'foundation':
        return <boxGeometry args={[2.4, 2.4, 2.4]} />;
      case 'structure':
        return <tetrahedronGeometry args={[2]} />;
      case 'semantic':
        return <dodecahedronGeometry args={[1.8]} />;
      case 'instruction':
        return <coneGeometry args={[1.4, 2.6, 8]} />;
      case 'output':
        return <sphereGeometry args={[2, 32, 32]} />;
      default:
        return <sphereGeometry args={[1.8, 24, 24]} />;
    }
  })();

  return (
    <mesh ref={meshRef}>
      {geometry}
      <meshPhysicalMaterial
        color={color}
        metalness={0.3}
        roughness={0.2}
        emissive={color}
        emissiveIntensity={0.4}
        transparent
        opacity={0.95}
      />
    </mesh>
  );
}

// Orbital ring
function OrbitalRing({ radius, color, rotationX, rotationZ }: {
  radius: number;
  color: string;
  rotationX: number;
  rotationZ: number;
}) {
  const ringRef = useRef<THREE.Mesh>(null);

  useFrame((_, delta) => {
    if (ringRef.current) {
      ringRef.current.rotation.z += delta * 0.5;
    }
  });

  return (
    <mesh ref={ringRef} rotation={[rotationX, 0, rotationZ]}>
      <torusGeometry args={[radius, 0.15, 8, 64]} />
      <meshBasicMaterial
        color={color}
        transparent
        opacity={0.5}
        side={THREE.DoubleSide}
        blending={THREE.AdditiveBlending}
        depthWrite={false}
      />
    </mesh>
  );
}

// Main scene content
function SceneContent({ layer, realm }: { layer: Layer; realm: Realm }) {
  const groupRef = useRef<THREE.Group>(null);
  const layerColor = LAYER_COLORS[layer] || '#6366f1';
  const realmColor = REALM_COLORS[realm] || '#6366f1';

  useFrame((_, delta) => {
    if (groupRef.current) {
      groupRef.current.rotation.y += delta * 0.2;
    }
  });

  const ringCount = realm === 'org' ? 2 : 1;

  return (
    <group ref={groupRef}>
      <CoreMesh layer={layer} color={layerColor} />

      {/* Rings based on realm */}
      <OrbitalRing
        radius={2.8}
        color={realmColor}
        rotationX={Math.PI / 2}
        rotationZ={0}
      />
      {ringCount > 1 && (
        <OrbitalRing
          radius={2.4}
          color={realmColor}
          rotationX={Math.PI / 3}
          rotationZ={Math.PI / 4}
        />
      )}
    </group>
  );
}

export const NodePreview3DSimple = memo(function NodePreview3DSimple({
  layer,
  realm,
  size = 100,
  className,
}: NodePreview3DSimpleProps) {
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
        camera={{ position: [8, 6, 10], fov: 45 }}
        gl={{ antialias: true, alpha: true }}
        style={{ background: 'transparent' }}
      >
        <ambientLight intensity={0.4} />
        <directionalLight position={[10, 20, 15]} intensity={0.8} />
        <pointLight position={[-10, 10, 10]} intensity={0.5} />

        <SceneContent layer={layer} realm={realm} />

        <OrbitControls
          enableZoom={false}
          enablePan={false}
          autoRotate
          autoRotateSpeed={1}
        />
      </Canvas>
    </div>
  );
});

export default NodePreview3DSimple;
