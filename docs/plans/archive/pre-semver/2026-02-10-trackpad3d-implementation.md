# Trackpad3D Implementation Plan

## Research Summary

Based on comprehensive research from 5 haiku agents + Context7 + Perplexity:

### Key Material Settings (meshPhysicalMaterial for white frosted plastic)
```typescript
{
  color: '#f5f5f5',          // Off-white (not pure white)
  metalness: 0.0,            // Non-metallic plastic
  roughness: 0.35,           // Frosted finish (0.3-0.4 sweet spot)
  clearcoat: 0.8,            // Protective glossy layer
  clearcoatRoughness: 0.2,   // Satin finish
  transmission: 0.1,         // Subtle translucency
  thickness: 0.5,            // Subsurface scattering depth
  ior: 1.49,                 // Plastic refractive index
}
```

### Lighting Setup (Three-point + Environment)
- Key light: [5, 15, 8], intensity=1.4
- Fill light: [-5, 6, -5], intensity=0.4
- Back light: [5, 5, 5], intensity=0.35
- Ambient: intensity=0.6
- Environment: "studio" preset

### Geometry
- RoundedBox for main chassis
- Side groove using displacement or separate mesh
- Black encoder knob with knurling pattern

## Component Structure

```
Trackpad3D.tsx
├── TrackpadChassis (RoundedBox, white frosted)
│   ├── TouchSurface (top glass layer)
│   └── SideGrooves (left edge detail)
├── BlackEncoder (top-right corner)
├── Background (dark grid, same as macropad)
└── Lighting (Stage or manual 3-point)
```

## Implementation Tasks

1. Create `Trackpad3D.tsx` component file
2. Implement TrackpadChassis with RoundedBox
3. Add side groove geometry
4. Implement BlackEncoder (reuse from SuperNovaePad3D)
5. Reuse Background from SuperNovaePad3D
6. Add proper lighting with Stage or manual setup
7. Add click/hover interactions for future touch zones

## File Location
`/apps/studio/src/components/macropad/Trackpad3D.tsx`

## Dependencies
- @react-three/fiber
- @react-three/drei (RoundedBox, Stage, Environment)
- three (meshPhysicalMaterial)
