'use client';

/* eslint-disable @typescript-eslint/no-explicit-any -- Three.js/gesture library interop requires any casts */

/**
 * RotaryEncoder - Draggable rotary encoder with knurled texture
 *
 * Based on reference images:
 * - Small knurled silver encoder (left side)
 * - Large black matte knob (right side)
 *
 * Features:
 * - Drag to rotate with visual feedback
 * - Click for default action
 * - Knurled texture pattern for small encoder
 * - Smooth matte finish for large knob
 */

import { useState, useRef, useCallback, useMemo } from 'react';
import { useThree, ThreeEvent } from '@react-three/fiber';
import { useDrag } from '@use-gesture/react';
import { Cylinder } from '@react-three/drei';
import * as THREE from 'three';
import type { RotaryEncoderProps } from '../types';

// Encoder dimensions from reference images
const SMALL_RADIUS = 0.22;
const SMALL_HEIGHT = 0.35;
const LARGE_RADIUS = 0.35;
const LARGE_HEIGHT = 0.45;
const KNURL_SEGMENTS = 48;

export function RotaryEncoder({
  position,
  size,
  onChange,
  onClick,
}: RotaryEncoderProps) {
  const [rotation, setRotation] = useState(0);
  const [dragging, setDragging] = useState(false);
  const [hovered, setHovered] = useState(false);
  const groupRef = useRef<THREE.Group>(null);
  const { size: canvasSize, viewport } = useThree();

  const isSmall = size === 'small';
  const radius = isSmall ? SMALL_RADIUS : LARGE_RADIUS;
  const height = isSmall ? SMALL_HEIGHT : LARGE_HEIGHT;

  // Knurl geometry for small encoder
  const knurlGeometry = useMemo(() => {
    if (!isSmall) return null;

    const points: number[] = [];
    for (let i = 0; i < KNURL_SEGMENTS; i++) {
      const angle = (i / KNURL_SEGMENTS) * Math.PI * 2;
      const r = SMALL_RADIUS * 1.03;

      // Vertical lines for knurling effect
      points.push(
        Math.cos(angle) * r, -SMALL_HEIGHT / 2 + 0.02, Math.sin(angle) * r,
        Math.cos(angle) * r, SMALL_HEIGHT / 2 - 0.02, Math.sin(angle) * r
      );
    }

    const geo = new THREE.BufferGeometry();
    geo.setAttribute('position', new THREE.Float32BufferAttribute(points, 3));
    return geo;
  }, [isSmall]);

  // Drag gesture
  const bind = useDrag(
    ({ movement: [mx], first, last, memo }) => {
      if (first) {
        setDragging(true);
        document.body.style.cursor = 'grabbing';
        return rotation;
      }

      const initial = memo ?? rotation;
      const aspect = canvasSize.width / viewport.width;
      const sensitivity = isSmall ? 60 : 40;
      const newRotation = initial + mx / (aspect * sensitivity);
      setRotation(newRotation);

      // Calculate delta for action
      const delta = newRotation - initial;
      if (Math.abs(delta) > 0.05) {
        onChange(delta > 0 ? 1 : -1);
      }

      if (last) {
        setDragging(false);
        document.body.style.cursor = hovered ? 'grab' : 'default';
      }

      return initial;
    },
    { filterTaps: true }
  );

  const handleClick = useCallback((e: ThreeEvent<MouseEvent>) => {
    if (!dragging && onClick) {
      e.stopPropagation();
      onClick();
    }
  }, [dragging, onClick]);

  const handlePointerOver = useCallback(() => {
    setHovered(true);
    if (!dragging) {
      document.body.style.cursor = 'grab';
    }
  }, [dragging]);

  const handlePointerOut = useCallback(() => {
    setHovered(false);
    if (!dragging) {
      document.body.style.cursor = 'default';
    }
  }, [dragging]);

  return (
    <group ref={groupRef} position={position}>
      {/* Base ring (matching light gray chassis) */}
      <mesh position={[0, -height / 2 - 0.03, 0]}>
        <cylinderGeometry args={[radius * 1.15, radius * 1.15, 0.06, 32]} />
        <meshStandardMaterial
          color="#9ca3af"
          metalness={0.3}
          roughness={0.6}
        />
      </mesh>

      {isSmall ? (
        // Small knurled silver encoder
        <group rotation-y={rotation}>
          {/* Main body */}
          <Cylinder
            args={[SMALL_RADIUS, SMALL_RADIUS, SMALL_HEIGHT, 32]}
            onClick={handleClick}
            onPointerOver={handlePointerOver}
            onPointerOut={handlePointerOut}
            castShadow
            {...(bind() as any)}
          >
            <meshStandardMaterial
              color="#b8c4ce"
              metalness={0.95}
              roughness={0.25}
              envMapIntensity={1.5}
            />
          </Cylinder>

          {/* Knurling lines */}
          {knurlGeometry && (
            <lineSegments geometry={knurlGeometry}>
              <lineBasicMaterial color="#8a9aa8" transparent opacity={0.6} />
            </lineSegments>
          )}

          {/* Top cap with indicator */}
          <mesh position={[0, SMALL_HEIGHT / 2 + 0.01, 0]}>
            <cylinderGeometry args={[SMALL_RADIUS * 0.85, SMALL_RADIUS * 0.85, 0.02, 32]} />
            <meshStandardMaterial
              color="#c8d4de"
              metalness={0.9}
              roughness={0.2}
            />
          </mesh>

          {/* Indicator dot */}
          <mesh position={[0, SMALL_HEIGHT / 2 + 0.025, -SMALL_RADIUS * 0.5]}>
            <sphereGeometry args={[0.03, 16, 16]} />
            <meshStandardMaterial color="#ffffff" emissive="#ffffff" emissiveIntensity={0.3} />
          </mesh>
        </group>
      ) : (
        // Large black matte knob
        <group rotation-y={rotation}>
          {/* Main body - tapered cylinder */}
          <Cylinder
            args={[LARGE_RADIUS * 0.9, LARGE_RADIUS, LARGE_HEIGHT, 32]}
            onClick={handleClick}
            onPointerOver={handlePointerOver}
            onPointerOut={handlePointerOut}
            castShadow
            {...(bind() as any)}
          >
            <meshStandardMaterial
              color="#1a1a1a"
              metalness={0.1}
              roughness={0.8}
            />
          </Cylinder>

          {/* Top flat surface */}
          <mesh position={[0, LARGE_HEIGHT / 2 + 0.01, 0]}>
            <cylinderGeometry args={[LARGE_RADIUS * 0.85, LARGE_RADIUS * 0.85, 0.02, 32]} />
            <meshStandardMaterial
              color="#252525"
              metalness={0.05}
              roughness={0.9}
            />
          </mesh>

          {/* Indicator line on top */}
          <mesh position={[0, LARGE_HEIGHT / 2 + 0.025, -LARGE_RADIUS * 0.4]} rotation={[0, 0, Math.PI / 2]}>
            <boxGeometry args={[0.02, LARGE_RADIUS * 0.5, 0.02]} />
            <meshStandardMaterial color="#4a4a4a" />
          </mesh>
        </group>
      )}

      {/* Highlight ring when hovered */}
      {hovered && (
        <mesh position={[0, 0, 0]}>
          <torusGeometry args={[radius * 1.1, 0.015, 16, 32]} />
          <meshBasicMaterial
            color={isSmall ? '#6366f1' : '#3b82f6'}
            transparent
            opacity={0.6}
          />
        </mesh>
      )}
    </group>
  );
}
