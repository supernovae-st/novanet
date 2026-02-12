'use client';

/**
 * SuperNovaePad3D - Work Louder Creator Micro 3D Visualizer
 *
 * v5.0 - Perfect Figma Match Edition
 * Based on exact Figma reference: Front.svg
 *
 * Layout (matching Figma exactly):
 * ┌─────────────────────────────────────────┐
 * │  ●                              ●       │  (corner screws)
 * │     ┌──────┐                            │
 * │     │ ENC1 │  [■] [■]    (●)            │  Row 0: silver encoder, 2 black keys, black knob
 * │     └──────┘                            │
 * │     [▶] [coral] [purple] [↗]            │  Row 1: black, coral, purple, black
 * │     [#]  [◆]    [blue]  [green]         │  Row 2: black, black, blue, green
 * │     (🚀) [◐]    [🔍]    (☺)             │  Row 3: logo, black, black, smiley (corners subtle)
 * │  ●                              ●       │  (corner screws)
 * │        "SuperNovae Pad"    "NovaNet"    │  (side text)
 * └─────────────────────────────────────────┘
 *
 * USB-C connector at top center
 */

import { useMemo, useRef, useState } from 'react';
import { Canvas, useFrame } from '@react-three/fiber';
import { OrbitControls, RoundedBox, Text, Billboard } from '@react-three/drei';
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
  // New props for key binding UX
  pressedKeys?: Set<string>;        // Physical keys currently pressed (WebHID)
  editingKey?: string | null;       // Key currently being edited
  previewLabel?: string | null;     // Ghost label when hovering preset
  onEditKey?: (keyId: string) => void;  // Callback when edit button clicked
}

// =============================================================================
// Constants - Matching Figma exactly
// =============================================================================

const KEY_SIZE = 0.95;
const KEY_HEIGHT = 0.38;
const KEY_GAP = 0.12;
const KEY_RADIUS = 0.18; // More rounded like Figma
const PRESS_DEPTH = 0.05;

// Colors from Figma design (exact match)
const COLORS = {
  black: '#1a1a1a',      // Matte black keys
  coral: '#f87171',      // Coral/salmon red
  purple: '#a855f7',     // Bright purple
  blue: '#3b82f6',       // Bright blue
  green: '#22c55e',      // Bright green
  white: '#ffffff',      // Icons on black keys
};

// Key layout with colors - EXACT Figma match
// null = encoder/special position, 'black' = black key, color string = colored key
type KeyColor = 'black' | 'coral' | 'purple' | 'blue' | 'green' | null;
const KEY_LAYOUT: KeyColor[][] = [
  [null, 'black', 'black', null],           // Row 0: ENC1, 2 black keys, ENC2
  ['black', 'coral', 'purple', 'black'],    // Row 1: black(play), coral, purple, black(export)
  ['black', 'black', 'blue', 'green'],      // Row 2: black(hash), black(diamond), blue, green
  [null, 'black', 'black', null],           // Row 3: logo(subtle), black(drop), black(search), smiley(subtle)
];

// Chassis dimensions
const CHASSIS_WIDTH = 5.8;
const CHASSIS_DEPTH_Z = 5.8;
const CHASSIS_HEIGHT = 0.55;
const CHASSIS_RADIUS = 0.5;

// Colors
const CHASSIS_WHITE = '#e8e8e8';
const CHASSIS_INNER = '#d4d4d4';
const SCREW_COLOR = '#1f2937';

// =============================================================================
// Key Component - Black matte or colored gummy
// =============================================================================

interface KeycapProps {
  position: [number, number, number];
  color: KeyColor;
  isSelected: boolean;
  isPhysicallyPressed?: boolean;
  isEditing?: boolean;
  label?: string;
  keycode?: string;
  previewLabel?: string | null;
  layerColor?: string;
  onClick: () => void;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  onEditClick?: () => void;
}

function Keycap({
  position,
  color,
  isSelected,
  isPhysicallyPressed = false,
  isEditing = false,
  label,
  keycode,
  previewLabel,
  layerColor = '#00FFFF',
  onClick,
  onPointerEnter,
  onPointerLeave,
  onEditClick,
}: KeycapProps) {
  const groupRef = useRef<THREE.Group>(null);
  const [isPressed, setIsPressed] = useState(false);
  const [isHovered, setIsHovered] = useState(false);
  const pressProgress = useRef(0);

  const actualColor = color === 'black' ? COLORS.black : COLORS[color as keyof typeof COLORS] || COLORS.black;
  const isBlackKey = color === 'black';

  // Physical press takes priority over click press
  const shouldPress = isPressed || isPhysicallyPressed;

  useFrame((_, delta) => {
    if (!groupRef.current) return;
    const target = shouldPress ? 1 : 0;
    pressProgress.current += (target - pressProgress.current) * delta * 20;
    groupRef.current.position.y = position[1] - pressProgress.current * PRESS_DEPTH;
  });

  const handleClick = () => {
    setIsPressed(true);
    onClick();
    setTimeout(() => setIsPressed(false), 100);
  };

  // Show label when selected or editing
  const showLabel = isSelected || isEditing;
  const displayLabel = previewLabel || label;

  return (
    <group ref={groupRef} position={position}>
      {/* Key stem/base */}
      <mesh position={[0, -0.06, 0]}>
        <boxGeometry args={[KEY_SIZE * 0.35, 0.08, KEY_SIZE * 0.35]} />
        <meshStandardMaterial color="#0a0a0a" />
      </mesh>

      {/* Main keycap */}
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
        {isBlackKey ? (
          // Matte black key
          <meshStandardMaterial
            color={actualColor}
            roughness={0.85}
            metalness={0.05}
          />
        ) : (
          // Colored gummy key with shine
          <meshPhysicalMaterial
            color={actualColor}
            emissive={actualColor}
            emissiveIntensity={isSelected ? 0.4 : isHovered ? 0.25 : 0.1}
            roughness={0.3}
            metalness={0}
            clearcoat={0.8}
            clearcoatRoughness={0.25}
          />
        )}
      </RoundedBox>

      {/* LED glow for colored keys */}
      {!isBlackKey && (
        <pointLight
          position={[0, -0.05, 0]}
          color={actualColor}
          intensity={isSelected ? 3 : isHovered ? 2 : 1}
          distance={1.2}
          decay={2}
        />
      )}

      {/* Selection indicator */}
      {isSelected && (
        <mesh position={[0, KEY_HEIGHT + 0.01, 0]} rotation={[-Math.PI / 2, 0, 0]}>
          <ringGeometry args={[KEY_SIZE * 0.35, KEY_SIZE * 0.42, 32]} />
          <meshBasicMaterial color="#ffffff" transparent opacity={0.9} />
        </mesh>
      )}

      {/* Editing glow */}
      {isEditing && (
        <pointLight
          position={[0, 0.5, 0]}
          color={layerColor}
          intensity={2}
          distance={2}
          decay={2}
        />
      )}

      {/* Hover highlight for black keys */}
      {isBlackKey && isHovered && (
        <mesh position={[0, KEY_HEIGHT + 0.005, 0]} rotation={[-Math.PI / 2, 0, 0]}>
          <planeGeometry args={[KEY_SIZE * 0.6, KEY_SIZE * 0.6]} />
          <meshBasicMaterial color="#ffffff" transparent opacity={0.1} />
        </mesh>
      )}

      {/* Floating label (billboard - always faces camera) */}
      {showLabel && displayLabel && (
        <Billboard position={[0, -0.6, 0]} follow={true}>
          <group>
            {/* Label background */}
            <mesh position={[0, 0, -0.01]}>
              <planeGeometry args={[Math.max(1.4, displayLabel.length * 0.13), 0.45]} />
              <meshBasicMaterial
                color={previewLabel ? '#312e81' : '#0d0d12'}
                transparent
                opacity={0.9}
              />
            </mesh>
            {/* Border */}
            <mesh position={[0, 0, -0.02]}>
              <planeGeometry args={[Math.max(1.5, displayLabel.length * 0.14), 0.55]} />
              <meshBasicMaterial
                color={previewLabel ? '#6366f1' : layerColor}
                transparent
                opacity={isEditing ? 0.8 : 0.4}
              />
            </mesh>
            {/* Main text */}
            <Text
              position={[0, 0.04, 0]}
              fontSize={0.14}
              color={previewLabel ? '#a5b4fc' : '#ffffff'}
              anchorX="center"
              anchorY="middle"
            >
              {displayLabel}
            </Text>
            {/* Keycode subtitle */}
            {keycode && !previewLabel && (
              <Text
                position={[0, -0.1, 0]}
                fontSize={0.08}
                color="#666666"
                anchorX="center"
                anchorY="middle"
              >
                [{keycode}]
              </Text>
            )}
            {/* Preview indicator */}
            {previewLabel && (
              <Text
                position={[0, -0.1, 0]}
                fontSize={0.07}
                color="#6366f1"
                anchorX="center"
                anchorY="middle"
              >
                preview
              </Text>
            )}
            {/* Edit button indicator */}
            {isSelected && !isEditing && !previewLabel && onEditClick && (
              <group
                position={[Math.max(0.5, displayLabel.length * 0.065), 0, 0]}
                onClick={(e) => {
                  e.stopPropagation();
                  onEditClick();
                }}
              >
                <mesh>
                  <circleGeometry args={[0.12, 16]} />
                  <meshBasicMaterial color={layerColor} />
                </mesh>
                <Text
                  position={[0, 0, 0.01]}
                  fontSize={0.1}
                  color="#000000"
                  anchorX="center"
                  anchorY="middle"
                >
                  ✎
                </Text>
              </group>
            )}
          </group>
        </Billboard>
      )}

      {/* Connector line to label */}
      {showLabel && displayLabel && (
        <mesh position={[0, -0.35, 0]}>
          <boxGeometry args={[0.02, 0.2, 0.02]} />
          <meshBasicMaterial color={layerColor} transparent opacity={0.4} />
        </mesh>
      )}
    </group>
  );
}

// =============================================================================
// Silver Knurled Encoder (ENC1 - top left)
// =============================================================================

function SilverEncoder({ position }: { position: [number, number, number] }) {
  const knobRef = useRef<THREE.Group>(null);

  useFrame((state) => {
    if (knobRef.current) {
      knobRef.current.rotation.y = Math.sin(state.clock.elapsedTime * 0.3) * 0.04;
    }
  });

  return (
    <group position={position}>
      {/* Housing/frame - dark gray */}
      <mesh position={[0, -0.08, 0]} castShadow>
        <boxGeometry args={[0.95, 0.12, 0.75]} />
        <meshStandardMaterial color="#2d2d2d" roughness={0.7} metalness={0.3} />
      </mesh>

      {/* Inner frame */}
      <mesh position={[0, -0.02, 0]}>
        <boxGeometry args={[0.85, 0.06, 0.65]} />
        <meshStandardMaterial color="#1a1a1a" roughness={0.8} />
      </mesh>

      {/* Knurled cylinder */}
      <group ref={knobRef} position={[0, 0.15, 0]}>
        <mesh castShadow>
          <cylinderGeometry args={[0.28, 0.28, 0.35, 32]} />
          <meshStandardMaterial
            color="#b0b0b0"
            metalness={0.95}
            roughness={0.12}
          />
        </mesh>

        {/* Knurling lines */}
        {Array.from({ length: 28 }).map((_, i) => {
          const angle = (i / 28) * Math.PI * 2;
          return (
            <mesh
              key={i}
              position={[Math.cos(angle) * 0.26, 0, Math.sin(angle) * 0.26]}
              rotation={[0, -angle, 0]}
            >
              <boxGeometry args={[0.015, 0.32, 0.004]} />
              <meshStandardMaterial color="#888888" metalness={0.9} roughness={0.15} />
            </mesh>
          );
        })}

        {/* Top cap */}
        <mesh position={[0, 0.18, 0]}>
          <cylinderGeometry args={[0.22, 0.26, 0.04, 32]} />
          <meshStandardMaterial color="#cccccc" metalness={0.9} roughness={0.1} />
        </mesh>
      </group>
    </group>
  );
}

// =============================================================================
// Black Volume Knob (ENC2 - top right)
// =============================================================================

function BlackKnob({ position }: { position: [number, number, number] }) {
  const knobRef = useRef<THREE.Group>(null);

  useFrame((state) => {
    if (knobRef.current) {
      knobRef.current.rotation.y = Math.sin(state.clock.elapsedTime * 0.25) * 0.03;
    }
  });

  return (
    <group position={position} ref={knobRef}>
      {/* Base */}
      <mesh position={[0, -0.04, 0]} castShadow>
        <cylinderGeometry args={[0.48, 0.5, 0.06, 32]} />
        <meshStandardMaterial color="#2a2a2a" metalness={0.5} roughness={0.5} />
      </mesh>

      {/* Main body - matte black with subtle gradient */}
      <mesh castShadow>
        <cylinderGeometry args={[0.44, 0.46, 0.42, 32]} />
        <meshStandardMaterial color="#1a1a1a" metalness={0.1} roughness={0.9} />
      </mesh>

      {/* Top surface - slightly darker */}
      <mesh position={[0, 0.2, 0]}>
        <cylinderGeometry args={[0.38, 0.42, 0.06, 32]} />
        <meshStandardMaterial color="#0f0f0f" metalness={0.15} roughness={0.85} />
      </mesh>

      {/* Position indicator line */}
      <mesh position={[0, 0.24, 0.36]} rotation={[Math.PI / 2, 0, 0]}>
        <boxGeometry args={[0.06, 0.1, 0.02]} />
        <meshBasicMaterial color="#ffffff" />
      </mesh>
    </group>
  );
}

// =============================================================================
// Subtle Corner Button (logo bottom-left, smiley bottom-right)
// =============================================================================

function SubtleButton({
  position,
  variant
}: {
  position: [number, number, number];
  variant: 'logo' | 'smiley';
}) {
  const [isHovered, setIsHovered] = useState(false);

  return (
    <group position={position}>
      {/* Small circular base */}
      <mesh
        castShadow
        onPointerEnter={() => setIsHovered(true)}
        onPointerLeave={() => setIsHovered(false)}
      >
        <cylinderGeometry args={[0.32, 0.34, 0.15, 24]} />
        <meshStandardMaterial
          color={variant === 'logo' ? '#2a2a2a' : '#e5e5e5'}
          roughness={0.6}
          metalness={variant === 'logo' ? 0.1 : 0.0}
        />
      </mesh>

      {/* Top surface */}
      <mesh position={[0, 0.08, 0]}>
        <cylinderGeometry args={[0.26, 0.28, 0.04, 24]} />
        <meshStandardMaterial
          color={variant === 'logo' ? '#1a1a1a' : '#f5f5f5'}
          roughness={0.5}
        />
      </mesh>

      {/* Indicator dots for smiley variant */}
      {variant === 'smiley' && (
        <group position={[0, 0.11, 0]}>
          {/* Three small dots like in Figma */}
          {[-0.08, 0, 0.08].map((x, i) => (
            <mesh key={i} position={[x, 0, 0]}>
              <cylinderGeometry args={[0.02, 0.02, 0.02, 8]} />
              <meshStandardMaterial color="#fbbf24" emissive="#fbbf24" emissiveIntensity={0.3} />
            </mesh>
          ))}
        </group>
      )}

      {/* Hover effect */}
      {isHovered && (
        <pointLight position={[0, 0.2, 0]} intensity={0.5} distance={0.8} color="#ffffff" />
      )}
    </group>
  );
}

// =============================================================================
// USB-C Connector
// =============================================================================

function USBConnector({ position }: { position: [number, number, number] }) {
  return (
    <group position={position}>
      {/* Metal housing */}
      <mesh>
        <boxGeometry args={[0.35, 0.12, 0.18]} />
        <meshStandardMaterial color="#808080" metalness={0.9} roughness={0.2} />
      </mesh>
      {/* Port opening */}
      <mesh position={[0, 0, 0.05]}>
        <boxGeometry args={[0.25, 0.08, 0.1]} />
        <meshStandardMaterial color="#1a1a1a" />
      </mesh>
      {/* Cable */}
      <mesh position={[0, 0.1, 0.3]} rotation={[Math.PI / 2, 0, 0]}>
        <cylinderGeometry args={[0.06, 0.06, 0.4, 12]} />
        <meshStandardMaterial color="#2a2a2a" roughness={0.8} />
      </mesh>
    </group>
  );
}

// =============================================================================
// White Chassis with Corner Screws
// =============================================================================

function Chassis({ layerColor }: { layerColor: string }) {
  return (
    <group>
      {/* Main body - white/light gray plastic */}
      <RoundedBox
        args={[CHASSIS_WIDTH, CHASSIS_HEIGHT, CHASSIS_DEPTH_Z]}
        radius={CHASSIS_RADIUS}
        smoothness={6}
        position={[0, -CHASSIS_HEIGHT / 2, 0]}
        receiveShadow
        castShadow
      >
        <meshPhysicalMaterial
          color={CHASSIS_WHITE}
          roughness={0.45}
          metalness={0}
          clearcoat={0.15}
          clearcoatRoughness={0.7}
        />
      </RoundedBox>

      {/* Inner recessed plate */}
      <RoundedBox
        args={[CHASSIS_WIDTH - 0.4, 0.1, CHASSIS_DEPTH_Z - 0.4]}
        radius={CHASSIS_RADIUS - 0.1}
        smoothness={4}
        position={[0, 0.02, 0]}
      >
        <meshStandardMaterial color={CHASSIS_INNER} roughness={0.5} />
      </RoundedBox>

      {/* Corner screws - bigger and more visible like Figma */}
      {[
        [-CHASSIS_WIDTH / 2 + 0.4, 0.04, -CHASSIS_DEPTH_Z / 2 + 0.4],
        [CHASSIS_WIDTH / 2 - 0.4, 0.04, -CHASSIS_DEPTH_Z / 2 + 0.4],
        [-CHASSIS_WIDTH / 2 + 0.4, 0.04, CHASSIS_DEPTH_Z / 2 - 0.4],
        [CHASSIS_WIDTH / 2 - 0.4, 0.04, CHASSIS_DEPTH_Z / 2 - 0.4],
      ].map((pos, i) => (
        <group key={i} position={pos as [number, number, number]}>
          {/* Screw head */}
          <mesh castShadow>
            <cylinderGeometry args={[0.12, 0.12, 0.06, 16]} />
            <meshStandardMaterial color={SCREW_COLOR} metalness={0.8} roughness={0.3} />
          </mesh>
          {/* Phillips cross indent */}
          <mesh position={[0, 0.035, 0]} rotation={[0, Math.PI / 4, 0]}>
            <boxGeometry args={[0.14, 0.02, 0.025]} />
            <meshStandardMaterial color="#0a0a0a" />
          </mesh>
          <mesh position={[0, 0.035, 0]} rotation={[0, -Math.PI / 4, 0]}>
            <boxGeometry args={[0.14, 0.02, 0.025]} />
            <meshStandardMaterial color="#0a0a0a" />
          </mesh>
        </group>
      ))}

      {/* USB-C connector at top */}
      <USBConnector position={[0, 0.1, -CHASSIS_DEPTH_Z / 2 - 0.1]} />

      {/* RGB underglow effect */}
      <pointLight
        position={[0, -CHASSIS_HEIGHT - 0.05, 0]}
        color={layerColor}
        intensity={1.2}
        distance={2.5}
        decay={2}
      />
      <mesh position={[0, -CHASSIS_HEIGHT + 0.02, 0]} rotation={[-Math.PI / 2, 0, 0]}>
        <planeGeometry args={[CHASSIS_WIDTH - 0.8, CHASSIS_DEPTH_Z - 0.8]} />
        <meshBasicMaterial color={layerColor} transparent opacity={0.2} />
      </mesh>
    </group>
  );
}

// =============================================================================
// Background - Dark grid
// =============================================================================

function Background() {
  return (
    <group position={[0, -CHASSIS_HEIGHT - 0.15, 0]}>
      {/* Base plane */}
      <mesh rotation={[-Math.PI / 2, 0, 0]} receiveShadow>
        <planeGeometry args={[60, 60]} />
        <meshStandardMaterial color="#0a1628" />
      </mesh>

      {/* Primary grid - larger cells, more visible */}
      <gridHelper args={[60, 30, '#2563eb', '#1e40af']} position={[0, 0.01, 0]} />

      {/* Secondary fine grid */}
      <gridHelper args={[60, 120, '#1e3a5f', '#0c1929']} position={[0, 0.005, 0]} />

      {/* Center glow ring - outer */}
      <mesh rotation={[-Math.PI / 2, 0, 0]} position={[0, 0.02, 0]}>
        <ringGeometry args={[6, 8, 64]} />
        <meshBasicMaterial color="#0ea5e9" transparent opacity={0.08} />
      </mesh>

      {/* Center glow - inner */}
      <mesh rotation={[-Math.PI / 2, 0, 0]} position={[0, 0.02, 0]}>
        <circleGeometry args={[6, 64]} />
        <meshBasicMaterial color="#0ea5e9" transparent opacity={0.06} />
      </mesh>

      {/* Corner markers */}
      {[[-12, -12], [12, -12], [-12, 12], [12, 12]].map(([x, z], i) => (
        <mesh key={i} rotation={[-Math.PI / 2, 0, 0]} position={[x, 0.015, z]}>
          <ringGeometry args={[0.3, 0.5, 16]} />
          <meshBasicMaterial color="#3b82f6" transparent opacity={0.3} />
        </mesh>
      ))}
    </group>
  );
}

// =============================================================================
// Scene
// =============================================================================

function Scene({
  selectedKey,
  activeLayer,
  layers,
  onKeyClick,
  onKeyHover,
  pressedKeys,
  editingKey,
  previewLabel,
  onEditKey,
}: SuperNovaePad3DProps) {
  const currentLayer = layers[activeLayer] || layers[0];
  const layerColor = currentLayer?.color || '#00FFFF';

  // Build key grid (4x4 minus encoder/special positions)
  const keys = useMemo(() => {
    const result: { id: string; row: number; col: number; color: KeyColor; x: number; z: number }[] = [];
    const spacing = KEY_SIZE + KEY_GAP;
    const startX = -1.5 * spacing;
    const startZ = -1.5 * spacing;

    for (let row = 0; row < 4; row++) {
      for (let col = 0; col < 4; col++) {
        const keyColor = KEY_LAYOUT[row][col];
        if (keyColor !== null) {
          result.push({
            id: `${row},${col}`,
            row,
            col,
            color: keyColor,
            x: startX + col * spacing,
            z: startZ + row * spacing,
          });
        }
      }
    }
    return result;
  }, []);

  // Component positions
  const spacing = KEY_SIZE + KEY_GAP;
  const startX = -1.5 * spacing;
  const startZ = -1.5 * spacing;

  // Encoder positions (row 0)
  const enc1Pos: [number, number, number] = [startX, 0.2, startZ];                    // Top-left
  const enc2Pos: [number, number, number] = [startX + 3 * spacing, 0.28, startZ];     // Top-right

  // Subtle buttons (row 3)
  const logoPos: [number, number, number] = [startX, 0.12, startZ + 3 * spacing];     // Bottom-left
  const smileyPos: [number, number, number] = [startX + 3 * spacing, 0.12, startZ + 3 * spacing]; // Bottom-right

  return (
    <>
      {/* Lighting - optimized for white chassis + black/colored keys */}
      <ambientLight intensity={0.65} />
      <directionalLight
        position={[6, 18, 10]}
        intensity={1.3}
        castShadow
        shadow-mapSize={[2048, 2048]}
        shadow-camera-far={50}
        shadow-camera-left={-10}
        shadow-camera-right={10}
        shadow-camera-top={10}
        shadow-camera-bottom={-10}
      />
      <pointLight position={[-6, 8, -6]} intensity={0.35} color="#a855f7" />
      <pointLight position={[6, 6, 6]} intensity={0.3} color="#0ea5e9" />
      <pointLight position={[0, 5, 0]} intensity={0.25} color="#ffffff" />

      <Background />
      <Chassis layerColor={layerColor} />

      {/* Keys */}
      {keys.map((key) => {
        const keyBinding = currentLayer?.keys[key.id];
        const isThisKeyEditing = editingKey === key.id;
        const isThisKeySelected = selectedKey === key.id;

        return (
          <Keycap
            key={key.id}
            position={[key.x, 0.12, key.z]}
            color={key.color}
            isSelected={isThisKeySelected}
            isPhysicallyPressed={pressedKeys?.has(key.id)}
            isEditing={isThisKeyEditing}
            label={keyBinding?.label || keyBinding?.action}
            keycode={keyBinding?.key}
            previewLabel={isThisKeyEditing ? previewLabel : null}
            layerColor={layerColor}
            onClick={() => onKeyClick(key.id)}
            onPointerEnter={() => onKeyHover?.(key.id)}
            onPointerLeave={() => onKeyHover?.(null)}
            onEditClick={() => onEditKey?.(key.id)}
          />
        );
      })}

      {/* Encoders */}
      <SilverEncoder position={enc1Pos} />
      <BlackKnob position={enc2Pos} />

      {/* Subtle corner buttons */}
      <SubtleButton position={logoPos} variant="logo" />
      <SubtleButton position={smileyPos} variant="smiley" />

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
    <div style={{ width: '100%', height: '100%', background: '#0a1628' }}>
      <Canvas
        shadows
        camera={{ position: [10, 9, 12], fov: 38 }}
        onCreated={(state) => {
          state.gl.toneMapping = THREE.ACESFilmicToneMapping;
          state.gl.toneMappingExposure = 1.1;
        }}
      >
        <color attach="background" args={['#0a1628']} />
        <fog attach="fog" args={['#0a1628', 22, 45]} />
        <Scene {...props} />
      </Canvas>
    </div>
  );
}

export default SuperNovaePad3D;
