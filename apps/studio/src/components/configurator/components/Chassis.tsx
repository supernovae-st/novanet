'use client';

/**
 * Chassis - Metal body of the macropad matching reference images
 *
 * Features:
 * - Dark blue anodized aluminum look
 * - Heavily rounded corners (like reference images)
 * - Inner recess for keycaps
 * - USB-C port at bottom
 * - Blueprint wireframe edges
 */

import { useMemo } from 'react';
import { RoundedBox, Edges } from '@react-three/drei';
import * as THREE from 'three';

// Chassis dimensions (from reference: ~36mm per section, 3x3 grid)
const CHASSIS_WIDTH = 3.4;
const CHASSIS_DEPTH = 3.4;
const CHASSIS_HEIGHT = 0.35;
const CHASSIS_RADIUS = 0.25;
const INNER_INSET = 0.15;
const INNER_DEPTH = 0.12;

// Material colors matching Figma design - light gray chassis
const CHASSIS_COLOR = '#e5e7eb';  // gray-200 - light gray body
const INNER_COLOR = '#d1d5db';    // gray-300 - slightly darker recess
const EDGE_COLOR = '#9ca3af';     // gray-400 - subtle edges
const USB_COLOR = '#374151';      // gray-700 - dark USB port

export function Chassis() {
  // USB-C port shape
  const usbGeometry = useMemo(() => {
    const shape = new THREE.Shape();
    const w = 0.35;
    const h = 0.12;
    const r = 0.04;

    shape.moveTo(-w / 2 + r, -h / 2);
    shape.lineTo(w / 2 - r, -h / 2);
    shape.quadraticCurveTo(w / 2, -h / 2, w / 2, -h / 2 + r);
    shape.lineTo(w / 2, h / 2 - r);
    shape.quadraticCurveTo(w / 2, h / 2, w / 2 - r, h / 2);
    shape.lineTo(-w / 2 + r, h / 2);
    shape.quadraticCurveTo(-w / 2, h / 2, -w / 2, h / 2 - r);
    shape.lineTo(-w / 2, -h / 2 + r);
    shape.quadraticCurveTo(-w / 2, -h / 2, -w / 2 + r, -h / 2);

    return new THREE.ExtrudeGeometry(shape, { depth: 0.15, bevelEnabled: false });
  }, []);

  return (
    <group position={[0, 0, 0]}>
      {/* Main outer body */}
      <RoundedBox
        args={[CHASSIS_WIDTH, CHASSIS_HEIGHT, CHASSIS_DEPTH]}
        radius={CHASSIS_RADIUS}
        smoothness={8}
        position={[0, CHASSIS_HEIGHT / 2, 0]}
        castShadow
        receiveShadow
      >
        <meshStandardMaterial
          color={CHASSIS_COLOR}
          metalness={0.3}
          roughness={0.6}
          envMapIntensity={0.8}
        />
        <Edges scale={1.002} threshold={15} color={EDGE_COLOR} linewidth={1} />
      </RoundedBox>

      {/* Inner recess for keycaps */}
      <RoundedBox
        args={[
          CHASSIS_WIDTH - INNER_INSET * 2,
          INNER_DEPTH,
          CHASSIS_DEPTH - INNER_INSET * 2,
        ]}
        radius={CHASSIS_RADIUS - 0.05}
        smoothness={4}
        position={[0, CHASSIS_HEIGHT + INNER_DEPTH / 2 - 0.01, 0]}
        receiveShadow
      >
        <meshStandardMaterial
          color={INNER_COLOR}
          metalness={0.2}
          roughness={0.7}
        />
      </RoundedBox>

      {/* Bottom plate */}
      <mesh position={[0, 0.02, 0]}>
        <boxGeometry args={[CHASSIS_WIDTH - 0.1, 0.04, CHASSIS_DEPTH - 0.1]} />
        <meshStandardMaterial
          color="#9ca3af"
          metalness={0.4}
          roughness={0.5}
        />
      </mesh>

      {/* USB-C port */}
      <mesh
        geometry={usbGeometry}
        position={[0, CHASSIS_HEIGHT / 2, CHASSIS_DEPTH / 2 - 0.01]}
        rotation={[Math.PI / 2, 0, 0]}
      >
        <meshStandardMaterial
          color={USB_COLOR}
          metalness={0.95}
          roughness={0.15}
        />
      </mesh>

      {/* USB-C inner connector */}
      <mesh position={[0, CHASSIS_HEIGHT / 2, CHASSIS_DEPTH / 2 + 0.06]}>
        <boxGeometry args={[0.22, 0.06, 0.08]} />
        <meshStandardMaterial color="#050505" metalness={0.3} roughness={0.8} />
      </mesh>

      {/* Corner screws (black like in Figma) */}
      {[
        [-CHASSIS_WIDTH / 2 + 0.25, CHASSIS_HEIGHT + 0.01, -CHASSIS_DEPTH / 2 + 0.25],
        [CHASSIS_WIDTH / 2 - 0.25, CHASSIS_HEIGHT + 0.01, -CHASSIS_DEPTH / 2 + 0.25],
        [-CHASSIS_WIDTH / 2 + 0.25, CHASSIS_HEIGHT + 0.01, CHASSIS_DEPTH / 2 - 0.25],
        [CHASSIS_WIDTH / 2 - 0.25, CHASSIS_HEIGHT + 0.01, CHASSIS_DEPTH / 2 - 0.25],
      ].map((pos, i) => (
        <mesh key={i} position={pos as [number, number, number]}>
          <cylinderGeometry args={[0.06, 0.06, 0.03, 16]} />
          <meshStandardMaterial color="#1f2937" metalness={0.2} roughness={0.8} />
        </mesh>
      ))}

      {/* Side text branding */}
      {/* Left side: "SuperNovae Pad - NovaNet" */}
      <mesh
        position={[-CHASSIS_WIDTH / 2 - 0.001, CHASSIS_HEIGHT / 2, 0]}
        rotation={[0, -Math.PI / 2, 0]}
      >
        <planeGeometry args={[CHASSIS_DEPTH * 0.8, CHASSIS_HEIGHT * 0.5]} />
        <meshBasicMaterial transparent opacity={0} />
      </mesh>
    </group>
  );
}
