# UX Architecture

TUI/Studio navigation and user experience.

## ADRs in this Domain

| ADR | Name | Status | Summary |
|-----|------|--------|---------|
| [008](adr-008-invariant-structure.md) | Invariant Structure, Localized Content | stable | Structure = 1×, Content = 200× |
| [022](adr-022-unified-tree.md) | Unified Tree Architecture | active | 2 modes: Graph + Nexus |

## Quick Reference

```
┌─────────────────────────────────────────────────────────────────┐
│  UNIFIED TREE (ADR-022)                                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  [1] Graph    Unified tree with lazy instance loading           │
│               Realm > Layer > Class > Instance                  │
│               Everything is a clickable node                    │
│                                                                 │
│  [2] Nexus    Hub for Quiz, Audit, Stats, Help                  │
│               Learning & validation tools                       │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│  v11.7 CHANGES                                                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  BEFORE: 5 modes (Meta/Data/Overlay/Query/Atlas)                │
│  AFTER:  2 modes (Graph/Nexus)                                  │
│                                                                 │
│  BEFORE: Realm/Layer = visual groupings                         │
│  AFTER:  Realm/Layer = clickable nodes with detail panels       │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## When to Consult

- **TUI navigation**: Check ADR-022 (unified tree)
- **Structure vs Content**: Check ADR-008 (invariant structure)
- **Mode confusion**: Check ADR-022 (5→2 modes)

## Key Insight

> "If it's a node in Neo4j, it's a node everywhere." — ADR-022 Principle
