# Version Compatibility Matrix

This document tracks version compatibility between supernovae-agi components.

> **Versioning Policy:** All components use [SemVer](https://semver.org/). Version 0.x indicates pre-production.
> Breaking changes bump MINOR (0.x → 0.y). Patches bump PATCH (0.x.y → 0.x.z).

---

## Current Versions

| Component | Version | Status | Notes |
|-----------|---------|--------|-------|
| **supernovae-agi** | 0.14.0 | Active | Monorepo coordinator |
| **NovaNet** | 0.14.0 | Active | Knowledge graph + MCP server |
| **NovaNet MCP** | 0.5.0 | Active | 8 MCP tools |
| **Nika** | 0.4.1 | Active | Workflow engine + MCP client |

---

## Compatibility Matrix

### supernovae-agi ↔ NovaNet ↔ Nika

| supernovae-agi | NovaNet | Nika | MCP Protocol | Status |
|----------------|---------|------|--------------|--------|
| 0.14.x | 0.14.x | 0.4.x | v1 | ✅ Current |
| 0.15.x | 0.15.x | 0.5.x | v1 | 🔜 Planned |

### MCP Tool Compatibility

NovaNet MCP Server exposes these tools. Nika workflows depend on specific versions.

| Tool | NovaNet Version | Nika Support | Contract |
|------|-----------------|--------------|----------|
| `novanet_generate` | 0.13.0+ | 0.2.0+ | `contracts/novanet_generate.json` |
| `novanet_describe` | 0.13.0+ | 0.2.0+ | `contracts/novanet_describe.json` |
| `novanet_traverse` | 0.13.0+ | 0.2.0+ | `contracts/novanet_traverse.json` |
| `novanet_search` | 0.13.0+ | 0.2.0+ | `contracts/novanet_search.json` |
| `novanet_assemble` | 0.13.0+ | 0.2.0+ | `contracts/novanet_assemble.json` |
| `novanet_atoms` | 0.13.0+ | 0.2.0+ | `contracts/novanet_atoms.json` |
| `novanet_query` | 0.13.0+ | 0.2.0+ | `contracts/novanet_query.json` |
| `novanet_introspect` | 0.14.0+ | 0.4.0+ | `contracts/novanet_introspect.json` |

---

## Breaking Changes Log

### NovaNet Breaking Changes

| Version | Change | Migration |
|---------|--------|-----------|
| 0.13.0 | `*Native` pattern (ADR-029) | Rename EntityContent → EntityNative |
| 0.13.0 | Slug ownership (ADR-030) | Move slug from Entity to Page |
| 0.12.0 | Kind → Class terminology | Update all references |
| 0.12.0 | Trait = Data Origin | Update trait values |

### Nika Breaking Changes

| Version | Change | Migration |
|---------|--------|-----------|
| 0.4.0 | rig-core migration | Use RigProvider, RigAgentLoop |
| 0.4.0 | Removed ClaudeProvider, OpenAIProvider | Use RigProvider::claude(), RigProvider::openai() |
| 0.3.0 | `for_each:` parallelism | Use tokio::spawn JoinSet |
| 0.2.0 | `invoke:` verb added | Use for MCP tool calls |
| 0.2.0 | `agent:` verb added | Use for agentic loops |
| 0.2.0 | EventLog v2 (16 variants) | Update event handlers |

---

## Upgrade Paths

### Upgrading NovaNet (0.12.x → 0.13.x)

```bash
# 1. Update submodule
cd novanet-dev
git fetch && git checkout v0.13.1

# 2. Run migrations
cargo run -- db migrate

# 3. Regenerate schema
cargo run -- schema generate

# 4. Update Nika workflows (if using EntityContent)
# Replace: entity: EntityContent
# With:    entity: EntityNative
```

### Upgrading Nika (0.1.x → 0.2.x)

```bash
# 1. Update submodule
cd nika-dev
git fetch && git checkout v0.2.0

# 2. Update workflows to use new verbs
# - Add `invoke:` for MCP calls (was direct exec)
# - Add `agent:` for agentic loops

# 3. Update event handlers for EventLog v2
```

---

## Version Pinning

### In supernovae-agi

Submodule versions are pinned in `.gitmodules` and tracked via git commit SHA.

```bash
# Check current submodule versions
git submodule status

# Update to specific version
cd novanet-dev && git checkout v0.13.1
cd ../nika-dev && git checkout v0.2.0
cd .. && git add novanet-dev nika-dev && git commit -m "chore(deps): update submodules"
```

### In Workflows

Pin MCP tool versions in workflow YAML:

```yaml
mcp:
  novanet:
    version: ">=0.13.0"  # Minimum version required
    tools:
      - novanet_generate
      - novanet_describe
```

---

## Contract Validation

MCP contracts are validated at:

1. **Build time**: `cargo run -- mcp validate` in nika-dev
2. **CI**: GitHub Actions workflow validates contracts on PR
3. **Runtime**: Nika validates tool responses against contracts

### Running Contract Validation

```bash
# Validate all contracts
cd nika-dev && cargo run -- mcp validate --contracts ../contracts/

# Validate specific tool
cargo run -- mcp validate --tool novanet_generate
```

---

## Release Coordination

When releasing a new version that affects compatibility:

1. **Update this matrix** with new version row
2. **Update contracts/** if MCP interface changes
3. **Tag all affected repos** with compatible versions
4. **Update CHANGELOG.md** with migration notes

### Coordinated Release Checklist

- [ ] NovaNet changes tested with current Nika
- [ ] Nika changes tested with current NovaNet
- [ ] VERSION_MATRIX.md updated
- [ ] Contracts validated
- [ ] CHANGELOGs updated
- [ ] Tags created (same day for breaking changes)

---

## Dependency Versions

### Shared Rust Dependencies

| Crate | NovaNet | Nika | Notes |
|-------|---------|------|-------|
| tokio | 1.43 | 1.43 | Async runtime |
| serde | 1.0 | 1.0 | Serialization |
| clap | 4.5 | 4.5 | CLI framework |
| ratatui | 0.29 | 0.29 | TUI framework |
| rmcp | 0.1 | 0.1 | MCP protocol |

### Shared Node Dependencies

| Package | NovaNet | Notes |
|---------|---------|-------|
| typescript | 5.x | Type checking |
| neo4j-driver | 5.x | Database |
| zod | 3.x | Schema validation |

---

*Last updated: 2026-02-19*
