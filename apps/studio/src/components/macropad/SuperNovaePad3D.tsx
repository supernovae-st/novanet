'use client';

/**
 * SuperNovaePad3D - Work Louder Creator Micro 3D Visualizer
 *
 * v4.1 - Figma Reference Edition
 * Based on official Work Louder x Figma design
 *
 * Layout:
 * ┌─────────────────────────────────────┐
 * │ ENC1   KEY KEY KEY   ENC2           │ Row 0
 * │ KEY    KEY KEY KEY                  │ Row 1
 * │ KEY    KEY KEY KEY                  │ Row 2
 * │ enc3       KEY KEY   opt            │ Row 3 (subtle controls)
 * └─────────────────────────────────────┘
 *
 * ENC3 and OPT button are subtle (reduced opacity) as they're secondary controls.
 */

import { useMemo, useRef, useState } from 'react';
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

const KEY_SIZE = 0.9;
const KEY_HEIGHT = 0.4;
const KEY_GAP = 0.15;
const KEY_RADIUS = 0.15; // Rounded corners like Figma
const PRESS_DEPTH = 0.06;

// Colors from Figma design
const COLORS = {
  red: '#ef4444',
  purple: '#8b5cf6',
  blue: '#3b82f6',
  green: '#22c55e',
  white: '#f8fafc',
};

// Key layout with colors (matching Figma)
// Format: [row][col] = color or null for encoder/empty position
const KEY_LAYOUT: (string | null)[][] = [
  [null, COLORS.red, COLORS.red, COLORS.red, null],     // Row 0: ENC1, 3 red keys, ENC2
  [COLORS.red, COLORS.purple, COLORS.blue, COLORS.blue], // Row 1: red, purple, blue, blue
  [COLORS.purple, COLORS.purple, COLORS.green, COLORS.green], // Row 2: purple, purple, green, green
  [null, COLORS.blue, COLORS.green, null],               // Row 3: (enc3), 2 center keys, (opt)
];

// Chassis - white/frosted like real device
const CHASSIS_WHITE = '#f1f5f9';
const CHASSIS_LIGHT = '#e2e8f0';
const PCB_COLOR = '#1e293b';

const CHASSIS_WIDTH = 6.5;
const CHASSIS_DEPTH_Z = 6.5;
const CHASSIS_HEIGHT = 0.6;

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
  const groupRef = useRef<THREE.Group>(null);
  const [isPressed, setIsPressed] = useState(false);
  const [isHovered, setIsHovered] = useState(false);
  const pressProgress = useRef(0);

  useFrame((_, delta) => {
    if (!groupRef.current) return;
    const target = isPressed ? 1 : 0;
    pressProgress.current += (target - pressProgress.current) * delta * 20;
    groupRef.current.position.y = position[1] - pressProgress.current * PRESS_DEPTH;
  });

  const handleClick = () => {
    setIsPressed(true);
    onClick();
    setTimeout(() => setIsPressed(false), 100);
  };

  return (
    <group ref={groupRef} position={position}>
      {/* LED glow */}
      <pointLight
        position={[0, -0.1, 0]}
        color={color}
        intensity={isSelected ? 4 : isHovered ? 2.5 : 1.5}
        distance={1.5}
        decay={2}
      />

      {/* Key stem */}
      <mesh position={[0, -0.08, 0]}>
        <boxGeometry args={[KEY_SIZE * 0.4, 0.1, KEY_SIZE * 0.4]} />
        <meshStandardMaterial color={PCB_COLOR} />
      </mesh>

      {/* Main keycap - gummy style with rounded corners */}
      <RoundedBox
        args={[KEY_SIZE, KEY_HEIGHT, KEY_SIZE]}
        radius={KEY_RADIUS}
        smoothness={4}
        position={[0, KEY_HEIGHT * 0.5, 0]}
        onClick={handleClick}
        onPointerDown={() => setIsPressed(true)}
        onPointerUp={() => setIsPressed(false)}
        onPointerLeave={() => {
          setIsPressed(false);
          setIsHovered(false);
          onPointerLeave?.();
        }}
        onPointerEnter={() => {
          setIsHovered(true);
          onPointerEnter?.();
        }}
        castShadow
      >
        <meshPhysicalMaterial
          color={color}
          emissive={color}
          emissiveIntensity={isSelected ? 0.5 : isHovered ? 0.3 : 0.15}
          roughness={0.25}
          metalness={0}
          clearcoat={0.9}
          clearcoatRoughness={0.2}
        />
      </RoundedBox>

      {/* Top highlight */}
      <mesh position={[0, KEY_HEIGHT * 0.75, 0]} rotation={[-Math.PI / 2, 0, 0]}>
        <circleGeometry args={[KEY_SIZE * 0.25, 32]} />
        <meshBasicMaterial color="#ffffff" transparent opacity={isSelected ? 0.35 : 0.12} />
      </mesh>

      {/* Selection ring */}
      {isSelected && (
        <mesh position={[0, KEY_HEIGHT * 0.78, 0]} rotation={[-Math.PI / 2, 0, 0]}>
          <ringGeometry args={[KEY_SIZE * 0.38, KEY_SIZE * 0.44, 32]} />
          <meshBasicMaterial color="#ffffff" />
        </mesh>
      )}
    </group>
  );
}

// =============================================================================
// Silver Encoder (knurled metal - ENC1 main, ENC3 subtle)
// =============================================================================

function SilverEncoder({
  position,
  size = 1,
  subtle = false
}: {
  position: [number, number, number];
  size?: number;
  subtle?: boolean;
}) {
  const knobRef = useRef<THREE.Mesh>(null);

  useFrame((state) => {
    if (knobRef.current) {
      knobRef.current.rotation.y = Math.sin(state.clock.elapsedTime * 0.4) * 0.05;
    }
  });

  const s = size;
  const opacity = subtle ? 0.4 : 1;

  return (
    <group position={position}>
      {/* Base ring - dark */}
      <mesh position={[0, -0.05 * s, 0]} castShadow={!subtle}>
        <cylinderGeometry args={[0.42 * s, 0.45 * s, 0.1 * s, 32]} />
        <meshStandardMaterial
          color="#374151"
          metalness={0.8}
          roughness={0.3}
          transparent={subtle}
          opacity={opacity}
        />
      </mesh>

      {/* Knurled cylinder - silver metallic */}
      <mesh ref={knobRef} castShadow={!subtle}>
        <cylinderGeometry args={[0.35 * s, 0.35 * s, 0.45 * s, 32]} />
        <meshStandardMaterial
          color="#9ca3af"
          metalness={0.95}
          roughness={0.15}
          transparent={subtle}
          opacity={opacity}
        />
      </mesh>

      {/* Knurling grooves - skip for subtle */}
      {!subtle && Array.from({ length: 24 }).map((_, i) => {
        const angle = (i / 24) * Math.PI * 2;
        return (
          <mesh
            key={i}
            position={[Math.cos(angle) * 0.33 * s, 0, Math.sin(angle) * 0.33 * s]}
            rotation={[0, -angle, 0]}
          >
            <boxGeometry args={[0.02 * s, 0.4 * s, 0.008 * s]} />
            <meshStandardMaterial color="#6b7280" metalness={0.9} roughness={0.2} />
          </mesh>
        );
      })}

      {/* Top face - flat silver */}
      <mesh position={[0, 0.23 * s, 0]}>
        <cylinderGeometry args={[0.28 * s, 0.32 * s, 0.05 * s, 32]} />
        <meshStandardMaterial
          color="#d1d5db"
          metalness={0.9}
          roughness={0.1}
          transparent={subtle}
          opacity={opacity}
        />
      </mesh>
    </group>
  );
}

// =============================================================================
// Tiny Optional Button (bottom right - very subtle)
// =============================================================================

function TinyButton({ position }: { position: [number, number, number] }) {
  return (
    <group position={position}>
      {/* Very small, subtle circular button */}
      <mesh>
        <cylinderGeometry args={[0.18, 0.18, 0.08, 16]} />
        <meshStandardMaterial
          color="#e2e8f0"
          roughness={0.5}
          transparent
          opacity={0.35}
        />
      </mesh>
      {/* Tiny center dot */}
      <mesh position={[0, 0.05, 0]}>
        <cylinderGeometry args={[0.06, 0.06, 0.02, 12]} />
        <meshStandardMaterial
          color="#94a3b8"
          transparent
          opacity={0.4}
        />
      </mesh>
    </group>
  );
}

// =============================================================================
// Black Volume Knob (ENC2)
// =============================================================================

function BlackKnob({ position }: { position: [number, number, number] }) {
  const knobRef = useRef<THREE.Group>(null);

  useFrame((state) => {
    if (knobRef.current) {
      knobRef.current.rotation.y = Math.sin(state.clock.elapsedTime * 0.3) * 0.03;
    }
  });

  return (
    <group position={position} ref={knobRef}>
      {/* Base ring - dark */}
      <mesh position={[0, -0.06, 0]} castShadow>
        <cylinderGeometry args={[0.48, 0.5, 0.08, 32]} />
        <meshStandardMaterial color="#374151" metalness={0.7} roughness={0.35} />
      </mesh>

      {/* Main body - matte black */}
      <mesh castShadow>
        <cylinderGeometry args={[0.42, 0.44, 0.5, 32]} />
        <meshStandardMaterial color="#1f2937" metalness={0.15} roughness={0.85} />
      </mesh>

      {/* Top - slightly concave look */}
      <mesh position={[0, 0.22, 0]}>
        <cylinderGeometry args={[0.35, 0.38, 0.08, 32]} />
        <meshStandardMaterial color="#111827" metalness={0.2} roughness={0.8} />
      </mesh>

      {/* Position indicator - white line */}
      <mesh position={[0, 0.27, 0.32]} rotation={[Math.PI / 2, 0, 0]}>
        <boxGeometry args={[0.05, 0.08, 0.015]} />
        <meshBasicMaterial color="#ffffff" />
      </mesh>
    </group>
  );
}

// =============================================================================
// White Frosted Chassis
// =============================================================================

function Chassis({ layerColor }: { layerColor: string }) {
  return (
    <group>
      {/* Main body - white frosted */}
      <RoundedBox
        args={[CHASSIS_WIDTH, CHASSIS_HEIGHT, CHASSIS_DEPTH_Z]}
        radius={0.4}
        smoothness={4}
        position={[0, -CHASSIS_HEIGHT / 2, 0]}
        receiveShadow
        castShadow
      >
        <meshPhysicalMaterial
          color={CHASSIS_WHITE}
          roughness={0.4}
          metalness={0}
          clearcoat={0.2}
          clearcoatRoughness={0.6}
        />
      </RoundedBox>

      {/* Inner plate - slightly darker */}
      <RoundedBox
        args={[CHASSIS_WIDTH - 0.3, 0.08, CHASSIS_DEPTH_Z - 0.3]}
        radius={0.3}
        smoothness={4}
        position={[0, 0.04, 0]}
      >
        <meshStandardMaterial color={CHASSIS_LIGHT} roughness={0.5} />
      </RoundedBox>

      {/* PCB plate - dark where keys mount */}
      <RoundedBox
        args={[CHASSIS_WIDTH - 0.6, 0.06, CHASSIS_DEPTH_Z - 0.6]}
        radius={0.25}
        smoothness={4}
        position={[0, 0.08, 0]}
      >
        <meshStandardMaterial color={PCB_COLOR} roughness={0.8} />
      </RoundedBox>

      {/* Corner screws - black */}
      {[
        [-CHASSIS_WIDTH / 2 + 0.35, 0.02, -CHASSIS_DEPTH_Z / 2 + 0.35],
        [CHASSIS_WIDTH / 2 - 0.35, 0.02, -CHASSIS_DEPTH_Z / 2 + 0.35],
        [-CHASSIS_WIDTH / 2 + 0.35, 0.02, CHASSIS_DEPTH_Z / 2 - 0.35],
        [CHASSIS_WIDTH / 2 - 0.35, 0.02, CHASSIS_DEPTH_Z / 2 - 0.35],
      ].map((pos, i) => (
        <mesh key={i} position={pos as [number, number, number]} castShadow>
          <cylinderGeometry args={[0.08, 0.08, 0.06, 12]} />
          <meshStandardMaterial color="#1f2937" metalness={0.9} roughness={0.2} />
        </mesh>
      ))}

      {/* RGB underglow */}
      <pointLight
        position={[0, -CHASSIS_HEIGHT - 0.1, 0]}
        color={layerColor}
        intensity={1.5}
        distance={3}
        decay={2}
      />
      <mesh position={[0, -CHASSIS_HEIGHT - 0.02, 0]} rotation={[-Math.PI / 2, 0, 0]}>
        <planeGeometry args={[CHASSIS_WIDTH - 1, CHASSIS_DEPTH_Z - 1]} />
        <meshBasicMaterial color={layerColor} transparent opacity={0.25} />
      </mesh>
    </group>
  );
}

// =============================================================================
// Background
// =============================================================================

function Background() {
  return (
    <group position={[0, -CHASSIS_HEIGHT - 0.2, 0]}>
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

function Scene({ selectedKey, activeLayer, layers, onKeyClick, onKeyHover }: SuperNovaePad3DProps) {
  const currentLayer = layers[activeLayer] || layers[0];
  const layerColor = currentLayer?.color || '#00FFFF';

  // Build key positions based on layout
  const keys = useMemo(() => {
    const result: { id: string; row: number; col: number; color: string; x: number; z: number }[] = [];
    const spacing = KEY_SIZE + KEY_GAP;

    // Row 0: skip 0,0 (ENC1) and 0,4 (ENC2), keys at 0,1 0,2 0,3
    for (let col = 1; col <= 3; col++) {
      const color = KEY_LAYOUT[0][col];
      if (color) {
        result.push({
          id: `0,${col}`,
          row: 0,
          col,
          color,
          x: (col - 2) * spacing, // Center the 3 keys
          z: -1.5 * spacing,
        });
      }
    }

    // Row 1: 4 keys
    for (let col = 0; col < 4; col++) {
      const color = KEY_LAYOUT[1][col];
      if (color) {
        result.push({
          id: `1,${col}`,
          row: 1,
          col,
          color,
          x: (col - 1.5) * spacing,
          z: -0.5 * spacing,
        });
      }
    }

    // Row 2: 4 keys
    for (let col = 0; col < 4; col++) {
      const color = KEY_LAYOUT[2][col];
      if (color) {
        result.push({
          id: `2,${col}`,
          row: 2,
          col,
          color,
          x: (col - 1.5) * spacing,
          z: 0.5 * spacing,
        });
      }
    }

    // Row 3: 2 center keys (positions 1 and 2)
    for (let col = 1; col <= 2; col++) {
      const color = KEY_LAYOUT[3][col];
      if (color) {
        result.push({
          id: `3,${col}`,
          row: 3,
          col,
          color,
          x: (col - 1.5) * spacing,
          z: 1.5 * spacing,
        });
      }
    }

    return result;
  }, []);

  // Encoder positions
  const spacing = KEY_SIZE + KEY_GAP;
  const enc1Pos: [number, number, number] = [-1.5 * spacing, 0.3, -1.5 * spacing]; // Top-left
  const enc2Pos: [number, number, number] = [1.5 * spacing, 0.35, -1.5 * spacing]; // Top-right
  const enc3Pos: [number, number, number] = [-1.5 * spacing, 0.12, 1.5 * spacing];  // Bottom-left (subtle)
  const optPos: [number, number, number] = [1.5 * spacing, 0.1, 1.5 * spacing];     // Bottom-right (tiny optional)

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

      <Background />
      <Chassis layerColor={layerColor} />

      {/* Keys */}
      {keys.map((key) => (
        <GummyKey
          key={key.id}
          position={[key.x, 0.15, key.z]}
          color={key.color}
          isSelected={selectedKey === key.id}
          onClick={() => onKeyClick(key.id)}
          onPointerEnter={() => onKeyHover?.(key.id)}
          onPointerLeave={() => onKeyHover?.(null)}
        />
      ))}

      {/* Encoders */}
      <SilverEncoder position={enc1Pos} size={1} />
      <BlackKnob position={enc2Pos} />
      <SilverEncoder position={enc3Pos} size={0.5} subtle />
      <TinyButton position={optPos} />

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

export function SuperNovaePad3D(props: SuperNovaePad3DProps) {
  return (
    <div style={{ width: '100%', height: '100%', background: '#0f172a' }}>
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
        <Scene {...props} />
      </Canvas>
    </div>
  );
}

export default SuperNovaePad3D;
