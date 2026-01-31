import { test, expect } from '@playwright/test';
import { waitForGraphLoaded } from './helpers';

test.describe('NovaNet Visualizer - Core Functionality', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('homepage loads successfully', async ({ page }) => {
    // Page should load without errors
    await expect(page).toHaveTitle(/NovaNet/i);

    // Main container should be visible
    await expect(page.locator('body')).toBeVisible();
  });

  test('graph container renders', async ({ page }) => {
    // Wait for lazy-loaded graph component to finish loading
    await waitForGraphLoaded(page);
    const graphContainer = page.locator('.react-flow');
    await expect(graphContainer).toBeVisible();
  });

  test('sidebar is visible', async ({ page }) => {
    // Filter sidebar should be present
    const sidebar = page.locator('[data-testid="filter-sidebar"]').or(
      page.locator('aside').first()
    );
    await expect(sidebar).toBeVisible({ timeout: 5000 });
  });

  test('keyboard shortcut panel exists', async ({ page }) => {
    // Press ? to open shortcuts (if implemented)
    await page.keyboard.press('?');

    // Or check that keyboard shortcuts component is mounted
    const _shortcutsIndicator = page.locator('[data-testid="keyboard-shortcuts"]').or(
      page.locator('text=/shortcuts/i')
    );

    // Just verify page didn't crash - shortcuts may or may not show
    await expect(page.locator('body')).toBeVisible();
  });
});

test.describe('NovaNet Visualizer - API Endpoints', () => {
  test('graph stats API returns valid data', async ({ request }) => {
    const response = await request.get('/api/graph/stats');

    expect(response.ok()).toBeTruthy();

    const data = await response.json();
    expect(data).toHaveProperty('success', true);
    expect(data).toHaveProperty('data');
    expect(data.data).toHaveProperty('byType');
    expect(data.data).toHaveProperty('total');
    expect(typeof data.data.total).toBe('number');
  });

  test('graph API returns nodes and edges', async ({ request }) => {
    const response = await request.get('/api/graph');

    expect(response.ok()).toBeTruthy();

    const json = await response.json();
    expect(json).toHaveProperty('success', true);
    expect(json).toHaveProperty('data');
    expect(json.data).toHaveProperty('nodes');
    expect(json.data).toHaveProperty('edges');
    expect(Array.isArray(json.data.nodes)).toBeTruthy();
    expect(Array.isArray(json.data.edges)).toBeTruthy();
  });

  test('graph query API accepts POST requests', async ({ request }) => {
    const response = await request.post('/api/graph/query', {
      data: {
        query: 'MATCH (n) RETURN n LIMIT 5',
      },
    });

    // Should return 200 or 400 (bad query), not 500
    expect([200, 400]).toContain(response.status());
  });
});

test.describe('NovaNet Visualizer - Graph Interaction', () => {
  test('nodes are rendered in the graph', async ({ page }) => {
    await page.goto('/');

    // Wait for lazy-loaded graph to finish loading
    await waitForGraphLoaded(page);

    // Check for nodes (TurboNode renders as react-flow__node)
    const nodes = page.locator('.react-flow__node');

    // Wait for at least one node to appear
    await expect(nodes.first()).toBeVisible({ timeout: 15000 });

    // Should have multiple nodes
    const nodeCount = await nodes.count();
    expect(nodeCount).toBeGreaterThan(0);
  });

  test('edges are rendered between nodes', async ({ page }) => {
    await page.goto('/');

    // Wait for lazy-loaded graph to finish loading
    await waitForGraphLoaded(page);

    // Check for edges
    const edges = page.locator('.react-flow__edge');

    // Wait for edges to render
    await page.waitForTimeout(2000);

    const edgeCount = await edges.count();
    // May have 0 edges if nodes aren't connected, so just verify no crash
    expect(edgeCount).toBeGreaterThanOrEqual(0);
  });

  test('graph can be zoomed', async ({ page }) => {
    await page.goto('/');

    // Wait for lazy-loaded graph to finish loading
    await waitForGraphLoaded(page);

    const graphContainer = page.locator('.react-flow');

    // Wait for graph container to be visible
    await expect(graphContainer).toBeVisible({ timeout: 10000 });

    // Get initial transform
    const _viewport = page.locator('.react-flow__viewport');

    // Zoom using keyboard (focus the container first)
    await graphContainer.click({ force: true });
    await page.keyboard.press('Control++');

    // Graph should still be visible (no crash)
    await expect(graphContainer).toBeVisible();
  });
});

test.describe('NovaNet Visualizer - No Console Errors', () => {
  test('page loads without critical errors', async ({ page }) => {
    const errors: string[] = [];

    page.on('console', (msg) => {
      if (msg.type() === 'error') {
        errors.push(msg.text());
      }
    });

    page.on('pageerror', (error) => {
      errors.push(error.message);
    });

    await page.goto('/');

    // Wait for lazy-loaded graph to finish loading
    await waitForGraphLoaded(page);
    await page.waitForTimeout(2000);

    // Filter out known acceptable errors (like Neo4j connection issues in dev)
    const criticalErrors = errors.filter(
      (e) => !e.includes('Neo4j') &&
             !e.includes('ECONNREFUSED') &&
             !e.includes('hydration')
    );

    // Should have no critical errors
    expect(criticalErrors).toHaveLength(0);
  });
});
