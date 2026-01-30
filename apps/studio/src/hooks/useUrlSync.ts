// hooks/useUrlSync.ts
// Bidirectional URL <-> ViewStore synchronization
'use client';

import { useEffect, useRef } from 'react';
import { useRouter, useSearchParams } from 'next/navigation';
import { useViewStore } from '@/stores/viewStore';

/**
 * Synchronizes the viewStore state with URL search params.
 *
 * - On mount: Reads URL params and syncs to store
 * - On store change: Updates URL (debounced)
 *
 * URL format: ?view=block-generation&key=hero-pricing&locale=fr-FR
 */
export function useUrlSync() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const { activeViewId, params, syncFromURL, toURLParams } = useViewStore();
  const initialized = useRef(false);
  const debounceRef = useRef<NodeJS.Timeout | null>(null);

  // Hydrate from URL on mount (only once)
  useEffect(() => {
    if (!initialized.current) {
      syncFromURL(searchParams);
      initialized.current = true;
    }
  }, [searchParams, syncFromURL]);

  // Sync store changes to URL (debounced)
  useEffect(() => {
    if (!initialized.current) return;

    // Clear any pending debounce
    if (debounceRef.current) {
      clearTimeout(debounceRef.current);
    }

    // Debounce URL updates by 300ms
    debounceRef.current = setTimeout(() => {
      const newParams = toURLParams();
      const currentParams = new URLSearchParams(searchParams.toString());

      // Only update if params changed
      if (newParams.toString() !== currentParams.toString()) {
        const newUrl = newParams.toString() ? `?${newParams.toString()}` : '/';
        router.replace(newUrl, { scroll: false });
      }
    }, 300);

    return () => {
      if (debounceRef.current) {
        clearTimeout(debounceRef.current);
      }
    };
  }, [activeViewId, params, router, searchParams, toURLParams]);
}
