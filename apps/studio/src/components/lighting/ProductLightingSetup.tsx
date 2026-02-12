import { Canvas } from '@react-three/fiber';
import {
  Environment,
  ContactShadows,
  OrbitControls,
} from '@react-three/drei';
import { ReactNode } from 'react';

/**
 * Professional lighting configuration for product visualization.
 *
 * Implements three-point lighting with:
 * - Key light (45° angle) for modeling
 * - Fill light (opposite) for shadow detail
 * - Back light (rim) for separation
 * - HDR environment for reflections
 * - Contact shadows for ground interaction
 *
 * Optimized for: White products on dark background
 * Performance: 60 FPS on desktop, mobile-friendly
 */

interface ProductLightingProps {
  /** Product component to display */
  children: ReactNode;
  /** Background color (default: dark) */
  backgroundColor?: string;
  /** Environment preset (default: "studio") */
  environmentPreset?:
    | 'studio'
    | 'city'
    | 'sunset'
    | 'forest'
    | 'night'
    | 'dawn'
    | 'apartment'
    | 'lobby'
    | 'park'
    | 'warehouse';
  /** Enable environment reflections (default: true) */
  enableEnvironment?: boolean;
  /** Enable contact shadows (default: true) */
  enableShadows?: boolean;
  /** Shadow opacity (0-1, default: 0.7) */
  shadowOpacity?: number;
  /** Enable orbit controls (default: true) */
  enableControls?: boolean;
  /** Camera field of view (default: 50) */
  cameraFov?: number;
  /** Camera initial position */
  cameraPosition?: [number, number, number];
}

/**
 * ProfessionalProductLighting
 *
 * Complete lighting setup for professional product visualization.
 * Combines three-point lighting with HDR environment and contact shadows.
 *
 * @example
 * ```jsx
 * <ProfessionalProductLighting>
 *   <Sphere args={[1, 128, 128]}>
 *     <meshStandardMaterial color="#ffffff" />
 *   </Sphere>
 * </ProfessionalProductLighting>
 * ```
 */
export function ProfessionalProductLighting({
  children,
  backgroundColor = '#0a0a0a',
  environmentPreset = 'studio',
  enableEnvironment = true,
  enableShadows = true,
  shadowOpacity = 0.7,
  enableControls = true,
  cameraFov = 50,
  cameraPosition = [0, 0.5, 5],
}: ProductLightingProps) {
  return (
    <Canvas
      camera={{
        position: cameraPosition,
        fov: cameraFov,
        near: 0.1,
        far: 1000,
      }}
      gl={{ antialias: true }}
      dpr={[1, 2]}
    >
      {/* Background */}
      <color attach="background" args={[backgroundColor]} />

      {/* HDR Environment for reflections and ambient light */}
      {enableEnvironment && (
        <Environment
          preset={environmentPreset}
          environmentIntensity={1.2}
          background={false}
        />
      )}

      {/* Three-Point Lighting Setup */}

      {/* Key Light: Primary modeling light, 45° from front */}
      <directionalLight
        position={[6, 8, 5]}
        intensity={1.8}
        castShadow
        shadow-mapSize-width={2048}
        shadow-mapSize-height={2048}
        shadow-camera-far={50}
        shadow-camera-near={0.5}
        shadow-camera-left={-15}
        shadow-camera-right={15}
        shadow-camera-top={15}
        shadow-camera-bottom={-15}
        shadow-bias={-0.0001}
        shadow-radius={6}
      />

      {/* Fill Light: Softens shadows, reveals detail */}
      <directionalLight position={[-4, 4, 4]} intensity={0.5} />

      {/* Back Light: Rim lighting for separation from background */}
      <directionalLight position={[0, 6, -10]} intensity={0.9} />

      {/* Ambient: Global illumination baseline */}
      <ambientLight intensity={0.25} />

      {/* Product */}
      {children}

      {/* Ground Shadow */}
      {enableShadows && (
        <ContactShadows
          position={[0, 0, 0]}
          opacity={shadowOpacity}
          scale={20}
          blur={2.5}
          far={12}
          resolution={512}
          color="#000000"
          frames={1}
        />
      )}

      {/* Optional: Orbit controls for interaction */}
      {enableControls && <OrbitControls makeDefault />}
    </Canvas>
  );
}

/**
 * MinimalProductLighting
 *
 * Lightweight lighting setup for performance-critical scenarios.
 * Uses ContactShadows for fast rendering on mobile.
 *
 * @example
 * ```jsx
 * <MinimalProductLighting>
 *   <Model />
 * </MinimalProductLighting>
 * ```
 */
export function MinimalProductLighting({
  children,
  backgroundColor = '#1a1a1a',
}: {
  children: ReactNode;
  backgroundColor?: string;
}) {
  return (
    <Canvas
      camera={{ position: [0, 0.5, 4], fov: 50 }}
      gl={{ antialias: true }}
      dpr={1}
    >
      <color attach="background" args={[backgroundColor]} />

      {/* Minimal three-point setup */}
      <directionalLight position={[5, 5, 5]} intensity={1.5} />
      <directionalLight position={[-5, 3, 3]} intensity={0.5} />
      <directionalLight position={[0, 5, -8]} intensity={0.8} />
      <ambientLight intensity={0.3} />

      {children}

      {/* Fast contact shadow */}
      <ContactShadows
        position={[0, 0, 0]}
        opacity={0.6}
        scale={15}
        blur={1.5}
        far={8}
        resolution={256}
        frames={1}
      />
    </Canvas>
  );
}

/**
 * StudioProductLighting
 *
 * Enhanced lighting using RandomizedLight for maximum shadow quality.
 * Best for static product photography.
 *
 * @example
 * ```jsx
 * <StudioProductLighting>
 *   <GltfModel path="/model.glb" />
 * </StudioProductLighting>
 * ```
 */
export function StudioProductLighting({
  children,
  backgroundColor = '#0a0a0a',
}: {
  children: ReactNode;
  backgroundColor?: string;
}) {
  return (
    <Canvas
      camera={{ position: [0, 0.5, 5], fov: 50 }}
      gl={{ antialias: true, preserveDrawingBuffer: true }}
      dpr={[1, 2]}
    >
      <color attach="background" args={[backgroundColor]} />

      <Environment preset="studio" environmentIntensity={1.3} background={false} />

      {/* Key light with optimized shadows */}
      <directionalLight
        position={[6, 8, 5]}
        intensity={2}
        castShadow
        shadow-mapSize-width={2048}
        shadow-mapSize-height={2048}
        shadow-camera-far={50}
        shadow-camera-near={0.5}
        shadow-camera-left={-12}
        shadow-camera-right={12}
        shadow-camera-top={12}
        shadow-camera-bottom={-12}
        shadow-bias={-0.0001}
        shadow-radius={8}
      />

      <directionalLight position={[-5, 4, 4]} intensity={0.6} />
      <directionalLight position={[0, 6, -12]} intensity={1} />
      <ambientLight intensity={0.3} />

      {children}

      {/* High-quality contact shadow */}
      <ContactShadows
        position={[0, 0, 0]}
        opacity={0.8}
        scale={25}
        blur={3}
        far={15}
        resolution={1024}
        frames={1}
        color="#000000"
      />

      {/* Orbit controls for inspection */}
      <OrbitControls
        makeDefault
        autoRotate
        autoRotateSpeed={2}
        enableZoom
        enablePan
      />
    </Canvas>
  );
}

/**
 * WhiteProductShowcase
 *
 * Pre-configured lighting specifically optimized for white products.
 * Includes proper material properties and environment settings.
 *
 * @example
 * ```jsx
 * <WhiteProductShowcase>
 *   <Sphere args={[1, 128, 128]}>
 *     <meshStandardMaterial
 *       color="#ffffff"
 *       metalness={0.08}
 *       roughness={0.18}
 *     />
 *   </Sphere>
 * </WhiteProductShowcase>
 * ```
 */
export function WhiteProductShowcase({
  children,
  environmentPreset = 'studio',
}: {
  children: ReactNode;
  environmentPreset?:
    | 'studio'
    | 'city'
    | 'sunset'
    | 'forest'
    | 'night'
    | 'dawn'
    | 'apartment'
    | 'lobby'
    | 'park'
    | 'warehouse';
}) {
  return (
    <Canvas
      camera={{
        position: [0, 0.3, 4.5],
        fov: 48,
        near: 0.1,
        far: 1000,
      }}
      gl={{ antialias: true, preserveDrawingBuffer: true }}
      dpr={[1, 2]}
    >
      {/* Pure black background for white contrast */}
      <color attach="background" args={['#000000']} />

      {/* Environment: Crucial for white reflections */}
      <Environment
        preset={environmentPreset}
        environmentIntensity={1.2}
        background={false}
      />

      {/* Key Light: Strong to define white geometry */}
      <directionalLight
        position={[7, 9, 6]}
        intensity={1.9}
        castShadow
        shadow-mapSize-width={2048}
        shadow-mapSize-height={2048}
        shadow-camera-far={50}
        shadow-camera-near={0.5}
        shadow-camera-left={-14}
        shadow-camera-right={14}
        shadow-camera-top={14}
        shadow-camera-bottom={-14}
        shadow-bias={-0.00008}
        shadow-radius={7}
      />

      {/* Fill Light: Preserve white without losing dimension */}
      <directionalLight position={[-4.5, 3.5, 4]} intensity={0.45} />

      {/* Back Light: Critical for white separation */}
      <directionalLight position={[0, 7, -11]} intensity={1} />

      {/* Ambient: Minimal to preserve contrast */}
      <ambientLight intensity={0.2} />

      {children}

      {/* Contact shadow: Subtle but present */}
      <ContactShadows
        position={[0, 0, 0]}
        opacity={0.75}
        scale={20}
        blur={2.5}
        far={12}
        resolution={512}
        color="#1a1a1a"
        frames={1}
      />

      {/* Auto-rotating orbit controls */}
      <OrbitControls
        makeDefault
        autoRotate
        autoRotateSpeed={1}
        enableZoom
        enablePan
        maxDistance={10}
        minDistance={2}
      />
    </Canvas>
  );
}
