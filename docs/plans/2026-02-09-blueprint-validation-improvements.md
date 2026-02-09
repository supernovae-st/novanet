# Blueprint Validation Improvements Plan

**Date**: 2026-02-09
**Status**: In Progress

## Objective

Enhance blueprint validation to catch more schema issues before they reach production.

## New Validation Checks

### Phase 1: Arc Scope Completeness (5 min)

Add INFO-level warning when arcs don't declare a scope.

```rust
// In check_arc_scope_coherence()
if arc.scope.is_none() {
    self.issues.push(ValidationIssue::info(
        "arc_scope",
        format!("{}: no scope declared", arc.arc_type)
    ).with_hint("Add 'scope: intra_realm' or 'scope: cross_realm' to arc-kind YAML"));
}
```

### Phase 2: Orphan Node Detection (10 min)

Detect node types that are defined but never used in any arc (neither source nor target).

```rust
fn check_orphan_nodes(&mut self, data: &BlueprintData) {
    let all_nodes: HashSet<&str> = data.node_kinds.iter()
        .map(|n| n.def.name.as_str())
        .collect();

    let used_in_arcs: HashSet<&str> = data.arc_defs.iter()
        .flat_map(|a| a.source.labels().into_iter().chain(a.target.labels()))
        .collect();

    let orphans: Vec<&str> = all_nodes.difference(&used_in_arcs).copied().collect();
    // Report as warnings
}
```

### Phase 3: Duplicate Arc Type Detection (5 min)

Catch duplicate arc type names (could happen if same arc defined in multiple files).

```rust
fn check_duplicate_arcs(&mut self, data: &BlueprintData) {
    let mut seen: HashMap<&str, usize> = HashMap::new();
    for arc in &data.arc_defs {
        *seen.entry(arc.arc_type.as_str()).or_insert(0) += 1;
    }
    // Report duplicates as errors
}
```

### Phase 4: Temperature Threshold Validation (5 min)

Validate that temperature_threshold values are in valid range [0.0, 1.0].

Note: This requires loading temperature data separately since it's not in ArcDef.

### Phase 5: Unit Tests for Scope Validation (10 min)

Add unit tests covering:
1. Valid intra_realm arc
2. Valid cross_realm arc
3. Invalid scope mismatch
4. Unknown scope value
5. Multiple source/target with mixed realms

## Success Criteria

- [ ] All new validations implemented
- [ ] Unit tests pass
- [ ] `cargo test blueprint::` green
- [ ] `cargo clippy` zero warnings

## Files Modified

- `src/blueprint/validation.rs` - Add new validation checks
- `src/blueprint/validation.rs` - Add unit tests
