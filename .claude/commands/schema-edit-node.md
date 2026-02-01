---
description: Edit an existing node type in the NovaNet knowledge graph
argument-hint: <node-name>
allowed-tools: Read, Write, Edit, Glob, Grep, Bash, AskUserQuestion
---

Modify an existing node type in the NovaNet ontology.

## Workflow

1. **Load Phase**: Read the current node definition:
   - Find YAML file: `packages/core/models/nodes/**/{node-name}.yaml`
   - Display current properties and relationships

2. **Discovery Phase**: Ask what to modify:
   - Add new properties?
   - Remove properties? (check for breaking changes)
   - Modify property types/constraints?
   - Add/remove relationships?

3. **Impact Analysis**:
   - Check TypeScript types that will change
   - Check seed files that use this node
   - Check queries that reference this node
   - Identify breaking changes

4. **Modification Phase**:
   - Update YAML file
   - Update `relations.yaml` if relationships changed
   - Run `novanet schema generate`
   - Run `novanet schema validate`

5. **Migration Phase** (if breaking):
   - Create migration in `packages/db/migrations/`
   - Update seed files if needed

## Naming Requirements

- **Node names**: PascalCase (e.g., `Concept`, `LocaleVoice`)
- **Property names**: snake_case (e.g., `display_name`, `llm_context`)
- **New properties**: Follow existing patterns in the node's YAML

## Safety Rules

- **NEVER** remove properties without explicit confirmation
- **ALWAYS** create migrations for breaking changes
- **ALWAYS** validate sync after changes

## Example

```bash
/schema:edit-node Concept
```

This will show current Concept definition and ask what you want to modify.
