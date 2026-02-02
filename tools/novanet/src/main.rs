//! NovaNet CLI + TUI — unified Rust binary for context graph management.
//!
//! Thin entry point: clap parsing → dispatch to lib functions → format output → exit.

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "novanet", version, about = "NovaNet context graph CLI")]
struct Cli {
    /// Monorepo root (auto-detected: walks up to find pnpm-workspace.yaml)
    #[arg(long, env = "NOVANET_ROOT")]
    root: Option<PathBuf>,

    /// Neo4j URI
    #[arg(long, env = "NEO4J_URI", default_value = "bolt://localhost:7687")]
    uri: String,

    /// Neo4j user
    #[arg(long, env = "NEO4J_USER", default_value = "neo4j")]
    user: String,

    /// Neo4j password
    #[arg(long, env = "NEO4J_PASSWORD", default_value = "novanetpassword")]
    password: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Mode 1: Data nodes only (WHERE NOT n:Meta)
    Data {
        #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
        format: OutputFormat,
    },
    /// Mode 2: Meta-graph only (MATCH (n:Meta))
    Meta {
        #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
        format: OutputFormat,
    },
    /// Mode 3: Data + Meta overlay
    Overlay {
        #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
        format: OutputFormat,
    },
    /// Mode 4: Facet-driven query
    Query(QueryArgs),
    /// CRUD: node operations
    Node {
        #[command(subcommand)]
        action: NodeAction,
    },
    /// CRUD: relation operations
    Relation {
        #[command(subcommand)]
        action: RelationAction,
    },
    /// Schema operations (generate artifacts, validate sync)
    Schema {
        #[command(subcommand)]
        action: SchemaAction,
    },
    /// Documentation generation (view-specific Mermaid diagrams)
    Doc {
        #[command(subcommand)]
        action: DocAction,
    },
    /// Database operations (seed, migrate, reset)
    Db {
        #[command(subcommand)]
        action: DbAction,
    },
    /// Interactive terminal UI
    #[cfg(feature = "tui")]
    Tui,
}

#[derive(clap::Args)]
struct QueryArgs {
    #[arg(long)]
    realm: Option<String>,
    #[arg(long)]
    layer: Option<String>,
    #[arg(long, name = "trait")]
    trait_filter: Option<String>,
    #[arg(long)]
    edge_family: Option<String>,
    #[arg(long)]
    kind: Option<String>,
    #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
    format: OutputFormat,
}

#[derive(ValueEnum, Clone, Copy)]
enum OutputFormat {
    Table,
    Json,
    Cypher,
}

#[derive(Subcommand)]
enum NodeAction {
    /// Create a new node
    Create,
    /// Edit an existing node
    Edit,
    /// Delete a node
    Delete,
}

#[derive(Subcommand)]
enum RelationAction {
    /// Create a new relation
    Create,
    /// Delete a relation
    Delete,
}

#[derive(Subcommand)]
enum SchemaAction {
    /// Generate all artifacts (Cypher, TypeScript, Mermaid) from YAML
    Generate {
        /// Dry-run: validate output without writing files
        #[arg(long)]
        dry_run: bool,
    },
    /// Validate YAML ↔ Neo4j sync
    Validate {
        /// Strict mode: fail on warnings
        #[arg(long)]
        strict: bool,
    },
}

#[derive(Subcommand)]
enum DocAction {
    /// Generate Mermaid diagrams for views
    Generate {
        /// Generate only this view (by ID)
        #[arg(long)]
        view: Option<String>,
        /// Dry-run: generate without writing files
        #[arg(long)]
        dry_run: bool,
        /// List available views instead of generating
        #[arg(long)]
        list: bool,
    },
}

#[derive(Subcommand)]
enum DbAction {
    /// Seed the database
    Seed,
    /// Run migrations
    Migrate,
    /// Reset database (drop + seed)
    Reset,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    // Resolve monorepo root for commands that need YAML access
    let root = novanet::config::resolve_root(cli.root.as_deref());

    match cli.command {
        Commands::Schema { action } => match action {
            SchemaAction::Generate { dry_run } => {
                let root = root?;
                eprintln!(
                    "novanet schema generate{} (root: {})",
                    if dry_run { " --dry-run" } else { "" },
                    root.display()
                );
                let results = novanet::commands::schema::schema_generate(&root, dry_run)?;
                for r in &results {
                    eprintln!(
                        "  {} {} ({} bytes, {}ms)",
                        if dry_run { "would write" } else { "wrote" },
                        r.output_path,
                        r.bytes,
                        r.duration_ms,
                    );
                }
                eprintln!(
                    "\n{} {} artifact(s)",
                    if dry_run {
                        "Would generate"
                    } else {
                        "Generated"
                    },
                    results.len()
                );
            }
            SchemaAction::Validate { strict } => {
                let root = root?;
                eprintln!(
                    "novanet schema validate{} (root: {})",
                    if strict { " --strict" } else { "" },
                    root.display()
                );
                let issues = novanet::commands::schema::schema_validate(&root)?;
                let errors: Vec<_> = issues
                    .iter()
                    .filter(|i| i.severity == novanet::commands::schema::Severity::Error)
                    .collect();
                let warnings: Vec<_> = issues
                    .iter()
                    .filter(|i| i.severity == novanet::commands::schema::Severity::Warning)
                    .collect();

                for issue in &issues {
                    let tag = match issue.severity {
                        novanet::commands::schema::Severity::Error => "ERROR",
                        novanet::commands::schema::Severity::Warning => "WARN",
                    };
                    eprintln!("  [{}] {}", tag, issue.message);
                }

                eprintln!("\n{} error(s), {} warning(s)", errors.len(), warnings.len());

                if !errors.is_empty() || (strict && !warnings.is_empty()) {
                    std::process::exit(1);
                }
            }
        },
        Commands::Doc { action } => match action {
            DocAction::Generate {
                view,
                dry_run,
                list,
            } => {
                let root = root?;
                if list {
                    eprintln!("novanet doc generate --list (root: {})", root.display());
                    let entries = novanet::commands::doc::doc_list(&root)?;
                    eprintln!();
                    eprintln!("  {:<30} {:<14} DESCRIPTION", "VIEW ID", "CATEGORY");
                    eprintln!("  {:<30} {:<14} ───────────", "───────", "────────");
                    for e in &entries {
                        eprintln!("  {:<30} {:<14} {}", e.id, e.category, e.description);
                    }
                    eprintln!("\n  {} view(s) available", entries.len());
                } else {
                    eprintln!(
                        "novanet doc generate{}{} (root: {})",
                        view.as_ref()
                            .map(|v| format!(" --view={v}"))
                            .unwrap_or_default(),
                        if dry_run { " --dry-run" } else { "" },
                        root.display()
                    );
                    let results =
                        novanet::commands::doc::doc_generate(&root, view.as_deref(), dry_run)?;
                    for r in &results {
                        eprintln!(
                            "  {} {} ({} bytes, {}ms)",
                            if dry_run { "would write" } else { "wrote" },
                            r.output_path,
                            r.bytes,
                            r.duration_ms,
                        );
                    }
                    eprintln!(
                        "\n{} {} view doc(s)",
                        if dry_run {
                            "Would generate"
                        } else {
                            "Generated"
                        },
                        results.len()
                    );
                }
            }
        },
        Commands::Data { format: _ } => eprintln!("TODO: Phase 7 — data command"),
        Commands::Meta { format: _ } => eprintln!("TODO: Phase 7 — meta command"),
        Commands::Overlay { format: _ } => eprintln!("TODO: Phase 7 — overlay command"),
        Commands::Query(_) => eprintln!("TODO: Phase 7 — query command"),
        Commands::Node { action: _ } => eprintln!("TODO: Phase 7 — node command"),
        Commands::Relation { action: _ } => eprintln!("TODO: Phase 7 — relation command"),
        Commands::Db { action: _ } => eprintln!("TODO: Phase 7 — db command"),
        #[cfg(feature = "tui")]
        Commands::Tui => eprintln!("TODO: Phase 7 — TUI"),
    }

    Ok(())
}
