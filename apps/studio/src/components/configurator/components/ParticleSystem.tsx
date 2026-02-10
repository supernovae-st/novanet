'use client';

/**
 * ParticleSystem - GPU-accelerated particle effects for key presses
 *
 * Features:
 * - Burst particles on key press
 * - Gravity and fade out
 * - Additive blending for glow
 * - Color based on key layer
 */

import { useRef, useMemo } from 'react';
import { useFrame } from '@react-three/fiber';
import * as THREE from 'three';

export interface Particle {
  id: number;
  position: THREE.Vector3;
  velocity: THREE.Vector3;
  life: number;
  maxLife: number;
  color: THREE.Color;
  size: number;
}

interface ParticleSystemProps {
  particles: Particle[];
}

const MAX_PARTICLES = 500;

export function ParticleSystem({ particles }: ParticleSystemProps) {
  const pointsRef = useRef<THREE.Points>(null);

  // Pre-allocate buffer geometry
  const { geometry, positionAttr, colorAttr, sizeAttr } = useMemo(() => {
    const geo = new THREE.BufferGeometry();
    const positions = new Float32Array(MAX_PARTICLES * 3);
    const colors = new Float32Array(MAX_PARTICLES * 3);
    const sizes = new Float32Array(MAX_PARTICLES);

    const posAttr = new THREE.BufferAttribute(positions, 3);
    const colAttr = new THREE.BufferAttribute(colors, 3);
    const sizeA = new THREE.BufferAttribute(sizes, 1);

    geo.setAttribute('position', posAttr);
    geo.setAttribute('color', colAttr);
    geo.setAttribute('size', sizeA);

    return { geometry: geo, positionAttr: posAttr, colorAttr: colAttr, sizeAttr: sizeA };
  }, []);

  useFrame(() => {
    if (!pointsRef.current) return;

    const positions = positionAttr.array as Float32Array;
    const colors = colorAttr.array as Float32Array;
    const sizes = sizeAttr.array as Float32Array;

    // Reset all to invisible
    for (let i = 0; i < MAX_PARTICLES; i++) {
      positions[i * 3] = 0;
      positions[i * 3 + 1] = -100; // Hide off-screen
      positions[i * 3 + 2] = 0;
      colors[i * 3] = 0;
      colors[i * 3 + 1] = 0;
      colors[i * 3 + 2] = 0;
      sizes[i] = 0;
    }

    // Update visible particles
    particles.forEach((particle, i) => {
      if (i >= MAX_PARTICLES) return;

      const alpha = Math.max(0, particle.life / particle.maxLife);
      const easeAlpha = alpha * alpha; // Ease out

      positions[i * 3] = particle.position.x;
      positions[i * 3 + 1] = particle.position.y;
      positions[i * 3 + 2] = particle.position.z;

      colors[i * 3] = particle.color.r * easeAlpha;
      colors[i * 3 + 1] = particle.color.g * easeAlpha;
      colors[i * 3 + 2] = particle.color.b * easeAlpha;

      sizes[i] = particle.size * easeAlpha;
    });

    positionAttr.needsUpdate = true;
    colorAttr.needsUpdate = true;
    sizeAttr.needsUpdate = true;
  });

  return (
    <points ref={pointsRef} geometry={geometry}>
      <pointsMaterial
        size={0.08}
        vertexColors
        transparent
        opacity={1}
        blending={THREE.AdditiveBlending}
        sizeAttenuation
        depthWrite={false}
      />
    </points>
  );
}

// Helper to create particle burst
let particleIdCounter = 0;

export function createParticleBurst(
  position: [number, number, number],
  color: string,
  count: number = 25
): Particle[] {
  const particles: Particle[] = [];
  const colorObj = new THREE.Color(color);

  for (let i = 0; i < count; i++) {
    const angle = (Math.PI * 2 * i) / count + Math.random() * 0.3;
    const speed = 1.5 + Math.random() * 2.5;
    const elevation = Math.random() * Math.PI * 0.4;

    particles.push({
      id: particleIdCounter++,
      position: new THREE.Vector3(position[0], position[1], position[2]),
      velocity: new THREE.Vector3(
        Math.cos(angle) * Math.cos(elevation) * speed,
        Math.sin(elevation) * speed * 2 + 1.5,
        Math.sin(angle) * Math.cos(elevation) * speed
      ),
      life: 1,
      maxLife: 1,
      color: colorObj,
      size: 0.06 + Math.random() * 0.04,
    });
  }

  return particles;
}

// Update particles with physics
export function updateParticles(
  particles: Particle[],
  delta: number,
  gravity: number = 8
): Particle[] {
  return particles
    .map((p) => {
      // Apply gravity
      p.velocity.y -= gravity * delta;

      // Update position
      p.position.add(p.velocity.clone().multiplyScalar(delta));

      // Decay life
      p.life -= delta * 1.2;

      return p;
    })
    .filter((p) => p.life > 0)
    .slice(-MAX_PARTICLES);
}
