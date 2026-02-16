# NovaNet ADR Cheat Sheet (v0.13.0)

Quick reference for daily development. Use `/adr <number>` for full details.

---

## Must-Know ADRs

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  ESSENTIAL ADRs FOR v0.13.0                                                   ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  ADR-029  *Native Pattern      EntityNative, PageNative (unified suffix)      ║
║  ADR-030  Slug Ownership       Page owns URL, Entity owns semantics           ║
║  ADR-024  Trait = Data Origin  defined/authored/imported/generated/retrieved  ║
║  ADR-025  Instruction Layer    PageStructure, PageInstruction naming          ║
║  ADR-021  Query-First          Cypher = source of truth                       ║
║  ADR-022  Unified Tree         2 modes: Graph + Nexus                         ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Quick Lookups

### Traits (ADR-024)

```
┌────────────┬─────────────────────────┬────────────────────────────────────────┐
│ Trait      │ Who Creates             │ Examples                               │
├────────────┼─────────────────────────┼────────────────────────────────────────┤
│ defined    │ Human, ONCE             │ Page, Block, Entity, Locale            │
│ authored   │ Human, PER locale       │ EntityNative, ProjectNative            │
│ imported   │ External data           │ Term, SEOKeyword, GEOQuery             │
│ generated  │ Our LLM                 │ PageNative, BlockNative                │
│ retrieved  │ External APIs           │ GEOAnswer, SEOKeywordMetrics           │
└────────────┴─────────────────────────┴────────────────────────────────────────┘
```

### Architecture (ADR-012)

```
SHARED (40 nodes, READ-ONLY)        ORG (21 nodes)
├── config      (3)                 ├── config      (1)  OrgConfig
├── locale      (6)                 ├── foundation  (6)  Project, Brand...
├── geography   (7)                 ├── structure   (3)  Page, Block...
└── knowledge  (24)                 ├── semantic    (4)  Entity, EntityNative
                                    ├── instruction (4)  PageStructure...
                                    └── output      (3)  PageNative...
```

### *Native Pattern (ADR-029)

```
BEFORE                          AFTER
------                          -----
EntityContent                   EntityNative (authored)
PageGenerated                   PageNative (generated)
HAS_CONTENT + HAS_GENERATED     HAS_NATIVE (unified)
```

### Inverse Arc Tiers (ADR-026)

```
TIER 1 (Required)     HAS_ENTITY↔ENTITY_OF, HAS_PAGE↔PAGE_OF
TIER 2 (Recommended)  HAS_TERMS↔TERMS_OF, USES_ENTITY↔USED_BY
TIER 3 (Optional)     CONTAINS_*, BELONGS_TO_ORG
```

### Visual Encoding (ADR-005 + ADR-013)

```
Fill color   → Layer      (config=gray, semantic=blue)
Border color → Realm      (shared=teal, org=sky)
Border style → Trait      (solid=defined, dashed=authored, dotted=imported)
Icons        → Dual       { web: "lucide", terminal: "◆" }
```

---

## Decision Trees

### "What trait should this node have?"

```
                    ┌─────────────────────────────┐
                    │ How is data created?        │
                    └─────────────────────────────┘
                                │
           ┌────────────────────┼────────────────────┐
           ▼                    ▼                    ▼
    Human creates?        External source?       Our LLM?
           │                    │                    │
    ┌──────┴──────┐      ┌──────┴──────┐            │
    ▼             ▼      ▼             ▼            ▼
  Once?      Per locale? Brought in? Fetched?    generated
    │             │           │           │
    ▼             ▼           ▼           ▼
 defined      authored    imported    retrieved
```

### "Do I need an inverse arc?"

```
Is it ownership (HAS_*)?
├── YES: Is traversal bidirectional frequent?
│   ├── YES → TIER 1: Create *_OF inverse
│   └── NO  → TIER 3: No inverse needed
└── NO: Is it knowledge/locale arc?
    ├── YES → TIER 2: Consider creating inverse
    └── NO  → TIER 3: Probably no inverse needed
```

### "Where does this node go?" (Realm/Layer)

```
Is it universal (same for all orgs)?
├── YES → SHARED realm
│   ├── BCP-47 locale config? → config layer
│   ├── Locale settings?      → locale layer
│   ├── Geographic?           → geography layer
│   └── Knowledge atoms?      → knowledge layer
└── NO → ORG realm
    ├── Org configuration?    → config layer
    ├── Project/Brand?        → foundation layer
    ├── Page/Block structure? → structure layer
    ├── Entity semantics?     → semantic layer
    ├── LLM instructions?     → instruction layer
    └── Generated output?     → output layer
```

---

## Common Mistakes

| Mistake | Correct Approach |
|---------|------------------|
| Using `EntityContent` | Use `EntityNative` (ADR-029) |
| Using `PageGenerated` | Use `PageNative` (ADR-029) |
| Slug on Entity | Slug on Page (ADR-030) |
| Creating inverse for every arc | Check tier first (ADR-026) |
| Using emoji for icons | Use `{ web, terminal }` (ADR-013) |
| Calling it "Edge" | Use "Arc" (ADR-001) |
| Using "Kind" | Use "Class" (ADR-023) |

---

## Commands

```
/adr 029         → Full ADR-029 content
/adr native      → Search "native" in all ADRs
/adr list        → All 32 ADRs by domain
/adr must-know   → Essential 6 ADRs
/adr domain arc  → Arc-design domain ADRs
/novanet-arch    → Architecture diagrams (cites ADRs)
```

---

## When to Consult ADRs

Use this guide to find the right ADR for your situation:

| Symptom / Question | ADR | Key Insight |
|-------------------|-----|-------------|
| "Should this node have locale-specific content?" | **029** | Use `*Native` suffix, trait determines who creates |
| "Where should the URL slug live?" | **030** | Page owns URL (slug), Entity owns semantics (key) |
| "What trait should this node have?" | **024** | Trait = Data Origin (defined/authored/imported/generated/retrieved) |
| "How should I name this instruction node?" | **025** | PageStructure (JSON), PageInstruction (markdown with @ refs) |
| "Do I need an inverse arc?" | **026** | Check tier: TIER 1 required, TIER 2 recommended, TIER 3 optional |
| "How should the TUI display this?" | **022** | Unified tree: everything is a node, 2 modes (Graph/Nexus) |
| "What color/border for this node?" | **005, 013** | Fill=Layer, Border=Realm, Style=Trait, Icons=dual format |
| "How do I define a new arc?" | **027** | llm_context: USE/TRIGGERS/NOT/RELATES pattern |
| "What's the Page-Entity relationship?" | **028** | 1:1 mandatory via [:REPRESENTS], @ reference system |

---

## LLM Context Pattern (ADR-027)

When documenting arcs or nodes, use this structure:

```
llm_context: |
  USE: when [primary use case for this element].
  TRIGGERS: "keyword1", "keyword2", "keyword3".
  NOT: for [what NOT to use this for] (use [alternative] instead).
  RELATES: [Source] (role), [Target] (role), [Related] (relationship).
```

Example for HAS_NATIVE arc:
```
USE: when loading locale-specific content for a defined node.
TRIGGERS: content, native, locale, localized, l10n.
NOT: for structure (use HAS_BLOCK), for definitions (read the invariant).
RELATES: Entity (parent), EntityNative (locale content), FOR_LOCALE (locale link).
```

---

## Version History

| Version | Key Changes |
|---------|-------------|
| v0.13.0 | *Native pattern, Slug ownership |
| v0.12.5 | SEO pillar/cluster, URL slugification |
| v0.12.3 | Page-Entity 1:1, Brand architecture |
| v0.12.0 | Kind→Class, Trait=Data Origin |
| v11.7   | Unified tree (5→2 modes) |

---

## Related Resources

| Resource | Use For |
|----------|---------|
| `/adr <number>` | Full ADR content |
| `/novanet-arch` | Architecture diagrams |
| `/schema:add-node` | Add new node type |
| `.claude/rules/adr/` | All 32 ADRs by domain |
| `novanet-decisions.md` | Consolidated ADR reference |
| `novanet-terminology.md` | Canonical terminology |
