'use client';

/**
 * SuperNovaePad3D - Main 3D canvas for the macropad configurator
 *
 * Based on reference images:
 * - 3x3 keycap grid
 * - Small knurled encoder on LEFT
 * - Large black knob on RIGHT
 * - Blueprint background with dimension lines
 * - Particle effects on key press
 *
 * Features:
 * - React Three Fiber canvas with postprocessing
 * - PresentationControls for orbit
 * - Bloom effect for LED glow
 * - Interactive keycaps and encoders
 * - Particle burst on key press
 */

import { Suspense, useCallback, useEffect, useMemo, useState, useRef } from 'react';
import { Canvas, useFrame } from '@react-three/fiber';
import {
  Environment,
  PresentationControls,
  ContactShadows,
  Center,
} from '@react-three/drei';
import { EffectComposer, Bloom, Vignette } from '@react-three/postprocessing';
import type { SuperNovaePad3DProps, KeyBinding } from './types';
import {
  BlueprintBackground,
  Chassis,
  Keycap,
  RotaryEncoder,
  ParticleSystem,
  createParticleBurst,
  updateParticles,
} from './components';
import type { Particle } from './components';
import { DEFAULT_BINDINGS, getVisualEncoding } from './utils/visualEncoding';

// Keycap grid position offset (centered)
const KEYCAP_BASE_Y = 0.55;

// Encoder positions based on reference images
// Small knurled encoder: LEFT side, slightly forward
const SMALL_ENCODER_POS: [number, number, number] = [-1.9, 0.55, -0.8];
// Large black knob: RIGHT side
const LARGE_ENCODER_POS: [number, number, number] = [1.9, 0.55, -0.3];

// Default bindings for the 9 keys
function createDefaultBindings(): KeyBinding[] {
  return DEFAULT_BINDINGS.map((b, i) => ({
    id: `key-${i}`,
    layer: b.layer,
    action: b.action,
    label: b.layer,
  }));
}

interface PadSceneProps {
  bindings: KeyBinding[];
  onKeyPress: (binding: KeyBinding, position: [number, number, number]) => void;
  onEncoderChange: (encoderId: string, delta: number) => void;
  particles: Particle[];
}

function PadScene({ bindings, onKeyPress, onEncoderChange, particles }: PadSceneProps) {
  return (
    <>
      {/* Environment and lighting */}
      <Environment preset="city" />
      <ambientLight intensity={0.4} />

      {/* Main directional light */}
      <directionalLight
        position={[5, 8, 5]}
        intensity={0.8}
        castShadow
        shadow-mapSize-width={2048}
        shadow-mapSize-height={2048}
        shadow-camera-far={20}
        shadow-camera-left={-5}
        shadow-camera-right={5}
        shadow-camera-top={5}
        shadow-camera-bottom={-5}
      />

      {/* Accent lights for dramatic effect */}
      <pointLight position={[-3, 3, 2]} intensity={0.3} color="#6366f1" />
      <pointLight position={[3, 3, -2]} intensity={0.3} color="#3b82f6" />

      {/* Presentation controls for orbit - matching reference angles */}
      <PresentationControls
        global
        polar={[-Math.PI / 3, Math.PI / 6]}
        azimuth={[-Math.PI / 3, Math.PI / 3]}
        snap
        rotation={[Math.PI / 8, -Math.PI / 12, 0]}
      >
        <Center>
          <group>
            {/* Blueprint background */}
            <BlueprintBackground />

            {/* Metal chassis */}
            <Chassis />

            {/* 9 keycaps in 3x3 grid */}
            {bindings.map((binding, index) => {
              // Calculate key position for particle emission
              const row = Math.floor(index / 3);
              const col = index % 3;
              const keyPos: [number, number, number] = [
                (col - 1) * 0.95,
                KEYCAP_BASE_Y,
                (row - 1) * 0.95,
              ];

              return (
                <Keycap
                  key={binding.id}
                  position={[0, KEYCAP_BASE_Y, 0]}
                  binding={binding}
                  index={index}
                  onPress={() => onKeyPress(binding, keyPos)}
                />
              );
            })}

            {/* Small knurled encoder - LEFT side */}
            <RotaryEncoder
              position={SMALL_ENCODER_POS}
              size="small"
              onChange={(delta) => onEncoderChange('encoder-small', delta)}
              onClick={() => onEncoderChange('encoder-small', 0)}
            />

            {/* Large black knob - RIGHT side */}
            <RotaryEncoder
              position={LARGE_ENCODER_POS}
              size="large"
              onChange={(delta) => onEncoderChange('encoder-large', delta)}
              onClick={() => onEncoderChange('encoder-large', 0)}
            />
          </group>
        </Center>
      </PresentationControls>

      {/* Particle system */}
      <ParticleSystem particles={particles} />

      {/* Contact shadows for grounding */}
      <ContactShadows
        position={[0, -0.02, 0]}
        opacity={0.5}
        scale={8}
        blur={2.5}
        far={4}
        color="#0a1020"
      />

      {/* Postprocessing effects */}
      <EffectComposer multisampling={4}>
        <Bloom
          intensity={0.6}
          luminanceThreshold={0.8}
          luminanceSmoothing={0.7}
          mipmapBlur
        />
        <Vignette eskil={false} offset={0.1} darkness={0.4} />
      </EffectComposer>
    </>
  );
}

function LoadingFallback() {
  return (
    <mesh>
      <boxGeometry args={[2, 0.3, 2]} />
      <meshBasicMaterial color="#1a2a4a" wireframe />
    </mesh>
  );
}

// Particle manager component
function ParticleManager({
  particlesRef,
}: {
  particlesRef: React.MutableRefObject<Particle[]>;
}) {
  useFrame((_, delta) => {
    particlesRef.current = updateParticles(particlesRef.current, delta);
  });
  return null;
}

export function SuperNovaePad3D({
  bindings,
  onKeyPress,
  onEncoderChange,
}: SuperNovaePad3DProps) {
  // Use default bindings if none provided
  const keyBindings = useMemo(
    () => bindings ?? createDefaultBindings(),
    [bindings]
  );

  // Particle state
  const [particles, setParticles] = useState<Particle[]>([]);
  const particlesRef = useRef<Particle[]>([]);

  // Sync ref with state for useFrame - must be in useEffect to avoid render-time ref access
  useEffect(() => {
    particlesRef.current = particles;
  }, [particles]);

  // Key press handler with particle burst
  const handleKeyPress = useCallback(
    (binding: KeyBinding, position: [number, number, number]) => {
      onKeyPress?.(binding);

      // Get color for particles
      const { color } = getVisualEncoding(binding.layer);

      // Create particle burst
      const newParticles = createParticleBurst(position, color, 20);
      setParticles((prev) => [...prev, ...newParticles].slice(-500));

      console.log('[SuperNovaePad3D] Key pressed:', binding.layer, binding.action);
    },
    [onKeyPress]
  );

  // Encoder handler
  const handleEncoderChange = useCallback(
    (encoderId: string, delta: number) => {
      onEncoderChange?.(encoderId, delta);
      console.log('[SuperNovaePad3D] Encoder:', encoderId, delta);
    },
    [onEncoderChange]
  );

  // Update particles each frame
  useCallback(() => {
    const interval = setInterval(() => {
      setParticles((prev) => updateParticles(prev, 1 / 60));
    }, 16);
    return () => clearInterval(interval);
  }, []);

  return (
    <Canvas
      camera={{ position: [0, 5, 6], fov: 40 }}
      shadows
      dpr={[1, 2]}
      gl={{
        antialias: true,
        alpha: true,
        powerPreference: 'high-performance',
      }}
      style={{ background: 'transparent' }}
    >
      <Suspense fallback={<LoadingFallback />}>
        <PadScene
          bindings={keyBindings}
          onKeyPress={handleKeyPress}
          onEncoderChange={handleEncoderChange}
          particles={particles}
        />
        <ParticleManager particlesRef={particlesRef} />
      </Suspense>
    </Canvas>
  );
}
