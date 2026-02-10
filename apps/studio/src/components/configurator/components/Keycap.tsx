'use client';

/**
 * Keycap - Black matte keycap matching Work Louder Micro design
 *
 * Features:
 * - Black matte material (matching Figma design)
 * - Colored keys for specific positions (red, purple, cyan, green)
 * - Hover: scale 1.05
 * - Press: scale 0.95, Y offset, elastic bounce
 * - White icons on black keys, dark icons on colored keys
 */

import { useState, useRef, useCallback } from 'react';
import { useFrame, ThreeEvent } from '@react-three/fiber';
import { RoundedBox, Text } from '@react-three/drei';
import type { Group } from 'three';
import type { KeycapProps } from '../types';
import { getVisualEncoding } from '../utils/visualEncoding';

// Keycap dimensions (matching reference images ~11mm per key)
const KEYCAP_SIZE = 0.75;
const KEYCAP_HEIGHT = 0.35;
const KEYCAP_RADIUS = 0.12;
const KEYCAP_GAP = 0.95;

// Animation constants - spring physics
const SPRING_STIFFNESS = 400;
const SPRING_DAMPING = 25;
const HOVER_SCALE = 1.08;
const PRESS_SCALE = 0.92;
const PRESS_Y_OFFSET = -0.06;

// Default black color for most keys
const DEFAULT_KEY_COLOR = '#1a1a1a';

// Check if a key should be colored based on layer
function isColoredKey(layer: string): boolean {
  // Only these layers get their actual color
  return ['config', 'locale', 'knowledge', 'geography'].includes(layer);
}

export function Keycap({ position, binding, onPress, index }: KeycapProps) {
  const [hovered, setHovered] = useState(false);
  const [pressed, setPressed] = useState(false);
  const groupRef = useRef<Group>(null);

  // Animation state with spring physics
  const scaleRef = useRef(1);
  const yOffsetRef = useRef(0);
  const velocityRef = useRef({ scale: 0, y: 0 });

  // Get visual encoding for this layer
  const { color: layerColor, icon } = getVisualEncoding(binding.layer);

  // Determine if this key should be colored or black
  const shouldBeColored = isColoredKey(binding.layer);
  const keyColor = shouldBeColored ? layerColor : DEFAULT_KEY_COLOR;
  const iconColor = shouldBeColored ? '#1a1a1a' : '#ffffff';  // Dark icons on colored, white on black

  // Calculate grid position (3x3 grid)
  const row = Math.floor(index / 3);
  const col = index % 3;
  const x = (col - 1) * KEYCAP_GAP;
  const z = (row - 1) * KEYCAP_GAP;

  // Spring animation
  useFrame((_, delta) => {
    const targetScale = pressed ? PRESS_SCALE : hovered ? HOVER_SCALE : 1;
    const targetY = pressed ? PRESS_Y_OFFSET : 0;

    // Spring physics for scale
    const scaleForce = (targetScale - scaleRef.current) * SPRING_STIFFNESS;
    const scaleDamping = -SPRING_DAMPING * velocityRef.current.scale;
    velocityRef.current.scale += (scaleForce + scaleDamping) * delta;
    scaleRef.current += velocityRef.current.scale * delta;

    // Spring physics for Y offset
    const yForce = (targetY - yOffsetRef.current) * SPRING_STIFFNESS;
    const yDamping = -SPRING_DAMPING * velocityRef.current.y;
    velocityRef.current.y += (yForce + yDamping) * delta;
    yOffsetRef.current += velocityRef.current.y * delta;

    // Apply transforms
    if (groupRef.current) {
      groupRef.current.scale.setScalar(scaleRef.current);
      groupRef.current.position.y = position[1] + yOffsetRef.current;
    }
  });

  const handleClick = useCallback((e: ThreeEvent<MouseEvent>) => {
    e.stopPropagation();
    setPressed(true);
    onPress();
    setTimeout(() => setPressed(false), 150);
  }, [onPress]);

  const handlePointerOver = useCallback((e: ThreeEvent<PointerEvent>) => {
    e.stopPropagation();
    setHovered(true);
    document.body.style.cursor = 'pointer';
  }, []);

  const handlePointerOut = useCallback(() => {
    setHovered(false);
    document.body.style.cursor = 'default';
  }, []);

  return (
    <group position={[position[0] + x, position[1], position[2] + z]}>
      {/* Main keycap group */}
      <group ref={groupRef}>
        {/* Keycap body - black matte or colored (matching Figma) */}
        <RoundedBox
          args={[KEYCAP_SIZE, KEYCAP_HEIGHT, KEYCAP_SIZE]}
          radius={KEYCAP_RADIUS}
          smoothness={4}
          onClick={handleClick}
          onPointerOver={handlePointerOver}
          onPointerOut={handlePointerOut}
          castShadow
          receiveShadow
        >
          <meshStandardMaterial
            color={keyColor}
            metalness={0.1}
            roughness={0.8}
            envMapIntensity={0.3}
          />
        </RoundedBox>

        {/* Subtle highlight on hover */}
        {hovered && (
          <RoundedBox
            args={[KEYCAP_SIZE + 0.02, KEYCAP_HEIGHT + 0.01, KEYCAP_SIZE + 0.02]}
            radius={KEYCAP_RADIUS}
            smoothness={4}
          >
            <meshBasicMaterial
              color={shouldBeColored ? keyColor : '#4b5563'}
              transparent
              opacity={0.3}
            />
          </RoundedBox>
        )}

        {/* Icon on top of keycap */}
        <Text
          position={[0, KEYCAP_HEIGHT / 2 + 0.01, 0]}
          rotation={[-Math.PI / 2, 0, 0]}
          fontSize={0.22}
          color={iconColor}
          anchorX="center"
          anchorY="middle"
          outlineWidth={0.01}
          outlineColor={shouldBeColored ? '#ffffff' : '#000000'}
          font="/fonts/inter-bold.woff"
        >
          {icon}
        </Text>
      </group>
    </group>
  );
}
