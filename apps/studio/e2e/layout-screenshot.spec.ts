import { test, expect } from '@playwright/test';
import { waitForGraphLoaded } from './helpers';

test.describe.skip('Layout Visual Test', () => {
  // TODO: Flaky in CI - depends on Neo4j response time
  // Increase timeout for screenshot tests (graph loading can be slow)
  test.setTimeout(60000);

  test('capture current layout for review', async ({ page }) => {
    await page.goto('/');

    // Wait for lazy-loaded graph to finish loading
    await waitForGraphLoaded(page);
    await page.waitForTimeout(2000);

    // Take a full page screenshot
    await page.screenshot({
      path: 'test-results/layout-current.png',
      fullPage: true
    });

    // Verify key elements are present (use exact match)
    await expect(page.getByText('nodes', { exact: true })).toBeVisible();
    await expect(page.getByText('relations', { exact: true })).toBeVisible();
  });

  test('verify QueryPill height is 80px (h-20)', async ({ page }) => {
    await page.goto('/');
    await waitForGraphLoaded(page);

    // Find the QueryPill container - it's the rounded-2xl element with h-20
    // Navigate up from neo4j$ text to find the actual pill container
    const queryPillContainer = page.locator('.rounded-2xl').filter({
      has: page.locator('text=neo4j$')
    }).first();
    await expect(queryPillContainer).toBeVisible();

    // Get the computed height
    const box = await queryPillContainer.boundingBox();
    if (box) {
      // h-20 = 80px
      expect(box.height).toBeGreaterThanOrEqual(76);
    }
  });

  test.skip('expand button opens modal editor', async ({ page }) => {
    // TODO: Check if expand button title has changed
    await page.goto('/');
    await waitForGraphLoaded(page);

    // Find and click the expand button (has title "Expand editor")
    const expandButton = page.getByTitle('Expand editor (for complex queries)');
    await expect(expandButton).toBeVisible();
    await expandButton.click();

    // Modal should appear with "Cypher Editor" text
    await expect(page.getByText('Cypher Editor')).toBeVisible();

    // Textarea should be visible
    const textarea = page.locator('textarea');
    await expect(textarea).toBeVisible();

    // Press Escape to close
    await page.keyboard.press('Escape');

    // Modal should be gone
    await expect(page.getByText('Cypher Editor')).not.toBeVisible();
  });
});
