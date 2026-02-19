# Visual Encoding

Colors, icons, and visual representation for NovaNet.

## ADRs in this Domain

| ADR | Name | Status | Summary |
|-----|------|--------|---------|
| [004](adr-004-no-color-duplication.md) | No Color Duplication | stable | Colors ONLY in taxonomy.yaml |
| [005](adr-005-trait-visual-encoding.md) | Trait-Based Visual Encoding | stable | Border style encodes trait |
| [009](adr-009-terminal-colors.md) | Terminal Color Graceful Degradation | stable | truecolor → 256 → 16 fallback |
| [013](adr-013-icons-source.md) | Icons Source of Truth | stable | visual-encoding.yaml dual format |

## Quick Reference

```
┌─────────────────────────────────────────────────────────────────┐
│  VISUAL CHANNELS                                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Fill color     → Layer      (config=gray, semantic=blue...)   │
│  Border color   → Realm      (shared=teal, org=sky)            │
│  Border style   → Trait      (see below)                       │
│  Arc stroke     → ArcFamily  (ownership=solid, semantic=dash)  │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│  TRAIT BORDER STYLES (ADR-005)                                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  defined     ████████  solid                                    │
│  authored    ▬ ▬ ▬ ▬  dashed                                   │
│  imported    ═══════  double                                    │
│  generated   ........  dotted                                   │
│  retrieved   · · · ·  dotted thin                              │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│  DUAL ICON FORMAT (ADR-013)                                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  { web: "lucide-name", terminal: "◆" }                          │
│  NO emoji allowed in code!                                      │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## When to Consult

- **Adding colors**: Check ADR-004 (single source: taxonomy.yaml)
- **Trait visualization**: Check ADR-005 (border styles)
- **TUI colors**: Check ADR-009 (graceful degradation)
- **Icons**: Check ADR-013 (dual format, no emoji)

## Key Insight

> "Colors in taxonomy.yaml ONLY. Icons use dual format {web, terminal}. No emoji."
