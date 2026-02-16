---
paths:
  - "tools/novanet/**/*.rs"
  - "tools/novanet/Cargo.toml"
---

# Rust Development Rules

## Error Handling
- Use `thiserror` for library errors (`NovaNetError` enum in `error.rs`)
- Use `color-eyre` only in `main.rs` for CLI error display
- Never use `.unwrap()` or `.expect()` in library code - propagate with `?`
- Pattern: `Result<T, NovaNetError>` aliased as `crate::Result<T>`

## Async Patterns
- Use `tokio` runtime with `neo4rs` for Neo4j
- Wrap `neo4rs::Graph` in `Arc` for sharing across tasks
- Use `mpsc` channels for TUI async communication

## Code Style
- Run `cargo fmt` before committing
- Zero `cargo clippy` warnings policy
- Edition 2021 features preferred
- Prefer `impl Into<String>` over `&str` for flexibility

## Module Structure
```
src/
  lib.rs          # Public API re-exports
  main.rs         # Thin CLI entry point
  config.rs       # Root discovery, paths
  db.rs           # Neo4j connection pool
  error.rs        # NovaNetError enum
  commands/       # CLI command handlers
  parsers/        # YAML parsing
  generators/     # Code generation
  tui/            # Terminal UI (feature-gated)
```

## Testing
- Unit tests in same file with `#[cfg(test)]`
- Integration tests require Neo4j: `cargo test -- --ignored`
- Current: 1055+ tests passing (use `cargo nextest run` for speed)

## TUI (ratatui)
- Feature-gated behind `tui` feature
- Galaxy theme colors in `tui/theme.rs`
- State machine pattern in `tui/app.rs`
