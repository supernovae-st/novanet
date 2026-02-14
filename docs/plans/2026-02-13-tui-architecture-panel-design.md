# TUI Architecture Panel & Nexus Arch Tab

**Status**: Approved
**Version**: v0.12.5
**Date**: 2026-02-13

---

## Overview

Add architecture documentation to the TUI at two levels:
1. **Graph mode**: Contextual architecture diagrams in the detail panel
2. **Nexus mode**: New "Arch" tab for browsing ADRs

---

## Design Decisions

### Decision 1: Multi-Level Documentation

| Level | Location | Content |
|-------|----------|---------|
| Contextual | Graph > Detail panel | ASCII ER diagram for key classes |
| Reference | Nexus > Arch tab | Full ADR browser with categories |

### Decision 2: Detail Panel Redesign

Split the detail panel into distinct boxes for better visual organization.

### Decision 3: Arch Tab in Nexus

New tab alongside Quiz, Audit, Stats, Help for architecture documentation.

---

## Graph Mode: Detail Panel Layout

### Current Layout

```
┌──────────────┬────────────────────────────────────────┬──────────────────┐
│ TREE         │ DETAIL (flat content)                  │ YAML             │
│ (full        │                                        │ (full height)    │
│  height)     │                                        │                  │
│              │ Arc Relationships (bottom)             │                  │
└──────────────┴────────────────────────────────────────┴──────────────────┘
```

### New Layout

```
┌──────────────┬────────────────────────────────────────┬──────────────────┐
│              │ ┌────────────────────────────────────┐ │                  │
│              │ │ HEADER: name, realm, layer, trait  │ │ YAML             │
│              │ └────────────────────────────────────┘ │                  │
│              │ ┌─────────────────┐ ┌────────────────┐ │                  │
│ TREE         │ │ PROPERTIES      │ │ COVERAGE       │ │                  │
│ (full        │ │ count, bar      │ │ req/opt %      │ │                  │
│  height)     │ └─────────────────┘ └────────────────┘ │                  │
│              │ ┌────────────────────────────────────┐ ├──────────────────┤
│              │ │ PROPERTY LIST (scrollable)         │ │ ARCHITECTURE     │
│              │ └────────────────────────────────────┘ │                  │
│              ├────────────────────────────────────────┤ (diagram)        │
│              │ ARC RELATIONSHIPS                      │                  │
│              │ outgoing / incoming                    │ [r] ADR link     │
└──────────────┴────────────────────────────────────────┴──────────────────┘
```

### Detail Panel Boxes

| Box | Content | Height |
|-----|---------|--------|
| **Header** | name, realm, layer, trait, instances | 3 lines |
| **Properties** | count bar, budget | 3 lines |
| **Coverage** | required/optional % bars | 3 lines |
| **Property List** | scrollable property list | flex |
| **Arc Relationships** | outgoing/incoming arcs | ~40% bottom |
| **Architecture** | ER diagram (right side of bottom) | ~40% bottom |

---

## Architecture Diagrams

### Classes with Dedicated Diagrams

| Class | Diagram Focus |
|-------|---------------|
| **Page** | Page ↔ Entity (1:1), HAS_BLOCK, USES_ENTITY |
| **Entity** | HAS_CONTENT, BELONGS_TO, SEMANTIC_LINK |
| **Block** | OF_TYPE, HAS_INSTRUCTION, USES_ENTITY |
| **Brand** | HAS_DESIGN, HAS_PRINCIPLES, HAS_PROMPT_STYLE |
| **Locale** | HAS_VOICE, HAS_CULTURE, FOR_LOCALE |
| **Project** | HAS_PAGE, HAS_ENTITY, HAS_BRAND |

### Page Architecture Diagram

```
        Project
          │
          │[:HAS_PAGE]
          ▼
┌────────────────────────────┐
│                            │
│ Page ══[:REPRESENTS]══▶    │
│   │      (1:1)      Entity │
│   │                   │    │
│   │[:HAS_BLOCK]       │[:HAS_CONTENT]
│   │  {order}          │    │
│   ▼                   ▼    │
│ Block          EntityContent
│   │                        │
│   └──[:USES_ENTITY]──▶     │
│              Entity        │
└────────────────────────────┘

[r] Jump to ADR-028
```

### Entity Architecture Diagram

```
       Project
          │
          │[:HAS_ENTITY]
          ▼
┌────────────────────────────┐
│                            │
│ Entity ──[:BELONGS_TO]──▶  │
│   │         EntityCategory │
│   │                        │
│   │[:HAS_CONTENT]          │
│   ▼                        │
│ EntityContent              │
│   │                        │
│   │[:FOR_LOCALE]           │
│   ▼                        │
│ Locale                     │
│                            │
│ ──[:SEMANTIC_LINK]──▶      │
│    {temp, link_type} Entity│
└────────────────────────────┘

[r] Jump to ADR-028
```

### Brand Architecture Diagram

```
       Project
          │
          │[:HAS_BRAND]
          ▼
┌────────────────────────────┐
│                            │
│ Brand ──[:HAS_DESIGN]──▶   │
│   │         BrandDesign    │
│   │                        │
│   │──[:HAS_PRINCIPLES]──▶  │
│   │        BrandPrinciples │
│   │                        │
│   │──[:HAS_PROMPT_STYLE]──▶│
│   │        PromptStyle     │
│   │                        │
│   └──[:TARGETS_PERSONA]──▶ │
│          AudiencePersona   │
└────────────────────────────┘

[r] Jump to ADR-028
```

---

## Nexus Mode: Arch Tab

### Header Update

```
[1]Graph [2]Nexus: [Q]uiz [A]udit [S]tats [H]elp [R]ch
                                                 ─┬─
                                                  └── "R" for aRch
```

### Arch Tab View

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│ 📐 Architecture Decision Records                                                │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ▼ Core Principles                                                              │
│    ├── ADR-007 Generation NOT Translation                                       │
│    ├── ADR-003 YAML-First Architecture                                          │
│    └── ADR-001 Arc Terminology                                                  │
│                                                                                 │
│  ▼ Schema Architecture (v0.12.x)                                                │
│    ├── ADR-028 Page-Entity Architecture ★                                       │
│    ├── ADR-024 Trait = Data Origin                                              │
│    ├── ADR-023 Class/Instance Terminology                                       │
│    └── ADR-025 Instruction Layer                                                │
│                                                                                 │
│  ▼ UX Architecture                                                              │
│    ├── ADR-022 Unified Tree (Graph/Nexus)                                       │
│    ├── ADR-021 Query-First Architecture                                         │
│    └── ADR-013 Icons Source of Truth                                            │
│                                                                                 │
│  ▼ Arc Policies                                                                 │
│    ├── ADR-026 Inverse Arc Policy                                               │
│    └── ADR-027 Generation Family Semantics                                      │
│                                                                                 │
│  ▶ Layer Evolution (v11.x history)                                              │
│                                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│ [j/k] Navigate  [Enter] View ADR  [/] Search  [q] Back                          │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### ADR Detail View

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│ ADR-028: Page-Entity Architecture                               v0.12.3         │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  STATUS: Approved                                                               │
│                                                                                 │
│  SUMMARY                                                                        │
│  ─────────────────────────────────────────────────────────────────────────────  │
│  • Page ↔ Entity = 1:1 obligatoire via [:REPRESENTS]                            │
│  • Slug = Entity.key (source of truth)                                          │
│  • Order on arc: [:HAS_BLOCK {order}]                                           │
│  • PageStructure/PageInstruction = CALCULATED (not stored)                      │
│                                                                                 │
│  DIAGRAM                                                                        │
│  ─────────────────────────────────────────────────────────────────────────────  │
│  Page ──[:REPRESENTS]──▶ Entity (1:1 mandatory)                                 │
│    │                        │                                                   │
│    │[:HAS_BLOCK {order}]    │[:HAS_CONTENT]                                     │
│    ▼                        ▼                                                   │
│  Block                 EntityContent@locale                                     │
│                                                                                 │
│  KEY RULES                                                                      │
│  ─────────────────────────────────────────────────────────────────────────────  │
│  • Every Page MUST have exactly one Entity                                      │
│  • SEO Keywords live on Entity, not Page                                        │
│  • Block.key = "{page_key}:{block_type}:{index}"                                │
│                                                                                 │
│  RELATED CLASSES                                                                │
│  ─────────────────────────────────────────────────────────────────────────────  │
│  Page, Entity, Block, EntityContent, BlockType, BlockInstruction                │
│                                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│ [Esc] Back  [n/p] Next/Prev  [g] Jump to Graph class                            │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Keybindings

### New Keybindings

| Key | Context | Action |
|-----|---------|--------|
| `r` | Graph (on class with diagram) | Jump to Nexus > Arch > related ADR |
| `R` | Nexus | Switch to Arch tab |
| `g` | Nexus > Arch > ADR detail | Jump to Graph > related class |

### Updated Header

```
[1]Graph [2]Nexus: [Q]uiz [A]udit [S]tats [H]elp [R]ch
```

---

## Implementation

### Files to Modify

| File | Changes |
|------|---------|
| `ui/mod.rs` | Layout redesign, bottom split |
| `ui/graph.rs` | Architecture diagram rendering |
| `ui/detail.rs` | **NEW**: Detail panel boxes |
| `nexus/mod.rs` | Add `Arch` to `NexusTab` enum |
| `nexus/arch.rs` | **NEW**: ADR browser |
| `data.rs` | Architecture diagram data |
| `app.rs` | Keybinding `r` for ADR jump |

### Data Structures

```rust
// Architecture diagram for a class
pub struct ArchitectureDiagram {
    pub class_name: String,
    pub adr_id: String,        // "ADR-028"
    pub diagram: Vec<String>,  // ASCII lines
}

// ADR entry for Nexus
pub struct AdrEntry {
    pub id: String,           // "ADR-028"
    pub title: String,        // "Page-Entity Architecture"
    pub version: String,      // "v0.12.3"
    pub status: String,       // "Approved"
    pub category: AdrCategory,
    pub summary: Vec<String>,
    pub diagram: Vec<String>,
    pub key_rules: Vec<String>,
    pub related_classes: Vec<String>,
}

pub enum AdrCategory {
    CorePrinciples,
    SchemaArchitecture,
    UxArchitecture,
    ArcPolicies,
    LayerEvolution,
}
```

### ADR Data Source

Parse ADRs from `.claude/rules/novanet-decisions.md` at startup:
- Extract ADR-XXX sections
- Parse status, summary, diagrams
- Group by category

---

## Migration

No breaking changes. Additive feature only.

---

## References

- ADR-028: Page-Entity Architecture
- ADR-022: Unified Tree Architecture
- Brainstorm session 2026-02-13
