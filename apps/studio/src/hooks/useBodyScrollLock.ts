'use client';

/**
 * useBodyScrollLock - Lock body scroll when modal is open
 *
 * Prevents background scrolling when a modal or overlay is visible.
 * Automatically cleans up on unmount.
 *
 * Uses ref-counting to support multiple concurrent modals:
 * - Only locks body when first consumer activates
 * - Only unlocks body when last consumer deactivates
 */

import { useEffect } from 'react';

// Module-level lock counter for ref-counted body scroll lock
// Supports multiple concurrent modals without conflicts
let lockCount = 0;

/**
 * Lock body scroll when condition is true
 *
 * @param isLocked - Whether to lock the body scroll
 *
 * @example
 * // Modal A and Modal B can both use this hook safely
 * useBodyScrollLock(isModalAOpen);
 * useBodyScrollLock(isModalBOpen);
 */
export function useBodyScrollLock(isLocked: boolean): void {
  useEffect(() => {
    if (isLocked) {
      lockCount++;
      // Only set overflow when first lock is acquired
      if (lockCount === 1) {
        document.body.style.overflow = 'hidden';
      }
    }

    return () => {
      if (isLocked) {
        lockCount--;
        // Only restore overflow when last lock is released
        if (lockCount === 0) {
          document.body.style.overflow = '';
        }
      }
    };
  }, [isLocked]);
}
