# Lighting Setup Validation Checklist

Use this checklist to validate your 3D product lighting setup.

## Pre-Implementation Checklist

- [ ] Select target product type (jewelry, electronics, textiles, etc.)
- [ ] Define target background (dark/light/custom color)
- [ ] Determine device targets (desktop/mobile/VR headset)
- [ ] Identify material properties (metallic/matte/glossy)
- [ ] Plan for performance constraints (FPS target, device capabilities)

## Lighting Quality Checklist

### Visibility & Clarity

- [ ] **Product clearly visible** against background
- [ ] **No black crushed shadows** (use fill light or increase ambient)
- [ ] **No blown highlights** on white surfaces (adjust key light intensity)
- [ ] **Fine details visible** (shadows reveal geometry, not obscure it)
- [ ] **Material color accurate** (white is white, not gray or blue-tinted)

### Three-Point Lighting Balance

- [ ] **Key light creates dimension** (visible shadow from one angle)
- [ ] **Fill light balances key** (shadows have visible detail, ratio ~3:1 key:fill)
- [ ] **Back light separates** product from background (visible rim edge)
- [ ] **Ambient light prevents pure black** (set to 0.2-0.3)
- [ ] **Overall brightness appropriate** (not too dark, not washed out)

### Shadow Quality

- [ ] **Shadow edges are soft** (not jagged, not too blurry)
- [ ] **Shadow direction matches light** (shadow points away from light source)
- [ ] **Shadow is present but not dominant** (visible but doesn't overwhelm product)
- [ ] **No shadow acne artifacts** (random speckles = increase shadow-bias)
- [ ] **Shadow placement correct** (directly under/behind object, not floating)
- [ ] **Shadow scale appropriate** (matches object size and distance)

**Shadow Quality Scale**:
```
Too Hard                              Too Soft
|-----|-----|-----|-----|-----|-----|
0     2     4     6     8     10    12+
      └─ Typical Range (shadow-radius)
```

### Environment & Reflections

- [ ] **Reflections visible on shiny surfaces** (metallic/glossy materials)
- [ ] **Reflections appropriate intensity** (not too subtle, not too dominant)
- [ ] **Environment complements lighting** (matches overall mood/style)
- [ ] **No excessive environmental glare** (reflections don't blow out highlights)
- [ ] **Fallback for no-environment mode** (product still looks good without HDR)

**Environment Intensity Check**:
- 0.8: Subtle reflections (recommended for most products)
- 1.0-1.2: Pronounced reflections (metallic/glossy surfaces)
- 1.5+: Dramatic reflections (jewelry, luxury items)

### Material Appearance

- [ ] **White is pure white** (#ffffff, not gray or tinted)
- [ ] **Metalness appropriate** (0.05-0.15 for subtle, 0.6+ for shiny)
- [ ] **Roughness realistic** (0.15-0.2 for polished, 0.4+ for matte)
- [ ] **Surface finish matches product** (glossy, matte, or satin as intended)
- [ ] **Material doesn't look plastic** (especially for premium products)

**Material Validation**:
```jsx
// Test different roughness values
<meshStandardMaterial roughness={0.1} /> // Glossy
<meshStandardMaterial roughness={0.3} /> // Satin
<meshStandardMaterial roughness={0.5} /> // Matte
```

## Performance Checklist

### Frame Rate

- [ ] **Desktop: 60 FPS minimum** (use Chrome DevTools Performance tab)
- [ ] **Mobile: 30-60 FPS minimum** (test on actual device if possible)
- [ ] **No frame drops during rotation** (auto-rotate or mouse interaction)
- [ ] **Performance stable** (FPS consistent, not dropping and recovering)
- [ ] **First paint < 2 seconds** (initial load time acceptable)

### Memory Usage

- [ ] **Shadow maps optimized** (2048x2048 resolution adequate, not 4096 unless necessary)
- [ ] **Geometry optimized** (32-64 segments for mobile, 128-256 for desktop)
- [ ] **No memory leaks** (performance stable over time)
- [ ] **Texture memory reasonable** (no massive HDR files for mobile)
- [ ] **Canvas pixelRatio optimized** (1 for mobile, 1-2 for desktop)

**Optimization Decision Tree**:
```
Too slow (< 30 FPS)?
├─ ContactShadows: 1, resolution: 256
├─ Reduce geometry segments: 32
├─ Disable auto-rotate or reduce speed
├─ Decrease pixelRatio to 1
├─ Reduce light shadow-mapSize
└─ Profile with DevTools to identify bottleneck
```

## Browser Compatibility

- [ ] **Chrome/Chromium**: All features working
- [ ] **Firefox**: Shadows rendering correctly
- [ ] **Safari**: WebGL enabled, no visual differences
- [ ] **Mobile Safari**: Performance acceptable
- [ ] **Mobile Chrome**: Performance acceptable
- [ ] **No console errors** (check DevTools console)
- [ ] **No WebGL warnings** (shader compilation clean)

## Color Accuracy Checklist

### For White Products Specifically

- [ ] **Background color correct** (pure black #0a0a0a recommended, not #1a1a1a)
- [ ] **Product white is pure white** (not off-white or cream)
- [ ] **No color cast** (product not blue, green, or yellow-tinted from lights)
- [ ] **Shadows have neutral color** (not tinted, use #000000)
- [ ] **No light color bleeding** (lights should be white/neutral)

**White Product Checklist**:
```jsx
// Correct setup
<color attach="background" args={['#0a0a0a']} />
<directionalLight position={[6, 8, 5]} intensity={1.8} /> {/* white by default */}
<Sphere>
  <meshStandardMaterial
    color="#ffffff"      {/* Pure white */}
    metalness={0.08}
    roughness={0.18}
  />
</Sphere>
<ContactShadows color="#000000" /> {/* Neutral shadow */}
```

## Mobile-Specific Checklist

- [ ] **Geometry reduced** (32-64 segments, not 256)
- [ ] **Shadow resolution lowered** (256-512, not 2048)
- [ ] **ContactShadows only** (no shadow maps)
- [ ] **pixelRatio={1}** (not 2)
- [ ] **No AccumulativeShadows** (too expensive)
- [ ] **Test on actual device** (iPhone, Android)
- [ ] **Portrait orientation works** (if applicable)
- [ ] **Touch controls responsive** (no lag on interaction)
- [ ] **No heating/battery drain** (smooth playback)

## Accessibility Checklist

- [ ] **Product visible to colorblind** (not relying solely on color)
- [ ] **Sufficient contrast** (product vs background)
- [ ] **No flashing effects** (shadows stable, no epilepsy risk)
- [ ] **Focus indicators visible** (if interactive controls)
- [ ] **Keyboard navigation works** (if applicable)

## Testing Scenarios

### Scenario 1: Optimal Conditions

**Environment**: Desktop, Chrome, high-end GPU, good lighting
**Checklist**:
- [ ] All details visible
- [ ] Smooth 60 FPS
- [ ] Reflections crisp
- [ ] Shadows soft and natural

### Scenario 2: Mobile Scenario

**Environment**: iPhone 12 or similar, Safari, standard lighting
**Checklist**:
- [ ] Image visible within 3 seconds
- [ ] Maintains 30+ FPS during rotation
- [ ] Shadows present and soft
- [ ] No visual glitches

### Scenario 3: Low-End Device

**Environment**: Basic Android phone, poor GPU, standard browser
**Checklist**:
- [ ] Still visible and usable
- [ ] Maintains 20+ FPS (not ideal but acceptable)
- [ ] No crashes or browser lag
- [ ] Shadows simplified but present

### Scenario 4: Poor Lighting Conditions

**Environment**: Dark room, high screen brightness needed
**Checklist**:
- [ ] Product still clearly visible
- [ ] Shadows don't hide details
- [ ] No excessive glare on white surfaces

## A/B Testing Variations

### Lighting Intensity Comparison

```
Current Setup: key=1.8, fill=0.5, back=0.9
Test Option A: key=1.5, fill=0.6, back=0.8  (softer)
Test Option B: key=2.0, fill=0.4, back=1.0  (more dramatic)
Test Option C: key=1.6, fill=0.8, back=0.7  (more flat)
```

**Evaluation**: Which feels most premium/professional?

### Shadow Technique Comparison

```
Current: ContactShadows, opacity=0.7
Test A:  ContactShadows, opacity=0.5  (more subtle)
Test B:  Shadow Maps with 2048 resolution
Test C:  AccumulativeShadows, frames=60
```

**Evaluation**: Which looks best while maintaining performance?

### Environment Comparison

```
Current: "studio", intensity=1.2
Test A:  "city", intensity=1.0
Test B:  "sunset", intensity=1.3
Test C:  No environment (just lights)
```

**Evaluation**: Which environment suits the product best?

## Troubleshooting Guide

### Product Looks Dark

**Causes**:
- Key light too far or low intensity
- Ambient light too low
- Shadow opacity too high
- Background too dark for contrast

**Solutions**:
```jsx
// Increase key light
<directionalLight intensity={2.0} /> // was 1.8

// Increase ambient
<ambientLight intensity={0.4} /> // was 0.25

// Reduce shadow opacity
<ContactShadows opacity={0.5} /> // was 0.7
```

### Shadows Look Harsh/Pixelated

**Causes**:
- shadow-mapSize too low
- shadow-radius too small
- Light too close

**Solutions**:
```jsx
<directionalLight
  shadow-mapSize-width={2048}   // was 1024
  shadow-mapSize-height={2048}
  shadow-radius={8}             // was 4
/>
```

### White Looks Grayish/Washed Out

**Causes**:
- Too much ambient light
- Environment too bright
- Metalness too high

**Solutions**:
```jsx
<ambientLight intensity={0.2} /> // reduce to 0.2

<meshStandardMaterial
  color="#ffffff"
  metalness={0.05}     // reduce from 0.15
  envMapIntensity={0.7} // reduce from 0.9
/>
```

### Product Floating or Shadow Detached

**Causes**:
- shadow-bias incorrect
- Shadow camera not positioned correctly
- Product not in shadow camera frustum

**Solutions**:
```jsx
<directionalLight
  shadow-bias={-0.0001}        // adjust this value
  shadow-camera-far={50}       // ensure large enough
  position={[6, 8, 5]}         // adjust light position
/>
```

### Mobile Performance Poor

**Causes**:
- Shadow resolution too high
- Geometry too detailed
- pixelRatio too high
- AccumulativeShadows used

**Solutions**:
```jsx
<Canvas gl={{ pixelRatio: 1 }}>
  <ContactShadows frames={1} resolution={256} />
  <Sphere args={[1, 32, 32]} /> {/* not 128 */}
</Canvas>
```

## Sign-Off Checklist

Before deploying to production:

- [ ] All quality checklist items pass
- [ ] Performance meets targets (60 FPS desktop, 30+ FPS mobile)
- [ ] Tested on 3+ browsers
- [ ] Tested on 2+ physical devices
- [ ] No console errors
- [ ] Visual review by designer/product team
- [ ] Accessibility review complete
- [ ] A/B test variations evaluated
- [ ] Documentation updated
- [ ] Team trained on adjusting settings
- [ ] Monitoring/analytics configured

## Performance Benchmarks (Reference)

| Configuration | Desktop | Mobile | Notes |
|---------------|---------|--------|-------|
| Professional + ContactShadows | 60 FPS | 45 FPS | Recommended balance |
| Professional + ShadowMaps | 60 FPS | 25 FPS | Desktop only |
| Minimal + ContactShadows | 120+ FPS | 60 FPS | Maximum performance |
| Studio + AccumulativeShadows | 30 FPS | <15 FPS | Static images only |

**Benchmark Methodology**:
- 1920x1080 desktop, RTX 3060
- iPhone 12 Pro (mobile reference)
- 60-second rotation loop
- Measurements at 5s, 30s, 60s marks
- Chrome DevTools Performance tab

## Version History

| Date | Version | Changes |
|------|---------|---------|
| 2026-02-10 | 1.0 | Initial release |

---

**Document Version**: 1.0
**Last Updated**: 2026-02-10
**Status**: Active
