# Nika Workflow Standardization Plan

**Date:** 2026-02-19
**Status:** In Progress
**Goal:** Professional-grade workflow validation, linting, and DX

---

## Overview

Standardize Nika workflow files with:
1. **File extension**: `.nika.yaml` (mandatory)
2. **JSON Schema**: Full schema validation
3. **Linting**: yamllint + custom rules
4. **IDE support**: VS Code schema association
5. **CI/CD**: GitHub Actions validation

---

## Phase 1: File Extension Standardization

### Convention

```
workflow.nika.yaml     ✅ Correct
workflow.yaml          ❌ Wrong (ambiguous)
workflow.nika          ❌ Wrong (not YAML)
```

### Files to Rename

```
examples/
├── uc1-entity-generation.yaml           → uc1-entity-generation.nika.yaml
├── uc2-multi-locale-generation.yaml     → uc2-multi-locale-generation.nika.yaml
├── uc4-seo-pipeline.yaml                → uc4-seo-pipeline.nika.yaml
├── uc5-graph-traversal.yaml             → uc5-graph-traversal.nika.yaml
├── uc10-comprehensive-landing-page.yaml → uc10-comprehensive-landing-page.nika.yaml
├── agent-novanet.yaml                   → agent-novanet.nika.yaml
├── agent-simple.yaml                    → agent-simple.nika.yaml
└── invoke-novanet.yaml                  → invoke-novanet.nika.yaml
```

---

## Phase 2: JSON Schema

### Schema Location

```
nika-dev/
├── schemas/
│   └── nika-workflow.schema.json    # JSON Schema Draft 2020-12
└── tools/nika/
    └── src/validation/
        └── schema.rs                # Rust validation using jsonschema crate
```

### Schema Structure

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://nika.dev/schemas/workflow.json",
  "title": "Nika Workflow",
  "description": "Schema for .nika.yaml workflow files",
  "type": "object",
  "required": ["schema", "tasks"],
  "properties": {
    "schema": {
      "type": "string",
      "pattern": "^nika/workflow@0\\.[1-4]$",
      "description": "Workflow schema version"
    },
    "workflow": {
      "type": "string",
      "pattern": "^[a-z][a-z0-9-]*$",
      "description": "Workflow identifier"
    },
    "provider": {
      "type": "string",
      "enum": ["claude", "openai", "mock"],
      "description": "Default LLM provider"
    },
    "mcp": {
      "$ref": "#/$defs/McpConfig"
    },
    "tasks": {
      "type": "array",
      "minItems": 1,
      "items": { "$ref": "#/$defs/Task" }
    },
    "flows": {
      "type": "array",
      "items": { "$ref": "#/$defs/Flow" }
    }
  }
}
```

### Rust Integration

```rust
// src/validation/schema.rs
use jsonschema::Validator;
use serde_json::Value;

pub fn validate_workflow(yaml_content: &str) -> Result<(), Vec<ValidationError>> {
    let schema: Value = serde_json::from_str(WORKFLOW_SCHEMA)?;
    let instance: Value = serde_yaml::from_str(yaml_content)?;

    let validator = Validator::new(&schema)?;

    let errors: Vec<_> = validator.iter_errors(&instance).collect();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.into_iter().map(Into::into).collect())
    }
}
```

---

## Phase 3: Linting Configuration

### yamllint Config

```yaml
# .yamllint.yaml
extends: default

rules:
  line-length:
    max: 120
    level: warning

  indentation:
    spaces: 2
    indent-sequences: true

  document-start: disable

  truthy:
    allowed-values: ['true', 'false', 'yes', 'no']

  comments:
    min-spaces-from-content: 1

ignore: |
  node_modules/
  target/
  .git/
```

### Pre-commit Hook

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/adrienverge/yamllint.git
    rev: v1.35.1
    hooks:
      - id: yamllint
        files: \.nika\.yaml$
        args: [-c, .yamllint.yaml, --strict]
```

---

## Phase 4: CLI Enhancements

### Enhanced Validate Command

```bash
# Current
nika validate workflow.nika.yaml

# Enhanced
nika validate workflow.nika.yaml --strict     # Fail on warnings
nika validate workflow.nika.yaml --schema     # Show schema version
nika validate examples/                        # Validate directory
nika validate --check-extension               # Verify .nika.yaml extension
```

### New Lint Command

```bash
nika lint workflow.nika.yaml                  # Lint single file
nika lint examples/                           # Lint directory
nika lint --fix workflow.nika.yaml            # Auto-fix issues
```

---

## Phase 5: IDE Integration

### VS Code Settings

```json
// .vscode/settings.json
{
  "yaml.schemas": {
    "./schemas/nika-workflow.schema.json": "*.nika.yaml"
  },
  "files.associations": {
    "*.nika.yaml": "yaml"
  },
  "[yaml]": {
    "editor.defaultFormatter": "redhat.vscode-yaml",
    "editor.formatOnSave": true
  }
}
```

### Schema Store Registration (Future)

Submit to https://www.schemastore.org for automatic discovery.

---

## Phase 6: CI/CD

### GitHub Actions

```yaml
# .github/workflows/validate-workflows.yml
name: Validate Nika Workflows

on:
  push:
    paths:
      - '**.nika.yaml'
  pull_request:
    paths:
      - '**.nika.yaml'

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install yamllint
        run: pip install yamllint

      - name: Lint YAML
        run: yamllint -c .yamllint.yaml **/*.nika.yaml

      - name: Setup Rust
        uses: dtolnay/rust-action@stable

      - name: Validate Workflows
        run: |
          cd tools/nika
          cargo run -- validate examples/*.nika.yaml
```

---

## Deliverables Checklist

- [ ] All example files renamed to `.nika.yaml`
- [ ] JSON Schema created at `schemas/nika-workflow.schema.json`
- [ ] `.yamllint.yaml` configuration
- [ ] Rust schema validation in `src/validation/schema.rs`
- [ ] VS Code settings in `.vscode/settings.json`
- [ ] GitHub Actions workflow
- [ ] Updated README.md with conventions
- [ ] Updated CLAUDE.md with validation rules

---

## Success Criteria

1. `nika validate` catches schema violations
2. `yamllint` enforces style consistency
3. VS Code shows inline errors + autocompletion
4. CI blocks PRs with invalid workflows
5. All 100% of example files use `.nika.yaml`

---

## References

- JSON Schema: https://json-schema.org/draft/2020-12/schema
- yamllint: https://github.com/adrienverge/yamllint
- YAML Language Server: https://github.com/redhat-developer/yaml-language-server
- Rust jsonschema: https://docs.rs/jsonschema
