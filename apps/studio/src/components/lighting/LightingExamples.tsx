/**
 * Lighting Configuration Examples
 *
 * Production-ready lighting setups for different product types and scenarios.
 * Copy-paste ready code snippets with detailed comments.
 */

import { Canvas } from '@react-three/fiber';
import {
  Environment,
  ContactShadows,
  Sphere,
  Box,
  OrbitControls,
} from '@react-three/drei';

/**
 * Example 1: Jewelry (Highly Reflective)
 *
 * Optimized for gold, silver, diamonds with strong environment reflection.
 * Minimal shadow for sparkle, high-quality environment.
 */
export function JewelryLighting() {
  return (
    <Canvas camera={{ position: [0, 0, 4], fov: 50 }}>
      <color attach="background" args={['#1a1a1a']} />

      {/* Premium environment for reflections */}
      <Environment preset="studio" environmentIntensity={1.5} />

      {/* Strong key light for sparkle */}
      <directionalLight
        position={[8, 10, 6]}
        intensity={2.2}
        castShadow
        shadow-mapSize-width={2048}
        shadow-mapSize-height={2048}
        shadow-bias={-0.0001}
        shadow-radius={4}
      />

      {/* Subtle fill light */}
      <directionalLight position={[-5, 3, 3]} intensity={0.35} />

      {/* Bright backlight for edge definition */}
      <directionalLight position={[0, 8, -12]} intensity={1.2} />

      <ambientLight intensity={0.4} />

      {/* Jewelry placeholder */}
      <Sphere args={[0.8, 256, 256]} castShadow receiveShadow>
        <meshStandardMaterial
          color="#fff9e6"
          metalness={0.95}
          roughness={0.05}
          envMapIntensity={1.2}
        />
      </Sphere>

      {/* Minimal shadow */}
      <ContactShadows opacity={0.5} scale={12} blur={1} resolution={256} />

      <OrbitControls autoRotate autoRotateSpeed={3} />
    </Canvas>
  );
}

/**
 * Example 2: Matte Consumer Product (Phone, Cosmetics)
 *
 * Balanced lighting for matte surfaces with soft shadows.
 * Emphasizes form over shine.
 */
export function MattProductLighting() {
  return (
    <Canvas
      camera={{ position: [0, 0.5, 5], fov: 48 }}
      shadows
    >
      <color attach="background" args={['#0f0f0f']} />

      <Environment preset="apartment" environmentIntensity={0.9} />

      {/* Softer key light for matte */}
      <directionalLight
        position={[5, 7, 5]}
        intensity={1.6}
        castShadow
        shadow-mapSize-width={2048}
        shadow-mapSize-height={2048}
        shadow-bias={-0.0001}
        shadow-radius={8}
      />

      <directionalLight position={[-4, 4, 4]} intensity={0.55} />
      <directionalLight position={[0, 5, -10]} intensity={0.75} />

      <ambientLight intensity={0.3} />

      {/* Matte product */}
      <Box args={[1.5, 2.5, 0.3]} castShadow receiveShadow>
        <meshStandardMaterial
          color="#ffffff"
          metalness={0.05}
          roughness={0.45}
        />
      </Box>

      {/* Visible shadow for form definition */}
      <ContactShadows
        opacity={0.75}
        scale={18}
        blur={2.5}
        far={12}
        resolution={512}
      />

      <OrbitControls autoRotate autoRotateSpeed={2} />
    </Canvas>
  );
}

/**
 * Example 3: Luxury Product (Watch, Pen)
 *
 * High-contrast lighting with dramatic shadows.
 * Emphasizes craftsmanship and materials.
 */
export function LuxuryProductLighting() {
  return (
    <Canvas camera={{ position: [0, 0, 5], fov: 45 }}>
      <color attach="background" args={['#0a0a0a']} />

      {/* Warm luxury environment */}
      <Environment preset="sunset" environmentIntensity={1.1} />

      {/* Dramatic key light */}
      <directionalLight
        position={[7, 9, 8]}
        intensity={2}
        castShadow
        shadow-mapSize-width={2048}
        shadow-mapSize-height={2048}
        shadow-camera-left={-10}
        shadow-camera-right={10}
        shadow-camera-top={10}
        shadow-camera-bottom={-10}
        shadow-bias={-0.00012}
        shadow-radius={6}
      />

      {/* Minimal fill for drama */}
      <directionalLight position={[-3, 2, 2]} intensity={0.3} />

      {/* Strong rim light */}
      <directionalLight position={[0, 7, -15]} intensity={1.3} />

      <ambientLight intensity={0.15} />

      {/* Luxury product */}
      <Sphere args={[1.2, 128, 128]} castShadow receiveShadow>
        <meshStandardMaterial
          color="#2a2a2a"
          metalness={0.9}
          roughness={0.15}
          envMapIntensity={0.9}
        />
      </Sphere>

      {/* Deep, defined shadow */}
      <ContactShadows
        opacity={0.85}
        scale={20}
        blur={2}
        far={10}
        resolution={512}
        color="#000000"
      />

      <OrbitControls autoRotate autoRotateSpeed={1.5} />
    </Canvas>
  );
}

/**
 * Example 4: Food Photography Simulation
 *
 * Warm lighting with strong backlighting to make food appealing.
 * High fill light to show texture without harsh shadows.
 */
export function FoodPhotographyLighting() {
  return (
    <Canvas camera={{ position: [0, 0.3, 4], fov: 50 }}>
      <color attach="background" args={['#2a2520']} />

      {/* Warm environment */}
      <Environment preset="sunset" environmentIntensity={1.4} />

      {/* Warm key light */}
      <directionalLight
        position={[5, 8, 6]}
        intensity={1.7}
        color="#ffe0b6"
        castShadow
        shadow-mapSize-width={1024}
        shadow-mapSize-height={1024}
        shadow-bias={-0.0001}
        shadow-radius={5}
      />

      {/* Strong fill to show texture */}
      <directionalLight
        position={[-6, 4, 3]}
        intensity={0.8}
        color="#ffccaa"
      />

      {/* Backlighting for glow */}
      <directionalLight
        position={[0, 6, -10]}
        intensity={1.4}
        color="#ffe4cc"
      />

      <ambientLight intensity={0.35} color="#ffccaa" />

      {/* Food product simulation */}
      <Box args={[2, 1, 0.5]} castShadow receiveShadow>
        <meshStandardMaterial
          color="#d4885f"
          metalness={0}
          roughness={0.6}
        />
      </Box>

      {/* Soft shadow for food */}
      <ContactShadows
        opacity={0.6}
        scale={20}
        blur={3}
        far={15}
        resolution={512}
      />

      <OrbitControls autoRotate autoRotateSpeed={1} />
    </Canvas>
  );
}

/**
 * Example 5: Technical Product (Electronics, Gadgets)
 *
 * Neutral, clinical lighting emphasizing precision and engineering.
 * Clear shadows with technical feeling.
 */
export function TechnicalProductLighting() {
  return (
    <Canvas
      camera={{ position: [0, 0.5, 5], fov: 50 }}
      gl={{ antialias: true }}
    >
      <color attach="background" args={['#1a1a1a']} />

      {/* Neutral environment */}
      <Environment preset="city" environmentIntensity={1} />

      {/* Precise key light */}
      <directionalLight
        position={[6, 8, 5]}
        intensity={1.8}
        color="#ffffff"
        castShadow
        shadow-mapSize-width={2048}
        shadow-mapSize-height={2048}
        shadow-bias={-0.0001}
        shadow-radius={5}
      />

      {/* Precise fill light */}
      <directionalLight
        position={[-5, 4, 4]}
        intensity={0.5}
        color="#ffffff"
      />

      {/* Technical backlighting */}
      <directionalLight
        position={[0, 6, -10]}
        intensity={0.8}
        color="#ffffff"
      />

      <ambientLight intensity={0.3} />

      {/* Technical product */}
      <Box args={[1.5, 1, 0.4]} castShadow receiveShadow position={[0, 0.5, 0]}>
        <meshStandardMaterial
          color="#333333"
          metalness={0.6}
          roughness={0.25}
        />
      </Box>

      {/* Clear technical shadow */}
      <ContactShadows
        opacity={0.7}
        scale={18}
        blur={1.5}
        far={12}
        resolution={512}
      />

      <OrbitControls autoRotate autoRotateSpeed={1.5} />
    </Canvas>
  );
}

/**
 * Example 6: Fabric/Textile Product
 *
 * Soft, diffuse lighting to show texture without harsh shadows.
 * Multiple fill lights for even illumination.
 */
export function FabricProductLighting() {
  return (
    <Canvas camera={{ position: [0, 0, 4.5], fov: 50 }}>
      <color attach="background" args={['#1c1c1c']} />

      {/* Soft environment */}
      <Environment preset="forest" environmentIntensity={1.1} />

      {/* Soft key light */}
      <directionalLight
        position={[4, 6, 5]}
        intensity={1.3}
        castShadow
        shadow-mapSize-width={1024}
        shadow-mapSize-height={1024}
        shadow-bias={-0.0001}
        shadow-radius={10}
      />

      {/* Multiple fill lights for soft texture */}
      <directionalLight position={[-5, 5, 3]} intensity={0.7} />
      <directionalLight position={[0, 4, 5]} intensity={0.5} />
      <directionalLight position={[0, 5, -8]} intensity={0.6} />

      <ambientLight intensity={0.4} />

      {/* Fabric product */}
      <Box args={[2, 2.5, 0.2]} castShadow receiveShadow>
        <meshStandardMaterial
          color="#e8e8e8"
          metalness={0}
          roughness={0.8}
        />
      </Box>

      {/* Soft, diffuse shadow */}
      <ContactShadows
        opacity={0.5}
        scale={20}
        blur={4}
        far={12}
        resolution={256}
      />

      <OrbitControls autoRotate autoRotateSpeed={1} />
    </Canvas>
  );
}

/**
 * Example 7: Minimal/Artistic Showcase
 *
 * Gallery-style presentation with minimal lighting.
 * Emphasizes form and negative space.
 */
export function MinimalArtisticLighting() {
  return (
    <Canvas camera={{ position: [0, 0, 4], fov: 50 }}>
      <color attach="background" args={['#0d0d0d']} />

      {/* Minimal environment */}
      <Environment
        preset="night"
        environmentIntensity={0.5}
        background={false}
      />

      {/* Single sculptural key light */}
      <directionalLight
        position={[8, 10, 7]}
        intensity={1.5}
        castShadow
        shadow-mapSize-width={2048}
        shadow-mapSize-height={2048}
        shadow-bias={-0.0001}
        shadow-radius={4}
      />

      {/* Minimal ambient */}
      <ambientLight intensity={0.1} />

      {/* Artistic product */}
      <Sphere args={[1, 64, 64]} castShadow receiveShadow>
        <meshStandardMaterial
          color="#4a4a4a"
          metalness={0.3}
          roughness={0.4}
        />
      </Sphere>

      {/* Subtle shadow */}
      <ContactShadows
        opacity={0.6}
        scale={15}
        blur={2}
        far={10}
        resolution={512}
      />

      <OrbitControls autoRotate autoRotateSpeed={0.8} />
    </Canvas>
  );
}

/**
 * Example 8: Performance-Optimized Mobile Version
 *
 * Minimal configuration for mobile devices.
 * Single directional light, low-res shadows, no environment.
 */
export function MobileOptimizedLighting() {
  return (
    <Canvas
      camera={{ position: [0, 0, 4], fov: 50 }}
      gl={{ antialias: false }}
      dpr={1}
    >
      <color attach="background" args={['#1a1a1a']} />

      {/* Single efficient key light */}
      <directionalLight
        position={[5, 5, 5]}
        intensity={1.5}
        castShadow={false}
      />

      {/* Simple fill */}
      <directionalLight position={[-3, 3, 3]} intensity={0.4} />

      <ambientLight intensity={0.3} />

      {/* Product */}
      <Sphere args={[1, 32, 32]}>
        <meshStandardMaterial color="#ffffff" metalness={0.1} roughness={0.2} />
      </Sphere>

      {/* Ultra-low-res shadow */}
      <ContactShadows
        opacity={0.6}
        scale={12}
        blur={1}
        far={8}
        resolution={128}
        frames={1}
      />

      {/* No auto-rotate for performance */}
      <OrbitControls enableZoom={false} enablePan={false} />
    </Canvas>
  );
}

/**
 * Material Reference for White Products
 *
 * Recommended material settings for different white product finishes.
 */
export const WhiteProductMaterials = {
  glossyWhite: {
    color: '#ffffff',
    metalness: 0.15,
    roughness: 0.15,
    envMapIntensity: 0.9,
  },
  mattWhite: {
    color: '#ffffff',
    metalness: 0.05,
    roughness: 0.45,
    envMapIntensity: 0.6,
  },
  silverMetallic: {
    color: '#e8e8e8',
    metalness: 0.9,
    roughness: 0.1,
    envMapIntensity: 1.0,
  },
  pearlWhite: {
    color: '#f5f5f5',
    metalness: 0.6,
    roughness: 0.2,
    envMapIntensity: 0.8,
  },
  ceramicWhite: {
    color: '#fafafa',
    metalness: 0.0,
    roughness: 0.5,
    envMapIntensity: 0.4,
  },
};

/**
 * Lighting Presets
 *
 * Quick reference for light intensity configurations.
 */
export const LightingPresets = {
  studio: {
    keyIntensity: 1.8,
    fillIntensity: 0.5,
    backIntensity: 0.9,
    ambientIntensity: 0.25,
    shadowRadius: 6,
  },
  dramatic: {
    keyIntensity: 2.0,
    fillIntensity: 0.3,
    backIntensity: 1.2,
    ambientIntensity: 0.15,
    shadowRadius: 8,
  },
  soft: {
    keyIntensity: 1.4,
    fillIntensity: 0.7,
    backIntensity: 0.7,
    ambientIntensity: 0.4,
    shadowRadius: 10,
  },
  minimal: {
    keyIntensity: 1.2,
    fillIntensity: 0.4,
    backIntensity: 0.5,
    ambientIntensity: 0.2,
    shadowRadius: 4,
  },
};
