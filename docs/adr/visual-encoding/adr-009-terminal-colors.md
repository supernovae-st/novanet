---
id: 9
title: "Terminal Color Graceful Degradation"
version: "v9.5"
status: stable
domain: visual-encoding
---

# ADR-009: Terminal Color Graceful Degradation

**Status**: Approved (v9.5)

**Decision**: TUI supports three color modes with automatic fallback.

```
truecolor (24-bit RGB)
    | not supported
256-color (xterm palette)
    | not supported
16-color (ANSI)
```

**Rationale**: Works on all terminals from VS Code to minimal SSH.
