import { test, expect, type Page } from '@playwright/test';

/**
 * Schema Mode E2E Tests
 *
 * Phase 5 tests from docs/plans/2026-01-30-schema-mode-v2.md
 *
 * Tests:
 * 1. Toggle between data and schema mode
 * 2. Schema mode shows 35 nodes in grouped layout
 * 3. URL sync works (?mode=schema)
 * 4. Scope groups are visible
 * 5. Filter panel shows hierarchical filters
 * 6. Collapsed state persists (collapse a scope, verify nodes hidden)
 */

// Increase timeout for graph loading and ELK layout (can be slow)
test.setTimeout(90000);

// Helper: Wait for schema mode to fully load (ELK layout complete)
async function waitForSchemaMode(page: Page) {
  // Wait for schema wrapper to appear
  await page.waitForSelector('[data-testid="react-flow-wrapper-schema"]', { timeout: 30000 });

  // Wait for loading indicator to disappear (ELK layout complete)
  const loadingIndicator = page.locator('[data-testid="schema-loading-indicator"]');
  await expect(loadingIndicator).not.toBeVisible({ timeout: 30000 });

  // Wait for React Flow to render nodes
  await page.waitForSelector('.react-flow', { timeout: 10000 });

  // Additional wait for ELK layout to stabilize
  await page.waitForTimeout(500);
}

// Helper: Get the data mode toggle button
function getDataModeToggle(page: Page) {
  // The toggle shows "Data" when in data mode, "Schema" when in schema mode
  return page.locator('button').filter({ hasText: /^(Data|Schema)$/ });
}

// Helper: Check if currently in schema mode
async function isInSchemaMode(page: Page): Promise<boolean> {
  const toggle = getDataModeToggle(page);
  const text = await toggle.textContent();
  return text?.includes('Schema') ?? false;
}

// Helper: Switch to schema mode if not already
async function switchToSchemaMode(page: Page) {
  const inSchemaMode = await isInSchemaMode(page);
  if (!inSchemaMode) {
    await getDataModeToggle(page).click();
    await waitForSchemaMode(page);
  }
}

test.describe('Schema Mode - Mode Toggle', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Wait for initial data mode to load
    await page.waitForSelector('.react-flow', { timeout: 30000 });
  });

  test('should display Data mode toggle by default', async ({ page }) => {
    const toggle = getDataModeToggle(page);
    await expect(toggle).toBeVisible();

    // Should show "Data" text (indicating data mode is active)
    await expect(toggle).toHaveText('Data');

    // Should have emerald color class (data mode color)
    await expect(toggle).toHaveClass(/text-emerald/);
  });

  test('should toggle from data mode to schema mode', async ({ page }) => {
    const toggle = getDataModeToggle(page);

    // Initially in data mode
    await expect(toggle).toHaveText('Data');

    // Click to switch to schema mode
    await toggle.click();

    // Wait for schema mode to load
    await waitForSchemaMode(page);

    // Should now show "Schema" text
    await expect(toggle).toHaveText('Schema');

    // Should have violet color class (schema mode color)
    await expect(toggle).toHaveClass(/text-violet/);
  });

  test('should toggle back from schema mode to data mode', async ({ page }) => {
    // Switch to schema mode first
    await switchToSchemaMode(page);

    const toggle = getDataModeToggle(page);
    await expect(toggle).toHaveText('Schema');

    // Click to switch back to data mode
    await toggle.click();

    // Wait for data mode to load
    await page.waitForSelector('[data-testid="react-flow-wrapper"]', { timeout: 30000 });

    // Should show "Data" text again
    await expect(toggle).toHaveText('Data');
  });
});

test.describe('Schema Mode - Schema Graph Display', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('.react-flow', { timeout: 30000 });
    await switchToSchemaMode(page);
  });

  test('should display grouped layout with scope groups', async ({ page }) => {
    // Should see all 3 scope group labels in the schema filter panel or graph
    // The scope groups are rendered with icon + label (e.g., "PROJECT", "GLOBAL", "SHARED")

    // Check schema filter panel for scope groups (these are always visible)
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');
    await expect(filterPanel).toBeVisible();

    // PROJECT scope
    await expect(filterPanel.getByText('PROJECT')).toBeVisible();

    // GLOBAL scope
    await expect(filterPanel.getByText('GLOBAL')).toBeVisible();

    // SHARED scope
    await expect(filterPanel.getByText('SHARED')).toBeVisible();
  });

  test('should display schema stats overlay with node count', async ({ page }) => {
    // The stats overlay shows "X nodes . Y edges" at bottom left
    const statsOverlay = page.locator('text=/\\d+ nodes/');
    await expect(statsOverlay).toBeVisible({ timeout: 10000 });

    // Get the stats text
    const statsText = await statsOverlay.textContent();

    // Should mention nodes and edges
    expect(statsText).toMatch(/\d+ nodes/);
    expect(statsText).toMatch(/\d+ edges/);
  });

  test('should render React Flow nodes in schema mode', async ({ page }) => {
    // Wait for React Flow nodes to render
    const nodes = page.locator('.react-flow__node');
    await expect(nodes.first()).toBeVisible({ timeout: 30000 });

    // Should have multiple nodes (scope groups + subcategory groups + schema nodes)
    const nodeCount = await nodes.count();
    expect(nodeCount).toBeGreaterThan(0);
  });
});

test.describe('Schema Mode - URL Sync', () => {
  test('should update URL when switching to schema mode', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('.react-flow', { timeout: 30000 });

    // Initially no mode param (data mode is default)
    await expect(page).not.toHaveURL(/mode=/);

    // Switch to schema mode
    await switchToSchemaMode(page);

    // URL should contain mode=schema (after debounce)
    await expect(page).toHaveURL(/mode=schema/, { timeout: 5000 });
  });

  test('should load in schema mode when URL has mode=schema', async ({ page }) => {
    // Go directly to schema mode URL
    await page.goto('/?mode=schema');

    // Wait for schema mode to load
    await waitForSchemaMode(page);

    // Toggle should show Schema
    const toggle = getDataModeToggle(page);
    await expect(toggle).toHaveText('Schema');

    // Should have the schema wrapper
    await expect(page.locator('[data-testid="react-flow-wrapper-schema"]')).toBeVisible();
  });

  test('should remove mode param when switching back to data mode', async ({ page }) => {
    // Start in schema mode
    await page.goto('/?mode=schema');
    await waitForSchemaMode(page);

    // Switch back to data mode
    const toggle = getDataModeToggle(page);
    await toggle.click();

    // Wait for data mode
    await page.waitForSelector('[data-testid="react-flow-wrapper"]', { timeout: 30000 });

    // URL should not have mode param (or mode=data which may be omitted as default)
    // Wait for URL sync debounce
    await page.waitForTimeout(500);
    const url = page.url();
    expect(url).not.toContain('mode=schema');
  });

  test('should persist schema mode across page refresh', async ({ page }) => {
    // Go to schema mode
    await page.goto('/?mode=schema');
    await waitForSchemaMode(page);

    // Refresh the page
    await page.reload();

    // Should still be in schema mode
    await waitForSchemaMode(page);

    const toggle = getDataModeToggle(page);
    await expect(toggle).toHaveText('Schema');
  });
});

test.describe('Schema Mode - Filter Panel', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/?mode=schema');
    await waitForSchemaMode(page);
  });

  test('should display SchemaFilterPanel in sidebar when in schema mode', async ({ page }) => {
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');
    await expect(filterPanel).toBeVisible();

    // Should have the header
    await expect(filterPanel.getByText('Schema Filters')).toBeVisible();
  });

  test('should show all 3 scopes with icons', async ({ page }) => {
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');

    // Check for scope labels (icons are emojis rendered with the label)
    await expect(filterPanel.getByText('PROJECT')).toBeVisible();
    await expect(filterPanel.getByText('GLOBAL')).toBeVisible();
    await expect(filterPanel.getByText('SHARED')).toBeVisible();
  });

  test('should display subcategories for each scope', async ({ page }) => {
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');

    // Project subcategories
    await expect(filterPanel.getByText('Foundation')).toBeVisible();
    await expect(filterPanel.getByText('Structure')).toBeVisible();
    await expect(filterPanel.getByText('Semantic')).toBeVisible();
    await expect(filterPanel.getByText('Instruction')).toBeVisible();
    await expect(filterPanel.getByText('Output')).toBeVisible();

    // Global subcategories
    await expect(filterPanel.getByText('Configuration')).toBeVisible();
    await expect(filterPanel.getByText('Knowledge')).toBeVisible();

    // Shared subcategories
    await expect(filterPanel.getByText('SEO')).toBeVisible();
    await expect(filterPanel.getByText('GEO')).toBeVisible();
  });

  test('should show node counts for subcategories', async ({ page }) => {
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');

    // Look for count patterns like "(3)", "(14)", etc.
    // Foundation has 3 nodes, Knowledge has 14 nodes
    await expect(filterPanel.getByText('(3)')).toBeVisible();
    await expect(filterPanel.getByText('(14)')).toBeVisible();
  });

  test('should show stats footer with totals', async ({ page }) => {
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');

    // Footer shows "35 node types . 9 subcategories . 3 scopes"
    await expect(filterPanel.getByText(/35 node types/)).toBeVisible();
    await expect(filterPanel.getByText(/9 subcategories/)).toBeVisible();
    await expect(filterPanel.getByText(/3 scopes/)).toBeVisible();
  });
});

test.describe('Schema Mode - Scope Collapse', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/?mode=schema');
    await waitForSchemaMode(page);
  });

  test('should collapse scope when header is clicked', async ({ page }) => {
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');

    // Find the GLOBAL scope header button
    const globalScopeHeader = filterPanel.getByRole('button', { name: /GLOBAL/ });
    await expect(globalScopeHeader).toBeVisible();

    // Initially expanded - should show subcategories
    await expect(filterPanel.getByText('Configuration')).toBeVisible();
    await expect(filterPanel.getByText('Knowledge')).toBeVisible();

    // Click to collapse
    await globalScopeHeader.click();

    // Wait for UI update
    await page.waitForTimeout(300);

    // Subcategories should be hidden
    await expect(filterPanel.getByText('Configuration')).not.toBeVisible();
    await expect(filterPanel.getByText('Knowledge')).not.toBeVisible();

    // Click again to expand
    await globalScopeHeader.click();
    await page.waitForTimeout(300);

    // Subcategories should be visible again
    await expect(filterPanel.getByText('Configuration')).toBeVisible();
    await expect(filterPanel.getByText('Knowledge')).toBeVisible();
  });

  test('should update graph nodes when scope is collapsed', async ({ page }) => {
    // Get initial node count from stats overlay
    const getNodeCount = async (): Promise<number> => {
      const statsText = await page.locator('text=/\\d+ nodes/').textContent();
      const match = statsText?.match(/(\d+) nodes/);
      return match ? parseInt(match[1], 10) : 0;
    };

    const initialCount = await getNodeCount();
    expect(initialCount).toBeGreaterThan(0);

    // Collapse the GLOBAL scope (15 nodes)
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');
    const globalScopeHeader = filterPanel.getByRole('button', { name: /GLOBAL/ });
    await globalScopeHeader.click();

    // Wait for ELK re-layout
    await page.waitForTimeout(1000);

    // Node count should decrease
    const newCount = await getNodeCount();
    expect(newCount).toBeLessThan(initialCount);
  });

  test('should have ARIA attributes for accessibility', async ({ page }) => {
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');

    // Scope headers should have aria-expanded
    const projectHeader = filterPanel.getByRole('button', { name: /PROJECT/ });
    await expect(projectHeader).toHaveAttribute('aria-expanded', 'true');

    // Click to collapse
    await projectHeader.click();
    await page.waitForTimeout(300);

    // Should now be collapsed
    await expect(projectHeader).toHaveAttribute('aria-expanded', 'false');

    // Subcategory buttons should have aria-pressed
    // First expand the scope again
    await projectHeader.click();
    await page.waitForTimeout(300);

    // Find a subcategory button
    const foundationButton = filterPanel.getByRole('button', { name: /Foundation/ });
    await expect(foundationButton).toHaveAttribute('aria-pressed');
  });
});

test.describe('Schema Mode - Subcategory Toggle', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/?mode=schema');
    await waitForSchemaMode(page);
  });

  test('should toggle subcategory visibility when clicked', async ({ page }) => {
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');

    // Get initial node count
    const getNodeCount = async (): Promise<number> => {
      const statsText = await page.locator('text=/\\d+ nodes/').textContent();
      const match = statsText?.match(/(\d+) nodes/);
      return match ? parseInt(match[1], 10) : 0;
    };

    const initialCount = await getNodeCount();

    // Click on "Foundation" subcategory to toggle it off
    const foundationButton = filterPanel.getByRole('button', { name: /Foundation/ });
    await foundationButton.click();

    // Wait for graph to re-render
    await page.waitForTimeout(1000);

    // Node count should decrease (Foundation has 3 nodes)
    const newCount = await getNodeCount();
    expect(newCount).toBeLessThan(initialCount);

    // Toggle it back on
    await foundationButton.click();
    await page.waitForTimeout(1000);

    // Count should be restored
    const restoredCount = await getNodeCount();
    expect(restoredCount).toBe(initialCount);
  });

  test('should show visual feedback when subcategory is hidden', async ({ page }) => {
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');

    // Find the Foundation button
    const foundationButton = filterPanel.getByRole('button', { name: /Foundation/ });

    // Initially should not have opacity-50 (visible state)
    await expect(foundationButton).not.toHaveClass(/opacity-50/);

    // Toggle off
    await foundationButton.click();
    await page.waitForTimeout(300);

    // Should now have opacity-50 class (hidden state)
    await expect(foundationButton).toHaveClass(/opacity-50/);
  });
});

test.describe('Schema Mode - Graph Interaction', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/?mode=schema');
    await waitForSchemaMode(page);
  });

  test('schema graph should be pannable', async ({ page }) => {
    const getViewportTransform = async () => {
      return page.evaluate(() => {
        const rf = document.querySelector('.react-flow__viewport');
        return rf?.getAttribute('style') ?? '';
      });
    };

    const initialTransform = await getViewportTransform();

    // Pan the graph by dragging the pane
    const graphPane = page.locator('.react-flow__pane');
    await graphPane.click({ position: { x: 200, y: 200 } });

    // Drag to pan
    const boundingBox = await graphPane.boundingBox();
    if (boundingBox) {
      await page.mouse.move(boundingBox.x + 200, boundingBox.y + 200);
      await page.mouse.down();
      await page.mouse.move(boundingBox.x + 300, boundingBox.y + 300);
      await page.mouse.up();
    }

    await page.waitForTimeout(200);

    const newTransform = await getViewportTransform();

    // Transform should have changed (panned)
    expect(newTransform).not.toBe(initialTransform);
  });

  test('schema graph should be zoomable', async ({ page }) => {
    const getViewportTransform = async () => {
      return page.evaluate(() => {
        const rf = document.querySelector('.react-flow__viewport');
        return rf?.getAttribute('style') ?? '';
      });
    };

    const initialTransform = await getViewportTransform();

    // Zoom using keyboard shortcut (= for zoom in)
    const graphPane = page.locator('.react-flow__pane');
    await graphPane.click();
    await page.keyboard.press('=');

    await page.waitForTimeout(300);

    const newTransform = await getViewportTransform();

    // Transform should have changed (zoomed)
    expect(newTransform).not.toBe(initialTransform);
  });

  test('clicking pane should clear any selection', async ({ page }) => {
    // Click on pane
    const graphPane = page.locator('.react-flow__pane');
    await graphPane.click();

    // Should not crash, graph should still be visible
    await expect(page.locator('.react-flow')).toBeVisible();
  });
});

test.describe('Schema Mode - Minimap', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/?mode=schema');
    await waitForSchemaMode(page);
  });

  test('should display minimap in schema mode', async ({ page }) => {
    // Minimap should be visible by default
    const minimap = page.locator('.react-flow__minimap');
    await expect(minimap).toBeVisible({ timeout: 10000 });
  });

  test('minimap toggle should work in schema mode', async ({ page }) => {
    // Press M to toggle minimap
    await page.keyboard.press('m');
    await page.waitForTimeout(300);

    // Minimap should be hidden
    const minimap = page.locator('.react-flow__minimap');
    await expect(minimap).not.toBeVisible();

    // Press M again to show
    await page.keyboard.press('m');
    await page.waitForTimeout(300);

    await expect(minimap).toBeVisible();
  });
});

test.describe('Schema Mode - Error Handling', () => {
  test('should not show console errors when switching modes', async ({ page }) => {
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
    await page.waitForSelector('.react-flow', { timeout: 30000 });

    // Switch to schema mode
    await switchToSchemaMode(page);

    // Wait for everything to settle
    await page.waitForTimeout(1000);

    // Switch back to data mode
    await getDataModeToggle(page).click();
    await page.waitForSelector('[data-testid="react-flow-wrapper"]', { timeout: 30000 });

    // Wait for everything to settle
    await page.waitForTimeout(1000);

    // Filter out known acceptable errors
    const criticalErrors = errors.filter(
      (e) =>
        !e.includes('Neo4j') &&
        !e.includes('ECONNREFUSED') &&
        !e.includes('hydration') &&
        !e.includes('ResizeObserver')
    );

    // Should have no critical errors
    expect(criticalErrors).toHaveLength(0);
  });
});
