# Entity Race Condition Fix Plan

## Issues Identified

### Issue 1: Race Condition in Entity Loading

**Location**: `app/mod.rs:1908-1914` + `mod.rs` event loop

**Problem Flow**:
```
1. User expands Entity class for first time
2. app/mod.rs sets:
   - pending.entity_categories = true
   - pending.instance = Some("Entity")
3. Event loop (mod.rs) executes BOTH in parallel:
   - load_instances_fast("Entity") → stores in instances["Entity"]
   - load_entity_categories() → stores in entity_categories
4. Result: Instances in WRONG storage (flat, not by category)
5. tree.rs looks for entity_category_instances → empty → shows nothing
```

**Solution**:
- Skip `pending.instance` for Entity when categories need loading
- After categories load, auto-trigger category instances loading

### Issue 2: Logic Error in Dual Storage Check

**Location**: `data.rs:2497-2502` and `data.rs:2579-2584`

**Problem**:
```rust
// Current (WRONG): Checks if ANY category has instances
let has_category_instances = !self.entity_category_instances.is_empty();
```

**Solution**:
```rust
// Use existing helper methods
if class_info.key == "Entity" {
    // Use entity_instances_flat() which handles both storages
} else {
    // Regular class handling
}
```

## Implementation

### Fix 1: app/mod.rs - Skip Entity instance load

```rust
// BEFORE
if class_key == "Entity" && self.tree.entity_categories.is_empty() {
    self.pending.entity_categories = true;
}
self.pending.instance = Some(class_key.to_string());

// AFTER
if class_key == "Entity" {
    if self.tree.entity_categories.is_empty() {
        // First time: load categories only, instances loaded per-category later
        self.pending.entity_categories = true;
        // DON'T set pending.instance - we'll load by category instead
    } else if !self.tree.has_entity_category_instances() {
        // Categories exist but no instances yet: trigger category instance loads
        for cat in &self.tree.entity_categories {
            // Queue first category that needs loading
            if !self.tree.entity_category_instances.contains_key(&cat.key) {
                self.pending.category_instances = Some(cat.key.clone());
                break;
            }
        }
    }
    // Don't use flat instance loading for Entity
} else {
    self.pending.instance = Some(class_key.to_string());
}
```

### Fix 2: mod.rs - Auto-load category instances after categories

```rust
// After categories load successfully, queue first category for instance loading
if app.take_pending_entity_categories_load() {
    match TaxonomyTree::load_entity_categories(db).await {
        Ok(categories) if app.navigation_generation == nav_gen => {
            app.tree.entity_categories = categories.clone();
            // Auto-trigger loading of first category's instances
            if let Some(first_cat) = categories.first() {
                app.pending.category_instances = Some(first_cat.key.clone());
            }
        }
        // ...
    }
}
```

### Fix 3: data.rs - Use helpers in item_count_for_mode

```rust
// Line ~2495-2517: Replace inline Entity check with helper
if class_info.key == "Entity" {
    count += self.entity_instance_count();
} else {
    if let Some(instances) = self.instances.get(&class_info.key) {
        count += instances.len();
    }
}
```

### Fix 4: data.rs - Use helpers in item_at_for_mode

```rust
// Line ~2577-2609: Replace inline Entity check with helper
if class_info.key == "Entity" {
    for instance in self.entity_instances_flat() {
        if idx == cursor {
            return Some(TreeItem::Instance(realm, layer, class_info, instance));
        }
        idx += 1;
    }
} else {
    if let Some(instances) = self.instances.get(&class_info.key) {
        // ... existing code
    }
}
```

## Testing

1. `cargo test` - All tests pass
2. `cargo clippy -- -D warnings` - Zero warnings
3. Manual test:
   - Start TUI with fresh DB
   - Expand Entity class
   - Verify instances appear (not empty)
   - Navigate to instances
   - Verify cursor works correctly
