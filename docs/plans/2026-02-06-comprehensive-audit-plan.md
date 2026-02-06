# Comprehensive Audit Plan - 2026-02-06

## Overview

Full codebase audit following v10.6 Icons Source of Truth implementation.
Multi-phase parallel agent execution for Rust code review and DX coherence verification.

## Phase 1: Rust Code Review (10 Parallel Agents)

### Agent Distribution

| Agent # | Module | Focus Area |
|---------|--------|------------|
| 1 | `generators/tui_icons.rs` | New TUI icons generator |
| 2 | `generators/visual_encoding.rs` | Icons section additions |
| 3 | `generators/mod.rs` + `commands/schema.rs` | Generator registration |
| 4 | `tui/theme.rs` | Icons struct + YAML loading |
| 5 | `tui/icons.rs` (generated) | Compile-time constants |
| 6 | `parsers/visual_encoding.rs` | Icons parsing |
| 7 | `tui/mod.rs` + `tui/app.rs` | TUI integration |
| 8 | Error handling across all modules | thiserror patterns |
| 9 | Test coverage | All icon-related tests |
| 10 | Performance | YAML loading, HashMap usage |

### Review Criteria

- Code quality and Rust idioms
- Error handling completeness
- Test coverage adequacy
- Documentation accuracy
- Clippy compliance
- Security considerations

## Phase 2: DX Documentation Coherence

### Files to Audit

**Root Level:**
- `CLAUDE.md` - Version, artifacts count, icons mention
- `README.md` - Overview accuracy
- `ROADMAP.md` - v10.6 status

**Tools/novanet:**
- `tools/novanet/CLAUDE.md` - Generator list, test count, patterns
- `tools/novanet/KEYBINDINGS.md` - Icon references

**Rules:**
- `.claude/rules/novanet-terminology.md` - Icons terminology
- `.claude/rules/novanet-decisions.md` - ADR-013 completeness

**Skills:**
- `.claude/skills/novanet-architecture/SKILL.md` - Icons section
- `.claude/skills/novanet-sync/SKILL.md` - Schema generate

**Agents:**
- `.claude/agents/code-reviewer.md` - NovaNet patterns
- `.claude/agents/neo4j-architect.md` - Schema awareness

**Hooks:**
- `.claude/hooks/` - Doc sync hooks

**Packages:**
- `packages/core/CLAUDE.md` - Types coherence
- `packages/db/CLAUDE.md` - Seed files list
- `apps/studio/CLAUDE.md` - Design system

### Coherence Checks

1. Version numbers consistent (v10.6.0)
2. Artifact counts match (12 generators)
3. Test counts accurate (499 tests)
4. Icon categories listed correctly
5. File paths accurate
6. Code examples valid

## Phase 3: Ralph Wiggum Loops (Codebase Audit)

### Loop 1: Dead Code Detection
- Unused functions
- Unreachable code
- Deprecated patterns

### Loop 2: Inconsistency Detection
- Naming convention violations
- Style inconsistencies
- Import patterns

### Loop 3: Legacy Pattern Detection
- Old terminology (Edge vs Arc)
- Deprecated v10.5 patterns
- Stale comments

### Loop 4: Test Gap Analysis
- Untested code paths
- Missing edge cases
- Integration test gaps

## Phase 4: Synthesis and Fixes

1. Collect all agent findings
2. Prioritize by severity
3. Fix critical issues
4. Update documentation
5. Run validation

## Phase 5: Final Validation

```bash
cargo test --lib
cargo clippy --all-features -- -D warnings
cargo run -- schema validate
cargo run -- schema generate --dry-run
```

## Success Criteria

- [ ] All 499+ tests pass
- [ ] Zero clippy warnings
- [ ] Schema validation: 0 errors, 0 warnings
- [ ] All documentation coherent
- [ ] No dead code detected
- [ ] No legacy patterns remaining
