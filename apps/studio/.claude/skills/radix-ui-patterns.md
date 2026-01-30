---
name: radix-ui-patterns
description: Radix UI primitives with CVA variants and Tailwind. Use when creating Dialog, Dropdown, Tooltip, Tabs, or other accessible UI components.
user-invocable: false
---

# Radix UI + CVA Patterns

> NovaNet component architecture with Radix UI + CVA + Tailwind

## Use When

- Creating new UI components (Dialog, Dropdown, Tooltip, Tabs)
- Implementing CVA variants for buttons, cards, inputs
- Using cn() helper for class merging
- Adding accessibility patterns (focus, keyboard, ARIA)
- Styling with data-[state=*] animations

## Quick Reference

| Component | Import | Key Features |
|-----------|--------|--------------|
| Dialog | `@radix-ui/react-dialog` | Portal, Overlay, focus trap |
| Dropdown | `@radix-ui/react-dropdown-menu` | Portal, sideOffset |
| Tooltip | `@radix-ui/react-tooltip` | Provider, sideOffset |
| Tabs | `@radix-ui/react-tabs` | data-[state=active/inactive] |

---

## Stack Overview

NovaNet uses a shadcn-like pattern without the CLI:

- **Radix UI**: Unstyled, accessible primitives
- **CVA**: Type-safe variant management
- **tailwind-merge**: Class merging without conflicts
- **clsx**: Conditional classes

---

## Core Utilities

### cn() Helper

```typescript
// lib/utils.ts
import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}
```

### Usage

```typescript
// Merge classes without conflicts
cn('px-4 py-2', 'px-6');  // → 'px-6 py-2'

// Conditional classes
cn('base', condition && 'conditional', className);

// With variants
cn(buttonVariants({ variant, size }), className);
```

---

## Component Pattern

### Basic Structure

```typescript
// components/ui/button.tsx
import * as React from 'react';
import { Slot } from '@radix-ui/react-slot';
import { cva, type VariantProps } from 'class-variance-authority';
import { cn } from '@/lib/utils';

const buttonVariants = cva(
  // Base styles (always applied)
  'inline-flex items-center justify-center rounded-lg font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-novanet-500 disabled:pointer-events-none disabled:opacity-50',
  {
    variants: {
      variant: {
        default: 'bg-novanet-500 text-white hover:bg-novanet-600',
        ghost: 'hover:bg-white/10',
        outline: 'border border-white/20 hover:bg-white/10',
        destructive: 'bg-red-500 text-white hover:bg-red-600',
      },
      size: {
        sm: 'h-8 px-3 text-sm',
        md: 'h-10 px-4',
        lg: 'h-12 px-6 text-lg',
        icon: 'h-10 w-10',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'md',
    },
  }
);

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {
  asChild?: boolean;
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant, size, asChild = false, ...props }, ref) => {
    const Comp = asChild ? Slot : 'button';
    return (
      <Comp
        className={cn(buttonVariants({ variant, size, className }))}
        ref={ref}
        {...props}
      />
    );
  }
);
Button.displayName = 'Button';

export { Button, buttonVariants };
```

---

## Radix UI Components

### Dialog (Modal)

```typescript
// components/ui/dialog.tsx
import * as React from 'react';
import * as DialogPrimitive from '@radix-ui/react-dialog';
import { X } from 'lucide-react';
import { cn } from '@/lib/utils';

const Dialog = DialogPrimitive.Root;
const DialogTrigger = DialogPrimitive.Trigger;
const DialogPortal = DialogPrimitive.Portal;
const DialogClose = DialogPrimitive.Close;

const DialogOverlay = React.forwardRef<
  React.ElementRef<typeof DialogPrimitive.Overlay>,
  React.ComponentPropsWithoutRef<typeof DialogPrimitive.Overlay>
>(({ className, ...props }, ref) => (
  <DialogPrimitive.Overlay
    ref={ref}
    className={cn(
      'fixed inset-0 z-50 bg-black/80 backdrop-blur-sm',
      'data-[state=open]:animate-in data-[state=closed]:animate-out',
      'data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0',
      className
    )}
    {...props}
  />
));

const DialogContent = React.forwardRef<
  React.ElementRef<typeof DialogPrimitive.Content>,
  React.ComponentPropsWithoutRef<typeof DialogPrimitive.Content>
>(({ className, children, ...props }, ref) => (
  <DialogPortal>
    <DialogOverlay />
    <DialogPrimitive.Content
      ref={ref}
      className={cn(
        'fixed left-[50%] top-[50%] z-50 translate-x-[-50%] translate-y-[-50%]',
        'w-full max-w-lg rounded-xl border border-white/12 bg-[hsl(240,6%,8%)]',
        'p-6 shadow-2xl shadow-black/50',
        'data-[state=open]:animate-in data-[state=closed]:animate-out',
        'data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0',
        'data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95',
        className
      )}
      {...props}
    >
      {children}
      <DialogPrimitive.Close className="absolute right-4 top-4 rounded-sm opacity-70 hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-novanet-500">
        <X className="h-4 w-4" />
        <span className="sr-only">Close</span>
      </DialogPrimitive.Close>
    </DialogPrimitive.Content>
  </DialogPortal>
));

export { Dialog, DialogTrigger, DialogContent, DialogClose };
```

### Dropdown Menu

```typescript
// components/ui/dropdown-menu.tsx
import * as React from 'react';
import * as DropdownMenuPrimitive from '@radix-ui/react-dropdown-menu';
import { Check, ChevronRight } from 'lucide-react';
import { cn } from '@/lib/utils';

const DropdownMenu = DropdownMenuPrimitive.Root;
const DropdownMenuTrigger = DropdownMenuPrimitive.Trigger;

const DropdownMenuContent = React.forwardRef<
  React.ElementRef<typeof DropdownMenuPrimitive.Content>,
  React.ComponentPropsWithoutRef<typeof DropdownMenuPrimitive.Content>
>(({ className, sideOffset = 4, ...props }, ref) => (
  <DropdownMenuPrimitive.Portal>
    <DropdownMenuPrimitive.Content
      ref={ref}
      sideOffset={sideOffset}
      className={cn(
        'z-50 min-w-[8rem] overflow-hidden rounded-lg',
        'border border-white/12 bg-[hsl(240,6%,8%)] backdrop-blur-xl',
        'p-1 shadow-xl shadow-black/50',
        'data-[state=open]:animate-in data-[state=closed]:animate-out',
        'data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0',
        'data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95',
        className
      )}
      {...props}
    />
  </DropdownMenuPrimitive.Portal>
));

const DropdownMenuItem = React.forwardRef<
  React.ElementRef<typeof DropdownMenuPrimitive.Item>,
  React.ComponentPropsWithoutRef<typeof DropdownMenuPrimitive.Item>
>(({ className, ...props }, ref) => (
  <DropdownMenuPrimitive.Item
    ref={ref}
    className={cn(
      'relative flex cursor-pointer select-none items-center rounded-md',
      'px-2 py-1.5 text-sm outline-none',
      'focus:bg-white/10 data-[disabled]:pointer-events-none data-[disabled]:opacity-50',
      className
    )}
    {...props}
  />
));

export { DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem };
```

### Tooltip

```typescript
// components/ui/tooltip.tsx
import * as React from 'react';
import * as TooltipPrimitive from '@radix-ui/react-tooltip';
import { cn } from '@/lib/utils';

const TooltipProvider = TooltipPrimitive.Provider;
const Tooltip = TooltipPrimitive.Root;
const TooltipTrigger = TooltipPrimitive.Trigger;

const TooltipContent = React.forwardRef<
  React.ElementRef<typeof TooltipPrimitive.Content>,
  React.ComponentPropsWithoutRef<typeof TooltipPrimitive.Content>
>(({ className, sideOffset = 4, ...props }, ref) => (
  <TooltipPrimitive.Content
    ref={ref}
    sideOffset={sideOffset}
    className={cn(
      'z-50 overflow-hidden rounded-md',
      'bg-zinc-900 px-3 py-1.5 text-xs text-white',
      'border border-white/10 shadow-lg',
      'animate-in fade-in-0 zoom-in-95',
      className
    )}
    {...props}
  />
));

export { Tooltip, TooltipTrigger, TooltipContent, TooltipProvider };
```

## CVA Best Practices

### Compound Variants

```typescript
const cardVariants = cva('rounded-xl border transition-all', {
  variants: {
    variant: {
      default: 'bg-[hsl(240,8%,5%)] border-white/12',
      glass: 'bg-[hsl(240,6%,8%)] backdrop-blur-xl border-white/12',
      interactive: 'bg-[hsl(240,8%,5%)] border-white/12 cursor-pointer',
    },
    selected: {
      true: '',
      false: '',
    },
  },
  compoundVariants: [
    // Interactive + selected
    {
      variant: 'interactive',
      selected: true,
      className: 'border-novanet-500 bg-novanet-500/10',
    },
    // Interactive + not selected
    {
      variant: 'interactive',
      selected: false,
      className: 'hover:bg-white/5 hover:border-white/20',
    },
  ],
  defaultVariants: {
    variant: 'default',
    selected: false,
  },
});
```

### Extract Variants Type

```typescript
import { type VariantProps } from 'class-variance-authority';

// Extract variant types for external use
export type ButtonVariant = VariantProps<typeof buttonVariants>['variant'];
export type ButtonSize = VariantProps<typeof buttonVariants>['size'];

// Use in other components
interface Props {
  buttonVariant?: ButtonVariant;
}
```

---

## Accessibility Patterns

### Keyboard Navigation

```typescript
// Radix handles this automatically, but for custom components:
const handleKeyDown = (e: React.KeyboardEvent) => {
  switch (e.key) {
    case 'Enter':
    case ' ':
      e.preventDefault();
      onClick();
      break;
    case 'Escape':
      onClose();
      break;
  }
};
```

### Screen Reader Labels

```typescript
// Always provide accessible names
<TooltipTrigger asChild>
  <button aria-label="Copy to clipboard">
    <CopyIcon aria-hidden="true" />
  </button>
</TooltipTrigger>
```

### Focus Management

```typescript
// Radix dialogs handle focus trap automatically
// For custom components, use data attributes:
<DialogContent
  onOpenAutoFocus={(e) => {
    // Focus specific element on open
    e.preventDefault();
    inputRef.current?.focus();
  }}
  onCloseAutoFocus={(e) => {
    // Focus trigger on close (default)
  }}
>
```

---

## Animation with tailwindcss-animate

### Data State Animations

```css
/* Already configured in tailwind.config.ts */
.data-[state=open]:animate-in
.data-[state=closed]:animate-out
.data-[state=closed]:fade-out-0
.data-[state=open]:fade-in-0
.data-[state=closed]:zoom-out-95
.data-[state=open]:zoom-in-95
```

### Custom Animations

```typescript
// tailwind.config.ts
animation: {
  'fade-in': 'fadeIn 0.2s ease-out',
  'slide-in': 'slideIn 0.2s ease-out',
  'glow': 'glow 2s ease-in-out infinite alternate',
}

// Usage
<div className="animate-fade-in" />
```

---

## Common Patterns

```typescript
// IconButton with tooltip
export function IconButton({ icon, label, ...props }: IconButtonProps) {
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <Button size="icon" variant="ghost" aria-label={label} {...props}>{icon}</Button>
      </TooltipTrigger>
      <TooltipContent>{label}</TooltipContent>
    </Tooltip>
  );
}

// Kbd for keyboard shortcuts
export const Kbd = ({ children }) => (
  <kbd className="h-5 rounded border border-white/20 bg-white/10 px-1.5 font-mono text-[10px]">
    {children}
  </kbd>
);

// Skeleton for loading states
export const Skeleton = ({ className }) => (
  <div className={cn('animate-pulse rounded-md bg-white/10', className)} />
);
```

---

## Checklist

- [ ] All components use `forwardRef`
- [ ] All components accept `className` prop
- [ ] Use `cn()` for class merging
- [ ] Export both component and variants
- [ ] Include `aria-label` for icon-only buttons
- [ ] Use `data-[state=*]` for animations
- [ ] Add `displayName` for DevTools
