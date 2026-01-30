<div align="center">

# 🪽 NovaNet CLI

**Command-line tools for NovaNet development**

[![TypeScript](https://img.shields.io/badge/TypeScript-5.9-3178C6?style=flat-square&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Node](https://img.shields.io/badge/Node-≥20-339933?style=flat-square&logo=node.js&logoColor=white)](https://nodejs.org/)

---

*Part of the [🪽 NovaNet Monorepo](../../README.md)*

</div>

---

## Overview

This package provides development tools for the NovaNet ecosystem:

- **validate** — Schema and relations validation
- **generate** — Documentation and Cypher generators
- **inspect** — Graph inspection tools

---

## Commands

```bash
# From monorepo root
pnpm --filter=@novanet/cli validate        # Validate schemas
pnpm --filter=@novanet/cli generate:docs   # Generate documentation

# Or from this package
pnpm validate
pnpm generate:docs
```

---

## Features

### Schema Validation

Validates YAML schema definitions in `@novanet/core/models/`:

- Node definitions match TypeScript types
- Relations reference valid node types
- Required properties are defined

### Documentation Generation

Generates Markdown documentation from view definitions:

- Mermaid diagrams from graph schema
- Node type reference tables
- Cypher query examples

---

## Dependencies

| Package | Purpose |
|---------|---------|
| [@novanet/core](../core/) | Types, schemas, generators |

---

## Related Packages

| Package | Description |
|---------|-------------|
| [@novanet/core](../core/) | Types, schemas, filters |
| [@novanet/db](../db/) | Neo4j infrastructure |
| [@novanet/studio](../../apps/studio/) | Graph visualization |

---

<div align="center">

**[🪽 NovaNet](../../README.md)** · [SuperNovae Studio](https://github.com/supernovae-st)

</div>
