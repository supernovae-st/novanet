# /ontology-audit - Ontology Synchronization Audit

Audits the NovaNet ontology for synchronization across all sources.

## Trigger

`/ontology-audit`

## Purpose

The **YAML models** (`packages/core/models/`) are the **single source of truth**. Everything else is derived:

```
YAML (source)
    │
    ├──> TypeScript types (src/types/)
    ├──> Zod schemas (src/schemas/)
    ├──> Mermaid diagrams (models/docs/views/*.md)
    ├──> Neo4j seeds (neo4j/seeds/)
    └──> Studio config (apps/studio/src/config/)
```

This command verifies **100% synchronization** between source and derived artifacts.

## Methodology: Use Case Tracing

For each use case, trace the **complete path** through the graph and verify:
1. All nodes in the path exist in YAML
2. All relations in the path exist in `relations.yaml`
3. Properties match between source and derived
4. Cardinality constraints are respected

## Actions

### 1. Load Source of Truth

```
Read and parse:
├── models/_index.yaml (35 nodes, 3 scopes)
├── models/relations.yaml (50 relations)
└── models/nodes/**/*.yaml (all node definitions)
```

### 2. Audit Mermaid Diagrams

For each `models/docs/views/*.md`:

| Check | Description |
|-------|-------------|
| **Nodes** | All 35 nodes present with correct scope |
| **Relations FROM→TO** | Direction matches `relations.yaml` |
| **Relations Props** | Properties (position, temperature) documented |
| **Styling** | `classDef` matches `locale_behavior` |

### 3. Audit TypeScript Types

For `src/types/`:

| Check | Description |
|-------|-------------|
| **NODE_TYPES** | Array has exactly 35 entries |
| **NODE_SCOPES** | Mapping matches `_index.yaml` scopes |
| **NODE_BEHAVIORS** | Matches `nodes_by_locale_behavior` |
| **Interfaces** | Properties match YAML definitions |

### 4. Audit Neo4j Seeds

For `neo4j/seeds/`:

| Check | Description |
|-------|-------------|
| **Constraints** | All node labels have unique key constraint |
| **Indexes** | Indexes match YAML `neo4j.indexes` |
| **Sample Data** | Properties match YAML standard_properties |

### 5. Audit Studio Config

For `apps/studio/src/config/`:

| Check | Description |
|-------|-------------|
| **nodeTypes.ts** | All 35 node types with colors/icons |
| **relationshipColors.ts** | All relations with semantic styling |
| **presets.ts** | Presets reference valid node types |

## Output Format

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  ONTOLOGY AUDIT v8.2.0                                                        ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║  Source of Truth: packages/core/models/                                       ║
║  Nodes: 35 | Relations: 50 | Realms: 3                                        ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  ✅ TypeScript Types        35/35 nodes, 50/50 relations                      ║
║  ❌ Mermaid Diagrams        26/35 nodes, 11 relation errors                   ║
║  ✅ Neo4j Seeds             35/35 constraints                                 ║
║  ⚠️  Studio Config           33/35 nodes (missing 2)                          ║
║                                                                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║  SYNC SCORE: 87%                                                              ║
╚═══════════════════════════════════════════════════════════════════════════════╝

ERRORS:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Mermaid: VIEW-COMPLETE-GRAPH.md                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│ ❌ Line 95: Block → PageL10n (should be Page → PageL10n)                    │
│ ❌ Line 98: Block → PagePrompt (should be Page → PagePrompt)                │
│ ❌ Missing: FOR_LOCALE relation (critical for L10n nodes)                   │
│ ❌ Node table: 26 nodes listed, 9 missing                                   │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Use Cases to Trace

### UC1: Block Generation Context

```
Project
  └─ :HAS_PAGE → Page
       └─ :HAS_BLOCK {position} → Block
            ├─ :OF_TYPE → BlockType
            │     └─ :HAS_RULES → BlockRules
            ├─ :HAS_PROMPT → BlockPrompt
            ├─ :USES_CONCEPT {temperature} → Concept
            │     └─ :HAS_L10N → ConceptL10n
            │           └─ :FOR_LOCALE → Locale
            └─ :HAS_OUTPUT → BlockL10n
                  └─ :FOR_LOCALE → Locale
```

Verify: Project, Page, Block, BlockType, BlockRules, BlockPrompt, Concept, ConceptL10n, BlockL10n, Locale

### UC2: Locale Knowledge Context

```
Locale
  ├─ :HAS_IDENTITY → LocaleIdentity
  ├─ :HAS_VOICE → LocaleVoice
  ├─ :HAS_CULTURE → LocaleCulture
  │     ├─ :HAS_CULTURE_REFERENCES → LocaleCultureReferences
  │     │     ├─ :HAS_REFERENCE → Reference
  │     │     └─ :HAS_METAPHOR → Metaphor
  │     └─ :HAS_CONSTRAINT → Constraint
  ├─ :HAS_MARKET → LocaleMarket
  ├─ :HAS_LEXICON → LocaleLexicon
  │     └─ :HAS_EXPRESSION → Expression
  ├─ :HAS_RULES_ADAPTATION → LocaleRulesAdaptation
  ├─ :HAS_RULES_FORMATTING → LocaleRulesFormatting
  │     └─ :HAS_PATTERN → Pattern
  └─ :HAS_RULES_SLUG → LocaleRulesSlug
```

Verify: All 15 Global scope nodes

### UC3: SEO/GEO Pipeline

```
Concept
  ├─ :TARGETS_SEO → SEOKeywordL10n
  │     ├─ :FOR_LOCALE → Locale
  │     └─ :HAS_METRICS → SEOKeywordMetrics
  └─ :TARGETS_GEO → GEOSeedL10n
        ├─ :FOR_LOCALE → Locale
        └─ :HAS_METRICS → GEOSeedMetrics

ConceptL10n
  ├─ :HAS_SEO_TARGET → SEOKeywordL10n (locale-aligned)
  └─ :HAS_GEO_TARGET → GEOSeedL10n (locale-aligned)

SEOMiningRun :SEO_MINES → SEOKeywordL10n
GEOMiningRun :GEO_MINES → GEOSeedL10n
```

Verify: All 6 Shared scope nodes

### UC4: Page Assembly

```
Page
  ├─ :OF_TYPE → PageType
  ├─ :HAS_OUTPUT → PageL10n
  │     ├─ :FOR_LOCALE → Locale
  │     ├─ :ASSEMBLES {position} → BlockL10n
  │     ├─ :BELONGS_TO_PROJECT_L10N → ProjectL10n
  │     └─ :PREVIOUS_VERSION → PageL10n (history)
  ├─ :LINKS_TO {concept_key, context, seo_weight, anchor_type, nofollow} → Page
  └─ :SUBTOPIC_OF → Page (pillar-cluster)
```

Verify: PageType, PageL10n, ProjectL10n + relation properties

## Fixing Desync Issues

When issues are found:

1. **YAML is always right** - If YAML differs from derived, fix the derived
2. **Regenerate Mermaid** - Use `MermaidGenerator` from source YAML
3. **Update TypeScript** - Run schema sync or manually fix
4. **Update Studio** - Sync `nodeTypes.ts` and `relationshipColors.ts`

## Automated Tests to Add

```typescript
// packages/core/src/__tests__/schema-sync.test.ts

describe('Schema Synchronization', () => {
  it('NODE_TYPES matches _index.yaml', () => {
    const yamlNodes = parseYaml('models/_index.yaml').files;
    expect(NODE_TYPES).toHaveLength(35);
    // verify each node exists
  });

  it('RelationRegistry matches relations.yaml', () => {
    const yamlRelations = parseYaml('models/relations.yaml').relations;
    expect(Object.keys(RelationRegistry)).toEqual(Object.keys(yamlRelations));
  });

  it('Mermaid diagrams match source', () => {
    // Parse Mermaid, extract nodes/relations, compare to YAML
  });
});
```

## Notes

- Run this audit before any release
- Any score < 100% blocks release
- Use TodoWrite to track fixes
- Reference: `packages/core/models/_index.yaml` for canonical structure
