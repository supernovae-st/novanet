'use client';

/**
 * ElementIdentityCard - Unified identity display for nodes and arcs
 *
 * This component provides a consistent visual representation for graph elements,
 * used both in panel headers and overview tabs.
 *
 * Features:
 * - Unified design for nodes and arcs
 * - Type badge with layer/arc colors
 * - Title (displayName for nodes, arc type for arcs)
 * - Key/ID with copy functionality
 * - Optional 3D preview slot
 * - Optional close button (for panel header usage)
 *
 * v11.7 — Design system unification
 */

import { memo, useMemo, type ReactNode } from 'react';
import { Hash, ArrowRight, X } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useCopyFeedback } from '@/hooks';
import { CopyButton } from '@/components/dx/CopyButton';
import { LayerIcon } from '@/components/ui/CategoryIcon';
import { TextScramble } from '@/components/ui/TextScramble';
import { gapTokens } from '@/design/tokens';
import type { Layer } from '@novanet/core/types';

// =============================================================================
// Types
// =============================================================================

interface BaseProps {
  /** Optional CSS class */
  className?: string;
  /** Optional close button handler */
  onClose?: () => void;
  /** Optional 3D preview component */
  preview?: ReactNode;
  /** Variant: 'card' (full with background) or 'header' (compact, no background) */
  variant?: 'card' | 'header';
}

interface NodeIdentityProps extends BaseProps {
  /** Element type: node */
  elementType: 'node';
  /** Layer for icon and colors */
  layer: Layer;
  /** Primary/secondary gradient colors */
  colors: { primary: string; secondary: string };
  /** Display name (title) */
  displayName: string;
  /** Node type label (badge text) */
  typeLabel: string;
  /** Key for copy */
  nodeKey: string;
}

interface ArcIdentityProps extends BaseProps {
  /** Element type: arc */
  elementType: 'arc';
  /** Arc type (e.g., "HAS_PAGE") */
  arcType: string;
  /** Arc ID */
  arcId: string;
  /** Primary/glow colors */
  colors: { primary: string; secondary: string };
}

export type ElementIdentityCardProps = NodeIdentityProps | ArcIdentityProps;

// =============================================================================
// Component
// =============================================================================

export const ElementIdentityCard = memo(function ElementIdentityCard(
  props: ElementIdentityCardProps
) {
  const { copied, copy } = useCopyFeedback();
  const { elementType, colors, className, onClose, preview, variant = 'card' } = props;

  // Determine content based on element type
  const content = useMemo(() => {
    if (elementType === 'node') {
      const { layer, displayName, typeLabel, nodeKey } = props as NodeIdentityProps;
      return {
        icon: (
          <div
            className="flex items-center justify-center w-7 h-7 rounded-lg flex-shrink-0"
            style={{
              background: `linear-gradient(135deg, ${colors.primary}30, ${colors.secondary}20)`,
            }}
          >
            <LayerIcon
              layer={layer}
              size={16}
              style={{ color: colors.primary }}
            />
          </div>
        ),
        badge: typeLabel,
        title: displayName,
        subtitle: nodeKey,
        copyValue: nodeKey,
        badgeIcon: null,
      };
    } else {
      const { arcType, arcId } = props as ArcIdentityProps;
      return {
        icon: null,
        badge: arcType.replace(/_/g, ' '),
        title: null,
        subtitle: arcId,
        copyValue: arcId,
        badgeIcon: <ArrowRight className="w-3.5 h-3.5" />,
      };
    }
  }, [elementType, colors, props]);

  // Header variant: compact inline display
  if (variant === 'header') {
    return (
      <div
        className={cn(
          'flex items-center justify-between px-4 py-3 border-b border-white/[0.06]',
          className
        )}
        style={{
          background: `linear-gradient(135deg, ${colors.primary}12, ${colors.secondary}08)`,
        }}
      >
        <div className={cn('flex items-center min-w-0', gapTokens.default)}>
          {/* Icon (nodes only) */}
          {content.icon}

          {/* Badge + Title for nodes, Badge only for arcs */}
          {elementType === 'node' ? (
            <div className="min-w-0">
              <h2 className="text-sm font-semibold text-white truncate">
                <TextScramble text={content.title || ''} duration={250} />
              </h2>
              <p className="text-xs text-white/40 truncate">
                <TextScramble text={content.subtitle || ''} duration={300} delay={50} />
              </p>
            </div>
          ) : (
            <div
              className={cn(
                'flex items-center px-3 py-1.5 rounded-lg text-xs font-mono font-semibold uppercase tracking-wider',
                gapTokens.tight
              )}
              style={{
                backgroundColor: `${colors.primary}20`,
                color: colors.primary,
                border: `1px solid ${colors.primary}40`,
                boxShadow: `0 0 12px ${colors.secondary}30`,
              }}
            >
              {content.badgeIcon}
              <span>{content.badge}</span>
            </div>
          )}
        </div>

        {/* Close button */}
        {onClose && (
          <button
            onClick={onClose}
            className="p-1.5 rounded hover:bg-white/10 text-white/40 hover:text-white transition-colors flex-shrink-0"
            title="Close panel (Esc)"
            aria-label="Close details panel"
          >
            <X className="w-4 h-4" />
          </button>
        )}
      </div>
    );
  }

  // Card variant: full display with optional preview
  return (
    <div
      className={cn(
        'relative p-4 rounded-xl overflow-hidden',
        className
      )}
      style={{
        background: `linear-gradient(135deg, ${colors.primary}12, ${colors.secondary}06)`,
        border: `1px solid ${colors.primary}20`,
      }}
    >
      <div className="flex items-start gap-4">
        {/* Left side: Text content */}
        <div className="flex-1 min-w-0">
          {/* Type badge */}
          <div
            className={cn(
              'inline-flex items-center px-2.5 py-1 rounded-full text-xs font-bold mb-3',
              gapTokens.tight
            )}
            style={{
              background: `linear-gradient(135deg, ${colors.primary}30, ${colors.secondary}20)`,
              color: colors.primary,
              boxShadow: `0 0 8px ${colors.primary}25`,
            }}
          >
            {content.badgeIcon}
            <span>{content.badge}</span>
          </div>

          {/* Display name (nodes only) */}
          {content.title && (
            <h3 className="text-base font-semibold text-white mb-1.5 leading-tight">
              <TextScramble text={content.title} duration={250} />
            </h3>
          )}

          {/* Key/ID with copy button */}
          <div className={cn('flex items-center text-sm', gapTokens.tight)}>
            <Hash className="w-3 h-3 text-white/25 flex-shrink-0" />
            <span className="text-white/40 truncate text-xs">
              <TextScramble text={content.subtitle || ''} duration={300} delay={50} />
            </span>
            <CopyButton
              onCopy={() => copy(content.copyValue)}
              isCopied={copied}
              label="Copy"
              size="sm"
            />
          </div>
        </div>

        {/* Right side: Preview slot */}
        {preview && (
          <div className="flex-shrink-0">
            {preview}
          </div>
        )}
      </div>
    </div>
  );
});

export default ElementIdentityCard;
