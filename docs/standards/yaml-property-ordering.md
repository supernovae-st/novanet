# YAML Property Ordering Standard (v0.13.1)

> **ADR Reference**: This document establishes the canonical property ordering for NovaNet YAML schema files.
> **Effective**: v0.13.1
> **Scope**: All `node-classes/*.yaml` and `arc-classes/*.yaml` files

## Design Principles

1. **LLM-First**: Semantic fields (`description`, `llm_context`) appear early for optimal RAG retrieval
2. **Fail-Fast**: Required identity fields first for early validation errors
3. **Logical Grouping**: Related properties grouped into numbered BLOCs

---

## Node-Class Canonical Order

```yaml
# ═══════════════════════════════════════════════════════════════════════════════
# NodeClass: {Name} — {One-line description}
# v0.13.1: LLM-First property ordering
# ═══════════════════════════════════════════════════════════════════════════════

node:
  # ─────────────────────────────────────────────────────────────────────────────
  # BLOC 1: IDENTITY (required)
  # ─────────────────────────────────────────────────────────────────────────────
  name: NodeName                    # PascalCase
  realm: shared | org               # WHERE does this node live?
  layer: config | locale | ...      # WHAT functional category?
  trait: defined | authored | ...   # HOW is data created?

  # ─────────────────────────────────────────────────────────────────────────────
  # BLOC 2: SEMANTIC (LLM-CRITICAL)
  # ─────────────────────────────────────────────────────────────────────────────
  description: |
    Detailed description of this node type.
    Can be multi-line for complex nodes.

  llm_context: |
    USE: when [primary use case].
    TRIGGERS: "keyword1", "keyword2", "keyword3".
    NOT: for [disambiguation] (use [alternative] instead).
    RELATES: [Related1] (relationship), [Related2] (relationship).

  # ─────────────────────────────────────────────────────────────────────────────
  # BLOC 3: VISUAL
  # ─────────────────────────────────────────────────────────────────────────────
  icon:
    web: lucide-icon-name           # For Studio/web UI
    terminal: "◆"                   # Unicode for TUI

  # ─────────────────────────────────────────────────────────────────────────────
  # BLOC 4: PROPERTIES
  # ─────────────────────────────────────────────────────────────────────────────
  standard_properties:
    key:
      type: string
      required: true
      description: "Unique identifier"
    display_name:
      type: string
      required: true
      description: "Human-readable name"
    created_at:
      type: datetime
      required: true
    updated_at:
      type: datetime
      required: true

  # Custom property sections (optional, before main properties)
  embedding_properties:             # For nodes with vector embeddings
    # ...

  routing_properties:               # For nodes with URL routing
    # ...

  properties:
    property_name:
      type: string | int | float | bool | datetime | array | object
      required: true | false
      enum: [value1, value2]        # Optional: allowed values
      default: value                # Optional: default value
      min: 0                        # Optional: for numeric types
      max: 100                      # Optional: for numeric types
      description: "Property description"
      example: "example value"

  # ─────────────────────────────────────────────────────────────────────────────
  # BLOC 5: RELATIONS
  # ─────────────────────────────────────────────────────────────────────────────
  relations:
    ARC_NAME:
      to: TargetNodeClass
      cardinality: "1:N"
      description: "Outgoing arc description"

  incoming_relations:
    INCOMING_ARC:
      from: SourceNodeClass
      cardinality: "N:1"
      description: "Incoming arc description"

  # ─────────────────────────────────────────────────────────────────────────────
  # BLOC 6: REFERENCE
  # ─────────────────────────────────────────────────────────────────────────────
  example:
    data:
      key: "example-key"
      display_name: "Example Node"
      # ... sample instance data

    cypher: |
      // Query to fetch this node type
      MATCH (n:NodeName {key: $key})
      RETURN n

  neo4j:                            # Optional: Neo4j-specific config
    indexes:
      - property: key
        type: unique
```

---

## Arc-Class Canonical Order

```yaml
# ═══════════════════════════════════════════════════════════════════════════════
# ArcClass: ARC_NAME — {One-line description}
# v0.13.1: LLM-First property ordering
# ═══════════════════════════════════════════════════════════════════════════════

arc:
  # ─────────────────────────────────────────────────────────────────────────────
  # BLOC 1: METADATA (optional - only for deprecated/experimental arcs)
  # ─────────────────────────────────────────────────────────────────────────────
  status: deprecated | experimental # Only if not "active"
  deprecation_note: "Use X instead" # Required if status: deprecated
  removal_version: "v0.14.0"        # Required if status: deprecated

  # ─────────────────────────────────────────────────────────────────────────────
  # BLOC 2: IDENTITY (required)
  # ─────────────────────────────────────────────────────────────────────────────
  name: ARC_NAME                    # UPPER_SNAKE_CASE
  family: ownership | localization | semantic | generation | mining
  scope: intra_realm | cross_realm

  # ─────────────────────────────────────────────────────────────────────────────
  # BLOC 3: SEMANTIC (LLM-CRITICAL)
  # ─────────────────────────────────────────────────────────────────────────────
  description: |
    Detailed description of this arc type.
    Explains the relationship semantics.

  llm_context: |
    USE: when [primary use case].
    TRIGGERS: "keyword1", "keyword2", "keyword3".
    NOT: for [disambiguation] (use [alternative] instead).
    RELATES: [Source] (role), [Target] (role), [Related Arc] (relationship).

  # ─────────────────────────────────────────────────────────────────────────────
  # BLOC 4: GRAPH STRUCTURE (required)
  # ─────────────────────────────────────────────────────────────────────────────
  source: SourceNodeClass           # Or [NodeClass1, NodeClass2, ...]
  target: TargetNodeClass           # Or [NodeClass1, NodeClass2, ...]
  cardinality: one_to_one | one_to_many | many_to_one | many_to_many

  # ─────────────────────────────────────────────────────────────────────────────
  # BLOC 5: FLAGS & RELATIONSHIPS (optional)
  # ─────────────────────────────────────────────────────────────────────────────
  inverse: INVERSE_ARC_NAME         # Forward arc declares its inverse
  inverse_of: FORWARD_ARC_NAME      # Inverse arc references forward arc
  required: true | false            # Is this relationship mandatory?
  is_self_referential: true         # Can source == target?

  # ─────────────────────────────────────────────────────────────────────────────
  # BLOC 6: PROPERTIES (optional)
  # ─────────────────────────────────────────────────────────────────────────────
  properties:
    - name: property_name
      type: string | int | float | bool | datetime | array
      required: true | false
      enum: [value1, value2]        # Optional
      default: value                # Optional
      min: 0.0                      # Optional: for numeric
      max: 1.0                      # Optional: for numeric
      description: "Property description"

  # ─────────────────────────────────────────────────────────────────────────────
  # BLOC 7: REFERENCE (required)
  # ─────────────────────────────────────────────────────────────────────────────
  cypher_pattern: "(Source)-[:ARC_NAME]->(Target)"

  examples:                         # Optional: query examples
    example_key:
      cypher: |
        MATCH (s:Source)-[:ARC_NAME]->(t:Target)
        RETURN s, t
      description: "Example query description"
```

---

## Property Sub-Field Order

Within `properties` or `standard_properties`, each property follows this order:

```yaml
property_name:
  type:         # 1. Data type (required)
  required:     # 2. Is required? (required)
  enum:         # 3. Allowed values (optional)
  default:      # 4. Default value (optional)
  min:          # 5. Minimum (optional, numeric only)
  max:          # 6. Maximum (optional, numeric only)
  pattern:      # 7. Regex pattern (optional, string only)
  description:  # 8. Description (required)
  example:      # 9. Example value (optional)
  examples:     # 10. Multiple examples (optional)
```

---

## llm_context Format

All `llm_context` fields MUST follow this format:

```
USE: when [primary use case for this node/arc].
TRIGGERS: "keyword1", "keyword2", "keyword3".
NOT: for [disambiguation] (use [alternative] instead).
RELATES: [Related1] (relationship type), [Related2] (relationship type).
```

**Example**:
```yaml
llm_context: |
  USE: when loading native content for an entity in a specific locale.
  TRIGGERS: "entity content", "localized entity", "native content", "entity translation".
  NOT: for entity definitions (use Entity), for page content (use PageNative).
  RELATES: Entity (parent), Locale (target locale), HAS_NATIVE (ownership arc).
```

---

## Header Comment Format

All YAML files should have a standardized header:

```yaml
# ═══════════════════════════════════════════════════════════════════════════════
# {Type}: {Name} — {One-line description}
# ═══════════════════════════════════════════════════════════════════════════════
#
# v{version}: {Change description if applicable}
#
# ADR References:
#   - ADR-0XX: {Related decision}
#
# ═══════════════════════════════════════════════════════════════════════════════
```

---

## Validation Rules

1. **Required Fields**: All BLOC 1 (Identity) and BLOC 2 (Semantic) fields are required
2. **Order Enforcement**: CI will validate property ordering matches this spec
3. **llm_context Format**: Must contain USE/TRIGGERS/NOT/RELATES sections
4. **No Empty Fields**: If a field exists, it must have a value

---

## Migration Checklist

When updating existing YAML files:

- [ ] Reorder properties according to BLOC structure
- [ ] Move `llm_context` to BLOC 2 (after description)
- [ ] Add `description` if missing (arc-classes)
- [ ] Standardize header comments
- [ ] Ensure `llm_context` follows USE/TRIGGERS/NOT/RELATES format
- [ ] Run `cargo run -- schema validate`
- [ ] Run `cargo test`

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| v0.13.1 | 2026-02-16 | Initial LLM-First ordering standard |
