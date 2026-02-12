'use client';

/**
 * Graph3DLegend - Visual encoding legend for 3D graph
 *
 * Shows the mapping between visual properties and graph classification:
 * - Shape → Layer
 * - Material → Trait
 * - Outline → Realm
 * - Particles → ArcFamily
 */

import { memo, useState } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { ChevronDown, ChevronUp, Layers, Circle, Hexagon, ArrowRight } from 'lucide-react';
import { cn } from '@/lib/utils';

// Layer shape descriptions
const LAYER_SHAPES = [
  { layer: 'config', shape: 'Octahedron', color: '#64748b' },
  { layer: 'locale', shape: 'Globe', color: '#64748b' },
  { layer: 'geography', shape: 'Icosahedron', color: '#10b981' },
  { layer: 'knowledge', shape: 'Torus', color: '#8b5cf6' },
  { layer: 'foundation', shape: 'Cube', color: '#3b82f6' },
  { layer: 'structure', shape: 'Pyramid', color: '#06b6d4' },
  { layer: 'semantic', shape: 'Dodecahedron', color: '#f97316' },
  { layer: 'instruction', shape: 'Cone', color: '#eab308' },
  { layer: 'output', shape: 'Glow Sphere', color: '#22c55e' },
];

// Trait materials
const TRAIT_MATERIALS = [
  { trait: 'invariant', style: 'Solid', description: 'Stable, metallic' },
  { trait: 'localized', style: 'Wireframe', description: 'Transparent outline' },
  { trait: 'knowledge', style: 'Glass', description: 'Semi-transparent' },
  { trait: 'generated', style: 'Emissive', description: 'Glowing' },
  { trait: 'aggregated', style: 'Points', description: 'Particle cloud' },
];

// Realm outlines
const REALM_OUTLINES = [
  { realm: 'shared', color: '#2aa198', description: 'Universal knowledge' },
  { realm: 'org', color: '#6c71c4', description: 'Organization-specific' },
];

// Arc families
const ARC_FAMILIES = [
  { family: 'ownership', color: '#3b82f6', particles: 3 },
  { family: 'localization', color: '#22c55e', particles: 5 },
  { family: 'semantic', color: '#f97316', particles: 4 },
  { family: 'generation', color: '#8b5cf6', particles: 6 },
  { family: 'mining', color: '#ec4899', particles: 2 },
];

interface LegendSectionProps {
  title: string;
  icon: React.ReactNode;
  children: React.ReactNode;
  defaultOpen?: boolean;
}

function LegendSection({ title, icon, children, defaultOpen = false }: LegendSectionProps) {
  const [isOpen, setIsOpen] = useState(defaultOpen);

  return (
    <div className="border-b border-white/10 last:border-0">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="w-full flex items-center justify-between px-3 py-2 text-xs font-medium text-white/70 hover:text-white/90 transition-colors"
      >
        <span className="flex items-center gap-2">
          {icon}
          {title}
        </span>
        {isOpen ? <ChevronUp size={12} /> : <ChevronDown size={12} />}
      </button>
      <AnimatePresence>
        {isOpen && (
          <motion.div
            initial={{ height: 0, opacity: 0 }}
            animate={{ height: 'auto', opacity: 1 }}
            exit={{ height: 0, opacity: 0 }}
            transition={{ duration: 0.2 }}
            className="overflow-hidden"
          >
            <div className="px-3 pb-2 space-y-1">
              {children}
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}

export interface Graph3DLegendProps {
  className?: string;
  collapsed?: boolean;
  onToggle?: () => void;
}

export const Graph3DLegend = memo(function Graph3DLegend({
  className,
  collapsed = false,
  onToggle,
}: Graph3DLegendProps) {
  if (collapsed) {
    return (
      <button
        onClick={onToggle}
        className={cn(
          'absolute bottom-4 right-4 p-2 rounded-lg bg-black/60 backdrop-blur-sm',
          'border border-white/10 text-white/50 hover:text-white/80 transition-colors',
          className
        )}
        title="Show legend"
      >
        <Layers size={16} />
      </button>
    );
  }

  return (
    <motion.div
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      className={cn(
        'absolute bottom-4 right-4 w-56 rounded-lg bg-black/80 backdrop-blur-sm',
        'border border-white/10 overflow-hidden',
        className
      )}
    >
      {/* Header */}
      <div className="flex items-center justify-between px-3 py-2 border-b border-white/10">
        <span className="text-xs font-semibold text-white/90">Visual Encoding</span>
        <button
          onClick={onToggle}
          className="text-white/40 hover:text-white/70 transition-colors"
          title="Collapse legend"
        >
          <ChevronDown size={14} />
        </button>
      </div>

      {/* Sections */}
      <div className="max-h-80 overflow-y-auto">
        {/* Layer → Shape */}
        <LegendSection title="Shape → Layer" icon={<Hexagon size={12} />} defaultOpen>
          <div className="grid grid-cols-2 gap-1">
            {LAYER_SHAPES.map(({ layer, shape: _shape, color }) => (
              <div key={layer} className="flex items-center gap-1.5">
                <div
                  className="w-2.5 h-2.5 rounded-sm"
                  style={{ backgroundColor: color }}
                />
                <span className="text-[10px] text-white/60 truncate">{layer}</span>
              </div>
            ))}
          </div>
        </LegendSection>

        {/* Trait → Material */}
        <LegendSection title="Material → Trait" icon={<Circle size={12} />}>
          {TRAIT_MATERIALS.map(({ trait, style }) => (
            <div key={trait} className="flex items-center justify-between">
              <span className="text-[10px] text-white/60">{trait}</span>
              <span className="text-[10px] text-white/40">{style}</span>
            </div>
          ))}
        </LegendSection>

        {/* Realm → Outline */}
        <LegendSection title="Outline → Realm" icon={<Circle size={12} />}>
          {REALM_OUTLINES.map(({ realm, color, description }) => (
            <div key={realm} className="flex items-center gap-2">
              <div
                className="w-3 h-3 rounded-full border-2"
                style={{ borderColor: color }}
              />
              <div>
                <span className="text-[10px] text-white/70">{realm}</span>
                <span className="text-[9px] text-white/40 ml-1">({description})</span>
              </div>
            </div>
          ))}
        </LegendSection>

        {/* ArcFamily → Particles */}
        <LegendSection title="Particles → Arc" icon={<ArrowRight size={12} />}>
          {ARC_FAMILIES.map(({ family, color, particles }) => (
            <div key={family} className="flex items-center justify-between">
              <span className="text-[10px] text-white/60">{family}</span>
              <div className="flex items-center gap-0.5">
                {Array.from({ length: particles }).map((_, i) => (
                  <div
                    key={i}
                    className="w-1 h-1 rounded-full"
                    style={{ backgroundColor: color }}
                  />
                ))}
              </div>
            </div>
          ))}
        </LegendSection>
      </div>

      {/* Footer hint */}
      <div className="px-3 py-1.5 bg-white/5 text-[9px] text-white/40 text-center">
        Drag to rotate · Scroll to zoom · Click to select
      </div>
    </motion.div>
  );
});

export default Graph3DLegend;
