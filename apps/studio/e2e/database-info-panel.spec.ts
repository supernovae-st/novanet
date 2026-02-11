import { test, expect } from '@playwright/test';
import { waitForGraphLoaded } from './helpers';

// Increase timeout for all tests in this file (schema takes time to load)
test.setTimeout(60000);

test.describe.skip('DatabaseInfoPanel - Multi-select Query Execution', () => {
  // TODO: Rewrite for Query-First Architecture (ADR-021)
  // The "Nodes" tab has been replaced with view-based filtering
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Wait for lazy-loaded graph to finish loading
    await waitForGraphLoaded(page);
    // Click on the "Nodes" tab to see node labels
    const nodesTab = page.locator('button[role="tab"]').filter({ hasText: 'Nodes' });
    await nodesTab.click();
  });

  test('database info panel loads with node labels', async ({ page }) => {
    // Wait for labels container to appear (schema loaded)
    const labelsContainer = page.locator('[data-testid="node-labels-container"]');
    await expect(labelsContainer).toBeVisible({ timeout: 30000 });

    // Should have at least one label button
    const labelButtons = labelsContainer.locator('button');
    await expect(labelButtons.first()).toBeVisible({ timeout: 10000 });
  });

  test('clicking a node label selects it', async ({ page }) => {
    // Wait for labels to load
    const labelsContainer = page.locator('[data-testid="node-labels-container"]');
    await expect(labelsContainer).toBeVisible({ timeout: 30000 });

    // Click the first node type button (has data-selected attribute, not category expand buttons)
    const firstLabel = labelsContainer.locator('button[data-selected]').first();
    await expect(firstLabel).toBeVisible();

    // Click to select
    await firstLabel.click();

    // Should be selected (data-selected="true")
    await expect(firstLabel).toHaveAttribute('data-selected', 'true');
  });

  test('node labels section renders category headers', async ({ page }) => {
    // Wait for labels to load
    const labelsContainer = page.locator('[data-testid="node-labels-container"]');
    await expect(labelsContainer).toBeVisible({ timeout: 30000 });

    // Check for category headers - look for buttons that are category headers (not leaf nodes)
    // Category headers have aria-expanded attribute
    const categoryHeaders = labelsContainer.locator('button[aria-expanded]');
    const categoryCount = await categoryHeaders.count();

    // Verify we have category sections
    expect(categoryCount).toBeGreaterThan(0);

    // Verify the first category header is visible
    await expect(categoryHeaders.first()).toBeVisible();
  });

  test('multi-select: can select multiple labels', async ({ page }) => {
    // Wait for labels to load
    const labelsContainer = page.locator('[data-testid="node-labels-container"]');
    await expect(labelsContainer).toBeVisible({ timeout: 30000 });

    // Get only node type buttons (have data-selected attribute), not category expand buttons
    const nodeTypeButtons = labelsContainer.locator('button[data-selected]');
    const buttonCount = await nodeTypeButtons.count();

    if (buttonCount >= 2) {
      // Select first label
      await nodeTypeButtons.nth(0).click();
      await expect(nodeTypeButtons.nth(0)).toHaveAttribute('data-selected', 'true');

      // Select second label (should be additive, not replace)
      await nodeTypeButtons.nth(1).click();
      await expect(nodeTypeButtons.nth(1)).toHaveAttribute('data-selected', 'true');

      // First should still be selected
      await expect(nodeTypeButtons.nth(0)).toHaveAttribute('data-selected', 'true');
    }
  });

  test('clicking selected label deselects it', async ({ page }) => {
    // Wait for labels to load
    const labelsContainer = page.locator('[data-testid="node-labels-container"]');
    await expect(labelsContainer).toBeVisible({ timeout: 30000 });

    // Get only node type buttons (have data-selected attribute), not category expand buttons
    const firstLabel = labelsContainer.locator('button[data-selected]').first();
    await expect(firstLabel).toBeVisible();

    // Select
    await firstLabel.click();
    await expect(firstLabel).toHaveAttribute('data-selected', 'true');

    // Deselect by clicking again
    await firstLabel.click();
    await expect(firstLabel).toHaveAttribute('data-selected', 'false');
  });

  test('node labels section renders with expandable categories', async ({ page }) => {
    // Wait for labels container to appear
    const labelsContainer = page.locator('[data-testid="node-labels-container"]');
    await expect(labelsContainer).toBeVisible({ timeout: 30000 });

    // Verify there are expandable category headers with chevron icons
    const expandableHeaders = labelsContainer.locator('button[aria-expanded]');
    const count = await expandableHeaders.count();
    expect(count).toBeGreaterThan(0);

    // Verify at least one has leaf items (buttons with data-selected)
    const leafButtons = labelsContainer.locator('button[data-selected]');
    await expect(leafButtons.first()).toBeVisible();
  });
});

test.describe('DatabaseInfoPanel - API Integration', () => {
  test('query API correctly returns nodes for label query', async ({ request }) => {
    // Test the API directly to ensure it works
    const response = await request.post('/api/graph/query', {
      data: {
        cypher: 'MATCH (n:Concept) RETURN n LIMIT 5',
      },
    });

    expect(response.ok()).toBeTruthy();

    const json = await response.json();
    expect(json.success).toBe(true);
    expect(json.data).toHaveProperty('nodes');
    expect(Array.isArray(json.data.nodes)).toBe(true);
  });

  test('schema API returns node labels and relationship types', async ({ request }) => {
    const response = await request.get('/api/graph/schema');

    expect(response.ok()).toBeTruthy();

    const json = await response.json();
    expect(json.success).toBe(true);
    expect(json.data).toHaveProperty('nodeLabels');
    expect(json.data).toHaveProperty('relationshipTypes');
    expect(Array.isArray(json.data.nodeLabels)).toBe(true);
    expect(Array.isArray(json.data.relationshipTypes)).toBe(true);
  });
});
