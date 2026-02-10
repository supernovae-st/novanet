# Nexus Implementation Plan

## Phase 0: Mode Consolidation (Pre-requisite)

### 0.1 Merge Meta + Data → Graph
- [ ] Add `GraphView` enum: `Taxonomy` | `Instances`
- [ ] Add `graph_view` field to `App`
- [ ] Add `t` keybinding to toggle view
- [ ] Update tree rendering to use `graph_view`
- [ ] Remove `NavMode::Meta` and `NavMode::Data`
- [ ] Add `NavMode::Graph`

### 0.2 Remove Unused Modes
- [ ] Delete `NavMode::Overlay` (never implemented)
- [ ] Delete `NavMode::Query` (never implemented)
- [ ] Delete `NavMode::Atlas` (moving to Nexus missions)

### 0.3 Rename Guide → Nexus
- [ ] Rename `src/tui/guide/` → `src/tui/nexus/`
- [ ] Rename `GuideState` → `NexusState`
- [ ] Update all imports and references
- [ ] Update `NavMode::Guide` → `NavMode::Nexus`

### 0.4 Update Keybindings
- [ ] `1` → Graph
- [ ] `2` → Audit
- [ ] `3` → Nexus
- [ ] Update `mode_cursors` array size: 7 → 3
- [ ] Update tests

**Estimated effort**: 2-3 hours

---

## Phase 1: Core Game State

### 1.1 New Data Structures

```rust
// src/tui/nexus/mod.rs
pub mod mission;
pub mod achievement;
pub mod progress;
pub mod effects;
```

### 1.2 Mission System
- [ ] Create `MissionId` enum (6 missions)
- [ ] Create `MissionStatus`: Locked, Available, InProgress, Completed, Mastered
- [ ] Create `Objective` struct with types
- [ ] Create mission definitions (YAML or const)

### 1.3 Achievement System
- [ ] Create `Achievement` enum (~20 achievements)
- [ ] Create `AchievementTracker` with XP
- [ ] Implement unlock conditions

### 1.4 Progress Persistence
- [ ] Create `ProgressStore` struct
- [ ] Implement JSON save/load to `~/.novanet/nexus-progress.json`
- [ ] Add auto-save every 60s

**Estimated effort**: 4-5 hours

---

## Phase 2: Visual Effects

### 2.1 Color System
- [ ] Create `nexus_colors` module
- [ ] Define rank colors
- [ ] Define XP gradient
- [ ] Define streak colors

### 2.2 Progress Widgets
- [ ] XP bar with Gauge widget
- [ ] Mission progress with LineGauge
- [ ] Streak sparkline

### 2.3 Flash Effects
- [ ] Create `FlashEffect` struct
- [ ] Implement success flash (green)
- [ ] Implement error flash (red)
- [ ] Implement achievement flash (gold)

### 2.4 Animated Counters
- [ ] Create `AnimatedCounter` struct
- [ ] XP gain animation
- [ ] Score counting

### 2.5 Pulsing Effects
- [ ] Pulsing borders for active elements
- [ ] Blinking cursor

**Estimated effort**: 3-4 hours

---

## Phase 3: Boot Sequence

### 3.1 Boot Animation
- [ ] ASCII logo reveal
- [ ] Loading progress bar
- [ ] "NEURAL LINK ESTABLISHED" message
- [ ] Player stats display

### 3.2 First-Time Experience
- [ ] Detect new player (no progress file)
- [ ] Show welcome sequence
- [ ] Set initial rank (CADET)

**Estimated effort**: 2 hours

---

## Phase 4: Dashboard View

### 4.1 Layout
```
┌─────────────────────────────────────────────────────────────┐
│  NEXUS MISSION CONTROL        ★★☆☆☆ NAVIGATOR  1,847 XP   │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  MISSIONS                      │  STATS                     │
│  ├─ ✓ Traits Constellation     │  Missions: 2/6             │
│  ├─ ✓ Layers Architecture      │  Achievements: 8/20        │
│  ├─ ▸ Arc Connections          │  Streak: 🔥 5 days         │
│  ├─ ○ Generation Pipeline      │                            │
│  ├─ 🔒 Query Mastery           │  RECENT ACTIVITY           │
│  └─ 🔒 Schema Design           │  ├─ +150 XP Mission 2      │
│                                │  ├─ 🏆 Layer Architect     │
│                                │  └─ +50 XP Streak bonus    │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│  [j/k] Navigate  [Enter] Start  [Tab] Challenges  [?] Help  │
└─────────────────────────────────────────────────────────────┘
```

### 4.2 Components
- [ ] Mission list with status icons
- [ ] XP bar in header
- [ ] Rank badge
- [ ] Stats panel
- [ ] Recent activity feed

**Estimated effort**: 3-4 hours

---

## Phase 5: Mission Briefing

### 5.1 Briefing Screen
- [ ] Mission title and description
- [ ] Objectives list with checkboxes
- [ ] Estimated time
- [ ] XP reward preview
- [ ] Prerequisites display

### 5.2 Objective Types
- [ ] Learn (read content, time-based)
- [ ] Explore (navigate to node/arc)
- [ ] Challenge (quiz question)
- [ ] LiveExample (view Neo4j data)

**Estimated effort**: 2-3 hours

---

## Phase 6: Challenge Mode

### 6.1 Quiz System
- [ ] Multiple choice (a/b/c/d keys)
- [ ] Timer display
- [ ] Streak counter
- [ ] Score calculation

### 6.2 Feedback
- [ ] Correct: green flash + ding
- [ ] Wrong: red flash + explanation
- [ ] Streak break notification

### 6.3 Scoring
- [ ] Base score × streak multiplier
- [ ] Speed bonus
- [ ] No-hints bonus

**Estimated effort**: 3-4 hours

---

## Phase 7: Achievement Unlocks

### 7.1 Unlock Animation
- [ ] Full-screen takeover
- [ ] ASCII badge art
- [ ] XP counter animation
- [ ] Dismissal on any key

### 7.2 Achievement Categories
- [ ] Learning milestones
- [ ] Exploration achievements
- [ ] Challenge achievements
- [ ] Meta achievements

**Estimated effort**: 2 hours

---

## Phase 8: Live Neo4j Integration

### 8.1 Live Examples
- [ ] Trait instances query
- [ ] Arc examples query
- [ ] Pipeline flow query

### 8.2 Real Data in Missions
- [ ] Show actual node counts
- [ ] Show real instance examples
- [ ] "This is YOUR data" personalization

**Estimated effort**: 2-3 hours

---

## Phase 9: Polish

### 9.1 Sound (Optional)
- [ ] Terminal bell on achievement
- [ ] Desktop notification support

### 9.2 Accessibility
- [ ] Color-blind friendly palette
- [ ] Screen reader hints

### 9.3 Testing
- [ ] Unit tests for game state
- [ ] Snapshot tests for renders
- [ ] Integration tests for progress

**Estimated effort**: 2-3 hours

---

## Total Estimated Effort

| Phase | Hours |
|-------|-------|
| Phase 0: Mode Consolidation | 2-3 |
| Phase 1: Core Game State | 4-5 |
| Phase 2: Visual Effects | 3-4 |
| Phase 3: Boot Sequence | 2 |
| Phase 4: Dashboard | 3-4 |
| Phase 5: Mission Briefing | 2-3 |
| Phase 6: Challenge Mode | 3-4 |
| Phase 7: Achievement Unlocks | 2 |
| Phase 8: Neo4j Integration | 2-3 |
| Phase 9: Polish | 2-3 |
| **TOTAL** | **25-34 hours** |

---

## File Structure After Implementation

```
src/tui/
├── mod.rs
├── app.rs              # Updated: NavMode::Graph, NavMode::Audit, NavMode::Nexus
├── theme.rs            # Updated: nexus_colors
├── effects.rs          # NEW: FlashEffect, AnimatedCounter, pulsing
├── nexus/              # RENAMED from guide/
│   ├── mod.rs          # NexusState, NexusView, boot sequence
│   ├── mission.rs      # MissionId, MissionStatus, Objective
│   ├── achievement.rs  # Achievement enum, AchievementTracker
│   ├── progress.rs     # ProgressStore, save/load
│   ├── dashboard.rs    # Dashboard rendering
│   ├── briefing.rs     # Mission briefing view
│   ├── challenge.rs    # Quiz system
│   ├── live_data.rs    # Neo4j integration
│   └── missions/       # Mission definitions
│       ├── mod.rs
│       ├── traits.rs
│       ├── layers.rs
│       ├── arcs.rs
│       ├── pipeline.rs
│       └── queries.rs
└── ui/
    └── mod.rs          # Updated rendering dispatch
```

---

## Success Criteria

1. **Engagement**: Users want to return to complete missions
2. **Clarity**: First-time users understand the graph in < 10 minutes
3. **Fun**: Achievement unlocks feel rewarding
4. **Performance**: No frame drops during animations
5. **Persistence**: Progress survives app restarts
