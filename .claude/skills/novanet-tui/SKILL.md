---
name: novanet-tui
description: Launch or explain the NovaNet interactive terminal UI. Use when user wants to explore the graph visually in the terminal, asks about TUI keybindings, or wants to navigate Realm/Layer/Class hierarchy interactively.
disable-model-invocation: false
user-invocable: true
---

# NovaNet TUI (Terminal User Interface)

Launch the Galaxy-themed mission control TUI for interactive graph exploration.

## Quick Start

```bash
cd tools/novanet
cargo run -- tui
```

Or from monorepo root:

```bash
cd tools/novanet && cargo run -- tui
```

## Features

The TUI provides an immersive terminal experience for exploring the NovaNet knowledge graph:

```
+=======================================================================================+
|  NOVANET TUI - Galaxy-Themed Mission Control (v0.12.0)                                |
+=======================================================================================+

+----------------------------------------------------------------------------------------+
|  PANELS                                                                                |
+----------------------------------------------------------------------------------------+
|                                                                                        |
|   +----------------------+  +------------------------------------------------------+   |
|   |  TAXONOMY TREE       |  |  DETAIL PANEL                                        |   |
|   |  (Left 40%)          |  |  (Right 60%)                                         |   |
|   |                      |  |                                                      |   |
|   |  Realm > Layer >Class|  |  - YAML definition                                   |   |
|   |  hierarchy           |  |  - Properties with types                             |   |
|   |                      |  |  - Relationships (from/to)                           |   |
|   |  v shared            |  |  - Statistics                                        |   |
|   |    v knowledge       |  |  - Cypher syntax highlighting                        |   |
|   |      - LocaleVoice   |  |                                                      |   |
|   |      - LocaleCulture |  |                                                      |   |
|   |    > config          |  |                                                      |   |
|   |  v org               |  |                                                      |   |
|   |                      |  |                                                      |   |
|   +----------------------+  +------------------------------------------------------+   |
|                                                                                        |
|   +------------------------------------------------------------------------------------+|
|   |  STATUS BAR                                                                       ||
|   |  [Mode: Graph] [View: Taxonomy] [Class: LocaleVoice] [?=Help] [/=Search]          ||
|   +------------------------------------------------------------------------------------+|
|                                                                                        |
+----------------------------------------------------------------------------------------+
```

## Keybindings

### Mode Switching (v0.12.0: 2 modes)

| Key | Action |
|-----|--------|
| `1` | Switch to Graph mode (unified tree: Realm > Layer > Class > Instance) |
| `2` | Switch to Nexus mode (learning hub: Quiz, Glossary, Stats, Help) |
| `t` | Toggle GraphView: Taxonomy <-> Instances (Graph mode only) |
| `N` | Cycle through both modes |
| `Tab` | Cycle focus: Tree -> Info -> Graph -> YAML |

### Tree Navigation

| Key | Action |
|-----|--------|
| `j` / `Down` | Move cursor down |
| `k` / `Up` | Move cursor up |
| `h` / `Left` | Collapse node |
| `l` / `Right` / `Space` / `Enter` | Expand node |
| `d` | Page down in tree |
| `u` | Page up in tree |

### Detail Panel (YAML Scrolling)

| Key | Action |
|-----|--------|
| `[` | Scroll YAML up one line |
| `]` | Scroll YAML down one line |
| `{` | Scroll YAML up one page |
| `}` | Scroll YAML down one page |

### Overlays

| Key | Action |
|-----|--------|
| `?` | Toggle help overlay |
| `Ctrl+P` | Open command palette |
| `/` or `f` | Open fuzzy search |
| `F1` | Open color legend overlay |
| `Esc` | Close current overlay |
| `q` | Quit (or close overlay) |

### Nexus Mode (Mode 2)

| Key | Action |
|-----|--------|
| `1-5` | Switch tabs (Quiz/Glossary/Stats/Help/Tutorial) |
| `gd` | Jump to defined trait |
| `ga` | Jump to authored trait |
| `gi` | Jump to imported trait |
| `gg` | Jump to generated trait |
| `gr` | Jump to retrieved trait |
| `g0` | Reset all cursors to top |
| `n` | Next "Did you know?" tip |

### Command Palette Commands

| Command | Description |
|---------|-------------|
| `Switch to Graph` | Graph mode (unified tree) |
| `Switch to Nexus` | Learning hub (Quiz, Glossary, Stats) |
| `Dashboard` | Show Neo4j statistics |
| `Search Class` | Fuzzy search classes |
| `Toggle Effects` | CRT/glitch effects |
| `Show Help` | Keybindings overlay |
| `Quit` | Exit TUI |

## Visual Features

### Galaxy Theme

- **SuperNovae color palette**: Deep space blues, nebula purples, star golds
- **Realm colors**: Shared (emerald), Org (sky blue)
- **Layer colors**: 10 distinct colors for each functional layer
- **ArcFamily colors**: 5 relationship type colors
- **Trait styles**: solid (defined), dashed (authored), dotted (imported), double (generated), thin-dotted (retrieved)

### Boot Animation (First Launch)

1. Matrix rain effect (falling characters)
2. Logo reveal (Saturn-graph ASCII art)
3. Fade transition to main UI

### Effects Engine

| Effect | Description |
|--------|-------------|
| CRT Scanlines | Retro monitor effect |
| Glitch Transitions | Mode change distortion |
| Nebula Pulse | Subtle background animation |
| Screen Shake | Emphasis effect |
| Typewriter | Text reveal animation |

### Onboarding (First Run)

1. Welcome screen with project overview
2. Guided tour through 5 key areas:
   - Taxonomy tree navigation
   - Detail panel exploration
   - Mode switching
   - Search and command palette
   - Keyboard shortcuts

## Requirements

- **Neo4j running**: `pnpm infra:up` (from monorepo root)
- **Terminal**: 80x24 minimum, 120x40 recommended
- **Colors**: True color (24-bit) support recommended

## Troubleshooting

### TUI won't start

```bash
# Check Neo4j is running
docker ps | grep neo4j

# Check connection
cd tools/novanet
cargo run -- meta  # Should show schema graph

# Then try TUI
cargo run -- tui
```

### Colors look wrong

- Ensure terminal supports true color (iTerm2, Kitty, Alacritty)
- Check `$TERM` is set to `xterm-256color` or similar

### Keyboard not responding

- Run diagnostic: `cargo run --example tui_diag`
- Checks if crossterm events are working

## Usage

User can invoke with:
- `/novanet-tui` - Launch TUI with default settings
- `/novanet-tui help` - Show keybindings quick reference
- `/novanet-tui keys` - Same as help
- `/novanet-tui features` - Show visual features overview

If user asks about keybindings, navigation, or visual features, show the relevant sections above without launching.

If user wants to actually launch the TUI, provide the command:
```bash
cd tools/novanet && cargo run -- tui
```
