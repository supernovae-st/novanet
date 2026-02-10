'use client';

/**
 * SuperNovaePad3D - Work Louder Creator Micro 3D Visualizer
 *
 * v3.0 - Clean White Gummy Edition
 * - Frosted white polycarbonate chassis (like real device)
 * - Gummy bear translucent keycaps with soft glow
 * - Clean minimalist aesthetic
 * - Accurate proportions and layout
 *
 * Layout (top view):
 * ┌─────────────────────────────────────────────┐
 * │  ◎           [USB-C]              ●         │
 * │     [R] [R] [R] [C]                         │
 * │     [P] [P] [C] [C]                         │
 * │     [M] [M] [M] [M]                         │
 * └─────────────────────────────────────────────┘
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
// Constants - Clean White Theme (Square Layout)
// =============================================================================

const KEY_SIZE = 1.0;
const KEY_HEIGHT = 0.55;
const KEY_GAP = 0.25;
const KEY_RADIUS = 0.12;
const PRESS_DEPTH = 0.08;
const PRESS_DURATION = 0.1;

// Gummy bear colors - matching screen 3 exactly
const KEY_COLORS: string[][] = [
  ['#f97068', '#f97068', '#8b5cf6', '#06b6d4'], // Row 0: salmon coral, salmon, purple, cyan
  ['#8b5cf6', '#8b5cf6', '#8b5cf6', '#5eead4'], // Row 1: purple x3, mint/teal
  ['#5eead4', '#5eead4', '#5eead4', '#5eead4'], // Row 2: mint all
];

// White/frosted chassis palette
const CHASSIS_WHITE = '#f8fafc';
const CHASSIS_FROST = '#e2e8f0';
const CHASSIS_EDGE = '#cbd5e1';
const CHASSIS_ACCENT = '#94a3b8';
const PCB_COLOR = '#1e293b';

// Square chassis dimensions
const CHASSIS_SIZE = 8.0;
const CHASSIS_DEPTH = 0.9;

// =============================================================================
// Gummy Bear Keycap with Press Animation
// =============================================================================

function GummyKeycap({
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
  const targetPress = useRef(0);

  // Press animation - smooth spring
  useFrame((_, delta) => {
    if (!groupRef.current) return;

    targetPress.current = isPressed ? 1 : 0;
    const speed = isPressed ? 20 : 14;
    pressProgress.current += (targetPress.current - pressProgress.current) * delta * speed;

    const pressOffset = pressProgress.current * PRESS_DEPTH;
    groupRef.current.position.y = position[1] - pressOffset;

    // Subtle squish effect
    const scaleXZ = 1 + pressProgress.current * 0.02;
    const scaleY = 1 - pressProgress.current * 0.05;
    groupRef.current.scale.set(scaleXZ, scaleY, scaleXZ);
  });

  const handleClick = () => {
    setIsPressed(true);
    onClick();
    setTimeout(() => setIsPressed(false), PRESS_DURATION * 1000);
  };

  return (
    <group ref={groupRef} position={position}>
      {/* Soft glow beneath - LED effect */}
      <pointLight
        position={[0, -0.15, 0]}
        color={color}
        intensity={isSelected ? 3 : isHovered ? 2 : 1.2}
        distance={1.5}
        decay={2}
      />

      {/* Key stem (dark, visible through translucent cap) */}
      <RoundedBox
        args={[KEY_SIZE * 0.5, 0.25, KEY_SIZE * 0.5]}
        radius={0.05}
        smoothness={4}
        position={[0, -0.12, 0]}
      >
        <meshStandardMaterial color={PCB_COLOR} roughness={0.9} />
      </RoundedBox>

      {/* Main gummy keycap - glossy translucent like real rubber/silicone */}
      <RoundedBox
        args={[KEY_SIZE, KEY_HEIGHT, KEY_SIZE]}
        radius={KEY_RADIUS}
        smoothness={6}
        position={[0, KEY_HEIGHT * 0.35, 0]}
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
          emissiveIntensity={isSelected ? 0.35 : isHovered ? 0.2 : 0.1}
          roughness={0.15}
          metalness={0}
          clearcoat={1}
          clearcoatRoughness={0.08}
          transmission={0.4}
          thickness={1.5}
          transparent
          opacity={0.95}
          ior={1.45}
          sheen={0.3}
          sheenRoughness={0.2}
          sheenColor={color}
        />
      </RoundedBox>

      {/* Top surface highlight - glossy */}
      <mesh
        position={[0, KEY_HEIGHT * 0.65, 0]}
        rotation={[-Math.PI / 2, 0, 0]}
      >
        <circleGeometry args={[KEY_SIZE * 0.35, 32]} />
        <meshBasicMaterial
          color="#ffffff"
          transparent
          opacity={isSelected ? 0.35 : isHovered ? 0.25 : 0.12}
        />
      </mesh>

      {/* Selection indicator - subtle ring */}
      {isSelected && (
        <mesh position={[0, KEY_HEIGHT * 0.7, 0]} rotation={[-Math.PI / 2, 0, 0]}>
          <ringGeometry args={[KEY_SIZE * 0.42, KEY_SIZE * 0.48, 32]} />
          <meshBasicMaterial color="#ffffff" transparent opacity={0.8} />
        </mesh>
      )}
    </group>
  );
}

// =============================================================================
// Encoder (small knurled knob - top left)
// =============================================================================

function Encoder({ position }: { position: [number, number, number] }) {
  const knobRef = useRef<THREE.Mesh>(null);

  useFrame((state) => {
    if (knobRef.current) {
      knobRef.current.rotation.y = Math.sin(state.clock.elapsedTime * 0.5) * 0.08;
    }
  });

  return (
    <group position={position}>
      {/* Base ring - silver */}
      <mesh position={[0, -0.08, 0]} castShadow>
        <cylinderGeometry args={[0.45, 0.48, 0.12, 32]} />
        <meshStandardMaterial color="#9ca3af" metalness={0.8} roughness={0.25} />
      </mesh>

      {/* Knurled knob - dark */}
      <mesh ref={knobRef} castShadow>
        <cylinderGeometry args={[0.38, 0.38, 0.4, 32]} />
        <meshStandardMaterial color="#374151" metalness={0.7} roughness={0.35} />
      </mesh>

      {/* Knurling ridges */}
      {Array.from({ length: 24 }).map((_, i) => {
        const angle = (i / 24) * Math.PI * 2;
        return (
          <mesh
            key={i}
            position={[Math.cos(angle) * 0.36, 0, Math.sin(angle) * 0.36]}
            rotation={[0, -angle, 0]}
          >
            <boxGeometry args={[0.025, 0.35, 0.012]} />
            <meshStandardMaterial color="#1f2937" metalness={0.8} />
          </mesh>
        );
      })}

      {/* Top indicator dot */}
      <mesh position={[0, 0.21, 0.3]} rotation={[Math.PI / 2, 0, 0]}>
        <circleGeometry args={[0.04, 16]} />
        <meshBasicMaterial color="#ffffff" />
      </mesh>
    </group>
  );
}

// =============================================================================
// Big Volume Knob (top right)
// =============================================================================

function BigKnob({ position }: { position: [number, number, number] }) {
  const knobRef = useRef<THREE.Group>(null);

  useFrame((state) => {
    if (knobRef.current) {
      knobRef.current.rotation.y = Math.sin(state.clock.elapsedTime * 0.3) * 0.04;
    }
  });

  return (
    <group position={position} ref={knobRef}>
      {/* Base ring - silver */}
      <mesh position={[0, -0.12, 0]} castShadow>
        <cylinderGeometry args={[0.72, 0.75, 0.1, 32]} />
        <meshStandardMaterial color="#9ca3af" metalness={0.85} roughness={0.2} />
      </mesh>

      {/* Main knob body - matte black */}
      <mesh castShadow>
        <cylinderGeometry args={[0.62, 0.65, 0.55, 32]} />
        <meshStandardMaterial color="#1f2937" metalness={0.2} roughness={0.8} />
      </mesh>

      {/* Top cap - slightly recessed */}
      <mesh position={[0, 0.22, 0]}>
        <cylinderGeometry args={[0.52, 0.55, 0.1, 32]} />
        <meshStandardMaterial color="#111827" metalness={0.3} roughness={0.7} />
      </mesh>

      {/* Grip ridges - more subtle */}
      {Array.from({ length: 16 }).map((_, i) => {
        const angle = (i / 16) * Math.PI * 2;
        return (
          <mesh
            key={i}
            position={[Math.cos(angle) * 0.6, 0, Math.sin(angle) * 0.6]}
            rotation={[0, -angle, 0]}
          >
            <boxGeometry args={[0.04, 0.45, 0.03]} />
            <meshStandardMaterial color="#374151" metalness={0.4} roughness={0.7} />
          </mesh>
        );
      })}

      {/* Position indicator line */}
      <mesh position={[0, 0.28, 0.45]} rotation={[Math.PI / 2, 0, 0]}>
        <boxGeometry args={[0.06, 0.1, 0.015]} />
        <meshBasicMaterial color="#ffffff" />
      </mesh>
    </group>
  );
}

// =============================================================================
// USB-C Cable (cleaner, lighter theme)
// =============================================================================

function USBCable({ startPosition }: { startPosition: [number, number, number] }) {
  const groupRef = useRef<THREE.Group>(null);

  useFrame((state) => {
    if (groupRef.current) {
      groupRef.current.rotation.x = Math.sin(state.clock.elapsedTime * 0.5) * 0.01;
    }
  });

  const curve = useMemo(() => {
    const [x, y, z] = startPosition;
    return new THREE.CatmullRomCurve3([
      new THREE.Vector3(x, y, z),
      new THREE.Vector3(x, y + 0.2, z - 0.4),
      new THREE.Vector3(x, y + 0.1, z - 1.2),
      new THREE.Vector3(x - 0.1, y - 0.2, z - 2.5),
      new THREE.Vector3(x, y - 0.6, z - 4),
    ]);
  }, [startPosition]);

  const tubeGeometry = useMemo(() => {
    return new THREE.TubeGeometry(curve, 32, 0.06, 12, false);
  }, [curve]);

  return (
    <group ref={groupRef}>
      {/* USB-C connector */}
      <group position={[startPosition[0], startPosition[1] + 0.03, startPosition[2] - 0.12]}>
        <mesh>
          <boxGeometry args={[0.35, 0.16, 0.28]} />
          <meshStandardMaterial color="#4b5563" metalness={0.5} roughness={0.4} />
        </mesh>
        <mesh position={[0, 0, 0.12]}>
          <boxGeometry args={[0.28, 0.1, 0.05]} />
          <meshStandardMaterial color="#9ca3af" metalness={0.9} roughness={0.15} />
        </mesh>
      </group>

      {/* Cable - white braided */}
      <mesh geometry={tubeGeometry}>
        <meshStandardMaterial color="#e5e7eb" roughness={0.75} metalness={0} />
      </mesh>

      {/* Strain relief */}
      <mesh
        position={[startPosition[0], startPosition[1] + 0.1, startPosition[2] - 0.32]}
        rotation={[Math.PI / 2, 0, 0]}
      >
        <cylinderGeometry args={[0.08, 0.065, 0.18, 16]} />
        <meshStandardMaterial color="#6b7280" roughness={0.6} />
      </mesh>
    </group>
  );
}

// =============================================================================
// Frosted White Polycarbonate Chassis
// =============================================================================

function Chassis({ layerColor }: { layerColor: string }) {
  const s = CHASSIS_SIZE;
  const r = 0.6; // Rounded corners

  return (
    <group>
      {/* Main body - frosted white polycarbonate */}
      <RoundedBox
        args={[s, CHASSIS_DEPTH, s]}
        radius={r}
        smoothness={4}
        position={[0, -0.45, 0]}
        receiveShadow
        castShadow
      >
        <meshPhysicalMaterial
          color={CHASSIS_WHITE}
          roughness={0.35}
          metalness={0}
          clearcoat={0.3}
          clearcoatRoughness={0.5}
        />
      </RoundedBox>

      {/* Edge bevel - slightly darker */}
      <RoundedBox
        args={[s - 0.15, CHASSIS_DEPTH + 0.02, s - 0.15]}
        radius={r - 0.05}
        smoothness={4}
        position={[0, -0.44, 0]}
      >
        <meshStandardMaterial color={CHASSIS_FROST} roughness={0.4} metalness={0} />
      </RoundedBox>

      {/* Top plate recess - darker for contrast */}
      <RoundedBox
        args={[s - 0.6, 0.12, s - 0.6]}
        radius={r - 0.25}
        smoothness={4}
        position={[0, 0.02, 0]}
        receiveShadow
      >
        <meshStandardMaterial color={PCB_COLOR} roughness={0.8} metalness={0.1} />
      </RoundedBox>

      {/* Subtle RGB underglow (layer color) */}
      <pointLight
        position={[0, -0.9, 0]}
        color={layerColor}
        intensity={1.5}
        distance={4}
        decay={2}
      />
      <mesh position={[0, -0.96, 0]} rotation={[-Math.PI / 2, 0, 0]}>
        <planeGeometry args={[s - 1.5, s - 1.5]} />
        <meshBasicMaterial color={layerColor} transparent opacity={0.25} />
      </mesh>

      {/* Corner screws - subtle */}
      {[
        [-s/2 + 0.5, 0.1, -s/2 + 0.5],
        [s/2 - 0.5, 0.1, -s/2 + 0.5],
        [-s/2 + 0.5, 0.1, s/2 - 0.5],
        [s/2 - 0.5, 0.1, s/2 - 0.5],
      ].map((pos, i) => (
        <group key={i} position={pos as [number, number, number]}>
          <mesh castShadow>
            <cylinderGeometry args={[0.12, 0.12, 0.06, 16]} />
            <meshStandardMaterial color="#9ca3af" metalness={0.85} roughness={0.2} />
          </mesh>
          <mesh position={[0, 0.035, 0]}>
            <cylinderGeometry args={[0.05, 0.05, 0.015, 6]} />
            <meshStandardMaterial color="#6b7280" metalness={0.9} />
          </mesh>
        </group>
      ))}

      {/* USB-C port at TOP CENTER */}
      <group position={[0, 0.05, -s/2 + 0.1]}>
        <mesh>
          <boxGeometry args={[0.9, 0.3, 0.2]} />
          <meshStandardMaterial color={PCB_COLOR} roughness={0.9} />
        </mesh>
        <mesh position={[0, 0, 0.08]}>
          <boxGeometry args={[0.7, 0.2, 0.08]} />
          <meshStandardMaterial color="#6b7280" metalness={0.85} roughness={0.2} />
        </mesh>
        <mesh position={[0, 0, 0.1]}>
          <boxGeometry args={[0.5, 0.12, 0.04]} />
          <meshStandardMaterial color="#1f2937" />
        </mesh>
      </group>

      {/* Bottom decorations - Figma-style icon (left) */}
      <group position={[-s/2 + 0.7, 0.08, s/2 - 0.7]}>
        {/* Simple geometric shape suggesting Figma logo */}
        <mesh rotation={[-Math.PI / 2, 0, 0]}>
          <circleGeometry args={[0.12, 4]} />
          <meshStandardMaterial color="#94a3b8" roughness={0.5} />
        </mesh>
      </group>

      {/* Bottom decorations - smiley icon (right) */}
      <group position={[s/2 - 0.7, 0.08, s/2 - 0.7]}>
        {/* Simple circle for smiley */}
        <mesh rotation={[-Math.PI / 2, 0, 0]}>
          <ringGeometry args={[0.08, 0.12, 32]} />
          <meshStandardMaterial color="#94a3b8" roughness={0.5} />
        </mesh>
      </group>

      {/* Side branding text area - left side (vertical) */}
      <group position={[-s/2 + 0.12, 0, 0]} rotation={[0, Math.PI / 2, 0]}>
        {/* Small dots representing text */}
        {Array.from({ length: 8 }).map((_, i) => (
          <mesh key={i} position={[0, 0.05, -1.2 + i * 0.35]} rotation={[-Math.PI / 2, 0, 0]}>
            <circleGeometry args={[0.03, 8]} />
            <meshStandardMaterial color="#cbd5e1" roughness={0.6} />
          </mesh>
        ))}
      </group>

      {/* Side branding text area - right side (vertical) */}
      <group position={[s/2 - 0.12, 0, 0]} rotation={[0, -Math.PI / 2, 0]}>
        {/* Small dots representing text */}
        {Array.from({ length: 6 }).map((_, i) => (
          <mesh key={i} position={[0, 0.05, -0.9 + i * 0.35]} rotation={[-Math.PI / 2, 0, 0]}>
            <circleGeometry args={[0.03, 8]} />
            <meshStandardMaterial color="#cbd5e1" roughness={0.6} />
          </mesh>
        ))}
      </group>
    </group>
  );
}

// =============================================================================
// Clean Background
// =============================================================================

function Background() {
  return (
    <group position={[0, -1.2, 0]}>
      {/* Floor - soft gray */}
      <mesh rotation={[-Math.PI / 2, 0, 0]} receiveShadow>
        <planeGeometry args={[60, 60]} />
        <meshStandardMaterial color="#f1f5f9" />
      </mesh>

      {/* Subtle grid */}
      <gridHelper args={[60, 60, '#e2e8f0', '#e2e8f0']} position={[0, 0.01, 0]} />

      {/* Soft shadow circle */}
      <mesh rotation={[-Math.PI / 2, 0, 0]} position={[0, 0.005, 0]}>
        <circleGeometry args={[5, 64]} />
        <meshBasicMaterial color="#94a3b8" transparent opacity={0.08} />
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
  const s = CHASSIS_SIZE;

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

  // Key positions - 4×3 grid centered with room for knobs at top
  const getKeyPosition = (row: number, col: number): [number, number, number] => {
    const spacing = KEY_SIZE + KEY_GAP;
    return [
      (col - 1.5) * spacing,
      KEY_HEIGHT / 2 + 0.12,
      0.5 + (row - 1) * spacing,
    ];
  };

  // Encoder position: TOP-LEFT
  const encoderPos: [number, number, number] = [-s/2 + 1.0, 0.3, -s/2 + 1.0];

  // Big knob position: TOP-RIGHT
  const knobPos: [number, number, number] = [s/2 - 1.0, 0.35, -s/2 + 1.0];

  return (
    <>
      {/* Clean, bright lighting for white theme */}
      <ambientLight intensity={0.6} />
      <directionalLight
        position={[6, 15, 8]}
        intensity={1.2}
        castShadow
        shadow-mapSize={[2048, 2048]}
        shadow-camera-far={50}
        shadow-camera-left={-10}
        shadow-camera-right={10}
        shadow-camera-top={10}
        shadow-camera-bottom={-10}
      />
      {/* Soft fill lights */}
      <pointLight position={[-6, 6, -4]} intensity={0.3} color="#f8fafc" />
      <pointLight position={[6, 5, 6]} intensity={0.25} color="#f8fafc" />

      {/* Clean background */}
      <Background />

      {/* USB Cable */}
      <USBCable startPosition={[0, 0.05, -s/2]} />

      {/* Chassis */}
      <Chassis layerColor={layerColor} />

      {/* 4×3 Gummy Keys */}
      {keys.map((key) => (
        <GummyKeycap
          key={key.id}
          position={getKeyPosition(key.row, key.col)}
          color={key.color}
          isSelected={selectedKey === key.id}
          onClick={() => onKeyClick(key.id)}
          onPointerEnter={() => onKeyHover?.(key.id)}
          onPointerLeave={() => onKeyHover?.(null)}
        />
      ))}

      {/* Encoder (TOP-LEFT) */}
      <Encoder position={encoderPos} />

      {/* Big knob (TOP-RIGHT) */}
      <BigKnob position={knobPos} />

      {/* Camera controls */}
      <OrbitControls
        enablePan={false}
        minDistance={7}
        maxDistance={20}
        minPolarAngle={0.3}
        maxPolarAngle={1.35}
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
    <div style={{ width: '100%', height: '100%', background: '#f8fafc' }}>
      <Canvas
        shadows
        camera={{ position: [0, 8, 11], fov: 40 }}
        onCreated={(state) => {
          state.gl.toneMapping = THREE.ACESFilmicToneMapping;
          state.gl.toneMappingExposure = 1.1;
        }}
      >
        <color attach="background" args={['#f8fafc']} />
        <fog attach="fog" args={['#f8fafc', 25, 50]} />
        <Scene {...props} />
      </Canvas>
    </div>
  );
}

export default SuperNovaePad3D;
