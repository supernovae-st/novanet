//! Taxonomy tree: Realm > Layer > Kind hierarchy.
//!
//! Builds a collapsible tree from meta-graph data (Kind nodes with
//! IN_REALM and IN_LAYER relationships). Supports keyboard navigation
//! (up/down/left/right) and expand/collapse.

/// A node in the taxonomy tree.
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub key: String,
    pub display_name: String,
    pub node_type: TreeNodeType,
    pub expanded: bool,
    pub children: Vec<TreeNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeNodeType {
    Realm,
    Layer,
    Kind,
}

impl TreeNode {
    pub fn new(
        key: impl Into<String>,
        display_name: impl Into<String>,
        node_type: TreeNodeType,
    ) -> Self {
        TreeNode {
            key: key.into(),
            display_name: display_name.into(),
            node_type,
            expanded: true,
            children: Vec::new(),
        }
    }

    /// Count total visible items (self + expanded children recursively).
    pub fn visible_count(&self) -> usize {
        if self.expanded {
            1 + self
                .children
                .iter()
                .map(|c| c.visible_count())
                .sum::<usize>()
        } else {
            1
        }
    }
}

/// The full taxonomy tree with navigation state.
#[derive(Debug, Clone)]
pub struct TaxonomyTree {
    pub roots: Vec<TreeNode>,
    pub cursor: usize,
}

impl TaxonomyTree {
    pub fn new(roots: Vec<TreeNode>) -> Self {
        TaxonomyTree { roots, cursor: 0 }
    }

    /// Build from flat meta-graph data (Realm/Layer/Kind rows).
    pub fn from_meta_rows(rows: &[MetaRow]) -> Self {
        let mut realms: Vec<TreeNode> = Vec::new();

        // Group by realm
        for row in rows.iter().filter(|r| r.label == "Realm") {
            realms.push(TreeNode::new(
                &row.key,
                &row.display_name,
                TreeNodeType::Realm,
            ));
        }

        // Add layers under their realms
        for row in rows.iter().filter(|r| r.label == "Layer") {
            if let Some(realm_key) = &row.parent_key {
                if let Some(realm) = realms.iter_mut().find(|r| r.key == *realm_key) {
                    realm.children.push(TreeNode::new(
                        &row.key,
                        &row.display_name,
                        TreeNodeType::Layer,
                    ));
                }
            }
        }

        // Add kinds under their layers
        for row in rows.iter().filter(|r| r.label == "Kind") {
            if let Some(layer_key) = &row.parent_key {
                for realm in &mut realms {
                    if let Some(layer) = realm.children.iter_mut().find(|l| l.key == *layer_key) {
                        layer.children.push(TreeNode::new(
                            &row.key,
                            &row.display_name,
                            TreeNodeType::Kind,
                        ));
                    }
                }
            }
        }

        TaxonomyTree::new(realms)
    }

    /// Total number of items in the tree (all levels).
    pub fn item_count(&self) -> usize {
        fn count(nodes: &[TreeNode]) -> usize {
            nodes.iter().map(|n| 1 + count(&n.children)).sum()
        }
        count(&self.roots)
    }

    /// Total visible items (respecting collapse state).
    pub fn visible_count(&self) -> usize {
        self.roots.iter().map(|r| r.visible_count()).sum()
    }

    /// Get the currently selected node (by cursor position in visible list).
    pub fn selected(&self) -> Option<&TreeNode> {
        let mut idx = 0;
        for root in &self.roots {
            if let Some(node) = find_at_index(root, self.cursor, &mut idx) {
                return Some(node);
            }
        }
        None
    }

    /// Move cursor down.
    pub fn cursor_down(&mut self) {
        let max = self.visible_count();
        if max > 0 && self.cursor < max - 1 {
            self.cursor += 1;
        }
    }

    /// Move cursor up.
    pub fn cursor_up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// Toggle expand/collapse on current node.
    pub fn toggle_expand(&mut self) {
        let cursor = self.cursor;
        let mut idx = 0;
        for root in &mut self.roots {
            if toggle_at_index(root, cursor, &mut idx) {
                return;
            }
        }
    }

    /// Collect visible items as flat list with (depth, node_ref) for rendering.
    pub fn visible_items(&self) -> Vec<(usize, &TreeNode)> {
        let mut items = Vec::new();
        for root in &self.roots {
            collect_visible(root, 0, &mut items);
        }
        items
    }
}

/// A row from the meta-graph query used to build the tree.
#[derive(Debug, Clone)]
pub struct MetaRow {
    pub label: String,
    pub key: String,
    pub display_name: String,
    pub parent_key: Option<String>,
}

// --- Internal helpers ---

fn find_at_index<'a>(node: &'a TreeNode, target: usize, idx: &mut usize) -> Option<&'a TreeNode> {
    if *idx == target {
        return Some(node);
    }
    *idx += 1;
    if node.expanded {
        for child in &node.children {
            if let Some(found) = find_at_index(child, target, idx) {
                return Some(found);
            }
        }
    }
    None
}

fn toggle_at_index(node: &mut TreeNode, target: usize, idx: &mut usize) -> bool {
    if *idx == target {
        node.expanded = !node.expanded;
        return true;
    }
    *idx += 1;
    if node.expanded {
        for child in &mut node.children {
            if toggle_at_index(child, target, idx) {
                return true;
            }
        }
    }
    false
}

fn collect_visible<'a>(node: &'a TreeNode, depth: usize, items: &mut Vec<(usize, &'a TreeNode)>) {
    items.push((depth, node));
    if node.expanded {
        for child in &node.children {
            collect_visible(child, depth + 1, items);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tree() -> TaxonomyTree {
        let rows = vec![
            MetaRow {
                label: "Realm".to_string(),
                key: "global".to_string(),
                display_name: "Global".to_string(),
                parent_key: None,
            },
            MetaRow {
                label: "Realm".to_string(),
                key: "project".to_string(),
                display_name: "Project".to_string(),
                parent_key: None,
            },
            MetaRow {
                label: "Layer".to_string(),
                key: "knowledge".to_string(),
                display_name: "Knowledge".to_string(),
                parent_key: Some("global".to_string()),
            },
            MetaRow {
                label: "Layer".to_string(),
                key: "structure".to_string(),
                display_name: "Structure".to_string(),
                parent_key: Some("project".to_string()),
            },
            MetaRow {
                label: "Kind".to_string(),
                key: "Locale".to_string(),
                display_name: "Locale".to_string(),
                parent_key: Some("knowledge".to_string()),
            },
            MetaRow {
                label: "Kind".to_string(),
                key: "Page".to_string(),
                display_name: "Page".to_string(),
                parent_key: Some("structure".to_string()),
            },
        ];
        TaxonomyTree::from_meta_rows(&rows)
    }

    #[test]
    fn build_tree_from_meta_rows() {
        let tree = sample_tree();
        assert_eq!(tree.roots.len(), 2);
        assert_eq!(tree.roots[0].key, "global");
        assert_eq!(tree.roots[0].children.len(), 1);
        assert_eq!(tree.roots[0].children[0].key, "knowledge");
        assert_eq!(tree.roots[0].children[0].children.len(), 1);
        assert_eq!(tree.roots[0].children[0].children[0].key, "Locale");
    }

    #[test]
    fn item_count() {
        let tree = sample_tree();
        // 2 realms + 2 layers + 2 kinds = 6
        assert_eq!(tree.item_count(), 6);
    }

    #[test]
    fn visible_count_all_expanded() {
        let tree = sample_tree();
        assert_eq!(tree.visible_count(), 6);
    }

    #[test]
    fn cursor_navigation() {
        let mut tree = sample_tree();
        assert_eq!(tree.cursor, 0);
        tree.cursor_down();
        assert_eq!(tree.cursor, 1);
        tree.cursor_down();
        assert_eq!(tree.cursor, 2);
        tree.cursor_up();
        assert_eq!(tree.cursor, 1);
        tree.cursor_up();
        assert_eq!(tree.cursor, 0);
        tree.cursor_up(); // stays at 0
        assert_eq!(tree.cursor, 0);
    }

    #[test]
    fn selected_node() {
        let tree = sample_tree();
        let selected = tree.selected().unwrap();
        assert_eq!(selected.key, "global");
    }

    #[test]
    fn toggle_expand_collapses_children() {
        let mut tree = sample_tree();
        assert_eq!(tree.visible_count(), 6);
        // Collapse global realm (cursor = 0)
        tree.toggle_expand();
        // global (collapsed) + project + structure + Page = 4
        assert_eq!(tree.visible_count(), 4);
    }

    #[test]
    fn visible_items_with_depth() {
        let tree = sample_tree();
        let items = tree.visible_items();
        assert_eq!(items.len(), 6);
        assert_eq!(items[0].0, 0); // realm: depth 0
        assert_eq!(items[0].1.key, "global");
        assert_eq!(items[1].0, 1); // layer: depth 1
        assert_eq!(items[1].1.key, "knowledge");
        assert_eq!(items[2].0, 2); // kind: depth 2
        assert_eq!(items[2].1.key, "Locale");
    }

    #[test]
    fn empty_tree() {
        let tree = TaxonomyTree::from_meta_rows(&[]);
        assert_eq!(tree.item_count(), 0);
        assert_eq!(tree.visible_count(), 0);
        assert!(tree.selected().is_none());
    }
}
