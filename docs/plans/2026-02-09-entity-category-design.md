# EntityCategory Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add EntityCategory nodes to organize 281 Entity nodes by their semantic type (THING, CONTENT_TYPE, etc.)

**Architecture:** 13 EntityCategory nodes in global/config with BELONGS_TO arcs from Entity (tenant/semantic). Remove Entity.type property. Update TUI to display hierarchical tree.

**Tech Stack:** Rust (novanet CLI), YAML schemas, Neo4j Cypher, ratatui TUI

---

## Task 1: Create EntityCategory Node Schema

**Files:**
- Create: `packages/core/models/node-classes/global/config/entity-category.yaml`

**Step 1: Write the YAML schema**

```yaml
# models/node-classes/global/config/entity-category.yaml
# EntityCategory - Semantic classification for Entity nodes (v11.1)

node:
  name: EntityCategory
  realm: global
  layer: config
  trait: invariant
  icon: "📂"
  description: "Semantic category for classifying Entity nodes by their purpose (THING, ACTION, CONCEPT, etc.)"

  standard_properties:
    key:
      type: string
      required: true
      pattern: "^[A-Z][A-Z_]*$"
      examples:
        - "THING"
        - "CONTENT_TYPE"
        - "ACTION"

    display_name:
      type: string
      required: true
      examples:
        - "Thing"
        - "Content Type"
        - "Action"

    description:
      type: string
      required: true
      examples:
        - "Core products and objects (QR Code, Smart Link)"
        - "What data QR encodes (URL, WiFi, vCard)"

    created_at:
      type: datetime
      required: true

    updated_at:
      type: datetime
      required: true

  properties:
    question:
      type: string
      required: true
      enum:
        - "WHAT?"
        - "WHERE?"
        - "WHY?"
        - "WHO?"
        - "HOW?"
        - "ABSTRACT"
        - "EXTERNAL"
      description: "Semantic question this category answers"
      examples:
        - "WHAT?": "THING, CONTENT_TYPE, FEATURE, TOOL"
        - "WHERE?": "MEDIUM"
        - "WHY?": "USE_CASE"
        - "WHO?": "INDUSTRY"
        - "HOW?": "ACTION, GUIDE, COMPARISON"
        - "ABSTRACT": "CONCEPT"
        - "EXTERNAL": "BRAND, INTEGRATION"

    sort_order:
      type: int
      required: false
      default: 0
      description: "Display order in TUI tree (lower = first)"

  incoming_relations:
    BELONGS_TO:
      from: Entity
      cardinality: "1:N"
      scope: cross_realm
      description: "Entities that belong to this category"

  examples:
    thing:
      key: "THING"
      display_name: "Thing"
      description: "Core products and objects (QR Code, Smart Link, Barcode)"
      question: "WHAT?"
      sort_order: 1

    content_type:
      key: "CONTENT_TYPE"
      display_name: "Content Type"
      description: "What data QR encodes (URL, WiFi, vCard, Instagram)"
      question: "WHAT?"
      sort_order: 2

    action:
      key: "ACTION"
      display_name: "Action"
      description: "User verbs (create, scan, track, design)"
      question: "HOW?"
      sort_order: 8
```

**Step 2: Validate schema location**

Run: `ls packages/core/models/node-classes/global/config/`
Expected: Directory exists (may need to create)

**Step 3: Commit**

```bash
git add packages/core/models/node-classes/global/config/entity-category.yaml
git commit -m "feat(schema): add EntityCategory node type

Introduces global/config EntityCategory for semantic classification
of Entity nodes. 13 categories based on existing Entity.type enum.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 2: Create BELONGS_TO Arc Schema

**Files:**
- Create: `packages/core/models/arc-classes/semantic/belongs-to.yaml`

**Step 1: Write the YAML schema**

```yaml
# models/arc-classes/semantic/belongs-to.yaml
# BELONGS_TO - Entity to EntityCategory classification arc (v11.1)

arc:
  name: BELONGS_TO
  family: semantic
  scope: cross_realm
  cardinality: "N:1"
  description: "Links Entity to its semantic category (tenant → global)"

  source:
    - Entity

  target:
    - EntityCategory

  properties: {}

  constraints:
    - "Each Entity must have exactly one BELONGS_TO arc"
    - "Target must be a valid EntityCategory node"

  examples:
    entity_to_category:
      source: "Entity:qr-code"
      target: "EntityCategory:THING"
      description: "QR Code is a THING"
```

**Step 2: Commit**

```bash
git add packages/core/models/arc-classes/semantic/belongs-to.yaml
git commit -m "feat(schema): add BELONGS_TO arc type

Cross-realm arc from Entity (tenant) to EntityCategory (global).
Replaces Entity.type property with queryable graph relationship.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 3: Update Entity Schema (Remove type)

**Files:**
- Modify: `packages/core/models/node-classes/tenant/semantic/entity.yaml`

**Step 1: Remove type property from Entity.yaml**

Remove the entire `type:` block from `properties:` section (lines ~238-275).

Keep the documentation comment block (lines ~193-237) but move it to EntityCategory.yaml as reference.

**Step 2: Add BELONGS_TO to relations**

Add to `relations:` section:

```yaml
    BELONGS_TO:
      to: EntityCategory
      cardinality: "N:1"
      scope: cross_realm
      description: "Semantic category this entity belongs to (replaces type property)"
```

**Step 3: Update examples to remove type field**

Remove `type: "THING"` etc. from all examples.

**Step 4: Commit**

```bash
git add packages/core/models/node-classes/tenant/semantic/entity.yaml
git commit -m "refactor(schema): remove Entity.type, add BELONGS_TO relation

Entity classification now via BELONGS_TO arc to EntityCategory.
Removes redundant type enum property in favor of graph relationship.

BREAKING: Entity.type property removed

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 4: Generate Schema Artifacts

**Step 1: Run schema generate**

```bash
cd tools/novanet
cargo run -- schema generate
```

Expected: All 12 generators succeed, including new EntityCategory type.

**Step 2: Run schema validate**

```bash
cargo run -- schema validate --strict
```

Expected: No errors, no warnings.

**Step 3: Commit generated files**

```bash
git add packages/core/src/
git add packages/db/seed/
git commit -m "chore(generated): regenerate artifacts for EntityCategory

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 5: Create Seed Cypher for 13 Categories

**Files:**
- Create: `packages/db/seed/01.1-entity-categories.cypher`

**Step 1: Write seed Cypher**

```cypher
// ═══════════════════════════════════════════════════════════════════════════════
// EntityCategory seed data (13 categories)
// ═══════════════════════════════════════════════════════════════════════════════

// WHAT? categories
CREATE (c:EntityCategory:NodeKind {
  key: 'THING',
  display_name: 'Thing',
  description: 'Core products and objects (QR Code, Smart Link, Barcode)',
  question: 'WHAT?',
  sort_order: 1,
  created_at: datetime(),
  updated_at: datetime()
});

CREATE (c:EntityCategory:NodeKind {
  key: 'CONTENT_TYPE',
  display_name: 'Content Type',
  description: 'What data QR encodes (URL, WiFi, vCard, Instagram)',
  question: 'WHAT?',
  sort_order: 2,
  created_at: datetime(),
  updated_at: datetime()
});

CREATE (c:EntityCategory:NodeKind {
  key: 'FEATURE',
  display_name: 'Feature',
  description: 'Software capabilities (Analytics, Tracking, Bulk QR)',
  question: 'WHAT?',
  sort_order: 3,
  created_at: datetime(),
  updated_at: datetime()
});

CREATE (c:EntityCategory:NodeKind {
  key: 'TOOL',
  display_name: 'Tool',
  description: 'Generators, scanners, builders',
  question: 'WHAT?',
  sort_order: 4,
  created_at: datetime(),
  updated_at: datetime()
});

// WHERE? category
CREATE (c:EntityCategory:NodeKind {
  key: 'MEDIUM',
  display_name: 'Medium',
  description: 'Surfaces and placements (posters, cards, packaging)',
  question: 'WHERE?',
  sort_order: 5,
  created_at: datetime(),
  updated_at: datetime()
});

// WHY? category
CREATE (c:EntityCategory:NodeKind {
  key: 'USE_CASE',
  display_name: 'Use Case',
  description: 'Application scenarios (marketing, events, file sharing)',
  question: 'WHY?',
  sort_order: 6,
  created_at: datetime(),
  updated_at: datetime()
});

// WHO? category
CREATE (c:EntityCategory:NodeKind {
  key: 'INDUSTRY',
  display_name: 'Industry',
  description: 'Vertical markets (restaurants, retail, healthcare)',
  question: 'WHO?',
  sort_order: 7,
  created_at: datetime(),
  updated_at: datetime()
});

// HOW? categories
CREATE (c:EntityCategory:NodeKind {
  key: 'ACTION',
  display_name: 'Action',
  description: 'User verbs (create, scan, track, design)',
  question: 'HOW?',
  sort_order: 8,
  created_at: datetime(),
  updated_at: datetime()
});

CREATE (c:EntityCategory:NodeKind {
  key: 'GUIDE',
  display_name: 'Guide',
  description: 'How-to instructional content',
  question: 'HOW?',
  sort_order: 9,
  created_at: datetime(),
  updated_at: datetime()
});

CREATE (c:EntityCategory:NodeKind {
  key: 'COMPARISON',
  display_name: 'Comparison',
  description: 'Versus content (static vs dynamic)',
  question: 'HOW?',
  sort_order: 10,
  created_at: datetime(),
  updated_at: datetime()
});

// ABSTRACT category
CREATE (c:EntityCategory:NodeKind {
  key: 'CONCEPT',
  display_name: 'Concept',
  description: 'Educational ideas (dynamic, static, quiet zone)',
  question: 'ABSTRACT',
  sort_order: 11,
  created_at: datetime(),
  updated_at: datetime()
});

// EXTERNAL categories
CREATE (c:EntityCategory:NodeKind {
  key: 'BRAND',
  display_name: 'Brand',
  description: 'Third-party brands (Google, Instagram, PayPal)',
  question: 'EXTERNAL',
  sort_order: 12,
  created_at: datetime(),
  updated_at: datetime()
});

CREATE (c:EntityCategory:NodeKind {
  key: 'INTEGRATION',
  display_name: 'Integration',
  description: 'Third-party integrations (Zapier, HubSpot)',
  question: 'EXTERNAL',
  sort_order: 13,
  created_at: datetime(),
  updated_at: datetime()
});

// Create index for fast lookup
CREATE INDEX entity_category_key IF NOT EXISTS FOR (c:EntityCategory) ON (c.key);
```

**Step 2: Commit**

```bash
git add packages/db/seed/01.1-entity-categories.cypher
git commit -m "feat(seed): add 13 EntityCategory nodes

Creates global EntityCategory nodes for Entity classification.
Ordered by semantic question (WHAT?, WHERE?, WHY?, etc.)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 6: Create Migration Cypher for BELONGS_TO Arcs

**Files:**
- Create: `packages/db/migrations/001-entity-belongs-to.cypher`

**Step 1: Write migration Cypher**

```cypher
// ═══════════════════════════════════════════════════════════════════════════════
// Migration: Create BELONGS_TO arcs from Entity.type property
// ═══════════════════════════════════════════════════════════════════════════════

// Create BELONGS_TO arcs based on existing type property
MATCH (e:Entity)
WHERE e.type IS NOT NULL
MATCH (c:EntityCategory {key: e.type})
MERGE (e)-[:BELONGS_TO]->(c);

// Verify all entities have BELONGS_TO
MATCH (e:Entity)
WHERE NOT (e)-[:BELONGS_TO]->(:EntityCategory)
RETURN count(e) AS orphaned_entities;
// Expected: 0

// Remove type property from Entity nodes
MATCH (e:Entity)
REMOVE e.type;

// Verify migration
MATCH (e:Entity)-[:BELONGS_TO]->(c:EntityCategory)
RETURN c.key AS category, count(e) AS entity_count
ORDER BY c.sort_order;
```

**Step 2: Commit**

```bash
git add packages/db/migrations/001-entity-belongs-to.cypher
git commit -m "feat(migration): Entity.type → BELONGS_TO arcs

Migrates 281 Entity nodes from type property to BELONGS_TO arcs.
Removes type property after arc creation.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 7: Update TUI Data Loading

**Files:**
- Modify: `tools/novanet/src/tui/data.rs`

**Step 1: Add EntityCategory to TaxonomyTree**

Update the Cypher query in `load_instances` or create new query to load categories:

```rust
/// Load EntityCategory nodes for Entity grouping
pub async fn load_entity_categories(db: &Db) -> Result<Vec<EntityCategory>> {
    let query = r#"
        MATCH (c:EntityCategory)
        RETURN c.key AS key, c.display_name AS display_name,
               c.question AS question, c.sort_order AS sort_order
        ORDER BY c.sort_order
    "#;
    // ... execute and parse
}

/// Load Entity instances grouped by category
pub async fn load_entities_by_category(db: &Db, category_key: &str) -> Result<Vec<Instance>> {
    let query = r#"
        MATCH (e:Entity)-[:BELONGS_TO]->(c:EntityCategory {key: $category})
        RETURN e.key AS key, e.display_name AS display_name
        ORDER BY e.display_name
    "#;
    // ... execute and parse
}
```

**Step 2: Update tree structure for Entity Kind**

When Kind is "Entity", load categories as intermediate level before instances.

**Step 3: Run tests**

```bash
cargo test tui
```

**Step 4: Commit**

```bash
git add tools/novanet/src/tui/data.rs
git commit -m "feat(tui): load EntityCategory for hierarchical display

Adds queries to fetch EntityCategory nodes and group Entity
instances by category in the tree view.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 8: Update TUI Tree Rendering

**Files:**
- Modify: `tools/novanet/src/tui/app.rs`
- Modify: `tools/novanet/src/tui/ui.rs`

**Step 1: Add category level to tree state**

Update `TreeNode` or add `CategoryNode` variant for Entity categories.

**Step 2: Update tree rendering**

In Data mode, when rendering Entity Kind:
- Show EntityCategory nodes as collapsible intermediate level
- Entity instances nested under their category

```
Entity (281)
├── THING (45)
│   ├── qr-code
│   ├── smart-link
│   └── ...
├── CONTENT_TYPE (53)
│   ├── qr-code-url
│   └── ...
└── ...
```

**Step 3: Run TUI and verify**

```bash
cargo run -- tui
```

Navigate to Data mode > Entity and verify hierarchy displays correctly.

**Step 4: Commit**

```bash
git add tools/novanet/src/tui/
git commit -m "feat(tui): display Entity hierarchy by category

Entity nodes now grouped by EntityCategory in Data mode tree.
Shows category count and allows drilling into category > instance.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Verification Checklist

After all tasks complete:

- [ ] `cargo test` — All 929+ tests pass
- [ ] `cargo clippy -- -D warnings` — Zero warnings
- [ ] `cargo run -- schema validate --strict` — No errors
- [ ] `cargo run -- tui` — Entity hierarchy displays correctly
- [ ] Neo4j Browser: `MATCH (c:EntityCategory) RETURN c` — 13 nodes
- [ ] Neo4j Browser: `MATCH ()-[r:BELONGS_TO]->() RETURN count(r)` — 281 arcs

---

## Rollback Plan

If issues arise:

```cypher
// Restore type property from BELONGS_TO arcs
MATCH (e:Entity)-[:BELONGS_TO]->(c:EntityCategory)
SET e.type = c.key;

// Remove BELONGS_TO arcs
MATCH ()-[r:BELONGS_TO]->(:EntityCategory)
DELETE r;

// Remove EntityCategory nodes
MATCH (c:EntityCategory) DELETE c;
```
