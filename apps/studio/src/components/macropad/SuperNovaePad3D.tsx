'use client';

/**
 * SuperNovaePad3D - Work Louder Creator Micro 3D Visualizer
 *
 * Layout (top view, from photo):
 * ┌─────────────────────────────────────────────┐
 * │  ◎           [USB-C]              ●         │  ◎ = encoder (knurled, top-left)
 * │     [R] [R] [R] [C]                         │  ● = big black knob (top-right)
 * │     [P] [P] [C] [C]                         │  R = red, P = purple
 * │     [M] [M] [M] [M]                         │  C = cyan, M = mint
 * │  ⊞                                    ☺     │  ⊞ = Figma logo, ☺ = smiley
 * └─────────────────────────────────────────────┘
 *
 * Matrix: 4 cols × 3 rows = 12 keys
 */

import { useMemo, useRef } from 'react';
import { Canvas, useFrame } from '@react-three/fiber';
import { OrbitControls, RoundedBox } from '@react-three/drei';
import * as THREE from 'three';

// =============================================================================
// Types
// =============================================================================

interface SuperNovaePad3DProps {
  selectedKey: string | null;
  activeLayer: number;
  layers: Array<{
    id: number;
    name: string;
    color: string;
    keys: Record<string, { key: string; label: string; action: string }>;
  }>;
  onKeyClick: (keyId: string) => void;
  onKeyHover?: (keyId: string | null) => void;
}

// =============================================================================
// Constants
// =============================================================================

const KEY_SIZE = 1.2;
const KEY_HEIGHT = 0.5;
const KEY_GAP = 0.15;
const KEY_RADIUS = 0.15;

// Key colors matching the photo (row, col) - 4 cols × 3 rows
const KEY_COLORS: string[][] = [
  ['#ff6b6b', '#ff6b6b', '#ff6b6b', '#4ecdc4'], // Row 0: red, red, red, cyan
  ['#a855f7', '#a855f7', '#4ecdc4', '#4ecdc4'], // Row 1: purple, purple, cyan, cyan
  ['#6ee7b7', '#6ee7b7', '#6ee7b7', '#6ee7b7'], // Row 2: mint, mint, mint, mint
];

// =============================================================================
// Gummy Keycap
// =============================================================================

function GummyKey({
  position,
  color,
  isSelected,
  onClick,
  onPointerEnter,
  onPointerLeave,
}: {
  position: [number, number, number];
  color: string;
  isSelected: boolean;
  onClick: () => void;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
}) {
  const meshRef = useRef<THREE.Mesh>(null);

  // Subtle hover animation
  useFrame(() => {
    if (meshRef.current) {
      const targetY = isSelected ? position[1] + 0.05 : position[1];
      meshRef.current.position.y += (targetY - meshRef.current.position.y) * 0.1;
    }
  });

  return (
    <group position={position}>
      {/* Glow under keycap */}
      <pointLight
        position={[0, -0.1, 0]}
        color={color}
        intensity={isSelected ? 3 : 1.5}
        distance={1.5}
        decay={2}
      />

      {/* Gummy keycap */}
      <RoundedBox
        ref={meshRef}
        args={[KEY_SIZE, KEY_HEIGHT, KEY_SIZE]}
        radius={KEY_RADIUS}
        smoothness={4}
        onClick={onClick}
        onPointerEnter={onPointerEnter}
        onPointerLeave={onPointerLeave}
        castShadow
      >
        <meshStandardMaterial
          color={color}
          emissive={color}
          emissiveIntensity={isSelected ? 0.4 : 0.15}
          roughness={0.3}
          metalness={0.1}
          transparent
          opacity={0.95}
        />
      </RoundedBox>

      {/* Selection ring */}
      {isSelected && (
        <mesh position={[0, KEY_HEIGHT / 2 + 0.02, 0]} rotation={[-Math.PI / 2, 0, 0]}>
          <ringGeometry args={[KEY_SIZE * 0.4, KEY_SIZE * 0.45, 32]} />
          <meshBasicMaterial color="#ffffff" transparent opacity={0.8} />
        </mesh>
      )}
    </group>
  );
}

// =============================================================================
// Encoder (Knurled, small - top left)
// =============================================================================

function Encoder({ position }: { position: [number, number, number] }) {
  return (
    <group position={position}>
      {/* Base */}
      <mesh position={[0, -0.1, 0]} castShadow>
        <cylinderGeometry args={[0.45, 0.45, 0.2, 24]} />
        <meshStandardMaterial color="#2a2a2a" metalness={0.8} roughness={0.3} />
      </mesh>

      {/* Knurled knob */}
      <mesh castShadow>
        <cylinderGeometry args={[0.4, 0.4, 0.5, 24]} />
        <meshStandardMaterial color="#888888" metalness={0.9} roughness={0.2} />
      </mesh>

      {/* Knurling lines */}
      {Array.from({ length: 16 }).map((_, i) => {
        const angle = (i / 16) * Math.PI * 2;
        return (
          <mesh
            key={i}
            position={[Math.cos(angle) * 0.38, 0, Math.sin(angle) * 0.38]}
          >
            <boxGeometry args={[0.04, 0.45, 0.02]} />
            <meshStandardMaterial color="#666666" metalness={0.8} />
          </mesh>
        );
      })}
    </group>
  );
}

// =============================================================================
// Big Black Knob (top right)
// =============================================================================

function BigKnob({ position }: { position: [number, number, number] }) {
  return (
    <group position={position}>
      {/* Base ring */}
      <mesh position={[0, -0.15, 0]} castShadow>
        <cylinderGeometry args={[0.75, 0.75, 0.1, 32]} />
        <meshStandardMaterial color="#1a1a1a" metalness={0.3} roughness={0.7} />
      </mesh>

      {/* Main knob */}
      <mesh castShadow>
        <cylinderGeometry args={[0.65, 0.7, 0.6, 32]} />
        <meshStandardMaterial color="#1a1a1a" metalness={0.2} roughness={0.8} />
      </mesh>

      {/* Top indent */}
      <mesh position={[0, 0.25, 0]}>
        <cylinderGeometry args={[0.55, 0.55, 0.1, 32]} />
        <meshStandardMaterial color="#0f0f0f" metalness={0.3} roughness={0.6} />
      </mesh>
    </group>
  );
}

// =============================================================================
// Chassis (white translucent with rounded corners)
// =============================================================================

function Chassis({ layerColor }: { layerColor: string }) {
  return (
    <group>
      {/* Main body - white translucent */}
      <RoundedBox
        args={[9, 0.8, 7]}
        radius={0.4}
        smoothness={4}
        position={[0, -0.4, 0]}
        receiveShadow
        castShadow
      >
        <meshStandardMaterial
          color="#f5f5f5"
          transparent
          opacity={0.95}
          roughness={0.4}
          metalness={0.1}
        />
      </RoundedBox>

      {/* Top plate - dark PCB */}
      <RoundedBox
        args={[8.5, 0.1, 6.5]}
        radius={0.3}
        smoothness={4}
        position={[0, 0.02, 0]}
        receiveShadow
      >
        <meshStandardMaterial color="#1a1a1a" roughness={0.6} />
      </RoundedBox>

      {/* RGB underglow */}
      <mesh position={[0, -0.81, 0]} rotation={[-Math.PI / 2, 0, 0]}>
        <planeGeometry args={[8, 6]} />
        <meshBasicMaterial color={layerColor} transparent opacity={0.4} />
      </mesh>

      {/* Corner screws */}
      {[
        [-3.8, 0.1, -2.8],
        [3.8, 0.1, -2.8],
        [-3.8, 0.1, 2.8],
        [3.8, 0.1, 2.8],
      ].map((pos, i) => (
        <mesh key={i} position={pos as [number, number, number]} castShadow>
          <cylinderGeometry args={[0.15, 0.15, 0.15, 12]} />
          <meshStandardMaterial color="#2a2a2a" metalness={0.7} roughness={0.3} />
        </mesh>
      ))}

      {/* USB-C port */}
      <mesh position={[0, 0.1, -3.3]} rotation={[Math.PI / 2, 0, 0]}>
        <boxGeometry args={[0.9, 0.35, 0.25]} />
        <meshStandardMaterial color="#444444" metalness={0.6} roughness={0.4} />
      </mesh>

      {/* USB-C inner */}
      <mesh position={[0, 0.1, -3.25]} rotation={[Math.PI / 2, 0, 0]}>
        <boxGeometry args={[0.6, 0.2, 0.15]} />
        <meshStandardMaterial color="#1a1a1a" />
      </mesh>
    </group>
  );
}

// =============================================================================
// Grid Background
// =============================================================================

function Grid() {
  return (
    <group position={[0, -1.2, 0]}>
      <mesh rotation={[-Math.PI / 2, 0, 0]} receiveShadow>
        <planeGeometry args={[40, 40]} />
        <meshStandardMaterial color="#0f172a" />
      </mesh>
      <gridHelper args={[40, 40, '#1e3a5f', '#0f2744']} position={[0, 0.01, 0]} />
    </group>
  );
}

// =============================================================================
// Scene
// =============================================================================

function Scene({ selectedKey, activeLayer, layers, onKeyClick, onKeyHover }: SuperNovaePad3DProps) {
  const currentLayer = layers[activeLayer] || layers[0];
  const layerColor = currentLayer?.color || '#00FFFF';

  // Build 4×3 key grid
  const keys = useMemo(() => {
    const result: { id: string; row: number; col: number; color: string }[] = [];
    for (let row = 0; row < 3; row++) {
      for (let col = 0; col < 4; col++) {
        result.push({
          id: `${row},${col}`,
          row,
          col,
          color: KEY_COLORS[row]?.[col] || '#a855f7',
        });
      }
    }
    return result;
  }, []);

  // Key positions - 4×3 grid, offset to leave room for encoder and knob
  const getKeyPosition = (row: number, col: number): [number, number, number] => {
    const spacing = KEY_SIZE + KEY_GAP;
    const offsetX = -0.5; // Center the grid slightly left
    const offsetZ = 0.3; // Shift down a bit
    return [
      offsetX + (col - 1.5) * spacing,
      KEY_HEIGHT / 2 + 0.1,
      offsetZ + (row - 1) * spacing,
    ];
  };

  return (
    <>
      {/* Lighting */}
      <ambientLight intensity={0.6} />
      <directionalLight position={[5, 10, 5]} intensity={1.2} castShadow />
      <pointLight position={[-5, 5, -5]} intensity={0.5} color="#8b5cf6" />
      <pointLight position={[5, 3, 5]} intensity={0.3} color="#00FFFF" />

      {/* Grid background */}
      <Grid />

      {/* Chassis */}
      <Chassis layerColor={layerColor} />

      {/* 4×3 Keys */}
      {keys.map((key) => (
        <GummyKey
          key={key.id}
          position={getKeyPosition(key.row, key.col)}
          color={key.color}
          isSelected={selectedKey === key.id}
          onClick={() => onKeyClick(key.id)}
          onPointerEnter={() => onKeyHover?.(key.id)}
          onPointerLeave={() => onKeyHover?.(null)}
        />
      ))}

      {/* Encoder (top-left, next to keys) */}
      <Encoder position={[-3.2, 0.35, -2.2]} />

      {/* Big black knob (top-right) */}
      <BigKnob position={[3.2, 0.4, -2.2]} />

      {/* Camera controls */}
      <OrbitControls
        enablePan={false}
        minDistance={8}
        maxDistance={18}
        minPolarAngle={0.3}
        maxPolarAngle={1.4}
        target={[0, 0, 0]}
      />
    </>
  );
}

// =============================================================================
// Export
// =============================================================================

export function SuperNovaePad3D(props: SuperNovaePad3DProps) {
  return (
    <div style={{ width: '100%', height: '100%', background: '#0f172a' }}>
      <Canvas
        shadows
        camera={{ position: [0, 10, 12], fov: 35 }}
        onCreated={(state) => {
          state.gl.toneMapping = THREE.ACESFilmicToneMapping;
          state.gl.toneMappingExposure = 1.2;
        }}
      >
        <color attach="background" args={['#0f172a']} />
        <Scene {...props} />
      </Canvas>
    </div>
  );
}

export default SuperNovaePad3D;
