# TUI Improvements Plan v2

**Date**: 2026-02-10
**Version**: v11.6.0
**Status**: Ready for Implementation

---

## Executive Summary

This plan covers 4 major TUI improvements identified through comprehensive codebase exploration:

| ID | Improvement | Effort | Impact | Sprint |
|----|-------------|--------|--------|--------|
| **A** | Key Handler Refactor | 6h | High (testability) | 1 |
| **B** | Atlas Mode Activation | 1.5h | Medium (educational) | 1 |
| **C** | Nexus Quiz Mode | 6h | High (gamification) | 2 |
| **D** | Render Cache System | 4h | Medium (performance) | 2 |

**Total Effort**: ~17.5 hours (2-3 sprints)

---

## A. Key Handler Refactor

### Problem

The `handle_key()` function in `app.rs` is **596 lines** with mixed concerns across 4 navigation modes (Meta, Data, Audit, Nexus). This creates:

- Poor testability (can't test mode-specific logic in isolation)
- Slow feature velocity (every change touches the giant function)
- High cognitive load (39 App fields, 100+ match branches)

### Solution: Trait-Based Mode Dispatch

```rust
pub trait ModeHandler {
    fn handle_key(&mut self, app: &mut App, key: KeyEvent) -> bool;
    fn mode(&self) -> NavMode;
}
```

### Implementation Steps

#### Phase 1: Foundation (1-2h)

1. **Create trait definition**
   - File: `src/tui/handlers/mod.rs`
   ```rust
   pub trait ModeHandler {
       fn handle_key(&mut self, app: &mut App, key: KeyEvent) -> bool;
       fn mode(&self) -> NavMode;
   }
   ```

2. **Create dispatcher**
   ```rust
   pub fn dispatch_mode_handler(app: &mut App, key: KeyEvent) -> bool {
       match app.mode {
           NavMode::Meta => MetaModeHandler.handle_key(app, key),
           NavMode::Data => DataModeHandler.handle_key(app, key),
           NavMode::Audit => AuditModeHandler.handle_key(app, key),
           NavMode::Nexus => NexusModeHandler.handle_key(app, key),
       }
   }
   ```

#### Phase 2: Extract Handlers (2-3h)

| Mode | Handler File | Lines | Complexity |
|------|--------------|-------|------------|
| Audit | `handlers/audit.rs` | ~50 | Low |
| Data | `handlers/data.rs` | ~60 | Medium |
| Meta | `handlers/meta.rs` | ~150 | Medium-High |
| Nexus | `handlers/nexus.rs` | ~30 | Low (wrapper) |

**Keybindings by mode**:

| Key | Meta | Data | Audit | Nexus |
|-----|------|------|-------|-------|
| `0` | - | Toggle hide empty | - | - |
| `s` | - | Schema overlay | - | - |
| `+/-` | - | Property focus | - | - |
| `r` | Refresh | Refresh | Refresh audit | - |
| `j/k` | Navigate | Navigate | Audit cursor | Delegate |

#### Phase 3: Integration (1-2h)

3. **Refactor `App::handle_key()`**
   - Keep: Overlay handling (help, legend, search, filter)
   - Keep: Global shortcuts (1-4 mode switch, Tab focus)
   - Replace: Mode-specific logic → dispatch to handlers
   - Target: 596 lines → ~200 lines (66% reduction)

#### Phase 4: Testing (1h)

4. **Write handler tests**
   - File: `tests/tui/handlers.rs`
   - Test each handler in isolation
   - Verify state mutations

### File Changes

| File | Change | Lines |
|------|--------|-------|
| `src/tui/handlers/mod.rs` | NEW | ~100 |
| `src/tui/handlers/audit.rs` | NEW | ~50 |
| `src/tui/handlers/meta.rs` | NEW | ~150 |
| `src/tui/handlers/data.rs` | NEW | ~60 |
| `src/tui/handlers/nexus.rs` | NEW | ~30 |
| `src/tui/app.rs` | REFACTOR | 596→200 |
| `src/tui/mod.rs` | UPDATE | +2 |

### Verification

```bash
cargo test --lib
cargo clippy -- -D warnings
# Manual: verify all keybindings in TUI
```

---

## B. Atlas Mode Activation

### Problem

Atlas mode is **100% implemented** (2,369 lines, 148 tests) but **0% connected** to the TUI:

- No `NavMode::Atlas` variant
- No render dispatch in `render_main()`
- No keyboard shortcut (5 key)

### Status Assessment

| View | Status | Lines | Tests |
|------|--------|-------|-------|
| RealmMap | ✅ Complete | 229 | 10 |
| SpreadingActivation | ✅ Complete | 299 | 17 |
| KnowledgeAtoms | ✅ Complete | 235 | 7 |
| GenerationPipeline | ✅ Complete | 292 | 3 |
| ViewTraversal | ✅ Complete | 292 | 3 |
| PageComposition | ⚠️ Partial | 205 | 4 |

### Implementation Steps (90 minutes)

1. **Add NavMode::Atlas** (5 min)
   - File: `src/tui/app.rs`
   ```rust
   pub enum NavMode {
       Meta,
       Data,
       Audit,
       Nexus,
       Atlas,  // NEW
   }
   ```

2. **Wire render dispatch** (5 min)
   - File: `src/tui/ui/mod.rs`
   ```rust
   match app.mode {
       NavMode::Atlas => render_atlas(f, app, main_area),
       // ... existing cases
   }
   ```

3. **Add keyboard shortcut** (5 min)
   - File: `src/tui/app.rs` (handle_key)
   ```rust
   KeyCode::Char('5') => {
       self.mode = NavMode::Atlas;
       true
   }
   ```

4. **Complete PageComposition data loading** (45 min)
   - Implement Neo4j query for page data
   - Wire async loading pattern (like other modes)

5. **Integration test** (20 min)
   - Test mode switching
   - Verify all 6 views render

### Verification

```bash
cargo run -- tui
# Press 5 → Atlas mode visible
# Navigate all 6 views
```

---

## C. Nexus Quiz Mode

### Problem

Nexus mode (4 tabs) lacks interactivity:
- Static "Did you know?" tips
- No knowledge verification
- No gamification/engagement

### Solution: Quiz Tab (#5)

Add interactive quiz with 15-20 questions covering:
- Traits (5 questions)
- Layers (5 questions)
- Arcs (5 questions)
- Pipeline (3 questions)
- Mixed (2 questions)

### Quiz State Structure

```rust
pub struct QuizState {
    pub current_idx: usize,
    pub total: usize,
    pub correct: usize,
    pub streak: usize,
    pub max_streak: usize,
    pub selected_option: usize,
    pub submitted: bool,
    pub category: QuizCategory,
}

pub struct QuizQuestion {
    pub id: u32,
    pub category: QuizCategory,
    pub text: &'static str,
    pub options: &'static [&'static str],
    pub correct: usize,
    pub explanation: &'static str,
    pub difficulty: u8,  // 1-5 stars
}
```

### Sample Questions

| # | Category | Question | Difficulty |
|---|----------|----------|------------|
| 1 | Traits | What is an INVARIANT node? | ★ |
| 2 | Traits | Which trait is LLM output? | ★ |
| 3 | Layers | How many SHARED layers? (4) | ★ |
| 4 | Layers | What does OUTPUT layer hold? | ★★ |
| 5 | Arcs | What is OWNERSHIP family? | ★ |
| 6 | Arcs | Arc scope options? | ★★ |
| 7 | Pipeline | Core principle? (Generation NOT Translation) | ★ |

### UI Layout

```
┌─────────────────────────────────────────────────────────────┐
│ [1]■ Traits [2]▣ Layers [3]⇄ Arcs [4]⚡ Pipeline [5]❓ Quiz │
├─────────────────────────────────────────────────────────────┤
│                      QUESTION 2/15                          │
│  What is an INVARIANT node?         [⭐ difficulty: 1/5]   │
│                                                             │
│  ┌────────────────────────────────────────────────────┐    │
│  │ (A) A node that changes with every locale         │    │
│  │ (B) A structural node that stays the same    ← ✓  │    │
│  │ (C) A node with generated LLM output              │    │
│  │ (D) A node that aggregates metrics                │    │
│  └────────────────────────────────────────────────────┘    │
│                                                             │
│  [j/k] Navigate  [Enter] Submit  [Esc] Skip                │
├─────────────────────────────────────────────────────────────┤
│ Score: 8/10  Streak: 3 ⚡  Time: 2:34                       │
└─────────────────────────────────────────────────────────────┘
```

### Implementation Steps

| Task | LOC | Time |
|------|-----|------|
| Quiz state struct + enums | 60 | 0.5h |
| 15 hardcoded questions | 180 | 1h |
| Quiz rendering | 300 | 1.5h |
| Quiz logic (nav, submit) | 150 | 1h |
| Key handler routing | 50 | 0.5h |
| Tab integration | 30 | 0.25h |
| Tests | 100 | 1h |
| **Total** | ~770 | **6h** |

### File Changes

| File | Change |
|------|--------|
| `src/tui/nexus/quiz.rs` | NEW (~600 LOC) |
| `src/tui/nexus/mod.rs` | Add Quiz tab |
| `src/tui/nexus/mod.rs` | Route quiz keys |

---

## D. Render Cache System

### Problem

Current rendering allocates **32.5KB/frame** at 60fps = **1.95MB/sec**:

| Component | Cost/Frame | Frequency |
|-----------|------------|-----------|
| Tree | 25KB | Every navigation |
| Info | 5KB | Every selection |
| Status | 0.5KB | **Every frame** |
| YAML | 2KB | Every selection |

### Solution: RenderCache<T>

```rust
pub struct RenderCache<T: Clone> {
    cached: Option<T>,
    change_key: u64,
}

impl<T: Clone> RenderCache<T> {
    pub fn render_if_changed<F>(&mut self, key: u64, f: F) -> T
    where F: FnOnce() -> T
    {
        if self.change_key != key {
            let value = f();
            self.cached = Some(value.clone());
            self.change_key = key;
            value
        } else {
            self.cached.clone().unwrap_or_else(f)
        }
    }
}
```

### Quick Wins (2h)

1. **Status bar realm mini-bar cache** (30 min)
   - File: `ui/status.rs:118-180`
   - Detection: `hash(realms.len(), colors)`
   - Saves: 0.5KB/frame

2. **Static tree separators** (15 min)
   - File: `ui/tree.rs:1638-1648`
   - Move `"─".repeat(width)` to constant
   - Saves: 1-2KB/frame

3. **Info panel Kind header cache** (45 min)
   - File: `ui/info.rs:786-828`
   - Cache header section separately
   - Saves: 2KB/selection

### Deeper Optimizations (2h)

4. **Tree two-level cache** (1h)
   - Level 1: Logical structure (order)
   - Level 2: Rendered spans (styling)
   - Saves: 8-15KB/frame on navigation

5. **YAML syntax highlight cache** (30 min)
   - Cache line→highlights mapping
   - Saves: 0.5KB/frame

### Expected Impact

| Metric | Before | After | Improvement |
|--------|--------|-------|-------------|
| Alloc/frame | 32.5KB | 6.6KB | 5× |
| Alloc/sec @60fps | 1.95MB | 396KB | 5× |

---

## Sprint Plan

### Sprint 1 (Week 1): Stabilization

| Day | Task | Hours |
|-----|------|-------|
| Mon | A: Key Handler trait + Audit handler | 2h |
| Tue | A: Data + Meta handlers | 3h |
| Wed | A: Nexus wrapper + App refactor | 2h |
| Thu | B: Atlas mode activation | 1.5h |
| Fri | Testing + cleanup | 2h |

**Deliverables**:
- Mode-specific key handlers extracted
- Atlas mode accessible via key 5
- All 958 tests passing

### Sprint 2 (Week 2): Enhancement

| Day | Task | Hours |
|-----|------|-------|
| Mon | C: Quiz state + 15 questions | 2h |
| Tue | C: Quiz rendering | 2h |
| Wed | C: Quiz logic + integration | 2h |
| Thu | D: RenderCache + quick wins | 2h |
| Fri | D: Deeper optimizations + testing | 2h |

**Deliverables**:
- Nexus Quiz mode with 15 questions
- RenderCache system with 5× performance improvement
- All tests passing

---

## Verification Checklist

### A. Key Handler Refactor
- [ ] Create `handlers/mod.rs` with ModeHandler trait
- [ ] Implement AuditModeHandler
- [ ] Implement DataModeHandler
- [ ] Implement MetaModeHandler
- [ ] Implement NexusModeHandler
- [ ] Refactor App::handle_key() to dispatch
- [ ] Add handler tests
- [ ] Verify all keybindings manually
- [ ] `cargo clippy` clean

### B. Atlas Mode
- [ ] Add NavMode::Atlas enum variant
- [ ] Wire render dispatch in render_main()
- [ ] Add key '5' shortcut
- [ ] Complete PageComposition data loading
- [ ] Test all 6 atlas views

### C. Nexus Quiz
- [ ] Create quiz.rs with QuizState
- [ ] Add 15 hardcoded questions
- [ ] Implement quiz rendering
- [ ] Implement quiz logic (nav, submit, score)
- [ ] Add Quiz tab to NexusTab enum
- [ ] Route quiz keys
- [ ] Add quiz tests

### D. Render Cache
- [ ] Implement RenderCache<T>
- [ ] Cache status bar realm mini-bar
- [ ] Extract static tree separators
- [ ] Cache info panel Kind header
- [ ] Implement tree two-level cache
- [ ] Benchmark before/after

---

## Risk Assessment

| Risk | Mitigation |
|------|------------|
| Key handler refactor breaks keybindings | Comprehensive manual testing |
| Atlas mode integration issues | Code is well-tested (148 tests) |
| Quiz questions too easy/hard | Start with 15, adjust difficulty |
| Cache invalidation bugs | Hash-based change detection |

---

## Success Metrics

| Metric | Current | Target |
|--------|---------|--------|
| handle_key() lines | 596 | <200 |
| Navigation modes | 4 | 5 (Atlas) |
| Nexus engagement | Passive | Interactive quiz |
| Render alloc/frame | 32.5KB | <10KB |
| Test coverage | 958 tests | 1000+ tests |

---

## References

- `src/tui/app.rs` - Main state machine (3,508 lines)
- `src/tui/ui/atlas.rs` - Atlas views (2,369 lines, 148 tests)
- `src/tui/nexus/mod.rs` - Nexus state (1,923 lines)
- `KEYBINDINGS.md` - Complete key reference
