---
description: End NovaNet development session with cleanup, validation, and summary
allowed-tools: Bash, Read
---

# NovaNet Session End

Clean up and end development session with validation.

## Workflow

### Step 1: Run Validation

```bash
pnpm type-check
pnpm lint
cargo clippy -- -D warnings
cargo test --quiet
```

Report any failures before proceeding.

### Step 2: Check for Uncommitted Changes

```bash
git status --short
git diff --stat
```

If there are uncommitted changes:
- List changed files
- Ask if user wants to commit
- If yes, use conventional commit format

### Step 3: Summarize Completed Work

Review the session and summarize:
- Files created/modified
- Commands run
- Issues resolved
- Outstanding TODOs

### Step 4: Sign Off

```
+===============================================================================+
|                         SESSION COMPLETE                                      |
+===============================================================================+

  Validation:
    Type-check:  PASSED
    Lint:        PASSED
    Clippy:      PASSED
    Tests:       1031 passed

  Git Status:
    Branch:      feature/example
    Committed:   Yes / No uncommitted changes

  Session Summary:
    - Modified 3 files
    - Added EntityNative node type
    - Fixed 2 type errors

  See you next time!

+===============================================================================+
```

## Quick Session End

If user just wants to leave quickly:

```bash
git status --short
```

Show uncommitted changes count and exit with brief message.
