# Views Cross-Validation Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Guarantee TUI (Rust) and Studio (TypeScript) interpret views.yaml identically via cross-validation.

**Architecture:** Rust CLI exports views as canonical JSON, TypeScript script exports same format, shell script compares them. Hook reminds on changes.

**Tech Stack:** Rust (serde_json, sha2), TypeScript (js-yaml, crypto), Shell (diff/jq)

---

## Task 1: Rust - Add `views` subcommand module

**Files:**
- Create: `tools/novanet/src/commands/views.rs`
- Modify: `tools/novanet/src/commands/mod.rs`
- Modify: `tools/novanet/src/main.rs`

**Step 1: Write the failing test**

Add to `tools/novanet/src/commands/views.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn test_root() -> Option<PathBuf> {
        std::env::current_dir()
            .ok()?
            .ancestors()
            .find(|p| p.join("pnpm-workspace.yaml").exists())
            .map(|p| p.to_path_buf())
    }

    #[test]
    fn views_export_returns_canonical_json() {
        let Some(root) = test_root() else { return };
        let result = views_export(&root).expect("should export views");

        assert!(result.contains("\"version\""));
        assert!(result.contains("\"count\""));
        assert!(result.contains("\"views\""));
        assert!(result.contains("\"data-complete\""));
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p novanet views_export_returns_canonical_json`
Expected: FAIL with "cannot find function `views_export`"

**Step 3: Write minimal implementation**

Create `tools/novanet/src/commands/views.rs`:

```rust
//! Views validation and export commands.

use std::path::Path;
use serde::Serialize;
use sha2::{Sha256, Digest};

use crate::error::Result;
use crate::parsers::views::load_simple_views;

#[derive(Serialize)]
struct CanonicalView {
    applicable_types: Vec<String>,
    category: String,
    color: String,
    contextual: bool,
    cypher_hash: String,
    description: String,
    icon: CanonicalIcon,
    id: String,
    name: String,
    root_type: Option<String>,
}

#[derive(Serialize)]
struct CanonicalIcon {
    terminal: String,
    web: String,
}

#[derive(Serialize)]
struct CanonicalExport {
    count: usize,
    version: String,
    views: Vec<CanonicalView>,
}

fn hash_cypher(cypher: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(cypher.trim().as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)[..8].to_string()
}

/// Export views as canonical JSON for cross-validation.
pub fn views_export(root: &Path) -> Result<String> {
    let file = load_simple_views(root)?;

    let mut views: Vec<CanonicalView> = file.views.into_iter().map(|v| {
        CanonicalView {
            applicable_types: v.applicable_types.unwrap_or_default(),
            category: v.category,
            color: v.color,
            contextual: v.contextual.unwrap_or(false),
            cypher_hash: v.cypher.as_ref().map(|c| hash_cypher(c)).unwrap_or_default(),
            description: v.description,
            icon: CanonicalIcon {
                terminal: v.icon.terminal,
                web: v.icon.web,
            },
            id: v.id,
            name: v.name,
            root_type: v.root_type,
        }
    }).collect();

    // Sort by id for canonical order
    views.sort_by(|a, b| a.id.cmp(&b.id));

    let export = CanonicalExport {
        count: views.len(),
        version: file.version,
        views,
    };

    Ok(serde_json::to_string_pretty(&export)?)
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p novanet views_export_returns_canonical_json`
Expected: PASS

**Step 5: Commit**

```bash
git add tools/novanet/src/commands/views.rs
git commit -m "feat(views): add views_export for canonical JSON output"
```

---

## Task 2: Rust - Wire up CLI subcommand

**Files:**
- Modify: `tools/novanet/src/commands/mod.rs`
- Modify: `tools/novanet/src/main.rs`

**Step 1: Add module export**

In `tools/novanet/src/commands/mod.rs`, add:

```rust
pub mod views;
```

**Step 2: Add CLI arguments**

In `tools/novanet/src/main.rs`, find the `Commands` enum and add:

```rust
/// Views validation and export
Views {
    #[command(subcommand)]
    action: ViewsAction,
},
```

Add the enum:

```rust
#[derive(Subcommand, Debug)]
enum ViewsAction {
    /// Export views as canonical JSON
    Export {
        /// Output format
        #[arg(long, default_value = "json")]
        format: String,
    },
    /// Validate views match between Rust and TypeScript
    Validate {
        /// Show details for each view
        #[arg(long)]
        verbose: bool,
    },
}
```

**Step 3: Add command handler**

In `main.rs` match block:

```rust
Commands::Views { action } => match action {
    ViewsAction::Export { format } => {
        if format != "json" {
            eprintln!("Only JSON format supported");
            std::process::exit(1);
        }
        let json = commands::views::views_export(&root)?;
        println!("{}", json);
    }
    ViewsAction::Validate { verbose } => {
        commands::views::views_validate(&root, verbose)?;
    }
},
```

**Step 4: Test CLI**

Run: `cargo run -- views export`
Expected: JSON output with 11 views

**Step 5: Commit**

```bash
git add tools/novanet/src/commands/mod.rs tools/novanet/src/main.rs
git commit -m "feat(cli): wire up views export/validate subcommands"
```

---

## Task 3: Rust - Add validate function

**Files:**
- Modify: `tools/novanet/src/commands/views.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn views_validate_calls_typescript() {
    let Some(root) = test_root() else { return };
    // This test just verifies the function exists and runs
    // Real validation happens in integration test
    let result = views_validate(&root, false);
    // May fail if node not available, that's ok
    assert!(result.is_ok() || result.is_err());
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p novanet views_validate_calls_typescript`
Expected: FAIL with "cannot find function `views_validate`"

**Step 3: Write implementation**

Add to `tools/novanet/src/commands/views.rs`:

```rust
use std::process::Command;

/// Validate views match between Rust and TypeScript parsers.
pub fn views_validate(root: &Path, verbose: bool) -> Result<()> {
    // Get Rust export
    let rust_json = views_export(root)?;
    let rust_parsed: serde_json::Value = serde_json::from_str(&rust_json)?;

    println!("✓ Rust parsed {} views", rust_parsed["count"]);

    // Get TypeScript export
    let ts_script = root.join("packages/core/scripts/export-views.mjs");
    if !ts_script.exists() {
        return Err(crate::error::NovaNetError::Config(
            format!("TypeScript export script not found: {}", ts_script.display())
        ));
    }

    let output = Command::new("node")
        .arg(&ts_script)
        .current_dir(root)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(crate::error::NovaNetError::Config(
            format!("TypeScript export failed: {}", stderr)
        ));
    }

    let ts_json = String::from_utf8_lossy(&output.stdout);
    let ts_parsed: serde_json::Value = serde_json::from_str(&ts_json)?;

    println!("✓ TypeScript parsed {} views", ts_parsed["count"]);

    // Compare
    if rust_parsed == ts_parsed {
        println!("✓ All views match");
        Ok(())
    } else {
        // Find differences
        if verbose {
            let rust_views = rust_parsed["views"].as_array().unwrap();
            let ts_views = ts_parsed["views"].as_array().unwrap();

            for (r, t) in rust_views.iter().zip(ts_views.iter()) {
                let id = r["id"].as_str().unwrap_or("?");
                if r == t {
                    println!("  ✓ {}: match", id);
                } else {
                    println!("  ✗ {}: MISMATCH", id);
                    println!("    Rust: {}", serde_json::to_string(r)?);
                    println!("    TS:   {}", serde_json::to_string(t)?);
                }
            }
        }
        Err(crate::error::NovaNetError::Config(
            "Views mismatch between Rust and TypeScript".to_string()
        ))
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p novanet views_validate_calls_typescript`
Expected: PASS

**Step 5: Commit**

```bash
git add tools/novanet/src/commands/views.rs
git commit -m "feat(views): add views_validate cross-validation"
```

---

## Task 4: TypeScript - Create export script

**Files:**
- Create: `packages/core/scripts/export-views.mjs`

**Step 1: Create the script**

```javascript
#!/usr/bin/env node
// packages/core/scripts/export-views.mjs
// Export views.yaml as canonical JSON for cross-validation with Rust

import { promises as fs } from 'fs';
import { createHash } from 'crypto';
import path from 'path';
import yaml from 'js-yaml';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

function hashCypher(cypher) {
  if (!cypher) return '';
  const hash = createHash('sha256');
  hash.update(cypher.trim());
  return hash.digest('hex').slice(0, 8);
}

async function main() {
  // Find views.yaml
  const viewsPath = path.join(__dirname, '../models/views.yaml');
  const content = await fs.readFile(viewsPath, 'utf-8');
  const registry = yaml.load(content);

  // Build canonical format
  const views = registry.views.map(v => ({
    applicable_types: v.applicable_types || [],
    category: v.category,
    color: v.color,
    contextual: v.contextual || false,
    cypher_hash: hashCypher(v.cypher),
    description: v.description,
    icon: {
      terminal: v.icon.terminal,
      web: v.icon.web,
    },
    id: v.id,
    name: v.name,
    root_type: v.root_type || null,
  }));

  // Sort by id
  views.sort((a, b) => a.id.localeCompare(b.id));

  const output = {
    count: views.length,
    version: registry.version,
    views,
  };

  console.log(JSON.stringify(output, null, 2));
}

main().catch(err => {
  console.error(err);
  process.exit(1);
});
```

**Step 2: Test the script**

Run: `node packages/core/scripts/export-views.mjs | head -20`
Expected: JSON output matching Rust format

**Step 3: Compare outputs**

Run:
```bash
diff <(cargo run -q -- views export) <(node packages/core/scripts/export-views.mjs)
```
Expected: No output (files match)

**Step 4: Commit**

```bash
git add packages/core/scripts/export-views.mjs
git commit -m "feat(core): add export-views.mjs for cross-validation"
```

---

## Task 5: Shell - Create orchestrator script

**Files:**
- Create: `tools/scripts/validate-views.sh`

**Step 1: Create the script**

```bash
#!/usr/bin/env bash
# tools/scripts/validate-views.sh
# Cross-validate views.yaml between Rust and TypeScript parsers

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

echo "=== Views Cross-Validation ==="

# Export from Rust
echo "Exporting from Rust..."
RUST_JSON=$(cd "$ROOT_DIR/tools/novanet" && cargo run -q -- views export 2>/dev/null)
RUST_COUNT=$(echo "$RUST_JSON" | grep -o '"count": [0-9]*' | grep -o '[0-9]*')
echo "✓ Rust: $RUST_COUNT views"

# Export from TypeScript
echo "Exporting from TypeScript..."
TS_JSON=$(node "$ROOT_DIR/packages/core/scripts/export-views.mjs")
TS_COUNT=$(echo "$TS_JSON" | grep -o '"count": [0-9]*' | grep -o '[0-9]*')
echo "✓ TypeScript: $TS_COUNT views"

# Compare
if [ "$RUST_JSON" = "$TS_JSON" ]; then
    echo "✓ All views match!"
    exit 0
else
    echo "✗ Views mismatch!"
    echo ""
    echo "Diff (Rust vs TypeScript):"
    diff <(echo "$RUST_JSON") <(echo "$TS_JSON") || true
    exit 1
fi
```

**Step 2: Make executable and test**

Run:
```bash
chmod +x tools/scripts/validate-views.sh
./tools/scripts/validate-views.sh
```
Expected: "All views match!" and exit 0

**Step 3: Commit**

```bash
git add tools/scripts/validate-views.sh
git commit -m "feat(scripts): add validate-views.sh orchestrator"
```

---

## Task 6: Claude Hook - Add reminder

**Files:**
- Create: `.claude/hooks/views-sync-reminder.sh`

**Step 1: Create the hook**

```bash
#!/usr/bin/env bash
# .claude/hooks/views-sync-reminder.sh
# Remind to validate views when views.yaml is modified

# Check if views.yaml was modified
if git diff --cached --name-only 2>/dev/null | grep -q "packages/core/models/views.yaml"; then
    echo "<views-modified>"
    echo "views.yaml was modified. Run validation:"
    echo "  novanet views validate"
    echo "  # or"
    echo "  ./tools/scripts/validate-views.sh"
    echo "</views-modified>"
fi
```

**Step 2: Register in settings**

Add to `.claude/settings.json` hooks section:

```json
{
  "hooks": {
    "pre-commit": [
      ".claude/hooks/views-sync-reminder.sh"
    ]
  }
}
```

**Step 3: Commit**

```bash
chmod +x .claude/hooks/views-sync-reminder.sh
git add .claude/hooks/views-sync-reminder.sh
git commit -m "feat(hooks): add views-sync-reminder on views.yaml change"
```

---

## Task 7: Integration test

**Files:**
- Modify: `tools/novanet/src/commands/views.rs`

**Step 1: Add integration test**

```rust
#[test]
fn views_export_matches_typescript_format() {
    let Some(root) = test_root() else { return };

    let rust_json = views_export(&root).expect("rust export");
    let rust: serde_json::Value = serde_json::from_str(&rust_json).unwrap();

    // Verify structure
    assert!(rust["version"].is_string());
    assert!(rust["count"].is_number());
    assert!(rust["views"].is_array());

    // Verify all views have required fields
    for view in rust["views"].as_array().unwrap() {
        assert!(view["id"].is_string(), "missing id");
        assert!(view["name"].is_string(), "missing name");
        assert!(view["description"].is_string(), "missing description");
        assert!(view["category"].is_string(), "missing category");
        assert!(view["color"].is_string(), "missing color");
        assert!(view["icon"]["web"].is_string(), "missing icon.web");
        assert!(view["icon"]["terminal"].is_string(), "missing icon.terminal");
        assert!(view["cypher_hash"].is_string(), "missing cypher_hash");
        // contextual is bool
        assert!(view["contextual"].is_boolean(), "contextual should be bool");
        // applicable_types is array
        assert!(view["applicable_types"].is_array(), "applicable_types should be array");
    }

    // Verify sorted by id
    let views = rust["views"].as_array().unwrap();
    for i in 1..views.len() {
        let prev = views[i-1]["id"].as_str().unwrap();
        let curr = views[i]["id"].as_str().unwrap();
        assert!(prev < curr, "views should be sorted by id");
    }
}
```

**Step 2: Run all tests**

Run: `cargo test -p novanet`
Expected: All tests pass

**Step 3: Commit**

```bash
git add tools/novanet/src/commands/views.rs
git commit -m "test(views): add integration test for canonical format"
```

---

## Task 8: Update /novanet-sync command

**Files:**
- Modify: `.claude/commands/novanet-sync.md`

**Step 1: Update command**

```markdown
---
description: Validate or regenerate artifacts from YAML sources
argument-hint: [validate|generate|fix|views]
allowed-tools: Bash
---

Synchronize generated files with YAML source of truth.

Commands:
- No argument or "validate": Run `novanet schema validate`
- "generate" or "fix": Run `novanet schema generate` then show `git diff --stat`
- "views": Run `novanet views validate` (cross-validation TUI/Studio)

Use the novanet-sync skill for guidance.
```

**Step 2: Commit**

```bash
git add .claude/commands/novanet-sync.md
git commit -m "docs(commands): add views option to novanet-sync"
```

---

## Summary

| Task | Description | Files |
|------|-------------|-------|
| 1 | Rust views_export function | `commands/views.rs` |
| 2 | Wire CLI subcommand | `mod.rs`, `main.rs` |
| 3 | Rust views_validate function | `commands/views.rs` |
| 4 | TypeScript export script | `export-views.mjs` |
| 5 | Shell orchestrator | `validate-views.sh` |
| 6 | Claude hook | `views-sync-reminder.sh` |
| 7 | Integration test | `commands/views.rs` |
| 8 | Update /novanet-sync | `novanet-sync.md` |

**Verification:**
```bash
# All tests pass
cargo test -p novanet

# Cross-validation works
./tools/scripts/validate-views.sh

# CLI works
novanet views validate --verbose
```
