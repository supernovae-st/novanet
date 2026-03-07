# Brain Repository Restructure Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Restructure brain/ repo from mixed schema/data organization to tiered architecture (ontology/foundation/projects/workspace) with data protection and versioned backups.

**Architecture:** 3-Tier data architecture with clear separation between regenerable schema (TIER 1), stable reference data (TIER 2), and protected business data (TIER 3). Docker volume persistence for production safety.

**Tech Stack:** Git, Docker, Neo4j, Cypher, Rust CLI (novanet), Bash

---

## Current State Analysis

```
brain/ (current)
├── models/              # YAML schemas (59 NodeClasses, 174 ArcClasses)
│   ├── node-classes/
│   ├── arc-classes/
│   ├── taxonomy.yaml
│   └── visual-encoding.yaml
├── seed/                # MIXED: schema + reference + business Cypher
│   ├── 00-constraints.cypher      # TIER 1: Regenerable
│   ├── 00.5-taxonomy.cypher       # TIER 1: Regenerable
│   ├── 01-classes.cypher          # TIER 1: Regenerable
│   ├── 02-arc-classes.cypher      # TIER 1: Regenerable
│   ├── 02.5-entity-categories.cypher # TIER 2: Reference
│   ├── 10-11-*.cypher             # TIER 3: Business (Entity bootstrap)
│   ├── 20-29-*.cypher             # TIER 2: Reference (Locales, Geography)
│   ├── 30-37-*.cypher             # TIER 3: Business (Project, Entities)
│   └── 99-autowire-classes.cypher # TIER 1: Regenerable
├── data/                # Reference/business data (unclear)
├── seo/                 # Working files
├── sessions/            # Working files
└── workflows/           # Working files
```

**Symlinks in novanet:**
- `packages/core/models` → `../../../brain/models`
- `packages/db/seed` → `../../../brain/seed`

---

## Target State

```
brain/
├── ontology/            # TIER 1: Schema (100% regenerable from YAML)
│   ├── node-classes/
│   ├── arc-classes/
│   ├── arc-families/
│   ├── layers/
│   ├── realms/
│   ├── traits/
│   ├── mixins/
│   ├── config/
│   ├── docs/
│   ├── taxonomy.yaml
│   ├── visual-encoding.yaml
│   ├── views.yaml
│   ├── _index.yaml
│   ├── README.md
│   └── _generated/      # Auto-generated Cypher (gitignored)
│       ├── 00-constraints.cypher
│       ├── 00.5-taxonomy.cypher
│       ├── 01-classes.cypher
│       ├── 02-arc-classes.cypher
│       └── 99-autowire-classes.cypher
│
├── foundation/          # TIER 2: Reference data (stable, universal)
│   ├── 02.5-entity-categories.cypher
│   ├── 20-locales.cypher
│   ├── 22-slugification.cypher
│   ├── 23-formatting.cypher
│   ├── 24-culture.cypher
│   ├── 26-expression.cypher
│   ├── 27-geographic-taxonomy.cypher
│   ├── 27-terms.cypher
│   ├── 28-locale-taxonomy-links.cypher
│   └── 29-countries.cypher
│
├── projects/            # TIER 3: Business data (PROTECTED)
│   └── qrcode-ai/       # Per-project organization
│       ├── 10-entities-bootstrap.cypher
│       ├── 11-entity-native-bootstrap.cypher
│       ├── 30-org-config.cypher
│       ├── 31-project-qrcode-ai.cypher
│       ├── 32-entity-native-remediation.cypher
│       ├── 32.5-entity-semantic-arcs.cypher
│       ├── 34-prompts.cypher
│       ├── 35-semantic-links.cypher
│       └── 37-entity-hierarchy.cypher
│
├── workspace/           # Working files (gitignored heavy files)
│   ├── seo/             # SEO research
│   ├── sessions/        # Claude sessions
│   └── archive/         # Deprecated files
│
└── README.md            # Documentation for brain/ structure
```

---

## Task 1: Create Directory Structure

**Files:**
- Create: `brain/ontology/` (move from models/)
- Create: `brain/ontology/_generated/`
- Create: `brain/foundation/`
- Create: `brain/projects/qrcode-ai/`
- Create: `brain/workspace/`

**Step 1: Create new directories**

```bash
cd /Users/thibaut/dev/supernovae/brain
mkdir -p ontology/_generated
mkdir -p foundation
mkdir -p projects/qrcode-ai
mkdir -p workspace/seo
mkdir -p workspace/sessions
mkdir -p workspace/archive
```

**Step 2: Verify directories exist**

Run: `ls -la /Users/thibaut/dev/supernovae/brain/`
Expected: All new directories visible

**Step 3: Commit structure (empty)**

```bash
# Create .gitkeep files for empty dirs
touch ontology/_generated/.gitkeep
touch projects/qrcode-ai/.gitkeep
touch workspace/archive/.gitkeep

git add .
git commit -m "chore(brain): create tiered directory structure"
```

---

## Task 2: Move YAML Schemas to ontology/

**Files:**
- Move: `brain/models/*` → `brain/ontology/`
- Delete: `brain/models/` (after move)

**Step 1: Move all YAML schema files**

```bash
cd /Users/thibaut/dev/supernovae/brain

# Move everything from models/ to ontology/
mv models/* ontology/

# Remove empty models directory
rmdir models
```

**Step 2: Verify move**

Run: `ls -la ontology/`
Expected: All YAML files and subdirectories present (node-classes/, arc-classes/, taxonomy.yaml, etc.)

**Step 3: Commit move**

```bash
git add .
git commit -m "refactor(brain): move YAML schemas to ontology/"
```

---

## Task 3: Separate Seed Files by Tier

**Files:**
- Move: `seed/00-*.cypher`, `seed/01-*.cypher`, `seed/02-arc-classes.cypher`, `seed/99-*.cypher` → `ontology/_generated/`
- Move: `seed/02.5-*.cypher`, `seed/20-29-*.cypher` → `foundation/`
- Move: `seed/10-*.cypher`, `seed/11-*.cypher`, `seed/30-*.cypher` → `projects/qrcode-ai/`

**Step 1: Move TIER 1 (Regenerable) to ontology/_generated/**

```bash
cd /Users/thibaut/dev/supernovae/brain

# Schema-generated files
mv seed/00-constraints.cypher ontology/_generated/
mv seed/00.5-taxonomy.cypher ontology/_generated/
mv seed/01-classes.cypher ontology/_generated/
mv seed/02-arc-classes.cypher ontology/_generated/
mv seed/99-autowire-classes.cypher ontology/_generated/
```

**Step 2: Move TIER 2 (Foundation) to foundation/**

```bash
# Reference data (stable, universal)
mv seed/02.5-entity-categories.cypher foundation/
mv seed/20-locales.cypher foundation/
mv seed/22-slugification.cypher foundation/
mv seed/23-formatting.cypher foundation/
mv seed/24-culture.cypher foundation/
mv seed/26-expression.cypher foundation/
mv seed/27-geographic-taxonomy.cypher foundation/
mv seed/27-terms.cypher foundation/
mv seed/28-locale-taxonomy-links.cypher foundation/
mv seed/29-countries.cypher foundation/
mv seed/29.5-locale-country-links.cypher foundation/
```

**Step 3: Move TIER 3 (Projects) to projects/qrcode-ai/**

```bash
# Business data (protected)
mv seed/10-entities-bootstrap.cypher projects/qrcode-ai/
mv seed/11-entity-native-bootstrap.cypher projects/qrcode-ai/
mv seed/30-org-config.cypher projects/qrcode-ai/
mv seed/31-project-qrcode-ai.cypher projects/qrcode-ai/
mv seed/32-entity-native-remediation.cypher projects/qrcode-ai/
mv seed/32.5-entity-semantic-arcs.cypher projects/qrcode-ai/
mv seed/34-prompts.cypher projects/qrcode-ai/
mv seed/35-semantic-links.cypher projects/qrcode-ai/
mv seed/37-entity-hierarchy.cypher projects/qrcode-ai/
```

**Step 4: Move deprecated/archive files**

```bash
# Archive
mv seed/_archive workspace/archive/seed-archive
mv seed/27-geographic-taxonomy.yaml workspace/archive/
mv seed/36-*.deprecated workspace/archive/
```

**Step 5: Move working files**

```bash
# Working files
mv seo/* workspace/seo/
mv sessions/* workspace/sessions/
rmdir seo sessions

# Keep workflows at root (part of CI/CD)
```

**Step 6: Clean up empty seed directory**

```bash
# Check what's left
ls -la seed/

# If empty, remove
rmdir seed
```

**Step 7: Verify structure**

Run: `find . -type f -name "*.cypher" | head -30`
Expected: Files in ontology/_generated/, foundation/, projects/qrcode-ai/

**Step 8: Commit separation**

```bash
git add .
git commit -m "refactor(brain): separate seed files into tiered structure

TIER 1 (ontology/_generated/): Schema Cypher (regenerable)
TIER 2 (foundation/): Reference data (stable)
TIER 3 (projects/): Business data (protected)
"
```

---

## Task 4: Update Symlinks in NovaNet

**Files:**
- Update: `novanet/packages/core/models` symlink
- Update: `novanet/packages/db/seed` symlink (needs new approach)

**Step 1: Update models symlink**

```bash
cd /Users/thibaut/dev/supernovae/novanet/packages/core

# Remove old symlink
rm models

# Create new symlink to ontology/
ln -s ../../../brain/ontology models
```

**Step 2: Verify models symlink**

Run: `ls -la /Users/thibaut/dev/supernovae/novanet/packages/core/models`
Expected: Symlink pointing to `../../../brain/ontology`

Run: `ls /Users/thibaut/dev/supernovae/novanet/packages/core/models/taxonomy.yaml`
Expected: File exists

**Step 3: Update seed symlink strategy**

The seed symlink is more complex because files are now in 3 locations. Two options:

**Option A: Create aggregated seed/ directory with symlinks**

```bash
cd /Users/thibaut/dev/supernovae/brain

# Create aggregated seed directory
mkdir -p seed

# Symlink tier directories
ln -s ../ontology/_generated seed/schema
ln -s ../foundation seed/foundation
ln -s ../projects seed/projects
```

**Option B: Update novanet to use new paths directly**

This requires updating the Rust CLI to read from multiple directories. More work but cleaner.

**For now, use Option A:**

```bash
cd /Users/thibaut/dev/supernovae/brain
mkdir -p seed
ln -s ../ontology/_generated seed/schema
ln -s ../foundation seed/foundation
ln -s ../projects seed/projects
```

**Step 4: Update db seed command (later task)**

The `novanet db seed` command needs to be updated to:
1. Run schema files first (ontology/_generated/)
2. Run foundation files (foundation/)
3. Run project files (projects/qrcode-ai/)

**Step 5: Commit symlink changes**

```bash
cd /Users/thibaut/dev/supernovae/novanet
git add .
git commit -m "refactor(novanet): update symlinks for brain restructure"
```

---

## Task 5: Update .gitignore Files

**Files:**
- Modify: `brain/.gitignore`
- Create: `brain/ontology/_generated/.gitignore`
- Create: `brain/workspace/.gitignore`

**Step 1: Create ontology/_generated/.gitignore**

```bash
cat > /Users/thibaut/dev/supernovae/brain/ontology/_generated/.gitignore << 'EOF'
# Generated Cypher files - can be regenerated from YAML
# Run: novanet schema generate
*.cypher
!.gitkeep
EOF
```

**Step 2: Create workspace/.gitignore**

```bash
cat > /Users/thibaut/dev/supernovae/brain/workspace/.gitignore << 'EOF'
# Large research files
*.jsonl
*.json
!*.schema.json

# Session files (can be large)
sessions/**/*.md
!sessions/README.md

# SEO research cache
seo/cache/
EOF
```

**Step 3: Update brain/.gitignore**

```bash
cat > /Users/thibaut/dev/supernovae/brain/.gitignore << 'EOF'
# Node modules (if any)
node_modules/

# OS files
.DS_Store

# Editor files
*.swp
*.swo

# Generated files are tracked in ontology/_generated/.gitignore
# Large workspace files are tracked in workspace/.gitignore
EOF
```

**Step 4: Commit gitignore updates**

```bash
cd /Users/thibaut/dev/supernovae/brain
git add .
git commit -m "chore(brain): add gitignore files for tiered structure"
```

---

## Task 6: Create README Documentation

**Files:**
- Create: `brain/README.md`

**Step 1: Write README**

```bash
cat > /Users/thibaut/dev/supernovae/brain/README.md << 'EOF'
# Brain Repository

Knowledge graph schema and data for NovaNet.

## Directory Structure

```
brain/
├── ontology/         # TIER 1: Schema (regenerable from YAML)
│   ├── node-classes/ # 59 NodeClass definitions
│   ├── arc-classes/  # 174 ArcClass definitions
│   ├── taxonomy.yaml # Realms, Layers, Traits
│   └── _generated/   # Auto-generated Cypher (gitignored)
│
├── foundation/       # TIER 2: Reference data (stable, universal)
│   ├── locales       # 200+ BCP-47 locales
│   ├── geography     # Countries, regions, clusters
│   ├── expressions   # Native idiomatic expressions
│   └── terms         # SEO terminology per locale
│
├── projects/         # TIER 3: Business data (PROTECTED)
│   └── qrcode-ai/    # QR Code AI project data
│
└── workspace/        # Working files (partially gitignored)
    ├── seo/          # SEO research
    └── sessions/     # Claude sessions
```

## Data Tiers

| Tier | Directory | Persistence | Backup | Reset Behavior |
|------|-----------|-------------|--------|----------------|
| **1** | `ontology/_generated/` | Regenerable | Not needed | Regenerated |
| **2** | `foundation/` | Git | Git history | Re-seeded |
| **3** | `projects/` | Git + Volume | Daily backup | **PROTECTED** |

## Commands

```bash
# Regenerate schema Cypher from YAML
novanet schema generate

# Seed database (all tiers)
novanet db seed

# Seed specific tier
novanet db seed --tier=foundation
novanet db seed --tier=projects

# Reset database (TIER 3 PROTECTED)
novanet db reset              # Resets TIER 1+2 only
novanet db reset --force-all  # Resets ALL (requires confirmation)

# Backup TIER 3 data
novanet db backup --tier=projects

# Restore TIER 3 data
novanet db restore --file=backup-2026-03-07.cypher
```

## Symlinks

NovaNet references brain/ via symlinks:

```
novanet/packages/core/models → ../../../brain/ontology
novanet/packages/db/seed     → ../../../brain/seed
```

## Versioning

- **TIER 1**: Version controlled via YAML (source of truth)
- **TIER 2**: Version controlled via Cypher files
- **TIER 3**: Version controlled + Docker volume backup

## Related

- [NovaNet CLAUDE.md](../novanet/CLAUDE.md) - Main project context
- [ADR-024](../dx/adr/novanet/adr-024-trait-data-origin.md) - Data origin traits
EOF
```

**Step 2: Commit README**

```bash
git add README.md
git commit -m "docs(brain): add README with tiered structure documentation"
```

---

## Task 7: Update Rust CLI for Tiered Seeding

**Files:**
- Modify: `tools/novanet/src/commands/db.rs`

**Step 1: Read current db.rs**

First, understand the current implementation.

**Step 2: Add --tier flag to seed command**

```rust
// In db.rs, update SeedArgs
#[derive(Args)]
pub struct SeedArgs {
    /// Tier to seed (schema, foundation, projects, all)
    #[arg(long, default_value = "all")]
    tier: String,

    /// Project name (required for tier=projects)
    #[arg(long)]
    project: Option<String>,
}
```

**Step 3: Implement tiered seeding logic**

```rust
pub async fn seed(args: &SeedArgs, config: &Config) -> Result<()> {
    let brain_root = config.project_root.join("../brain");

    let tiers_to_seed = match args.tier.as_str() {
        "schema" => vec!["ontology/_generated"],
        "foundation" => vec!["foundation"],
        "projects" => {
            let project = args.project.as_ref()
                .ok_or_else(|| anyhow!("--project required for tier=projects"))?;
            vec![format!("projects/{}", project)]
        }
        "all" => vec![
            "ontology/_generated",
            "foundation",
            "projects/qrcode-ai", // TODO: discover projects
        ],
        _ => return Err(anyhow!("Invalid tier: {}", args.tier)),
    };

    for tier_path in tiers_to_seed {
        seed_directory(&brain_root.join(tier_path), &config.neo4j).await?;
    }

    Ok(())
}
```

**Step 4: Test tiered seeding**

Run: `cargo run -- db seed --tier=schema`
Expected: Only schema files seeded

Run: `cargo run -- db seed --tier=foundation`
Expected: Only foundation files seeded

**Step 5: Commit CLI changes**

```bash
git add .
git commit -m "feat(cli): add --tier flag to db seed command"
```

---

## Task 8: Add Reset Protection for TIER 3

**Files:**
- Modify: `tools/novanet/src/commands/db.rs`

**Step 1: Update reset command**

```rust
#[derive(Args)]
pub struct ResetArgs {
    /// Force reset ALL data including TIER 3 (requires confirmation)
    #[arg(long)]
    force_all: bool,
}

pub async fn reset(args: &ResetArgs, config: &Config) -> Result<()> {
    if args.force_all {
        // Prompt for confirmation
        println!("⚠️  WARNING: This will delete ALL data including TIER 3 (business data)");
        println!("   Type 'yes-delete-all' to confirm:");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if input.trim() != "yes-delete-all" {
            println!("Reset cancelled.");
            return Ok(());
        }

        // Full reset
        drop_all_data(&config.neo4j).await?;
        seed_all_tiers(&config).await?;
    } else {
        // Safe reset: only TIER 1+2
        drop_tier_1_2(&config.neo4j).await?;
        seed_tier_1_2(&config).await?;
        println!("✅ TIER 1+2 reset. TIER 3 data preserved.");
    }

    Ok(())
}
```

**Step 2: Test reset protection**

Run: `cargo run -- db reset`
Expected: Only TIER 1+2 reset, TIER 3 preserved

Run: `cargo run -- db reset --force-all`
Expected: Prompts for confirmation

**Step 3: Commit reset protection**

```bash
git add .
git commit -m "feat(cli): add TIER 3 protection to db reset"
```

---

## Task 9: Add Backup/Restore Commands

**Files:**
- Create: `tools/novanet/src/commands/backup.rs`
- Modify: `tools/novanet/src/commands/mod.rs`

**Step 1: Create backup command**

```rust
// backup.rs
use clap::Args;

#[derive(Args)]
pub struct BackupArgs {
    /// Tier to backup (projects, foundation, all)
    #[arg(long, default_value = "projects")]
    tier: String,

    /// Output file path
    #[arg(short, long)]
    output: Option<PathBuf>,
}

pub async fn backup(args: &BackupArgs, config: &Config) -> Result<()> {
    let timestamp = chrono::Utc::now().format("%Y-%m-%d-%H%M%S");
    let filename = args.output.clone().unwrap_or_else(|| {
        PathBuf::from(format!("backup-{}-{}.cypher", args.tier, timestamp))
    });

    let cypher = match args.tier.as_str() {
        "projects" => export_tier_3_data(&config.neo4j).await?,
        "foundation" => export_tier_2_data(&config.neo4j).await?,
        "all" => export_all_data(&config.neo4j).await?,
        _ => return Err(anyhow!("Invalid tier")),
    };

    std::fs::write(&filename, cypher)?;
    println!("✅ Backup saved to: {}", filename.display());

    Ok(())
}
```

**Step 2: Create restore command**

```rust
#[derive(Args)]
pub struct RestoreArgs {
    /// Backup file to restore
    #[arg(short, long)]
    file: PathBuf,

    /// Skip confirmation prompt
    #[arg(long)]
    yes: bool,
}

pub async fn restore(args: &RestoreArgs, config: &Config) -> Result<()> {
    if !args.yes {
        println!("⚠️  This will restore data from: {}", args.file.display());
        println!("   Existing data may be overwritten. Continue? [y/N]");
        // ... confirmation logic
    }

    let cypher = std::fs::read_to_string(&args.file)?;
    execute_cypher(&config.neo4j, &cypher).await?;

    println!("✅ Restore complete");
    Ok(())
}
```

**Step 3: Test backup/restore**

Run: `cargo run -- db backup --tier=projects`
Expected: Creates backup-projects-TIMESTAMP.cypher

Run: `cargo run -- db restore --file=backup-projects-*.cypher`
Expected: Restores data from backup

**Step 4: Commit backup/restore**

```bash
git add .
git commit -m "feat(cli): add db backup and db restore commands"
```

---

## Task 10: Configure Docker Volume Persistence

**Files:**
- Modify: `packages/db/docker-compose.yml`

**Step 1: Update docker-compose.yml**

```yaml
version: '3.8'

services:
  neo4j:
    image: neo4j:5.26-community
    container_name: novanet-neo4j
    ports:
      - "7474:7474"
      - "7687:7687"
    environment:
      - NEO4J_AUTH=neo4j/${NEO4J_PASSWORD:-novanetpassword}
      - NEO4J_PLUGINS=["apoc"]
    volumes:
      # Named volume for data persistence (survives container restart)
      - novanet-data:/data
      # Named volume for logs
      - novanet-logs:/logs
      # Bind mount for imports (optional)
      - ./import:/import

volumes:
  novanet-data:
    name: novanet-neo4j-data
  novanet-logs:
    name: novanet-neo4j-logs
```

**Step 2: Verify volume persistence**

Run: `docker volume ls | grep novanet`
Expected: novanet-neo4j-data, novanet-neo4j-logs

**Step 3: Test persistence across restart**

```bash
# Add test data
docker exec novanet-neo4j cypher-shell -u neo4j -p novanetpassword \
  "CREATE (t:Test {name: 'persistence-test', created: datetime()})"

# Restart container
docker compose down
docker compose up -d

# Verify data persists
docker exec novanet-neo4j cypher-shell -u neo4j -p novanetpassword \
  "MATCH (t:Test) RETURN t.name"
```

Expected: `persistence-test` returned

**Step 4: Commit Docker changes**

```bash
git add .
git commit -m "feat(db): configure Docker named volumes for data persistence"
```

---

## Task 11: Push All Changes

**Step 1: Push brain repo**

```bash
cd /Users/thibaut/dev/supernovae/brain
git push origin main
```

**Step 2: Push novanet repo**

```bash
cd /Users/thibaut/dev/supernovae/novanet
git push origin main
```

**Step 3: Verify GitHub**

Check that both repos are updated with new structure.

---

## Summary

| Task | Description | Files Changed |
|------|-------------|---------------|
| 1 | Create directory structure | brain/ |
| 2 | Move YAML to ontology/ | brain/models → brain/ontology |
| 3 | Separate seed by tier | brain/seed → 3 locations |
| 4 | Update novanet symlinks | packages/core/models, packages/db/seed |
| 5 | Update .gitignore | brain/.gitignore, ontology/_generated/.gitignore |
| 6 | Create README | brain/README.md |
| 7 | Add --tier flag to seed | tools/novanet/src/commands/db.rs |
| 8 | Add reset protection | tools/novanet/src/commands/db.rs |
| 9 | Add backup/restore | tools/novanet/src/commands/backup.rs |
| 10 | Docker volume persistence | packages/db/docker-compose.yml |
| 11 | Push all changes | Git |

---

## Verification Checklist

After implementation, verify:

- [ ] `ls brain/ontology/` shows YAML files
- [ ] `ls brain/ontology/_generated/` shows Cypher files
- [ ] `ls brain/foundation/` shows reference Cypher
- [ ] `ls brain/projects/qrcode-ai/` shows business Cypher
- [ ] `novanet schema generate` regenerates ontology/_generated/
- [ ] `novanet db seed --tier=schema` works
- [ ] `novanet db seed --tier=foundation` works
- [ ] `novanet db reset` preserves TIER 3
- [ ] `novanet db backup --tier=projects` creates backup file
- [ ] Docker data persists across `docker compose down/up`
- [ ] CSR remains at 99.99%+
