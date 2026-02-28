# NovaNet Release Template

> Shared template: See `dx/.github/RELEASE_TEMPLATE.md` for full documentation.

## Title Format

```
{emoji} NovaNet v{version} — {Tagline}
```

## Body Template

```markdown
## ⚠️ Breaking Changes (if any)

### {Change Name}
- Description of breaking change
- Migration: `command or code`

## ✨ New Features

### {Feature Name}
- Feature description
- Usage example if applicable

## 🔄 Changed

- **{Component}** - Description of change

## 🐛 Fixed

- **{Bug}** - Description of fix

## 🔄 Migration (if breaking)

\`\`\`bash
# Migration commands
cargo run -- schema generate
cargo run -- db migrate
\`\`\`

## 📊 Stats

- **{N} Rust tests passing**
- **{N} nodes** ({N} shared + {N} org), **{N} layers**, **{N} traits**
- **{N} arcs** ({N} families)
- Schema validation: 0 errors, 0 warnings

---
**Full Changelog**: https://github.com/supernovae-st/novanet/compare/v{prev}...v{current}
```

## NovaNet-Specific Sections

### For Schema Changes
```markdown
## 🔷 Schema Changes

| Before | After | ADR |
|--------|-------|-----|
| `OldName` | `NewName` | ADR-XXX |
```

### For MCP Tool Additions
```markdown
## 🔍 MCP Tools

### novanet_{tool_name}
- **Purpose**: Description
- **Params**: `param1`, `param2`
- **Returns**: Description
```

## Emoji Quick Reference

| Context | Emoji |
|---------|-------|
| SemVer/Migration | `🔄` |
| Schema/Arc changes | `🔷` |
| Architecture | `🏛️` |
| MCP/Introspection | `🔍` |
| Breaking changes | `⚠️` |
| New features | `✨` |
