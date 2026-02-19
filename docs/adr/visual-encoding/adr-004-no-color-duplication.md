---
id: 4
title: "No Color Duplication"
version: "v9.5"
status: stable
domain: visual-encoding
---

# ADR-004: No Color Duplication

**Status**: Approved (v9.5)

**Decision**: Colors are defined ONLY in `taxonomy.yaml`. Visual encoding references colors, doesn't duplicate them.

```yaml
# taxonomy.yaml - Colors defined here
node_realms:
  - key: shared       # v11.2: renamed from global
    color: "#2aa198"
  - key: org          # v11.2: renamed from tenant
    color: "#0ea5e9"

# visual-encoding.yaml - References, no hex values
channel_mapping:
  node:
    fill_color: layer    # Uses taxonomy.node_layers[].color
    border_color: realm  # Uses taxonomy.node_realms[].color
```

**Rationale**: Single source of truth for colors prevents inconsistencies.
