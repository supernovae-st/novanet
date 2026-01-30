import { test, expect } from '@playwright/test';

test.describe('Layout Visual Test', () => {
  test('capture current layout for review', async ({ page }) => {
    await page.goto('/');

    // Wait for the page to fully load
    await page.waitForSelector('.react-flow', { timeout: 10000 });
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
    await page.waitForSelector('.react-flow', { timeout: 10000 });

    // Find the QueryPill by the neo4j$ prompt
    const queryPill = page.locator('text=neo4j$').locator('..');
    await expect(queryPill).toBeVisible();

    // Get the computed height
    const box = await queryPill.boundingBox();
    if (box) {
      console.log(`QueryPill height: ${box.height}px`);
      // h-20 = 80px
      expect(box.height).toBeGreaterThanOrEqual(76);
    }
  });

  test('expand button opens modal editor', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('.react-flow', { timeout: 10000 });

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
