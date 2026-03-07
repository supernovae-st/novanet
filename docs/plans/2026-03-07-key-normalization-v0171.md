# Key Normalization v0.17.1 Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Normalize all NovaNet key patterns to Option B convention and achieve CSR 100%

**Architecture:** 4 Cypher migrations to standardize key patterns across Term, Expression, Pattern, and PatternSet nodes using the validated Option B separator convention.

**Tech Stack:** Neo4j Cypher, novanet MCP tools (novanet_query, novanet_audit, novanet_write)

---

## Convention Reference (Option B)

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  KEY NAMING CONVENTION - OPTION B                                               │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  SEPARATORS:                                                                    │
│  ├── :  = Type prefix (class identifier)                                        │
│  ├── @  = Locale VARIANT (*Native classes with parent relationship)             │
│  ├── /  = Locale IDENTITY (knowledge atoms where locale IS the identity)        │
│  └── -  = Word separator in slugs                                               │
│                                                                                 │
│  EXAMPLES:                                                                      │
│  ├── entity:qr-code           (Entity - no locale suffix)                       │
│  ├── entity:qr-code@fr-FR     (EntityNative - locale variant)                   │
│  ├── term:fr-FR/seo           (Term - locale identity)                          │
│  ├── expression:fr-FR/SUCCESS/0 (Expression - locale identity)                  │
│  ├── pattern:en-US/cta-create (Pattern - locale identity)                       │
│  └── pattern-set:en-US/cta    (PatternSet - locale identity)                    │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Pre-Migration State

| Class | Count | Current Pattern | Target Pattern |
|-------|-------|-----------------|----------------|
| Term | 35 | `term:fr-FR:slug` | `term:fr-FR/slug` |
| Expression | 17,343 | `fr-FR/SUCCESS/0` | `expression:fr-FR/SUCCESS/0` |
| Pattern | 3 | `pattern:slug@locale` | `pattern:locale/slug` |
| PatternSet | 1 | `pattern-set:cat@locale` | `pattern-set:locale/cat` |
| **CSR** | 100% | 77,846 constraints | Maintain 100% |

---

## Task 1: Update ADR-036 with Option B Convention

**Files:**
- Modify: `dx/adr/novanet/schema-architecture/adr-036-key-naming-conventions.md`

**Step 1: Read current ADR-036**

```bash
cat dx/adr/novanet/schema-architecture/adr-036-key-naming-conventions.md
```

**Step 2: Update ADR-036 with Option B convention**

Add the separator convention section at the top after the Problem statement:

```markdown
## Separator Convention (Option B)

| Separator | Name | Usage |
|-----------|------|-------|
| `:` | Type prefix | Class identifier at start of key |
| `@` | Variant | Locale variant of parent (*Native classes) |
| `/` | Identity | Locale identity in knowledge atoms |
| `-` | Word | Word separator in slugs |

**Key principle:**
- `@` = "variant of" (has parent relationship)
- `/` = "identity is" (no parent, locale IS the identity)
```

Update Tier 2 (Knowledge Atoms) table with corrected patterns:

```markdown
| Class | Key Pattern | Example |
|-------|-------------|---------|
| `Term` | `term:{locale}/{slug}` | `term:fr-FR/referencement-naturel` |
| `TermSet` | `termset:{locale}` | `termset:fr-FR` |
| `Expression` | `expression:{locale}/{category}/{n}` | `expression:fr-FR/SUCCESS/0` |
| `ExpressionSet` | `expression-set:{locale}` | `expression-set:fr-FR` |
| `Pattern` | `pattern:{locale}/{slug}` | `pattern:en-US/cta-create` |
| `PatternSet` | `pattern-set:{locale}/{category}` | `pattern-set:en-US/cta` |
```

**Step 3: Verify edit is correct**

```bash
grep -A 20 "Tier 2: Knowledge Atoms" dx/adr/novanet/schema-architecture/adr-036-key-naming-conventions.md
```

**Step 4: Commit**

```bash
git add dx/adr/novanet/schema-architecture/adr-036-key-naming-conventions.md
git commit -m "docs(adr): update ADR-036 with Option B separator convention

- Add separator convention section (: @ / -)
- Update Tier 2 knowledge atoms patterns
- : = type prefix, @ = variant, / = identity, - = words

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika 🦋 <nika@supernovae.studio>"
```

---

## Task 2: Migration 036 - Term Keys

**Files:**
- Create: `/Users/thibaut/dev/supernovae/brain/seed/migrations/036-normalize-term-keys.cypher`

**Step 1: Create migration file**

```cypher
// Migration 036: Normalize Term keys to term:locale/slug pattern
// PENDING: 2026-03-07
//
// Change: term:locale:slug → term:locale/slug
// Affected: ~35 rows
// Convention: Option B - use / for locale identity in knowledge atoms

// ============================================================================
// STEP 1: Normalize Term keys (change second : to /)
// Pattern: term:locale:slug → term:locale/slug
// ============================================================================
MATCH (t:Term)
WHERE t.key STARTS WITH 'term:'
WITH t, split(t.key, ':') AS parts
WHERE size(parts) = 3
SET t.key = parts[0] + ':' + parts[1] + '/' + parts[2]
RETURN count(t) AS normalized;
```

**Step 2: Run migration via novanet_query**

```
Use MCP tool: novanet_query
cypher: |
  MATCH (t:Term)
  WHERE t.key STARTS WITH 'term:'
  WITH t, split(t.key, ':') AS parts
  WHERE size(parts) = 3
  SET t.key = parts[0] + ':' + parts[1] + '/' + parts[2]
  RETURN count(t) AS normalized
```

**Step 3: Verify migration**

```
Use MCP tool: novanet_query
cypher: MATCH (t:Term) RETURN t.key AS key LIMIT 5
```

Expected: Keys now follow `term:locale/slug` pattern

**Step 4: Run CSR audit**

```
Use MCP tool: novanet_audit
target: all
```

Expected: CSR = 100%

**Step 5: Update migration file with results**

Add results comment at top of file:
```cypher
// Results:
// - 35 rows normalized
// - term:fr-FR:seo → term:fr-FR/seo (example)
```

**Step 6: Commit**

```bash
git add brain/seed/migrations/036-normalize-term-keys.cypher
git commit -m "feat(migration): 036 normalize Term keys to term:locale/slug

- Change term:locale:slug → term:locale/slug
- 35 rows normalized
- CSR maintained at 100%

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika 🦋 <nika@supernovae.studio>"
```

---

## Task 3: Migration 037 - Expression Keys

**Files:**
- Create: `/Users/thibaut/dev/supernovae/brain/seed/migrations/037-normalize-expression-keys.cypher`

**Step 1: Create migration file**

```cypher
// Migration 037: Normalize Expression keys to expression:locale/category/n pattern
// PENDING: 2026-03-07
//
// Change: locale/category/n → expression:locale/category/n
// Affected: ~17,343 rows
// Convention: Option B - add expression: prefix for type identification

// ============================================================================
// STEP 1: Add expression: prefix to Expression keys
// Pattern: locale/category/n → expression:locale/category/n
// ============================================================================
MATCH (e:Expression)
WHERE NOT e.key STARTS WITH 'expression:'
SET e.key = 'expression:' + e.key
RETURN count(e) AS normalized;
```

**Step 2: Run migration via novanet_query**

```
Use MCP tool: novanet_query
cypher: |
  MATCH (e:Expression)
  WHERE NOT e.key STARTS WITH 'expression:'
  SET e.key = 'expression:' + e.key
  RETURN count(e) AS normalized
```

**Step 3: Verify migration**

```
Use MCP tool: novanet_query
cypher: MATCH (e:Expression) RETURN e.key AS key LIMIT 5
```

Expected: Keys now follow `expression:locale/category/n` pattern

**Step 4: Run CSR audit**

```
Use MCP tool: novanet_audit
target: all
```

Expected: CSR = 100%

**Step 5: Update migration file with results**

```cypher
// Results:
// - 17,343 rows normalized
// - fr-FR/SUCCESS/0 → expression:fr-FR/SUCCESS/0 (example)
```

**Step 6: Commit**

```bash
git add brain/seed/migrations/037-normalize-expression-keys.cypher
git commit -m "feat(migration): 037 normalize Expression keys with prefix

- Add expression: prefix to all Expression keys
- locale/cat/n → expression:locale/cat/n
- 17,343 rows normalized
- CSR maintained at 100%

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika 🦋 <nika@supernovae.studio>"
```

---

## Task 4: Migration 038 - Pattern Keys

**Files:**
- Create: `/Users/thibaut/dev/supernovae/brain/seed/migrations/038-normalize-pattern-keys.cypher`

**Step 1: Create migration file**

```cypher
// Migration 038: Normalize Pattern keys to pattern:locale/slug pattern
// PENDING: 2026-03-07
//
// Change: pattern:slug@locale → pattern:locale/slug
// Affected: ~3 rows
// Convention: Option B - restructure to locale/slug order

// ============================================================================
// STEP 1: Normalize Pattern keys (restructure @ to /)
// Pattern: pattern:slug@locale → pattern:locale/slug
// ============================================================================
MATCH (p:Pattern)
WHERE p.key CONTAINS '@'
WITH p,
     replace(p.key, 'pattern:', '') AS rest
WITH p, split(rest, '@') AS parts
WHERE size(parts) = 2
SET p.key = 'pattern:' + parts[1] + '/' + parts[0]
RETURN count(p) AS normalized;
```

**Step 2: Run migration via novanet_query**

```
Use MCP tool: novanet_query
cypher: |
  MATCH (p:Pattern)
  WHERE p.key CONTAINS '@'
  WITH p,
       replace(p.key, 'pattern:', '') AS rest
  WITH p, split(rest, '@') AS parts
  WHERE size(parts) = 2
  SET p.key = 'pattern:' + parts[1] + '/' + parts[0]
  RETURN count(p) AS normalized
```

**Step 3: Verify migration**

```
Use MCP tool: novanet_query
cypher: MATCH (p:Pattern) RETURN p.key AS key LIMIT 5
```

Expected: Keys now follow `pattern:locale/slug` pattern

**Step 4: Run CSR audit**

```
Use MCP tool: novanet_audit
target: all
```

Expected: CSR = 100%

**Step 5: Update migration file with results**

```cypher
// Results:
// - 3 rows normalized
// - pattern:cta-create@en-US → pattern:en-US/cta-create (example)
```

**Step 6: Commit**

```bash
git add brain/seed/migrations/038-normalize-pattern-keys.cypher
git commit -m "feat(migration): 038 normalize Pattern keys to locale/slug

- Restructure pattern:slug@locale → pattern:locale/slug
- 3 rows normalized
- CSR maintained at 100%

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika 🦋 <nika@supernovae.studio>"
```

---

## Task 5: Migration 039 - PatternSet Keys

**Files:**
- Create: `/Users/thibaut/dev/supernovae/brain/seed/migrations/039-normalize-patternset-keys.cypher`

**Step 1: Create migration file**

```cypher
// Migration 039: Normalize PatternSet keys to pattern-set:locale/category pattern
// PENDING: 2026-03-07
//
// Change: pattern-set:category@locale → pattern-set:locale/category
// Affected: ~1 row
// Convention: Option B - restructure to locale/category order

// ============================================================================
// STEP 1: Normalize PatternSet keys (restructure @ to /)
// Pattern: pattern-set:category@locale → pattern-set:locale/category
// ============================================================================
MATCH (ps:PatternSet)
WHERE ps.key CONTAINS '@'
WITH ps,
     replace(ps.key, 'pattern-set:', '') AS rest
WITH ps, split(rest, '@') AS parts
WHERE size(parts) = 2
SET ps.key = 'pattern-set:' + parts[1] + '/' + parts[0]
RETURN count(ps) AS normalized;
```

**Step 2: Run migration via novanet_query**

```
Use MCP tool: novanet_query
cypher: |
  MATCH (ps:PatternSet)
  WHERE ps.key CONTAINS '@'
  WITH ps,
       replace(ps.key, 'pattern-set:', '') AS rest
  WITH ps, split(rest, '@') AS parts
  WHERE size(parts) = 2
  SET ps.key = 'pattern-set:' + parts[1] + '/' + parts[0]
  RETURN count(ps) AS normalized
```

**Step 3: Verify migration**

```
Use MCP tool: novanet_query
cypher: MATCH (ps:PatternSet) RETURN ps.key AS key LIMIT 5
```

Expected: Keys now follow `pattern-set:locale/category` pattern

**Step 4: Run CSR audit**

```
Use MCP tool: novanet_audit
target: all
```

Expected: CSR = 100%

**Step 5: Update migration file with results**

```cypher
// Results:
// - 1 row normalized
// - pattern-set:cta@en-US → pattern-set:en-US/cta (example)
```

**Step 6: Commit**

```bash
git add brain/seed/migrations/039-normalize-patternset-keys.cypher
git commit -m "feat(migration): 039 normalize PatternSet keys to locale/category

- Restructure pattern-set:cat@locale → pattern-set:locale/cat
- 1 row normalized
- CSR maintained at 100%

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika 🦋 <nika@supernovae.studio>"
```

---

## Task 6: Final Verification

**Step 1: Run comprehensive key pattern audit**

```
Use MCP tool: novanet_query
cypher: |
  MATCH (n)
  WHERE n.key IS NOT NULL
  WITH labels(n)[0] AS class, n.key AS key
  RETURN class,
         CASE
           WHEN key CONTAINS '@' AND NOT class ENDS WITH 'Native' THEN 'VIOLATION: @ in non-Native'
           WHEN class IN ['Term', 'Expression', 'Pattern', 'PatternSet'] AND NOT key CONTAINS '/' THEN 'VIOLATION: missing / for knowledge atom'
           ELSE 'OK'
         END AS status,
         count(*) AS count
  ORDER BY status DESC, count DESC
```

Expected: All rows show 'OK' status

**Step 2: Run full CSR audit**

```
Use MCP tool: novanet_audit
target: all
```

Expected: CSR = 100%, 0 violations

**Step 3: Verify schema validation**

```bash
cd /Users/thibaut/dev/supernovae/novanet && cargo run -- schema validate
```

Expected: All validations pass

**Step 4: Summary commit**

```bash
git add -A
git commit -m "docs(plan): complete key normalization v0.17.1

Migrations completed:
- 036: Term keys (35 rows)
- 037: Expression keys (17,343 rows)
- 038: Pattern keys (3 rows)
- 039: PatternSet keys (1 row)

Convention: Option B (: @ / -)
CSR: 100% maintained throughout

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika 🦋 <nika@supernovae.studio>"
```

---

## Verification Checkpoints

| Checkpoint | After Task | Command | Expected |
|------------|------------|---------|----------|
| CP1 | Task 2 | `novanet_audit target:all` | CSR 100% |
| CP2 | Task 3 | `novanet_audit target:all` | CSR 100% |
| CP3 | Task 4 | `novanet_audit target:all` | CSR 100% |
| CP4 | Task 5 | `novanet_audit target:all` | CSR 100% |
| CP5 | Task 6 | Full key pattern audit | 0 violations |

---

## Rollback Plan

If any migration causes issues:

```cypher
// Rollback 036 (Term)
MATCH (t:Term)
WHERE t.key STARTS WITH 'term:' AND t.key CONTAINS '/'
WITH t, split(t.key, ':') AS prefix_parts
WITH t, prefix_parts[0] AS prefix, split(prefix_parts[1], '/') AS rest
SET t.key = prefix + ':' + rest[0] + ':' + rest[1]
RETURN count(t);

// Rollback 037 (Expression)
MATCH (e:Expression)
WHERE e.key STARTS WITH 'expression:'
SET e.key = replace(e.key, 'expression:', '')
RETURN count(e);

// Rollback 038 (Pattern)
MATCH (p:Pattern)
WHERE p.key STARTS WITH 'pattern:' AND p.key CONTAINS '/'
WITH p, replace(p.key, 'pattern:', '') AS rest
WITH p, split(rest, '/') AS parts
SET p.key = 'pattern:' + parts[1] + '@' + parts[0]
RETURN count(p);

// Rollback 039 (PatternSet)
MATCH (ps:PatternSet)
WHERE ps.key STARTS WITH 'pattern-set:' AND ps.key CONTAINS '/'
WITH ps, replace(ps.key, 'pattern-set:', '') AS rest
WITH ps, split(rest, '/') AS parts
SET ps.key = 'pattern-set:' + parts[1] + '@' + parts[0]
RETURN count(ps);
```

---

## Post-Migration Documentation Updates

After all migrations complete, update these files:

1. **CLAUDE.md** - Update version to v0.17.1
2. **CHANGELOG.md** - Add v0.17.1 entry with migration summary
3. **ADR-036** - Mark as APPLIED with date and row counts

---

## Summary

| Task | Description | Rows | Time Est. |
|------|-------------|------|-----------|
| 1 | Update ADR-036 | - | 5 min |
| 2 | Migration 036 (Term) | 35 | 5 min |
| 3 | Migration 037 (Expression) | 17,343 | 5 min |
| 4 | Migration 038 (Pattern) | 3 | 5 min |
| 5 | Migration 039 (PatternSet) | 1 | 5 min |
| 6 | Final verification | - | 10 min |
| **Total** | | **17,382** | **35 min** |
