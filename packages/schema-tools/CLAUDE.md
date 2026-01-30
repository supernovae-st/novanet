# @novanet/schema-tools

Build-time generators and validators for NovaNet schema artifacts.

## Overview

This package contains tools that run at **build time** (not runtime) to:
- Generate Mermaid diagrams from the schema
- Generate TypeScript subcategory mappings from YAML definitions
- Validate synchronization between YAML, TypeScript, and Neo4j schemas

These tools were moved from `@novanet/core` in v8.2.0 to separate build-time concerns from runtime code.

## Commands

```bash
# From monorepo root
pnpm --filter=@novanet/schema-tools generate:all    # Generate all artifacts
pnpm --filter=@novanet/schema-tools validate:sync   # Validate schema sync
pnpm --filter=@novanet/schema-tools test            # Run tests

# Or from this package
pnpm generate:all
pnpm validate:sync
pnpm test
```

## File Structure

```
schema-tools/
├── src/
│   ├── index.ts                 # Main exports
│   ├── config/
│   │   └── colors.ts            # Full Mermaid color palette
│   ├── generators/
│   │   ├── MermaidGenerator.ts  # Full-graph Mermaid diagrams
│   │   └── SubcategoryGenerator.ts  # TypeScript code generation
│   ├── parsers/
│   │   └── RelationsParser.ts   # YAML relations parser
│   └── utils/
│       └── filePathToNodeName.ts
├── scripts/
│   ├── generate-all.ts          # Generate all artifacts
│   └── validate-sync.ts         # Validate schema synchronization
└── package.json
```

## Exports

```typescript
import {
  MermaidGenerator,
  SubcategoryGenerator,
  RelationsParser,
  filePathToNodeName,
  PALETTE,           // Full color palette
  LAYER_COLORS,      // Layer-specific colors
} from '@novanet/schema-tools';
```

## Usage

### MermaidGenerator

Generates full-graph Mermaid diagrams from the schema:

```typescript
import { MermaidGenerator } from '@novanet/schema-tools';

const mermaid = await MermaidGenerator.generate({
  indexPath: 'models/_index.yaml',
  relationsPath: 'models/relations.yaml',
});
```

### SubcategoryGenerator

Generates TypeScript code for node subcategory mappings:

```typescript
import { SubcategoryGenerator } from '@novanet/schema-tools';

const code = await SubcategoryGenerator.generate({
  indexPath: 'models/_index.yaml',
});
```

### RelationsParser

Parses and validates relations.yaml:

```typescript
import { RelationsParser } from '@novanet/schema-tools';

const relations = await RelationsParser.parse('models/relations.yaml');
```

## Dependencies

- Uses `@novanet/core` for types and schemas
- Uses `yaml` for YAML parsing
- Uses `zod` for validation

## Integration with CI

The `validate:sync` script is called by CI to ensure:
- Generated TypeScript matches YAML definitions
- No drift between schema definition and code

```yaml
# .github/workflows/ci.yml
- name: Validate schema sync
  run: pnpm schema:validate
```
