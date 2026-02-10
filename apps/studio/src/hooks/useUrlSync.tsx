// hooks/useUrlSync.ts
// Bidirectional URL <-> ViewStore synchronization
// v12.0: Simplified - only syncs view state, no navigation mode
'use client';

import { useEffect, useRef, Suspense } from 'react';
import { useRouter, useSearchParams } from 'next/navigation';
import { useViewStore } from '@/stores/viewStore';

/**
 * Internal hook that uses useSearchParams (requires Suspense boundary)
 */
function useUrlSyncInternal() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const { activeViewId, params, syncFromURL, toURLParams } = useViewStore();

  const initialized = useRef(false);
  const debounceRef = useRef<NodeJS.Timeout | null>(null);

  // Hydrate from URL on mount (only once)
  useEffect(() => {
    if (!initialized.current) {
      // Sync viewStore from URL
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
      // Build URL params from viewStore
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

/**
 * URL Sync wrapper component (handles Suspense for useSearchParams)
 */
function UrlSyncProvider() {
  useUrlSyncInternal();
  return null;
}

/**
 * Synchronizes the viewStore state with URL search params.
 * Must be called inside a component - renders a Suspense-wrapped provider.
 *
 * - On mount: Reads URL params and syncs to store
 * - On store change: Updates URL (debounced)
 *
 * URL format: ?view=block-generation&key=hero-pricing&locale=fr-FR
 */
export function useUrlSync() {
  // Note: This returns a component to render, which handles Suspense internally
  // The actual implementation is in useUrlSyncInternal
}

/**
 * Component version for use in JSX - wraps with Suspense boundary
 */
export function UrlSyncComponent() {
  return (
    <Suspense fallback={null}>
      <UrlSyncProvider />
    </Suspense>
  );
}
