# NovaNet DX & Testing Guides - Complete Index

**Comprehensive documentation for testing terminology, nomenclature validation, and schema coherence in NovaNet.**

---

## Document Library

### 📖 Guides (In This Directory)

| Document | Purpose | Length | Best For |
|----------|---------|--------|----------|
| **README.md** | Overview & navigation | 10 min | Getting oriented |
| **QUICK-REF-testing-DX.md** | Test patterns, templates, checklists | 5 min | Quick reference while coding |
| **DX-testing-nomenclature-validation.md** | Complete philosophy & patterns | 20 min | Understanding & learning |
| **v11.8-migration-testing.md** | Phased testing for v11.8 | 30 min | Implementing the migration |
| **v11.7-implementation-guide.md** | v11.7 TUI architecture | 20 min | Understanding unified tree |

### 📋 Reference Documents (In `.claude/rules/`)

| Document | Content | Type |
|----------|---------|------|
| **novanet-terminology.md** | Canonical terminology definitions | Source of truth |
| **novanet-decisions.md** | All ADRs (001-025), including ADR-023/024 | Architecture decisions |

### 🔧 Configuration Files

| File | Location | Purpose |
|------|----------|---------|
| **CLAUDE.md** (project) | `./CLAUDE.md` | Project conventions & principles |
| **CLAUDE.md** (tools) | `./tools/novanet/CLAUDE.md` | Rust binary conventions |

---

## Reading Paths

### Path 1: "I need test patterns right now"

```
1. QUICK-REF-testing-DX.md (5 min)
   ↓ (copy test template from here)
2. DX-testing-nomenclature-validation.md § "Testing Patterns" (10 min)
   ↓ (understand why pattern works)
3. Start coding tests
```

### Path 2: "I want to understand terminology testing"

```
1. README.md → "Key Concepts" (5 min)
2. DX-testing-nomenclature-validation.md § "Core Principles" (10 min)
3. novanet-terminology.md (10 min)
4. DX-testing-nomenclature-validation.md § "Testing Patterns" (15 min)
5. novanet-decisions.md → ADR-023, ADR-024 (20 min)
```

### Path 3: "I'm implementing the v11.8 migration"

```
1. novanet-terminology.md → "Deprecated Terms" (5 min)
2. novanet-decisions.md → ADR-023, ADR-024, ADR-025 (15 min)
3. v11.8-migration-testing.md → "What Changed" (5 min)
4. v11.8-migration-testing.md → "Testing Phases" (30 min)
5. QUICK-REF-testing-DX.md → "Migration Workflow" (5 min)
6. Run tests + implement
```

### Path 4: "I'm new to NovaNet"

```
1. README.md § "The Guides" (10 min)
2. novanet-terminology.md (20 min)
3. DX-testing-nomenclature-validation.md § "Core Principles" (15 min)
4. novanet-decisions.md → ADR-012, ADR-013, ADR-018, ADR-023, ADR-024 (30 min)
5. v11.7-implementation-guide.md (20 min)
```

---

## Quick Navigation

### By Topic

**Traits & Data Origin**
- Define: novanet-terminology.md § "Node Naming Convention"
- Rationale: novanet-decisions.md § ADR-024
- Testing: QUICK-REF-testing-DX.md § "Test Template: Nomenclature Validation"

**Classes (formerly Kinds)**
- Define: novanet-terminology.md § "Core Vocabulary"
- Rationale: novanet-decisions.md § ADR-023
- Testing: v11.8-migration-testing.md § "Test 2.1: New Traits Are Valid"

**Realms & Layers**
- Define: novanet-terminology.md § "v11.5 Realm Architecture"
- Rationale: novanet-decisions.md § ADR-012, ADR-018, ADR-019, ADR-020
- Testing: QUICK-REF-testing-DX.md § "Test Template: Realm Values"

**Page Structures & Instructions**
- Define: novanet-terminology.md § "Deprecated Terms" (PageType → PageStructure)
- Rationale: novanet-decisions.md § ADR-025
- Testing: v11.8-migration-testing.md § "Step 4: Update Type Names"

**Arcs & Relationships**
- Define: novanet-terminology.md § "Arc Classification"
- Rationale: novanet-decisions.md § ADR-001, ADR-015, ADR-016
- Testing: DX-testing-nomenclature-validation.md § "Pattern: YAML-First Validation"

### By Problem

**"Tests are failing, I don't know why"**
→ QUICK-REF-testing-DX.md § "Common Failure Patterns"

**"How do I test terminology consistency?"**
→ DX-testing-nomenclature-validation.md § "Testing Patterns for Terminology"

**"What should I name this node?"**
→ novanet-terminology.md § "Node Naming Convention (v11.5)"

**"Old terminology keeps sneaking in"**
→ v11.8-migration-testing.md § "Phase 1: Prepare (Red)"

**"I need to implement the v11.8 migration"**
→ v11.8-migration-testing.md (entire document)

**"How do I validate YAML coherence?"**
→ DX-testing-nomenclature-validation.md § "Pattern 3: YAML-First Validation Tests"

---

## Core Testing Concepts

### The Three Types of Tests

```
1. YAML Coherence (Fast, no Neo4j)
   - Path matches realm/layer
   - Required fields present
   - Trait values valid
   - No orphan references
   → Run: cargo test --lib

2. Semantic Correctness (Fast, logical)
   - Trait aligns with layer
   - Naming patterns match trait
   - Arc scope matches realms
   → Run: cargo test trait_layer_alignment

3. Output Consistency (Slow, regeneration)
   - Generated code uses new terms
   - CLI output uses new terminology
   - TUI displays correctly
   - Neo4j schema matches YAML
   → Run: cargo test snapshot
```

### The Three-Phase Migration

```
🔴 RED (Tests fail until code changes)
   - Define new values
   - Define old values removed

🟢 GREEN (Update code, make tests pass)
   - Update YAML
   - Regenerate artifacts
   - Update type system

🔵 REFACTOR (Comprehensive validation)
   - Semantic correctness
   - Naming patterns
   - Documentation
   - Output consistency
```

---

## Key Files & Locations

```
novanet-hq/
├── .claude/
│   ├── guides/                           ← YOU ARE HERE
│   │   ├── README.md                     ← Start here
│   │   ├── QUICK-REF-testing-DX.md       ← Quick templates
│   │   ├── DX-testing-nomenclature-validation.md  ← Full patterns
│   │   ├── v11.8-migration-testing.md    ← v11.8 specific
│   │   └── v11.7-implementation-guide.md ← TUI architecture
│   ├── rules/
│   │   ├── novanet-terminology.md        ← Terminology definitions
│   │   ├── novanet-decisions.md          ← All ADRs (001-025)
│   │   └── security.md                   ← Security patterns
│   └── hooks/                            ← Pre-commit hooks
├── tools/novanet/src/
│   ├── blueprint/validation.rs           ← Real validation patterns
│   ├── generators/test_utils.rs          ← Test fixtures
│   └── parsers/yaml_node.rs              ← Trait enum
├── packages/core/models/
│   ├── node-kinds/                       ← 60 YAML definitions
│   ├── arc-kinds/                        ← Arc definitions
│   ├── taxonomy.yaml                     ← Realm/layer definitions
│   └── visual-encoding.yaml              ← Icons & visual encoding
└── packages/core/src/graph/
    └── visual-encoding.ts                ← Generated TypeScript
```

---

## Checklist: First Time Using These Guides?

- [ ] Read `README.md` (understanding of available guides)
- [ ] Choose your reading path (above)
- [ ] Skim `novanet-terminology.md` (15 min context)
- [ ] Copy a test template from `QUICK-REF-testing-DX.md`
- [ ] Read relevant section in `DX-testing-nomenclature-validation.md`
- [ ] Write your first test
- [ ] Run: `cargo test --lib`
- [ ] Share results with team

---

## File Statistics

| Document | Words | Sections | Code Examples |
|----------|-------|----------|----------------|
| README.md | ~2,200 | 15 | 5 |
| QUICK-REF-testing-DX.md | ~2,100 | 12 | 20 |
| DX-testing-nomenclature-validation.md | ~6,700 | 20 | 30 |
| v11.8-migration-testing.md | ~7,200 | 20 | 35 |
| **Total guides** | **~18,200** | **67** | **90** |

---

## Update Schedule

| Document | Last Updated | Frequency | Why |
|----------|--------------|-----------|-----|
| novanet-terminology.md | 2026-02-13 | Per ADR | Terminology source of truth |
| novanet-decisions.md | 2026-02-13 | Per ADR | Architecture decisions |
| DX-testing-nomenclature-validation.md | 2026-02-13 | Per release | Testing best practices |
| v11.8-migration-testing.md | 2026-02-13 | Once (v11.8) | Migration specific |
| QUICK-REF-testing-DX.md | 2026-02-13 | Per release | Keeps templates current |

---

## Key Principles Documented

### 1. Consistency as a Feature
- Terminology is testable
- Tests validate coherence can't drift
- See: DX-testing-nomenclature-validation.md § "Core Principles"

### 2. Reference Documents Over Rules
- CLAUDE.md, ADRs, terminology guide are source of truth
- Tests verify adherence
- See: DX-testing-nomenclature-validation.md § "Pattern 1"

### 3. Validation Before Execution
- Validate YAML coherence first (no Neo4j)
- Check Neo4j sync second (optional)
- See: blueprint/validation.rs (real code)

### 4. Semantic Validation Issues
- Include severity, category, message, fix hint
- Help developers self-serve fixes
- See: QUICK-REF-testing-DX.md § "Validation Issue Pattern"

### 5. Three-Phase Testing (Red-Green-Refactor)
- Phase 1: Define expected state (Red)
- Phase 2: Update code (Green)
- Phase 3: Validate comprehensively (Refactor)
- See: v11.8-migration-testing.md § "Testing Phases"

---

## Connection to Claude Documentation

These guides build on patterns from Claude's Agent Skills Best Practices:

**From official Claude docs**:
> "Consistency helps Claude understand and follow instructions."

**Applied to NovaNet**:
- Terminology consistency reduces cognitive load
- Tests enforce consistency programmatically
- Reference documents provide clarity
- Validation loops catch errors early

**Sources**:
- Claude Agent Skills § Best Practices
- Claude Test & Evaluate § Develop Tests
- Claude Prompt Engineering § System Prompts

---

## Getting Help

**For specific questions**:

| Question | Document | Section |
|----------|----------|---------|
| "How do I write a test?" | QUICK-REF-testing-DX.md | Test Template |
| "What's the philosophy?" | DX-testing-nomenclature-validation.md | Core Principles |
| "What's the right term?" | novanet-terminology.md | Terminology |
| "Why did we decide X?" | novanet-decisions.md | ADR-023/024/025 |
| "How do I run tests?" | QUICK-REF-testing-DX.md | Running Tests |
| "v11.8 migration steps?" | v11.8-migration-testing.md | Testing Phases |

**For broader questions**:
- Post in team chat with link to relevant section
- Include specific question + document path
- Example: "Reading DX-testing-nomenclature-validation.md § Pattern 1, question about..."

---

## Contributing Updates

When updating any guide:

1. ✓ Run tests to verify changes are valid
2. ✓ Update novanet-terminology.md if terminology changed
3. ✓ Update novanet-decisions.md if decision changed (create new ADR)
4. ✓ Update version information in README.md
5. ✓ Create PR with all updated docs together

Example commit message:
```
docs(guides): Update v11.8 testing patterns and add ADR-025

- Add PageStructure/PageInstruction test patterns
- Document ADR-025 instruction layer renames
- Update migration phase examples
- Add rollback procedure

Fixes #123
```

---

## Version Lock

**Current NovaNet Version**: v11.8
- **Realms**: shared (4 layers), org (6 layers) = 10 total
- **Nodes**: 59 total (39 shared + 20 org)
- **Traits**: defined, authored, imported, generated, retrieved
- **Latest ADR**: ADR-025 (Instruction Layer Renaming)

These documents are current through v11.8.0 (Class Act).

For v11.9+, update:
- `README.md` § "Version Information"
- `v11.8-migration-testing.md` → create `v11.9-migration-testing.md`
- `novanet-terminology.md` § "Deprecated Terms"

---

## Document Map (Text-Based)

```
START HERE
    ↓
README.md
    ├→ New to NovaNet?
    │  └→ novanet-terminology.md
    │
    ├→ Need test patterns?
    │  └→ QUICK-REF-testing-DX.md
    │
    ├→ Want to understand?
    │  └→ DX-testing-nomenclature-validation.md
    │
    └→ Implementing v11.8?
       ├→ novanet-decisions.md (ADR-023/024/025)
       └→ v11.8-migration-testing.md

TOOLS & EXAMPLES
    ├→ tools/novanet/src/blueprint/validation.rs
    ├→ tools/novanet/src/generators/test_utils.rs
    └→ .github/workflows/ (CI/CD examples)
```

---

**Last Updated**: 2026-02-13
**Current Version**: v11.8.0 (Class Act)
**Total Documentation**: 18,200+ words, 90+ code examples, 67 sections

Start with README.md and choose your reading path above. Happy testing!
