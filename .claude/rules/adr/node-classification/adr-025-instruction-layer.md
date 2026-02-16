---
id: "025"
title: "Instruction Layer Renaming"
version: "v11.8"
status: "active"
domain: "node-classification"
---

# ADR-025: Instruction Layer Renaming

**Status**: Approved (v11.8)

**Problem**: Current instruction layer names don't reflect their function:

- `PageType` → actually defines page STRUCTURE (JSON with headers, sections)
- `BlockType` → defines block JSON schema (this one is OK)
- `PagePrompt` → actually contains page INSTRUCTIONS (markdown with @ refs)
- `BlockPrompt` → contains block INSTRUCTIONS (markdown with @ refs)

The existing Studio UI already uses the correct names: "Page Structures" and "Page Instructions".

**Decision**: Rename to match function and existing UI:

| Before | After | Function |
|--------|-------|----------|
| PageType | **PageStructure** | JSON defining which BlockTypes in what order |
| BlockType | **BlockType** | (keep) JSON schema for a block |
| PagePrompt | **PageInstruction** | Markdown with LLM directives and @ references |
| BlockPrompt | **BlockInstruction** | Markdown with LLM directives and @ references |

**Arc Changes**:

| Before | After |
|--------|-------|
| `[:OF_TYPE]` (Page→PageType) | `[:HAS_STRUCTURE]` (Page→PageStructure) |
| `[:HAS_PROMPT]` (Page→PagePrompt) | `[:HAS_INSTRUCTION]` (Page→PageInstruction) |
| `[:OF_TYPE]` (Block→BlockType) | `[:OF_TYPE]` (keep - BlockType unchanged) |
| `[:HAS_PROMPT]` (Block→BlockPrompt) | `[:HAS_INSTRUCTION]` (Block→BlockInstruction) |

**@ Reference System**:

Instructions support @ references that resolve at generation time:

```markdown
# PageInstruction example
Generate pricing page comparing @entity:tier-pro vs @entity:tier-basic
See @page:features for product consistency

# BlockInstruction example
[TRANSLATE] title: Highlight benefits of @entity:tier-pro
[FIXED] cta_url: /signup
```

At generation time:
- `@entity:tier-pro` → loads `EntityNative(tier-pro@{locale})`
- `@page:features` → loads `Page(features)` context
- `[TRANSLATE]` → field needs locale-native generation
- `[FIXED]` → field is invariant (URLs, technical values)

**Pipeline**:

```
Page
├── [:HAS_STRUCTURE] → PageStructure
│   └── JSON compilé depuis l'ordre des blocks:
│       { "blocks": ["hero", "features", "pricing", "cta"] }
│
├── [:HAS_INSTRUCTION] → PageInstruction
│   └── Markdown compilé depuis BlockInstructions (dans l'ordre):
│       BlockInstruction₁ + BlockInstruction₂ + ... = PageInstruction
│
└── [:HAS_BLOCK {order: N}] → Block (l'ordre est sur l'arc!)
    │
    ├── [:OF_TYPE] → BlockType
    │   └── JSON Schema: { slug, title, description, cta_url, ... }
    │
    └── [:HAS_INSTRUCTION] → BlockInstruction
        └── Markdown avec @ références
```

**L'ordre des blocs** (propriété `order` sur [:HAS_BLOCK]) détermine:
1. **PageStructure JSON** — L'ordre des BlockTypes
2. **PageInstruction** — La compilation séquentielle des BlockInstructions
3. **PageNative** — L'ordre final du contenu généré

**Rationale**:

1. **PageStructure**: Describes WHAT it is (the structure combining blocks)
2. **PageInstruction**: Describes WHAT it is (instructions for LLM)
3. **BlockType**: Already correct (defines the type/schema of a block)
4. **BlockInstruction**: Consistent with PageInstruction
5. **Aligned with UI**: The existing Studio UI uses these exact names

**Reference**: `docs/plans/2026-02-13-nomenclature-v118-design.md`
