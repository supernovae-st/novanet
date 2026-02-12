'use client';

/**
 * KeyLabel3D - Billboard text label floating below a key
 *
 * Features:
 * - Always faces camera (billboard mode)
 * - Shows action label + keycode
 * - Edit button on hover
 * - Ghost preview when hovering presets
 */

import { useRef, useState } from 'react';
import { useFrame } from '@react-three/fiber';
import { Text, RoundedBox } from '@react-three/drei';
import * as THREE from 'three';

// =============================================================================
// Types
// =============================================================================

interface KeyLabel3DProps {
  position: [number, number, number];
  label: string;
  keycode?: string;
  color?: string;
  isEditing?: boolean;
  previewLabel?: string | null;
  onEditClick?: () => void;
}

// =============================================================================
// Component
// =============================================================================

export function KeyLabel3D({
  position,
  label,
  keycode,
  color = '#00FFFF',
  isEditing = false,
  previewLabel,
  onEditClick,
}: KeyLabel3DProps) {
  const groupRef = useRef<THREE.Group>(null);
  const [isHovered, setIsHovered] = useState(false);

  // Billboard effect - always face camera
  useFrame(({ camera }) => {
    if (groupRef.current) {
      groupRef.current.quaternion.copy(camera.quaternion);
    }
  });

  // Display label (preview takes priority)
  const displayLabel = previewLabel || label;
  const isPreview = !!previewLabel;

  // Calculate background width based on text length
  const bgWidth = Math.max(1.2, displayLabel.length * 0.12 + 0.4);

  return (
    <group ref={groupRef} position={position}>
      {/* Background panel */}
      <RoundedBox
        args={[bgWidth, 0.5, 0.05]}
        radius={0.08}
        smoothness={4}
        position={[0, 0, -0.03]}
        onPointerEnter={() => setIsHovered(true)}
        onPointerLeave={() => setIsHovered(false)}
        onClick={(e) => {
          e.stopPropagation();
          onEditClick?.();
        }}
      >
        <meshStandardMaterial
          color={isPreview ? '#1a1a2e' : '#0d0d12'}
          transparent
          opacity={isPreview ? 0.7 : 0.9}
        />
      </RoundedBox>

      {/* Border glow */}
      <mesh position={[0, 0, -0.035]}>
        <planeGeometry args={[bgWidth + 0.06, 0.56]} />
        <meshBasicMaterial
          color={isPreview ? '#6366f1' : color}
          transparent
          opacity={isEditing ? 0.6 : isHovered ? 0.4 : 0.2}
        />
      </mesh>

      {/* Main label */}
      <Text
        position={[0, 0.06, 0]}
        fontSize={0.16}
        color={isPreview ? '#a5b4fc' : '#ffffff'}
        anchorX="center"
        anchorY="middle"
        font="/fonts/inter-medium.woff"
      >
        {displayLabel}
      </Text>

      {/* Keycode subtitle */}
      {keycode && !isPreview && (
        <Text
          position={[0, -0.12, 0]}
          fontSize={0.09}
          color="#666666"
          anchorX="center"
          anchorY="middle"
          font="/fonts/jetbrains-mono.woff"
        >
          [{keycode}]
        </Text>
      )}

      {/* Edit indicator */}
      {(isHovered || isEditing) && !isPreview && (
        <group position={[bgWidth / 2 - 0.15, 0, 0]}>
          {/* Edit dot */}
          <mesh>
            <circleGeometry args={[0.06, 16]} />
            <meshBasicMaterial color={color} />
          </mesh>
          {/* Pencil icon placeholder */}
          <Text
            position={[0, 0, 0.01]}
            fontSize={0.07}
            color="#000000"
            anchorX="center"
            anchorY="middle"
          >
            ✎
          </Text>
        </group>
      )}

      {/* Preview indicator */}
      {isPreview && (
        <Text
          position={[0, -0.12, 0]}
          fontSize={0.08}
          color="#6366f1"
          anchorX="center"
          anchorY="middle"
        >
          preview
        </Text>
      )}

      {/* Connector line to key */}
      <mesh position={[0, 0.35, -0.04]}>
        <planeGeometry args={[0.02, 0.2]} />
        <meshBasicMaterial color={color} transparent opacity={0.4} />
      </mesh>
    </group>
  );
}

export default KeyLabel3D;
