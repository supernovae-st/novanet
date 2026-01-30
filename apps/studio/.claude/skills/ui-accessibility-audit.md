---
name: ui-accessibility-audit
description: WCAG accessibility and UX patterns. Use when building UI components, forms, or reviewing accessibility.
user-invocable: false
---

# UI Accessibility & UX Audit for NovaNet Studio

> Adapted from Vercel web-interface-guidelines (100+ rules)

## 1. Focus Management

### All Interactive Elements Must Be Focusable

```typescript
// BAD - Div as button (not focusable)
<div onClick={handleClick}>Click me</div>

// GOOD - Semantic button
<button onClick={handleClick}>Click me</button>

// GOOD - If div is necessary, add role and tabIndex
<div role="button" tabIndex={0} onClick={handleClick} onKeyDown={handleKeyDown}>
  Click me
</div>
```

### Focus Indicators Must Be Visible

```css
/* BAD - Hidden focus */
*:focus { outline: none; }

/* GOOD - Visible focus ring */
.focus-visible:focus-visible {
  @apply ring-2 ring-novanet-500 ring-offset-2 ring-offset-black;
}

/* In Tailwind - use focus-visible */
<button className="focus-visible:ring-2 focus-visible:ring-novanet-500">
```

### Trap Focus in Modals

```typescript
// Use Radix UI Dialog (handles focus trap automatically)
import * as Dialog from '@radix-ui/react-dialog';

export function Modal({ children, open, onOpenChange }: ModalProps) {
  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Portal>
        <Dialog.Overlay className="fixed inset-0 bg-black/80" />
        <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2">
          {children}
          <Dialog.Close asChild>
            <button aria-label="Close">X</button>
          </Dialog.Close>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
```

### Restore Focus After Modal Close

```typescript
// Radix handles this automatically
// For custom modals:
const previousFocus = useRef<HTMLElement | null>(null);

useEffect(() => {
  if (isOpen) {
    previousFocus.current = document.activeElement as HTMLElement;
  } else {
    previousFocus.current?.focus();
  }
}, [isOpen]);
```

---

## 2. Keyboard Navigation

### Support Standard Shortcuts

```typescript
// NovaNet shortcuts (from CLAUDE.md)
useEffect(() => {
  const handleKeyDown = (e: KeyboardEvent) => {
    // Don't trigger when typing in inputs
    if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) {
      return;
    }

    switch (e.key) {
      case 'Escape':
        closeModal();
        clearSelection();
        break;
      case 'f':
        fitView();
        break;
      case 'v':
        toggleViewMode();
        break;
      case '?':
        openShortcutsModal();
        break;
    }
  };

  window.addEventListener('keydown', handleKeyDown);
  return () => window.removeEventListener('keydown', handleKeyDown);
}, []);
```

### Arrow Key Navigation in Lists

```typescript
export function NodeList({ nodes }: { nodes: NodeType[] }) {
  const [focusedIndex, setFocusedIndex] = useState(0);
  const listRef = useRef<HTMLUListElement>(null);

  const handleKeyDown = (e: KeyboardEvent) => {
    switch (e.key) {
      case 'ArrowDown':
        e.preventDefault();
        setFocusedIndex((i) => Math.min(i + 1, nodes.length - 1));
        break;
      case 'ArrowUp':
        e.preventDefault();
        setFocusedIndex((i) => Math.max(i - 1, 0));
        break;
      case 'Enter':
        selectNode(nodes[focusedIndex].id);
        break;
    }
  };

  return (
    <ul ref={listRef} role="listbox" onKeyDown={handleKeyDown}>
      {nodes.map((node, i) => (
        <li
          key={node.id}
          role="option"
          aria-selected={i === focusedIndex}
          tabIndex={i === focusedIndex ? 0 : -1}
        >
          {node.label}
        </li>
      ))}
    </ul>
  );
}
```

---

## 3. Screen Reader Support

### Use Semantic HTML

```typescript
// BAD - Div soup
<div className="header">
  <div className="title">NovaNet</div>
  <div className="nav">...</div>
</div>

// GOOD - Semantic elements
<header>
  <h1>NovaNet</h1>
  <nav aria-label="Main navigation">...</nav>
</header>
```

### ARIA Labels for Icons

```typescript
// BAD - Icon without label
<button>
  <CopyIcon />
</button>

// GOOD - Accessible icon button
<button aria-label="Copy to clipboard">
  <CopyIcon aria-hidden="true" />
</button>

// GOOD - With visible label
<button>
  <CopyIcon aria-hidden="true" />
  <span>Copy</span>
</button>
```

### Live Regions for Dynamic Content

```typescript
// Announce status changes
export function CopyButton({ text }: { text: string }) {
  const [copied, setCopied] = useState(false);

  const copy = async () => {
    await navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <>
      <button onClick={copy} aria-label={copied ? 'Copied!' : 'Copy to clipboard'}>
        {copied ? <CheckIcon /> : <CopyIcon />}
      </button>
      {/* Screen reader announcement */}
      <span role="status" aria-live="polite" className="sr-only">
        {copied ? 'Copied to clipboard' : ''}
      </span>
    </>
  );
}
```

---

## 4. Color & Contrast

### Minimum Contrast Ratios

```css
/* WCAG AA requirements */
/* Normal text: 4.5:1 */
/* Large text (18px+ or 14px+ bold): 3:1 */
/* UI components: 3:1 */

/* NovaNet color system */
:root {
  --text-primary: #ffffff;      /* Use on dark bg */
  --text-secondary: #a1a1aa;    /* Muted text - ensure 4.5:1 */
  --text-disabled: #52525b;     /* Meets 3:1 on black */
}

/* Check contrast with: https://webaim.org/resources/contrastchecker/ */
```

### Don't Rely on Color Alone

```typescript
// BAD - Only color indicates state
<div className={node.hasError ? 'text-red-500' : 'text-green-500'}>
  {node.status}
</div>

// GOOD - Color + icon + text
<div className={node.hasError ? 'text-red-500' : 'text-green-500'}>
  {node.hasError ? <ErrorIcon /> : <CheckIcon />}
  <span>{node.hasError ? 'Error' : 'Success'}</span>
</div>
```

---

## 5. Forms & Inputs

### Labels Are Required

```typescript
// BAD - No label
<input type="text" placeholder="Search..." />

// GOOD - Visible label
<label>
  <span>Search nodes</span>
  <input type="text" />
</label>

// GOOD - Hidden label for icon inputs
<label>
  <span className="sr-only">Search nodes</span>
  <SearchIcon className="absolute left-3" />
  <input type="text" className="pl-10" placeholder="Search..." />
</label>
```

### Error Messages

```typescript
export function NodeForm() {
  const [errors, setErrors] = useState<Record<string, string>>({});

  return (
    <form>
      <div>
        <label htmlFor="node-label">Label</label>
        <input
          id="node-label"
          aria-invalid={!!errors.label}
          aria-describedby={errors.label ? 'label-error' : undefined}
        />
        {errors.label && (
          <p id="label-error" role="alert" className="text-red-500 text-sm">
            {errors.label}
          </p>
        )}
      </div>
    </form>
  );
}
```

### Loading States

```typescript
export function SubmitButton({ loading }: { loading: boolean }) {
  return (
    <button type="submit" disabled={loading} aria-busy={loading}>
      {loading ? (
        <>
          <Spinner aria-hidden="true" />
          <span className="sr-only">Loading...</span>
        </>
      ) : (
        'Submit'
      )}
    </button>
  );
}
```

---

## 6. Motion & Animation

### Respect Reduced Motion

```css
/* Default animation */
.animate-fade-in {
  animation: fadeIn 0.3s ease-in-out;
}

/* Respect user preference */
@media (prefers-reduced-motion: reduce) {
  .animate-fade-in {
    animation: none;
    opacity: 1;
  }
}
```

```typescript
// In JavaScript
const prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

const animationDuration = prefersReducedMotion ? 0 : 300;
```

### No Auto-Playing Animations

```typescript
// BAD - Auto-plays immediately
useEffect(() => {
  startAnimation();
}, []);

// GOOD - User-triggered or with controls
<button onClick={startAnimation}>Play Animation</button>

// GOOD - Provide stop control
{isAnimating && (
  <button onClick={stopAnimation} aria-label="Stop animation">
    Stop
  </button>
)}
```

---

## 7. Performance as Accessibility

### Skeleton Loading

```typescript
// Show structure immediately
export function GraphSkeleton() {
  return (
    <div className="animate-pulse" aria-busy="true" aria-label="Loading graph...">
      <div className="h-8 w-48 bg-white/10 rounded mb-4" />
      <div className="h-[600px] bg-white/5 rounded" />
    </div>
  );
}
```

### Optimistic UI

```typescript
// Don't wait for server to show feedback
async function deleteNode(id: string) {
  // Immediately update UI
  setNodes((nodes) => nodes.filter((n) => n.id !== id));

  try {
    await api.deleteNode(id);
  } catch {
    // Revert on error
    refetch();
    toast.error('Failed to delete node');
  }
}
```

---

## 8. Dark Mode, i18n & Responsive

### Dark Mode (NovaNet Default)

```css
:root {
  --bg-primary: #000000;
  --bg-secondary: #18181b;
  --text-primary: #ffffff;
  --text-secondary: #a1a1aa;
}
```

### RTL & Locale Support

```typescript
const dir = ['ar', 'he', 'fa'].includes(locale) ? 'rtl' : 'ltr';
const formatDate = (date, locale) => new Intl.DateTimeFormat(locale).format(date);
```

### Touch Targets (44x44px minimum)

```typescript
<button className="min-w-11 min-h-11 p-3">
```

---

## Audit Checklist

### Focus

- [ ] All interactive elements are keyboard accessible
- [ ] Focus order is logical (matches visual order)
- [ ] Focus indicators are visible
- [ ] Focus is trapped in modals
- [ ] Focus is restored when modals close

### Screen Readers

- [ ] All images have alt text
- [ ] Icon buttons have aria-label
- [ ] Dynamic content uses aria-live
- [ ] Forms have proper labels
- [ ] Errors are announced

### Color

- [ ] Text meets 4.5:1 contrast ratio
- [ ] UI components meet 3:1 contrast ratio
- [ ] Information not conveyed by color alone

### Motion

- [ ] Respects prefers-reduced-motion
- [ ] No auto-playing animations
- [ ] Users can pause/stop animations

### Forms

- [ ] All inputs have labels
- [ ] Errors are clearly indicated
- [ ] Required fields are marked
- [ ] Loading states are communicated

### Performance

- [ ] Loading states show structure (skeletons)
- [ ] Actions feel instant (optimistic UI)
- [ ] Large lists are virtualized

---

## Testing Tools

```bash
npx lighthouse http://localhost:3000 --only-categories=accessibility
npm install @axe-core/react @axe-core/playwright --save-dev
```

```typescript
// Playwright: expect(await new AxeBuilder({ page }).analyze()).violations.toEqual([]);
```
