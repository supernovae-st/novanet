# @novanet/cli

Command-line tools for NovaNet development.

## Overview

This package contains development tools and generators:
- **validate**: Schema and relations validation
- **generate**: Documentation and Cypher generators
- **inspect**: Graph inspection tools

## Commands

```bash
# From monorepo root
pnpm --filter=@novanet/cli validate        # Validate schemas
pnpm --filter=@novanet/cli generate:docs   # Generate documentation

# Or from this package
pnpm validate
pnpm generate:docs
```

## Dependencies

- Uses `@novanet/core` for types, schemas, and generators
