# LocalePicker Redesign - CommandPalette Style

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Redesign LocalePicker to match CommandPalette visual style with rich locale cards

**Architecture:** Replace custom styling with NovaNet design system (glass-floating, novanet-500 accents), add rich card format with locale details, and include "All locales" clear option

**Tech Stack:** React 19, Motion, Tailwind CSS, NovaNet design tokens

---

## Task 1: Update Modal Container to CommandPalette Style

**Files:**
- Modify: `src/components/sidebar/LocalePicker.tsx`

**Step 1: Replace modal styling**

Replace the current modal container with glass-floating style:

```tsx
// OLD: Custom dark background
className="relative w-full max-w-4xl max-h-[85vh] overflow-hidden bg-[#0d1117] rounded-2xl shadow-2xl"

// NEW: glass-floating style (matches CommandPalette)
className="relative w-full max-w-4xl max-h-[85vh] overflow-hidden glass-floating"
```

**Step 2: Update backdrop**

```tsx
// OLD
className="absolute inset-0 bg-black/80 backdrop-blur-xl"

// NEW (matches CommandPalette)
className="absolute inset-0 bg-black/70 backdrop-blur-sm animate-in fade-in duration-200"
```

**Step 3: Update header styling**

```tsx
// OLD
className="flex items-center justify-between px-6 py-5 border-b border-white/10"

// NEW (matches CommandPalette)
className="flex items-center justify-between px-6 py-4 border-b border-white/[0.08]"
```

**Step 4: Update close button to novanet accent**

```tsx
// OLD: White button with black X
className="w-10 h-10 rounded-full bg-white flex items-center justify-center"

// NEW: Subtle button matching CommandPalette
className="p-2 rounded-lg hover:bg-white/10 transition-colors text-white/60 hover:text-white"
```

---

## Task 2: Redesign Locale Cards as Rich Cards

**Files:**
- Modify: `src/components/sidebar/LocalePicker.tsx`

**Step 1: Update LocaleCard component interface**

```tsx
interface LocaleCardProps {
  code: string;
  info: { name: string; flag: string };
  isSelected: boolean;
  isFocused: boolean;
  onSelect: () => void;
  isAllLocales?: boolean; // For the special "All" card
}
```

**Step 2: Implement rich card layout**

```tsx
const LocaleCard = memo(function LocaleCard({
  code,
  info,
  isSelected,
  isFocused,
  onSelect,
  isAllLocales = false,
}: LocaleCardProps) {
  // Parse locale code for region info
  const [lang, region] = code.split('-');

  return (
    <motion.button
      variants={gridItemVariants}
      whileHover={{ scale: 1.02 }}
      whileTap={{ scale: 0.98 }}
      onClick={onSelect}
      className={cn(
        'flex flex-col items-center justify-center gap-2 p-4 rounded-xl',
        'border transition-all duration-150',
        'min-h-[110px] relative',
        isSelected
          ? 'bg-novanet-500/20 border-novanet-500/30 text-white'
          : isFocused
            ? 'bg-white/[0.06] border-white/20 text-white'
            : 'bg-white/[0.02] border-transparent hover:bg-white/[0.06] hover:border-white/10 text-white/80'
      )}
    >
      {/* Selection indicator */}
      {isSelected && (
        <motion.div
          initial={{ scale: 0 }}
          animate={{ scale: 1 }}
          className="absolute top-2 right-2 w-5 h-5 rounded-full bg-novanet-500 flex items-center justify-center"
        >
          <Check className="w-3 h-3 text-white" />
        </motion.div>
      )}

      {/* Flag */}
      <span className="text-3xl">{info.flag}</span>

      {/* Name */}
      <span className="text-sm font-medium text-center">{info.name}</span>

      {/* Code & Region (rich info) */}
      {!isAllLocales && (
        <span className="text-xs text-white/40">
          {code}{region ? ` · ${region}` : ''}
        </span>
      )}
    </motion.button>
  );
});
```

---

## Task 3: Add "All Locales" First Card

**Files:**
- Modify: `src/components/sidebar/LocalePicker.tsx`

**Step 1: Add ALL_LOCALES_CARD constant**

```tsx
const ALL_LOCALES_CARD = {
  code: null,
  info: { name: 'All Languages', flag: '🌐' },
  count: ALL_LOCALES.length,
};
```

**Step 2: Render All card first in grid**

```tsx
{/* Grid */}
<motion.div ref={gridRef} variants={gridContainerVariants} className="grid grid-cols-4 gap-3">
  {/* All Locales card - always first */}
  <LocaleCard
    key="all"
    code=""
    info={{ name: `All Languages`, flag: '🌐' }}
    isSelected={selectedLocale === null}
    isFocused={focusedIndex === 0}
    onSelect={() => handleSelect(null)}
    isAllLocales
  />

  {/* Filtered locales */}
  {filteredLocales.map((code, index) => {
    const info = localeInfoCache.get(code)!;
    return (
      <LocaleCard
        key={code}
        code={code}
        info={info}
        isSelected={selectedLocale === code}
        isFocused={focusedIndex === index + 1} // +1 for All card
        onSelect={() => handleSelect(code)}
      />
    );
  })}
</motion.div>
```

**Step 3: Update keyboard navigation for +1 offset**

Update `handleKeyDown` to account for the All card at index 0.

---

## Task 4: Update Search Input to CommandPalette Style

**Files:**
- Modify: `src/components/sidebar/LocalePicker.tsx`

**Step 1: Match CommandPalette search header**

```tsx
{/* Search Header - CommandPalette style */}
<div className="flex items-center gap-3 p-4 border-b border-white/[0.08]">
  <Search className="w-5 h-5 text-white/40 shrink-0" />
  <input
    ref={searchRef}
    type="text"
    value={searchInput}
    onChange={(e) => setSearchInput(e.target.value)}
    placeholder="Search languages..."
    className="flex-1 bg-transparent text-white placeholder-white/40 text-base outline-none"
    autoComplete="off"
    spellCheck={false}
  />
  {searchInput && (
    <button
      onClick={() => setSearchInput('')}
      className="p-1.5 hover:bg-white/10 rounded-lg transition-colors text-white/40 hover:text-white/60"
    >
      <X className="w-4 h-4" />
    </button>
  )}
</div>
```

---

## Task 5: Update Footer to Match CommandPalette

**Files:**
- Modify: `src/components/sidebar/LocalePicker.tsx`

**Step 1: Replace footer styling**

```tsx
{/* Footer - CommandPalette style */}
<div className="p-3 border-t border-white/[0.08] bg-black/20">
  <div className="flex items-center justify-between text-xs text-white/50">
    <span>{filteredLocales.length + 1} languages</span>
    <div className="flex items-center gap-4">
      <span className="flex items-center gap-1.5">
        <kbd>↑↓←→</kbd>
        <span>Navigate</span>
      </span>
      <span className="flex items-center gap-1.5">
        <kbd>↵</kbd>
        <span>Select</span>
      </span>
      <span className="flex items-center gap-1.5">
        <kbd>Esc</kbd>
        <span>Close</span>
      </span>
    </div>
  </div>
</div>
```

---

## Task 6: Add Modal Enter Animation

**Files:**
- Modify: `src/components/sidebar/LocalePicker.tsx`

**Step 1: Add animate-scale-in class to modal**

```tsx
className="relative w-full max-w-4xl max-h-[85vh] overflow-hidden glass-floating animate-scale-in"
```

---

## Verification

After all tasks, run:
```bash
npm run type-check
npx playwright test --reporter=list
```

All tests should pass and the LocalePicker should visually match CommandPalette style.
