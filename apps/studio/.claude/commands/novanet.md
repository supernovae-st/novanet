# /novanet - Session Start

Starts a NovaNet Studio development session.

## Trigger

`/novanet`

## Actions

1. **Greet** - "Hello Thibaut! Starting NovaNet Studio session."

2. **Check Status**
   ```bash
   cd /Users/thibaut/supernovae-st/novanet-hq/apps/studio
   git status
   pnpm type-check
   ```

3. **Load Context**
   - Read CLAUDE.md
   - Check current branch
   - Review recent commits

4. **Show Quick Actions**
   ```
   ╔═══════════════════════════════════════════════════════════════╗
   ║  🎯 NOVANET STUDIO v8.2.0                                     ║
   ╠═══════════════════════════════════════════════════════════════╣
   ║                                                               ║
   ║  Quick Actions:                                               ║
   ║  • pnpm dev        → Start dev server (localhost:3000)        ║
   ║  • pnpm build      → Production build                         ║
   ║  • pnpm test       → Run Jest tests                           ║
   ║  • pnpm e2e        → Run Playwright E2E tests                 ║
   ║                                                               ║
   ║  From monorepo root:                                          ║
   ║  • pnpm infra:up   → Start Neo4j                              ║
   ║  • pnpm infra:seed → Seed database                            ║
   ║                                                               ║
   ║  Skills:                                                      ║
   ║  • react-flow-patterns.md                                     ║
   ║  • force-graph-patterns.md                                    ║
   ║  • zustand-patterns.md                                        ║
   ║  • neo4j-patterns.md                                          ║
   ║                                                               ║
   ╚═══════════════════════════════════════════════════════════════╝
   ```

5. **Ask** - "What would you like to work on?"

## Notes

- Always check `pnpm type-check` before starting work
- Use `pnpm` (not npm) for all package operations
- Reference skills for implementation patterns
- Use TodoWrite for multi-step tasks
