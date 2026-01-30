---
description: Validate or regenerate TypeScript/Mermaid from YAML sources
argument-hint: [validate|generate|fix]
allowed-tools: Bash
---

Synchronize generated files with YAML source of truth.

Commands:
- No argument or "validate": Run `pnpm schema:validate`
- "generate" or "fix": Run `pnpm schema:generate` then show `git diff --stat`

Use the novanet-sync skill for guidance.
