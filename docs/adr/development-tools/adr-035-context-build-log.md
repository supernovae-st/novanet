---
id: 35
title: "Context Build Log"
version: "v0.14.0"
status: active
domain: development-tools
---

# ADR-035: Context Build Log

**Status**: Approved (v0.14.0)

**Problem**: Debugging context assembly for LLM generation is opaque:
1. No visibility into which knowledge atoms were selected
2. No understanding of token budget allocation decisions
3. Difficult to diagnose why generated content references unexpected entities
4. No audit trail for context assembly decisions

**Decision**: Add `context_build_log` to `novanet_generate` MCP tool response for step-by-step observability of context assembly.

## context_build_log Schema

```rust
pub struct ContextBuildLog {
    /// Phase 1: Structure analysis (Page, Block hierarchy)
    pub structure_phase: StructurePhaseLog,

    /// Phase 2: Entity resolution and native content loading
    pub entities_phase: EntitiesPhaseLog,

    /// Phase 3: Knowledge atoms selection
    pub atoms_phase: AtomsPhaseLog,

    /// Phase 4: Context anchors calculation
    pub anchors_phase: AnchorsPhaseLog,

    /// Phase 5: Token budget decisions
    pub token_decisions: TokenDecisionsLog,
}
```

## Log Phases

### Phase 1: Structure Phase

```json
{
  "structure_phase": {
    "page_key": "create-qr-code",
    "block_count": 3,
    "blocks": [
      { "key": "head-seo-meta", "type": "seo-meta" },
      { "key": "hero-section", "type": "hero" },
      { "key": "features-grid", "type": "grid" }
    ],
    "traversal_depth": 2,
    "duration_ms": 12
  }
}
```

### Phase 2: Entities Phase

```json
{
  "entities_phase": {
    "primary_entity": "qr-code",
    "related_entities": ["dynamic-qr-code", "qr-code-art"],
    "entity_count": 3,
    "natives_loaded": [
      { "entity": "qr-code", "locale": "fr-FR", "forms_count": 5 }
    ],
    "duration_ms": 45
  }
}
```

### Phase 3: Atoms Phase

```json
{
  "atoms_phase": {
    "locale": "fr-FR",
    "terms_loaded": 23,
    "expressions_loaded": 8,
    "patterns_loaded": 12,
    "culture_refs_loaded": 3,
    "taboos_checked": 2,
    "domains_queried": ["qr-code", "marketing", "technology"],
    "duration_ms": 78
  }
}
```

### Phase 4: Anchors Phase

```json
{
  "anchors_phase": {
    "anchor_count": 7,
    "anchors": [
      { "type": "entity", "key": "qr-code", "relevance": 1.0 },
      { "type": "term", "key": "code-qr", "relevance": 0.9 },
      { "type": "expression", "key": "scanner-qr", "relevance": 0.8 }
    ],
    "spreading_activation_rounds": 2,
    "duration_ms": 34
  }
}
```

### Phase 5: Token Decisions

```json
{
  "token_decisions": {
    "budget_total": 4000,
    "budget_used": 3847,
    "budget_remaining": 153,
    "allocation": {
      "structure": 450,
      "entities": 1200,
      "atoms": 1800,
      "anchors": 397
    },
    "truncations": [
      { "type": "terms", "original": 45, "kept": 23, "reason": "budget" }
    ],
    "duration_ms": 5
  }
}
```

## Full Example Response

```json
{
  "prompt": "Generate landing page content for QR Code creation...",
  "evidence_summary": "Entity: qr-code with 3 related entities...",
  "locale_context": { "locale": "fr-FR", "voice": "professional" },
  "context_anchors": [...],
  "denomination_forms": [...],
  "context_build_log": {
    "structure_phase": { ... },
    "entities_phase": { ... },
    "atoms_phase": { ... },
    "anchors_phase": { ... },
    "token_decisions": { ... }
  }
}
```

## MCP Tool Update

The `novanet_generate` tool now includes `context_build_log` in its response:

```
novanet_generate
  params: focus_key, locale, mode, token_budget, spreading_depth
  returns: prompt, evidence_summary, locale_context, context_anchors,
           denomination_forms (ADR-033), context_build_log (ADR-035)
```

## Use Cases

### 1. Debugging Unexpected Content

When generated content references wrong entities:

```bash
# Check which entities were loaded
jq '.context_build_log.entities_phase.related_entities' response.json
```

### 2. Token Budget Tuning

When content is truncated unexpectedly:

```bash
# Check token allocation
jq '.context_build_log.token_decisions' response.json
```

### 3. Locale Audit

Verify locale-specific atoms are being loaded:

```bash
# Check atoms phase
jq '.context_build_log.atoms_phase' response.json
```

### 4. Performance Profiling

Identify slow phases:

```bash
# Sum all phase durations
jq '[.context_build_log | to_entries[] | .value.duration_ms] | add' response.json
```

## Nika Workflow Integration

Nika workflows can use `context_build_log` for debugging:

```yaml
tasks:
  - id: generate_with_debug
    invoke:
      mcp: novanet
      tool: novanet_generate
      params:
        entity: "qr-code"
        locale: "fr-FR"
        token_budget: 4000
    use.ctx: generation

  - id: log_debug
    use:
      gen: generation
    exec:
      command: |
        echo "Token budget used: {{use.gen.context_build_log.token_decisions.budget_used}}"
        echo "Entities loaded: {{use.gen.context_build_log.entities_phase.entity_count}}"
```

## Implementation

Location: `tools/novanet-mcp/src/tools/generate.rs`

Key changes:
1. Added `ContextBuildLog` struct with `JsonSchema` derive
2. Added timing instrumentation to each assembly phase
3. Included log in `GenerateResponse` struct
4. Updated MCP tool schema

## Benefits

1. **Transparency**: Full visibility into context assembly decisions
2. **Debuggability**: Pinpoint exactly where issues occur
3. **Optimization**: Identify slow phases and token waste
4. **Audit Trail**: Track which knowledge was used for generation
5. **DX**: Developers can understand and tune context assembly

## Rationale

**Why structured phases?**
- Maps to actual code flow (5 distinct assembly stages)
- Each phase has different debugging needs
- Easy to extend with additional metrics

**Why include timing?**
- Performance profiling without external tools
- Identify bottlenecks in production
- Compare across locales/entities

**Why JSON schema?**
- MCP clients can validate response structure
- IDE autocomplete for workflow authors
- Documentation generation

## Related ADRs

- ADR-033 (Denomination Forms): Returns canonical forms alongside context_build_log
- ADR-021 (Query-First): Context assembly uses Cypher queries internally
- ADR-030 (Slug Ownership): URL forms tracked in entities_phase

## Reference

- Implementation: `tools/novanet-mcp/src/tools/generate.rs`
- Schema: `ContextBuildLog` struct in `tools/novanet-mcp/src/types.rs`
- Tests: `tools/novanet-mcp/tests/generate_test.rs`
- DX ID: DX-11 in ROADMAP.md
