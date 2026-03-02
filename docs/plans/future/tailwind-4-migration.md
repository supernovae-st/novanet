# Tailwind CSS 4 Migration Plan

**Status**: Deferred
**Priority**: Medium
**Blocked by**: Major breaking changes, ecosystem readiness

## Context

Tailwind CSS 4 is a major rewrite with significant breaking changes. This document outlines what's needed for migration when we're ready.

## Breaking Changes in Tailwind 4

### Configuration

1. **No more `tailwind.config.js`**
   - Configuration moves to CSS `@theme` directive
   - JavaScript config deprecated in favor of CSS-based config

2. **PostCSS changes**
   ```css
   /* tailwind.css (new approach) */
   @import "tailwindcss";

   @theme {
     --color-primary: #6366f1;
     --spacing-lg: 2rem;
   }
   ```

3. **Content detection**
   - Automatic content detection (no `content: []` needed)
   - May need explicit excludes for generated files

### Class Name Changes

1. **Shadow utilities**
   - `shadow-sm` → different values
   - Custom shadows need updating

2. **Ring utilities**
   - Default ring width changes
   - May affect focus states

3. **Color opacity**
   - `bg-blue-500/50` syntax changes
   - Review all opacity modifiers

### Plugin Ecosystem

1. **`@tailwindcss/typography`**
   - May need update for v4 compatibility
   - Prose classes may change

2. **`tailwindcss-animate`**
   - Check compatibility
   - Animation utilities may need updates

## Affected Files

### Configuration
- `apps/studio/tailwind.config.js` → CSS migration
- `apps/studio/postcss.config.mjs` → Update plugins

### Component Files (audit needed)
- `apps/studio/src/components/**/*.tsx` - All components using Tailwind
- `apps/studio/src/app/globals.css` - Global styles
- Design tokens in `apps/studio/src/design/` - May need restructuring

## Migration Steps (When Prioritized)

### Phase 1: Audit (2-4 hours)
1. Run Tailwind 4 upgrade tool (when available)
2. Identify all breaking class usages
3. Document custom configuration to migrate

### Phase 2: Configuration (1-2 hours)
1. Convert `tailwind.config.js` to CSS `@theme`
2. Update PostCSS configuration
3. Update typography plugin

### Phase 3: Class Updates (4-8 hours)
1. Update shadow utilities
2. Update ring utilities
3. Update color opacity syntax
4. Run visual regression tests

### Phase 4: Verification (2-4 hours)
1. Full visual QA of all pages
2. Test dark mode (if applicable)
3. Test responsive breakpoints
4. Performance comparison

## Estimated Total Effort

**10-18 hours** depending on:
- Tool availability for automated migration
- Number of custom utilities
- Visual regression tooling

## Decision

Deferring migration because:
- Tailwind 4 just released, ecosystem not ready
- `tailwindcss-animate` compatibility unknown
- Current v3 works correctly
- No blocking features needed from v4

## Re-evaluate When

- Tailwind 4 stable for 3+ months
- All plugins confirmed compatible
- Next.js officially supports Tailwind 4
- Planning major UI refresh anyway
