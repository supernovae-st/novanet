---
id: "022"
title: "Unified Tree Architecture"
version: "v11.7"
status: "active"
domain: "ux-architecture"
---

# ADR-022: Unified Tree Architecture

**Status**: Approved (v11.7)

**Problem**: NovaNet had inconsistent behavior between Neo4j and UI:
1. Realm, Layer, Trait ARE nodes in Neo4j (`:Meta:Realm`, `:Meta:Layer`)
2. But TUI/Studio treated them as visual groupings, not clickable nodes
3. 5 separate modes (Meta/Data/Overlay/Query/Atlas) created confusion
4. Emoji icons in code instead of proper icon system

**Decision**: Unify into single tree where everything is a clickable node.

**Changes**:
- 5 modes → 2 modes: `[1]Graph` (unified tree) + `[2]Nexus` (hub)
- Realm, Layer, ArcFamily, ArcKind are clickable nodes with detail panels
- Kind nodes expand to show instances (lazy loading, 10 + "load more")
- Dual icons: `{ web: "lucide-name", terminal: "◆" }` - no emoji
- Atlas removed, Audit moved to Nexus hub

**Principle**: "If it's a node in Neo4j, it's a node everywhere"

**Consequences**:
- Neo4j migration needed (HAS_LAYER, HAS_KIND, BELONGS_TO_FAMILY arcs)
- Types defined before generators
- Backward compatibility shim for old nav modes
- Performance optimization for large instance counts (200K+)

**Files affected**:
- TUI: `tools/novanet/src/tui/{app,data,ui,theme}.rs`
- Studio: `apps/studio/src/components/graph/`, stores
- YAML: `visual-encoding.yaml`, `views/_registry.yaml`

## Header Simplification

```
BEFORE (v11.6): [1]Meta [2]Data [3]Overlay [4]Query [5]Atlas
AFTER (v11.7):  [1]Graph [2]Nexus
```

## Changes Table

| Aspect | Before | After |
|--------|--------|-------|
| Modes | 5 (Meta/Data/Overlay/Query/Atlas) | 2 (Graph/Nexus) |
| Realm/Layer | Visual groupings (folders) | Clickable nodes |
| Instances | Hidden or separate Data mode | Expandable under Kind |
| Search | Separate Query mode | `[/]` overlay in Graph |
| Atlas | Separate mode | Removed |
| Audit | In Atlas | In Nexus hub |
| Icons | Mixed emoji | Unicode only (no emoji) |

## Unified Tree Structure

```
▼ Nodes (60)
  ▼ ◉ Realm:shared           ← Clickable :Meta:Realm node
    ▼ ⚙ Layer:config         ← Clickable :Meta:Layer node
      ▼ ◆ Kind:Locale [200]  ← Clickable :Meta:Kind node
        ● Locale:fr-FR       ← Clickable :Locale instance
        ● Locale:en-US
▼ Arcs (169)
  ▼ → ArcFamily:ownership
    → ArcKind:HAS_PROJECT
```

## Nexus Hub

```
[2]Nexus
├── Quiz    — Test NovaNet knowledge
├── Audit   — Validate schema consistency
├── Stats   — Dashboard with graph metrics
└── Help    — Keybindings and documentation
```

## Color Architecture

Single source of truth for colors:
- `taxonomy.yaml` = DEFINES colors (hex values)
- `visual-encoding.yaml` = USES colors (no hex, references taxonomy)
- TUI + Studio = same colors from taxonomy.yaml

## UI Elements Preserved

From current TUI (keep these):
- Trait icons: `■(inv)` `□(loc)` `◇(kno)` `★(gen)` `⋆(agg)`
- Arc counts: `→N` (outgoing) `←N` (incoming)
- Property counts: `⊞required/total`
- Instance counts: `Kind (N)`, `Layer (N)`
- Colored badges: `●org` `◎shd` `◆sem` etc.
- Layer headers with kind count: `◇3`

**Reference**: `docs/plans/2026-02-11-unified-tree-design.md`
