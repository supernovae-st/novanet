---
description: Look up NovaNet ADR by number or keyword (fuzzy search supported)
argument-hint: <number|keyword|domain> (e.g., "029", "native", "schema")
allowed-tools: Read, Glob, Grep
---

# ADR Lookup

Quick lookup for NovaNet Architecture Decision Records with fuzzy search.

---

## Overview

```
/adr 029              -> Find ADR-029 (*Native Pattern)
/adr native           -> Search ADRs containing "native"
/adr trait            -> Find ADRs about traits
/adr list             -> List all ADRs with summaries
/adr must-know        -> Show essential ADRs for daily development
/adr domain schema    -> List ADRs in schema-architecture domain
/adr slug url page    -> Multi-word search (matches any)
```

## Aliases & Common Typos

| You Type | Interpreted As |
|----------|----------------|
| `nativ`, `natve` | `native` (fuzzy) |
| `slugs`, `slug` | slug |
| `trait`, `traits` | trait |
| `arc`, `arcs`, `edge` | arc |
| `class`, `kind` | class (ADR-023) |
| `yaml`, `yaml-first` | yaml |
| `query`, `cypher` | query (ADR-021) |

---

## Workflow

### Step 1: Parse Query

Based on `$ARGUMENTS`:

| Pattern | Action |
|---------|--------|
| Numeric (e.g., `029`, `29`, `7`) | Find ADR by ID |
| `list` | Show all ADRs grouped by domain |
| `must-know` | Show essential v0.13.0 ADRs |
| `active` / `stable` / `deprecated` | Filter by status |
| `domain <name>` | Filter by domain (partial match) |
| Multi-word (e.g., `slug url`) | Match ANY word (OR search) |
| Other text | Fuzzy search in name, summary, tags |

**Fuzzy Search Algorithm:**
1. Exact match in name/summary/tags → Score 100
2. Partial match (substring) → Score 80
3. Similar spelling (Levenshtein ≤ 2) → Score 60
4. Related concept (via aliases) → Score 40

**Domain shortcuts:**
- `schema` → `schema-architecture`
- `node` → `node-classification`
- `vis` → `visual-encoding`
- `ux` → `ux-architecture`
- `dep` → `deprecated`

### Step 2: Read Index

Read the ADR index file (in parent workspace):

```
../docs/adr/_index.yaml
```

Parse the YAML to get:
- `adrs[]`: All ADR entries with metadata
- `domains{}`: Domain groupings
- `by_status{}`: Status groupings
- `must_know{}`: Essential ADRs

### Step 3: Find Matches

**For numeric query:**
```
Find ADR where id == parseInt(query)
```

**For keyword query:**
Search case-insensitive in:
1. `name` field
2. `summary` field
3. `tags[]` array
4. `domain` field

### Step 4: Display Results

**Single match format:**

```
+==============================================================================+
|  ADR-{id}: {name}                                                            |
+==============================================================================+

Status:   {status} ({version})
Domain:   {domain}
File:     .claude/rules/adr/{file}
Summary:  {summary}
Tags:     {tags.join(", ")}

Dependencies: ADR-{depends_on.join(", ADR-")}
Supersedes:   {supersedes || "none"}
Superseded:   {superseded_by || "none"}

+------------------------------------------------------------------------------+

Read full content? [Enter to continue, or specify another ADR]
```

**Multiple matches format:**

```
+==============================================================================+
|  Found {N} ADRs matching "{query}"                                           |
+==============================================================================+

| ID  | Name                          | Status | Summary                      |
|-----|-------------------------------|--------|------------------------------|
| 024 | Trait = Data Origin           | active | Trait redefined as WHERE...  |
| 029 | *Native Pattern               | active | Unified *Native suffix...    |
...

Enter ADR number to view details, or refine search.
```

**List format (for `/adr list`):**

```
+==============================================================================+
|  NovaNet ADRs (v0.13.0) - 32 total                                           |
+==============================================================================+

CORE PRINCIPLES (5)
+-----+-------------------------------+--------+--------------------------------+
| 001 | Arc Terminology               | stable | Use 'Arc' (not Edge/Relation)  |
| 003 | YAML-First Architecture       | stable | YAML = single source of truth  |
| 007 | Generation, Not Translation   | stable | Native generation, not transl. |
| 010 | Skill-First DX                | stable | Update DX before code changes  |
| 021 | Query-First Architecture      | active | Cypher = source of truth       |
+-----+-------------------------------+--------+--------------------------------+

SCHEMA ARCHITECTURE (6)
...

[Continue for each domain]
```

**Must-know format (for `/adr must-know`):**

```
+==============================================================================+
|  Essential ADRs for v0.13.0 Development                                      |
+==============================================================================+

These ADRs are critical for daily development. Know them well.

| ID  | Name                          | Why Essential                        |
|-----|-------------------------------|--------------------------------------|
| 021 | Query-First Architecture      | How graph visualization works        |
| 022 | Unified Tree Architecture     | TUI/Studio navigation (2 modes)      |
| 024 | Trait = Data Origin           | 5 traits: defined/authored/imported/ |
| 025 | Instruction Layer             | PageStructure, PageInstruction naming|
| 029 | *Native Pattern               | EntityNative, PageNative naming      |
| 030 | Slug Ownership                | Page owns URL, Entity owns semantics |

Read any of these? Enter ADR number (e.g., 029).
```

### Step 5: Offer Full Content

After displaying summary, offer to read full ADR content:

```
Read the ADR file at: .claude/rules/adr/{file}
Display full content to user.
```

---

## Examples

```bash
# Find by number
/adr 029
-> Shows ADR-029 (*Native Pattern) details

# Search by keyword
/adr native
-> Finds ADR-029 (*Native Pattern), ADR-014 (deprecated)

# Find trait-related ADRs
/adr trait
-> Finds ADR-024, ADR-005

# List all by domain
/adr list
-> Shows all 32 ADRs grouped by domain

# Show essential ADRs
/adr must-know
-> Shows 6 critical ADRs for v0.13.0

# Filter by status
/adr active
-> Shows all active-status ADRs (21-32)

/adr deprecated
-> Shows superseded and historical ADRs

# Domain filter (shows domain README + ADRs)
/adr domain schema
-> Shows schema-architecture README + 6 ADRs in that domain

# Multi-word search (OR logic)
/adr slug url page
-> Finds ADRs matching "slug" OR "url" OR "page"

# Fuzzy/typo tolerant
/adr nativ
-> Still finds "native" (Levenshtein distance = 1)

# Combined queries
/adr domain arc inverse
-> Finds inverse-related ADRs within arc-design domain
```

---

## Quick Reference

| Domain | Key ADRs | Focus |
|--------|----------|-------|
| core-principles | 1, 3, 7, 10, 21 | Philosophy, methodology |
| schema-architecture | 6, 12, 17, 28, 29, 30 | Realm, layer, nodes |
| node-classification | 2, 23, 24, 25 | Naming, traits |
| arc-design | 15, 16, 26, 27 | Arc families, inverses |
| visual-encoding | 4, 5, 9, 13 | Colors, icons |
| ux-architecture | 8, 22 | TUI/Studio navigation |
| seo-geo | 31, 32 | SEO pillar/cluster, URL |
| deprecated | 11, 14, 18, 19, 20 | Historical reference |

---

## Related

| Command | Use For |
|---------|---------|
| `/novanet-arch` | Visual architecture diagrams |
| `/schema` | Schema operations (add/edit nodes) |
| `/codebase-audit` | Find ADR terminology violations |
