# Macropad Components

3D visualization components for Work Louder devices.

## Work Louder Creator Micro - Layout Exact

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  WORK LOUDER CREATOR MICRO - LAYOUT EXACT                                     ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║                         [USB-C port]                                          ║
║                              │                                                ║
║     ┌────────────────────────┴────────────────────────┐                       ║
║     │  ○ screw                                ○ screw │                       ║
║     │                                                 │                       ║
║     │  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐           │  ROW 0                ║
║     │  │ ENC1 │ │ 0,1  │ │ 0,2  │ │ ENC2 │           │  (encoders + keys)    ║
║     │  │SILVER│ └──────┘ └──────┘ │BLACK │           │                       ║
║     │  └──────┘                   └──────┘           │                       ║
║     │                                                 │                       ║
║     │  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐           │  ROW 1                ║
║     │  │ 1,0  │ │ 1,1  │ │ 1,2  │ │ 1,3  │           │  (CORAL, PURPLE)      ║
║     │  └──────┘ │CORAL │ │PURPLE│ └──────┘           │                       ║
║     │           └──────┘ └──────┘                    │                       ║
║     │                                                 │                       ║
║     │  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐           │  ROW 2                ║
║     │  │ 2,0  │ │ 2,1  │ │ 2,2  │ │ 2,3  │           │  (BLUE, GREEN)        ║
║     │  └──────┘ └──────┘ │ BLUE │ │GREEN │           │                       ║
║     │                    └──────┘ └──────┘           │                       ║
║     │                                                 │                       ║
║     │  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐           │  ROW 3                ║
║     │  │ 3,0  │ │ 3,1  │ │ 3,2  │ │ 3,3  │           │  (all black)          ║
║     │  └──────┘ └──────┘ └──────┘ └──────┘           │                       ║
║     │                                                 │                       ║
║     │  ○ screw         "all work & no play"  ○ screw │                       ║
║     └─────────────────────────────────────────────────┘                       ║
║                                                                               ║
║  TOTAL: 14 keys + 2 encoders in 4×4 grid + 4 screws (corners)                ║
║                                                                               ║
║  KEY COLORS:                                                                  ║
║  - Row 1, Col 1: CORAL/RED (#ef4444)                                         ║
║  - Row 1, Col 2: PURPLE (#a855f7)                                            ║
║  - Row 2, Col 2: BLUE (#3b82f6)                                              ║
║  - Row 2, Col 3: GREEN (#22c55e)                                             ║
║  - All others: BLACK (#1a1a1a)                                               ║
║                                                                               ║
║  ENCODERS (in grid at positions [0,0] and [0,3]):                            ║
║  - [0,0] Left: Silver knurled cylinder                                       ║
║  - [0,3] Right: Black matte volume knob                                      ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

## Specifications

| Property        | Value                                      |
| --------------- | ------------------------------------------ |
| Matrix          | 4 rows x 4 cols = **14 keys + 2 encoders** |
| Encoders        | 2 (at positions [0,0] and [0,3])           |
| ENC1 [0,0]      | Silver knurled cylinder                    |
| ENC2 [0,3]      | Black matte volume knob                    |
| Chassis         | White/light gray plastic, rounded corners  |
| RGB Underglow   | Yes (rainbow animated)                     |
| USB             | USB-C (top center)                         |
| Screws          | 4 corner Phillips screws (dark gray)       |
| Branding        | "all work & no play" (bottom center)       |

## Key Colors

| Position | Color  | Hex       |
| -------- | ------ | --------- |
| [1,1]    | Coral  | `#ef4444` |
| [1,2]    | Purple | `#a855f7` |
| [2,2]    | Blue   | `#3b82f6` |
| [2,3]    | Green  | `#22c55e` |
| Others   | Black  | `#1a1a1a` |
| Chassis  | White  | `#f5f5f5` |

## Components

### CreatorBoardLowPoly.tsx

Main 3D visualizer using React Three Fiber.

Features:
- 16 keys in 4x4 grid
- 2 encoders above the grid
- Rainbow RGB underglow (animated HSL)
- Press animation on keys
- Simple low-poly aesthetic

### MacropadVisualizer.tsx

Modal wrapper with layer tabs, key details panel, WebHID connection.

## MCP Magic Prompt

```
Create a simple 3D macropad component with React Three Fiber.

EXACT LAYOUT:
- 4x4 grid with 14 square keys + 2 rotary encoders
- Encoders at positions [0,0] (top-left) and [0,3] (top-right)
- White/cream chassis with rounded corners
- Rainbow RGB underglow around the edges (animated)
- 4 corner screws

KEY COLORS:
- Position [1,1]: Coral red
- Position [1,2]: Purple
- Position [2,2]: Blue
- Position [2,3]: Green
- All others: Matte black

ENCODER STYLES:
- [0,0] Left encoder: Silver metallic knurled cylinder
- [0,3] Right encoder: Black matte volume knob with white indicator dot

INTERACTIONS:
- Keys are clickable with satisfying press animation (push down + scale)
- Simple, clean, low-poly aesthetic
- Smooth rainbow color animation on underglow
```

## WebHID Integration

- Vendor ID: `0x574C` (Work Louder)
- Product ID: `0xe6e3` (Creator Micro)

See `src/config/macropad/devices.ts` for device definitions.
