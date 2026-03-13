# NovaNet Hybrid Data Management System — Implementation Plan

**Version**: v0.20.0 | **Status**: Proposed | **Date**: 2026-03-13
**Author**: Claude + Thibaut | **Tracks**: A (YAML Governance) + C (Docker Snapshots)

---

## Executive Summary

NovaNet's Neo4j graph (~20K nodes, 59 classes, 159 arcs, 200+ locales) currently has a **one-way data pipeline**: YAML → Cypher → Neo4j. Runtime data created through MCP tools or Nika workflows is volatile — lost on `infra:reset`. This plan implements a **bidirectional, YAML-first data management system** with two complementary tracks:

- **Track A** (Governance): Neo4j → YAML → git — promote runtime data to version-controlled seeds
- **Track C** (Safety): Docker volume binary snapshots — instant point-in-time recovery

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  HYBRID DATA MANAGEMENT SYSTEM                                                  │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Track A: YAML Governance (long-term)                                           │
│  ┌──────────┐    ┌─────────┐    ┌──────────┐    ┌──────────┐    ┌─────────┐   │
│  │  Neo4j   │ →  │ Export  │ →  │  Review  │ →  │ Promote  │ →  │  Git    │   │
│  │ Runtime  │    │  YAML   │    │  Diff    │    │  Seeds   │    │ Commit  │   │
│  └──────────┘    └─────────┘    └──────────┘    └──────────┘    └─────────┘   │
│                                                                                 │
│  Track C: Docker Snapshots (safety net)                                         │
│  ┌──────────┐    ┌─────────┐    ┌──────────┐                                   │
│  │  Neo4j   │ →  │  Stop   │ →  │ tar.gz   │ →  ~/.novanet/snapshots/          │
│  │ Volume   │    │ + Tar   │    │ Volume   │                                   │
│  └──────────┘    └─────────┘    └──────────┘                                   │
│                                                                                 │
│  TUI: Live backup indicator in status bar (green/yellow/red)                   │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## 1. Data Layer Model

Understanding the 4 data layers is essential for the system design:

| Layer | Persistence | Source of Truth | Examples |
|-------|-------------|-----------------|----------|
| **L1: Schema** | Immortal | `models/*.yaml` in git | NodeClass, ArcClass, Realm, Layer |
| **L2: Knowledge** | Immortal | `packages/db/seed/*.cypher` in git | Locale, Formatting, Expression |
| **L3: Content** | Protected | `private-data/data/*.yaml` in git | Entity, EntityNative, Page, Block |
| **L4: Runtime** | **Volatile** | Neo4j only (today) | MCP-created nodes, Nika outputs |

**The critical gap**: L4 → L3 promotion. This plan bridges it.

---

## 2. CLI Command Design

### 2.1 Track A: `novanet data` (Governance)

```
novanet data
├── export          Neo4j → YAML files (to .novanet/export/)
│   ├── --class=<C>         Filter by node class (Entity, EntityNative, Page...)
│   ├── --project=<P>       Filter by project key (qrcode-ai)
│   ├── --locale=<L>        Filter by locale (fr-FR)
│   ├── --since=<ISO>       Incremental: only nodes updated since date
│   ├── --incremental       Use checkpoint from last export
│   ├── --output=<DIR>      Custom output directory
│   └── --dry-run           Preview what would be exported
│
├── promote         .novanet/export/ → private-data/data/ (with review)
│   ├── --interactive       Show diff and ask confirmation per file (default)
│   ├── --no-interactive    Accept all changes without prompting
│   ├── --dry-run           Show what would be promoted
│   └── --regenerate        Run seed generate after promotion
│
├── diff            Compare Neo4j state vs YAML seeds
│   ├── --class=<C>         Filter by node class
│   ├── --format=table|json Output format
│   └── --verbose           Show property-level changes
│
└── sync            Bidirectional convenience wrapper
    ├── pull                export + promote (Neo4j → YAML)
    └── push                seed generate + db seed (YAML → Neo4j)
```

### 2.2 Track C: `novanet snapshot` (Safety)

```
novanet snapshot
├── create          Binary snapshot of Neo4j Docker volume
│   ├── --description=<D>   Human label for the snapshot
│   ├── --include-apoc      Also create APOC Cypher export (default: true)
│   └── --snapshot-dir=<D>  Custom directory (default: ~/.novanet/snapshots/)
│
├── list            Show existing snapshots with metadata
│   └── --verbose           Include node/arc counts and size
│
├── restore <NAME>  Restore a snapshot (destructive, asks confirmation)
│   └── --yes               Skip confirmation prompt
│
└── prune           Delete old snapshots
    ├── --keep=<N>          Keep N most recent (default: 5)
    └── --dry-run           Show what would be deleted
```

### 2.3 UX Principles

- **Every destructive operation** has `--dry-run` and `--yes` flags
- **Default behavior is safe**: `promote` is interactive, `restore` asks confirmation
- **Progress bars** via `indicatif` for long operations
- **Colored output** via `colored` crate (already in use)
- **Error hints** integrated with existing `ErrorHint` system

---

## 3. Architecture Blueprint

### 3.1 New Files

| File | Lines (est.) | Purpose |
|------|-------------|---------|
| `commands/data_export.rs` | ~500 | Track A: Neo4j → YAML export |
| `commands/data_promote.rs` | ~400 | Track A: YAML → private-data promotion |
| `commands/data_diff.rs` | ~300 | Track A: Neo4j vs YAML comparison |
| `commands/snapshot.rs` | ~600 | Track C: Docker volume snapshots |
| `core/checkpoint.rs` | ~150 | Incremental export checkpoint tracking |

### 3.2 Modified Files

| File | Changes |
|------|---------|
| `main.rs` | Add `Data` and `Snapshot` command enums + dispatch |
| `commands/mod.rs` | Export 4 new modules |
| `tui/app.rs` (state) | Add `BackupStatus` field to `App` struct |
| `tui/ui/status.rs` | Add backup indicator to status bar |
| `Cargo.toml` | Add `similar` crate for YAML diffs |

### 3.3 Directory Structure

```
~/.novanet/
├── config.toml              # Existing: CLI configuration
├── backups/                 # Existing: brain/ tar.gz backups
│   └── novanet-backup-*.tar.gz
│
├── export/                  # NEW (Track A): Temporary export staging
│   ├── entities/
│   │   └── qrcode-ai/
│   │       └── entities.yaml
│   ├── natives/
│   │   └── qrcode-ai/
│   │       ├── fr-FR.yaml
│   │       ├── es-MX.yaml
│   │       └── en-US.yaml
│   ├── pages/
│   │   └── qrcode-ai/
│   │       └── pages.yaml
│   └── .checkpoint.json     # Last export timestamp per class
│
└── snapshots/               # NEW (Track C): Docker volume snapshots
    ├── novanet-snap-20260313-143022/
    │   ├── volume.tar.gz    # Binary Docker volume archive
    │   ├── apoc-export.cypher   # Supplementary Cypher dump
    │   └── metadata.json    # Timestamp, counts, description
    └── novanet-snap-20260312-120000/
        └── ...
```

---

## 4. Track A: YAML Governance — Detailed Design

### 4.1 Export: `novanet data export`

**Core flow**: Query Neo4j → Transform to YAML structs → Write to `.novanet/export/`

```rust
// commands/data_export.rs

/// Export arguments
#[derive(Debug, Clone, Parser)]
pub struct DataExportArgs {
    /// Filter by node class (Entity, EntityNative, Page, PageNative, Block...)
    #[arg(long)]
    pub class: Option<Vec<String>>,

    /// Filter by project key
    #[arg(long)]
    pub project: Option<String>,

    /// Filter by locale (for *Native classes)
    #[arg(long)]
    pub locale: Option<String>,

    /// Only export nodes updated since this ISO date
    #[arg(long)]
    pub since: Option<String>,

    /// Use checkpoint from last export
    #[arg(long)]
    pub incremental: bool,

    /// Output directory (default: ~/.novanet/export)
    #[arg(long)]
    pub output: Option<PathBuf>,

    /// Preview without writing
    #[arg(long)]
    pub dry_run: bool,
}
```

**Cypher query strategy** — uses standard Neo4j queries (no APOC dependency for basic export):

```cypher
// Full export of a class
MATCH (n:EntityNative)
WHERE ($project IS NULL OR n.key STARTS WITH $project)
  AND ($locale IS NULL OR n.key ENDS WITH $locale)
  AND ($since IS NULL OR n.updated_at > datetime($since))
RETURN n
ORDER BY n.key

// With relationships (for context)
MATCH (n:EntityNative)-[r:FOR_LOCALE]->(l:Locale)
WHERE n.key STARTS WITH $project
RETURN n, type(r) AS rel_type, l.key AS locale_key
```

**YAML output format** — mirrors existing `private-data/data/` structure:

```yaml
# .novanet/export/natives/qrcode-ai/fr-FR.yaml
# Exported from Neo4j on 2026-03-13T14:30:22Z
# Class: EntityNative | Project: qrcode-ai | Locale: fr-FR
---
entity_natives:
  - key: "qr-code@fr-FR"
    entity_key: "qr-code"
    locale: "fr-FR"
    display_name: "Code QR"
    content: "Un code QR est un code-barres bidimensionnel..."
    denomination_forms:
      - type: text
        value: "code QR"
        priority: 1
      - type: title
        value: "Code QR"
        priority: 1
      - type: abbrev
        value: "QR"
        priority: 1
      - type: url
        value: "code-qr"
        priority: 1
    provenance:
      source: "nika"
      workflow: "00-entity-native-bootstrap"
      generated_at: "2026-03-12T10:00:00Z"
```

**Checkpoint system** — tracks last export timestamp per class:

```json
// .novanet/export/.checkpoint.json
{
  "version": 1,
  "exports": {
    "Entity": {
      "last_export": "2026-03-13T14:30:22Z",
      "node_count": 45,
      "filters": { "project": "qrcode-ai" }
    },
    "EntityNative": {
      "last_export": "2026-03-13T14:30:22Z",
      "node_count": 1250,
      "filters": { "project": "qrcode-ai" }
    }
  }
}
```

### 4.2 Promote: `novanet data promote`

**Core flow**: `.novanet/export/` → diff → review → merge into `private-data/data/`

```rust
// commands/data_promote.rs

/// Promote arguments
#[derive(Debug, Clone, Parser)]
pub struct DataPromoteArgs {
    /// Show diff and confirm per file (default: true)
    #[arg(long, default_value = "true")]
    pub interactive: bool,

    /// Preview without writing
    #[arg(long)]
    pub dry_run: bool,

    /// Run seed generate after promotion
    #[arg(long)]
    pub regenerate: bool,
}
```

**Diff display** — using `similar` crate for unified diffs:

```
novanet data promote

  Reviewing exports...

  📄 natives/qrcode-ai/fr-FR.yaml
     + NEW: qr-code@fr-FR (EntityNative)
       display_name: "Code QR"
       content: "Un code QR est un code-barres bidimensionnel..."

     ~ MODIFIED: dynamic-qr@fr-FR (EntityNative)
       - content: "Un QR code dynamique permet de..."
       + content: "Un code QR dynamique permet de modifier..."

     Promote this file? [y/N/a(ll)/s(kip all)]
```

**Merge strategy**:
- **New entries**: Append to YAML file
- **Modified entries**: Show diff, ask confirmation, update in place
- **Deleted in Neo4j**: Warn but do NOT delete from YAML (YAML is authoritative)
- **Conflict resolution**: YAML always wins — if same key exists in both, keep YAML version

### 4.3 Diff: `novanet data diff`

**Core flow**: Compare Neo4j state against YAML seeds, report additions/modifications/deletions.

```
novanet data diff --class=EntityNative

  Comparing Neo4j vs YAML seeds...

  EntityNative:
    In Neo4j:    1,250 nodes
    In YAML:     1,180 nodes

    + 70 nodes only in Neo4j (not yet promoted)
    ~ 12 nodes modified since last seed
    = 1,168 nodes in sync

    Run `novanet data export --class=EntityNative` to capture changes.
```

### 4.4 Sync: `novanet data sync`

Convenience wrapper for common bidirectional operations:

```bash
# Pull: Neo4j → YAML (export + promote)
novanet data sync pull

# Push: YAML → Neo4j (seed generate + db seed)
novanet data sync push
```

---

## 5. Track C: Docker Snapshots — Detailed Design

### 5.1 Snapshot Create

**Core flow**: Stop Neo4j → tar volume via alpine → restart → wait → optional APOC export

```rust
// commands/snapshot.rs

pub async fn run_snapshot_create(args: SnapshotCreateArgs) -> Result<PathBuf> {
    let snapshot_dir = resolve_snapshot_dir(&args)?;
    let timestamp = Utc::now().format("%Y%m%d-%H%M%S");
    let name = format!("novanet-snap-{}", timestamp);
    let path = snapshot_dir.join(&name);
    fs::create_dir_all(&path)?;

    let pb = ProgressBar::new(5);
    pb.set_style(progress_style());

    // Step 1: Pre-snapshot stats (while Neo4j is still running)
    pb.set_message("Collecting stats...");
    let stats = collect_neo4j_stats().await?;
    pb.inc(1);

    // Step 2: Stop Neo4j
    pb.set_message("Stopping Neo4j...");
    docker_stop("novanet-neo4j").await?;
    pb.inc(1);

    // Step 3: Tar the volume
    pb.set_message("Archiving volume...");
    docker_tar_volume("novanet_neo4j_data", &path.join("volume.tar.gz")).await?;
    pb.inc(1);

    // Step 4: Restart Neo4j
    pb.set_message("Restarting Neo4j...");
    docker_start("novanet-neo4j").await?;
    wait_for_neo4j(30).await?;
    pb.inc(1);

    // Step 5: Write metadata
    pb.set_message("Writing metadata...");
    write_metadata(&path, &args, &stats)?;
    pb.inc(1);

    pb.finish_with_message(format!("Snapshot created: {}", name));
    Ok(path)
}
```

**Docker operations** — all via `tokio::process::Command`:

```rust
/// Tar a Docker named volume via an alpine container
async fn docker_tar_volume(volume: &str, output: &Path) -> Result<()> {
    let parent = output.parent().unwrap();
    let filename = output.file_name().unwrap().to_string_lossy();

    let status = tokio::process::Command::new("docker")
        .args([
            "run", "--rm",
            "-v", &format!("{}:/volume:ro", volume),
            "-v", &format!("{}:/backup", parent.display()),
            "alpine",
            "tar", "czf", &format!("/backup/{}", filename),
            "-C", "/volume", "."
        ])
        .status()
        .await?;

    if !status.success() {
        return Err(NovaNetError::Validation(
            "Failed to create volume archive".into()
        ));
    }
    Ok(())
}
```

**Metadata format**:

```json
{
  "version": 1,
  "created_at": "2026-03-13T14:30:22Z",
  "description": "Before schema migration v0.20",
  "neo4j_version": "5.26.0-community",
  "stats": {
    "node_count": 19817,
    "arc_count": 58607,
    "locale_count": 204
  },
  "volume": {
    "name": "novanet_neo4j_data",
    "archive_size_bytes": 45678912,
    "sha256": "a1b2c3d4..."
  },
  "includes_apoc": true
}
```

### 5.2 Snapshot Restore

```rust
pub async fn run_snapshot_restore(args: SnapshotRestoreArgs) -> Result<()> {
    let snap_path = resolve_snapshot_path(&args.snapshot)?;
    let metadata = read_metadata(&snap_path)?;

    // Show confirmation
    println!("Restore snapshot:");
    println!("  Created: {}", metadata.created_at);
    println!("  Nodes:   {}", metadata.stats.node_count);
    println!("  Arcs:    {}", metadata.stats.arc_count);
    if let Some(desc) = &metadata.description {
        println!("  Description: {}", desc);
    }

    if !args.yes {
        println!("\n  This will OVERWRITE all current Neo4j data!");
        // Use dialoguer or simple stdin confirm
        if !confirm("Continue?")? {
            println!("Restore cancelled.");
            return Ok(());
        }
    }

    let pb = ProgressBar::new(4);

    // 1. Stop Neo4j
    pb.set_message("Stopping Neo4j...");
    docker_stop("novanet-neo4j").await?;
    pb.inc(1);

    // 2. Replace volume
    pb.set_message("Restoring volume...");
    docker_rm_volume("novanet_neo4j_data").await?;
    docker_create_volume("novanet_neo4j_data").await?;
    docker_untar_volume("novanet_neo4j_data", &snap_path.join("volume.tar.gz")).await?;
    pb.inc(1);

    // 3. Start Neo4j
    pb.set_message("Starting Neo4j...");
    docker_start("novanet-neo4j").await?;
    pb.inc(1);

    // 4. Wait ready
    pb.set_message("Waiting for Neo4j...");
    wait_for_neo4j(30).await?;
    pb.inc(1);

    pb.finish_with_message("Snapshot restored successfully");
    Ok(())
}
```

---

## 6. TUI Backup Indicator

### 6.1 Design

The status bar in `tui/ui/status.rs` already has a rich layout:

```
HINTS │ MODE │ BREADCRUMB │ STATS │ REALM-BAR │ [1]G [2]N ?:help
```

We add the backup indicator **before the mode keys**, as a compact colored segment:

```
HINTS │ MODE │ BREADCRUMB │ STATS │ REALM-BAR │ ● 2h │ [1]G [2]N ?:help
                                                 ^^^^
                                            Backup indicator
```

### 6.2 Color Coding

| Color | Condition | Icon | Example |
|-------|-----------|------|---------|
| Green | Last backup/snapshot < 24h ago | `●` | `● 2h` |
| Yellow | Last backup/snapshot 1-7 days ago | `●` | `● 3d` |
| Red | Last backup/snapshot > 7 days ago OR never | `●` | `● 12d` or `● Never` |

### 6.3 Implementation

**State** — add to `App` struct in `tui/app.rs`:

```rust
/// Backup/snapshot status for TUI indicator
pub struct BackupStatus {
    pub last_backup: Option<DateTime<Utc>>,
    pub last_snapshot: Option<DateTime<Utc>>,
}

impl BackupStatus {
    /// Most recent of backup or snapshot
    pub fn most_recent(&self) -> Option<DateTime<Utc>> {
        match (self.last_backup, self.last_snapshot) {
            (Some(b), Some(s)) => Some(b.max(s)),
            (Some(b), None) => Some(b),
            (None, Some(s)) => Some(s),
            (None, None) => None,
        }
    }

    /// Age label: "2h", "3d", "Never"
    pub fn age_label(&self) -> String {
        match self.most_recent() {
            Some(ts) => {
                let age = Utc::now().signed_duration_since(ts);
                if age.num_hours() < 24 {
                    format!("{}h", age.num_hours())
                } else {
                    format!("{}d", age.num_days())
                }
            }
            None => "Never".to_string(),
        }
    }

    /// Status color
    pub fn color(&self) -> Color {
        match self.most_recent() {
            Some(ts) => {
                let age = Utc::now().signed_duration_since(ts);
                if age.num_days() < 1 { Color::Green }
                else if age.num_days() < 7 { Color::Yellow }
                else { Color::Red }
            }
            None => Color::Red,
        }
    }
}
```

**Rendering** — insert into `render_status()` in `tui/ui/status.rs`, before the mini-cheatsheet section:

```rust
// 5.5 BACKUP INDICATOR (before mode keys)
if let Some(backup_status) = &app.backup_status {
    spans.push(Span::styled(" │ ", STYLE_SEPARATOR));
    let color = backup_status.color();
    let label = backup_status.age_label();
    spans.push(Span::styled("●", Style::default().fg(color)));
    spans.push(Span::styled(format!(" {}", label), Style::default().fg(color)));
}
```

**Loading** — compute on TUI startup (non-blocking):

```rust
impl BackupStatus {
    pub fn load() -> Self {
        let backup_dir = dirs::home_dir()
            .map(|h| h.join(".novanet/backups"));
        let snapshot_dir = dirs::home_dir()
            .map(|h| h.join(".novanet/snapshots"));

        let last_backup = backup_dir
            .and_then(|d| most_recent_file_date(&d).ok());
        let last_snapshot = snapshot_dir
            .and_then(|d| most_recent_snapshot_date(&d).ok());

        Self { last_backup, last_snapshot }
    }
}
```

---

## 7. Integration with Existing Systems

### 7.1 Pre-Seed Auto-Snapshot

Before every `infra:reset` or `db reset`, automatically create a snapshot:

```rust
// In main.rs, before run_reset()
if !skip_backup {
    println!("Creating pre-reset snapshot...");
    snapshot::run_snapshot_create(SnapshotCreateArgs {
        description: Some("Auto: pre-reset".into()),
        include_apoc: false,  // Speed: skip APOC for auto-snapshots
        snapshot_dir: None,
    }).await?;
}
```

### 7.2 seed.sh Integration

Add optional pre-seed snapshot in `packages/db/seed.sh`:

```bash
# [0/5] Pre-Seed Snapshot (if novanet binary available)
if [ -f "$NOVANET_BINARY" ] && [ "${SKIP_SNAPSHOT:-0}" != "1" ]; then
    echo -e "${YELLOW}[0/5] Creating pre-seed snapshot...${NC}"
    "$NOVANET_BINARY" snapshot create --description="Auto: pre-seed" --include-apoc=false
fi
```

### 7.3 Existing Backup System Coexistence

The existing `novanet backup` command backs up `brain/` (YAML models). The new systems complement it:

| System | What | Where | When |
|--------|------|-------|------|
| `novanet backup` | brain/ YAML | `~/.novanet/backups/` | Manual |
| `novanet data export` | Neo4j → YAML | `~/.novanet/export/` | Manual/Incremental |
| `novanet snapshot` | Docker volume | `~/.novanet/snapshots/` | Manual/Auto pre-reset |

---

## 8. YAML File Format Standard

### 8.1 Entity Export

```yaml
# Class: Entity | Project: qrcode-ai
# Exported: 2026-03-13T14:30:22Z | Source: Neo4j
---
entities:
  - key: "qr-code"
    display_name: "QR Code"
    content: "A two-dimensional barcode that stores information..."
    node_class: "Entity"
    llm_context: |
      USE: when referencing the concept of QR codes in any content.
      TRIGGERS: "QR code", "QR", "barcode", "2D code".
      NOT: for specific implementations (use Page instead).
    provenance:
      source: seed
      version: "0.19.0"
    created_at: "2026-02-15T10:00:00Z"
    updated_at: "2026-03-12T16:45:00Z"
```

### 8.2 EntityNative Export (grouped by locale)

```yaml
# Class: EntityNative | Project: qrcode-ai | Locale: fr-FR
# Exported: 2026-03-13T14:30:22Z | Source: Neo4j
---
entity_natives:
  - key: "qr-code@fr-FR"
    entity_key: "qr-code"
    locale: "fr-FR"
    display_name: "Code QR"
    content: "Un code QR est un code-barres bidimensionnel..."
    denomination_forms:
      - { type: text, value: "code QR", priority: 1 }
      - { type: title, value: "Code QR", priority: 1 }
      - { type: abbrev, value: "QR", priority: 1 }
      - { type: url, value: "code-qr", priority: 1 }
    provenance:
      source: nika
      workflow: "00-entity-native-bootstrap"
      generated_at: "2026-03-12T10:00:00Z"
```

### 8.3 Page Export

```yaml
# Class: Page | Project: qrcode-ai
# Exported: 2026-03-13T14:30:22Z | Source: Neo4j
---
pages:
  - key: "qrcode-ai:home"
    display_name: "Home"
    content: "Landing page for QR Code AI"
    node_class: "Page"
    entity_key: "qr-code"
    slug_template: "/"
    provenance:
      source: seed
```

---

## 9. Dependencies

### New Cargo Dependencies

```toml
# In tools/novanet/Cargo.toml [dependencies]
similar = "2.6"     # Unified diff for YAML comparison (promote --interactive)
```

All other dependencies are already present: `serde_yaml`, `tokio` (with `process` feature), `chrono`, `indicatif`, `colored`, `serde_json`, `flate2`, `tar`.

---

## 10. Testing Strategy

### 10.1 Unit Tests

| Module | Tests | What |
|--------|-------|------|
| `data_export.rs` | 15 | Query building, YAML serialization, checkpoint save/load |
| `data_promote.rs` | 12 | Merge logic, diff display, conflict resolution |
| `data_diff.rs` | 8 | Comparison logic, count aggregation |
| `snapshot.rs` | 10 | Metadata serialization, path resolution, validation |
| `checkpoint.rs` | 6 | Save/load/incremental logic |
| `BackupStatus` | 5 | Color coding, age labels, most_recent logic |

### 10.2 Integration Tests

```bash
# E2E: Export → Promote cycle
cargo test --test data_integration -- --ignored

# E2E: Snapshot round-trip (requires Docker)
cargo test --test snapshot_integration -- --ignored
```

### 10.3 Test Data

Use existing seed data for testing. No mock Neo4j needed for unit tests — mock the query results with static YAML.

---

## 11. Implementation Phases

### Phase 1: Foundation — Core Export

**Scope**: `novanet data export` command with basic functionality.

| Task | File | Description |
|------|------|-------------|
| 1.1 | `commands/data_export.rs` | Create DataExportArgs struct with clap derive |
| 1.2 | `commands/data_export.rs` | Implement Neo4j query builder with class/project/locale filters |
| 1.3 | `commands/data_export.rs` | Implement YAML serialization matching private-data format |
| 1.4 | `core/checkpoint.rs` | Checkpoint save/load for incremental exports |
| 1.5 | `commands/data_export.rs` | Progress bar with indicatif |
| 1.6 | `main.rs` | Wire DataExport command |
| 1.7 | Tests | 15 unit tests for export logic |

### Phase 2: Promotion Pipeline

**Scope**: `novanet data promote` and `novanet data diff` commands.

| Task | File | Description |
|------|------|-------------|
| 2.1 | `Cargo.toml` | Add `similar` dependency |
| 2.2 | `commands/data_promote.rs` | Implement YAML file merge logic |
| 2.3 | `commands/data_promote.rs` | Interactive diff display with `similar` |
| 2.4 | `commands/data_promote.rs` | Confirmation prompts (y/N/all/skip) |
| 2.5 | `commands/data_diff.rs` | Neo4j vs YAML comparison |
| 2.6 | `main.rs` | Wire Promote and Diff commands |
| 2.7 | Tests | 20 unit tests for promote + diff |

### Phase 3: Docker Snapshots

**Scope**: `novanet snapshot` commands (create/list/restore/prune).

| Task | File | Description |
|------|------|-------------|
| 3.1 | `commands/snapshot.rs` | Docker stop/start/tar/untar helpers |
| 3.2 | `commands/snapshot.rs` | Snapshot create with progress |
| 3.3 | `commands/snapshot.rs` | Snapshot restore with confirmation |
| 3.4 | `commands/snapshot.rs` | Snapshot list with metadata display |
| 3.5 | `commands/snapshot.rs` | Snapshot prune with keep/dry-run |
| 3.6 | `main.rs` | Wire Snapshot commands |
| 3.7 | Tests | 10 unit tests for snapshot operations |

### Phase 4: TUI Integration

**Scope**: Backup status indicator in TUI status bar.

| Task | File | Description |
|------|------|-------------|
| 4.1 | `tui/app.rs` | Add `BackupStatus` to App state |
| 4.2 | `tui/ui/status.rs` | Render backup indicator (green/yellow/red dot + age) |
| 4.3 | `tui/app.rs` | Load BackupStatus on TUI startup |
| 4.4 | Tests | 5 unit tests for indicator logic |

### Phase 5: Integration & Sync

**Scope**: `novanet data sync`, pre-reset auto-snapshot, documentation.

| Task | File | Description |
|------|------|-------------|
| 5.1 | `commands/data_export.rs` | Implement `sync pull` (export + promote) |
| 5.2 | `commands/data_export.rs` | Implement `sync push` (generate + seed) |
| 5.3 | `main.rs` | Pre-reset auto-snapshot integration |
| 5.4 | CLAUDE.md | Update documentation with new commands |
| 5.5 | Tests | E2E integration tests |

---

## 12. Risk Mitigation

| Risk | Impact | Mitigation |
|------|--------|------------|
| Docker not running | Snapshot fails | Detect early, show error hint with `pnpm infra:up` suggestion |
| Large graph (>100K nodes) | Slow export | Incremental mode + progress bars + class filtering |
| YAML merge conflicts | Data loss | YAML-wins policy, never auto-delete, interactive review |
| Volume tar during write | Corrupted snapshot | Stop Neo4j before tar (container must be stopped) |
| Disk space (snapshots) | Full disk | Auto-prune with configurable keep count |

---

## 13. Future Enhancements (Not in v0.20)

- **Scheduled auto-export**: cron-like scheduling for periodic YAML exports
- **APOC stream mode**: For servers where APOC file export is disabled, use `stream: true` to get Cypher as query result
- **Selective restore**: Restore only specific classes from a snapshot (via APOC import)
- **Remote snapshots**: Push snapshots to S3/GCS for off-site backup
- **TUI snapshot dialog**: Overlay in TUI to create/restore snapshots interactively

---

## Summary

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  DELIVERABLES                                                                   │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Track A: YAML Governance                                                       │
│  ├── novanet data export     Neo4j → YAML (incremental, filtered)              │
│  ├── novanet data promote    YAML → private-data (interactive diff)            │
│  ├── novanet data diff       Neo4j vs YAML comparison                          │
│  └── novanet data sync       pull/push convenience wrapper                     │
│                                                                                 │
│  Track C: Docker Snapshots                                                      │
│  ├── novanet snapshot create   Binary volume archive + metadata                │
│  ├── novanet snapshot list     Show snapshots with stats                       │
│  ├── novanet snapshot restore  Restore from archive (with confirmation)        │
│  └── novanet snapshot prune    Keep N most recent snapshots                    │
│                                                                                 │
│  TUI: Backup indicator        Green/Yellow/Red dot + age in status bar         │
│                                                                                 │
│  New files: 5 | Modified files: 5 | Est. lines: ~2,000                        │
│  New tests: ~56 unit + integration                                              │
│  New dependency: similar 2.6                                                    │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```
