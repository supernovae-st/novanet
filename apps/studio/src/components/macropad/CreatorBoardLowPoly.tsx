'use client';

/**
 * Work Louder Creator Micro - 3D Visualizer
 *
 * EXACT LAYOUT:
 * - 14 keys + 2 encoders in 4×4 grid
 * - Encoders replace top-left [0,0] and top-right [0,3] positions
 * - 4 corner screws
 * - Rainbow RGB underglow
 *
 * LAYOUT:
 * [ENC1]  [black]  [black]  [ENC2]    <- Row 0 (encoders at corners)
 * [black] [CORAL]  [PURPLE] [black]   <- Row 1
 * [black] [black]  [BLUE]   [GREEN]   <- Row 2
 * [black] [black]  [black]  [black]   <- Row 3
 */

import { useRef, useMemo, useState } from 'react';
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

// Layout
const ROWS = 4;
const COLS = 4;
const KEY_SIZE = 0.9;
const KEY_GAP = 0.15;
const KEY_HEIGHT = 0.55; // Increased from 0.35 for better clickability

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

// ============================================================================
// Simple Keycap
// ============================================================================
function Key({
  pos,
  color,
  selected,
  onClick,
}: {
  pos: [number, number, number];
  color: string;
  selected: boolean;
  onClick: () => void;
}) {
  const ref = useRef<THREE.Mesh>(null);
  const [pressed, setPressed] = useState(false);
  const pressY = useRef(0);

  const hex = COLORS[color] || COLORS.black;
  const isColored = color !== 'black';

  useFrame((_, dt) => {
    const target = pressed ? -0.12 : 0;
    pressY.current += (target - pressY.current) * dt * 15;
    if (ref.current) {
      ref.current.position.y = pos[1] + pressY.current;
    }
  });

  return (
    <RoundedBox
      ref={ref}
      args={[KEY_SIZE, KEY_HEIGHT, KEY_SIZE]}
      radius={0.1}
      smoothness={4}
      position={pos}
      castShadow
      onPointerDown={() => { setPressed(true); onClick(); }}
      onPointerUp={() => setPressed(false)}
      onPointerLeave={() => setPressed(false)}
    >
      <meshStandardMaterial
        color={hex}
        emissive={isColored ? hex : '#000'}
        emissiveIntensity={pressed ? 0.5 : selected ? 0.25 : isColored ? 0.08 : 0}
        roughness={isColored ? 0.25 : 0.7}
        metalness={isColored ? 0.1 : 0.05}
        envMapIntensity={isColored ? 0.8 : 0.3}
      />
    </RoundedBox>
  );
}

// ============================================================================
// Encoder - Silver Knurled (left) or Black Volume Knob (right)
// ============================================================================
function Encoder({ pos, silver }: { pos: [number, number, number]; silver: boolean }) {
  const ref = useRef<THREE.Group>(null);

  useFrame((s) => {
    if (ref.current) ref.current.rotation.y = Math.sin(s.clock.elapsedTime * 0.5) * 0.08;
  });

  if (silver) {
    // SILVER SCROLL WHEEL - horizontal LEFT-RIGHT
    return (
      <group ref={ref} position={pos}>
        {/* Dark base/mount */}
        <mesh position={[0, 0.02, 0]}>
          <boxGeometry args={[0.7, 0.1, 0.5]} />
          <meshStandardMaterial color="#222222" roughness={0.8} metalness={0.2} />
        </mesh>
        {/* Main silver roller - rotated to be LEFT-RIGHT horizontal */}
        <group position={[0, 0.28, 0]} rotation={[0, 0, Math.PI / 2]}>
          {/* Silver cylinder body - BRIGHT GRAY */}
          <mesh>
            <cylinderGeometry args={[0.26, 0.26, 0.6, 48]} />
            <meshStandardMaterial
              color="#e8e8e8"
              roughness={0.3}
              metalness={0.6}
            />
          </mesh>
          {/* Knurled ridges around the roller - lighter */}
          {[-0.24, -0.14, -0.04, 0.06, 0.16, 0.26].map((y, i) => (
            <mesh key={i} position={[0, y, 0]}>
              <torusGeometry args={[0.27, 0.015, 8, 48]} />
              <meshStandardMaterial
                color="#d0d0d0"
                roughness={0.35}
                metalness={0.5}
              />
            </mesh>
          ))}
          {/* End caps - bright silver */}
          <mesh position={[0, 0.31, 0]}>
            <cylinderGeometry args={[0.2, 0.26, 0.03, 32]} />
            <meshStandardMaterial color="#f0f0f0" roughness={0.2} metalness={0.5} />
          </mesh>
          <mesh position={[0, -0.31, 0]}>
            <cylinderGeometry args={[0.2, 0.26, 0.03, 32]} />
            <meshStandardMaterial color="#f0f0f0" roughness={0.2} metalness={0.5} />
          </mesh>
        </group>
      </group>
    );
  }

  // BLACK VOLUME KNOB - flat with white indicator line
  return (
    <group ref={ref} position={pos}>
      {/* Base ring */}
      <mesh position={[0, -0.02, 0]}>
        <cylinderGeometry args={[0.4, 0.4, 0.08, 24]} />
        <meshStandardMaterial color="#1a1a1a" roughness={0.9} metalness={0.05} />
      </mesh>
      {/* Main knob body - flatter */}
      <mesh position={[0, 0.12, 0]}>
        <cylinderGeometry args={[0.38, 0.4, 0.22, 32]} />
        <meshStandardMaterial color="#1a1a1a" roughness={0.85} metalness={0.05} />
      </mesh>
      {/* Top surface - slightly domed */}
      <mesh position={[0, 0.24, 0]}>
        <cylinderGeometry args={[0.36, 0.38, 0.04, 32]} />
        <meshStandardMaterial color="#252525" roughness={0.8} metalness={0.1} />
      </mesh>
      {/* White indicator LINE on top */}
      <mesh position={[0.18, 0.27, 0]} rotation={[0, 0, Math.PI / 2]}>
        <boxGeometry args={[0.025, 0.16, 0.015]} />
        <meshBasicMaterial color="#ffffff" />
      </mesh>
    </group>
  );
}

// ============================================================================
// Rainbow Underglow - Solid Square
// ============================================================================
function Underglow() {
  const ref = useRef<THREE.Mesh>(null);
  const t = useRef(0);

  useFrame((_, dt) => {
    t.current += dt;
    if (ref.current) {
      const hue = (t.current * 0.1) % 1;
      (ref.current.material as THREE.MeshBasicMaterial).color.setHSL(hue, 1, 0.55);
    }
  });

  return (
    <RoundedBox
      ref={ref}
      args={[6.5, 0.1, 6.5]}
      radius={0.5}
      smoothness={4}
      position={[0, -1.3, 0]}
    >
      <meshBasicMaterial color="#ff0000" transparent opacity={0.9} />
    </RoundedBox>
  );
}

// ============================================================================
// Chassis
// ============================================================================
function Chassis() {
  return (
    <group>
      {/* Slightly larger chassis */}
      <RoundedBox args={[5.1, 0.5, 5.1]} radius={0.3} smoothness={2} position={[0, -0.25, 0]} receiveShadow castShadow>
        <meshStandardMaterial color="#f5f5f5" roughness={0.35} metalness={0.05} />
      </RoundedBox>
      {/* Screws */}
      {[[-2.25, -2.25], [2.25, -2.25], [-2.25, 2.25], [2.25, 2.25]].map(([x, z], i) => (
        <mesh key={i} position={[x, 0.02, z]}>
          <cylinderGeometry args={[0.08, 0.08, 0.03, 8]} />
          <meshStandardMaterial color="#333" metalness={0.6} roughness={0.4} />
        </mesh>
      ))}
    </group>
  );
}

// ============================================================================
// Scene
// ============================================================================
function Scene({ selectedKey, onKeyClick }: Props) {
  const spacing = KEY_SIZE + KEY_GAP;
  const startX = -((COLS - 1) * spacing) / 2;
  const startZ = -((ROWS - 1) * spacing) / 2; // Centered, no offset

  return (
    <>
      {/* Lighting setup for metallic reflections */}
      <ambientLight intensity={0.5} />
      <directionalLight position={[5, 10, 5]} intensity={1.5} castShadow />
      <directionalLight position={[-5, 8, -3]} intensity={0.8} color="#ffffff" />
      <directionalLight position={[0, 5, -8]} intensity={0.4} color="#e0e0ff" />
      <pointLight position={[-3, 4, -2]} intensity={0.5} color="#a855f7" />
      <pointLight position={[3, 3, 2]} intensity={0.4} color="#3b82f6" />
      {/* Rim light for metallic highlights */}
      <spotLight position={[0, 8, 0]} intensity={0.6} angle={0.5} penumbra={1} />

      {/* Background */}
      <mesh rotation={[-Math.PI / 2, 0, 0]} position={[0, -0.45, 0]} receiveShadow>
        <planeGeometry args={[25, 25]} />
        <meshStandardMaterial color="#0f172a" />
      </mesh>
      <gridHelper args={[25, 25, '#1e3a5f', '#0c1929']} position={[0, -0.44, 0]} />

      <Chassis />
      <Underglow />

      {/* 14 Keys + 2 Encoders in 4×4 grid */}
      {Array.from({ length: ROWS }).map((_, row) =>
        Array.from({ length: COLS }).map((_, col) => {
          const id = `${row},${col}`;
          const x = startX + col * spacing;
          const z = startZ + row * spacing;

          // Check if this position is an encoder
          const isEncoder = ENCODER_POSITIONS.some(([r, c]) => r === row && c === col);

          if (isEncoder) {
            // Silver encoder on left (col 0), black on right (col 3)
            return <Encoder key={id} pos={[x, 0.15, z]} silver={col === 0} />;
          }

          return (
            <Key
              key={id}
              pos={[x, KEY_HEIGHT / 2, z]}
              color={KEY_COLORS[row][col]}
              selected={selectedKey === id}
              onClick={() => onKeyClick(id)}
            />
          );
        })
      )}

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
// Export
// ============================================================================
export function CreatorBoardLowPoly(props: Props) {
  return (
    <div style={{ width: '100%', height: '100%', background: '#0f172a' }}>
      <Canvas shadows camera={{ position: [0, 5, 8], fov: 40 }}>
        <color attach="background" args={['#0f172a']} />
        <fog attach="fog" args={['#0f172a', 12, 25]} />
        <Scene {...props} />
      </Canvas>
    </div>
  );
}

export default CreatorBoardLowPoly;
