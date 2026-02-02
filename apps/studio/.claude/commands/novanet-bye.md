# /novanet-bye - Session End

Ends a NovaNet Studio development session with proper cleanup.

## Trigger

`/novanet-bye`

## Actions

1. **Verify Clean State**
   ```bash
   cd /Users/thibaut/supernovae-st/novanet-hq/apps/studio
   pnpm type-check
   pnpm lint
   git status
   ```

2. **Check for Uncommitted Work**
   - If changes exist, ask about committing
   - Follow conventional commit format

3. **Summary**
   ```
   ╔═══════════════════════════════════════════════════════════════╗
   ║  👋 SESSION COMPLETE                                          ║
   ╠═══════════════════════════════════════════════════════════════╣
   ║                                                               ║
   ║  ✅ Type check passed                                         ║
   ║  ✅ Lint passed                                                ║
   ║  ✅ Changes committed                                          ║
   ║                                                               ║
   ║  Work completed:                                              ║
   ║  • [List of completed items from TodoWrite]                   ║
   ║                                                               ║
   ║  Next steps:                                                  ║
   ║  • [Any pending items or follow-ups]                          ║
   ║                                                               ║
   ╚═══════════════════════════════════════════════════════════════╝
   ```

4. **Sign Off** - "See you soon! 🚀"

## Checklist

- [ ] All tests pass
- [ ] Type check clean
- [ ] Lint clean
- [ ] Changes committed
- [ ] No TODO items left in progress
