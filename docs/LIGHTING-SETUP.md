# Professional 3D Product Lighting in React Three Fiber

**Status**: Complete Research & Implementation
**Date**: 2026-02-10
**Framework**: React Three Fiber (@react-three/fiber) + Drei (@react-three/drei)

## Quick Start: Best Lighting Configuration

For white products on dark background (recommended):

```jsx
import { Canvas } from '@react-three/fiber';
import { Environment, ContactShadows, Sphere, OrbitControls } from '@react-three/drei';

export function ProductViewer() {
  return (
    <Canvas camera={{ position: [0, 0.5, 5], fov: 50 }}>
      <color attach="background" args={['#0a0a0a']} />

      {/* HDR Environment for reflections */}
      <Environment preset="studio" environmentIntensity={1.2} background={false} />

      {/* Key Light: 45° angle, primary modeling */}
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

      {/* Fill Light: Opposite side, reveals shadow detail */}
      <directionalLight position={[-4, 4, 4]} intensity={0.5} />

      {/* Back Light: Rim lighting for separation */}
      <directionalLight position={[0, 6, -10]} intensity={0.9} />

      {/* Ambient: Global baseline */}
      <ambientLight intensity={0.25} />

      {/* Product */}
      <Sphere args={[1.2, 128, 128]} castShadow receiveShadow position={[0, 0.5, 0]}>
        <meshStandardMaterial color="#ffffff" metalness={0.08} roughness={0.18} />
      </Sphere>

      {/* Ground Shadow */}
      <ContactShadows position={[0, 0, 0]} opacity={0.7} scale={20} blur={2.5} far={12} resolution={512} />

      <OrbitControls makeDefault />
    </Canvas>
  );
}
```

## Five Core Lighting Techniques

### 1. Three-Point Lighting

Professional studio technique with three distinct lights:

| Light | Role | Position | Intensity |
|-------|------|----------|-----------|
| **Key** | Primary illumination | [6, 8, 5] (45° angle) | 1.8 |
| **Fill** | Soften shadows | [-4, 4, 4] (opposite) | 0.5 |
| **Back** | Rim lighting | [0, 6, -10] (behind) | 0.9 |

**Best for**: All products, creates dimension and separation

### 2. Environment Maps (HDR)

Provides realistic reflections and ambient lighting via HDRI images.

```jsx
// Preset environments (simplest)
<Environment preset="studio" environmentIntensity={1.2} background={false} />

// Custom HDR file
<Environment files="/hdri/studio.hdr" environmentIntensity={1.2} />

// Available presets: studio, city, sunset, forest, night, dawn, apartment, lobby, park, warehouse
```

**Best for**: Reflective surfaces (metallic, glossy), realistic materials

**Intensity guidance**:
- 0.8-1.0: Subtle reflections (natural)
- 1.2-1.5: Pronounced reflections (polished)
- 2.0+: Dramatic reflections (shiny)

### 3. Shadow Techniques: ContactShadows vs Shadow Maps

#### ContactShadows (Fast)

```jsx
<ContactShadows
  position={[0, 0, 0]}
  opacity={0.7}
  scale={20}
  blur={2.5}
  far={12}
  resolution={512}
  frames={1}
/>
```

**Advantages**:
- 5-10x faster than shadow maps
- Perfect for static products
- No shadow artifacts
- Minimal GPU overhead

**Disadvantages**:
- Approximation only
- No self-shadowing
- Limited to ground shadows

#### Shadow Maps (Realistic)

```jsx
<directionalLight
  position={[5, 5, 5]}
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
```

**Advantages**:
- Physically accurate
- Self-shadowing of object
- Works on all surfaces

**Disadvantages**:
- Higher GPU cost
- Requires tuning
- Potential artifacts

**Recommendation**: Use ContactShadows for simplicity, shadow maps for complex geometry

### 4. Soft Shadows Configuration

#### Method 1: Light Radius (Simplest)

```jsx
<directionalLight
  shadow-radius={6}  // 0-16: higher = softer
/>
```

Values:
- 0-2: Hard, unrealistic
- 3-5: Natural (recommended)
- 6-8: Very soft
- 9+: Extremely soft

#### Method 2: SoftShadows Shader

```jsx
import { SoftShadows } from '@react-three/drei';

<Canvas shadows>
  <SoftShadows />
  {/* Rest of scene */}
</Canvas>
```

#### Method 3: AccumulativeShadows (Maximum Quality)

```jsx
import { AccumulativeShadows, RandomizedLight } from '@react-three/drei';

<AccumulativeShadows frames={100} alphaTest={0.85} opacity={0.8}>
  <RandomizedLight
    amount={8}
    radius={5}
    intensity={1}
    position={[5, 5, 5]}
  />
</AccumulativeShadows>
```

### 5. HDR Lighting with Drei

#### Stage Component (One-Line Solution)

```jsx
import { Stage } from '@react-three/drei';

<Canvas>
  <Stage
    preset="rembrandt"              // Lighting preset
    intensity={1}
    environment="studio"            // HDR environment
    shadows="contact"               // Shadow type
    adjustCamera={1.2}
  >
    <Model />
  </Stage>
</Canvas>
```

**Stage Presets**:
- `"rembrandt"` - Classic portrait lighting (recommended)
- `"portrait"` - Softer, flattering
- `"upfront"` - Frontal, even
- `"soft"` - Diffuse, minimal shadows

**Environment Presets**:
- `"studio"` - Balanced (neutral)
- `"city"` - Bright, direct
- `"sunset"` - Warm, dramatic
- `"forest"` - Soft, diffuse

#### Advanced Stage Configuration

```jsx
<Stage
  preset="rembrandt"
  intensity={1.2}
  environment={{
    preset: 'studio',
    backgroundIntensity: 0.8,
    environmentIntensity: 1.5,
  }}
  shadows={{
    type: 'accumulative',
    frames: 60,
    color: '#000000',
    opacity: 0.9,
    scale: 15,
    blur: 2,
  }}
  adjustCamera={1.5}
>
  <Model />
</Stage>
```

## Material Settings Reference

### White Products

**Glossy White**:
```jsx
<meshStandardMaterial
  color="#ffffff"
  metalness={0.08}
  roughness={0.18}
  envMapIntensity={0.9}
/>
```

**Matte White**:
```jsx
<meshStandardMaterial
  color="#ffffff"
  metalness={0.05}
  roughness={0.45}
  envMapIntensity={0.6}
/>
```

**Metallic Silver**:
```jsx
<meshStandardMaterial
  color="#e8e8e8"
  metalness={0.9}
  roughness={0.1}
  envMapIntensity={1.0}
/>
```

**Pearl White**:
```jsx
<meshStandardMaterial
  color="#f5f5f5"
  metalness={0.6}
  roughness={0.2}
  envMapIntensity={0.8}
/>
```

**Ceramic White**:
```jsx
<meshStandardMaterial
  color="#fafafa"
  metalness={0.0}
  roughness={0.5}
  envMapIntensity={0.4}
/>
```

## Performance Optimization

### Mobile (Target: 60 FPS)

```jsx
<Canvas gl={{ antialias: true, pixelRatio: 1 }}>
  {/* Use ContactShadows, not shadow maps */}
  <ContactShadows frames={1} resolution={256} />

  {/* Lower geometry detail */}
  <Sphere args={[1, 32, 32]} /> {/* vs 128 segments */}

  {/* Reduce environment intensity */}
  <Environment environmentIntensity={0.8} />
</Canvas>
```

### Desktop (Maximum Quality)

```jsx
<Canvas gl={{ antialias: true, pixelRatio: 2 }}>
  {/* Use shadow maps with 4K resolution */}
  <directionalLight
    shadow-mapSize-width={4096}
    shadow-mapSize-height={4096}
  />

  {/* High geometry detail */}
  <Sphere args={[1, 256, 256]} />

  {/* Boost environment */}
  <Environment environmentIntensity={1.5} />

  {/* AccumulativeShadows for quality */}
  <AccumulativeShadows frames={200} />
</Canvas>
```

## Complete Component Library

Three pre-built components available in `/apps/studio/src/components/lighting/ProductLightingSetup.tsx`:

### 1. ProfessionalProductLighting

Full three-point lighting with all features:
```jsx
<ProfessionalProductLighting
  environmentPreset="studio"
  shadowOpacity={0.7}
  enableControls={true}
>
  <Model />
</ProfessionalProductLighting>
```

### 2. MinimalProductLighting

Lightweight for performance-critical scenarios:
```jsx
<MinimalProductLighting>
  <Model />
</MinimalProductLighting>
```

### 3. StudioProductLighting

Enhanced quality for photography:
```jsx
<StudioProductLighting>
  <Model />
</StudioProductLighting>
```

### 4. WhiteProductShowcase

Optimized specifically for white products:
```jsx
<WhiteProductShowcase environmentPreset="studio">
  <Model />
</WhiteProductShowcase>
```

## Shadow Map Tuning Guide

If shadows look wrong, adjust these parameters:

| Issue | Solution |
|-------|----------|
| **Shadow acne** (artifacts) | Increase `shadow-bias` to -0.0001 or lower |
| **Shadow detaching** | Decrease `shadow-bias` toward 0 |
| **Pixelated shadows** | Increase `shadow-mapSize-width/height` to 2048 or 4096 |
| **Jagged edges** | Increase `shadow-radius` to 6-8 |
| **Shadows too dark** | Increase `ambientLight` intensity |
| **No shadows visible** | Check `castShadow={true}` on light and objects |
| **Camera frustum too small** | Increase `shadow-camera-left/right/top/bottom` |
| **Shadows cut off** | Increase `shadow-camera-far` value |

## Light Intensity Reference

| Scenario | Key | Fill | Back | Ambient |
|----------|-----|------|------|---------|
| Studio (recommended) | 1.8 | 0.5 | 0.9 | 0.25 |
| Dramatic | 2.0 | 0.3 | 1.2 | 0.15 |
| Soft | 1.4 | 0.7 | 0.7 | 0.4 |
| Minimal | 1.2 | 0.4 | 0.5 | 0.2 |
| Jewelry | 2.2 | 0.35 | 1.2 | 0.4 |

## Key Takeaways

1. **Start with Stage component** for quick professional results
2. **Use three-point lighting** for all products (key 45°, fill opposite, back rim)
3. **Choose ContactShadows** for performance, shadow maps for complexity
4. **Apply environment** for reflections on metallic/glossy surfaces
5. **Tune shadow radius** (4-8) for natural softness
6. **White products need**: dark background, subtle metalness (0.08), environment reflections
7. **Mobile optimization**: pixel ratio 1, lower resolution, ContactShadows only
8. **Always test on target devices** - lighting performance varies

## Files Generated

1. **docs/lighting-research.md** - Comprehensive research document
2. **apps/studio/src/components/lighting/ProductLightingSetup.tsx** - Four reusable components
3. **apps/studio/src/components/lighting/LightingExamples.tsx** - Eight specialized examples
4. **docs/LIGHTING-SETUP.md** - This quick reference (you are here)

## Documentation Links

- Drei Docs: https://context7.com/pmndrs/drei
- Three.js Shadow Maps: https://context7.com/mrdoob/three.js
- React Three Fiber: https://context7.com/pmndrs/react-three-fiber

## Next Steps

1. Copy one of the components from ProductLightingSetup.tsx
2. Adjust camera position for your product size
3. Fine-tune shadow radius (4-8) based on visual preference
4. Test on mobile with pixelRatio={1}
5. Benchmark performance with Chrome DevTools

---

**Version**: 1.0
**Last Updated**: 2026-02-10
**Status**: Production Ready
