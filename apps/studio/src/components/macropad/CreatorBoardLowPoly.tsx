'use client';

/**
 * Work Louder Creator Micro - 3D Visualizer (Performance Optimized)
 *
 * EXACT LAYOUT:
 * - 14 keys + 2 encoders in 4×4 grid
 * - Encoders replace top-left [0,0] and top-right [0,3] positions
 * - 4 corner screws
 * - Rainbow RGB underglow
 *
 * OPTIMIZATIONS:
 * - Memoized geometries and materials (avoid recreation)
 * - React.memo on components (prevent unnecessary re-renders)
 * - Shared materials across similar meshes
 * - Primitive components for reusable geometries
 * - Optimized useFrame callbacks
 */

import { useRef, useMemo, useState, memo, useCallback } from 'react';
import { Canvas, useFrame } from '@react-three/fiber';
import { OrbitControls, RoundedBox } from '@react-three/drei';
import * as THREE from 'three';

// Types
interface Props {
  selectedKey: string | null;
  activeLayer: number;
  layers: Array<{ id: number; name: string; color: string; keys: Record<string, any> }>;
  onKeyClick: (keyId: string) => void;
  onKeyHover?: (keyId: string | null) => void;
}

// Layout constants
const ROWS = 4;
const COLS = 4;
const KEY_SIZE = 0.9;
const KEY_GAP = 0.15;
const KEY_HEIGHT = 0.55;

// Encoder positions (replace top-left and top-right keys)
const ENCODER_POSITIONS: [number, number][] = [[0, 0], [0, 3]];

// Colors
const COLORS: Record<string, string> = {
  black: '#1a1a1a',
  coral: '#ef4444',
  purple: '#a855f7',
  blue: '#3b82f6',
  green: '#22c55e',
};

// Key color map [row][col]
const KEY_COLORS = [
  ['black', 'black', 'black', 'black'],
  ['black', 'coral', 'purple', 'black'],
  ['black', 'black', 'blue', 'green'],
  ['black', 'black', 'black', 'black'],
];

// Pre-computed screw positions
const SCREW_POSITIONS: [number, number][] = [
  [-2.25, -2.25], [2.25, -2.25], [-2.25, 2.25], [2.25, 2.25]
];

// Knurl ridge positions for silver encoder
const KNURL_POSITIONS = [-0.24, -0.14, -0.04, 0.06, 0.16, 0.26];

// ============================================================================
// Shared Geometries Hook
// ============================================================================
function useSharedGeometries() {
  return useMemo(() => ({
    screw: new THREE.CylinderGeometry(0.08, 0.08, 0.03, 8),
    silverBase: new THREE.BoxGeometry(0.7, 0.1, 0.5),
    silverCylinder: new THREE.CylinderGeometry(0.26, 0.26, 0.6, 48),
    silverTorus: new THREE.TorusGeometry(0.27, 0.015, 8, 48),
    silverCap: new THREE.CylinderGeometry(0.2, 0.26, 0.03, 32),
    blackBase: new THREE.CylinderGeometry(0.4, 0.4, 0.08, 24),
    blackBody: new THREE.CylinderGeometry(0.38, 0.4, 0.22, 32),
    blackTop: new THREE.CylinderGeometry(0.36, 0.38, 0.04, 32),
    indicator: new THREE.BoxGeometry(0.025, 0.16, 0.015),
    plane: new THREE.PlaneGeometry(25, 25),
  }), []);
}

// ============================================================================
// Shared Materials Hook
// ============================================================================
function useSharedMaterials() {
  return useMemo(() => ({
    screw: new THREE.MeshStandardMaterial({ color: '#333', metalness: 0.6, roughness: 0.4 }),
    chassis: new THREE.MeshStandardMaterial({ color: '#f5f5f5', roughness: 0.35, metalness: 0.05 }),
    silverBase: new THREE.MeshStandardMaterial({ color: '#222222', roughness: 0.8, metalness: 0.2 }),
    silverBody: new THREE.MeshStandardMaterial({ color: '#e8e8e8', roughness: 0.3, metalness: 0.6 }),
    silverRidge: new THREE.MeshStandardMaterial({ color: '#d0d0d0', roughness: 0.35, metalness: 0.5 }),
    silverCap: new THREE.MeshStandardMaterial({ color: '#f0f0f0', roughness: 0.2, metalness: 0.5 }),
    blackMatte: new THREE.MeshStandardMaterial({ color: '#1a1a1a', roughness: 0.9, metalness: 0.05 }),
    blackBody: new THREE.MeshStandardMaterial({ color: '#1a1a1a', roughness: 0.85, metalness: 0.05 }),
    blackTop: new THREE.MeshStandardMaterial({ color: '#252525', roughness: 0.8, metalness: 0.1 }),
    indicator: new THREE.MeshBasicMaterial({ color: '#ffffff' }),
    background: new THREE.MeshStandardMaterial({ color: '#0f172a' }),
  }), []);
}

// ============================================================================
// Simple Keycap (Memoized)
// ============================================================================
interface KeyProps {
  pos: [number, number, number];
  color: string;
  selected: boolean;
  onClick: () => void;
}

const Key = memo(function Key({ pos, color, selected, onClick }: KeyProps) {
  const ref = useRef<THREE.Mesh>(null);
  const [pressed, setPressed] = useState(false);
  const pressY = useRef(0);
  const baseY = pos[1];

  const hex = COLORS[color] || COLORS.black;
  const isColored = color !== 'black';

  // Memoize material properties to avoid recalculation
  const materialProps = useMemo(() => ({
    color: hex,
    emissive: isColored ? hex : '#000',
    roughness: isColored ? 0.25 : 0.7,
    metalness: isColored ? 0.1 : 0.05,
    envMapIntensity: isColored ? 0.8 : 0.3,
  }), [hex, isColored]);

  // Optimized useFrame - only update when animating
  useFrame((_, dt) => {
    const target = pressed ? -0.12 : 0;
    const diff = target - pressY.current;

    // Skip update if nearly at target (optimization)
    if (Math.abs(diff) < 0.001) {
      if (pressY.current !== target) {
        pressY.current = target;
        if (ref.current) ref.current.position.y = baseY + target;
      }
      return;
    }

    pressY.current += diff * dt * 15;
    if (ref.current) {
      ref.current.position.y = baseY + pressY.current;
    }
  });

  // Calculate emissive intensity based on state
  const emissiveIntensity = pressed ? 0.5 : selected ? 0.25 : isColored ? 0.08 : 0;

  const handlePointerDown = useCallback(() => {
    setPressed(true);
    onClick();
  }, [onClick]);

  const handlePointerUp = useCallback(() => setPressed(false), []);
  const handlePointerLeave = useCallback(() => setPressed(false), []);

  return (
    <RoundedBox
      ref={ref}
      args={[KEY_SIZE, KEY_HEIGHT, KEY_SIZE]}
      radius={0.1}
      smoothness={4}
      position={pos}
      castShadow
      onPointerDown={handlePointerDown}
      onPointerUp={handlePointerUp}
      onPointerLeave={handlePointerLeave}
    >
      <meshStandardMaterial
        {...materialProps}
        emissiveIntensity={emissiveIntensity}
      />
    </RoundedBox>
  );
});

// ============================================================================
// Silver Encoder (Memoized)
// ============================================================================
const SilverEncoder = memo(function SilverEncoder({ pos }: { pos: [number, number, number] }) {
  const ref = useRef<THREE.Group>(null);
  const geometries = useSharedGeometries();
  const materials = useSharedMaterials();

  useFrame((state) => {
    if (ref.current) {
      ref.current.rotation.y = Math.sin(state.clock.elapsedTime * 0.5) * 0.08;
    }
  });

  return (
    <group ref={ref} position={pos}>
      {/* Dark base/mount */}
      <mesh position={[0, 0.02, 0]} geometry={geometries.silverBase} material={materials.silverBase} />

      {/* Main silver roller - rotated to be LEFT-RIGHT horizontal */}
      <group position={[0, 0.28, 0]} rotation={[0, 0, Math.PI / 2]}>
        {/* Silver cylinder body */}
        <mesh geometry={geometries.silverCylinder} material={materials.silverBody} />

        {/* Knurled ridges */}
        {KNURL_POSITIONS.map((y, i) => (
          <mesh key={i} position={[0, y, 0]} geometry={geometries.silverTorus} material={materials.silverRidge} />
        ))}

        {/* End caps */}
        <mesh position={[0, 0.31, 0]} geometry={geometries.silverCap} material={materials.silverCap} />
        <mesh position={[0, -0.31, 0]} geometry={geometries.silverCap} material={materials.silverCap} />
      </group>
    </group>
  );
});

// ============================================================================
// Black Volume Knob (Memoized)
// ============================================================================
const BlackEncoder = memo(function BlackEncoder({ pos }: { pos: [number, number, number] }) {
  const ref = useRef<THREE.Group>(null);
  const geometries = useSharedGeometries();
  const materials = useSharedMaterials();

  useFrame((state) => {
    if (ref.current) {
      ref.current.rotation.y = Math.sin(state.clock.elapsedTime * 0.5) * 0.08;
    }
  });

  return (
    <group ref={ref} position={pos}>
      {/* Base ring */}
      <mesh position={[0, -0.02, 0]} geometry={geometries.blackBase} material={materials.blackMatte} />
      {/* Main knob body */}
      <mesh position={[0, 0.12, 0]} geometry={geometries.blackBody} material={materials.blackBody} />
      {/* Top surface */}
      <mesh position={[0, 0.24, 0]} geometry={geometries.blackTop} material={materials.blackTop} />
      {/* White indicator LINE */}
      <mesh position={[0.18, 0.27, 0]} rotation={[0, 0, Math.PI / 2]} geometry={geometries.indicator} material={materials.indicator} />
    </group>
  );
});

// ============================================================================
// Rainbow Underglow (Optimized)
// ============================================================================
const Underglow = memo(function Underglow() {
  const ref = useRef<THREE.Mesh>(null);
  const t = useRef(0);

  // Create material once
  const material = useMemo(() =>
    new THREE.MeshBasicMaterial({ color: '#ff0000', transparent: true, opacity: 0.9 }),
  []);

  useFrame((_, dt) => {
    t.current += dt;
    const hue = (t.current * 0.1) % 1;
    material.color.setHSL(hue, 1, 0.55);
  });

  return (
    <RoundedBox
      ref={ref}
      args={[6.5, 0.1, 6.5]}
      radius={0.5}
      smoothness={4}
      position={[0, -1.3, 0]}
      material={material}
    />
  );
});

// ============================================================================
// Chassis (Memoized with shared geometries)
// ============================================================================
const Chassis = memo(function Chassis() {
  const geometries = useSharedGeometries();
  const materials = useSharedMaterials();

  return (
    <group>
      {/* Main chassis */}
      <RoundedBox
        args={[5.1, 0.5, 5.1]}
        radius={0.3}
        smoothness={2}
        position={[0, -0.25, 0]}
        receiveShadow
        castShadow
        material={materials.chassis}
      />
      {/* Screws - using shared geometry and material */}
      {SCREW_POSITIONS.map(([x, z], i) => (
        <mesh
          key={i}
          position={[x, 0.02, z]}
          geometry={geometries.screw}
          material={materials.screw}
        />
      ))}
    </group>
  );
});

// ============================================================================
// Background (Memoized)
// ============================================================================
const Background = memo(function Background() {
  const geometries = useSharedGeometries();
  const materials = useSharedMaterials();

  return (
    <>
      <mesh
        rotation={[-Math.PI / 2, 0, 0]}
        position={[0, -0.45, 0]}
        receiveShadow
        geometry={geometries.plane}
        material={materials.background}
      />
      <gridHelper args={[25, 25, '#1e3a5f', '#0c1929']} position={[0, -0.44, 0]} />
    </>
  );
});

// ============================================================================
// Lights (Memoized - lights are expensive)
// ============================================================================
const Lights = memo(function Lights() {
  return (
    <>
      <ambientLight intensity={0.5} />
      <directionalLight position={[5, 10, 5]} intensity={1.5} castShadow />
      <directionalLight position={[-5, 8, -3]} intensity={0.8} color="#ffffff" />
      <directionalLight position={[0, 5, -8]} intensity={0.4} color="#e0e0ff" />
      <pointLight position={[-3, 4, -2]} intensity={0.5} color="#a855f7" />
      <pointLight position={[3, 3, 2]} intensity={0.4} color="#3b82f6" />
      <spotLight position={[0, 8, 0]} intensity={0.6} angle={0.5} penumbra={1} />
    </>
  );
});

// ============================================================================
// Scene (Pre-compute key positions)
// ============================================================================
function Scene({ selectedKey, onKeyClick }: Props) {
  // Pre-compute all key data once
  const keyData = useMemo(() => {
    const spacing = KEY_SIZE + KEY_GAP;
    const startX = -((COLS - 1) * spacing) / 2;
    const startZ = -((ROWS - 1) * spacing) / 2;

    const keys: Array<{
      id: string;
      row: number;
      col: number;
      x: number;
      z: number;
      isEncoder: boolean;
      isSilver: boolean;
      color: string;
    }> = [];

    for (let row = 0; row < ROWS; row++) {
      for (let col = 0; col < COLS; col++) {
        const id = `${row},${col}`;
        const x = startX + col * spacing;
        const z = startZ + row * spacing;
        const isEncoder = ENCODER_POSITIONS.some(([r, c]) => r === row && c === col);

        keys.push({
          id,
          row,
          col,
          x,
          z,
          isEncoder,
          isSilver: col === 0,
          color: KEY_COLORS[row][col],
        });
      }
    }

    return keys;
  }, []);

  return (
    <>
      <Lights />
      <Background />
      <Chassis />
      <Underglow />

      {/* 14 Keys + 2 Encoders in 4×4 grid */}
      {keyData.map((key) => {
        if (key.isEncoder) {
          return key.isSilver
            ? <SilverEncoder key={key.id} pos={[key.x, 0.15, key.z]} />
            : <BlackEncoder key={key.id} pos={[key.x, 0.15, key.z]} />;
        }

        return (
          <Key
            key={key.id}
            pos={[key.x, KEY_HEIGHT / 2, key.z]}
            color={key.color}
            selected={selectedKey === key.id}
            onClick={() => onKeyClick(key.id)}
          />
        );
      })}

      <OrbitControls
        enablePan={false}
        minDistance={5}
        maxDistance={12}
        minPolarAngle={0.3}
        maxPolarAngle={1.3}
        target={[0, 0, 0.5]}
      />
    </>
  );
}

// ============================================================================
// Export (with performance settings)
// ============================================================================
export function CreatorBoardLowPoly(props: Props) {
  return (
    <div style={{ width: '100%', height: '100%', background: '#0f172a' }}>
      <Canvas
        shadows
        camera={{ position: [0, 5, 8], fov: 40 }}
        // Performance optimizations
        gl={{
          antialias: true,
          powerPreference: 'high-performance',
        }}
        // Always render for continuous animations (rainbow, encoders)
        frameloop="always"
      >
        <color attach="background" args={['#0f172a']} />
        <fog attach="fog" args={['#0f172a', 12, 25]} />
        <Scene {...props} />
      </Canvas>
    </div>
  );
}

export default CreatorBoardLowPoly;
