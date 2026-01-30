# Sidebar Refonte - Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Redesign the left sidebar to remove duplicates, improve space usage, and enhance UX with smooth accordions and Nika-style design.

**Architecture:** Remove Quick Views (doublon), merge Priority/Freshness into "Quick Filters", keep Node Types with count badges, add search with ⌘K hint.

**Tech Stack:** React 19, Zustand, Tailwind CSS, Lucide icons, Nika glassmorphism

---

## Task 1: Remove ViewPresetSelector from Sidebar

**Files:**
- Modify: `src/components/sidebar/FilterSidebar.tsx`
- Delete: `src/components/sidebar/ViewPresetSelector.tsx` (if no longer used elsewhere)

**Step 1: Remove import and usage**

In `FilterSidebar.tsx`, remove:
```tsx
// DELETE THIS LINE
import { ViewPresetSelector } from './ViewPresetSelector';
```

And remove the component usage in the JSX (around line 95-100):
```tsx
// DELETE THIS BLOCK
{/* Quick View Presets */}
<ViewPresetSelector />
```

**Step 2: Verify no other imports**

Run: `grep -r "ViewPresetSelector" src/`

If only FilterSidebar imports it, delete the file:
```bash
rm src/components/sidebar/ViewPresetSelector.tsx
```

**Step 3: Update exports if needed**

Check `src/components/sidebar/index.ts` and remove export if present.

**Step 4: Commit**

```bash
git add -A
git commit -m "refactor(sidebar): remove duplicate ViewPresetSelector"
```

---

## Task 2: Create Reusable Accordion Component

**Files:**
- Create: `src/components/ui/Accordion.tsx`

**Step 1: Create Accordion component**

```tsx
'use client';

import { useState, useRef, useEffect, ReactNode } from 'react';
import { ChevronRight } from 'lucide-react';
import { cn } from '@/lib/utils';

export interface AccordionProps {
  title: string;
  icon?: ReactNode;
  badge?: string | number;
  defaultOpen?: boolean;
  children: ReactNode;
  className?: string;
}

export function Accordion({
  title,
  icon,
  badge,
  defaultOpen = false,
  children,
  className,
}: AccordionProps) {
  const [isOpen, setIsOpen] = useState(defaultOpen);
  const contentRef = useRef<HTMLDivElement>(null);
  const [height, setHeight] = useState<number | undefined>(
    defaultOpen ? undefined : 0
  );

  useEffect(() => {
    if (contentRef.current) {
      setHeight(isOpen ? contentRef.current.scrollHeight : 0);
    }
  }, [isOpen]);

  return (
    <div className={cn('border-b border-white/[0.06]', className)}>
      {/* Header */}
      <button
        onClick={() => setIsOpen(!isOpen)}
        className={cn(
          'w-full flex items-center justify-between px-4 py-3',
          'hover:bg-white/[0.03] transition-colors',
          'group'
        )}
      >
        <div className="flex items-center gap-2">
          <ChevronRight
            className={cn(
              'w-4 h-4 text-white/40 transition-transform duration-200',
              isOpen && 'rotate-90'
            )}
          />
          {icon && (
            <span className="text-white/50 group-hover:text-white/70 transition-colors">
              {icon}
            </span>
          )}
          <span className="text-sm font-medium text-white/70 group-hover:text-white/90 transition-colors">
            {title}
          </span>
        </div>
        {badge !== undefined && (
          <span className="text-xs text-white/40 bg-white/[0.06] px-2 py-0.5 rounded-full">
            {badge}
          </span>
        )}
      </button>

      {/* Content */}
      <div
        style={{ height }}
        className="overflow-hidden transition-[height] duration-200 ease-out"
      >
        <div ref={contentRef} className="px-4 pb-3">
          {children}
        </div>
      </div>
    </div>
  );
}
```

**Step 2: Export from ui/index.ts**

Add to `src/components/ui/index.ts`:
```tsx
export { Accordion } from './Accordion';
```

**Step 3: Commit**

```bash
git add src/components/ui/Accordion.tsx src/components/ui/index.ts
git commit -m "feat(ui): add reusable Accordion component with smooth animation"
```

---

## Task 3: Create SearchInput with ⌘K Hint

**Files:**
- Create: `src/components/ui/SearchInput.tsx`

**Step 1: Create SearchInput component**

```tsx
'use client';

import { forwardRef, InputHTMLAttributes } from 'react';
import { Search } from 'lucide-react';
import { cn } from '@/lib/utils';

export interface SearchInputProps extends Omit<InputHTMLAttributes<HTMLInputElement>, 'type'> {
  shortcutHint?: string[];
  onShortcut?: () => void;
}

export const SearchInput = forwardRef<HTMLInputElement, SearchInputProps>(
  ({ className, shortcutHint = ['⌘', 'K'], placeholder = 'Search...', ...props }, ref) => {
    return (
      <div className="relative group">
        <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-white/30 group-focus-within:text-white/50 transition-colors" />
        <input
          ref={ref}
          type="text"
          placeholder={placeholder}
          className={cn(
            'w-full h-10 pl-10 pr-16 rounded-xl',
            'bg-white/[0.04] border border-white/[0.08]',
            'text-sm text-white placeholder:text-white/30',
            'focus:outline-none focus:bg-white/[0.06] focus:border-white/[0.12]',
            'transition-all duration-200',
            className
          )}
          {...props}
        />
        {shortcutHint && shortcutHint.length > 0 && (
          <div className="absolute right-2 top-1/2 -translate-y-1/2 flex items-center gap-0.5">
            {shortcutHint.map((key, idx) => (
              <span key={idx} className="flex items-center">
                <kbd
                  className={cn(
                    'inline-flex items-center justify-center min-w-[22px] h-6 px-1.5',
                    'bg-white/[0.06] border border-white/[0.1] rounded-md',
                    'text-[10px] font-mono text-white/40',
                    'group-focus-within:bg-white/[0.08] group-focus-within:text-white/50',
                    'transition-colors'
                  )}
                >
                  {key}
                </kbd>
                {idx < shortcutHint.length - 1 && (
                  <span className="text-white/20 mx-0.5 text-[10px]">+</span>
                )}
              </span>
            ))}
          </div>
        )}
      </div>
    );
  }
);

SearchInput.displayName = 'SearchInput';
```

**Step 2: Export from ui/index.ts**

Add to `src/components/ui/index.ts`:
```tsx
export { SearchInput } from './SearchInput';
```

**Step 3: Commit**

```bash
git add src/components/ui/SearchInput.tsx src/components/ui/index.ts
git commit -m "feat(ui): add SearchInput with Nika-style keyboard shortcut hint"
```

---

## Task 4: Create QuickFilters Component (Priority + Freshness)

**Files:**
- Create: `src/components/sidebar/QuickFilters.tsx`
- Delete: `src/components/sidebar/PriorityFreshnessFilters.tsx`

**Step 1: Create QuickFilters component**

```tsx
'use client';

import { memo } from 'react';
import { Zap, Clock, AlertTriangle, Timer } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useFilterStore } from '@/stores/filterStore';
import { Accordion } from '@/components/ui/Accordion';

const PRIORITY_OPTIONS = [
  { value: 'critical', label: 'Critical', icon: AlertTriangle, color: 'text-red-400' },
  { value: 'high', label: 'High', color: 'text-orange-400' },
  { value: 'medium', label: 'Medium', color: 'text-yellow-400' },
  { value: 'low', label: 'Low', color: 'text-green-400' },
] as const;

const FRESHNESS_OPTIONS = [
  { value: 'realtime', label: '⚡ RT', fullLabel: 'Realtime' },
  { value: 'hourly', label: '1h', fullLabel: 'Hourly' },
  { value: 'daily', label: '24h', fullLabel: 'Daily' },
  { value: 'static', label: 'Static', fullLabel: 'Static' },
] as const;

type PriorityValue = typeof PRIORITY_OPTIONS[number]['value'];
type FreshnessValue = typeof FRESHNESS_OPTIONS[number]['value'];

export const QuickFilters = memo(function QuickFilters() {
  const priorityFilter = useFilterStore((s) => s.priorityFilter);
  const freshnessFilter = useFilterStore((s) => s.freshnessFilter);
  const togglePriority = useFilterStore((s) => s.togglePriority);
  const toggleFreshness = useFilterStore((s) => s.toggleFreshness);

  const activeCount =
    (priorityFilter?.length || 0) + (freshnessFilter?.length || 0);

  return (
    <Accordion
      title="Quick Filters"
      icon={<Zap className="w-4 h-4" />}
      badge={activeCount > 0 ? activeCount : undefined}
      defaultOpen={true}
    >
      <div className="space-y-3">
        {/* Priority */}
        <div className="space-y-1.5">
          <span className="text-xs text-white/40 uppercase tracking-wider">Priority</span>
          <div className="flex flex-wrap gap-1.5">
            {PRIORITY_OPTIONS.map((opt) => {
              const isActive = priorityFilter?.includes(opt.value as PriorityValue);
              return (
                <button
                  key={opt.value}
                  onClick={() => togglePriority(opt.value as PriorityValue)}
                  className={cn(
                    'px-2.5 py-1.5 rounded-lg text-xs font-medium transition-all duration-150',
                    'border',
                    isActive
                      ? 'bg-white/[0.1] border-white/[0.15] text-white'
                      : 'bg-white/[0.03] border-white/[0.06] text-white/50 hover:bg-white/[0.06] hover:text-white/70'
                  )}
                >
                  <span className={cn(isActive && opt.color)}>{opt.label}</span>
                </button>
              );
            })}
          </div>
        </div>

        {/* Freshness */}
        <div className="space-y-1.5">
          <span className="text-xs text-white/40 uppercase tracking-wider">Freshness</span>
          <div className="flex flex-wrap gap-1.5">
            {FRESHNESS_OPTIONS.map((opt) => {
              const isActive = freshnessFilter?.includes(opt.value as FreshnessValue);
              return (
                <button
                  key={opt.value}
                  onClick={() => toggleFreshness(opt.value as FreshnessValue)}
                  title={opt.fullLabel}
                  className={cn(
                    'px-2.5 py-1.5 rounded-lg text-xs font-medium transition-all duration-150',
                    'border',
                    isActive
                      ? 'bg-white/[0.1] border-white/[0.15] text-white'
                      : 'bg-white/[0.03] border-white/[0.06] text-white/50 hover:bg-white/[0.06] hover:text-white/70'
                  )}
                >
                  {opt.label}
                </button>
              );
            })}
          </div>
        </div>
      </div>
    </Accordion>
  );
});
```

**Step 2: Update sidebar exports**

Update `src/components/sidebar/index.ts`:
```tsx
// Remove: export { PriorityFreshnessFilters } from './PriorityFreshnessFilters';
// Add:
export { QuickFilters } from './QuickFilters';
```

**Step 3: Delete old file**

```bash
rm src/components/sidebar/PriorityFreshnessFilters.tsx
```

**Step 4: Commit**

```bash
git add -A
git commit -m "feat(sidebar): create QuickFilters merging Priority and Freshness"
```

---

## Task 5: Refactor NodeTypeCategories with Count Badges

**Files:**
- Modify: `src/components/sidebar/FilterSidebar.tsx`

**Step 1: Extract NodeTypeCategory component**

Add this component inside FilterSidebar.tsx or create separate file:

```tsx
interface NodeTypeCategoryProps {
  category: string;
  types: NodeTypeConfig[];
  isExpanded: boolean;
  onToggle: () => void;
  selectedTypes: string[];
  onTypeToggle: (type: string) => void;
}

const NodeTypeCategory = memo(function NodeTypeCategory({
  category,
  types,
  isExpanded,
  onToggle,
  selectedTypes,
  onTypeToggle,
}: NodeTypeCategoryProps) {
  const activeCount = types.filter((t) => selectedTypes.includes(t.type)).length;
  const categoryConfig = CATEGORY_CONFIG[category as keyof typeof CATEGORY_CONFIG];

  return (
    <div className="border-b border-white/[0.04] last:border-b-0">
      <button
        onClick={onToggle}
        className={cn(
          'w-full flex items-center justify-between px-3 py-2.5',
          'hover:bg-white/[0.03] transition-colors group'
        )}
      >
        <div className="flex items-center gap-2">
          <ChevronRight
            className={cn(
              'w-3.5 h-3.5 text-white/30 transition-transform duration-200',
              isExpanded && 'rotate-90'
            )}
          />
          <span className="text-xs text-white/50">{categoryConfig?.icon}</span>
          <span className="text-sm text-white/70 group-hover:text-white/90 transition-colors capitalize">
            {category}
          </span>
        </div>
        <div className="flex items-center gap-2">
          {activeCount > 0 && (
            <span className="text-[10px] text-novanet-400 bg-novanet-400/10 px-1.5 py-0.5 rounded-full">
              {activeCount}
            </span>
          )}
          <span className="text-[10px] text-white/30">
            {types.length}
          </span>
        </div>
      </button>

      {isExpanded && (
        <div className="px-3 pb-2 grid grid-cols-2 gap-1 animate-in slide-in-from-top-1 duration-150">
          {types.map((nodeType) => {
            const isSelected = selectedTypes.includes(nodeType.type);
            return (
              <button
                key={nodeType.type}
                onClick={() => onTypeToggle(nodeType.type)}
                className={cn(
                  'flex items-center gap-1.5 px-2 py-1.5 rounded-lg text-xs transition-all',
                  isSelected
                    ? 'bg-white/[0.08] text-white'
                    : 'text-white/50 hover:bg-white/[0.04] hover:text-white/70'
                )}
              >
                <span className="text-sm">{nodeType.icon}</span>
                <span className="truncate">{nodeType.label}</span>
              </button>
            );
          })}
        </div>
      )}
    </div>
  );
});
```

**Step 2: Commit**

```bash
git add src/components/sidebar/FilterSidebar.tsx
git commit -m "refactor(sidebar): add count badges to NodeTypeCategory"
```

---

## Task 6: Refactor FilterSidebar Main Layout

**Files:**
- Modify: `src/components/sidebar/FilterSidebar.tsx`

**Step 1: Update imports**

```tsx
import { SearchInput } from '@/components/ui/SearchInput';
import { QuickFilters } from './QuickFilters';
// Remove: import { ViewPresetSelector } from './ViewPresetSelector';
// Remove: import { PriorityFreshnessFilters } from './PriorityFreshnessFilters';
```

**Step 2: Update layout structure**

```tsx
export function FilterSidebar({ className }: FilterSidebarProps) {
  // ... existing state and hooks

  return (
    <div className={cn(
      'h-full flex flex-col',
      'bg-black/40 backdrop-blur-xl',
      'border-r border-white/[0.06]',
      className
    )}>
      {/* Header */}
      <div className="flex items-center gap-3 px-4 py-4 border-b border-white/[0.06]">
        <div className="w-9 h-9 rounded-xl bg-gradient-to-br from-novanet-400/20 to-novanet-600/20 flex items-center justify-center border border-novanet-400/20">
          <Filter className="w-4 h-4 text-novanet-400" />
        </div>
        <div>
          <h2 className="text-sm font-semibold text-white">Filters</h2>
          <p className="text-[10px] text-white/40">Refine visible nodes</p>
        </div>
      </div>

      {/* Search */}
      <div className="px-3 py-3 border-b border-white/[0.06]">
        <SearchInput
          placeholder="Search node types..."
          shortcutHint={['⌘', 'K']}
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
        />
      </div>

      {/* Quick Filters (Priority + Freshness) */}
      <QuickFilters />

      {/* Node Types - Scrollable */}
      <div className="flex-1 overflow-y-auto scrollbar-thin">
        <div className="px-3 py-2 flex items-center justify-between border-b border-white/[0.06]">
          <span className="text-xs font-medium text-white/50 uppercase tracking-wider">
            Node Types
          </span>
          <span className="text-[10px] text-white/30">
            {selectedNodeTypes.length}/{totalNodeTypes}
          </span>
        </div>

        {/* Categories */}
        {filteredCategories.map(([category, types]) => (
          <NodeTypeCategory
            key={category}
            category={category}
            types={types}
            isExpanded={expandedCategories.includes(category)}
            onToggle={() => toggleCategory(category)}
            selectedTypes={selectedNodeTypes}
            onTypeToggle={toggleNodeType}
          />
        ))}
      </div>

      {/* Footer */}
      <div className="px-4 py-3 border-t border-white/[0.06] flex items-center justify-between">
        <span className="text-xs text-white/40">
          <span className="text-white/60 font-medium">{activeFilterCount}</span> active
        </span>
        <button
          onClick={resetFilters}
          disabled={activeFilterCount === 0}
          className={cn(
            'text-xs px-3 py-1.5 rounded-lg transition-colors',
            activeFilterCount > 0
              ? 'text-novanet-400 hover:bg-novanet-400/10'
              : 'text-white/20 cursor-not-allowed'
          )}
        >
          Reset All
        </button>
      </div>
    </div>
  );
}
```

**Step 3: Commit**

```bash
git add src/components/sidebar/FilterSidebar.tsx
git commit -m "refactor(sidebar): new layout with SearchInput and QuickFilters"
```

---

## Task 7: Add CSS Animations

**Files:**
- Modify: `src/app/globals.css`

**Step 1: Add accordion animations**

```css
/* Accordion animations */
@keyframes slide-in-from-top {
  from {
    opacity: 0;
    transform: translateY(-4px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.animate-in {
  animation-fill-mode: both;
}

.slide-in-from-top-1 {
  animation: slide-in-from-top 150ms ease-out;
}

/* Scrollbar styling */
.scrollbar-thin {
  scrollbar-width: thin;
  scrollbar-color: rgba(255, 255, 255, 0.1) transparent;
}

.scrollbar-thin::-webkit-scrollbar {
  width: 6px;
}

.scrollbar-thin::-webkit-scrollbar-track {
  background: transparent;
}

.scrollbar-thin::-webkit-scrollbar-thumb {
  background-color: rgba(255, 255, 255, 0.1);
  border-radius: 3px;
}

.scrollbar-thin::-webkit-scrollbar-thumb:hover {
  background-color: rgba(255, 255, 255, 0.2);
}
```

**Step 2: Commit**

```bash
git add src/app/globals.css
git commit -m "style: add accordion and scrollbar animations"
```

---

## Task 8: Final Cleanup and Testing

**Step 1: Run type check**

```bash
npm run type-check
```

**Step 2: Run linter**

```bash
npm run lint
```

**Step 3: Fix any issues**

**Step 4: Manual testing checklist**

- [ ] Search input shows ⌘K hint
- [ ] Quick Filters accordion opens/closes smoothly
- [ ] Priority chips toggle correctly
- [ ] Freshness chips toggle correctly
- [ ] Node Type categories expand/collapse with animation
- [ ] Count badges update when selecting types
- [ ] Reset All button works
- [ ] Scrolling works in Node Types section
- [ ] No duplicate Quick Views (removed from sidebar)

**Step 5: Final commit**

```bash
git add -A
git commit -m "feat(sidebar): complete UX refonte with accordions and Nika design"
```

---

## Summary

| Before | After |
|--------|-------|
| Quick Views 3x3 grid (~400px) | Removed (doublon) |
| Priority section (~100px) | Merged into Quick Filters |
| Freshness section (~100px) | Merged into Quick Filters |
| Basic search | Search with ⌘K Nika kbd hint |
| Categories no badges | Categories with count badges |
| Static layout | Smooth accordion animations |
| ~600px fixed content | ~200px + scrollable |
