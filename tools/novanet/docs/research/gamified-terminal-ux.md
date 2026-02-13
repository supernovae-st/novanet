# Gamified Terminal/CLI Experience Research

Research compiled for Nexus TUI gamification features.

---

## 1. Terminal-Based Games with Good UX

### Roguelikes (ASCII/Unicode Masters)

| Game | Stars | Key Engagement Patterns |
|------|-------|------------------------|
| **NetHack** | 3,476 | Discovery through exploration, persistent consequences, emergent stories |
| **Cataclysm: DDA** | 12,034 | Survival tension, crafting depth, world simulation |
| **Brogue CE** | 1,287 | Visual clarity, color-coded danger, procedural narrative |

**What They Do Right:**

1. **Information Density**
   - Every character conveys meaning (`@` = player, `D` = dragon)
   - Color encodes danger level (red = hostile, blue = magical)
   - ASCII becomes a visual language users learn

2. **Feedback Loops**
   - Immediate: screen flash on hit, color change on damage
   - Short-term: leveling up, finding items
   - Long-term: high scores, persistent progress

3. **Discovery Moments**
   - "You found a hidden door"
   - "The scroll reads: ELBERETH"
   - Secrets reward exploration

### Tetris TUI (samtay/tetris - 982 stars)

```
cargo install samtay/tetris
```

**Engagement Patterns:**
- Level progression (difficulty ramp)
- High score persistence
- Clean Unicode rendering with fallback

### Terminal Snake Variants

| Tool | Stars | Notable Feature |
|------|-------|-----------------|
| taniarascia/snek | 275 | JavaScript, clean implementation |
| sssnake | 224 | Self-playing mode (mesmerizing to watch) |
| wick3dr0se/snake | 113 | Pure Bash v5.1+ |

---

## 2. CLI Tools with Achievement Systems

### Git-Gamify (22 stars, growing)

**URL:** https://github.com/DeerYang/git-gamify

**Core Mechanics:**
```python
# XP System
- Commit: +10 XP base
- Push: +20 XP base
- Streak bonus: +5 XP per consecutive day
- Volume bonus: +1 XP per 10 lines changed

# Leveling
Level = floor(sqrt(XP / 100))
```

**Achievement Examples:**
```
[First Blood]     - First commit
[Marathoner]      - 7-day streak
[Night Owl]       - Commit after midnight
[Early Bird]      - Commit before 6 AM
[Centurion]       - 100 commits
[Thousand Warrior]- 1000 commits
```

**Adaptable to Nexus:**
- Schema commits could earn XP
- Arc creation milestones
- Query optimization achievements
- Locale coverage badges

### Habitica CLI (Concept Reference)

Habitica gamifies productivity with:
- HP (health points) - lose by missing tasks
- XP and leveling
- Gold for equipment
- Pets and mounts as long-term rewards

**Terminal adaptations exist but are under-developed.**

---

## 3. Educational Terminal Games

### Vim Learning Games

| Tool | Stars | Approach |
|------|-------|----------|
| VimGore | 29 | Interactive challenges |
| VSCode Vim Academy | 78 | Gamified motions |
| VimVentures | 3 | Spaceship metaphor |

**Pattern:** Learn through play, not through docs.

### Typing Tests

| Tool | Stars | Key Feature |
|------|-------|------------|
| MonkeyType | 19,374 | Minimalist, metrics-focused |
| Typeman | 175 | TUI mode available |
| kboard | 192 | Terminal-native practice |

**Engagement Patterns:**
- WPM as score
- Accuracy percentages
- Personal best tracking
- Streaks and consistency

### SQL Murder Mystery (Concept)

Game-based learning for SQL queries. Pattern: wrap learning in narrative.

**Adaptable:** Could create "Graph Detective" mode for Nexus - solve mysteries using Cypher queries.

---

## 4. Hacking-Themed Terminal Games

### Direct Terminal Games

| Tool | Stars | Type |
|------|-------|------|
| Aurora-OS.js | 427 | Web OS simulator |
| malware-slayer | 105 | Hack-and-slash platformer |

### Inspirations (Non-Terminal but Influenceable)

**Hacknet (Steam game):**
- Realistic terminal aesthetics
- `scan`, `probe`, `porthack` commands
- Green-on-black, amber-on-black modes
- Sound effects on successful hacks
- Progress tree for skills

**Uplink (Steam game):**
- Mission-based progression
- Equipment upgrades
- Tension through time limits
- "Trace" meter creates urgency

**Terminal Aesthetics to Adapt:**
```
[SYSTEM] Establishing connection...
[OK] Port 22 open
[OK] Authentication successful
[NEXUS] Welcome, Operator.

    _   __________  ____  _______
   / | / / ____/ / / / / / / ___/
  /  |/ / __/ | |/_/ / / /\__ \
 / /|  / /____>  </ /_/ /___/ /
/_/ |_/_____/_/|_|\____//____/

> Mission: Map the knowledge graph
> Objective: Discover 3 hidden EntityCategories
```

---

## 5. ASCII Art Animations for Celebrations

### Confetty (490 stars)

**URL:** https://github.com/maaslalani/confetty

```bash
# Install
go install github.com/maaslalani/confetty@latest

# Usage
confetty           # Confetti
confetty fireworks # Fireworks

# SSH preview
ssh -p 2222 ssh.caarlos0.dev  # Confetti
ssh -p 2223 ssh.caarlos0.dev  # Fireworks
```

**Implementation:** Go + Bubble Tea TUI framework. Particle system.

### Firework-rs (180 stars)

**URL:** https://github.com/Wayoung7/firework-rs

**Rust crate for terminal fireworks:**
```rust
// Features
- Colorful ASCII fireworks
- Customizable particles
- Gradient support
- Cross-platform
```

**Can be integrated directly into Nexus as Rust code.**

### Alive-Progress (6,241 stars)

**URL:** https://github.com/rsalmei/alive-progress

**Python's most engaging progress bar:**
- Live spinners that react to processing speed
- ETA with exponential smoothing
- Final receipt with stats
- Suspend/resume capability

**Key Insight:** Progress bars can have personality.

### CMatrix (4,921 stars)

**URL:** https://github.com/abishekvashok/cmatrix

Matrix rain effect. Creates atmosphere.

**Variants:**
- UniMatrix (1,891 stars) - Unicode katakana
- Neo (864 stars) - 32-bit color
- rusty-rain (433 stars) - Rust implementation

### ASCIIquarium (1,124 stars)

Relaxing aquarium animation. Demonstrates sustained ambient animations.

---

## 6. Engagement Mechanics Summary

### Visual Feedback Patterns

| Pattern | Example | Effect |
|---------|---------|--------|
| **Flash** | Screen blink on error | Immediate attention |
| **Color pulse** | Green flash on success | Positive reinforcement |
| **Particle burst** | Confetti on achievement | Celebration |
| **Progress animation** | Spinner variants | Activity indication |
| **ASCII art reveal** | Logo animation | Branding moment |

### Progress Systems

| System | Mechanic | Psychological Hook |
|--------|----------|-------------------|
| **XP/Leveling** | Accumulate points | Continuous progress |
| **Streaks** | Consecutive days | Loss aversion |
| **Achievements** | Milestone unlocks | Collection drive |
| **High scores** | Personal/global best | Competition |
| **Ranks/Titles** | Named progression | Identity |

### Reward Loops

```
Immediate (0-1s):
- Sound (bell, tone)
- Visual flash
- Color change
- Counter increment

Short-term (1s-1hr):
- Achievement unlock
- Level up
- Streak continuation
- Stats update

Long-term (1hr+):
- Rank advancement
- Profile evolution
- Unlockable features
- Leaderboard position
```

### Sound in Terminal

Limited but impactful:
- `\a` (BEL) - terminal bell
- System notifications via `notify-send` (Linux) or `osascript` (macOS)
- arttime demonstrates native notifications with sound

---

## 7. Concrete Adaptations for Nexus

### Achievement Ideas

```rust
// Schema Achievements
"Architect"        - Create first NodeKind
"Arc Weaver"       - Create 10 ArcKinds
"Layer Cake"       - Use all 11 layers
"Realm Walker"     - Work in both realms
"Completionist"    - 100% schema coverage

// Query Achievements
"First Query"      - Execute first Cypher
"Speed Demon"      - Query under 100ms
"Deep Diver"       - 5+ hop traversal
"Filter Master"    - Use all facet filters

// Exploration Achievements
"Atlas Complete"   - View all 59 nodes
"Arc Explorer"     - Traverse all arc families
"Locale Hunter"    - Query 10+ locales

// Easter Eggs
"Night Shift"      - Use TUI after midnight
"Marathon"         - 4-hour session
"Secret Command"   - Discover hidden feature
```

### Visual Effects to Implement

```rust
// On Achievement Unlock
mod effects {
    pub fn confetti(area: Rect) {
        // Particle system with falling characters
        // Characters: *, ., ', `, ~, ^
        // Colors: cycle through realm/layer palette
    }

    pub fn flash(color: Color) {
        // Briefly invert or tint entire screen
    }

    pub fn pulse(widget: Rect) {
        // Animate border brightness
    }

    pub fn typing_reveal(text: &str) {
        // Character-by-character reveal
        // Like "hacking" terminal effect
    }
}
```

### Progress Bar Personality

```rust
// Instead of: [=====>    ] 50%
// Show:       [::NEXUS::>    ] MAPPING GRAPH 50%

// Spinner variants for different operations
static SPINNERS: &[&[&str]] = &[
    &[".", "..", "...", "...."],           // Simple
    &["[=   ]", "[==  ]", "[=== ]", "[====]"], // Loading
    &["( )", "(o)", "(O)", "(o)"],         // Pulsing
    &["</>", "<//>", "<///>", "</////>"],  // Code-y
];
```

### Mission/Quest System

```rust
struct Mission {
    id: &'static str,
    title: &'static str,
    objective: &'static str,
    xp_reward: u32,
    badge: Option<&'static str>,
}

// Example missions
static MISSIONS: &[Mission] = &[
    Mission {
        id: "first_query",
        title: "First Contact",
        objective: "Execute your first query",
        xp_reward: 10,
        badge: Some("[OPERATOR]"),
    },
    Mission {
        id: "complete_schema",
        title: "Knowledge Architect",
        objective: "Define all 61 NodeKinds",
        xp_reward: 500,
        badge: Some("[ARCHITECT]"),
    },
];
```

---

## 8. Implementation Priority

### Phase 1: Foundation
1. **Stats tracking** - Session time, queries, nodes viewed
2. **Simple achievements** - First actions, milestones
3. **Visual feedback** - Flash on success/error

### Phase 2: Engagement
4. **XP system** - Points for actions
5. **Level progression** - Titles/ranks
6. **Streak tracking** - Consecutive day usage

### Phase 3: Delight
7. **Celebration effects** - Confetti, fireworks
8. **Easter eggs** - Hidden commands, secrets
9. **Mission system** - Guided exploration

### Phase 4: Social
10. **Profile export** - Share achievements
11. **Leaderboards** - Opt-in comparison
12. **Challenges** - Time-limited events

---

## 9. Key Libraries for Rust Implementation

| Crate | Purpose | Notes |
|-------|---------|-------|
| **ratatui** | TUI framework | Already using |
| **indicatif** | Progress bars | Customizable |
| **console** | Terminal utils | Colors, emoji |
| **notify-rust** | Desktop notifications | Achievement alerts |
| **rand** | Randomness | Particle effects |
| **chrono** | Time tracking | Streaks, sessions |
| **serde** | Serialization | Stats persistence |

---

## 10. Sources

### Primary Repositories
- https://github.com/NetHack/NetHack (3,476 stars)
- https://github.com/CleverRaven/Cataclysm-DDA (12,034 stars)
- https://github.com/tmewett/BrogueCE (1,287 stars)
- https://github.com/maaslalani/confetty (490 stars)
- https://github.com/Wayoung7/firework-rs (180 stars)
- https://github.com/rsalmei/alive-progress (6,241 stars)
- https://github.com/DeerYang/git-gamify (22 stars)
- https://github.com/samtay/tetris (982 stars)
- https://github.com/abishekvashok/cmatrix (4,921 stars)
- https://github.com/cmatsuoka/asciiquarium (1,124 stars)
- https://github.com/monkeytypegame/monkeytype (19,374 stars)
- https://github.com/poetaman/arttime (1,262 stars)

### Key Insights

1. **Information density** - Every pixel matters in terminal UX
2. **Sound is underused** - Terminal bell + notifications are impactful
3. **Streaks drive engagement** - Loss aversion is powerful
4. **Discovery creates stories** - Users remember finding hidden features
5. **Particle effects work** - Confetti/fireworks are universally delightful
6. **Progress visibility** - Users need to see advancement

---

## Methodology

- Tools used: GitHub API search, repository README analysis
- Repositories analyzed: 50+
- Search queries: 25+
- Categories covered: Games, productivity, education, effects

## Confidence Level

**High** - Based on popular open-source projects with proven engagement patterns. Implementation patterns are concrete and adaptable.
