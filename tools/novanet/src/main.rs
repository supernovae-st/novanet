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

    /// Neo4j password (required for database commands, set via NEO4J_PASSWORD env var)
    #[arg(long, env = "NEO4J_PASSWORD")]
    password: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Comprehensive schema-graph visualization and validation
    Blueprint {
        /// Specific view to render
        #[arg(long, value_enum)]
        view: Option<novanet::blueprint::views::BlueprintView>,
        /// Output format
        #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
        format: OutputFormat,
        /// Skip validation for faster output
        #[arg(long)]
        no_validate: bool,
    },
    /// Mode 2: Data nodes only (WHERE NOT n:Meta)
    Data {
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
    /// CRUD: arc operations
    Arc {
        #[command(subcommand)]
        action: ArcAction,
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
        /// Filter by Class label
        #[arg(long, name = "class")]
        class: Option<String>,
        /// Maximum results (1-10000)
        #[arg(long, default_value_t = 50, value_parser = clap::value_parser!(i64).range(1..=10000))]
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
    /// Knowledge generation from ATH data (slugification, formatting, voice, culture)
    Knowledge {
        #[command(subcommand)]
        action: KnowledgeAction,
    },
    /// Entity data operations (seed, list, validate)
    Entity {
        #[command(subcommand)]
        action: EntityAction,
    },
    /// Export subgraph to various formats (Cypher, JSON, GraphML, CSV)
    Export(novanet::commands::export::ExportArgs),
    /// Views validation and export for cross-validation (TUI/Studio)
    Views {
        #[command(subcommand)]
        action: ViewsAction,
    },
    /// Interactive terminal UI
    #[cfg(feature = "tui")]
    Tui {
        /// Fresh start: regenerate schema + reset database before launching
        #[arg(long)]
        fresh: bool,
    },
    /// Generate shell completions (bash, zsh, fish, powershell, elvish)
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
    /// Run system health checks
    Doctor {
        /// Skip Neo4j connection check (for offline/CI use)
        #[arg(long)]
        skip_db: bool,
        /// Show verbose output
        #[arg(long, short)]
        verbose: bool,
    },
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
    arc_family: Option<String>,
    #[arg(long, name = "class")]
    class: Option<String>,
    #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
    format: OutputFormat,
}

#[derive(Subcommand)]
enum NodeAction {
    /// Create a new node (auto-wires OF_CLASS)
    Create {
        /// Class label (e.g., Page, Entity, Project)
        #[arg(long, name = "class")]
        class: String,
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
enum ArcAction {
    /// Create an arc between two nodes
    Create {
        /// Source node key
        #[arg(long)]
        from: String,
        /// Target node key
        #[arg(long)]
        to: String,
        /// Arc class (e.g., FOR_LOCALE, HAS_BLOCK)
        #[arg(long, name = "class")]
        class: String,
        /// Optional JSON properties for the arc
        #[arg(long, default_value = "{}")]
        props: String,
    },
    /// Delete a specific arc
    Delete {
        /// Source node key
        #[arg(long)]
        from: String,
        /// Target node key
        #[arg(long)]
        to: String,
        /// Arc class to delete
        #[arg(long, name = "class")]
        class: String,
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
    /// Validate Cypher seed files against YAML definitions
    CypherValidate {
        /// Strict mode: fail on warnings
        #[arg(long)]
        strict: bool,
        /// Only validate specific file(s)
        #[arg(long)]
        file: Option<Vec<std::path::PathBuf>>,
    },
    /// Extract schema statistics (node/arc counts) in JSON format
    Stats {
        /// Output format (json or table)
        #[arg(long, value_enum, default_value_t = OutputFormat::Json)]
        format: OutputFormat,
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
    /// Generate 20-locales.cypher from CSV + MD sources
    Generate {
        /// Path to CSV file with 200 locales
        #[arg(long)]
        csv: std::path::PathBuf,
        /// Path to directory with 1-identity/*.md files
        #[arg(long)]
        identity_dir: std::path::PathBuf,
        /// Output path (default: packages/db/seed/20-locales.cypher)
        #[arg(long)]
        output: Option<std::path::PathBuf>,
        /// Dry-run: generate without writing file
        #[arg(long)]
        dry_run: bool,
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
    /// Verify YAML↔Neo4j arc consistency
    Verify,
}

#[derive(Subcommand)]
enum KnowledgeAction {
    /// Generate knowledge seed files from ATH data
    Generate {
        /// Knowledge tier: technical, voice, culture, market, all
        #[arg(long, default_value = "all")]
        tier: String,
        /// Custom ATH data path (default: $NOVANET_ATH_PATH or error if unset)
        #[arg(long)]
        ath_path: Option<String>,
        /// Dry-run: generate without writing files
        #[arg(long)]
        dry_run: bool,
    },
    /// List available knowledge tiers and their status
    List,
}

#[derive(Subcommand)]
enum EntityAction {
    /// Seed entity data from phase YAML files
    Seed {
        /// Project name (e.g., qrcode-ai)
        #[arg(long)]
        project: String,
        /// Specific phase number (seeds all phases if omitted)
        #[arg(long)]
        phase: Option<u32>,
        /// Dry-run: generate Cypher without writing files
        #[arg(long)]
        dry_run: bool,
    },
    /// List available phases for a project
    List {
        /// Project name (e.g., qrcode-ai)
        #[arg(long)]
        project: String,
    },
    /// Validate entity data without generating
    Validate {
        /// Project name (e.g., qrcode-ai)
        #[arg(long)]
        project: String,
    },
}

#[derive(Subcommand)]
enum ViewsAction {
    /// Export views as canonical JSON for cross-validation
    Export {
        /// Output format (only json supported)
        #[arg(long, default_value = "json")]
        format: String,
    },
    /// Validate views match between Rust and TypeScript parsers
    Validate {
        /// Show details for each view
        #[arg(long)]
        verbose: bool,
    },
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let Cli {
        root: cli_root,
        uri,
        user,
        password,
        command,
    } = Cli::parse();

    // Default to TUI when no command is provided
    let command = match command {
        Some(cmd) => cmd,
        None => {
            #[cfg(feature = "tui")]
            {
                Commands::Tui { fresh: false }
            }
            #[cfg(not(feature = "tui"))]
            {
                eprintln!("error: No command provided. Use --help for available commands.");
                eprintln!("       TUI is not available (compile with --features tui).");
                std::process::exit(1);
            }
        }
    };

    // Initialize tracing (skip for TUI mode which has its own terminal handling)
    #[cfg(feature = "tui")]
    let is_tui = matches!(command, Commands::Tui { .. });
    #[cfg(not(feature = "tui"))]
    let is_tui = false;

    if !is_tui {
        use tracing_subscriber::EnvFilter;
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .with_target(false)
            .init();
    }

    // Resolve monorepo root for commands that need YAML access
    let root = novanet::config::resolve_root(cli_root.as_deref());

    match command {
        // ── Blueprint (YAML + optional Neo4j) ────────────────────
        Commands::Blueprint {
            view,
            format,
            no_validate,
        } => {
            let root = root?;
            // Try to connect to Neo4j for full validation, but work without it
            let db = if password.is_some() {
                match connect_db(&uri, &user, password.as_ref()).await {
                    Ok(db) => Some(db),
                    Err(_) => {
                        eprintln!("⚠  Could not connect to Neo4j, running YAML-only mode");
                        None
                    }
                }
            } else {
                None
            };
            novanet::commands::blueprint::run_blueprint(
                db.as_ref(),
                &root,
                view,
                format,
                no_validate,
            )
            .await?;
        }

        // ── Read commands (Neo4j) ────────────────────────────────
        Commands::Data { format } => {
            let db = connect_db(&uri, &user, password.as_ref()).await?;
            eprintln!("novanet data --format={format:?}");
            novanet::commands::read::run_data(&db, format).await?;
        }
        Commands::Overlay { format } => {
            let db = connect_db(&uri, &user, password.as_ref()).await?;
            eprintln!("novanet overlay --format={format:?}");
            novanet::commands::read::run_overlay(&db, format).await?;
        }
        Commands::Query(ref args) => {
            let db = connect_db(&uri, &user, password.as_ref()).await?;
            let filter = novanet::facets::FacetFilter::from_cli(
                args.realm.as_deref(),
                args.layer.as_deref(),
                args.trait_filter.as_deref(),
                args.arc_family.as_deref(),
                args.class.as_deref(),
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
            SchemaAction::Stats { format } => {
                let root = root?;
                eprintln!("novanet schema stats (root: {})", root.display());
                novanet::commands::schema::schema_stats(&root, format)?;
            }
            SchemaAction::CypherValidate { strict, file } => {
                let root = root?;
                eprintln!(
                    "novanet schema cypher-validate{} (root: {})",
                    if strict { " --strict" } else { "" },
                    root.display()
                );
                let issues = novanet::validation::validate_cypher_files(&root, file)?;
                let errors: Vec<_> = issues
                    .iter()
                    .filter(|i| i.severity == novanet::validation::IssueSeverity::Error)
                    .collect();
                let warnings: Vec<_> = issues
                    .iter()
                    .filter(|i| i.severity == novanet::validation::IssueSeverity::Warning)
                    .collect();

                // Print issues with file:line reference
                for issue in &issues {
                    let tag = match issue.severity {
                        novanet::validation::IssueSeverity::Error => "ERROR",
                        novanet::validation::IssueSeverity::Warning => "WARN",
                    };
                    let location = if let Some(line) = issue.line {
                        format!("{}:{}", issue.file.display(), line)
                    } else {
                        issue.file.display().to_string()
                    };
                    eprintln!(
                        "  [{}] [{}] {}: {}",
                        tag, issue.rule, location, issue.message
                    );
                }

                // Print summary
                eprintln!("{}", novanet::validation::format_summary(&issues));

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
            ref class,
            limit,
            format,
        } => {
            let db = connect_db(&uri, &user, password.as_ref()).await?;
            eprintln!("novanet search --query={query:?}");
            novanet::commands::search::run_search(&db, query, class.as_deref(), limit, format)
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
            let db = connect_db(&uri, &user, password.as_ref()).await?;
            match action {
                NodeAction::Create { class, key, props } => {
                    eprintln!("novanet node create --class={class} --key={key}");
                    novanet::commands::node::run_create(&db, class, key, props).await?;
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
        Commands::Arc { ref action } => {
            let db = connect_db(&uri, &user, password.as_ref()).await?;
            match action {
                ArcAction::Create {
                    from,
                    to,
                    class,
                    props,
                } => {
                    eprintln!("novanet arc create --from={from} --to={to} --class={class}");
                    novanet::commands::arc::run_create(&db, from, to, class, props).await?;
                }
                ArcAction::Delete { from, to, class } => {
                    eprintln!("novanet arc delete --from={from} --to={to} --class={class}");
                    novanet::commands::arc::run_delete(&db, from, to, class).await?;
                }
            }
        }
        Commands::Locale { ref action } => match action {
            LocaleAction::List { format } => {
                let db = connect_db(&uri, &user, password.as_ref()).await?;
                eprintln!("novanet locale list --format={format:?}");
                novanet::commands::locale::run_list(&db, *format).await?;
            }
            LocaleAction::Import { file } => {
                let db = connect_db(&uri, &user, password.as_ref()).await?;
                eprintln!("novanet locale import --file={}", file.display());
                novanet::commands::locale::run_import(&db, file).await?;
            }
            LocaleAction::Generate {
                csv,
                identity_dir,
                output,
                dry_run,
            } => {
                let root = root?;
                let output_path = output
                    .clone()
                    .unwrap_or_else(|| root.join("packages/db/seed/20-locales.cypher"));
                eprintln!(
                    "novanet locale generate --csv={} --identity-dir={} --output={}{}",
                    csv.display(),
                    identity_dir.display(),
                    output_path.display(),
                    if *dry_run { " --dry-run" } else { "" }
                );
                novanet::commands::locale::run_generate(csv, identity_dir, &output_path, *dry_run)?;
            }
        },
        Commands::Db { ref action } => {
            let db = connect_db(&uri, &user, password.as_ref()).await?;
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
                DbAction::Verify => {
                    eprintln!("novanet db verify (root: {})", root.display());
                    let result = novanet::commands::db::run_verify(&db, &root).await?;
                    result.print_report();
                    if !result.is_synced() {
                        std::process::exit(1);
                    }
                }
            }
        }
        Commands::Knowledge { ref action } => {
            let root = root?;
            match action {
                KnowledgeAction::Generate {
                    tier,
                    ath_path,
                    dry_run,
                } => {
                    let tier_enum = novanet::commands::knowledge::KnowledgeTier::parse(tier)
                        .ok_or_else(|| {
                            color_eyre::eyre::eyre!(
                                "Invalid tier '{}'. Valid: technical, voice, culture, market, all",
                                tier
                            )
                        })?;

                    eprintln!(
                        "novanet knowledge generate --tier={}{}{}",
                        tier,
                        ath_path
                            .as_ref()
                            .map(|p| format!(" --ath-path={}", p))
                            .unwrap_or_default(),
                        if *dry_run { " --dry-run" } else { "" }
                    );

                    let results = novanet::commands::knowledge::knowledge_generate(
                        &root,
                        tier_enum,
                        ath_path.as_deref(),
                        *dry_run,
                    )?;

                    for r in &results {
                        eprintln!(
                            "  {} {} ({} bytes, {}ms, {} nodes)",
                            if *dry_run { "would write" } else { "wrote" },
                            r.output_path,
                            r.bytes,
                            r.duration_ms,
                            r.node_count
                        );
                    }
                    eprintln!(
                        "\n{} {} knowledge file(s)",
                        if *dry_run {
                            "Would generate"
                        } else {
                            "Generated"
                        },
                        results.len()
                    );
                }
                KnowledgeAction::List => {
                    eprintln!("novanet knowledge list\n");
                    let tiers = novanet::commands::knowledge::knowledge_list();
                    for t in &tiers {
                        eprintln!(
                            "  [{:^8}] {}: {}",
                            t.status.to_uppercase(),
                            t.tier,
                            t.description
                        );
                        for src in &t.sources {
                            eprintln!("            └── {}", src);
                        }
                    }
                    eprintln!("\n  {} tier(s)", tiers.len());
                }
            }
        }
        Commands::Entity { ref action } => {
            let root = root?;
            match action {
                EntityAction::Seed {
                    project,
                    phase,
                    dry_run,
                } => {
                    eprintln!(
                        "novanet entity seed --project={}{}{}",
                        project,
                        phase.map(|p| format!(" --phase={}", p)).unwrap_or_default(),
                        if *dry_run { " --dry-run" } else { "" }
                    );

                    let results =
                        novanet::commands::entity::entity_seed(&root, project, *phase, *dry_run)?;

                    for r in &results {
                        eprintln!(
                            "  {} {} (phase {}: {}, {} entities, {} arcs, {} bytes, {}ms)",
                            if *dry_run { "would write" } else { "wrote" },
                            r.output_path,
                            r.phase,
                            r.phase_name,
                            r.entity_count,
                            r.arc_count,
                            r.bytes,
                            r.duration_ms,
                        );
                        for w in &r.warnings {
                            eprintln!("    ⚠  {}", w);
                        }
                    }

                    let total_entities: usize = results.iter().map(|r| r.entity_count).sum();
                    let total_arcs: usize = results.iter().map(|r| r.arc_count).sum();

                    eprintln!(
                        "\n{} {} phase(s), {} entities, {} arcs",
                        if *dry_run {
                            "Would generate"
                        } else {
                            "Generated"
                        },
                        results.len(),
                        total_entities,
                        total_arcs
                    );
                }
                EntityAction::List { project } => {
                    eprintln!("novanet entity list --project={}\n", project);

                    let phases = novanet::commands::entity::entity_list(&root, project)?;

                    if phases.is_empty() {
                        eprintln!("  No phases found for project '{}'", project);
                    } else {
                        eprintln!(
                            "  {:>5}  {:<25}  {:>8}  {:>6}  FILE",
                            "PHASE", "NAME", "ENTITIES", "ARCS"
                        );
                        eprintln!(
                            "  {:>5}  {:<25}  {:>8}  {:>6}  ────",
                            "─────", "────────────────────────", "────────", "────"
                        );
                        for p in &phases {
                            eprintln!(
                                "  {:>5}  {:<25}  {:>8}  {:>6}  {}",
                                p.phase, p.name, p.entity_count, p.arc_count, p.file
                            );
                        }
                        eprintln!("\n  {} phase(s)", phases.len());
                    }
                }
                EntityAction::Validate { project } => {
                    eprintln!("novanet entity validate --project={}\n", project);

                    let results = novanet::commands::entity::entity_validate(&root, project)?;

                    let mut has_errors = false;
                    for r in &results {
                        let status = if r.valid { "✓" } else { "✗" };
                        eprintln!("  {} Phase {}: {}", status, r.phase, r.file);

                        for e in &r.errors {
                            eprintln!("    ✗ {}", e);
                            has_errors = true;
                        }
                        for w in &r.warnings {
                            eprintln!("    ⚠  {}", w);
                        }
                    }

                    let valid_count = results.iter().filter(|r| r.valid).count();
                    eprintln!("\n  {}/{} phase(s) valid", valid_count, results.len());

                    if has_errors {
                        std::process::exit(1);
                    }
                }
            }
        }

        // ── Views (YAML, no Neo4j) ─────────────────────────────────
        Commands::Views { ref action } => {
            let root = root?;
            match action {
                ViewsAction::Export { format } => {
                    if format != "json" {
                        eprintln!("Only JSON format supported");
                        std::process::exit(1);
                    }
                    let json = novanet::commands::views::views_export(&root)?;
                    println!("{}", json);
                }
                ViewsAction::Validate { verbose } => {
                    eprintln!(
                        "novanet views validate{}",
                        if *verbose { " --verbose" } else { "" }
                    );
                    match novanet::commands::views::views_validate(&root, *verbose) {
                        Ok(()) => {}
                        Err(e) => {
                            eprintln!("✗ {}", e);
                            std::process::exit(1);
                        }
                    }
                }
            }
        }

        // ── Export (Neo4j Required) ─────────────────────────────
        Commands::Export(ref args) => {
            let db = connect_db(&uri, &user, password.as_ref()).await?;
            eprintln!("novanet export --format={:?}", args.format);
            novanet::commands::export::run_export(&db, args.clone(), novanet::output::OutputFormat::Table).await?;
        }

        #[cfg(feature = "tui")]
        Commands::Tui { fresh } => {
            let root = root?;

            if fresh {
                // --fresh: regenerate schema + reset database
                eprintln!("↻ Fresh start: regenerating schema...");
                let results = novanet::commands::schema::schema_generate(&root, false)?;
                eprintln!("   ✓ Generated {} artifact(s)", results.len());

                eprintln!("⊞  Resetting database...");
                let db = connect_db(&uri, &user, password.as_ref()).await?;
                novanet::commands::db::run_reset(&db, &root).await?;
                eprintln!("   ✓ Database reset complete");

                eprintln!("➜ Launching TUI...\n");
                novanet::tui::run(&db, &root).await?;
            } else {
                // Normal mode: validate schema and warn if out of sync
                let issues = novanet::commands::schema::schema_validate(&root)?;
                let errors: Vec<_> = issues
                    .iter()
                    .filter(|i| i.severity == novanet::commands::schema::Severity::Error)
                    .collect();
                let warnings: Vec<_> = issues
                    .iter()
                    .filter(|i| i.severity == novanet::commands::schema::Severity::Warning)
                    .collect();

                if !errors.is_empty() || !warnings.is_empty() {
                    eprintln!("⚠  Schema validation found issues:");
                    for e in &errors {
                        eprintln!("   ✗ {}", e.message);
                    }
                    for w in &warnings {
                        eprintln!("   ⚠  {}", w.message);
                    }
                    eprintln!();
                    eprintln!("   Run: cargo run -- tui --fresh");
                    eprintln!("   Or:  cargo run -- schema generate && pnpm infra:reset");
                    eprintln!();
                }

                let db = connect_db(&uri, &user, password.as_ref()).await?;
                novanet::tui::run(&db, &root).await?;
            }
        }

        // ── Completions (no Neo4j, no root) ──────────────────────────
        Commands::Completions { shell } => {
            novanet::commands::completions::run_completions::<Cli>(shell)?;
        }

        // ── Doctor (YAML + optional Neo4j) ───────────────────────────
        Commands::Doctor { skip_db, verbose } => {
            let root = root?;
            let db = if skip_db {
                None
            } else {
                match connect_db(&uri, &user, password.as_ref()).await {
                    Ok(db) => Some(db),
                    Err(e) => {
                        eprintln!("⚠  Could not connect to Neo4j: {}", e);
                        None
                    }
                }
            };
            novanet::commands::doctor::run_doctor(&root, db.as_ref(), verbose).await?;
        }
    }

    Ok(())
}

/// Connect to Neo4j (shared by all commands that need database access).
async fn connect_db(
    uri: &str,
    user: &str,
    password: Option<&String>,
) -> color_eyre::Result<novanet::db::Db> {
    let password = password.ok_or_else(|| {
        color_eyre::eyre::eyre!(
            "Neo4j password required. Set NEO4J_PASSWORD environment variable or use --password flag."
        )
    })?;
    eprintln!("Connecting to Neo4j at {}...", uri);
    let db = novanet::db::Db::connect(uri, user, password).await?;
    Ok(db)
}
