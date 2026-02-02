//! CRUD dialog forms for the TUI.
//!
//! Modal dialogs for node create/edit/delete and relation create/delete.
//! Each dialog has form fields with validation and a live Cypher preview.

/// Dialog type — determines which form fields to show.
///
/// `EditNode`, `DeleteNode`, and `DeleteRelation` are infrastructure for
/// Phase 7C (instance-level data browser). Currently only `CreateNode` and
/// `CreateRelation` are wired to key bindings.
#[derive(Debug, Clone)]
pub enum DialogKind {
    CreateNode,
    #[allow(dead_code)] // Phase 7C: requires instance-level data browser
    EditNode {
        key: String,
        current_kind: String,
    },
    #[allow(dead_code)] // Phase 7C: requires instance-level data browser
    DeleteNode {
        key: String,
        display_name: String,
    },
    CreateRelation {
        from_key: Option<String>,
    },
    #[allow(dead_code)] // Phase 7C: requires instance-level data browser
    DeleteRelation {
        from_key: String,
        to_key: String,
        rel_type: String,
    },
}

impl DialogKind {
    /// Title shown in the dialog border.
    pub fn title(&self) -> &'static str {
        match self {
            DialogKind::CreateNode => "Create Node",
            DialogKind::EditNode { .. } => "Edit Node",
            DialogKind::DeleteNode { .. } => "Delete Node",
            DialogKind::CreateRelation { .. } => "Create Relation",
            DialogKind::DeleteRelation { .. } => "Delete Relation",
        }
    }
}

/// Whether a field is text input, dropdown, or readonly display.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldKind {
    Text,
    Dropdown,
    Readonly,
}

/// Single form field with validation.
#[derive(Debug, Clone)]
pub struct FormField {
    pub label: String,
    pub value: String,
    pub cursor: usize,
    pub required: bool,
    pub error: Option<String>,
    pub field_kind: FieldKind,
    pub options: Vec<String>,
    pub selected_option: usize,
}

impl FormField {
    /// Create a text input field.
    pub fn text(label: &str, required: bool) -> Self {
        Self {
            label: label.to_string(),
            value: String::new(),
            cursor: 0,
            required,
            error: None,
            field_kind: FieldKind::Text,
            options: Vec::new(),
            selected_option: 0,
        }
    }

    /// Create a dropdown field with predefined options.
    pub fn dropdown(label: &str, options: Vec<String>) -> Self {
        Self {
            label: label.to_string(),
            value: options.first().cloned().unwrap_or_default(),
            cursor: 0,
            required: true,
            error: None,
            field_kind: FieldKind::Dropdown,
            options,
            selected_option: 0,
        }
    }

    /// Create a readonly display field.
    pub fn readonly(label: &str, value: &str) -> Self {
        Self {
            label: label.to_string(),
            value: value.to_string(),
            cursor: 0,
            required: false,
            error: None,
            field_kind: FieldKind::Readonly,
            options: Vec::new(),
            selected_option: 0,
        }
    }

    /// Insert a character at the cursor position.
    pub fn push_char(&mut self, c: char) {
        if self.field_kind == FieldKind::Readonly {
            return;
        }
        self.value.insert(self.cursor, c);
        self.cursor += c.len_utf8();
        self.error = None; // Clear error on input
    }

    /// Delete the character before the cursor.
    pub fn pop_char(&mut self) {
        if self.field_kind == FieldKind::Readonly || self.cursor == 0 {
            return;
        }
        // Find the previous char boundary
        let prev = self.value[..self.cursor]
            .char_indices()
            .next_back()
            .map(|(i, _)| i)
            .unwrap_or(0);
        self.value.remove(prev);
        self.cursor = prev;
        self.error = None;
    }

    /// Move cursor left.
    pub fn cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor = self.value[..self.cursor]
                .char_indices()
                .next_back()
                .map(|(i, _)| i)
                .unwrap_or(0);
        }
    }

    /// Move cursor right.
    pub fn cursor_right(&mut self) {
        if self.cursor < self.value.len() {
            self.cursor = self.value[self.cursor..]
                .char_indices()
                .nth(1)
                .map(|(i, _)| self.cursor + i)
                .unwrap_or(self.value.len());
        }
    }

    /// Select next dropdown option.
    pub fn option_next(&mut self) {
        if self.field_kind != FieldKind::Dropdown || self.options.is_empty() {
            return;
        }
        self.selected_option = (self.selected_option + 1) % self.options.len();
        self.value = self.options[self.selected_option].clone();
    }

    /// Select previous dropdown option.
    pub fn option_prev(&mut self) {
        if self.field_kind != FieldKind::Dropdown || self.options.is_empty() {
            return;
        }
        if self.selected_option == 0 {
            self.selected_option = self.options.len() - 1;
        } else {
            self.selected_option -= 1;
        }
        self.value = self.options[self.selected_option].clone();
    }

    /// Validate this field and set error message if invalid.
    pub fn validate(&mut self) -> bool {
        if self.required && self.value.trim().is_empty() {
            self.error = Some(format!("{} is required", self.label));
            return false;
        }
        self.error = None;
        true
    }

    /// Check if this field accepts text input.
    pub fn is_editable(&self) -> bool {
        self.field_kind == FieldKind::Text
    }
}

/// Dialog state — form fields + navigation.
#[derive(Debug, Clone)]
pub struct DialogState {
    pub kind: DialogKind,
    pub fields: Vec<FormField>,
    pub focused: usize,
    pub cypher_preview: String,
    pub submitting: bool,
    pub error: Option<String>,
}

impl DialogState {
    /// Create a new dialog with fields appropriate for the dialog kind.
    pub fn new(kind: DialogKind, kind_labels: &[String], edge_kind_keys: &[String]) -> Self {
        let fields = match &kind {
            DialogKind::CreateNode => vec![
                FormField::dropdown("Kind", kind_labels.to_vec()),
                FormField::text("Key", true),
                FormField::text("Display Name", false),
                FormField::text("Description", false),
            ],
            DialogKind::EditNode { key, current_kind } => vec![
                FormField::readonly("Kind", current_kind),
                FormField::readonly("Key", key),
                FormField::text("Properties (JSON)", true),
            ],
            DialogKind::DeleteNode {
                key, display_name, ..
            } => vec![
                FormField::readonly("Node", &format!("{display_name} ({key})")),
                FormField::text("Type \"delete\" to confirm", true),
            ],
            DialogKind::CreateRelation { from_key } => vec![
                if let Some(fk) = from_key {
                    FormField::readonly("From Key", fk)
                } else {
                    FormField::text("From Key", true)
                },
                FormField::text("To Key", true),
                FormField::dropdown("Relation Type", edge_kind_keys.to_vec()),
            ],
            DialogKind::DeleteRelation {
                from_key,
                to_key,
                rel_type,
            } => vec![
                FormField::readonly(
                    "Relation",
                    &format!("({from_key})-[:{rel_type}]->({to_key})"),
                ),
                FormField::text("Type \"delete\" to confirm", true),
            ],
        };

        // Find the first editable field to focus
        let focused = fields
            .iter()
            .position(|f| f.field_kind != FieldKind::Readonly)
            .unwrap_or(0);

        let mut state = Self {
            kind,
            fields,
            focused,
            cypher_preview: String::new(),
            submitting: false,
            error: None,
        };
        state.update_cypher_preview();
        state
    }

    /// Move focus to the next editable field.
    pub fn focus_next(&mut self) {
        let len = self.fields.len();
        for offset in 1..=len {
            let idx = (self.focused + offset) % len;
            if self.fields[idx].field_kind != FieldKind::Readonly {
                self.focused = idx;
                return;
            }
        }
    }

    /// Move focus to the previous editable field.
    pub fn focus_prev(&mut self) {
        let len = self.fields.len();
        for offset in 1..=len {
            let idx = (self.focused + len - offset) % len;
            if self.fields[idx].field_kind != FieldKind::Readonly {
                self.focused = idx;
                return;
            }
        }
    }

    /// Get the currently focused field.
    pub fn focused_field(&self) -> &FormField {
        &self.fields[self.focused]
    }

    /// Get a mutable reference to the currently focused field.
    pub fn focused_field_mut(&mut self) -> &mut FormField {
        &mut self.fields[self.focused]
    }

    /// Validate all fields. Returns true if all valid.
    pub fn is_valid(&mut self) -> bool {
        // Special case: delete confirmation must match "delete"
        if matches!(
            self.kind,
            DialogKind::DeleteNode { .. } | DialogKind::DeleteRelation { .. }
        ) {
            let confirm_field = self
                .fields
                .iter()
                .find(|f| f.label.contains("delete"))
                .map(|f| f.value.trim().to_lowercase());
            if confirm_field.as_deref() != Some("delete") {
                return false;
            }
        }

        let mut all_valid = true;
        for field in &mut self.fields {
            if !field.validate() {
                all_valid = false;
            }
        }
        all_valid
    }

    /// Find a field value by label.
    pub fn field_value(&self, label: &str) -> Option<&str> {
        self.fields
            .iter()
            .find(|f| f.label == label)
            .map(|f| f.value.as_str())
    }

    /// Regenerate the Cypher preview from current field values.
    pub fn update_cypher_preview(&mut self) {
        self.cypher_preview = match &self.kind {
            DialogKind::CreateNode => {
                let kind = self.field_value("Kind").unwrap_or("?");
                let key = self.field_value("Key").unwrap_or("?");
                let display_name = self.field_value("Display Name").unwrap_or("");
                let desc = self.field_value("Description").unwrap_or("");
                create_node_cypher(kind, key, display_name, desc)
            }
            DialogKind::EditNode { key, .. } => {
                let props = self.field_value("Properties (JSON)").unwrap_or("{}");
                edit_node_cypher(key, props)
            }
            DialogKind::DeleteNode { key, .. } => delete_node_cypher(key),
            DialogKind::CreateRelation { .. } => {
                let from = self.field_value("From Key").unwrap_or("?");
                let to = self.field_value("To Key").unwrap_or("?");
                let rt = self.field_value("Relation Type").unwrap_or("?");
                create_relation_cypher(from, to, rt)
            }
            DialogKind::DeleteRelation {
                from_key,
                to_key,
                rel_type,
            } => delete_relation_cypher(from_key, to_key, rel_type),
        };
    }
}

// ---------------------------------------------------------------------------
// Cypher preview generators
// ---------------------------------------------------------------------------

/// Generate Cypher preview for node creation.
pub fn create_node_cypher(kind: &str, key: &str, display_name: &str, description: &str) -> String {
    let mut lines = vec![
        format!("CREATE (n:{kind} {{key: '{key}'}})"),
        "SET n.created_at = datetime(), n.updated_at = datetime()".to_string(),
    ];
    if !display_name.is_empty() {
        lines.push(format!("SET n.display_name = '{display_name}'"));
    }
    if !description.is_empty() {
        lines.push(format!("SET n.description = '{description}'"));
    }
    lines.push(format!(
        "WITH n MATCH (k:Kind {{label: '{kind}'}}) CREATE (n)-[:OF_KIND]->(k)"
    ));
    lines.push("RETURN n".to_string());
    lines.join("\n")
}

/// Generate Cypher preview for node edit.
pub fn edit_node_cypher(key: &str, props_json: &str) -> String {
    format!(
        "MATCH (n {{key: '{key}'}})\nSET n += {props_json}\nSET n.updated_at = datetime()\nRETURN n"
    )
}

/// Generate Cypher preview for node deletion.
pub fn delete_node_cypher(key: &str) -> String {
    format!("MATCH (n {{key: '{key}'}})\nDETACH DELETE n")
}

/// Generate Cypher preview for relation creation.
pub fn create_relation_cypher(from: &str, to: &str, rel_type: &str) -> String {
    format!("MATCH (a {{key: '{from}'}}), (b {{key: '{to}'}})\nCREATE (a)-[:{rel_type}]->(b)")
}

/// Generate Cypher preview for relation deletion.
pub fn delete_relation_cypher(from: &str, to: &str, rel_type: &str) -> String {
    format!("MATCH (a {{key: '{from}'}})-[r:{rel_type}]->(b {{key: '{to}'}})\nDELETE r")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_kinds() -> Vec<String> {
        vec![
            "Page".to_string(),
            "Block".to_string(),
            "Concept".to_string(),
        ]
    }

    fn sample_edge_kinds() -> Vec<String> {
        vec![
            "HAS_BLOCK".to_string(),
            "HAS_PAGE".to_string(),
            "OF_KIND".to_string(),
        ]
    }

    // -----------------------------------------------------------------------
    // FormField tests
    // -----------------------------------------------------------------------

    #[test]
    fn text_field_push_pop() {
        let mut f = FormField::text("Key", true);
        f.push_char('a');
        f.push_char('b');
        assert_eq!(f.value, "ab");
        assert_eq!(f.cursor, 2);
        f.pop_char();
        assert_eq!(f.value, "a");
        assert_eq!(f.cursor, 1);
    }

    #[test]
    fn text_field_cursor_movement() {
        let mut f = FormField::text("Key", false);
        f.push_char('h');
        f.push_char('i');
        assert_eq!(f.cursor, 2);
        f.cursor_left();
        assert_eq!(f.cursor, 1);
        f.cursor_left();
        assert_eq!(f.cursor, 0);
        f.cursor_left(); // clamp at 0
        assert_eq!(f.cursor, 0);
        f.cursor_right();
        assert_eq!(f.cursor, 1);
        f.cursor_right();
        assert_eq!(f.cursor, 2);
        f.cursor_right(); // clamp at end
        assert_eq!(f.cursor, 2);
    }

    #[test]
    fn readonly_field_ignores_input() {
        let mut f = FormField::readonly("Info", "hello");
        f.push_char('x');
        assert_eq!(f.value, "hello");
        f.pop_char();
        assert_eq!(f.value, "hello");
    }

    #[test]
    fn dropdown_option_cycling() {
        let opts = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let mut f = FormField::dropdown("Pick", opts);
        assert_eq!(f.value, "A");
        f.option_next();
        assert_eq!(f.value, "B");
        f.option_next();
        assert_eq!(f.value, "C");
        f.option_next(); // wraps
        assert_eq!(f.value, "A");
        f.option_prev(); // wraps backwards
        assert_eq!(f.value, "C");
        f.option_prev();
        assert_eq!(f.value, "B");
    }

    #[test]
    fn field_validation_required_empty() {
        let mut f = FormField::text("Key", true);
        assert!(!f.validate());
        assert!(f.error.is_some());
    }

    #[test]
    fn field_validation_required_filled() {
        let mut f = FormField::text("Key", true);
        f.push_char('x');
        assert!(f.validate());
        assert!(f.error.is_none());
    }

    #[test]
    fn field_validation_not_required_empty() {
        let mut f = FormField::text("Desc", false);
        assert!(f.validate());
    }

    #[test]
    fn field_is_editable() {
        assert!(FormField::text("x", false).is_editable());
        assert!(!FormField::readonly("x", "v").is_editable());
        assert!(!FormField::dropdown("x", vec!["a".into()]).is_editable());
    }

    // -----------------------------------------------------------------------
    // DialogState tests
    // -----------------------------------------------------------------------

    #[test]
    fn create_node_dialog_fields() {
        let dlg = DialogState::new(DialogKind::CreateNode, &sample_kinds(), &[]);
        assert_eq!(dlg.fields.len(), 4); // Kind, Key, Display Name, Description
        assert_eq!(dlg.fields[0].field_kind, FieldKind::Dropdown);
        assert_eq!(dlg.fields[0].value, "Page"); // first option
        assert_eq!(dlg.fields[1].field_kind, FieldKind::Text);
        assert!(dlg.fields[1].required);
        // Focus should skip dropdown and land on Key text field
        assert_eq!(dlg.focused, 0); // dropdown is editable in sense of cycling
    }

    #[test]
    fn edit_node_dialog_fields() {
        let dlg = DialogState::new(
            DialogKind::EditNode {
                key: "my-page".into(),
                current_kind: "Page".into(),
            },
            &[],
            &[],
        );
        assert_eq!(dlg.fields.len(), 3);
        assert_eq!(dlg.fields[0].field_kind, FieldKind::Readonly);
        assert_eq!(dlg.fields[1].field_kind, FieldKind::Readonly);
        assert_eq!(dlg.fields[2].field_kind, FieldKind::Text);
        assert_eq!(dlg.focused, 2); // first editable
    }

    #[test]
    fn delete_node_dialog_needs_confirm() {
        let mut dlg = DialogState::new(
            DialogKind::DeleteNode {
                key: "x".into(),
                display_name: "X".into(),
            },
            &[],
            &[],
        );
        assert!(!dlg.is_valid()); // confirmation not typed

        // Type "delete"
        let confirm_idx = dlg
            .fields
            .iter()
            .position(|f| f.label.contains("delete"))
            .unwrap();
        for c in "delete".chars() {
            dlg.fields[confirm_idx].push_char(c);
        }
        assert!(dlg.is_valid());
    }

    #[test]
    fn create_relation_dialog_fields() {
        let dlg = DialogState::new(
            DialogKind::CreateRelation {
                from_key: Some("page1".into()),
            },
            &[],
            &sample_edge_kinds(),
        );
        assert_eq!(dlg.fields.len(), 3);
        assert_eq!(dlg.fields[0].field_kind, FieldKind::Readonly); // from_key pre-filled
        assert_eq!(dlg.fields[0].value, "page1");
        assert_eq!(dlg.fields[1].field_kind, FieldKind::Text); // to_key
        assert_eq!(dlg.fields[2].field_kind, FieldKind::Dropdown); // rel type
    }

    #[test]
    fn focus_cycling_skips_readonly() {
        let mut dlg = DialogState::new(
            DialogKind::EditNode {
                key: "k".into(),
                current_kind: "Page".into(),
            },
            &[],
            &[],
        );
        assert_eq!(dlg.focused, 2); // Properties (JSON) field
        dlg.focus_next();
        assert_eq!(dlg.focused, 2); // Only one editable, stays
        dlg.focus_prev();
        assert_eq!(dlg.focused, 2);
    }

    #[test]
    fn field_value_lookup() {
        let dlg = DialogState::new(DialogKind::CreateNode, &sample_kinds(), &[]);
        assert_eq!(dlg.field_value("Kind"), Some("Page"));
        assert_eq!(dlg.field_value("Nonexistent"), None);
    }

    #[test]
    fn cypher_preview_create_node() {
        let dlg = DialogState::new(DialogKind::CreateNode, &sample_kinds(), &[]);
        assert!(dlg.cypher_preview.contains("CREATE"));
        assert!(dlg.cypher_preview.contains("Page"));
        assert!(dlg.cypher_preview.contains("OF_KIND"));
    }

    #[test]
    fn cypher_preview_delete_node() {
        let dlg = DialogState::new(
            DialogKind::DeleteNode {
                key: "my-page".into(),
                display_name: "My Page".into(),
            },
            &[],
            &[],
        );
        assert!(dlg.cypher_preview.contains("DETACH DELETE"));
        assert!(dlg.cypher_preview.contains("my-page"));
    }

    #[test]
    fn cypher_preview_create_relation() {
        let dlg = DialogState::new(
            DialogKind::CreateRelation { from_key: None },
            &[],
            &sample_edge_kinds(),
        );
        assert!(dlg.cypher_preview.contains("CREATE"));
        assert!(dlg.cypher_preview.contains("HAS_BLOCK"));
    }

    // -----------------------------------------------------------------------
    // Cypher generator tests
    // -----------------------------------------------------------------------

    #[test]
    fn create_node_cypher_basic() {
        let cypher = create_node_cypher("Page", "my-page", "My Page", "");
        assert!(cypher.contains("CREATE (n:Page {key: 'my-page'})"));
        assert!(cypher.contains("display_name"));
        assert!(!cypher.contains("description"));
    }

    #[test]
    fn create_node_cypher_with_description() {
        let cypher = create_node_cypher("Block", "b1", "Block 1", "A block");
        assert!(cypher.contains("description"));
        assert!(cypher.contains("A block"));
    }

    #[test]
    fn edit_node_cypher_basic() {
        let cypher = edit_node_cypher("my-page", "{\"title\": \"hello\"}");
        assert!(cypher.contains("MATCH"));
        assert!(cypher.contains("my-page"));
        assert!(cypher.contains("updated_at"));
    }

    #[test]
    fn delete_node_cypher_basic() {
        let cypher = delete_node_cypher("my-page");
        assert!(cypher.contains("DETACH DELETE"));
    }

    #[test]
    fn create_relation_cypher_basic() {
        let cypher = create_relation_cypher("page1", "block1", "HAS_BLOCK");
        assert!(cypher.contains("page1"));
        assert!(cypher.contains("block1"));
        assert!(cypher.contains("HAS_BLOCK"));
    }

    #[test]
    fn delete_relation_cypher_basic() {
        let cypher = delete_relation_cypher("page1", "block1", "HAS_BLOCK");
        assert!(cypher.contains("DELETE r"));
        assert!(cypher.contains("HAS_BLOCK"));
    }
}
