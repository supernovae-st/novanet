---
id: "027"
title: "Generation Family Arc Semantics"
version: "v0.12.1"
status: "active"
domain: "arc-design"
---

# ADR-027: Generation Family Arc Semantics

**Status**: Approved (v0.12.1)

**Problem**: The generation family arcs lacked clear documentation and consistent llm_context patterns, making it difficult to understand:
- The generation pipeline flow (Instruction → PromptArtifact → Generated → Output)
- When to use each arc for different traversal patterns
- How to distinguish similar arcs (GENERATED vs HAS_NATIVE)

**Decision**: Document the generation family semantics with clear flow diagrams and standardized llm_context.

## Generation Pipeline Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  GENERATION PIPELINE                                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. AUTHORING (instruction layer)                                           │
│     PageInstruction ──[:INCLUDES_STYLE]──> Style                            │
│     BlockInstruction ──[:INCLUDES_STYLE]──> Style                           │
│                                                                             │
│  2. COMPILATION (instruction → prompt)                                      │
│     PageInstruction ──[:COMPILED_FROM]──< PromptArtifact                    │
│     PromptArtifact ──[:INCLUDES_ENTITY]──> Entity                           │
│                                                                             │
│  3. GENERATION (prompt → content)                                           │
│     BlockInstruction ──[:GENERATED]──> BlockNative                          │
│     PageInstruction ──[:GENERATED]──> PageNative                            │
│                                                                             │
│  4. PROVENANCE (tracking)                                                   │
│     BlockNative ──[:INFLUENCED_BY]──> EntityNative                          │
│     BlockNative ──[:GENERATED_FROM]──> BlockType                            │
│                                                                             │
│  5. OUTPUT (assembly & deployment)                                          │
│     Page ──[:HAS_NATIVE]──> PageNative                                      │
│     PageNative ──[:ASSEMBLES]──> BlockNative                                │
│     OutputArtifact ──[:BUNDLES]──> PageNative                               │
│     *Generated ──[:PREVIOUS_VERSION]──> *Generated                          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Arc Disambiguation

| Arc | Direction | Purpose | When to Use |
|-----|-----------|---------|-------------|
| `GENERATED` | Instruction → Generated | Provenance | "Which instruction made this?" |
| `HAS_NATIVE` | Structure → Generated | Ownership | "What's the output for this page?" |
| `NATIVE_OF` | Generated → Structure | Inverse | "Which page owns this output?" |
| `GENERATED_FROM` | Generated → Type | Validation | "Is this block schema-valid?" |
| `COMPILED_FROM` | Artifact → Instruction | Audit | "What template made this prompt?" |
| `INCLUDES_ENTITY` | Artifact → Entity | Context | "What entities were in the prompt?" |
| `INCLUDES_STYLE` | Instruction → Style | Config | "What style settings apply?" |
| `INFLUENCED_BY` | Generated → Content | Attribution | "What content influenced output?" |
| `ASSEMBLES` | PageGen → BlockGen | Render | "What blocks in what order?" |
| `BUNDLES` | Artifact → Generated | Deploy | "What's in this release?" |
| `PREVIOUS_VERSION` | Generated → Generated | History | "What was the previous version?" |

## llm_context Standard Pattern

All generation family arcs now follow the USE/TRIGGERS/NOT/RELATES pattern:

```yaml
llm_context: |
  USE: when [primary use case].
  TRIGGERS: "keyword1", "keyword2", "keyword3".
  NOT: for [disambiguation] (use [alternative] instead).
  RELATES: [Source] (source), [Target] (target), [Related Arc] (relationship).
```

**Rationale**:

1. **Pipeline Clarity**: Clear separation of authoring, compilation, generation, and output phases
2. **Arc Disambiguation**: "GENERATED" (provenance) vs "HAS_NATIVE" (ownership) is now documented
3. **LLM Context**: Standardized llm_context enables better RAG and spreading activation
4. **Audit Trail**: Complete provenance from instruction through prompt to final output

**Reference**: Generation family arc files in `packages/core/models/arc-classes/generation/`
