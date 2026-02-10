'use client';

/**
 * BlueprintBackground - Technical drawing grid with dimension lines
 *
 * Features:
 * - Dark blue grid pattern
 * - Dimension lines with measurements
 * - Technical annotations
 * - SuperNovae branding
 */

import { Text, Line } from '@react-three/drei';

const GRID_SIZE = 6;
const GRID_DIVISIONS = 12;
const BLUEPRINT_BLUE = '#1a365d';
const GRID_LINE_COLOR = '#2d4a6f';
const ANNOTATION_COLOR = '#4a7a9f';

interface DimensionLineProps {
  start: [number, number, number];
  end: [number, number, number];
  label: string;
  offset?: number;
}

function DimensionLine({ start, end, label, offset = 0.3 }: DimensionLineProps) {
  const midX = (start[0] + end[0]) / 2;
  const midZ = (start[2] + end[2]) / 2;
  const isHorizontal = Math.abs(start[0] - end[0]) > Math.abs(start[2] - end[2]);

  return (
    <group>
      {/* Main dimension line */}
      <Line
        points={[start, end]}
        color={ANNOTATION_COLOR}
        lineWidth={1}
      />

      {/* End caps */}
      <Line
        points={[
          [start[0], start[1], start[2] - (isHorizontal ? 0.1 : 0)],
          [start[0], start[1], start[2] + (isHorizontal ? 0.1 : 0)],
        ]}
        color={ANNOTATION_COLOR}
        lineWidth={1}
      />

      {/* Label */}
      <Text
        position={[
          midX + (isHorizontal ? 0 : offset),
          0.01,
          midZ + (isHorizontal ? offset : 0),
        ]}
        rotation={[-Math.PI / 2, 0, 0]}
        fontSize={0.12}
        color={ANNOTATION_COLOR}
        anchorX="center"
        anchorY="middle"
        font="/fonts/mono.woff"
      >
        {label}
      </Text>
    </group>
  );
}

export function BlueprintBackground() {
  return (
    <group position={[0, -0.01, 0]}>
      {/* Base plane */}
      <mesh rotation={[-Math.PI / 2, 0, 0]} position={[0, 0, 0]}>
        <planeGeometry args={[GRID_SIZE, GRID_SIZE]} />
        <meshBasicMaterial color={BLUEPRINT_BLUE} transparent opacity={0.95} />
      </mesh>

      {/* Grid lines */}
      <gridHelper
        args={[GRID_SIZE, GRID_DIVISIONS, GRID_LINE_COLOR, GRID_LINE_COLOR]}
        position={[0, 0.001, 0]}
      />

      {/* Dimension lines */}
      <DimensionLine
        start={[-1.5, 0.002, 2]}
        end={[1.5, 0.002, 2]}
        label="90.45mm"
      />
      <DimensionLine
        start={[2, 0.002, -1.5]}
        end={[2, 0.002, 1.5]}
        label="90.45mm"
        offset={0.4}
      />

      {/* Branding */}
      <Text
        position={[-2.5, 0.002, 2.5]}
        rotation={[-Math.PI / 2, 0, 0]}
        fontSize={0.15}
        color={ANNOTATION_COLOR}
        anchorX="left"
        anchorY="middle"
        font="/fonts/mono.woff"
      >
        SuperNovae Pad - NovaNet
      </Text>

      {/* Revision */}
      <Text
        position={[2.5, 0.002, 2.5]}
        rotation={[-Math.PI / 2, 0, 0]}
        fontSize={0.1}
        color={ANNOTATION_COLOR}
        anchorX="right"
        anchorY="middle"
        font="/fonts/mono.woff"
      >
        REV 1.0 2026-02
      </Text>
    </group>
  );
}
