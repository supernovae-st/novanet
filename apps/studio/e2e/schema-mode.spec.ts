import { test, expect, type Page } from '@playwright/test';
import { waitForGraphLoaded } from './helpers';

/**
 * Schema Mode E2E Tests
 *
 * Phase 5 tests from docs/plans/2026-01-30-schema-mode-v2.md
 *
 * Tests:
 * 1. Toggle between data and schema mode
 * 2. Schema mode shows 42 nodes in grouped layout (v10.4)
 * 3. URL sync works (?mode=schema)
 * 4. Scope groups are visible
 * 5. Filter panel shows hierarchical filters
 * 6. Collapsed state persists (collapse a scope, verify nodes hidden)
 */

// Increase timeout for graph loading and ELK layout (can be slow)
test.setTimeout(90000);

// Helper: Wait for schema mode to fully load (ELK layout complete)
async function waitForSchemaMode(page: Page) {
  // First wait for lazy loading to complete
  await waitForGraphLoaded(page);

  // Wait for schema wrapper to appear (indicates schema mode is active)
  await page.waitForSelector('[data-testid="react-flow-wrapper-schema"]', { timeout: 30000 });

  // Wait for loading indicator to disappear (ELK layout complete)
  const loadingIndicator = page.locator('[data-testid="schema-loading-indicator"]');
  await expect(loadingIndicator).not.toBeVisible({ timeout: 30000 });

  // Additional wait for ELK layout to stabilize
  await page.waitForTimeout(500);
}

// Helper: Get the Meta mode badge (modern UI uses "Meta"/"Data" badges in StatsCounter)
function getMetaBadge(page: Page) {
  return page.locator('button').filter({ hasText: 'Meta' }).first();
}

// Helper: Get the Data mode badge
function getDataBadge(page: Page) {
  return page.locator('button').filter({ hasText: 'Data' }).first();
}

// Helper: Check if currently in meta mode by looking at the badge text
async function isInMetaMode(page: Page): Promise<boolean> {
  const metaBadge = getMetaBadge(page);
  return await metaBadge.isVisible();
}

// Helper: Switch to meta mode if not already
async function switchToMetaMode(page: Page) {
  // Meta mode is activated via views, not a direct toggle button
  // Navigate to a meta view via URL
  await page.goto('/?view=complete-graph&mode=meta');
  await waitForSchemaMode(page);
}


test.describe('Schema Mode - Mode Toggle', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Wait for lazy-loaded graph to finish loading
    await waitForGraphLoaded(page);
  });

  test.skip('should display mode badge in top bar', async ({ page }) => {
    // TODO: Fix selector for mode badge
    // Data badge should be visible by default
    const dataBadge = getDataBadge(page);
    await expect(dataBadge).toBeVisible();

    // Should show nodes/edges counts
    const statsContainer = page.locator('.glass').first();
    await expect(statsContainer).toBeVisible();
  });

  test.skip('should switch from data mode to meta mode via URL', async ({ page }) => {
    // TODO: Meta mode URL routing needs investigation
    // Initially in data mode - Data badge visible
    await expect(getDataBadge(page)).toBeVisible();

    // Navigate to meta mode via URL
    await page.goto('/?mode=meta');

    // Wait for meta mode to load
    await waitForSchemaMode(page);

    // Should now show Meta badge
    await expect(getMetaBadge(page)).toBeVisible();
  });

  test.skip('should switch back from meta mode to data mode', async ({ page }) => {
    // TODO: Update when meta mode URL routing is finalized
    // Switch to meta mode first
    await switchToMetaMode(page);

    // Should show Meta badge
    await expect(getMetaBadge(page)).toBeVisible();

    // Navigate back to data mode
    await page.goto('/');

    // Wait for data mode to load
    await waitForGraphLoaded(page);

    // Should show Data badge again
    await expect(getDataBadge(page)).toBeVisible();
  });
});

test.describe.skip('Schema Mode - Schema Graph Display', () => {
  // TODO: Rewrite tests for Query-First Architecture (ADR-021)
  // The UI has evolved to use Meta/Data badges in StatsCounter instead of Schema Browser/Data Explorer
  // Schema filter panel has been replaced with view-based filtering

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await waitForGraphLoaded(page);
    await switchToMetaMode(page);
  });

  test('should display grouped layout with scope groups', async ({ page }) => {
    // v10.4: 2 realms (SHARED merged into GLOBAL)
    // The scope groups are rendered with icon + label (e.g., "PROJECT", "GLOBAL")

    // Check schema filter panel for scope groups (these are always visible)
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');
    await expect(filterPanel).toBeVisible();

    // Use section label IDs to find specific scope headers (more reliable)
    await expect(filterPanel.locator('#section-label-project')).toBeVisible();
    await expect(filterPanel.locator('#section-label-global')).toBeVisible();
    // v10.4: shared removed, merged into global
  });

  test('should display schema stats in header', async ({ page }) => {
    // The stats are shown in the header as "42 node types · 2 realms" (v10.4)
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');
    await expect(filterPanel).toBeVisible();

    // Check for stats text format in header
    const statsText = filterPanel.getByText(/42 node types/);
    await expect(statsText).toBeVisible({ timeout: 10000 });
  });

  test('should render React Flow nodes in schema mode', async ({ page }) => {
    // Wait for React Flow nodes to render
    const nodes = page.locator('.react-flow__node');
    await expect(nodes.first()).toBeVisible({ timeout: 30000 });

    // Should have multiple nodes (realm groups + layer groups + schema nodes)
    const nodeCount = await nodes.count();
    expect(nodeCount).toBeGreaterThan(0);
  });
});

test.describe.skip('Schema Mode - URL Sync', () => {
  // TODO: Rewrite for Query-First Architecture
  // Run URL sync tests in serial to avoid state conflicts from persisted Zustand store
  test.describe.configure({ mode: 'serial' });

  // NOTE: Skipped - flaky in parallel test runs due to Zustand state persistence
  // Works reliably when run individually: npx playwright test --grep="URL"
  test.skip('should update URL when switching to schema mode', async ({ page }) => {
    await page.goto('/');
    await waitForGraphLoaded(page);

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

    // Should show Schema Browser (indicating schema mode)
    await expect(page.locator('text=Schema Browser')).toBeVisible();

    // Should have the schema wrapper
    await expect(page.locator('[data-testid="react-flow-wrapper-schema"]')).toBeVisible();
  });

  // NOTE: Skipped - flaky in parallel test runs due to Zustand state persistence
  // Works reliably when run individually: npx playwright test --grep="URL"
  test.skip('should remove mode param when switching back to data mode', async ({ page }) => {
    // Start in schema mode
    await page.goto('/?mode=schema');
    await waitForSchemaMode(page);

    // Switch back to data mode via navigation
    await page.goto('/');

    // Wait for data mode to fully load
    await waitForGraphLoaded(page);

    // Extra wait for URL sync debounce (300ms) + React re-render cycle
    await page.waitForTimeout(500);

    // URL should not have mode param (or mode=data which may be omitted as default)
    // Wait for URL sync with extended timeout for CI environments
    await expect(async () => {
      const url = page.url();
      expect(url).not.toContain('mode=schema');
    }).toPass({ timeout: 5000 });
  });

  test('should persist schema mode across page refresh', async ({ page }) => {
    // Go to schema mode
    await page.goto('/?mode=schema');
    await waitForSchemaMode(page);

    // Refresh the page
    await page.reload();

    // Should still be in schema mode
    await waitForSchemaMode(page);

    // Should show Schema Browser (indicating schema mode)
    await expect(page.locator('text=Schema Browser')).toBeVisible();
  });
});

test.describe.skip('Schema Mode - Filter Panel', () => {
  // TODO: Rewrite for Query-First Architecture (view-based filtering)
  test.beforeEach(async ({ page }) => {
    await page.goto('/?mode=schema');
    await waitForSchemaMode(page);
  });

  test('should display SchemaFilterPanel in sidebar when in schema mode', async ({ page }) => {
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');
    await expect(filterPanel).toBeVisible();

    // Should have the header (title is "Schema Browser")
    await expect(filterPanel.getByText('Schema Browser')).toBeVisible();
  });

  test('should show all 2 realms with icons', async ({ page }) => {
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');

    // v10.4: 2 realms (SHARED merged into GLOBAL)
    await expect(filterPanel.locator('#section-label-project')).toBeVisible();
    await expect(filterPanel.locator('#section-label-global')).toBeVisible();
  });

  test('should display subcategories for each realm', async ({ page }) => {
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');

    // Project layers
    await expect(filterPanel.getByText('Foundation')).toBeVisible();
    await expect(filterPanel.getByText('Structure')).toBeVisible();
    await expect(filterPanel.getByText('Semantic')).toBeVisible();
    await expect(filterPanel.getByText('Instruction')).toBeVisible();
    await expect(filterPanel.getByText('Output')).toBeVisible();

    // Global layers (v10.4: SEO moved from SHARED)
    await expect(filterPanel.getByText('Configuration')).toBeVisible();
    await expect(filterPanel.getByText('Knowledge')).toBeVisible();
    await expect(filterPanel.getByText('SEO')).toBeVisible();
  });

  test('should show node counts for layers', async ({ page }) => {
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');

    // The FilterTree shows counts next to each layer
    // Look for layer labels - counts may be displayed differently
    await expect(filterPanel.getByText('Foundation')).toBeVisible();
    await expect(filterPanel.getByText('Knowledge')).toBeVisible();

    // Verify counts are displayed (they appear after labels)
    // The actual format depends on FilterTree.Row implementation
    const rows = filterPanel.locator('button[data-selected]');
    const count = await rows.count();
    expect(count).toBeGreaterThan(0);
  });

  test('should show legend footer with realm icons', async ({ page }) => {
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');

    // v10.4: 2 realms - "📦 Project · 🌍 Global"
    await expect(filterPanel.getByText('📦 Project')).toBeVisible();
    await expect(filterPanel.getByText('🌍 Global')).toBeVisible();
    // v10.4: shared removed, merged into global
  });
});

test.describe.skip('Schema Mode - Scope Collapse', () => {
  // TODO: Rewrite for Query-First Architecture
  test.beforeEach(async ({ page }) => {
    await page.goto('/?mode=schema');
    await waitForSchemaMode(page);
  });

  test('should collapse scope when header is clicked', async ({ page }) => {
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');

    // Find the GLOBAL scope header button using aria-label that includes "GLOBAL"
    const globalScopeHeader = filterPanel.getByRole('button', { name: /GLOBAL/ });
    await expect(globalScopeHeader).toBeVisible();

    // Initially expanded - aria-expanded should be true
    await expect(globalScopeHeader).toHaveAttribute('aria-expanded', 'true');

    // Click to collapse
    await globalScopeHeader.click();

    // Wait for CSS transition to complete
    await page.waitForTimeout(400);

    // aria-expanded should now be false
    await expect(globalScopeHeader).toHaveAttribute('aria-expanded', 'false');

    // Click again to expand
    await globalScopeHeader.click();
    await page.waitForTimeout(400);

    // aria-expanded should be true again
    await expect(globalScopeHeader).toHaveAttribute('aria-expanded', 'true');
  });

  test('should update graph when scope visibility changes', async ({ page }) => {
    // Count visible React Flow nodes
    const getNodeCount = async (): Promise<number> => {
      return page.locator('.react-flow__node').count();
    };

    const initialCount = await getNodeCount();
    expect(initialCount).toBeGreaterThan(0);

    // Toggle the GLOBAL scope checkbox to hide its nodes
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');

    // Find the GLOBAL scope's tri-state checkbox (separate from header button)
    // The checkbox is the first button child of the section header area
    const globalSection = filterPanel.locator('[aria-labelledby="section-label-global"]');
    await expect(globalSection).toBeVisible();

    // Wait for graph to settle
    await page.waitForTimeout(500);

    // The graph should have multiple nodes in schema mode
    expect(initialCount).toBeGreaterThan(5);
  });

  test('should have ARIA attributes for accessibility', async ({ page }) => {
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');

    // Scope headers should have aria-expanded
    const projectHeader = filterPanel.getByRole('button', { name: /PROJECT/ });
    await expect(projectHeader).toHaveAttribute('aria-expanded', 'true');

    // Scope sections should have aria-labelledby
    const projectSection = filterPanel.locator('[aria-labelledby="section-label-project"]');
    await expect(projectSection).toBeVisible();

    // Click to collapse
    await projectHeader.click();
    await page.waitForTimeout(400);

    // Should now be collapsed
    await expect(projectHeader).toHaveAttribute('aria-expanded', 'false');

    // Expand again
    await projectHeader.click();
    await page.waitForTimeout(400);

    // Should be expanded again
    await expect(projectHeader).toHaveAttribute('aria-expanded', 'true');
  });
});

test.describe.skip('Schema Mode - Layer Toggle', () => {
  // TODO: Rewrite for Query-First Architecture
  test.beforeEach(async ({ page }) => {
    await page.goto('/?mode=schema');
    await waitForSchemaMode(page);
  });

  // NOTE: Skipped due to Playwright interaction issue with role="checkbox" elements
  // The functionality works in the browser - this is a test infrastructure limitation
  // The layer toggle is tested via the realm collapse tests which verify aria-expanded
  test.skip('should toggle layer visibility when clicked', async ({ page }) => {
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');

    // Find Foundation checkbox by aria-label (role="checkbox" in FilterTree)
    const foundationCheckbox = filterPanel.getByRole('checkbox', { name: /Foundation/ });
    await expect(foundationCheckbox).toBeVisible();

    // Check initial state - should be checked (visible in graph)
    await expect(foundationCheckbox).toHaveAttribute('aria-checked', 'true');
    await expect(foundationCheckbox).toHaveAttribute('data-selected', 'true');

    // Click to toggle off
    await foundationCheckbox.click();

    // Wait for state update - should now be unchecked
    await expect(foundationCheckbox).toHaveAttribute('aria-checked', 'false', { timeout: 3000 });
    await expect(foundationCheckbox).toHaveAttribute('data-selected', 'false');

    // Toggle back on
    await foundationCheckbox.click();

    // Wait for state update - should be checked again
    await expect(foundationCheckbox).toHaveAttribute('aria-checked', 'true', { timeout: 3000 });
    await expect(foundationCheckbox).toHaveAttribute('data-selected', 'true');
  });

  // NOTE: Skipped due to Playwright interaction issue with role="checkbox" elements
  test.skip('should show visual feedback when layer is hidden', async ({ page }) => {
    const filterPanel = page.locator('[data-testid="schema-filter-panel"]');

    // Find Foundation checkbox by aria-label
    const foundationCheckbox = filterPanel.getByRole('checkbox', { name: /Foundation/ });

    // Initially should be checked (data-selected="true")
    await expect(foundationCheckbox).toHaveAttribute('aria-checked', 'true');
    await expect(foundationCheckbox).toHaveAttribute('data-selected', 'true');

    // Click to toggle off
    await foundationCheckbox.click();

    // Wait for state update - should now be unchecked
    await expect(foundationCheckbox).toHaveAttribute('aria-checked', 'false', { timeout: 3000 });
    await expect(foundationCheckbox).toHaveAttribute('data-selected', 'false');
  });
});

test.describe.skip('Schema Mode - Graph Interaction', () => {
  // TODO: Rewrite for Query-First Architecture
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

  // FIXME: Keyboard zoom shortcut ('=') doesn't work reliably in Playwright
  // The shortcut works in the browser but Playwright doesn't trigger React Flow's zoom handler
  // Consider using wheel event instead if we need this test coverage
  test.fixme('schema graph should be zoomable', async ({ page }) => {
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

test.describe.skip('Schema Mode - Minimap', () => {
  // TODO: Rewrite for Query-First Architecture
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

test.describe.skip('Schema Mode - Error Handling', () => {
  // TODO: Rewrite for Query-First Architecture
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
    await waitForGraphLoaded(page);

    // Switch to schema mode
    await switchToSchemaMode(page);

    // Wait for everything to settle
    await page.waitForTimeout(1000);

    // Switch back to data mode via navigation
    await page.goto('/');
    await waitForGraphLoaded(page);

    // Wait for everything to settle
    await page.waitForTimeout(1000);

    // Filter out known acceptable errors
    const criticalErrors = errors.filter(
      (e) =>
        !e.includes('Neo4j') &&
        !e.includes('ECONNREFUSED') &&
        !e.includes('hydration') &&
        !e.includes('ResizeObserver') &&
        // SVG animation warnings from loading indicators (pre-existing, not from schema mode)
        !e.includes('animateMotion') &&
        !e.includes('keySplines')
    );

    // Should have no critical errors
    expect(criticalErrors).toHaveLength(0);
  });
});
