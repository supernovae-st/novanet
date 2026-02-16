---
name: novanet-adr
description: Navigate NovaNet Architecture Decision Records (ADRs). Use when user asks about ADRs, architecture decisions, why something was designed a certain way, or needs to look up specific decision records by number, domain, status, or keyword.
disable-model-invocation: false
user-invocable: true
---

# NovaNet ADR Navigation

Navigate and look up NovaNet Architecture Decision Records from the machine-readable index.

## Instructions

Based on `$ARGUMENTS`, perform the appropriate lookup:

1. **Read the index first**: Always start by reading `.claude/rules/adr/_index.yaml`
2. **Parse the lookup type** from arguments:
   - `ADR-XXX` or number → Look up specific ADR by ID
   - Domain name → List ADRs in that domain
   - Status (`active`, `stable`, `superseded`, `historical`) → Filter by status
   - Keyword/tag → Search tags array
   - `must-know` or `essential` → Show the 6 must-know ADRs for v0.13.0
   - Empty or `list` → Show domain overview

3. **Return structured output** with:
   - ADR number and name
   - Status badge
   - Summary
   - File path for full content
   - Related ADRs (depends_on, superseded_by)

---

## Domain Quick Reference

| Domain | Description | Key ADRs |
|--------|-------------|----------|
| **core-principles** | Foundation philosophy | 001 (Arc), 003 (YAML-First), 007 (Generation), 021 (Query-First) |
| **schema-architecture** | Realm, layer, node org | 012 (2-Realm), 029 (*Native), 030 (Slug Ownership) |
| **node-classification** | Naming, traits, axes | 023 (Class/Instance), 024 (Trait=Origin), 025 (Instruction) |
| **arc-design** | Arc families, inverses | 026 (Inverse Policy), 027 (Generation Family) |
| **visual-encoding** | Colors, icons | 004 (No Color Dup), 013 (Icons Source) |
| **ux-architecture** | TUI/Studio navigation | 022 (Unified Tree) |
| **seo-geo** | SEO pillar/cluster, URLs | 031 (Pillar/Cluster), 032 (Slugification) |
| **deprecated** | Superseded/historical | 011, 014, 018, 019, 020 |

---

## Must-Know ADRs for v0.13.0

These 6 ADRs are essential for daily development:

| ADR | Name | Why Essential |
|-----|------|---------------|
| **029** | *Native Pattern | Unified suffix for locale-specific nodes (EntityNative, PageNative) |
| **030** | Slug Ownership | Page owns URL (slug, full_path), Entity owns semantics (key) |
| **024** | Trait = Data Origin | 5 traits: defined/authored/imported/generated/retrieved |
| **025** | Instruction Layer | PageStructure, PageInstruction, BlockInstruction naming |
| **021** | Query-First | Cypher is the source of truth for graph visualization |
| **022** | Unified Tree | 2 TUI/Studio modes: Graph + Nexus |

---

## Lookup Examples

### By ADR Number

```
/novanet-adr 029
/novanet-adr ADR-024
```

Returns full metadata including summary, version, status, dependencies, tags, and file path.

### By Domain

```
/novanet-adr schema-architecture
/novanet-adr arc-design
```

Lists all ADRs in that domain with status badges.

### By Status

```
/novanet-adr active    # 12 ADRs currently active
/novanet-adr stable    # 15 stable foundation ADRs
/novanet-adr superseded
```

### By Tag/Keyword

```
/novanet-adr native     # Tags containing "native"
/novanet-adr trait      # ADRs about traits
/novanet-adr naming     # Naming conventions
```

### Essential ADRs

```
/novanet-adr must-know
/novanet-adr essential
```

Shows the 6 must-know ADRs for v0.13.0.

---

## Status Badges

| Status | Meaning | Badge |
|--------|---------|-------|
| `stable` | Foundation, unlikely to change | `[STABLE]` |
| `active` | Current, may evolve | `[ACTIVE]` |
| `superseded` | Replaced by another ADR | `[SUPERSEDED]` |
| `historical` | Migration record, informational only | `[HISTORICAL]` |

---

## Output Format

When returning ADR information, use this format:

```
ADR-029: *Native Pattern [ACTIVE]
Version: v0.12.5 | Domain: schema-architecture

Summary: Unified *Native suffix for locale-specific nodes (EntityNative, PageNative)

Tags: schema, naming, localization
Depends on: ADR-024
Supersedes: ADR-014

Full content: .claude/rules/adr/schema-architecture/adr-029-native-pattern.md
```

---

## Quick Stats (v0.13.0)

- **Total ADRs**: 32
- **Stable**: 15 | **Active**: 12 | **Superseded**: 2 | **Historical**: 3
- **Domains**: 8
- **Must-know for daily work**: 6

---

## Related Documentation

- **Full ADR Index**: `.claude/rules/novanet-decisions.md`
- **Terminology**: `.claude/rules/novanet-terminology.md`
- **Arc Design Guide**: `.claude/rules/arc-design-guide.md`
