# @novanet/cli

Command-line tools for NovaNet development.

## Overview

This package contains development tools and generators:
- **validate**: Schema and relations validation
- **generate**: Documentation and Cypher generators
- **inspect**: Graph inspection tools

## Commands

```bash
# Validate schemas
npm run validate

# Generate documentation
npm run generate:docs
```

## Dependencies

- Uses `@novanet/core` for types, schemas, and generators
- Uses `@novanet/db` for database operations
