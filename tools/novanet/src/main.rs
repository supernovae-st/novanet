//! NovaNet CLI + TUI — unified Rust binary for context graph management.
//!
//! Thin entry point: clap parsing → dispatch to lib functions → format output → exit.

use clap::{Parser, Subcommand};
use novanet::output::OutputFormat;
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
    /// Filter operations (JSON stdin → Cypher stdout for Studio)
    Filter {
        #[command(subcommand)]
        action: FilterAction,
    },
    /// Search nodes by text (fulltext + property match)
    Search {
        /// Search query string
        #[arg(long)]
        query: String,
        /// Filter by Kind label
        #[arg(long)]
        kind: Option<String>,
        /// Maximum results
        #[arg(long, default_value_t = 50)]
        limit: i64,
        #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
        format: OutputFormat,
    },
    /// Locale operations (list, import)
    Locale {
        #[command(subcommand)]
        action: LocaleAction,
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

#[derive(Subcommand)]
enum NodeAction {
    /// Create a new node (auto-wires OF_KIND)
    Create {
        /// Kind label (e.g., Page, Concept, Project)
        #[arg(long)]
        kind: String,
        /// Unique key for the node
        #[arg(long)]
        key: String,
        /// JSON properties: '{"display_name":"...", "description":"..."}'
        #[arg(long, default_value = "{}")]
        props: String,
    },
    /// Edit an existing node (property merge, not replace)
    Edit {
        /// Node key to edit
        #[arg(long)]
        key: String,
        /// JSON properties to merge: '{"display_name":"updated"}'
        #[arg(long, name = "set")]
        set_props: String,
    },
    /// Delete a node (DETACH DELETE — removes all relationships)
    Delete {
        /// Node key to delete
        #[arg(long)]
        key: String,
        /// Required confirmation flag for destructive operation
        #[arg(long)]
        confirm: bool,
    },
}

#[derive(Subcommand)]
enum RelationAction {
    /// Create a relationship between two nodes
    Create {
        /// Source node key
        #[arg(long)]
        from: String,
        /// Target node key
        #[arg(long)]
        to: String,
        /// Relationship type (e.g., FOR_LOCALE, HAS_BLOCK)
        #[arg(long, name = "type")]
        rel_type: String,
        /// Optional JSON properties for the relation
        #[arg(long, default_value = "{}")]
        props: String,
    },
    /// Delete a specific relationship
    Delete {
        /// Source node key
        #[arg(long)]
        from: String,
        /// Target node key
        #[arg(long)]
        to: String,
        /// Relationship type to delete
        #[arg(long, name = "type")]
        rel_type: String,
    },
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
enum FilterAction {
    /// Build Cypher from JSON filter (stdin → stdout, for Studio subprocess)
    Build,
}

#[derive(Subcommand)]
enum LocaleAction {
    /// List locales with their knowledge satellite status
    List {
        #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
        format: OutputFormat,
    },
    /// Import locale data from a Cypher file
    Import {
        /// Path to Cypher file with locale data
        #[arg(long)]
        file: std::path::PathBuf,
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

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    // Resolve monorepo root for commands that need YAML access
    let root = novanet::config::resolve_root(cli.root.as_deref());

    match cli.command {
        // ── Read commands (Neo4j) ────────────────────────────────
        Commands::Data { format } => {
            let db = connect_db(&cli).await?;
            eprintln!("novanet data --format={format:?}");
            novanet::commands::read::run_data(&db, format).await?;
        }
        Commands::Meta { format } => {
            let db = connect_db(&cli).await?;
            eprintln!("novanet meta --format={format:?}");
            novanet::commands::read::run_meta(&db, format).await?;
        }
        Commands::Overlay { format } => {
            let db = connect_db(&cli).await?;
            eprintln!("novanet overlay --format={format:?}");
            novanet::commands::read::run_overlay(&db, format).await?;
        }
        Commands::Query(ref args) => {
            let db = connect_db(&cli).await?;
            let filter = novanet::facets::FacetFilter::from_cli(
                args.realm.as_deref(),
                args.layer.as_deref(),
                args.trait_filter.as_deref(),
                args.edge_family.as_deref(),
                args.kind.as_deref(),
            );
            novanet::commands::read::run_query(&db, filter, args.format).await?;
        }

        // ── Schema + Doc (YAML, no Neo4j) ────────────────────────
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

        // ── Search (Neo4j) ──────────────────────────────────────
        Commands::Search {
            ref query,
            ref kind,
            limit,
            format,
        } => {
            let db = connect_db(&cli).await?;
            eprintln!("novanet search --query={query:?}");
            novanet::commands::search::run_search(&db, query, kind.as_deref(), limit, format)
                .await?;
        }

        // ── Filter (Studio subprocess, no Neo4j) ─────────────────
        Commands::Filter { action } => match action {
            FilterAction::Build => {
                eprintln!("novanet filter build (reading JSON from stdin)");
                novanet::commands::filter::run_filter_build()?;
            }
        },

        // ── Write commands (Neo4j) ──────────────────────────────
        Commands::Node { ref action } => {
            let db = connect_db(&cli).await?;
            match action {
                NodeAction::Create { kind, key, props } => {
                    eprintln!("novanet node create --kind={kind} --key={key}");
                    novanet::commands::node::run_create(&db, kind, key, props).await?;
                }
                NodeAction::Edit { key, set_props } => {
                    eprintln!("novanet node edit --key={key}");
                    novanet::commands::node::run_edit(&db, key, set_props).await?;
                }
                NodeAction::Delete { key, confirm } => {
                    eprintln!("novanet node delete --key={key}");
                    novanet::commands::node::run_delete(&db, key, *confirm).await?;
                }
            }
        }
        Commands::Relation { ref action } => {
            let db = connect_db(&cli).await?;
            match action {
                RelationAction::Create {
                    from,
                    to,
                    rel_type,
                    props,
                } => {
                    eprintln!("novanet relation create --from={from} --to={to} --type={rel_type}");
                    novanet::commands::relation::run_create(&db, from, to, rel_type, props).await?;
                }
                RelationAction::Delete { from, to, rel_type } => {
                    eprintln!("novanet relation delete --from={from} --to={to} --type={rel_type}");
                    novanet::commands::relation::run_delete(&db, from, to, rel_type).await?;
                }
            }
        }
        Commands::Locale { ref action } => {
            let db = connect_db(&cli).await?;
            match action {
                LocaleAction::List { format } => {
                    eprintln!("novanet locale list --format={format:?}");
                    novanet::commands::locale::run_list(&db, *format).await?;
                }
                LocaleAction::Import { file } => {
                    eprintln!("novanet locale import --file={}", file.display());
                    novanet::commands::locale::run_import(&db, file).await?;
                }
            }
        }
        Commands::Db { ref action } => {
            let db = connect_db(&cli).await?;
            let root = root?;
            match action {
                DbAction::Seed => {
                    eprintln!("novanet db seed (root: {})", root.display());
                    novanet::commands::db::run_seed(&db, &root).await?;
                }
                DbAction::Migrate => {
                    eprintln!("novanet db migrate (root: {})", root.display());
                    novanet::commands::db::run_migrate(&db, &root).await?;
                }
                DbAction::Reset => {
                    eprintln!("novanet db reset (root: {})", root.display());
                    novanet::commands::db::run_reset(&db, &root).await?;
                }
            }
        }
        #[cfg(feature = "tui")]
        Commands::Tui => {
            let root = root?;
            let db = connect_db(&cli).await?;
            novanet::tui::run(&db, &root).await?;
        }
    }

    Ok(())
}

/// Connect to Neo4j (shared by all commands that need database access).
async fn connect_db(cli: &Cli) -> color_eyre::Result<novanet::db::Db> {
    eprintln!("Connecting to Neo4j at {}...", cli.uri);
    let db = novanet::db::Db::connect(&cli.uri, &cli.user, &cli.password).await?;
    Ok(db)
}
