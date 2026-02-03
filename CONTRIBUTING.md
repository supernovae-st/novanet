# Contributing to NovaNet

Thank you for your interest in contributing to NovaNet!

## Getting Started

### Prerequisites

- Node.js >= 20
- pnpm >= 9
- Docker (for Neo4j)
- Rust >= 1.84 (for CLI/TUI)

### Setup

```bash
# Clone the repository
git clone git@github.com:supernovae-st/novanet-dev.git
cd novanet-dev

# Install dependencies
pnpm install

# Start Neo4j
pnpm infra:up

# Seed database
pnpm infra:seed

# Start development
pnpm dev
```

## Development Workflow

### Branch Strategy

```
main            # Stable releases only
feat/*          # Feature branches
fix/*           # Bug fixes
docs/*          # Documentation changes
```

### Making Changes

1. Create a feature branch from `main`
2. Make your changes
3. Run quality checks:
   ```bash
   pnpm lint
   pnpm type-check
   pnpm test
   ```
4. For Rust changes:
   ```bash
   cd tools/novanet
   cargo fmt
   cargo clippy -- -D warnings
   cargo test
   ```
5. Commit using Conventional Commits
6. Open a Pull Request

### Commit Convention

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): description

feat(studio): add dark mode toggle
fix(core): resolve type export issue
docs(readme): update installation steps
refactor(novanet): simplify TUI state machine
chore: update dependencies
```

**Types:** `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `perf`

**Scopes:** `core`, `studio`, `db`, `novanet`, `ci`

## Code Style

### TypeScript

- 2 spaces indentation
- Single quotes
- Semicolons required
- 100 char line length
- Strict TypeScript (`strict: true`)

### Rust

- `cargo fmt` for formatting
- `cargo clippy -- -D warnings` (zero warnings policy)
- Edition 2024

## Testing

### TypeScript

```bash
pnpm test                 # Run all tests
pnpm test:coverage        # With coverage (target: 80%)
pnpm test:watch           # Watch mode
```

### Rust

```bash
cargo test                # Unit tests
cargo test -- --ignored   # Integration tests (requires Neo4j)
```

## Project Structure

```
novanet-dev/
├── packages/
│   ├── core/           # Types, schemas, filters
│   └── db/             # Neo4j infrastructure
├── apps/
│   └── studio/         # Web visualization
└── tools/
    └── novanet/        # Rust CLI + TUI
```

## Pull Request Guidelines

- PRs should target `main`
- Include tests for new features
- Update documentation if needed
- Keep PRs focused (one feature/fix per PR)
- CI must pass before merge

## Need Help?

- Check existing [issues](https://github.com/supernovae-st/novanet-dev/issues)
- Read the [CLAUDE.md](./CLAUDE.md) for architecture details
- Review [ROADMAP.md](./ROADMAP.md) for planned features

## License

This project is proprietary software owned by [SuperNovae Studio](https://github.com/supernovae-st).
