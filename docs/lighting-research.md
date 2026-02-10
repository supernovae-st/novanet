# Professional Lighting Setup for 3D Product Visualization in React Three Fiber

**Date**: 2026-02-10
**Framework**: React Three Fiber + Drei
**Target**: White product on dark background with professional studio lighting

## Overview

This document provides comprehensive lighting strategies for product visualization using React Three Fiber and Drei. We focus on five key techniques:

1. **Three-point lighting** (key, fill, backlight)
2. **Environment maps** for realistic reflections and context
3. **Contact shadows vs shadow maps** (performance vs quality trade-offs)
4. **Soft shadows configuration** (visual quality)
5. **HDR lighting with Drei** (professional results)

---

## 1. Three-Point Lighting Setup

Three-point lighting is the foundation of professional product visualization. It consists of:

- **Key Light**: Primary light defining object shape and dimension
- **Fill Light**: Softens shadows, reveals detail
- **Backlight**: Separates product from background, adds depth

### Basic Three-Point Lighting Configuration

```jsx
import { Canvas } from '@react-three/fiber';
import { Sphere, useGLTF } from '@react-three/drei';

export function ProductWithThreePointLighting() {
  return (
    <Canvas
      camera={{ position: [0, 0, 5], fov: 50 }}
      gl={{ antialias: true, pixelRatio: window.devicePixelRatio }}
    >
      {/* Background - dark for contrast */}
      <color attach="background" args={['#1a1a1a']} />

      {/* Key Light: Main directional light, 45° from object */}
      <directionalLight
        position={[5, 5, 5]}
        intensity={1.5}
        castShadow
        shadow-mapSize-width={2048}
        shadow-mapSize-height={2048}
        shadow-camera-far={50}
        shadow-camera-near={0.5}
        shadow-camera-left={-10}
        shadow-camera-right={10}
        shadow-camera-top={10}
        shadow-camera-bottom={-10}
        shadow-bias={-0.0001}
        shadow-radius={8}
      />

      {/* Fill Light: Soft light to fill shadows, opposite side */}
      <directionalLight
        position={[-5, 3, 3]}
        intensity={0.6}
        castShadow={false}
      />

      {/* Backlight: Creates rim lighting for separation */}
      <directionalLight
        position={[0, 5, -8]}
        intensity={0.8}
        castShadow={false}
      />

      {/* Ambient light: Global illumination baseline */}
      <ambientLight intensity={0.3} />

      {/* Product */}
      <Sphere args={[1, 64, 64]} castShadow receiveShadow>
        <meshStandardMaterial
          color="#ffffff"
          metalness={0.1}
          roughness={0.2}
        />
      </Sphere>

      {/* Ground plane for shadow reception */}
      <mesh position={[0, -1, 0]} receiveShadow rotation={[-Math.PI / 2, 0, 0]}>
        <planeGeometry args={[20, 20]} />
        <meshStandardMaterial color="#1a1a1a" />
      </mesh>
    </Canvas>
  );
}
```

**Key Light Properties**:
- Position: 45° angle (x: 5, y: 5, z: 5) for dimensional modeling
- Intensity: 1.5 for dominant illumination
- Shadow parameters tuned for 2048px maps
- Radius: 8 for soft shadow edges

**Fill Light Properties**:
- Position: Opposite side (x: -5), slightly lower
- Intensity: 0.6 (40% of key light) to avoid washing out shadows
- No shadow casting to keep calculation simple

**Backlight Properties**:
- Position: Behind object for rim effect
- Intensity: 0.8 to separate product from background
- Subtle but effective for depth

---

## 2. Environment Maps for Reflections

Environment maps provide realistic reflections and ambient light. Drei offers preset environments and custom HDR support.

### Studio Environment with Reflections

```jsx
import { Canvas } from '@react-three/fiber';
import { Environment, Sphere } from '@react-three/drei';

export function ProductWithEnvironment() {
  return (
    <Canvas camera={{ position: [0, 0, 5] }}>
      {/* Dark background with studio preset */}
      <Environment
        preset="studio"          // Built-in studio environment
        background={false}       // Don't use as scene background
        backgroundIntensity={1}
        environmentIntensity={1.2} // Boost reflections slightly
      />

      {/* Optional: Add custom lighting on top of environment */}
      <directionalLight position={[5, 5, 5]} intensity={1} castShadow />

      {/* Product with high metalness shows environment */}
      <Sphere args={[1, 64, 64]} castShadow receiveShadow>
        <meshStandardMaterial
          color="#ffffff"
          metalness={0.8}        // High reflectivity
          roughness={0.1}        // Polished surface
          envMapIntensity={1}    // Reflect environment fully
        />
      </Sphere>

      <mesh position={[0, -1, 0]} receiveShadow rotation={[-Math.PI / 2, 0, 0]}>
        <planeGeometry args={[20, 20]} />
        <meshStandardMaterial color="#1a1a1a" />
      </mesh>
    </Canvas>
  );
}
```

### Custom HDR Environment

```jsx
import { Environment } from '@react-three/drei';

export function ProductWithCustomHDR() {
  return (
    <Canvas camera={{ position: [0, 0, 5] }}>
      {/* Custom HDR file for maximum control */}
      <Environment
        files="/hdri/studio-hdr.hdr"
        intensity={1.2}
        environmentIntensity={1}
        background={false}
      />

      {/* Alternative: Cube map (6 images) */}
      {/* <Environment
        files={[
          '/textures/px.jpg',
          '/textures/nx.jpg',
          '/textures/py.jpg',
          '/textures/ny.jpg',
          '/textures/pz.jpg',
          '/textures/nz.jpg',
        ]}
        path="/textures/cube/"
      /> */}

      {/* Product */}
      <Sphere args={[1, 64, 64]} castShadow receiveShadow>
        <meshStandardMaterial color="#ffffff" metalness={0.6} roughness={0.2} />
      </Sphere>
    </Canvas>
  );
}
```

**Environment Presets Available**:
- `"studio"` - Balanced lighting for product showcase
- `"city"` - Urban environment (bright, direct)
- `"sunset"` - Warm, dramatic lighting
- `"forest"` - Soft, diffuse lighting
- `"night"` - Low-key dramatic effect

**Environment Intensity Tips**:
- 0.8-1.0: Subtle reflections, natural look
- 1.2-1.5: Pronounced reflections, shiny surfaces
- 2.0+: Dramatic, may blow out highlights

---

## 3. Contact Shadows vs Shadow Maps

Different shadow techniques offer trade-offs between performance and visual quality.

### ContactShadows: Fast, Approximated

```jsx
import { Canvas } from '@react-three/fiber';
import { ContactShadows, Sphere } from '@react-three/drei';

export function ProductWithContactShadows() {
  return (
    <Canvas
      camera={{ position: [0, 0, 5] }}
      gl={{ antialias: true }}
    >
      <color attach="background" args={['#1a1a1a']} />

      {/* Key and fill lights (no shadow casting needed) */}
      <directionalLight position={[5, 5, 5]} intensity={1.5} />
      <directionalLight position={[-5, 3, 3]} intensity={0.6} />
      <directionalLight position={[0, 5, -8]} intensity={0.8} />
      <ambientLight intensity={0.3} />

      {/* Product - no need for receiveShadow/castShadow */}
      <Sphere args={[1, 64, 64]}>
        <meshStandardMaterial color="#ffffff" metalness={0.1} roughness={0.2} />
      </Sphere>

      {/* Fast contact shadow approximation */}
      <ContactShadows
        position={[0, -1, 0]}
        opacity={0.8}           // Shadow darkness
        scale={15}              // Spread of shadow
        blur={2}                // Softness
        far={10}                // Shadow distance from object
        resolution={256}        // Quality (lower = faster)
        color="#000000"
        frames={1}              // Render once (static), Infinity for dynamic
      />
    </Canvas>
  );
}
```

**ContactShadows Advantages**:
- 5-10x faster than shadow maps
- No GPU shadow texture overhead
- Perfect for static products
- Looks great for ground shadows

**ContactShadows Disadvantages**:
- Approximation only (not realistic for all angles)
- No self-shadowing of object
- Limited to ground plane shadows
- Single contact point

### Shadow Maps: Realistic, More Expensive

```jsx
import { Canvas } from '@react-three/fiber';
import { Sphere } from '@react-three/drei';

export function ProductWithShadowMaps() {
  return (
    <Canvas
      shadows="variance"        // VSM for softer shadows
      camera={{ position: [0, 0, 5] }}
      gl={{ antialias: true }}
    >
      <color attach="background" args={['#1a1a1a']} />

      {/* Key light with shadow map */}
      <directionalLight
        position={[5, 5, 5]}
        intensity={1.5}
        castShadow
        shadow-mapSize-width={2048}
        shadow-mapSize-height={2048}
        shadow-camera-far={50}
        shadow-camera-near={0.5}
        shadow-camera-left={-10}
        shadow-camera-right={10}
        shadow-camera-top={10}
        shadow-camera-bottom={-10}
        shadow-bias={-0.0001}
        shadow-radius={8}
      />

      {/* Fill and back lights (no shadow) */}
      <directionalLight position={[-5, 3, 3]} intensity={0.6} />
      <directionalLight position={[0, 5, -8]} intensity={0.8} />
      <ambientLight intensity={0.3} />

      {/* Product casts and receives shadows */}
      <Sphere args={[1, 64, 64]} castShadow receiveShadow>
        <meshStandardMaterial color="#ffffff" metalness={0.1} roughness={0.2} />
      </Sphere>

      {/* Ground receives shadows */}
      <mesh
        position={[0, -1, 0]}
        receiveShadow
        rotation={[-Math.PI / 2, 0, 0]}
      >
        <planeGeometry args={[20, 20]} />
        <meshStandardMaterial color="#1a1a1a" />
      </mesh>
    </Canvas>
  );
}
```

**Shadow Map Advantages**:
- Physically accurate shadows
- Self-shadowing of object geometry
- Realistic shadow interaction with all surfaces
- Better for complex product shapes

**Shadow Map Disadvantages**:
- Higher GPU cost
- Requires tuning (bias, map size, camera)
- Potential shadow acne artifacts
- More VRAM usage

**Shadow Comparison**:

| Feature | ContactShadows | Shadow Maps |
|---------|---|---|
| Performance | Excellent | Good |
| Quality | Approximated | Realistic |
| Self-shadows | No | Yes |
| Flexibility | Limited | High |
| Artifacts | Minimal | Possible acne |
| Best for | Static products | Complex shapes |

---

## 4. Soft Shadows Configuration

Soft shadows improve visual quality by simulating light size and diffusion.

### Using SoftShadows Shader

```jsx
import { Canvas } from '@react-three/fiber';
import { SoftShadows, Sphere } from '@react-three/drei';

export function ProductWithSoftShadows() {
  return (
    <Canvas
      shadows
      camera={{ position: [0, 0, 5] }}
      gl={{ antialias: true }}
    >
      {/* Apply soft shadow shader globally */}
      <SoftShadows />

      <color attach="background" args={['#1a1a1a']} />

      <directionalLight
        position={[5, 5, 5]}
        intensity={1.5}
        castShadow
        shadow-mapSize-width={2048}
        shadow-mapSize-height={2048}
        shadow-camera-far={50}
        shadow-camera-near={0.5}
        shadow-camera-left={-10}
        shadow-camera-right={10}
        shadow-camera-top={10}
        shadow-camera-bottom={-10}
        shadow-radius={4}  // Smaller radius since shader handles softness
      />

      <directionalLight position={[-5, 3, 3]} intensity={0.6} />
      <directionalLight position={[0, 5, -8]} intensity={0.8} />
      <ambientLight intensity={0.3} />

      <Sphere args={[1, 64, 64]} castShadow receiveShadow>
        <meshStandardMaterial color="#ffffff" metalness={0.1} roughness={0.2} />
      </Sphere>

      <mesh
        position={[0, -1, 0]}
        receiveShadow
        rotation={[-Math.PI / 2, 0, 0]}
      >
        <planeGeometry args={[20, 20]} />
        <meshStandardMaterial color="#1a1a1a" />
      </mesh>
    </Canvas>
  );
}
```

### Manual Soft Shadow via Light Radius

```jsx
// In directionalLight configuration
shadow-radius={8}           // Blur shadow edges (0-16 typical)
shadow-bias={-0.0001}       // Prevent self-shadowing artifacts
shadow-mapSize-width={2048} // Higher resolution = better detail
```

**Shadow Radius Effects**:
- 0-2: Hard, sharp shadows (unrealistic)
- 3-5: Soft, natural shadows (recommended)
- 6-8: Very soft, diffuse (light looking source)
- 9+: Extremely soft (loss of definition)

### AccumulativeShadows: Maximum Softness

```jsx
import { Canvas } from '@react-three/fiber';
import {
  AccumulativeShadows,
  RandomizedLight,
  Sphere,
} from '@react-three/drei';

export function ProductWithAccumulativeShadows() {
  return (
    <Canvas
      camera={{ position: [0, 0, 5] }}
      gl={{ antialias: true }}
    >
      <color attach="background" args={['#1a1a1a']} />

      {/* Accumulate shadows from multiple randomized lights */}
      <AccumulativeShadows
        temporal         // Accumulate over frames
        frames={100}     // Number of frames to accumulate
        alphaTest={0.85}
        opacity={0.8}
        scale={20}
        blur={2}
      >
        {/* Multiple lights from different angles */}
        <RandomizedLight
          amount={8}
          radius={5}
          intensity={1}
          ambient={0.5}
          position={[5, 5, 5]}
          bias={-0.0001}
          mapSize={512}
          castShadow
        />
      </AccumulativeShadows>

      {/* Key and fill lights for main illumination */}
      <directionalLight position={[5, 5, 5]} intensity={1.5} />
      <directionalLight position={[-5, 3, 3]} intensity={0.6} />
      <directionalLight position={[0, 5, -8]} intensity={0.8} />
      <ambientLight intensity={0.3} />

      <Sphere args={[1, 64, 64]}>
        <meshStandardMaterial color="#ffffff" metalness={0.1} roughness={0.2} />
      </Sphere>
    </Canvas>
  );
}
```

**AccumulativeShadows Best For**:
- Studio photography style renders
- Highest visual quality
- Static or slowly moving objects
- Professional product catalogs

---

## 5. Professional HDR Lighting with Drei

Combining Drei's Environment with Stage for one-line professional setup.

### Stage Component: All-In-One Solution

```jsx
import { Canvas } from '@react-three/fiber';
import { Stage, Sphere } from '@react-three/drei';
import { Suspense } from 'react';

export function ProductWithStage() {
  return (
    <Canvas camera={{ position: [0, 0, 5] }}>
      <Suspense fallback={null}>
        <Stage
          preset="rembrandt"       // Lighting preset
          intensity={1}            // Overall brightness
          environment="studio"     // HDR environment
          shadows="contact"        // Shadow type
          adjustCamera={1.2}       // Auto-zoom to fit
          center={{ disableY: true }} // Disable Y centering
        >
          <Sphere args={[1, 64, 64]}>
            <meshStandardMaterial color="#ffffff" metalness={0.1} roughness={0.2} />
          </Sphere>
        </Stage>
      </Suspense>
    </Canvas>
  );
}
```

**Stage Presets**:
- `"rembrandt"` - Classic portrait lighting (recommended for products)
- `"portrait"` - Softer, flattering lighting
- `"upfront"` - Frontal, even lighting
- `"soft"` - Very diffuse, minimal shadows

### Advanced Stage with Custom Shadows

```jsx
import { Canvas } from '@react-three/fiber';
import { Stage, Sphere } from '@react-three/drei';

export function ProductWithCustomStage() {
  return (
    <Canvas camera={{ position: [0, 0, 5] }} shadows>
      <Stage
        preset="rembrandt"
        intensity={1.2}
        environment={{
          preset: 'city',
          backgroundBlurriness: 0.3,
          backgroundIntensity: 0.8,
          environmentIntensity: 1.5,
        }}
        shadows={{
          type: 'accumulative',
          frames: 60,
          color: '#000000',
          colorBlend: 1.5,
          opacity: 0.9,
          scale: 15,
          blur: 2,
        }}
        adjustCamera={1.5}
      >
        <Sphere args={[1, 64, 64]} castShadow receiveShadow>
          <meshStandardMaterial
            color="#ffffff"
            metalness={0.15}
            roughness={0.15}
            envMapIntensity={1}
          />
        </Sphere>
      </Stage>
    </Canvas>
  );
}
```

---

## Complete Production Configuration

### Best Practice: White Product on Dark Background

This is the optimal configuration for professional product visualization:

```jsx
import { Canvas } from '@react-three/fiber';
import {
  Environment,
  ContactShadows,
  Sphere,
} from '@react-three/drei';

export function ProfessionalProductLighting() {
  return (
    <Canvas
      camera={{
        position: [0, 0.5, 5],
        fov: 50,
        near: 0.1,
        far: 1000,
      }}
      gl={{
        antialias: true,
        pixelRatio: Math.min(window.devicePixelRatio, 2),
      }}
    >
      {/* Dark background for contrast */}
      <color attach="background" args={['#0a0a0a']} />

      {/* HDR Environment for reflections */}
      <Environment
        preset="studio"
        environmentIntensity={1.2}
        background={false}
      />

      {/* Key Light: Primary modeling light */}
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
      <directionalLight
        position={[-4, 4, 4]}
        intensity={0.5}
        castShadow={false}
      />

      {/* Back Light: Rim lighting for separation */}
      <directionalLight
        position={[0, 6, -10]}
        intensity={0.9}
        castShadow={false}
      />

      {/* Ambient: Global illumination */}
      <ambientLight intensity={0.25} />

      {/* Product: White material with subtle metalness */}
      <Sphere args={[1.2, 128, 128]} castShadow receiveShadow position={[0, 0.5, 0]}>
        <meshStandardMaterial
          color="#ffffff"
          metalness={0.08}
          roughness={0.18}
          envMapIntensity={0.8}
        />
      </Sphere>

      {/* Contact Shadow: Fast, effective ground shadow */}
      <ContactShadows
        position={[0, 0, 0]}
        opacity={0.7}
        scale={20}
        blur={2.5}
        far={12}
        resolution={512}
        color="#000000"
        frames={1}
      />
    </Canvas>
  );
}
```

**Configuration Rationale**:

1. **Key Light** (6, 8, 5):
   - 45° angle from front and top
   - High intensity (1.8) defines form
   - 2048 shadow maps for detail
   - Radius: 6 for natural softness

2. **Fill Light** (-4, 4, 4):
   - Opposite side, slightly lower
   - 0.5 intensity (28% of key) preserves shadows
   - No shadow casting for performance

3. **Back Light** (0, 6, -10):
   - Behind object for rim effect
   - 0.9 intensity for strong separation
   - Creates halo on white product

4. **Ambient Light**:
   - 0.25 intensity provides base illumination
   - Prevents complete black shadows

5. **Environment**:
   - "studio" preset balances neutrality and interest
   - 1.2 intensity for visible but not dominant reflections

6. **Material (White Product)**:
   - Color: Pure white (#ffffff)
   - Metalness: 0.08 (very subtle)
   - Roughness: 0.18 (slightly polished, realistic)
   - envMapIntensity: 0.8 (shows environment subtly)

7. **ContactShadows**:
   - Fast rendering (frames: 1)
   - 512 resolution balances quality and performance
   - 0.7 opacity visible but not overwhelming
   - Blur: 2.5 for natural softness

---

## Performance Optimization Tips

### For Fast Rendering (Mobile):

```jsx
// Use ContactShadows instead of shadow maps
<ContactShadows frames={1} resolution={256} />

// Lower environment intensity
<Environment environmentIntensity={0.8} />

// Reduce geometry complexity
<Sphere args={[1, 32, 32]} /> // Instead of [1, 128, 128]

// Optimize pixel ratio
gl={{ pixelRatio: 1 }} // Instead of 2
```

### For Maximum Quality (Desktop):

```jsx
// Use shadow maps with 4K resolution
shadow-mapSize-width={4096}
shadow-mapSize-height={4096}

// Use AccumulativeShadows
<AccumulativeShadows frames={200} />

// Higher environment intensity
<Environment environmentIntensity={1.5} />

// Full geometry detail
<Sphere args={[1, 256, 256]} />
```

---

## Key Takeaways

1. **Three-point lighting** is foundation: key (45°), fill (opposite), back (rim)
2. **Environment maps** add realism with minimal effort via Drei presets
3. **ContactShadows** = fast & good; **ShadowMaps** = realistic & expensive
4. **Soft shadows** essential for products: use radius 4-6 or SoftShadows component
5. **White products** benefit from: dark background, subtle metalness, environment reflections
6. **Stage component** provides one-line solution for professional results

## Resources

- **Drei Documentation**: https://context7.com/pmndrs/drei
- **Three.js Shadow Maps**: https://context7.com/mrdoob/three.js
- **Product Lighting Reference**: Three.js examples with shadow configuration

---

## Testing Checklist

- [ ] Product clearly visible against dark background
- [ ] Shadow is soft but defined (not harsh, not nonexistent)
- [ ] Environment reflections visible on metallic surfaces
- [ ] Back light creates visible rim on product edges
- [ ] Fill light prevents dark crushed shadows
- [ ] Performance target: 60 FPS on target devices
- [ ] No shadow acne or z-fighting artifacts
- [ ] Material whiteness preserved (not washed out)
