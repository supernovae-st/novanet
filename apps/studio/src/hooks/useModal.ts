'use client';

/**
 * useModal - Unified modal management hook
 *
 * Combines common modal patterns:
 * - Client-side mounting check for SSR
 * - Body scroll lock when open
 * - Escape key to close
 * - Outside click to close (optional)
 *
 * Extracted from CommandPalette, KeyboardShortcuts, AiChat, CypherEditModal
 */

import { useState, useEffect, useRef, type RefObject } from 'react';
import { useBodyScrollLock } from './useBodyScrollLock';
import { useEscapeKey } from './useEscapeKey';
import { useOutsideClick } from './useOutsideClick';

export interface UseModalOptions {
  /** Whether to lock body scroll when modal is open (default: true) */
  lockScroll?: boolean;
  /** Whether to close on Escape key (default: true) */
  closeOnEscape?: boolean;
  /** Whether to close on outside click (default: false) */
  closeOnOutsideClick?: boolean;
}

export interface UseModalReturn {
  /** Whether component is mounted (for SSR safety) */
  mounted: boolean;
  /** Whether modal should be rendered (mounted && isOpen) */
  shouldRender: boolean;
  /** Ref to attach to modal container (for outside click detection) */
  modalRef: RefObject<HTMLDivElement | null>;
}

/**
 * Unified modal management
 *
 * @param isOpen - Whether modal is open
 * @param onClose - Callback to close modal
 * @param options - Configuration options
 *
 * @example
 * const { shouldRender, modalRef } = useModal(isOpen, onClose, {
 *   closeOnOutsideClick: true
 * });
 *
 * if (!shouldRender) return null;
 *
 * return createPortal(
 *   <div ref={modalRef}>...</div>,
 *   document.body
 * );
 */
export function useModal(
  isOpen: boolean,
  onClose: () => void,
  options: UseModalOptions = {}
): UseModalReturn {
  const {
    lockScroll = true,
    closeOnEscape = true,
    closeOnOutsideClick = false,
  } = options;

  const [mounted, setMounted] = useState(false);
  const modalRef = useRef<HTMLDivElement>(null);

  // Client-side mounting check for SSR
  useEffect(() => {
    setMounted(true);
  }, []);

  // Body scroll lock
  useBodyScrollLock(lockScroll && isOpen);

  // Escape key handling
  useEscapeKey(onClose, closeOnEscape && isOpen);

  // Outside click handling
  useOutsideClick(modalRef, onClose, closeOnOutsideClick && isOpen);

  return {
    mounted,
    shouldRender: mounted && isOpen,
    modalRef,
  };
}
