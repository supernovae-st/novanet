'use client';

/**
 * EasterEggButton - Hidden trigger for Matrix explosion effect
 *
 * Hover: Mouse icon transforms to click mode with glow
 * Click: Triggers global Matrix explosion effect across entire site
 */

import { memo, useState } from 'react';
import { MousePointer2, MousePointerClick } from 'lucide-react';
import { cn } from '@/lib/utils';

interface EasterEggButtonProps {
  onClick: () => void;
  className?: string;
}

export const EasterEggButton = memo(function EasterEggButton({
  onClick,
  className,
}: EasterEggButtonProps) {
  const [isHovered, setIsHovered] = useState(false);
  const [isPressed, setIsPressed] = useState(false);

  return (
    <button
      type="button"
      onClick={onClick}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => { setIsHovered(false); setIsPressed(false); }}
      onMouseDown={() => setIsPressed(true)}
      onMouseUp={() => setIsPressed(false)}
      className={cn(
        'relative p-2.5 rounded-lg transition-all duration-300',
        // Always show background so button is visible
        'bg-white/[0.03] border border-white/10 hover:bg-white/[0.08] hover:border-white/20',
        // Hand cursor
        'cursor-pointer',
        // Pressed state
        isPressed && 'scale-95',
        className
      )}
      title="Click for a surprise..."
      aria-label="Easter egg button"
    >
      {/* Glow ring on hover */}
      {isHovered && (
        <div
          className="absolute inset-0 rounded-lg animate-ping opacity-30"
          style={{
            background: 'radial-gradient(circle, rgba(255,255,255,0.2) 0%, transparent 70%)',
            animationDuration: '1.5s',
          }}
        />
      )}

      {/* Icon transition */}
      <div className="relative w-5 h-5">
        {/* Default: mouse pointer */}
        <MousePointer2
          size={20}
          className={cn(
            'absolute inset-0 transition-all duration-300 text-white/50',
            isHovered && 'opacity-0 scale-50 rotate-12'
          )}
        />

        {/* Hover: click mode with glow */}
        <MousePointerClick
          size={20}
          className={cn(
            'absolute inset-0 transition-all duration-300',
            isHovered
              ? 'opacity-100 scale-110 text-white drop-shadow-[0_0_8px_rgba(255,255,255,0.8)]'
              : 'opacity-0 scale-50'
          )}
        />
      </div>

      {/* Sparkle particles on hover */}
      {isHovered && (
        <div className="absolute inset-0 pointer-events-none overflow-hidden rounded-lg">
          {[...Array(6)].map((_, i) => (
            <span
              key={i}
              className="absolute w-1 h-1 bg-white rounded-full animate-ping"
              style={{
                left: `${20 + Math.random() * 60}%`,
                top: `${20 + Math.random() * 60}%`,
                animationDelay: `${i * 150}ms`,
                animationDuration: '1s',
                opacity: 0.6,
              }}
            />
          ))}
        </div>
      )}
    </button>
  );
});
