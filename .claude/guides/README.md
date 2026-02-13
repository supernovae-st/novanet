# NovaNet DX Testing & Nomenclature Guides

Complete documentation for testing terminology, nomenclature consistency, and schema validation in NovaNet.

---

## Quick Start

**New to DX testing?** Start here:

1. **[QUICK-REF-testing-DX.md](./QUICK-REF-testing-DX.md)** — 5-minute overview with templates and checklists
2. **[DX-testing-nomenclature-validation.md](./DX-testing-nomenclature-validation.md)** — Complete patterns and philosophy
3. **[v11.8-migration-testing.md](./v11.8-migration-testing.md)** — Specific testing for v11.8 Class/Instance/Data Origin

---

## The Guides

### 1. QUICK-REF-testing-DX.md

**For**: Developers who need test patterns, now.

**Contains**:
- Core test pattern (3 lines)
- Test checklist: Red → Green → Refactor
- Test templates (copy-paste ready)
- Debugging commands
- Pre-commit hook
- One-minute test that catches 90% of issues

**Best for**: Implementing new nomenclature tests, quick reference during coding

---

### 2. DX-testing-nomenclature-validation.md

**For**: Understanding the philosophy and patterns behind DX testing.

**Contains**:
- Core principles (consistency as a feature, reference documents over rules)
- Testing patterns:
  - Test fixtures with semantic names
  - Semantic validation issues
  - YAML-first validation tests
  - Snapshot testing for generated code
- Complete checklist for v11.8 DX tests
- Real NovaNet examples from `blueprint/validation.rs`
- Best practices from Claude documentation
- Common pitfalls and solutions
- How to run tests (local, pre-commit, CI/CD)
- Migration workflow (Red → Green → Refactor)

**Best for**: Understanding WHY we test terminology, learning patterns, training new developers

---

### 3. v11.8-migration-testing.md

**For**: Testing the v11.8 nomenclature migration (Class/Instance/Data Origin).

**Contains**:
- What changed in v11.8 (tables of old vs new terminology)
- 3 testing phases:
  - Phase 1: Prepare (Red tests)
  - Phase 2: Execute (Green tests)
  - Phase 3: Validate (Refactor tests)
- Step-by-step implementation:
  - Update YAML files
  - Regenerate artifacts
  - Update trait parsing
  - Update type names
  - Test everything
- Specific test code for each phase
- How to run the migration test suite
- Rollback plan
- Post-migration maintenance

**Best for**: Implementing or validating the v11.8 migration, understanding phased testing approach

---

## Key Concepts

### Reference Documents Over Rules

Rather than encode rules in code, establish reference documents and test against them:

```
CLAUDE.md / novanet-terminology.md / ADR files
    ↓ (source of truth)
Test code validates adherence
    ↓
Tests fail if violation exists
```

### Three-Phase Migration Testing

```
🔴 RED    → Define new state (tests fail until migration complete)
🟢 GREEN  → Update code (make tests pass)
🔵 REFACTOR → Validate semantics (comprehensive consistency tests)
```

### Validation Before Execution

Always validate coherence **before** using data:
- Path must match content (realm/layer declared in YAML)
- Arc scope must match actual source/target realms
- Traits must follow naming patterns

### Semantic Validation Issues

Structure validation errors with:
- ✓ Severity (Error, Warning, Info)
- ✓ Category (for filtering/grouping)
- ✓ Human-readable message
- ✓ Actionable fix hint

```rust
ValidationIssue::error("trait_mismatch", "Page: wrong trait")
    .with_hint("Change to: trait: defined")
```

---

## File Structure

```
.claude/guides/
├── README.md (you are here)
├── QUICK-REF-testing-DX.md
├── DX-testing-nomenclature-validation.md
└── v11.8-migration-testing.md
```

---

## Running Tests

### All YAML-coherence tests (no Neo4j)

```bash
cd tools/novanet
cargo test --lib
```

### Specific test category

```bash
cargo test realm          # Realm validation tests
cargo test trait          # Trait validation tests
cargo test naming_pattern # Naming pattern tests
cargo test snapshot       # Snapshot comparison tests
```

### With full output

```bash
cargo test --lib -- --nocapture --test-threads=1
```

### Schema validation

```bash
cargo run -- schema validate        # Quick validation
cargo run -- schema validate --strict  # Fail on warnings
```

---

## v11.8 Nomenclature Changes

### ADR-023: Kind → Class, Meta → Schema

| Old | New |
|-----|-----|
| `NodeKind` | `NodeClass` |
| `ArcKind` | `ArcClass` |
| `:Meta:Kind` | `:Schema:Class` |
| "Meta Node" | "Class" |

### ADR-024: Data Origin Traits

| Old Trait | New Trait | Meaning |
|-----------|-----------|---------|
| `invariant` | **`defined`** | Human-created once |
| `localized` | **`authored`** | Human-written per locale |
| `knowledge` (trait) | **`imported`** | External data |
| `derived` | **`generated`** | LLM-produced |
| `aggregated` | **`retrieved`** | API snapshots |

### ADR-025: Instruction Layer

| Old | New |
|-----|-----|
| `PageType` | `PageStructure` |
| `PagePrompt` | `PageInstruction` |
| `BlockPrompt` | `BlockInstruction` |

---

## Integration with CLAUDE.md

These guides support the NovaNet coding principles:

✓ **Question First** — ADRs explain the WHY (ADR-023, ADR-024, ADR-025)
✓ **Research First** — Guides document existing patterns before coding
✓ **Test Before Code** — TDD workflow (Red → Green → Refactor)
✓ **Verify Before Ship** — Validation tests ensure coherence

---

## Common Workflows

### Adding a New Node Type

1. Create YAML: `packages/core/models/node-kinds/{realm}/{layer}/{name}.yaml`
2. Run tests: `cargo test test_path_content_match` (validates path ↔ content)
3. Regenerate: `cargo run -- schema generate`
4. Verify: `cargo test test_no_orphans` (ensures referenced nodes exist)

### Updating Terminology

1. Create Red tests: `cargo test test_new_term_values` (fails until migration)
2. Update YAML: `sed -i 's/old/new/' packages/core/models/**/*.yaml`
3. Regenerate: `cargo run -- schema generate`
4. Make tests pass: `cargo test test_new_term_values` (now passes)
5. Validate semantics: `cargo test test_trait_layer_alignment`

### Debugging Terminology Issues

```bash
# Find problematic nodes
cargo test naming_pattern -- --nocapture

# Show detailed validation
cargo run -- schema validate --strict

# Check distribution
grep -r "trait:" packages/core/models/ | cut -d: -f3 | sort | uniq -c

# Compare old vs new
grep -r "invariant" packages/core/models/ # Should be empty
grep -r "defined" packages/core/models/ | wc -l  # Should be ~31
```

---

## Philosophy

**Terminology is not just style—it's a testable property of system coherence.**

From the Claude documentation on Agent Skills:
> "Consistency helps Claude understand and follow instructions."

The same principle applies to codebases:
- Consistent terminology reduces cognitive load
- Tests validate consistency can't drift
- Migrations are safer with phased testing
- Errors guide developers to correct terms

---

## Contributing to These Guides

**Before updating a guide, ensure**:

1. ✓ Tests document the change
2. ✓ CLAUDE.md reflects new principles
3. ✓ ADRs explain the decision
4. ✓ Examples use correct terminology
5. ✓ Tools run successfully

Example PR workflow:
```
Commit 1: Update guide + add tests
Commit 2: Run all tests (verified ✓)
Commit 3: Update ADR if decision changed
```

---

## Quick Links

| Concept | Location |
|---------|----------|
| Trait definitions | `.claude/rules/novanet-terminology.md` → "YAML Source Files" |
| Node structure | `.claude/rules/novanet-decisions.md` → ADR-024 |
| Testing patterns | `DX-testing-nomenclature-validation.md` → "Testing Patterns" |
| v11.8 migration | `v11.8-migration-testing.md` → "Testing Phases" |
| Test templates | `QUICK-REF-testing-DX.md` → "Test Templates" |

---

## Version Information

- **Current NovaNet Version**: v11.8
- **Latest ADR**: ADR-025 (Instruction Layer)
- **Latest Traits**: defined, authored, imported, generated, retrieved
- **Realms**: shared (4 layers), org (6 layers)
- **Total Nodes**: 60 (39 shared + 21 org)

---

## Support

- Questions about tests? → See `DX-testing-nomenclature-validation.md`
- Need a test template? → See `QUICK-REF-testing-DX.md`
- Implementing v11.8? → See `v11.8-migration-testing.md`
- Terminology definitions? → See `.claude/rules/novanet-terminology.md`
- Architecture decisions? → See `.claude/rules/novanet-decisions.md`
