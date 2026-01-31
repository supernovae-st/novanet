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

  // Wait for React Flow container
  await page.waitForSelector('.react-flow', { timeout: 10000 });
}

/**
 * Wait for schema mode to be active
 * Schema mode has different loading behavior than data mode
 */
export async function waitForSchemaMode(page: Page, timeout = 30000) {
  // Wait for schema mode toggle to be active
  await page.waitForSelector('button[aria-pressed="true"]:has-text("Schema")', { timeout });

  // Wait for schema graph to render
  await page.waitForSelector('.react-flow', { timeout: 10000 });
}

/**
 * Wait for data mode to be active (default mode)
 */
export async function waitForDataMode(page: Page, timeout = 30000) {
  // Wait for data mode toggle to be active
  await page.waitForSelector('button[aria-pressed="true"]:has-text("Data")', { timeout });

  // Wait for graph to finish loading
  await waitForGraphLoaded(page, timeout);
}

/**
 * Navigate to schema mode from any state
 */
export async function navigateToSchemaMode(page: Page) {
  const schemaButton = page.locator('button:has-text("Schema")');
  await schemaButton.click();
  await waitForSchemaMode(page);
}

/**
 * Navigate to data mode from any state
 */
export async function navigateToDataMode(page: Page) {
  const dataButton = page.locator('button:has-text("Data")');
  await dataButton.click();
  await waitForDataMode(page);
}
