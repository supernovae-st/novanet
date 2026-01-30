import { test, expect } from '@playwright/test';

/**
 * Smart FitView with Dynamic Insets - E2E Tests
 *
 * Tests the viewport behavior when UI elements change:
 * - Sidebar toggle
 * - Details panel open/close
 * - Focus mode
 * - Node interactions
 */

// Increase timeout for graph loading (Neo4j queries can be slow)
test.setTimeout(60000);

test.describe('Smart FitView with Dynamic Insets', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Wait for graph to load with longer timeout
    await page.waitForSelector('.react-flow', { timeout: 30000 });
    // Wait for nodes to render
    await expect(page.locator('.react-flow__node').first()).toBeVisible({ timeout: 30000 });
    // Extra wait for layout to stabilize
    await page.waitForTimeout(500);
  });

  test('fitView respects sidebar state', async ({ page }) => {
    // Get initial viewport transform
    const getViewportTransform = async () => {
      return page.evaluate(() => {
        const rf = document.querySelector('.react-flow__viewport');
        return rf?.getAttribute('style') ?? '';
      });
    };

    const initialTransform = await getViewportTransform();

    // Close sidebar with [ key
    await page.keyboard.press('[');

    // Wait for animation (sidebar closes + fitView triggers)
    await page.waitForTimeout(500);

    // Get new viewport after sidebar toggle
    const newTransform = await getViewportTransform();

    // Viewport should have changed (more space available, graph re-fits)
    expect(newTransform).not.toBe(initialTransform);
  });

  test('double-click on node centers but does NOT open panel', async ({ page }) => {
    // Get initial viewport
    const getViewportTransform = async () => {
      return page.evaluate(() => {
        const rf = document.querySelector('.react-flow__viewport');
        return rf?.getAttribute('style') ?? '';
      });
    };

    const initialTransform = await getViewportTransform();

    // Find and double-click a node
    const node = page.locator('.react-flow__node').first();
    await node.dblclick();

    // Wait for viewport change
    await page.waitForTimeout(500);

    // Panel should NOT be visible (double-click no longer opens panel)
    const detailsPanel = page.locator('aside').filter({ hasText: /Node Details|Relationship Details/ });
    await expect(detailsPanel).not.toBeVisible();

    // Viewport should have changed (centered on node)
    const newTransform = await getViewportTransform();
    expect(newTransform).not.toBe(initialTransform);
  });

  test('closing panel triggers fitView', async ({ page }) => {
    const getViewportTransform = async () => {
      return page.evaluate(() => {
        const rf = document.querySelector('.react-flow__viewport');
        return rf?.getAttribute('style') ?? '';
      });
    };

    // Open panel by ⌘+clicking a node
    const node = page.locator('.react-flow__node').first();
    await node.click({ modifiers: ['Meta'] });

    // Wait for panel to appear
    const detailsPanel = page.locator('aside').filter({ hasText: /Node Details|Relationship Details/ });
    await expect(detailsPanel).toBeVisible({ timeout: 2000 });

    // Wait for viewport to settle
    await page.waitForTimeout(500);

    // Get viewport with panel open
    const viewportWithPanel = await getViewportTransform();

    // Close panel with ] key
    await page.keyboard.press(']');

    // Wait for animation
    await page.waitForTimeout(500);

    // Panel should be closed
    await expect(detailsPanel).not.toBeVisible();

    // Get viewport after panel closed
    const viewportAfterClose = await getViewportTransform();

    // Viewport should have changed (reclaimed space)
    expect(viewportAfterClose).not.toBe(viewportWithPanel);
  });

  test('focus mode uses minimal insets', async ({ page }) => {
    // Ensure sidebar is open (may have been closed by parallel tests due to persisted state)
    const filterSidebar = page.locator('aside.w-72');
    const toggleButton = page.locator('button[title="Toggle sidebar ([)"]');

    // Check if sidebar is visible, if not, open it
    const isSidebarVisible = await filterSidebar.isVisible().catch(() => false);
    if (!isSidebarVisible) {
      await toggleButton.click();
      await page.waitForTimeout(300);
    }

    // Verify sidebar is now visible
    await expect(filterSidebar).toBeVisible({ timeout: 5000 });

    // Enter focus mode with G key
    await page.keyboard.press('g');

    // Wait for focus mode animation
    await page.waitForTimeout(500);

    // Focus mode indicator should appear
    const focusIndicator = page.locator('text=Focus Mode');
    await expect(focusIndicator).toBeVisible({ timeout: 2000 });

    // Filter sidebar should be hidden in focus mode
    await expect(filterSidebar).not.toBeVisible();

    // Exit focus mode with G key
    await page.keyboard.press('g');

    // Wait for animation
    await page.waitForTimeout(500);

    // Focus indicator should be gone
    await expect(focusIndicator).not.toBeVisible();

    // Note: Current implementation does NOT automatically restore sidebar when exiting focus mode
    // The sidebar toggle button should be visible (allowing manual restoration)
    await expect(toggleButton).toBeVisible({ timeout: 2000 });

    // Manually reopen sidebar to verify it works after exiting focus mode
    await toggleButton.click();
    await page.waitForTimeout(300);

    // Sidebar should now be visible again
    await expect(filterSidebar).toBeVisible({ timeout: 5000 });
  });

  test('single-click does NOT open panel (⌘+click required)', async ({ page }) => {
    // Single-click on a node
    const node = page.locator('.react-flow__node').first();
    await node.click();

    // Wait briefly
    await page.waitForTimeout(300);

    // Details panel should NOT be visible (single-click no longer opens panel)
    const detailsPanel = page.locator('aside').filter({ hasText: /Node Details|Relationship Details/ });
    await expect(detailsPanel).not.toBeVisible();

    // Now test ⌘+click opens the panel
    await node.click({ modifiers: ['Meta'] });
    await page.waitForTimeout(300);

    // Panel should now be visible
    await expect(detailsPanel).toBeVisible({ timeout: 2000 });
  });
});

test.describe('Smart FitView - Viewport Transform Verification', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('.react-flow', { timeout: 30000 });
    await expect(page.locator('.react-flow__node').first()).toBeVisible({ timeout: 30000 });
    await page.waitForTimeout(500);
  });

  test('viewport transform contains translate and scale', async ({ page }) => {
    // Verify that the viewport transform is properly structured
    const transform = await page.evaluate(() => {
      const rf = document.querySelector('.react-flow__viewport');
      return rf?.getAttribute('style') ?? '';
    });

    // Should contain transform with translate3d (React Flow uses this)
    expect(transform).toContain('transform');
    expect(transform).toMatch(/translate|matrix/);
  });

  test('sidebar toggle changes viewport translate values', async ({ page }) => {
    // Helper to extract translate values from transform style
    const getTranslateX = async () => {
      return page.evaluate(() => {
        const rf = document.querySelector('.react-flow__viewport');
        const style = rf?.getAttribute('style') ?? '';
        // Extract translateX from transform: translate(Xpx, Ypx) or translate3d(Xpx, Ypx, Zpx)
        const match = style.match(/translate(?:3d)?\(\s*([-\d.]+)/);
        return match ? parseFloat(match[1]) : 0;
      });
    };

    const initialX = await getTranslateX();

    // Toggle sidebar
    await page.keyboard.press('[');
    await page.waitForTimeout(500);

    const newX = await getTranslateX();

    // When sidebar closes, viewport should shift (reclaiming space)
    // The exact direction depends on the implementation, but it should change
    expect(newX).not.toBe(initialX);
  });
});

test.describe('Smart FitView - Edge Cases', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('.react-flow', { timeout: 30000 });
    await expect(page.locator('.react-flow__node').first()).toBeVisible({ timeout: 30000 });
    await page.waitForTimeout(500);
  });

  test('escape key closes panel', async ({ page }) => {
    // Open panel with ⌘+click
    const node = page.locator('.react-flow__node').first();
    await node.click({ modifiers: ['Meta'] });

    // Wait for panel
    const detailsPanel = page.locator('aside').filter({ hasText: /Node Details|Relationship Details/ });
    await expect(detailsPanel).toBeVisible({ timeout: 2000 });

    // Close with Escape
    await page.keyboard.press('Escape');
    await page.waitForTimeout(300);

    // Panel should be closed
    await expect(detailsPanel).not.toBeVisible();
  });

  test('multiple rapid sidebar toggles do not break viewport', async ({ page }) => {
    // Rapidly toggle sidebar multiple times
    for (let i = 0; i < 3; i++) {
      await page.keyboard.press('[');
      await page.waitForTimeout(100);
    }

    // Wait for animations to settle
    await page.waitForTimeout(600);

    // Graph should still be visible and functional
    const graphContainer = page.locator('.react-flow');
    await expect(graphContainer).toBeVisible();

    // Nodes should still be visible
    const nodes = page.locator('.react-flow__node');
    await expect(nodes.first()).toBeVisible();
  });

  test('close button closes panel', async ({ page }) => {
    // Open panel with ⌘+click
    const node = page.locator('.react-flow__node').first();
    await node.click({ modifiers: ['Meta'] });

    // Wait for panel
    const detailsPanel = page.locator('aside').filter({ hasText: /Node Details|Relationship Details/ });
    await expect(detailsPanel).toBeVisible({ timeout: 2000 });

    // Close panel using the close button (] or Esc title indicates the button)
    const closeButton = detailsPanel.locator('button[title*="Close"]');
    await closeButton.click();
    await page.waitForTimeout(500);

    // Panel should be closed
    await expect(detailsPanel).not.toBeVisible({ timeout: 3000 });
  });
});
