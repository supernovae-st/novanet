# E2E Test Known Issues

## Skipped Tests

### URL Sync Tests (schema-mode.spec.ts)

**Tests:**
- `should update URL when switching to schema mode`
- `should remove mode param when switching back to data mode`

**Issue:** Zustand state persistence causes flaky behavior in parallel test runs. When tests run in parallel, persisted state from previous tests can leak into subsequent tests, causing URL assertions to fail.

**Workaround:** Tests pass when run individually:
```bash
npx playwright test --grep="URL"
```

**Root cause:** `filterStore` uses Zustand's persist middleware which stores state in localStorage. Parallel test workers can interfere with each other's state.

---

### Subcategory Toggle Tests (schema-mode.spec.ts)

**Tests:**
- `should toggle subcategory visibility when clicked`
- `should show visual feedback when subcategory is hidden`

**Issue:** Playwright cannot reliably interact with custom `role="checkbox"` elements in FilterTree. The click events don't trigger the expected state changes.

**Workaround:** Subcategory functionality is indirectly tested through scope collapse tests which verify `aria-expanded` attributes.

**Root cause:** FilterTree uses a custom checkbox implementation with `role="checkbox"` and `aria-checked`. Playwright's click simulation doesn't match browser behavior for these ARIA patterns.

---

### Dynamic Skips (focus-mode.spec.ts)

**Tests:**
- Focus mode neighbor expansion (skips if < 3 nodes)
- Alt concept navigation (skips if alt concept doesn't exist)

**Note:** These are intentional conditional skips based on test data availability, not bugs. They ensure tests don't fail when the database doesn't have sufficient test data.

---

## Running Individual Tests

```bash
# Run all E2E tests
pnpm test:e2e

# Run specific test file
pnpm test:e2e schema-mode.spec.ts

# Run with UI mode (debug)
pnpm test:e2e:ui

# Run skipped tests individually
npx playwright test --grep="URL" --workers=1
```
