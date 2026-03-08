# Dynamic Seeds Architecture (C-Full)

**Date**: 2026-03-08
**Status**: Draft
**Author**: Claude + Thibaut

## Problem Statement

Currently, NovaNet data persistence has a critical flaw:

1. **Seed files** (`packages/db/seed/*.cypher`) contain static data
2. **Nika pipelines** generate content directly into Neo4j
3. **Reset** (`pnpm infra:reset`) wipes Neo4j and only restores seed files
4. **Result**: All Nika-generated content is LOST on reset

## Solution: YAML as Source of Truth

```
brain/data/*.yaml  →  novanet seed generate  →  seed/*.cypher  →  Neo4j
     ↑                                                              │
     └──────────────  novanet seed import  ◄────────────────────────┘
```

## Architecture

### Directory Structure

```
brain/
├── models/                    # Schema (existing)
│   ├── node-classes/
│   ├── arc-classes/
│   └── taxonomy.yaml
│
└── data/                      # NEW: Business data
    ├── _index.yaml            # Data manifest
    │
    ├── shared/                # Realm: shared (universal)
    │   ├── locales.yaml       # 200 locales (existing seed)
    │   ├── expressions/       # Knowledge atoms by locale
    │   │   ├── _index.yaml    # Expression categories
    │   │   ├── en-US.yaml
    │   │   ├── fr-FR.yaml
    │   │   └── ...
    │   └── geography/
    │       └── regions.yaml
    │
    └── org/                   # Realm: org (project-specific)
        ├── project.yaml       # Project + ProjectNative
        ├── entities.yaml      # Entity + EntityNative
        ├── pages.yaml         # Page + PageNative
        ├── blocks.yaml        # Block + BlockNative
        └── seo/
            ├── keywords.yaml  # SEOKeyword
            └── sets.yaml      # SEOKeywordSet
```

### YAML Schema

#### Entity Definition

```yaml
# brain/data/org/entities.yaml
version: "1.0"
class: Entity
natives_class: EntityNative

entities:
  - key: qr-code
    display_name: QR Code
    description: Quick Response code - 2D barcode technology
    created_at: 2026-01-15T00:00:00Z

    natives:
      en-US:
        display_name: QR Code
        description: A QR code is a two-dimensional barcode...
        denomination_forms:
          - type: text
            value: qr code
            priority: 1
          - type: title
            value: QR Code
            priority: 1
          - type: abbrev
            value: QR
            priority: 1
          - type: url
            value: qr-code
            priority: 1

      fr-FR:
        display_name: Code QR
        description: Un code QR est un code-barres bidimensionnel...
        denomination_forms:
          - type: text
            value: code QR
            priority: 1
          - type: title
            value: Code QR
            priority: 1
          - type: abbrev
            value: QR
            priority: 1
          - type: url
            value: code-qr
            priority: 1
```

#### Page Definition

```yaml
# brain/data/org/pages.yaml
version: "1.0"
class: Page
natives_class: PageNative

pages:
  - key: page:qr-code-landing
    display_name: QR Code Landing Page
    description: Main landing page for QR Code AI

    # Relationships (auto-create arcs)
    represents: qr-code          # → REPRESENTS arc to Entity
    blocks:                      # → HAS_BLOCK arcs
      - block:qr-code-hero
      - block:qr-code-what-is
      - block:qr-code-use-cases
      - block:qr-code-cta

    natives:
      en-US:
        display_name: QR Code Generator
        slug: create-qr-code
        meta_title: Free QR Code Generator | Create QR Codes Online
        meta_description: Create custom QR codes for free...

      fr-FR:
        display_name: Générateur de Code QR
        slug: creer-code-qr
        meta_title: Générateur de Code QR Gratuit
        meta_description: Créez des codes QR personnalisés...
```

#### Expression Definition

```yaml
# brain/data/shared/expressions/fr-FR.yaml
version: "1.0"
locale: fr-FR
class: Expression
container_class: ExpressionSet

categories:
  - key: SUCCESS
    display_name: Success expressions
    expressions:
      - text: "Parfait !"
        register: casual
      - text: "Excellent travail"
        register: formal
      - text: "C'est gagné"
        register: casual

  - key: SPEED
    display_name: Speed expressions
    expressions:
      - text: "En un clin d'œil"
        register: casual
      - text: "Instantanément"
        register: formal
```

## CLI Commands

### `novanet seed generate`

Generates Cypher from YAML:

```bash
novanet seed generate [--class=Entity] [--dry-run]

# Reads: brain/data/**/*.yaml
# Writes: packages/db/seed/generated/*.cypher
```

**Generated structure:**
```
packages/db/seed/generated/
├── 001-schema.cypher          # Schema nodes (from models/)
├── 010-locales.cypher         # Locale nodes
├── 020-expressions.cypher     # Expression + ExpressionSet
├── 030-entities.cypher        # Entity + EntityNative
├── 040-pages.cypher           # Page + PageNative
├── 050-blocks.cypher          # Block + BlockNative
└── 060-seo.cypher             # SEO nodes
```

### `novanet seed validate`

Validates YAML against schema:

```bash
novanet seed validate [--fix]

# Checks:
# - Required properties present
# - References exist (Entity before EntityNative)
# - Locale codes valid
# - Key formats correct (ADR-036)
```

### `novanet seed import`

Imports from Neo4j to YAML:

```bash
novanet seed import --class=BlockNative [--locale=fr-FR] [--merge]

# Reads: Neo4j
# Writes: brain/data/org/blocks.yaml (merges or overwrites)
```

### `novanet seed diff`

Shows differences:

```bash
novanet seed diff [--class=Entity]

# Output:
# + block:qr-code-hero@de-DE (in Neo4j, not in YAML)
# - block:qr-code-old@en-US (in YAML, not in Neo4j)
# ~ entity:qr-code (modified: display_name changed)
```

## Implementation Plan

### Phase 1: Structure (1h)

- [ ] Create `brain/data/` directory structure
- [ ] Create `_index.yaml` manifest schema
- [ ] Add to `.gitignore` patterns if needed

### Phase 2: YAML Parser (3h)

- [ ] Define Rust structs for YAML schemas
- [ ] Implement serde deserialization
- [ ] Add validation logic
- [ ] Unit tests

### Phase 3: Cypher Generator (3h)

- [ ] Implement node generation (MERGE with properties)
- [ ] Implement arc generation (auto-create relationships)
- [ ] Handle timestamps (created_at, updated_at)
- [ ] Ensure idempotency (MERGE, not CREATE)

### Phase 4: CLI Commands (3h)

- [ ] `novanet seed generate` command
- [ ] `novanet seed validate` command
- [ ] Integration with existing `seed.sh`

### Phase 5: Import Command (2h)

- [ ] `novanet seed import` command
- [ ] Neo4j to YAML conversion
- [ ] Merge logic (don't overwrite manual edits)

### Phase 6: Migration (2h)

- [ ] Export current Neo4j data to YAML
- [ ] Verify round-trip (YAML → Cypher → Neo4j → YAML)
- [ ] Update documentation

## Auto-Generated Arcs

The generator automatically creates these arcs:

| Parent → Child | Arc Type | Trigger |
|----------------|----------|---------|
| Entity → EntityNative | `HAS_NATIVE` | `natives:` section |
| Page → PageNative | `HAS_NATIVE` | `natives:` section |
| Block → BlockNative | `HAS_NATIVE` | `natives:` section |
| *Native → Locale | `FOR_LOCALE` | locale key in natives |
| Page → Entity | `REPRESENTS` | `represents:` field |
| Page → Block | `HAS_BLOCK` | `blocks:` list |
| * → Schema:Class | `OF_CLASS` | Always |

## Validation Rules

1. **Key format**: Must match ADR-036 patterns
2. **Required properties**: Based on schema YAML
3. **Reference integrity**: Referenced nodes must exist
4. **Locale validity**: Must be valid BCP-47 code
5. **No duplicates**: Keys must be unique per class

## Future Enhancements

1. **Auto-import hook**: Nika post-run hook to auto-import
2. **Diff in CI**: Fail CI if Neo4j differs from YAML
3. **Selective sync**: Only sync specific classes/locales
4. **Conflict resolution**: Interactive merge for conflicts

## References

- ADR-036: Key normalization (@ = localized content)
- ADR-029: *Native pattern
- ADR-033: Denomination forms
