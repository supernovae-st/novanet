---
name: novanet-tui
description: Launch or explain the NovaNet interactive terminal UI. Use when user wants to explore the graph visually in the terminal, asks about TUI keybindings, or wants to navigate Realm/Layer/Kind hierarchy interactively.
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
╔═══════════════════════════════════════════════════════════════════════════════════════╗
║  NOVANET TUI — Galaxy-Themed Mission Control                                          ║
╚═══════════════════════════════════════════════════════════════════════════════════════╝

┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  PANELS                                                                                  │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                          │
│   ┌──────────────────────┐  ┌──────────────────────────────────────────────────────┐   │
│   │  TAXONOMY TREE       │  │  DETAIL PANEL                                         │   │
│   │  (Left 40%)          │  │  (Right 60%)                                          │   │
│   │                      │  │                                                       │   │
│   │  Realm > Layer > Kind│  │  • YAML definition                                    │   │
│   │  hierarchy           │  │  • Properties with types                              │   │
│   │                      │  │  • Relationships (from/to)                            │   │
│   │  ▼ global            │  │  • Statistics                                         │   │
│   │    ▼ knowledge       │  │  • Cypher syntax highlighting                         │   │
│   │      • LocaleVoice   │  │                                                       │   │
│   │      • LocaleCulture │  │                                                       │   │
│   │    ▶ config          │  │                                                       │   │
│   │  ▶ project           │  │                                                       │   │
│   │  ▶ shared            │  │                                                       │   │
│   └──────────────────────┘  └──────────────────────────────────────────────────────┘   │
│                                                                                          │
│   ┌──────────────────────────────────────────────────────────────────────────────────┐  │
│   │  STATUS BAR                                                                       │  │
│   │  [NavMode: META] [Kind: LocaleVoice] [?=Help] [Ctrl+P=Palette] [/=Search]        │  │
│   └──────────────────────────────────────────────────────────────────────────────────┘  │
│                                                                                          │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Keybindings

### Navigation Mode

| Key | Action |
|-----|--------|
| `1` | Switch to META mode (meta-graph only) |
| `2` | Switch to DATA mode (real instances) |
| `3` | Switch to OVERLAY mode (data + meta) |
| `4` | Switch to QUERY mode (faceted filters) |
| `Tab` | Cycle NavMode forward |

### Tree Navigation

| Key | Action |
|-----|--------|
| `j` / `↓` | Move cursor down |
| `k` / `↑` | Move cursor up |
| `h` / `←` | Collapse node |
| `l` / `→` / `Space` / `Enter` | Expand node |
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
| `/` | Open fuzzy search |
| `Esc` | Close current overlay |
| `q` | Quit (or close overlay) |

### Command Palette Commands

| Command | Description |
|---------|-------------|
| `Switch to Meta` | NavMode: meta-graph |
| `Switch to Data` | NavMode: real instances |
| `Switch to Overlay` | NavMode: combined |
| `Switch to Query` | NavMode: faceted |
| `Dashboard` | Show Neo4j statistics |
| `Search Kind` | Fuzzy search kinds |
| `Toggle Effects` | CRT/glitch effects |
| `Show Help` | Keybindings overlay |
| `Quit` | Exit TUI |

## Visual Features

### Galaxy Theme

- **SuperNovae color palette**: Deep space blues, nebula purples, star golds
- **Realm colors**: Global (emerald), Project (violet), Shared (amber)
- **Layer colors**: 9 distinct colors for each functional layer
- **EdgeFamily colors**: 5 relationship type colors

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
   - NavMode switching
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
cargo run -- meta  # Should show meta-graph

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
