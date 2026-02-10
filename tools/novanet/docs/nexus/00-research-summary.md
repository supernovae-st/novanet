# Nexus Research Summary

## Overview

Research conducted for transforming the NovaNet TUI Guide into **Nexus** - a gamified, interactive knowledge hub with space/hacking theme.

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Merge Meta + Data | → **Graph** | Single unified view with instances + arcs |
| Remove modes | Overlay, Query, Atlas | Redundant or unused |
| Rename Guide | → **Nexus** | Space theme, knowledge hub concept |
| Final modes | **3 modes**: Graph, Audit, Nexus | Simplified navigation |

## Terminology

| Game Term | Meaning |
|-----------|---------|
| Decrypt | Learn a concept |
| Uplink | Connect to Neo4j live data |
| Intel | Hints and examples |
| Mission Briefing | Objective description |
| Neural Link | Session connection |
| Sector Scan | Explore graph |
| Deploy | Create node/arc |

## Rank System

```
★☆☆☆☆ CADET        (0-500 XP)
★★☆☆☆ NAVIGATOR    (500-1,500 XP)
★★★☆☆ COMMANDER    (1,500-3,000 XP)
★★★★☆ CAPTAIN      (3,000-5,000 XP)
★★★★★ ADMIRAL      (5,000-10,000 XP)
★★★★★ LEGEND       (10,000+ XP)
```

## XP System

- **Mission completion**: 150-1000 XP (varies by difficulty)
- **Challenge correct**: 100 XP × streak multiplier
- **Speedrun bonus**: +50-200 XP
- **No hints bonus**: +25-75 XP
- **Perfect score**: +500 XP

## Streak Multipliers

| Streak | Multiplier |
|--------|------------|
| 3 correct | ×1.5 |
| 5 correct | ×2.0 |
| 7 correct | ×2.5 |
| 10 correct | ×3.0 |

## Sources

1. **Git-Gamify** - XP/achievement system design
2. **firework-rs** - Rust particle effects
3. **Brogue CE** - Color-as-meaning terminal design
4. **alive-progress** - Progress bar personality
5. **Hacknet/Uplink** - Hacking game aesthetics
6. **OverTheWire Bandit** - Progressive terminal learning
7. **ratatui widgets** - Gauge, Sparkline, LineGauge

## Files Created

- `00-research-summary.md` - This file
- `01-game-design.md` - Full game mechanics
- `02-visual-effects.md` - Ratatui widget effects
- `03-implementation-plan.md` - Step-by-step plan
