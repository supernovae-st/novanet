---
name: release
description: Release workflow for supernovae-agi monorepo. Use when preparing or executing releases for NovaNet, Nika, or coordinated releases.
user-invocable: true
---

# Release Skill

Smart release workflow with automatic version detection and tag proposals.

**Applies to ALL projects in supernovae-agi workspace:**
- `supernovae-agi` (root workspace)
- `nika-dev/tools/nika` (Nika CLI)
- `novanet-dev/tools/novanet-mcp` (NovaNet MCP Server)
- `novanet-dev/tools/novanet` (NovaNet CLI)

## Usage

```bash
/release              # Analyze ALL projects and propose versions
/release push         # Push current project with tag proposal
/release status       # Show pending changes across all projects
/release nika         # Release Nika only (from any location)
/release novanet      # Release NovaNet only
/release mcp          # Release NovaNet MCP only
/release all          # Coordinated release (all projects)
```

## Project Detection

When user says "push", "release", or "sync", detect current project:

```bash
# Detect project from current directory
PWD=$(pwd)
if [[ "$PWD" == *"nika-dev/tools/nika"* ]]; then
    PROJECT="nika"
    VERSION_FILE="Cargo.toml"
elif [[ "$PWD" == *"novanet-dev/tools/novanet-mcp"* ]]; then
    PROJECT="novanet-mcp"
    VERSION_FILE="Cargo.toml"
elif [[ "$PWD" == *"novanet-dev/tools/novanet"* ]]; then
    PROJECT="novanet"
    VERSION_FILE="Cargo.toml"
elif [[ "$PWD" == *"novanet-dev"* ]]; then
    PROJECT="novanet-workspace"
    VERSION_FILE="package.json"
else
    PROJECT="supernovae-agi"
    VERSION_FILE="VERSION"
fi
```

## Smart Push with Tag Proposal

**MANDATORY WORKFLOW** when pushing after significant work:

1. Analyze commits since last tag
2. Detect change types (feat/fix/breaking/ci/test)
3. Propose appropriate version bump
4. Present interactive options

### Automatic Detection

| Commit Type | Version Bump |
|-------------|--------------|
| `BREAKING CHANGE` or `!` | Major (x.0.0) |
| `feat:` | Minor (0.x.0) |
| `fix:`, `ci:`, `test:` | Patch (0.0.x) |

### Interactive Options

When pushing, you MUST present these options:

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📦 Release Check | Last tag: v0.4.1

📝 X commits since v0.4.1:
  ▸ feat(nika): add spawn_agent tool
  ▸ ci(nika): add integration tests
  ▸ test: MVP 8 verification suite

📊 Summary:
  feat: 2 | fix: 1 | ci: 3 | test: 2

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🏷️  Tag Options:

  [1] v0.5.0 (minor) ← Recommended
  [2] v0.4.2 (patch)
  [3] v1.0.0 (major)
  [4] Custom version
  [s] Skip - push without tagging
  [c] Cancel push
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## Implementation

When user says "push", "release", or any sync-related intent:

### Step 1: Analyze

```bash
# Get last tag
LAST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "v0.0.0")

# Count commits
git rev-list ${LAST_TAG}..HEAD --count

# Analyze types
git log ${LAST_TAG}..HEAD --oneline | grep -ci 'feat'
git log ${LAST_TAG}..HEAD --oneline | grep -ci 'fix'
git log ${LAST_TAG}..HEAD --oneline | grep -ciE 'BREAKING|!'
```

### Step 2: Present Options

Use `AskUserQuestion` tool with options:

```json
{
  "questions": [{
    "question": "Version bump for release?",
    "header": "Tag version",
    "options": [
      { "label": "v0.5.0 (minor) (Recommended)", "description": "New features added" },
      { "label": "v0.4.2 (patch)", "description": "Bug fixes and CI only" },
      { "label": "v1.0.0 (major)", "description": "Breaking changes" },
      { "label": "Skip tagging", "description": "Push without version tag" }
    ],
    "multiSelect": false
  }]
}
```

### Step 3: Execute

Based on selection:

```bash
# If version selected:
git tag -a vX.Y.Z -m "Release vX.Y.Z

## Changes
- feat: ...
- fix: ...

Co-Authored-By: Claude <noreply@anthropic.com>"

git push origin main --tags

# If skip:
git push origin main
```

## Version Files to Update

| Project | Files | Current |
|---------|-------|---------|
| Nika | `nika-dev/tools/nika/Cargo.toml`, `.claude/settings.json` | v0.5.0 |
| NovaNet MCP | `novanet-dev/tools/novanet-mcp/Cargo.toml` | v0.5.0 |
| NovaNet CLI | `novanet-dev/tools/novanet/Cargo.toml` | v0.14.0 |
| NovaNet Core | `novanet-dev/package.json` | v0.14.0 |
| Workspace | `VERSION` | v0.14.0 |

## Multi-Project Status Check

When running `/release status`, check ALL projects:

```bash
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📦 Release Status - supernovae-agi Workspace"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check each project
for proj in "nika-dev/tools/nika" "novanet-dev/tools/novanet-mcp" "novanet-dev"; do
    cd "$proj"
    TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "none")
    COMMITS=$(git rev-list ${TAG}..HEAD --count 2>/dev/null || echo "?")
    echo "  $proj: $TAG (+$COMMITS commits)"
    cd -
done
```

## Coordinated Release Workflow

For `/release all` (synchronized versions):

1. Check all projects for pending changes
2. Propose version for each project
3. Update VERSION_MATRIX.md
4. Tag each project
5. Update submodule references in supernovae-agi
6. Tag supernovae-agi workspace
7. Push all with tags

## CI Integration

Tags trigger:
- `.github/workflows/release.yml` - Build artifacts
- `.github/workflows/ci.yml` - Integration tests (tags v*)

## Checklist Before Release

- [ ] All tests pass (`cargo nextest run`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Format check (`cargo fmt --check`)
- [ ] CHANGELOG.md updated (if major/minor)
- [ ] Version bumped in Cargo.toml
- [ ] CI workflows updated if needed

## Examples

### Quick Patch Release

```
User: "push les changements"

Claude:
1. Run git status, see 3 commits (2 fix, 1 ci)
2. Detect: patch release recommended
3. Present options via AskUserQuestion
4. User selects "v0.4.2 (patch)"
5. Create tag, push with --tags
```

### Feature Release

```
User: "c'est bon on peut release"

Claude:
1. Run analysis, see 5 commits (3 feat, 2 test)
2. Detect: minor release recommended
3. Present options
4. User selects "v0.5.0 (minor)"
5. Update Cargo.toml version
6. Create tag with changelog
7. Push with --tags
```

### Skip Tagging

```
User: "push juste les changes ci"

Claude:
1. Run analysis, see 2 commits (ci only)
2. Present options with "Skip" highlighted for CI-only
3. User selects "Skip tagging"
4. Push without tag
```

## Related Skills

- `/workspace-nav` - Switch between projects before release
- `/security-audit` - Run before major releases
- `/codebase-audit` - Clean up before releases
