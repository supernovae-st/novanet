# Nexus Game Design Document

## Vision

**Nexus** transforms learning NovaNet from passive documentation reading into an **addictive game experience**. Users don't just learn - they **decrypt knowledge**, **complete missions**, and **rank up** through the galaxy of graph concepts.

## Core Pillars

1. **Ludique** - Fun first, education second
2. **Interactif** - Every action has feedback
3. **Progressif** - Clear path from Cadet to Legend
4. **Immersif** - Space/hacker aesthetic throughout

---

## Game Loop

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│   ┌─────────┐     ┌─────────┐     ┌─────────┐              │
│   │ SELECT  │────▶│ EXECUTE │────▶│ REWARD  │              │
│   │ MISSION │     │ MISSION │     │   XP    │              │
│   └─────────┘     └─────────┘     └─────────┘              │
│        │                               │                    │
│        │                               ▼                    │
│        │                        ┌─────────────┐            │
│        │                        │ LEVEL UP?   │            │
│        │                        └──────┬──────┘            │
│        │                               │                    │
│        │         ┌─────────────────────┴──────────┐        │
│        │         ▼                                ▼        │
│        │   ┌───────────┐                  ┌─────────────┐  │
│        │   │ NEW RANK  │                  │ ACHIEVEMENT │  │
│        │   │  UNLOCK   │                  │   UNLOCK    │  │
│        │   └───────────┘                  └─────────────┘  │
│        │         │                                │        │
│        │         └────────────┬───────────────────┘        │
│        │                      ▼                            │
│        │               ┌─────────────┐                     │
│        └───────────────│  DASHBOARD  │◀────────────────────┘
│                        └─────────────┘                      │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## Boot Sequence

When the user enters Nexus mode, they see a dramatic boot sequence:

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                                                                               ║
║      ███╗   ██╗███████╗██╗  ██╗██╗   ██╗███████╗                              ║
║      ████╗  ██║██╔════╝╚██╗██╔╝██║   ██║██╔════╝                              ║
║      ██╔██╗ ██║█████╗   ╚███╔╝ ██║   ██║███████╗                              ║
║      ██║╚██╗██║██╔══╝   ██╔██╗ ██║   ██║╚════██║                              ║
║      ██║ ╚████║███████╗██╔╝ ██╗╚██████╔╝███████║                              ║
║      ╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝                              ║
║                                                                               ║
║                    KNOWLEDGE HUB v1.0                                         ║
║                                                                               ║
║      ESTABLISHING NEURAL LINK...                                              ║
║      [████████████████████░░░░░░░░░░] 67%                                     ║
║                                                                               ║
║      > Loading mission database...                                            ║
║      > Syncing with Neo4j uplink...                                           ║
║      > Decrypting knowledge base...                                           ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

After 2-3 seconds, transitions to:

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                                                                               ║
║      ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓                   ║
║      ▓                                                   ▓                   ║
║      ▓   ✓ NEURAL LINK ESTABLISHED                       ▓                   ║
║      ▓                                                   ▓                   ║
║      ▓   Welcome back, NAVIGATOR                         ▓                   ║
║      ▓                                                   ▓                   ║
║      ▓   ★★☆☆☆  1,847 XP                                ▓                   ║
║      ▓   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━                   ▓                   ║
║      ▓   [████████████████░░░░░░░░░░░░] 74% to Commander ▓                   ║
║      ▓                                                   ▓                   ║
║      ▓   🔥 5 day streak                                 ▓                   ║
║      ▓   🏆 8 achievements unlocked                      ▓                   ║
║      ▓                                                   ▓                   ║
║      ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓                   ║
║                                                                               ║
║                        Press any key to continue...                           ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Dashboard

The main hub for all Nexus activities:

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  NEXUS MISSION CONTROL                    ★★☆☆☆ NAVIGATOR    1,847 / 2,500 XP║
║  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━║
╠═══════════════════════════════════════════╦═══════════════════════════════════╣
║                                           ║                                   ║
║  ▸ MISSIONS                               ║  📊 STATS                         ║
║  ─────────────────────────────────        ║  ─────────────────────────────    ║
║                                           ║                                   ║
║  ✓ M001: First Contact                    ║  Missions:     2/6 ████░░░░ 33%   ║
║    └─ Decrypt the node vocabulary         ║  Challenges:  12/48 ██░░░░░░ 25%  ║
║                                           ║  Achievements: 8/20 ████░░░░ 40%  ║
║  ✓ M002: Realm Exploration                ║                                   ║
║    └─ Navigate Shared and Org realms      ║  🔥 STREAK: 5 days                ║
║                                           ║  ▁▂▃▄▅▆▇█ ← activity              ║
║  ▸ M003: Arc Mastery              [NEXT]  ║                                   ║
║    └─ Understand arc families             ║  ─────────────────────────────    ║
║    └─ 150 XP + 🏆 Arc Navigator           ║                                   ║
║                                           ║  📜 RECENT ACTIVITY               ║
║  ○ M004: Generation Pipeline              ║                                   ║
║    └─ 🔒 Requires: M001, M003             ║  +150 XP  Mission Complete        ║
║                                           ║  🏆      Layer Architect          ║
║  🔒 M005: Query Mastery                   ║  +50 XP   Streak Bonus (5 days)   ║
║    └─ 🔒 Requires: M002, M003             ║  +100 XP  Challenge Mode (×1.5)   ║
║                                           ║                                   ║
║  🔒 M006: Schema Architect                ║                                   ║
║    └─ 🔒 Requires: M004, M005             ║                                   ║
║                                           ║                                   ║
╠═══════════════════════════════════════════╩═══════════════════════════════════╣
║  [j/k] Navigate   [Enter] Start Mission   [c] Challenges   [a] Achievements  ║
║  [?] Help         [q] Return to Graph                                         ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Mission Briefing

When selecting a mission:

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  MISSION BRIEFING                                                             ║
║  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║   ╔══════════════════════════════════════════════════════════════════════╗   ║
║   ║                                                                      ║   ║
║   ║   MISSION 003: ARC MASTERY                                          ║   ║
║   ║   ─────────────────────────────────────────────────────────────────  ║   ║
║   ║                                                                      ║   ║
║   ║   Arcs are the directed connections between nodes in the graph.     ║   ║
║   ║   Understanding arc families is crucial for querying and            ║   ║
║   ║   navigating the NovaNet knowledge structure.                       ║   ║
║   ║                                                                      ║   ║
║   ║   OBJECTIVES                                                         ║   ║
║   ║   ──────────────────────────────────────────────────────────────    ║   ║
║   ║   [ ] Decrypt the 5 arc families                                    ║   ║
║   ║   [ ] Navigate to an ownership arc in Graph mode                    ║   ║
║   ║   [ ] View a cross-realm arc connection                             ║   ║
║   ║   [ ] Complete the Arc Challenge (3 questions)                      ║   ║
║   ║                                                                      ║   ║
║   ║   REWARDS                                                            ║   ║
║   ║   ──────────────────────────────────────────────────────────────    ║   ║
║   ║   ⚡ 150 XP                                                          ║   ║
║   ║   🏆 Arc Navigator achievement                                       ║   ║
║   ║                                                                      ║   ║
║   ║   ⏱️  Estimated: 12 minutes                                          ║   ║
║   ║                                                                      ║   ║
║   ╚══════════════════════════════════════════════════════════════════════╝   ║
║                                                                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║  [Enter] START MISSION          [Esc] Back to Dashboard                       ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Active Mission

During mission execution:

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  M003: ARC MASTERY                           ⏱️ 04:32    [██████░░░░] 60%     ║
║  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━║
╠═════════════════════════════════════════════╦═════════════════════════════════╣
║                                             ║                                 ║
║  OBJECTIVE 2/4                              ║  📡 INTEL                       ║
║  ─────────────────────────────────────      ║  ─────────────────────────────  ║
║                                             ║                                 ║
║  Navigate to an ownership arc in Graph      ║  Ownership arcs define          ║
║  mode and observe its properties.           ║  hierarchical containment:      ║
║                                             ║                                 ║
║  ┌─────────────────────────────────────┐    ║  Project ──[:HAS_PAGE]──▶ Page  ║
║  │                                     │    ║  Page ────[:HAS_BLOCK]──▶ Block ║
║  │  INSTRUCTIONS                       │    ║  Entity ──[:HAS_NATIVE]▶ EntityNative  ║
║  │  ─────────────────────────────────  │    ║                                 ║
║  │                                     │    ║  The parent "owns" the child.   ║
║  │  1. Press [1] to switch to Graph    │    ║                                 ║
║  │  2. Navigate to Arcs section        │    ║  ─────────────────────────────  ║
║  │  3. Find an arc in "ownership"      │    ║                                 ║
║  │  4. Press [Enter] to inspect        │    ║  💡 HINT (costs 10 XP)          ║
║  │  5. Return here with [3]            │    ║  Press [h] to reveal            ║
║  │                                     │    ║                                 ║
║  └─────────────────────────────────────┘    ║  ─────────────────────────────  ║
║                                             ║                                 ║
║  STATUS: Waiting for Graph navigation...    ║  🔍 LIVE DATA                   ║
║                                             ║  ─────────────────────────────  ║
║                                             ║  43 ownership arcs found        ║
║                                             ║  Example: HAS_PAGE (1:N)        ║
║                                             ║                                 ║
╠═════════════════════════════════════════════╩═════════════════════════════════╣
║  [1] Graph   [3] Nexus   [n] Next   [h] Hint (-10 XP)   [?] Help             ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Challenge Mode

Fast-paced quiz battles:

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  ⚡ CHALLENGE MODE                                    🔥 STREAK: 4  ×1.5 XP   ║
║  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║                           QUESTION 3 / 5                                      ║
║                           ⏱️ 00:12                                            ║
║                                                                               ║
║   ╔══════════════════════════════════════════════════════════════════════╗   ║
║   ║                                                                      ║   ║
║   ║   Which arc family handles locale-specific content relationships?   ║   ║
║   ║                                                                      ║   ║
║   ╚══════════════════════════════════════════════════════════════════════╝   ║
║                                                                               ║
║      ┌─────────────────────────────────────────────────────────────────┐     ║
║      │                                                                 │     ║
║      │   [A]  ownership                                                │     ║
║      │                                                                 │     ║
║      │   [B]  localization                                    ◀────── │     ║
║      │                                                                 │     ║
║      │   [C]  semantic                                                 │     ║
║      │                                                                 │     ║
║      │   [D]  generation                                               │     ║
║      │                                                                 │     ║
║      └─────────────────────────────────────────────────────────────────┘     ║
║                                                                               ║
║                        Score: 300 XP (×1.5 = 450 XP)                         ║
║                                                                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║  [A/B/C/D] Answer    [h] Hint (-25 XP)    [Esc] Abort Challenge               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Correct Answer Flash (Green, 100ms)

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                                                                               ║
║                              ✓ CORRECT!                                       ║
║                                                                               ║
║                           🔥 STREAK: 5  ×2.0 XP                               ║
║                                                                               ║
║                              +100 XP                                          ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### Wrong Answer Flash (Red, 100ms)

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                                                                               ║
║                              ✗ INCORRECT                                      ║
║                                                                               ║
║                      The answer was: localization                             ║
║                                                                               ║
║                           Streak reset to 0                                   ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Achievement Unlock

Full-screen celebration:

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                                                                               ║
║                                                                               ║
║                                                                               ║
║                         ★ ★ ★ ★ ★ ★ ★ ★ ★ ★                                  ║
║                                                                               ║
║                    ╔═══════════════════════════╗                             ║
║                    ║                           ║                             ║
║                    ║   🏆 ACHIEVEMENT UNLOCKED ║                             ║
║                    ║                           ║                             ║
║                    ║      ARC NAVIGATOR        ║                             ║
║                    ║                           ║                             ║
║                    ║   "Master of connections" ║                             ║
║                    ║                           ║                             ║
║                    ║        +100 XP            ║                             ║
║                    ║                           ║                             ║
║                    ╚═══════════════════════════╝                             ║
║                                                                               ║
║                         ★ ★ ★ ★ ★ ★ ★ ★ ★ ★                                  ║
║                                                                               ║
║                                                                               ║
║                                                                               ║
║                       Press any key to continue                               ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Rank Progression

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  RANK PROGRESSION                                                             ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║     ┌─────────┐   ┌─────────┐   ┌─────────┐   ┌─────────┐   ┌─────────┐      ║
║     │ ★☆☆☆☆  │   │ ★★☆☆☆  │   │ ★★★☆☆  │   │ ★★★★☆  │   │ ★★★★★  │      ║
║     │         │   │         │   │         │   │         │   │         │      ║
║     │  CADET  │──▶│NAVIGATOR│──▶│COMMANDER│──▶│ CAPTAIN │──▶│ ADMIRAL │      ║
║     │         │   │         │   │         │   │         │   │         │      ║
║     │  0 XP   │   │ 500 XP  │   │ 1500 XP │   │ 3000 XP │   │ 5000 XP │      ║
║     └─────────┘   └────┬────┘   └─────────┘   └─────────┘   └─────────┘      ║
║                        │                                                      ║
║                        │ YOU ARE HERE                                         ║
║                        ▼                                                      ║
║              ┌──────────────────────────────────────┐                        ║
║              │                                      │                        ║
║              │  ★★☆☆☆  NAVIGATOR                   │                        ║
║              │                                      │                        ║
║              │  XP: 1,847 / 2,500                   │                        ║
║              │  [██████████████░░░░░░░] 74%         │                        ║
║              │                                      │                        ║
║              │  Next rank: COMMANDER                │                        ║
║              │  Remaining: 653 XP                   │                        ║
║              │                                      │                        ║
║              └──────────────────────────────────────┘                        ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Missions Overview

| ID | Name | Objectives | XP | Achievement |
|----|------|------------|-----|-------------|
| M001 | First Contact | 3 | 100 | Explorer |
| M002 | Realm Exploration | 4 | 150 | Realm Walker |
| M003 | Arc Mastery | 4 | 150 | Arc Navigator |
| M004 | Generation Pipeline | 5 | 200 | Pipeline Engineer |
| M005 | Query Mastery | 6 | 250 | Query Ninja |
| M006 | Schema Architect | 5 | 300 | Schema Master |

---

## Achievements

| Category | Achievement | XP | Condition |
|----------|-------------|-----|-----------|
| Learning | First Step | 10 | Complete first objective |
| Learning | Explorer | 100 | Complete M001 |
| Learning | Realm Walker | 100 | Complete M002 |
| Learning | Arc Navigator | 100 | Complete M003 |
| Learning | Pipeline Engineer | 100 | Complete M004 |
| Learning | Query Ninja | 150 | Complete M005 |
| Learning | Schema Master | 150 | Complete M006 |
| Exploration | Deep Diver | 25 | Drill 3 levels in Graph |
| Exploration | Data Explorer | 50 | View 10 instances |
| Exploration | Trait Collector | 50 | View all 5 traits |
| Exploration | Layer Explorer | 50 | View all 11 layers |
| Challenge | Perfect Answer | 25 | 100% on a quiz |
| Challenge | Quiz Master | 75 | Complete 10 challenges |
| Challenge | On Fire | 30 | 5 correct streak |
| Meta | All Clear | 500 | All missions complete |
| Meta | Grand Master | 1000 | All achievements |
| Meta | Speed Runner | 75 | Mission in < 5 min |
| Meta | Dedicated | 100 | 1 hour in Nexus |
| Hidden | Night Owl | 25 | Use Nexus after midnight |
| Hidden | Early Bird | 25 | Use Nexus before 6 AM |

---

## Sound Effects (Optional)

| Event | Sound |
|-------|-------|
| Correct answer | Terminal bell `\a` |
| Achievement | Desktop notification |
| Rank up | Double bell |
| Wrong answer | Low beep |

---

## Persistence

Progress saved to `~/.novanet/nexus-progress.json`:

```json
{
  "version": 1,
  "rank": "navigator",
  "xp": 1847,
  "missions": {
    "M001": { "completed": true, "time": 420 },
    "M002": { "completed": true, "time": 380 },
    "M003": { "in_progress": true, "objectives": [true, false, false, false] }
  },
  "achievements": ["first_step", "explorer", "realm_walker"],
  "challenges": {
    "best_streak": 7,
    "total_correct": 42,
    "total_attempted": 58
  },
  "session": {
    "total_time": 3600,
    "streak_days": 5,
    "last_login": "2025-02-10T14:30:00Z"
  }
}
```

---

## Future Ideas

1. **Multiplayer** - Compare progress with team
2. **Daily Challenges** - New quiz each day
3. **Speedrun Mode** - Timed full mission runs
4. **Custom Missions** - User-created objectives
5. **Leaderboard** - Organization ranking
