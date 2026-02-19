---
id: 10
title: "Skill-First DX"
version: "v9.5"
status: stable
domain: core-principles
---

# ADR-010: Skill-First DX

**Status**: Approved (v9.5)

**Decision**: Update DX tools (skills, CLAUDE.md, rules) BEFORE implementing code changes.

**Rationale**:
- Claude Code reads these files for context
- Outdated docs cause confusion during implementation
- DX is cheap to update, expensive to fix later
