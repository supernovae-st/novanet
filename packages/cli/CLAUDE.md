# @novanet/cli

Command-line tools for NovaNet development.

## Status

**Coming soon** - This package is a placeholder for future CLI tools.

## Planned Features

- **validate**: Schema and relations validation
- **generate**: Documentation and Cypher generators
- **inspect**: Graph inspection tools

## Current State

The CLI currently only contains a stub entry point. Actual implementation is planned for a future release.

For now, use the scripts in individual packages:
- `@novanet/core` - `pnpm --filter=@novanet/core validate:docs`
- `@novanet/schema-tools` - `pnpm --filter=@novanet/schema-tools validate:sync`

## Dependencies

- Uses `@novanet/core` for types, schemas, and generators
