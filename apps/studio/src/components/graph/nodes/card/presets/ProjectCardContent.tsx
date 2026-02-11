'use client';

/**
 * ProjectCardContent - Content preset for project nodes
 *
 * Used by: ProjectNode
 *
 * Layout:
 * ┌──────────────────────────────────┐
 * │ [Logo Image]        [Type Badge] │
 * │   with ring          • Project   │
 * │                                  │
 * │ Display Name (large)             │
 * │ key (optional)                   │
 * └──────────────────────────────────┘
 */

import { memo, useState, useMemo } from 'react';
import { cn } from '@/lib/utils';
import Image from 'next/image';
import { Briefcase } from 'lucide-react';
import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../CardShell';

// NovaNet logo URL
const NOVANET_LOGO_URL = 'https://pbs.twimg.com/profile_images/1788187862883598336/q8u1VSz3_400x400.jpg';

// =============================================================================
// Types
// =============================================================================

export interface ProjectNodeData {
  id: string;
  type: string;
  key: string;
  displayName: string;
  logoUrl?: string;
}

export interface ProjectCardContentProps extends CardContext {
  data: ProjectNodeData;
}

// =============================================================================
// Component
// =============================================================================

export const ProjectCardContent = memo(function ProjectCardContent({
  data,
  colors,
  selected,
}: ProjectCardContentProps) {
  const [imageError, setImageError] = useState(false);
  const logoUrl = data.logoUrl || NOVANET_LOGO_URL;

  // Memoize ring style
  const ringStyle = useMemo(() => ({
    boxShadow: selected ? `0 0 15px ${colors.primary}40` : undefined,
  }), [colors.primary, selected]);

  return (
    <div className="px-5 py-5">
      {/* Grid background pattern (Project-specific decoration) */}
      <div
        className={cn(
          'absolute inset-0 pointer-events-none',
          selected ? 'opacity-[0.05]' : 'opacity-[0.03]'
        )}
        style={{
          backgroundImage: `
            linear-gradient(rgba(139, 92, 246, 0.5) 1px, transparent 1px),
            linear-gradient(90deg, rgba(139, 92, 246, 0.5) 1px, transparent 1px)
          `,
          backgroundSize: '20px 20px',
        }}
      />

      {/* Row 1: Logo + Badge */}
      <div className="relative flex items-center justify-between mb-4">
        {/* Logo */}
        <div
          className={cn(
            'w-14 h-14 rounded-xl overflow-hidden ring-2 transition duration-200',
            selected ? 'ring-white/40' : 'ring-white/20'
          )}
          style={ringStyle}
        >
          {!imageError ? (
            <Image
              src={logoUrl}
              alt="Project"
              width={56}
              height={56}
              className="object-cover w-full h-full"
              unoptimized
              onError={() => setImageError(true)}
            />
          ) : (
            <div
              className="w-full h-full flex items-center justify-center"
              style={{ background: `${colors.primary}20` }}
            >
              <Briefcase size={28} style={{ color: colors.primary }} />
            </div>
          )}
        </div>

        {/* Type Badge */}
        <div
          className={cn('flex items-center px-3 py-1.5 rounded-full border', gapTokens.default)}
          style={{
            background: `${colors.primary}15`,
            borderColor: `${colors.primary}40`,
          }}
        >
          <span
            className={cn('w-2 h-2 rounded-full', selected && 'animate-pulse')}
            style={{
              background: colors.primary,
              boxShadow: `0 0 8px ${colors.primary}`,
            }}
          />
          <span
            className="text-xs font-bold uppercase tracking-wider"
            style={{ color: colors.secondary }}
          >
            Project
          </span>
        </div>
      </div>

      {/* Row 2: Name + Key */}
      <h2 className="relative text-xl font-extrabold text-white truncate leading-tight">
        {data.displayName}
      </h2>

      {data.key && data.key !== data.displayName && (
        <p
          className="relative font-mono text-sm mt-1 truncate"
          style={{ color: `${colors.primary}70` }}
        >
          {data.key}
        </p>
      )}
    </div>
  );
});
