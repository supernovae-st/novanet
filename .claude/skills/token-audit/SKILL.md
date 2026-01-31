---
name: token-audit
description: Audit design token adoption across the codebase. Use to verify gap/spacing tokens, find non-tokenized patterns, and ensure design system consistency.
disable-model-invocation: false
user-invocable: true
allowed-tools: Grep, Glob, Read
argument-hint: [gaps|all|summary]
---

# Design Token Audit

Systematic audit of design token adoption in the codebase.

## Token System

**Source:** `src/design/tokens.ts`

### gapTokens (Spacing)

| Token | Class | Size |
|-------|-------|------|
| tight | gap-1 | 4px |
| compact | gap-1.5 | 6px |
| default | gap-2 | 8px |
| comfortable | gap-2.5 | 10px |
| spacious | gap-3 | 12px |
| large | gap-4 | 16px |

**Non-tokenizable values:**
- `gap-0.5` (2px) - Too small for semantic token
- `gap-3.5` (14px) - Between spacious and large
- `gap-5` (20px) - Beyond large, consider xlarge token

## Commands

Based on `$ARGUMENTS`, execute the appropriate audit:

### `gaps` (default)

Audit gap-* pattern adoption:

```bash
# Count all gap-* patterns by file
grep -r "gap-[0-9]" apps/studio/src --include="*.tsx" --include="*.ts" -c 2>/dev/null | grep -v ":0$" | sort -t: -k2 -nr
```

**Expected output for 100% adoption:**
- `tokens.ts` - Token definitions (OK)
- Files with gap-0.5, gap-3.5, gap-5 - Non-tokenizable (OK)
- Files with gap-2 in cva - Static strings (OK)

### `summary`

Quick summary of token adoption:

```bash
echo "=== Gap Token Adoption Audit ==="
echo ""
echo "Tokenizable patterns (should be 0):"
grep -r "gap-1[^.]" apps/studio/src --include="*.tsx" -l 2>/dev/null | grep -v tokens.ts || echo "  None - All migrated!"
grep -r "gap-2[^.]" apps/studio/src --include="*.tsx" -l 2>/dev/null | grep -v tokens.ts | grep -v button.tsx || echo "  None - All migrated!"
grep -r "gap-2\.5" apps/studio/src --include="*.tsx" -l 2>/dev/null | grep -v tokens.ts || echo "  None - All migrated!"
grep -r "gap-3[^.]" apps/studio/src --include="*.tsx" -l 2>/dev/null | grep -v tokens.ts || echo "  None - All migrated!"
grep -r "gap-4" apps/studio/src --include="*.tsx" -l 2>/dev/null | grep -v tokens.ts || echo "  None - All migrated!"
echo ""
echo "Non-tokenizable (expected, OK to keep):"
grep -r "gap-0\.5\|gap-3\.5\|gap-5" apps/studio/src --include="*.tsx" -l 2>/dev/null
echo ""
echo "Special cases:"
echo "  button.tsx - cva requires static strings"
```

### `all`

Full audit with detailed report:

1. Run gap audit
2. Show files needing migration
3. Show non-tokenizable patterns
4. Suggest token extensions if needed

## Migration Workflow

When you find non-tokenized patterns:

1. **Add import:** `import { gapTokens } from '@/design/tokens';`
2. **Wrap in cn():** `className={cn('flex items-center', gapTokens.default)}`
3. **Map values:**
   - `gap-1` → `gapTokens.tight`
   - `gap-1.5` → `gapTokens.compact`
   - `gap-2` → `gapTokens.default`
   - `gap-2.5` → `gapTokens.comfortable`
   - `gap-3` → `gapTokens.spacious`
   - `gap-4` → `gapTokens.large`

## Token Extensions

If audit reveals many non-tokenizable patterns, consider extending tokens:

```typescript
// In src/design/tokens.ts
export const gapTokens = {
  // ... existing
  micro: 'gap-0.5',   // 2px - for tight icon groups
  xlarge: 'gap-5',    // 20px - for section spacing
  '2xlarge': 'gap-6', // 24px - for major sections
} as const;
```

## Integration with Ralph Wiggum

This skill is part of the codebase audit toolkit:

1. Run `/codebase-audit` for full codebase health check
2. Run `/token-audit gaps` for token-specific audit
3. Fix patterns and verify with `/token-audit summary`

## CI Integration (Future)

Consider adding to CI pipeline:

```yaml
- name: Token Audit
  run: |
    # Fail if tokenizable patterns found outside tokens.ts
    VIOLATIONS=$(grep -r "gap-[1234]\b" apps/studio/src --include="*.tsx" -l | grep -v tokens.ts | grep -v button.tsx | wc -l)
    if [ "$VIOLATIONS" -gt 0 ]; then
      echo "Found $VIOLATIONS files with non-tokenized gap patterns"
      exit 1
    fi
```
