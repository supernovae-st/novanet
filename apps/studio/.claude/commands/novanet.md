# /novanet - Session Start

Starts a NovaNet Studio development session.

## Trigger

`/novanet`

## Actions

1. **Greet** - "Hello Thibaut! Starting NovaNet Studio session."

2. **Check Status**
   ```bash
   cd /Users/thibaut/Projects/traduction_ai/novanet/studio
   git status
   npm run type-check
   ```

3. **Load Context**
   - Read CLAUDE.md
   - Check current branch
   - Review recent commits

4. **Show Quick Actions**
   ```
   ╔═══════════════════════════════════════════════════════════════╗
   ║  🎯 NOVANET STUDIO                                             ║
   ╠═══════════════════════════════════════════════════════════════╣
   ║                                                               ║
   ║  Quick Actions:                                               ║
   ║  • npm run dev      → Start dev server                        ║
   ║  • npm run build    → Production build                        ║
   ║  • npm test         → Run tests                               ║
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

- Always check `npm run type-check` before starting work
- Reference skills for implementation patterns
- Use TodoWrite for multi-step tasks
