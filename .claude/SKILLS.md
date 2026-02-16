# NovaNet Skills

Automatic context skills triggered by questions/actions. See [README.md](./README.md) for overview.

---

## `novanet-architecture`

**Trigger:** Questions about architecture, system overview, codebase structure, schema-graph

**Provides:**
- Full architecture ASCII diagram
- v0.13.0 Schema-Graph (faceted classification with Realm/Layer/Class/Trait/ArcFamily)
- Source of Truth structure
- Pipeline sync diagram (12 generators + Rust validation)
- Locale Knowledge structure
- Infrastructure details
- Package dependencies (includes Rust binary)
- Generation pipeline
- Rust binary architecture (`tools/novanet/`)

---

## `novanet-sync`

**Trigger:** YAML changes, sync validation, schema questions

**Provides:**
- Source of Truth documentation (v0.13.0 terminology)
- Generated artifacts mapping (12 generators)
- Validation commands (Rust authoritative)
- CI integration details
- Troubleshooting guide

---

## `novanet-tui`

**Trigger:** TUI launch, keybindings questions, terminal UI navigation

**Provides:**
- Launch command (`cargo run -- tui`)
- Keybindings reference (navigation, Graph/Nexus modes, scrolling, overlays)
- Visual features (Galaxy theme, boot animation, effects engine)
- v11.7 Unified Tree (Realm/Layer/Class/Instance as clickable nodes)
- Troubleshooting guide

| Argument | Description |
|----------|-------------|
| _(empty)_ | Launch TUI |
| `help`, `keys` | Show keybindings reference |
| `features` | Show visual features overview |

**v11.7 Navigation:**
```
[1]Graph   Unified tree with all nodes clickable
[2]Nexus   Hub (Quiz, Audit, Stats, Help)
[/]        Search overlay
```

---

## `security-audit`

**Trigger:** Security checks, dependency audits, vulnerability scanning

**Provides:**
- Rust audit (cargo-deny, cargo-audit, cargo-machete)
- TypeScript audit (pnpm audit, code patterns)
- CI security checks verification
- Exception review and management

| Argument | Description |
|----------|-------------|
| `rust` | Audit Rust dependencies only |
| `typescript` | Audit TypeScript dependencies only |
| `all` | Full audit (default) |
| `exceptions` | List security exceptions |

---

## `codebase-audit` (Ralph Wiggum Loop)

**Trigger:** Before releases, after refactoring, periodic maintenance

**Invocation:** `/codebase-audit [mode] [--fix]`

**Process:**
1. **SCAN** - Launch 10 parallel agents (haiku model)
2. **SYNTHESIZE** - Prioritize findings (CRITICAL → LOW)
3. **FIX** - Apply corrections with tests
4. **VERIFY** - Re-run until clean

**10 Parallel Agents:**
1. YAML Schema Validation
2. Generated Artifacts Sync
3. Rust Code Quality
4. Test Coverage Analysis
5. Documentation Freshness
6. Dependency Audit
7. Performance Patterns
8. Security Patterns
9. Dead Code Detection
10. Semantic Coherence

| Mode | Agents | Description |
|------|--------|-------------|
| `full` | 10 | Complete audit (default) |
| `quick` | 4 | Essential checks only |
| `yaml` | 2 | YAML schema + sync |
| `rust` | 2 | Rust quality + tests |
| `typescript` | 2 | TypeScript + dead code |
| `security` | 2 | Security + deps |
| `docs` | 2 | Documentation accuracy |
| `--fix` | - | Auto-fix issues |

---

## `token-audit`

**Trigger:** Design system consistency checks, gap/spacing verification

**Provides:**
- Gap/spacing token adoption analysis
- Non-tokenized pattern detection
- Design system consistency report

| Argument | Description |
|----------|-------------|
| `gaps` | Audit gap/spacing tokens only |
| `all` | Full token audit |
| `summary` | Quick summary |
