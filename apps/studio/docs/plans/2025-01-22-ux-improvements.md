# UX Improvements Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add VIEW_PRESETS v7.2.1 UI, Priority/Freshness filters, sync keyboard shortcuts, and enhance GlowingBorder with Nika-style states.

**Architecture:** Component-based approach using existing Zustand stores (filterStore has v7.2.1 methods ready), Radix UI for dropdowns, and extending current GlowingBorder with state variants.

**Tech Stack:** React 19, TypeScript 5.7, Tailwind CSS, Radix UI, Zustand 5

---

## Task 1: Create ViewPresetSelector Component

**Files:**
- Create: `src/components/sidebar/ViewPresetSelector.tsx`
- Modify: `src/components/sidebar/FilterSidebar.tsx:162-175`

**Step 1: Write the failing test**

```typescript
// src/components/sidebar/__tests__/ViewPresetSelector.test.tsx
import { render, screen, fireEvent } from '@testing-library/react';
import { ViewPresetSelector } from '../ViewPresetSelector';

describe('ViewPresetSelector', () => {
  it('renders all 9 VIEW_PRESETS', () => {
    render(<ViewPresetSelector />);
    expect(screen.getByText('Project Structure')).toBeInTheDocument();
    expect(screen.getByText('Translation Chain')).toBeInTheDocument();
    expect(screen.getByText('All Nodes')).toBeInTheDocument();
  });

  it('calls onSelect when preset is clicked', () => {
    const onSelect = jest.fn();
    render(<ViewPresetSelector onSelect={onSelect} />);
    fireEvent.click(screen.getByText('Project Structure'));
    expect(onSelect).toHaveBeenCalledWith('project-structure');
  });

  it('shows active preset with visual indicator', () => {
    render(<ViewPresetSelector activePresetId="locale-knowledge" />);
    const activeItem = screen.getByText('Locale Knowledge').closest('button');
    expect(activeItem).toHaveClass('bg-white/[0.1]');
  });
});
```

**Step 2: Run test to verify it fails**

Run: `npm test -- --testPathPattern=ViewPresetSelector`
Expected: FAIL with "Cannot find module '../ViewPresetSelector'"

**Step 3: Write minimal implementation**

```typescript
// src/components/sidebar/ViewPresetSelector.tsx
'use client';

import { cn } from '@/lib/utils';
import { VIEW_PRESETS, type ViewPreset } from '@/lib/filterAdapter';
import { useFilterStore } from '@/stores/filterStore';
import { Sparkles } from 'lucide-react';

interface ViewPresetSelectorProps {
  className?: string;
  onSelect?: (presetId: string) => void;
  activePresetId?: string;
}

export function ViewPresetSelector({ className, onSelect, activePresetId }: ViewPresetSelectorProps) {
  const { applyViewPreset, activePresetId: storeActiveId } = useFilterStore();
  const activeId = activePresetId ?? storeActiveId;

  const handleSelect = (preset: ViewPreset) => {
    applyViewPreset(preset.id);
    onSelect?.(preset.id);
  };

  return (
    <div className={cn('space-y-1', className)}>
      <div className="flex items-center gap-2 px-3 py-2 text-xs text-white/40">
        <Sparkles className="w-3.5 h-3.5" />
        <span className="uppercase tracking-wider font-medium">Quick Views</span>
      </div>
      <div className="grid grid-cols-3 gap-1 px-2">
        {VIEW_PRESETS.map((preset) => (
          <button
            key={preset.id}
            onClick={() => handleSelect(preset)}
            className={cn(
              'flex flex-col items-center gap-1.5 px-2 py-2.5 rounded-lg text-center',
              'transition-all duration-200',
              activeId === preset.id
                ? 'bg-white/[0.1] border border-white/[0.15] text-white'
                : 'bg-white/[0.03] border border-transparent text-white/60 hover:bg-white/[0.06] hover:text-white/80'
            )}
          >
            <span className="text-lg">{preset.icon}</span>
            <span className="text-[10px] font-medium leading-tight truncate w-full">
              {preset.name.split(' ')[0]}
            </span>
            {preset.shortcut && (
              <kbd className="text-[9px] px-1.5 py-0.5 bg-white/[0.08] rounded text-white/40 font-mono">
                {preset.shortcut}
              </kbd>
            )}
          </button>
        ))}
      </div>
    </div>
  );
}
```

**Step 4: Run test to verify it passes**

Run: `npm test -- --testPathPattern=ViewPresetSelector`
Expected: PASS

**Step 5: Commit**

```bash
git add src/components/sidebar/ViewPresetSelector.tsx src/components/sidebar/__tests__/
git commit -m "feat(sidebar): add ViewPresetSelector for quick view presets

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Task 2: Add Priority/Freshness Filters to FilterSidebar

**Files:**
- Modify: `src/components/sidebar/FilterSidebar.tsx:356-378`
- Create: `src/components/sidebar/PriorityFreshnessFilters.tsx`

**Step 1: Write the failing test**

```typescript
// src/components/sidebar/__tests__/PriorityFreshnessFilters.test.tsx
import { render, screen, fireEvent } from '@testing-library/react';
import { PriorityFreshnessFilters } from '../PriorityFreshnessFilters';

describe('PriorityFreshnessFilters', () => {
  it('renders priority chips', () => {
    render(<PriorityFreshnessFilters />);
    expect(screen.getByText('Critical')).toBeInTheDocument();
    expect(screen.getByText('High')).toBeInTheDocument();
    expect(screen.getByText('Medium')).toBeInTheDocument();
    expect(screen.getByText('Low')).toBeInTheDocument();
  });

  it('renders freshness chips', () => {
    render(<PriorityFreshnessFilters />);
    expect(screen.getByText('Realtime')).toBeInTheDocument();
    expect(screen.getByText('Hourly')).toBeInTheDocument();
    expect(screen.getByText('Daily')).toBeInTheDocument();
    expect(screen.getByText('Static')).toBeInTheDocument();
  });

  it('toggles priority filter on click', () => {
    render(<PriorityFreshnessFilters />);
    const criticalChip = screen.getByText('Critical');
    fireEvent.click(criticalChip);
    expect(criticalChip.closest('button')).toHaveClass('bg-red-500/20');
  });
});
```

**Step 2: Run test to verify it fails**

Run: `npm test -- --testPathPattern=PriorityFreshnessFilters`
Expected: FAIL with "Cannot find module"

**Step 3: Write minimal implementation**

```typescript
// src/components/sidebar/PriorityFreshnessFilters.tsx
'use client';

import { cn } from '@/lib/utils';
import { useFilterStore } from '@/stores/filterStore';
import type { Priority, Freshness } from '@/lib/filterAdapter';
import { AlertCircle, Clock } from 'lucide-react';

const PRIORITY_CONFIG: Record<Priority, { label: string; color: string }> = {
  critical: { label: 'Critical', color: 'red' },
  high: { label: 'High', color: 'orange' },
  medium: { label: 'Medium', color: 'yellow' },
  low: { label: 'Low', color: 'green' },
};

const FRESHNESS_CONFIG: Record<Freshness, { label: string; color: string }> = {
  realtime: { label: 'Realtime', color: 'cyan' },
  hourly: { label: 'Hourly', color: 'blue' },
  daily: { label: 'Daily', color: 'indigo' },
  static: { label: 'Static', color: 'gray' },
};

export function PriorityFreshnessFilters() {
  const { priorityFilter, freshnessFilter, setPriorityFilter, setFreshnessFilter } = useFilterStore();

  const togglePriority = (p: Priority) => {
    const current = new Set(priorityFilter);
    if (current.has(p)) {
      current.delete(p);
    } else {
      current.add(p);
    }
    setPriorityFilter(Array.from(current));
  };

  const toggleFreshness = (f: Freshness) => {
    const current = new Set(freshnessFilter);
    if (current.has(f)) {
      current.delete(f);
    } else {
      current.add(f);
    }
    setFreshnessFilter(Array.from(current));
  };

  return (
    <div className="space-y-4">
      {/* Priority */}
      <div>
        <div className="flex items-center gap-2 px-3 py-2 text-xs text-white/40">
          <AlertCircle className="w-3.5 h-3.5" />
          <span className="uppercase tracking-wider font-medium">Priority</span>
        </div>
        <div className="flex flex-wrap gap-1.5 px-3">
          {(Object.entries(PRIORITY_CONFIG) as [Priority, typeof PRIORITY_CONFIG['critical']][]).map(([key, { label, color }]) => (
            <button
              key={key}
              onClick={() => togglePriority(key)}
              className={cn(
                'px-2.5 py-1 rounded-full text-xs font-medium transition-all',
                'border',
                priorityFilter.includes(key)
                  ? `bg-${color}-500/20 border-${color}-500/50 text-${color}-300`
                  : 'bg-white/[0.03] border-white/[0.08] text-white/50 hover:bg-white/[0.06]'
              )}
              style={{
                backgroundColor: priorityFilter.includes(key) ? `var(--color-${color}-500, oklch(0.6 0.2 ${color === 'red' ? 25 : color === 'orange' ? 40 : color === 'yellow' ? 80 : 140}))20` : undefined,
                borderColor: priorityFilter.includes(key) ? `var(--color-${color}-500, oklch(0.6 0.2 0))50` : undefined,
              }}
            >
              {label}
            </button>
          ))}
        </div>
      </div>

      {/* Freshness */}
      <div>
        <div className="flex items-center gap-2 px-3 py-2 text-xs text-white/40">
          <Clock className="w-3.5 h-3.5" />
          <span className="uppercase tracking-wider font-medium">Freshness</span>
        </div>
        <div className="flex flex-wrap gap-1.5 px-3">
          {(Object.entries(FRESHNESS_CONFIG) as [Freshness, typeof FRESHNESS_CONFIG['realtime']][]).map(([key, { label }]) => (
            <button
              key={key}
              onClick={() => toggleFreshness(key)}
              className={cn(
                'px-2.5 py-1 rounded-full text-xs font-medium transition-all',
                'border',
                freshnessFilter.includes(key)
                  ? 'bg-novanet-500/20 border-novanet-500/50 text-novanet-300'
                  : 'bg-white/[0.03] border-white/[0.08] text-white/50 hover:bg-white/[0.06]'
              )}
            >
              {label}
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}
```

**Step 4: Run test to verify it passes**

Run: `npm test -- --testPathPattern=PriorityFreshnessFilters`
Expected: PASS

**Step 5: Commit**

```bash
git add src/components/sidebar/PriorityFreshnessFilters.tsx src/components/sidebar/__tests__/
git commit -m "feat(sidebar): add priority and freshness filter chips

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Task 3: Integrate New Components into FilterSidebar

**Files:**
- Modify: `src/components/sidebar/FilterSidebar.tsx`

**Step 1: Write the failing E2E test**

```typescript
// e2e/filter-presets.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Filter Presets UI', () => {
  test('view presets are visible in sidebar', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('.react-flow', { timeout: 10000 });

    // ViewPresetSelector should be visible
    await expect(page.locator('text=Quick Views')).toBeVisible();
    await expect(page.locator('text=Project')).toBeVisible();
    await expect(page.locator('text=Translation')).toBeVisible();
  });

  test('priority filters are visible', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Priority')).toBeVisible();
    await expect(page.locator('text=Critical')).toBeVisible();
    await expect(page.locator('text=High')).toBeVisible();
  });

  test('clicking preset changes filter state', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('.react-flow', { timeout: 10000 });

    // Click on "Locale" preset
    await page.click('text=Locale');

    // Should filter to locale-related nodes
    await page.waitForTimeout(500);
    await expect(page.locator('.react-flow__node')).toBeVisible();
  });
});
```

**Step 2: Run E2E test to verify it fails**

Run: `npx playwright test filter-presets.spec.ts`
Expected: FAIL (Quick Views not visible)

**Step 3: Integrate components**

```typescript
// Modify src/components/sidebar/FilterSidebar.tsx
// Add imports at top:
import { ViewPresetSelector } from './ViewPresetSelector';
import { PriorityFreshnessFilters } from './PriorityFreshnessFilters';

// Add after Header section (around line 175), before Search:
{/* Quick View Presets */}
<div className="border-b border-white/[0.06]">
  <ViewPresetSelector />
</div>

{/* Priority & Freshness Filters */}
<div className="border-b border-white/[0.06] py-3">
  <PriorityFreshnessFilters />
</div>
```

**Step 4: Run E2E test to verify it passes**

Run: `npx playwright test filter-presets.spec.ts`
Expected: PASS

**Step 5: Commit**

```bash
git add src/components/sidebar/FilterSidebar.tsx e2e/filter-presets.spec.ts
git commit -m "feat(sidebar): integrate ViewPresetSelector and PriorityFreshnessFilters

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Task 4: Synchronize Keyboard Shortcuts with VIEW_PRESETS

**Files:**
- Modify: `src/components/ui/KeyboardShortcuts.tsx:60-72`
- Modify: `src/lib/keyboard.ts` (add VIEW_PRESET shortcut handler)

**Step 1: Write the failing test**

```typescript
// src/lib/__tests__/keyboard.test.ts
import { handleViewPresetShortcut } from '../keyboard';

describe('handleViewPresetShortcut', () => {
  it('maps 1 to project-structure preset', () => {
    const result = handleViewPresetShortcut('1');
    expect(result).toBe('project-structure');
  });

  it('maps 2 to translation-chain preset', () => {
    const result = handleViewPresetShortcut('2');
    expect(result).toBe('translation-chain');
  });

  it('maps 0 to all-nodes preset', () => {
    const result = handleViewPresetShortcut('0');
    expect(result).toBe('all-nodes');
  });

  it('returns null for non-shortcut keys', () => {
    const result = handleViewPresetShortcut('a');
    expect(result).toBeNull();
  });
});
```

**Step 2: Run test to verify it fails**

Run: `npm test -- --testPathPattern=keyboard.test`
Expected: FAIL

**Step 3: Update keyboard.ts and KeyboardShortcuts.tsx**

```typescript
// Add to src/lib/keyboard.ts
import { VIEW_PRESETS, getViewPresetByShortcut } from './filterAdapter';

export function handleViewPresetShortcut(key: string): string | null {
  const preset = getViewPresetByShortcut(key);
  return preset?.id ?? null;
}
```

```typescript
// Update src/components/ui/KeyboardShortcuts.tsx presets section (lines 60-72)
const SHORTCUT_CATEGORIES: ShortcutCategory[] = [
  // ... navigation and view sections stay the same ...
  {
    id: 'presets',
    title: 'Presets',
    icon: <Hash className="w-4 h-4" />,
    shortcuts: [
      { keys: ['1'], description: 'Project Structure' },
      { keys: ['2'], description: 'Translation Chain' },
      { keys: ['3'], description: 'Locale Knowledge' },
      { keys: ['4'], description: 'Concept Network' },
      { keys: ['5'], description: 'Prompts & Rules' },
      { keys: ['6'], description: 'SEO & GEO' },
      { keys: ['7'], description: 'High Priority' },
      { keys: ['8'], description: 'Realtime Content' },
      { keys: ['0'], description: 'All Nodes' },
    ],
  },
  // ... graph section stays the same ...
];
```

**Step 4: Run test to verify it passes**

Run: `npm test -- --testPathPattern=keyboard.test`
Expected: PASS

**Step 5: Commit**

```bash
git add src/lib/keyboard.ts src/components/ui/KeyboardShortcuts.tsx src/lib/__tests__/
git commit -m "fix(shortcuts): sync keyboard shortcuts with VIEW_PRESETS v7.2.1

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Task 5: Enhance GlowingBorder with State Variants

**Files:**
- Modify: `src/components/ui/GlowingBorder.tsx`
- Modify: `src/app/globals.css` (add new animations)

**Step 1: Write the failing test**

```typescript
// src/components/ui/__tests__/GlowingBorder.test.tsx
import { render, screen } from '@testing-library/react';
import { GlowingBorder } from '../GlowingBorder';

describe('GlowingBorder states', () => {
  it('renders error state with red glow', () => {
    const { container } = render(
      <GlowingBorder color="#8b5cf6" state="error">
        Content
      </GlowingBorder>
    );
    const glowLayer = container.querySelector('[class*="error"]');
    expect(glowLayer).toBeInTheDocument();
  });

  it('renders running state with pulse animation', () => {
    const { container } = render(
      <GlowingBorder color="#8b5cf6" state="running">
        Content
      </GlowingBorder>
    );
    expect(container.querySelector('[class*="animate-pulse-fast"]')).toBeInTheDocument();
  });

  it('renders pending state with subtle animation', () => {
    const { container } = render(
      <GlowingBorder color="#8b5cf6" state="pending">
        Content
      </GlowingBorder>
    );
    expect(container.querySelector('[class*="animate-pulse-slow"]')).toBeInTheDocument();
  });
});
```

**Step 2: Run test to verify it fails**

Run: `npm test -- --testPathPattern=GlowingBorder.test`
Expected: FAIL

**Step 3: Update GlowingBorder.tsx**

```typescript
// Add state prop to GlowingBorderProps
export type GlowingBorderState = 'idle' | 'running' | 'pending' | 'error' | 'success' | 'partial';

export interface GlowingBorderProps {
  // ... existing props ...
  /** State for contextual styling */
  state?: GlowingBorderState;
}

// Add state color mapping
const STATE_COLORS: Record<GlowingBorderState, string> = {
  idle: 'inherit',
  running: '#3b82f6',    // blue
  pending: '#fbbf24',    // amber
  error: '#ef4444',      // red
  success: '#10b981',    // emerald
  partial: '#f97316',    // orange
};

// Update component to use state
export function GlowingBorder({
  // ... existing props ...
  state = 'idle',
}: GlowingBorderProps) {
  const stateColor = state !== 'idle' ? STATE_COLORS[state] : null;
  const effectiveColor = stateColor || primary;

  // Add state-specific classes
  const stateClasses = {
    running: 'animate-pulse-fast',
    pending: 'animate-pulse-slow',
    error: 'animate-shake',
    success: 'animate-success-flash',
    partial: 'animate-partial-glow',
  };

  // ... rest of component with effectiveColor instead of primary ...
}
```

**Step 4: Add animations to globals.css**

```css
/* Add to src/app/globals.css */
@keyframes pulse-fast {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.6; }
}

@keyframes pulse-slow {
  0%, 100% { opacity: 0.8; }
  50% { opacity: 0.4; }
}

@keyframes shake {
  0%, 100% { transform: translateX(0); }
  25% { transform: translateX(-2px); }
  75% { transform: translateX(2px); }
}

@keyframes success-flash {
  0% { box-shadow: 0 0 20px rgba(16, 185, 129, 0.8); }
  100% { box-shadow: 0 0 10px rgba(16, 185, 129, 0.3); }
}

.animate-pulse-fast { animation: pulse-fast 0.8s ease-in-out infinite; }
.animate-pulse-slow { animation: pulse-slow 2s ease-in-out infinite; }
.animate-shake { animation: shake 0.3s ease-in-out; }
.animate-success-flash { animation: success-flash 0.5s ease-out; }
```

**Step 5: Run test to verify it passes**

Run: `npm test -- --testPathPattern=GlowingBorder.test`
Expected: PASS

**Step 6: Commit**

```bash
git add src/components/ui/GlowingBorder.tsx src/app/globals.css src/components/ui/__tests__/
git commit -m "feat(GlowingBorder): add state variants (error/running/pending/success)

Nika-inspired state management for contextual visual feedback.

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Task 6: Final Integration and E2E Tests

**Files:**
- Modify: `e2e/app.spec.ts`
- Run full E2E suite

**Step 1: Add comprehensive E2E tests**

```typescript
// Add to e2e/app.spec.ts
test.describe('NovaNet Visualizer - Filter UI', () => {
  test('view presets are accessible', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('.react-flow');

    // Quick Views section visible
    await expect(page.locator('text=Quick Views')).toBeVisible();

    // At least 9 preset buttons (grid of presets)
    const presetButtons = page.locator('[class*="grid"] button');
    await expect(presetButtons).toHaveCount(9);
  });

  test('keyboard shortcut 1 applies project structure preset', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('.react-flow');

    // Press 1
    await page.keyboard.press('1');
    await page.waitForTimeout(500);

    // Graph should still be visible (no crash)
    await expect(page.locator('.react-flow')).toBeVisible();
  });

  test('priority filter chips are interactive', async ({ page }) => {
    await page.goto('/');

    const criticalChip = page.locator('button:has-text("Critical")');
    await criticalChip.click();

    // Should have visual feedback (active state)
    await expect(criticalChip).toHaveCSS('background-color', /rgba/);
  });
});
```

**Step 2: Run full E2E suite**

Run: `npx playwright test`
Expected: ALL PASS

**Step 3: Final commit**

```bash
git add e2e/app.spec.ts
git commit -m "test(e2e): add comprehensive filter UI tests

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Summary

| Task | Component | Status |
|------|-----------|--------|
| 1 | ViewPresetSelector | Pending |
| 2 | PriorityFreshnessFilters | Pending |
| 3 | FilterSidebar Integration | Pending |
| 4 | Keyboard Shortcuts Sync | Pending |
| 5 | GlowingBorder States | Pending |
| 6 | E2E Tests | Pending |

**Dependencies:**
- Task 3 depends on Tasks 1, 2
- Task 6 depends on all other tasks

**Execution:** Use subagent-driven-development for Tasks 1-5, then run final E2E tests.
