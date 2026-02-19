---
id: "013"
title: "Icons Source of Truth"
version: "v10.6"
status: "active"
domain: "visual-encoding"
updated: "v11.2, v0.12.0"
---

# ADR-013: Icons Source of Truth

**Status**: Approved (v10.6, updated v11.2)

**Decision**: Centralize all icons in `visual-encoding.yaml`, providing both web (Lucide) and terminal (Unicode) variants.

**Location**: `packages/core/models/visual-encoding.yaml` -> `icons:` section

**Structure**:
```yaml
icons:
  realms:           # shared, org (v11.2: renamed from global, tenant)
  layers:           # config, locale, geography, knowledge, foundation, structure, semantic, instruction, output
  traits:           # defined, authored, imported, generated, retrieved (v0.12.0 ADR-024)
  arc_families:     # ownership, localization, semantic, generation, mining
  states:           # no_connection, no_kinds, no_results, no_instances, loading, success, error, warning
  navigation:       # expanded, collapsed, leaf, search, help, back, copy
  quality:          # complete, partial, empty, required, optional, chart
  modes:            # meta, data, overlay, query, atlas, audit

# Each icon has:
  category:
    key:
      web: "lucide-icon-name"    # For Studio/web
      terminal: "U"              # Unicode for TUI
      description: "..."
```

**Generated artifacts**:
- `tools/novanet/src/tui/theme.rs` -> `Icons` struct (loaded at runtime)
- Future: `packages/core/src/config/icons.generated.ts` (TypeScript constants)

**Rationale**:
- Single source of truth for ALL icons (no duplicates in code)
- Dual format: web (Lucide) + terminal (Unicode) for different contexts
- TUI loads icons from YAML at startup with fallback defaults
- Consistent iconography across Studio and TUI
- Colorblind-safe: icons supplement color, not replace it

**Categories explained**:

| Category | Purpose | Example |
|----------|---------|---------|
| realms | Where node lives | U shared, U org |
| layers | Functional category | U config, U semantic |
| traits | Data origin | U defined, U authored, U imported, U generated, U retrieved |
| states | UI empty states | U loading, U no_kinds |
| navigation | Tree controls | U expanded, U collapsed |
| quality | Data completeness | U complete, U partial |
| modes | Navigation modes | M meta, D data |
