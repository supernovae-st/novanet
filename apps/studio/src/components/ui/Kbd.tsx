'use client';

/**
 * Kbd - Keyboard shortcut badge component (Skeuomorphic)
 *
 * Thin wrapper around KeyboardKey with size="sm" for backward compatibility.
 * For more control (sizes, interactive mode), use KeyboardKey directly.
 */

import { memo } from 'react';
import { KeyboardKey } from './KeyboardKey';

interface KbdProps {
  children: React.ReactNode;
  className?: string;
}

export const Kbd = memo(function Kbd({ children, className }: KbdProps) {
  return (
    <KeyboardKey size="sm" className={className}>
      {children}
    </KeyboardKey>
  );
});
