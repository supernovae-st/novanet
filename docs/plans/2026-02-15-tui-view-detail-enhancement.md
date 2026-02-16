# TUI View Detail Enhancement Plan

## Objectif

Enrichir le panneau VIEW DETAIL dans la TUI avec:
1. Requête Cypher complète (scrollable, pas tronquée)
2. Lien Studio clickable
3. Liste des relations (arcs) utilisés avec couleurs par famille
4. Schéma ASCII du pattern de traversal

## Architecture Actuelle

```
src/tui/nexus/views.rs
├── ViewsState          # État (cursors, scroll)
├── LoadedViews         # Données chargées depuis views.yaml
├── render_views_tab()  # Point d'entrée
├── render_views_list() # Liste gauche (catégories + views)
└── render_view_detail()# Panneau droit (PROBLÈME: truncated)
```

## Tâches

### Task 1: ViewDetailSection enum + scroll state

Ajouter une enum pour les sections du détail et un scroll offset:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewDetailSection {
    #[default]
    Info,
    Cypher,
    Relations,
    Schema,
}

pub struct ViewsState {
    // ... existing
    pub detail_section: ViewDetailSection,
    pub detail_scroll: u16,
}
```

**Fichier**: `src/tui/nexus/views.rs`

### Task 2: Extraire les arcs du Cypher

Parser le Cypher pour extraire les noms de relations:

```rust
fn extract_relations_from_cypher(cypher: &str) -> Vec<String> {
    // Regex: \[r\d*:([A-Z_]+)\]
    // Returns: ["HAS_BLOCK", "USES_ENTITY", "HAS_NATIVE", ...]
}
```

**Fichier**: `src/tui/nexus/views.rs`

### Task 3: Lookup arc colors from taxonomy

Charger les couleurs des familles d'arcs depuis taxonomy.yaml:

```rust
fn arc_family_color(arc_name: &str, app: &App) -> Color {
    // Lookup arc in app.taxonomy.arc_classes
    // Get family, return family.color
}
```

**Fichier**: `src/tui/nexus/views.rs` (utiliser `app.taxonomy`)

### Task 4: Render full Cypher (scrollable)

Remplacer la version tronquée par une version complète scrollable:

```rust
fn render_cypher_section(lines: &mut Vec<Line>, cypher: &str, scroll: u16, area_height: u16) {
    // Syntax highlighting (MATCH, RETURN, OPTIONAL, WHERE)
    // Full query with scroll offset
}
```

**Fichier**: `src/tui/nexus/views.rs`

### Task 5: Render relations list

Afficher les arcs extraits avec leurs couleurs:

```rust
fn render_relations_section(lines: &mut Vec<Line>, relations: &[String], app: &App) {
    // For each arc:
    //   → ARC_NAME  [family_icon] family  color_swatch
}
```

**Fichier**: `src/tui/nexus/views.rs`

### Task 6: Generate ASCII schema diagram

Générer un schéma ASCII du pattern de traversal:

```
┌─────────────────────────────────────────┐
│  Page Context (gen-page)                │
├─────────────────────────────────────────┤
│                                         │
│  ┌──────┐   HAS_BLOCK    ┌───────┐      │
│  │ Page ├───────────────►│ Block │      │
│  └──┬───┘                └───┬───┘      │
│     │                        │          │
│     │ REPRESENTS       USES_ENTITY      │
│     ▼                        ▼          │
│  ┌────────┐            ┌────────┐       │
│  │ Entity │◄───────────┤ Entity │       │
│  └───┬────┘            └────────┘       │
│      │                                  │
│      │ HAS_NATIVE                       │
│      ▼                                  │
│  ┌──────────────┐                       │
│  │ EntityNative │                       │
│  └──────────────┘                       │
│                                         │
└─────────────────────────────────────────┘
```

**Fichier**: `src/tui/nexus/views.rs` (nouvelle fonction `generate_view_ascii_schema`)

### Task 7: Studio link

Ajouter le lien vers Studio:

```rust
// In render_view_detail:
lines.push(Line::from(vec![
    Span::styled("Studio:   ", Style::default().fg(Color::DarkGray)),
    Span::styled(
        format!("http://localhost:3000/views/{}", view.id),
        Style::default().fg(Color::Cyan).add_modifier(Modifier::UNDERLINED),
    ),
]));
```

**Fichier**: `src/tui/nexus/views.rs`

### Task 8: Tab navigation between sections

Ajouter la navigation par Tab entre les sections:

```rust
// In handlers/views.rs or handlers/nexus.rs:
KeyCode::Tab => {
    state.views.detail_section = match state.views.detail_section {
        ViewDetailSection::Info => ViewDetailSection::Cypher,
        ViewDetailSection::Cypher => ViewDetailSection::Relations,
        ViewDetailSection::Relations => ViewDetailSection::Schema,
        ViewDetailSection::Schema => ViewDetailSection::Info,
    };
}
```

**Fichier**: `src/tui/handlers/views.rs` ou `nexus.rs`

### Task 9: Scroll dans la section Cypher

Utiliser `Ctrl+d/u` ou `Page Down/Up` pour scroller:

```rust
KeyCode::Char('d') if modifiers.contains(KeyModifiers::CONTROL) => {
    if state.views.detail_section == ViewDetailSection::Cypher {
        state.views.detail_scroll += 10;
    }
}
```

**Fichier**: `src/tui/handlers/views.rs`

### Task 10: Tests

Ajouter des tests pour:
- `extract_relations_from_cypher()` - extraction correcte
- `generate_view_ascii_schema()` - génération ASCII
- Navigation Tab et scroll

**Fichier**: `src/tui/nexus/views.rs` (section tests)

## Keybindings Ajoutés

| Key | Action |
|-----|--------|
| `Tab` | Cycle entre sections (Info → Cypher → Relations → Schema) |
| `Shift+Tab` | Cycle inverse |
| `Ctrl+d` / `Page Down` | Scroll down dans Cypher |
| `Ctrl+u` / `Page Up` | Scroll up dans Cypher |
| `y` | Yank (copier) le Cypher complet |

## Dépendances

- `regex` - déjà présent, pour parser les arcs du Cypher
- `app.taxonomy` - déjà chargé, pour les couleurs des familles

## Impact

- Meilleure compréhension des views
- Debug plus facile (Cypher complet visible)
- Lien direct vers Studio pour visualisation
- Documentation visuelle des patterns de traversal

## Estimation

~4-6h de travail (10 tâches)
