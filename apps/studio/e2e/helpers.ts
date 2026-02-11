import { Page } from '@playwright/test';

/**
 * Shared E2E test helpers for NovaNet Studio
 *
 * The app uses lazy loading with React.lazy() and Suspense for the Graph2D component.
 * This means we need to wait for:
 * 1. The loading indicator ("Loading graph...") to disappear
 * 2. The React Flow container to render
 */

/**
 * Wait for the graph to fully load
 * Handles the lazy-loaded Graph2D component's Suspense boundary
 */
export async function waitForGraphLoaded(page: Page, timeout = 30000) {
  // Wait for loading indicator to disappear (Suspense fallback)
  await page.waitForFunction(
    () => !document.body.textContent?.includes('Loading graph...'),
    { timeout }
  );

  // Wait for React Flow container (could be 2D or 3D view)
  await page.waitForSelector('.react-flow, [data-testid="react-flow-wrapper"], canvas', {
    timeout: 15000,
    state: 'visible'
  });

  // Give it a moment to render nodes
  await page.waitForTimeout(500);
}
