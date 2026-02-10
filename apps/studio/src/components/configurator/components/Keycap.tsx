'use client';

/**
 * Keycap - Interactive translucent keycap with LED glow
 *
 * Features:
 * - MeshTransmissionMaterial for realistic glass effect
 * - Hover: scale 1.05, increased glow
 * - Press: scale 0.95, Y offset, elastic bounce
 * - Particle burst on press
 * - Layer icon displayed on top
 */

import { useState, useRef, useCallback, useEffect } from 'react';
import { useFrame, ThreeEvent } from '@react-three/fiber';
import { RoundedBox, Text, MeshTransmissionMaterial } from '@react-three/drei';
import * as THREE from 'three';
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

export function Keycap({ position, binding, onPress, index }: KeycapProps) {
  const [hovered, setHovered] = useState(false);
  const [pressed, setPressed] = useState(false);
  const groupRef = useRef<THREE.Group>(null);
  const glowRef = useRef<THREE.Mesh>(null);

  // Animation state with spring physics
  const scaleRef = useRef(1);
  const yOffsetRef = useRef(0);
  const velocityRef = useRef({ scale: 0, y: 0 });
  const glowIntensityRef = useRef(0.3);

  // Get visual encoding for this layer
  const { color, icon } = getVisualEncoding(binding.layer);
  const colorObj = new THREE.Color(color);

  // Calculate grid position (3x3 grid)
  const row = Math.floor(index / 3);
  const col = index % 3;
  const x = (col - 1) * KEYCAP_GAP;
  const z = (row - 1) * KEYCAP_GAP;

  // Spring animation
  useFrame((_, delta) => {
    const targetScale = pressed ? PRESS_SCALE : hovered ? HOVER_SCALE : 1;
    const targetY = pressed ? PRESS_Y_OFFSET : 0;
    const targetGlow = pressed ? 0.9 : hovered ? 0.6 : 0.35;

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

    // Smooth glow transition
    glowIntensityRef.current += (targetGlow - glowIntensityRef.current) * 8 * delta;

    // Apply transforms
    if (groupRef.current) {
      groupRef.current.scale.setScalar(scaleRef.current);
      groupRef.current.position.y = position[1] + yOffsetRef.current;
    }

    // Update glow mesh
    if (glowRef.current) {
      glowRef.current.scale.setScalar(1.15 + glowIntensityRef.current * 0.3);
      const mat = glowRef.current.material as THREE.MeshBasicMaterial;
      mat.opacity = glowIntensityRef.current;
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
      {/* Glow halo underneath */}
      <mesh
        ref={glowRef}
        position={[0, -KEYCAP_HEIGHT / 2, 0]}
        rotation={[-Math.PI / 2, 0, 0]}
      >
        <circleGeometry args={[KEYCAP_SIZE * 0.7, 32]} />
        <meshBasicMaterial
          color={color}
          transparent
          opacity={0.3}
          blending={THREE.AdditiveBlending}
        />
      </mesh>

      {/* Main keycap group */}
      <group ref={groupRef}>
        {/* Keycap body - translucent with transmission */}
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
          <MeshTransmissionMaterial
            transmission={0.85}
            thickness={1.5}
            roughness={0.05}
            chromaticAberration={0.03}
            anisotropicBlur={0.3}
            color={colorObj}
            emissive={color}
            emissiveIntensity={pressed ? 2.5 : hovered ? 1.8 : 1.2}
            toneMapped={false}
            samples={4}
            resolution={256}
          />
        </RoundedBox>

        {/* Inner LED light source */}
        <pointLight
          position={[0, -KEYCAP_HEIGHT * 0.3, 0]}
          color={color}
          intensity={pressed ? 3 : hovered ? 2 : 1}
          distance={1.2}
          decay={2}
        />

        {/* Icon on top of keycap */}
        <Text
          position={[0, KEYCAP_HEIGHT / 2 + 0.01, 0]}
          rotation={[-Math.PI / 2, 0, 0]}
          fontSize={0.22}
          color="#ffffff"
          anchorX="center"
          anchorY="middle"
          outlineWidth={0.015}
          outlineColor="#000000"
          font="/fonts/inter-bold.woff"
        >
          {icon}
        </Text>
      </group>
    </group>
  );
}
