---
description: Validate or regenerate artifacts from YAML sources
argument-hint: [validate|generate|fix]
allowed-tools: Bash
---

Synchronize generated files with YAML source of truth.

Commands:
- No argument or "validate": Run `novanet schema validate`
- "generate" or "fix": Run `novanet schema generate` then show `git diff --stat`

Use the novanet-sync skill for guidance.
