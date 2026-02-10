'use client';

/**
 * Trackpad3D - Work Louder Loop Module 3D Visualizer
 *
 * v1.0 - Frosted White Edition
 * Based on screenshot reference
 *
 * Features:
 * - White frosted plastic chassis with meshPhysicalMaterial
 * - Side groove details on left edge
 * - Black rotary encoder on top-right
 * - Touch surface with subtle glow
 * - Same dark background as SuperNovaePad3D
 */

import { useRef } from 'react';
import { Canvas, useFrame } from '@react-three/fiber';
import { OrbitControls, RoundedBox, ContactShadows, Environment } from '@react-three/drei';
import * as THREE from 'three';

// =============================================================================
// Types
// =============================================================================

interface Trackpad3DProps {
  /** Additional class names */
  className?: string;
}

// =============================================================================
// Constants
// =============================================================================

// Trackpad dimensions (slightly rectangular, portrait orientation)
const TRACKPAD_WIDTH = 5.5;
const TRACKPAD_DEPTH = 6.0;
const TRACKPAD_HEIGHT = 0.4;
const CORNER_RADIUS = 0.35;

// Colors
const CHASSIS_WHITE = '#f1f5f9';
const SURFACE_WHITE = '#f8fafc';
const GROOVE_DARK = '#334155';

// =============================================================================
// Touch Surface (Top glass layer)
// =============================================================================

function TouchSurface() {
  return (
    <RoundedBox
      args={[TRACKPAD_WIDTH - 0.15, 0.08, TRACKPAD_DEPTH - 0.15]}
      radius={CORNER_RADIUS - 0.05}
      smoothness={8}
      position={[0, TRACKPAD_HEIGHT / 2 + 0.02, 0]}
    >
      <meshPhysicalMaterial
        color={SURFACE_WHITE}
        metalness={0.0}
        roughness={0.15}
        clearcoat={0.95}
        clearcoatRoughness={0.1}
        transmission={0.05}
        thickness={0.3}
        ior={1.49}
        envMapIntensity={1.2}
      />
    </RoundedBox>
  );
}

// =============================================================================
// Side Grooves (Left edge decorative detail)
// =============================================================================

function SideGrooves() {
  const grooveCount = 3;
  const grooveSpacing = 0.25;
  const grooveHeight = TRACKPAD_DEPTH * 0.7;
  const startZ = -grooveHeight / 2;

  return (
    <group position={[-TRACKPAD_WIDTH / 2 + 0.08, 0, 0]}>
      {Array.from({ length: grooveCount }).map((_, i) => (
        <mesh
          key={i}
          position={[0.05 * i, TRACKPAD_HEIGHT / 2 - 0.02, 0]}
          rotation={[Math.PI / 2, 0, 0]}
        >
          <boxGeometry args={[0.03, grooveHeight, 0.08]} />
          <meshStandardMaterial
            color={GROOVE_DARK}
            metalness={0.3}
            roughness={0.7}
          />
        </mesh>
      ))}
    </group>
  );
}

// =============================================================================
// Main Chassis (Frosted white body)
// =============================================================================

function TrackpadChassis() {
  return (
    <group>
      {/* Main body - white frosted plastic */}
      <RoundedBox
        args={[TRACKPAD_WIDTH, TRACKPAD_HEIGHT, TRACKPAD_DEPTH]}
        radius={CORNER_RADIUS}
        smoothness={8}
        position={[0, 0, 0]}
        castShadow
        receiveShadow
      >
        <meshPhysicalMaterial
          color={CHASSIS_WHITE}
          metalness={0.0}
          roughness={0.35}
          clearcoat={0.8}
          clearcoatRoughness={0.2}
          transmission={0.08}
          thickness={0.5}
          ior={1.49}
          envMapIntensity={1.0}
        />
      </RoundedBox>

      {/* Touch surface */}
      <TouchSurface />

      {/* Side grooves */}
      <SideGrooves />

      {/* Bottom edge detail - darker inset */}
      <RoundedBox
        args={[TRACKPAD_WIDTH - 0.1, 0.05, TRACKPAD_DEPTH - 0.1]}
        radius={CORNER_RADIUS - 0.02}
        smoothness={4}
        position={[0, -TRACKPAD_HEIGHT / 2 + 0.02, 0]}
      >
        <meshStandardMaterial
          color="#cbd5e1"
          roughness={0.6}
        />
      </RoundedBox>
    </group>
  );
}

// =============================================================================
// Black Encoder Knob (Top-right corner)
// =============================================================================

function BlackEncoder({ position }: { position: [number, number, number] }) {
  const knobRef = useRef<THREE.Group>(null);

  useFrame((state) => {
    if (knobRef.current) {
      knobRef.current.rotation.y = Math.sin(state.clock.elapsedTime * 0.3) * 0.03;
    }
  });

  return (
    <group position={position} ref={knobRef}>
      {/* Base ring - dark */}
      <mesh position={[0, -0.04, 0]} castShadow>
        <cylinderGeometry args={[0.38, 0.4, 0.06, 32]} />
        <meshStandardMaterial color="#374151" metalness={0.7} roughness={0.35} />
      </mesh>

      {/* Main body - matte black */}
      <mesh castShadow>
        <cylinderGeometry args={[0.32, 0.34, 0.35, 32]} />
        <meshStandardMaterial color="#1f2937" metalness={0.15} roughness={0.85} />
      </mesh>

      {/* Top - slightly concave look */}
      <mesh position={[0, 0.16, 0]}>
        <cylinderGeometry args={[0.26, 0.3, 0.06, 32]} />
        <meshStandardMaterial color="#111827" metalness={0.2} roughness={0.8} />
      </mesh>

      {/* Knurling grooves */}
      {Array.from({ length: 20 }).map((_, i) => {
        const angle = (i / 20) * Math.PI * 2;
        return (
          <mesh
            key={i}
            position={[Math.cos(angle) * 0.3, 0, Math.sin(angle) * 0.3]}
            rotation={[0, -angle, 0]}
          >
            <boxGeometry args={[0.015, 0.28, 0.006]} />
            <meshStandardMaterial color="#374151" metalness={0.9} roughness={0.2} />
          </mesh>
        );
      })}

      {/* Position indicator - white line */}
      <mesh position={[0, 0.2, 0.24]} rotation={[Math.PI / 2, 0, 0]}>
        <boxGeometry args={[0.04, 0.06, 0.012]} />
        <meshBasicMaterial color="#ffffff" />
      </mesh>
    </group>
  );
}

// =============================================================================
// Background (Same as SuperNovaePad3D)
// =============================================================================

function Background() {
  return (
    <group position={[0, -TRACKPAD_HEIGHT / 2 - 0.2, 0]}>
      <mesh rotation={[-Math.PI / 2, 0, 0]} receiveShadow>
        <planeGeometry args={[50, 50]} />
        <meshStandardMaterial color="#0f172a" />
      </mesh>
      <gridHelper args={[50, 50, '#1e3a5f', '#0c1929']} position={[0, 0.01, 0]} />
      <mesh rotation={[-Math.PI / 2, 0, 0]} position={[0, 0.02, 0]}>
        <circleGeometry args={[5, 48]} />
        <meshBasicMaterial color="#0ea5e9" transparent opacity={0.05} />
      </mesh>
    </group>
  );
}

// =============================================================================
// Scene
// =============================================================================

function Scene() {
  // Encoder position (top-right corner)
  const encoderPos: [number, number, number] = [
    TRACKPAD_WIDTH / 2 - 0.7,
    TRACKPAD_HEIGHT / 2 + 0.2,
    -TRACKPAD_DEPTH / 2 + 0.7
  ];

  return (
    <>
      {/* Lighting */}
      <ambientLight intensity={0.6} />
      <directionalLight
        position={[5, 15, 8]}
        intensity={1.4}
        castShadow
        shadow-mapSize={[2048, 2048]}
        shadow-camera-far={50}
        shadow-camera-left={-10}
        shadow-camera-right={10}
        shadow-camera-top={10}
        shadow-camera-bottom={-10}
      />
      <pointLight position={[-5, 6, -5]} intensity={0.4} color="#a855f7" />
      <pointLight position={[5, 5, 5]} intensity={0.35} color="#0ea5e9" />
      <pointLight position={[0, 4, 0]} intensity={0.3} color="#ffffff" />

      {/* Environment for realistic reflections */}
      <Environment preset="studio" />

      <Background />

      {/* Main trackpad */}
      <TrackpadChassis />

      {/* Encoder */}
      <BlackEncoder position={encoderPos} />

      {/* Contact shadows for realism */}
      <ContactShadows
        position={[0, -TRACKPAD_HEIGHT / 2 - 0.18, 0]}
        opacity={0.5}
        scale={12}
        blur={2}
        far={4}
        resolution={256}
        color="#000000"
      />

      <OrbitControls
        enablePan={false}
        minDistance={5}
        maxDistance={15}
        minPolarAngle={0.2}
        maxPolarAngle={1.5}
        target={[0, 0, 0]}
        enableDamping
        dampingFactor={0.05}
      />
    </>
  );
}

// =============================================================================
// Export
// =============================================================================

export function Trackpad3D({ className }: Trackpad3DProps) {
  return (
    <div className={className} style={{ width: '100%', height: '100%', background: '#0f172a' }}>
      <Canvas
        shadows
        camera={{ position: [0, 7, 9], fov: 42 }}
        onCreated={(state) => {
          state.gl.toneMapping = THREE.ACESFilmicToneMapping;
          state.gl.toneMappingExposure = 1.2;
        }}
      >
        <color attach="background" args={['#0f172a']} />
        <fog attach="fog" args={['#0f172a', 18, 35]} />
        <Scene />
      </Canvas>
    </div>
  );
}

export default Trackpad3D;
