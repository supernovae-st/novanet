# NovaNet DX Improvement Plan

**Date**: 2026-02-10
**Version**: v11.5.0
**Status**: Ready for Implementation

## Executive Summary

This plan documents 8 DX improvements based on research from Perplexity (best practices), Context7 (library docs), and codebase exploration. All improvements follow existing patterns and require minimal changes.

---

## Improvement Overview

| # | Feature | Effort | Impact | Files Changed |
|---|---------|--------|--------|---------------|
| 1 | Shell completions | 30 lines | HIGH | 3 Rust files |
| 2 | TUI footer hints | 75 lines | HIGH | 1 Rust file |
| 3 | Doctor command | 100 lines | HIGH | 3 Rust files |
| 4 | turbo.json improvements | 20 lines | MEDIUM | 2 JSON files |
| 5 | lib/ barrel exports | 10 lines | LOW | 1 TS file |
| 6 | g/G navigation (TUI) | 10 lines | MEDIUM | Already done! |
| 7 | --explain flag | 50 lines | MEDIUM | 2 Rust files |
| 8 | Format check task | 5 lines | LOW | 1 JSON file |

---

## 1. Shell Completions Command

### Goal
Add `novanet completions <shell>` to generate shell completions for bash, zsh, fish.

### Implementation

**File: `tools/novanet/Cargo.toml`** - Add dependency:
```toml
[dependencies]
clap_complete = "4"
```

**File: `tools/novanet/src/commands/mod.rs`** - Register module:
```rust
pub mod completions;
```

**File: `tools/novanet/src/commands/completions.rs`** - New file:
```rust
use clap::{Command, CommandFactory};
use clap_complete::{generate, Shell};
use std::io;

use crate::main::Cli;

pub fn run_completions(shell: Shell) -> crate::Result<()> {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    generate(shell, &mut cmd, name, &mut io::stdout());
    Ok(())
}
```

**File: `tools/novanet/src/main.rs`** - Add command:
```rust
// Add to Commands enum (around line 120):
/// Generate shell completions
Completions {
    /// Shell to generate for (bash, zsh, fish, powershell, elvish)
    #[arg(value_enum)]
    shell: clap_complete::Shell,
},

// Add to dispatch (around line 850):
Commands::Completions { shell } => {
    novanet::commands::completions::run_completions(shell)?;
}
```

### Usage
```bash
# Generate and install
novanet completions bash > ~/.local/share/bash-completion/completions/novanet
novanet completions zsh > ~/.zfunc/_novanet
novanet completions fish > ~/.config/fish/completions/novanet.fish
```

---

## 2. TUI Footer Hints Panel

### Goal
Add contextual keybinding hints footer below status bar, following audit.rs pattern.

### Implementation

**File: `tools/novanet/src/tui/ui/mod.rs`**

**Step 1**: Modify layout constraints (line ~440):
```rust
// Change from:
.constraints([
    Constraint::Length(1), // Header
    Constraint::Min(0),    // Main content
    Constraint::Length(1), // Status bar
])

// To:
.constraints([
    Constraint::Length(1), // Header
    Constraint::Min(0),    // Main content
    Constraint::Length(1), // Status bar
    Constraint::Length(2), // Footer hints
])
```

**Step 2**: Add render call (line ~460):
```rust
render_header(f, chunks[0], app);
render_main(f, chunks[1], app);
render_status(f, chunks[2], app);
render_footer_hints(f, chunks[3], app);  // ADD THIS
```

**Step 3**: Add new function (after render_status):
```rust
/// Render contextual keybinding hints in footer
fn render_footer_hints(f: &mut Frame, area: Rect, app: &App) {
    let hints = match app.mode {
        NavMode::Graph => match app.focus {
            Focus::Tree => "[h/l] Expand/Collapse  [j/k] Navigate  [Space] Toggle  [g/G] Top/Bottom  [Tab] Next Panel",
            Focus::Info => "[j/k] Scroll  [Tab] Next Panel  [y] Copy Value  [Enter] Select",
            Focus::Yaml => "[j/k] Scroll  [Tab] Next Panel  [y] Copy YAML",
            Focus::Graph => "[Click] Select Node  [Drag] Pan  [Scroll] Zoom  [Tab] Next Panel",
        },
        NavMode::Audit => "[j/k] Navigate  [Enter] Drill Down  [r] Refresh  [1-3] Switch Mode  [?] Help",
        NavMode::Nexus => "[1-4] Tabs  [j/k] Navigate  [Enter] Select  [Esc] Back  [/] Search",
    };

    let line = Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled(hints, Style::default().fg(Color::Rgb(100, 100, 120))),
    ]);

    let paragraph = Paragraph::new(line)
        .style(Style::default().bg(Color::Rgb(15, 15, 25)));

    f.render_widget(paragraph, area);
}
```

---

## 3. Doctor Command

### Goal
Add `novanet doctor` for system health checks (Neo4j, YAML, schema sync).

### Implementation

**File: `tools/novanet/src/commands/mod.rs`**:
```rust
pub mod doctor;
```

**File: `tools/novanet/src/commands/doctor.rs`**:
```rust
use std::path::Path;
use crate::db::Db;
use crate::config;
use crate::parsers;

#[derive(Debug)]
pub struct HealthCheck {
    pub name: &'static str,
    pub status: HealthStatus,
    pub message: String,
}

#[derive(Debug)]
pub enum HealthStatus {
    Ok,
    Warning,
    Error,
}

pub async fn run_doctor(root: &Path, db: Option<&Db>) -> crate::Result<Vec<HealthCheck>> {
    let mut checks = Vec::new();

    // 1. Check YAML validity
    checks.push(check_yaml_validity(root)?);

    // 2. Check Neo4j connection
    if let Some(db) = db {
        checks.push(check_neo4j_connection(db).await?);
    } else {
        checks.push(HealthCheck {
            name: "Neo4j Connection",
            status: HealthStatus::Warning,
            message: "Skipped (no connection provided)".into(),
        });
    }

    // 3. Check schema sync
    checks.push(check_schema_sync(root)?);

    // 4. Check models directory
    checks.push(check_models_directory(root)?);

    // Print results
    for check in &checks {
        let icon = match check.status {
            HealthStatus::Ok => "✓",
            HealthStatus::Warning => "⚠",
            HealthStatus::Error => "✗",
        };
        println!("{} {} - {}", icon, check.name, check.message);
    }

    Ok(checks)
}

fn check_yaml_validity(root: &Path) -> crate::Result<HealthCheck> {
    let models_dir = config::models_dir(root);
    match parsers::taxonomy::load_taxonomy(&models_dir) {
        Ok(_) => Ok(HealthCheck {
            name: "YAML Validity",
            status: HealthStatus::Ok,
            message: "taxonomy.yaml loads successfully".into(),
        }),
        Err(e) => Ok(HealthCheck {
            name: "YAML Validity",
            status: HealthStatus::Error,
            message: format!("Failed to load: {}", e),
        }),
    }
}

async fn check_neo4j_connection(db: &Db) -> crate::Result<HealthCheck> {
    match db.execute("RETURN 1").await {
        Ok(_) => Ok(HealthCheck {
            name: "Neo4j Connection",
            status: HealthStatus::Ok,
            message: "Connected successfully".into(),
        }),
        Err(e) => Ok(HealthCheck {
            name: "Neo4j Connection",
            status: HealthStatus::Error,
            message: format!("Failed: {}", e),
        }),
    }
}

fn check_schema_sync(root: &Path) -> crate::Result<HealthCheck> {
    // Check if generated files match YAML
    let issues = crate::commands::schema::schema_validate(root)?;
    if issues.is_empty() {
        Ok(HealthCheck {
            name: "Schema Sync",
            status: HealthStatus::Ok,
            message: "All artifacts in sync with YAML".into(),
        })
    } else {
        Ok(HealthCheck {
            name: "Schema Sync",
            status: HealthStatus::Warning,
            message: format!("{} issues found", issues.len()),
        })
    }
}

fn check_models_directory(root: &Path) -> crate::Result<HealthCheck> {
    let models = config::models_dir(root);
    let node_kinds = models.join("node-classes");
    let arc_kinds = models.join("arc-classes");

    if node_kinds.exists() && arc_kinds.exists() {
        Ok(HealthCheck {
            name: "Models Directory",
            status: HealthStatus::Ok,
            message: format!("Found at {}", models.display()),
        })
    } else {
        Ok(HealthCheck {
            name: "Models Directory",
            status: HealthStatus::Error,
            message: "Missing node-classes or arc-classes directories".into(),
        })
    }
}
```

**File: `tools/novanet/src/main.rs`**:
```rust
// Add to Commands enum:
/// Run system health checks
Doctor {
    /// Skip Neo4j connection check
    #[arg(long)]
    skip_db: bool,
},

// Add to dispatch:
Commands::Doctor { skip_db } => {
    let root = root?;
    let db = if *skip_db { None } else { Some(connect_db(&cli).await?) };
    novanet::commands::doctor::run_doctor(&root, db.as_ref()).await?;
}
```

### Usage
```bash
novanet doctor              # Full check
novanet doctor --skip-db    # Skip Neo4j (CI/offline)
```

---

## 4. Turbo.json Improvements

### Goal
Add missing watch tasks and output caching.

### Implementation

**File: `turbo.json`**:
```json
{
  "tasks": {
    "build": {
      "dependsOn": ["^build"],
      "outputs": ["dist/**", ".next/**", "!.next/cache/**"]
    },
    "dev": {
      "cache": false,
      "persistent": true
    },
    "lint": {
      "outputs": [".eslintcache"]
    },
    "type-check": {
      "dependsOn": ["^build"],
      "outputs": ["tsconfig.tsbuildinfo"]
    },
    "test": {
      "outputs": ["coverage/**"]
    },
    "test:watch": {
      "cache": false,
      "persistent": true
    },
    "format:check": {
      "outputs": []
    }
  }
}
```

**File: `package.json`** - Add format:check:
```json
{
  "scripts": {
    "format:check": "turbo run format:check"
  }
}
```

---

## 5. Lib Barrel Exports

### Goal
Add missing exports to `apps/studio/src/lib/index.ts`.

### Implementation

**File: `apps/studio/src/lib/index.ts`** - Add:
```typescript
// Utilities (add at end of file)
export * from './formatters';
export * from './novanetBridge';
export * from './schemaGenerator';
export * from './schemaLayoutELK';
export { toast } from './toast';
```

---

## 6. g/G Navigation

### Status: ALREADY IMPLEMENTED

Found in `tools/novanet/src/tui/app.rs` lines 889+:
```rust
KeyCode::Char('g') | KeyCode::Home => self.select_first(),
KeyCode::Char('G') | KeyCode::End => self.select_last(),
```

---

## 7. --explain Flag (Future)

### Goal
Add `--explain` flag to Neo4j commands to show Cypher before execution.

### Scope
This is a larger change affecting multiple commands. Defer to future sprint.

---

## 8. Format Check Task

### Goal
Add unified format checking for CI.

### Implementation
Covered in turbo.json changes above. Add scripts to packages:

**packages/core/package.json**:
```json
{
  "scripts": {
    "format:check": "prettier --check src/"
  }
}
```

**apps/studio/package.json**:
```json
{
  "scripts": {
    "format:check": "prettier --check src/"
  }
}
```

---

## Implementation Order

1. **Shell completions** - Quick win, immediate DX impact
2. **TUI footer hints** - High visibility improvement
3. **Doctor command** - Essential for troubleshooting
4. **turbo.json** - Improves CI cache efficiency
5. **Lib barrel exports** - Minor cleanup
6. **Format check** - CI improvement

---

## Verification

After implementation, verify:

```bash
# Shell completions
novanet completions bash | head -5

# Doctor command
novanet doctor --skip-db

# TUI footer
cargo run -- tui  # Check footer appears

# Turbo cache
pnpm lint && pnpm lint  # Second run should be cached

# Tests
cargo test
pnpm test
```

---

## References

- Perplexity research: Monorepo DX, Rust CLI, Neo4j tooling
- Context7: Turborepo (801 snippets), Clap (10324 snippets), Ratatui (1398 snippets)
- Codebase exploration: CLI structure, TUI layout, folder organization
