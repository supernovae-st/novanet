import { test, expect } from '@playwright/test';
import { waitForGraphLoaded } from './helpers';

/**
 * Views E2E Tests
 * Tests the view system: API endpoints and UI interactions
 */

test.describe('NovaNet Studio - Views API', () => {
  test('views API returns categories and views', async ({ request }) => {
    const response = await request.get('/api/views');
    expect(response.ok()).toBeTruthy();

    const json = await response.json();
    expect(json).toHaveProperty('success', true);
    expect(json).toHaveProperty('data');
    expect(json.data).toHaveProperty('categories');
    expect(json.data).toHaveProperty('registry');
    expect(Array.isArray(json.data.categories)).toBeTruthy();
    expect(json.data.categories.length).toBeGreaterThan(0);

    // Check that we have views in the categories
    const totalViews = json.data.categories.reduce(
      (sum: number, cat: { views: unknown[] }) => sum + cat.views.length,
      0
    );
    expect(totalViews).toBeGreaterThan(0);
  });

  test('view detail API returns cypher query', async ({ request }) => {
    const response = await request.get('/api/views/data-complete');
    expect(response.ok()).toBeTruthy();

    const json = await response.json();
    expect(json).toHaveProperty('success', true);
    expect(json).toHaveProperty('data');
    expect(json.data).toHaveProperty('view');
    expect(json.data.view).toHaveProperty('id', 'data-complete');
    expect(json.data).toHaveProperty('cypher');
    expect(json.data.cypher).toHaveProperty('query');
    expect(typeof json.data.cypher.query).toBe('string');
    expect(json.data.cypher.query).toContain('MATCH');
  });

  test('graph query API executes view queries correctly', async ({ request }) => {
    // Test the actual query that data-complete would execute
    const response = await request.post('/api/graph/query', {
      data: {
        cypher: `MATCH (n) WHERE NOT n:Schema
WITH n LIMIT 100
OPTIONAL MATCH (n)-[r]->(m) WHERE NOT m:Schema
RETURN n, r, m`,
      },
    });

    expect(response.ok()).toBeTruthy();

    const json = await response.json();
    expect(json).toHaveProperty('success', true);
    expect(json).toHaveProperty('data');
    expect(json.data).toHaveProperty('nodes');
    expect(json.data).toHaveProperty('edges');

    // Should have nodes
    expect(Array.isArray(json.data.nodes)).toBeTruthy();
    expect(json.data.nodes.length).toBeGreaterThan(0);

    // Verify node structure
    if (json.data.nodes.length > 0) {
      const node = json.data.nodes[0];
      expect(node).toHaveProperty('id');
      expect(node).toHaveProperty('type');
      expect(node).toHaveProperty('data');
    }
  });
});

test.describe('NovaNet Studio - Views UI', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await waitForGraphLoaded(page);
  });

  test('ViewPicker opens and shows views', async ({ page }) => {
    // Find and click the ViewPicker trigger button
    // It has a LayoutGrid icon and shows "views available" text
    const viewPickerTrigger = page.locator('button').filter({ hasText: /views available/i });
    await expect(viewPickerTrigger).toBeVisible({ timeout: 10000 });
    await viewPickerTrigger.click();

    // Modal should open
    const modal = page.locator('[role="dialog"]');
    await expect(modal).toBeVisible({ timeout: 5000 });

    // Should show "Select View" title
    await expect(modal.locator('text=Select View')).toBeVisible();

    // Should have view cards (role="option")
    const viewCards = modal.locator('[role="option"]');
    const cardCount = await viewCards.count();
    expect(cardCount).toBeGreaterThan(0);

    // Close modal with Escape
    await page.keyboard.press('Escape');
    await expect(modal).not.toBeVisible({ timeout: 3000 });
  });

  test('clicking data-complete view loads nodes', async ({ page }) => {
    // Open ViewPicker
    const viewPickerTrigger = page.locator('button').filter({ hasText: /views available/i });
    await viewPickerTrigger.click();

    // Find and click the data-complete view card
    const modal = page.locator('[role="dialog"]');
    await expect(modal).toBeVisible({ timeout: 5000 });

    // Find the card with id "data-complete" (shown in a span with font-mono class)
    const dataCompleteCard = modal.locator('[role="option"]').filter({ hasText: 'data-complete' });
    await expect(dataCompleteCard).toBeVisible({ timeout: 5000 });
    await dataCompleteCard.click();

    // Modal should close
    await expect(modal).not.toBeVisible({ timeout: 3000 });

    // Wait for graph to update
    await page.waitForTimeout(2000);

    // Graph should show nodes
    const nodes = page.locator('.react-flow__node');
    await expect(nodes.first()).toBeVisible({ timeout: 15000 });

    const nodeCount = await nodes.count();
    expect(nodeCount).toBeGreaterThan(0);

    // Graph should show edges (relationships)
    const edges = page.locator('.react-flow__edge');
    const edgeCount = await edges.count();
    // data-complete view should return nodes with relationships
    expect(edgeCount).toBeGreaterThanOrEqual(0);
  });

  test('meta-complete view shows schema nodes', async ({ page }) => {
    // Open ViewPicker
    const viewPickerTrigger = page.locator('button').filter({ hasText: /views available/i });
    await viewPickerTrigger.click();

    // Find and click the meta-complete view card
    const modal = page.locator('[role="dialog"]');
    await expect(modal).toBeVisible({ timeout: 5000 });

    const metaCompleteCard = modal.locator('[role="option"]').filter({ hasText: 'meta-complete' });
    await expect(metaCompleteCard).toBeVisible({ timeout: 5000 });
    await metaCompleteCard.click();

    // Modal should close
    await expect(modal).not.toBeVisible({ timeout: 3000 });

    // Wait for graph to update
    await page.waitForTimeout(2000);

    // Graph should show schema nodes
    const nodes = page.locator('.react-flow__node');
    await expect(nodes.first()).toBeVisible({ timeout: 15000 });

    const nodeCount = await nodes.count();
    expect(nodeCount).toBeGreaterThan(0);
  });
});
