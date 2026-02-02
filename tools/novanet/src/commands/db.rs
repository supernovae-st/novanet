//! Database commands: `novanet db seed`, `novanet db migrate`, `novanet db reset`.
//!
//! Reads `.cypher` files from `packages/db/seed/` and `packages/db/migrations/`,
//! splits them into individual statements, and executes against Neo4j.

use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::db::Db;

/// Run `novanet db seed`: execute all seed files + migrations.
pub async fn run_seed(db: &Db, root: &Path) -> crate::Result<()> {
    let seed_dir = crate::config::seed_dir(root);
    let migrations_dir = crate::config::migrations_dir(root);

    let seed_files = collect_cypher_files(&seed_dir)?;
    let migration_files = collect_cypher_files(&migrations_dir)?;

    eprintln!(
        "Seeding: {} seed file(s), {} migration(s)",
        seed_files.len(),
        migration_files.len()
    );

    let mut total_stmts = 0u64;
    let start = Instant::now();

    for path in &seed_files {
        let count = execute_cypher_file(db, path).await?;
        total_stmts += count;
    }

    for path in &migration_files {
        let count = execute_cypher_file(db, path).await?;
        total_stmts += count;
    }

    let elapsed = start.elapsed();
    eprintln!(
        "\nSeed complete: {} statement(s) in {:.1}s",
        total_stmts,
        elapsed.as_secs_f64()
    );

    Ok(())
}

/// Run `novanet db migrate`: execute only migration files.
pub async fn run_migrate(db: &Db, root: &Path) -> crate::Result<()> {
    let migrations_dir = crate::config::migrations_dir(root);
    let migration_files = collect_cypher_files(&migrations_dir)?;

    eprintln!("Running {} migration(s)", migration_files.len());

    let mut total_stmts = 0u64;
    let start = Instant::now();

    for path in &migration_files {
        let count = execute_cypher_file(db, path).await?;
        total_stmts += count;
    }

    let elapsed = start.elapsed();
    eprintln!(
        "\nMigrations complete: {} statement(s) in {:.1}s",
        total_stmts,
        elapsed.as_secs_f64()
    );

    Ok(())
}

/// Run `novanet db reset`: drop all data, then seed.
pub async fn run_reset(db: &Db, root: &Path) -> crate::Result<()> {
    eprintln!("Resetting database...");

    // Drop all constraints and indexes first
    eprintln!("  Dropping constraints...");
    drop_all_constraints(db).await?;

    // Delete all nodes and relationships
    eprintln!("  Deleting all nodes and relationships...");
    delete_all_data(db).await?;

    eprintln!("  Database cleared.");
    eprintln!();

    // Re-seed from scratch
    run_seed(db, root).await
}

/// Collect `.cypher` files from a directory, sorted alphabetically.
fn collect_cypher_files(dir: &Path) -> crate::Result<Vec<PathBuf>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(crate::NovaNetError::Io)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("cypher") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    files.sort();
    Ok(files)
}

/// Read a Cypher file, split into statements, execute each one.
async fn execute_cypher_file(db: &Db, path: &Path) -> crate::Result<u64> {
    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    let content = std::fs::read_to_string(path).map_err(crate::NovaNetError::Io)?;

    let statements = split_cypher_statements(&content);

    if statements.is_empty() {
        eprintln!("  {filename}: (empty)");
        return Ok(0);
    }

    let start = Instant::now();
    let mut executed = 0u64;

    for stmt in &statements {
        db.execute(stmt).await.map_err(|e| {
            // Wrap with file context for better error messages
            crate::NovaNetError::Validation(format!(
                "Failed in {filename} (statement #{}):\n  {stmt:.120}\n  Error: {e}",
                executed + 1
            ))
        })?;
        executed += 1;
    }

    let elapsed = start.elapsed();
    eprintln!(
        "  {filename}: {executed} statement(s) ({:.1}s)",
        elapsed.as_secs_f64()
    );

    Ok(executed)
}

/// Split a multi-statement Cypher file into individual statements.
///
/// Handles:
/// - `;` as statement delimiter
/// - `//` single-line comments (stripped)
/// - String literals in single quotes (`;` inside strings is preserved)
/// - Empty statements are skipped
pub fn split_cypher_statements(input: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            // Single-line comment: skip to end of line
            '/' if chars.peek() == Some(&'/') => {
                chars.next(); // consume second '/'
                for c in chars.by_ref() {
                    if c == '\n' {
                        break;
                    }
                }
                // Preserve the newline for readability
                current.push('\n');
            }
            // String literal: consume until closing quote
            '\'' => {
                current.push('\'');
                loop {
                    match chars.next() {
                        Some('\'') => {
                            current.push('\'');
                            // Check for escaped quote ('')
                            if chars.peek() == Some(&'\'') {
                                current.push(chars.next().unwrap());
                            } else {
                                break;
                            }
                        }
                        Some(c) => current.push(c),
                        None => break, // unterminated string
                    }
                }
            }
            // Statement delimiter
            ';' => {
                let trimmed = current.trim().to_string();
                if !trimmed.is_empty() {
                    statements.push(trimmed);
                }
                current.clear();
            }
            // Regular character
            _ => current.push(ch),
        }
    }

    // Last statement (may not end with ;)
    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() {
        statements.push(trimmed);
    }

    statements
}

/// Drop all constraints and indexes (for reset).
async fn drop_all_constraints(db: &Db) -> crate::Result<()> {
    // List all constraints
    let rows = db
        .execute("SHOW CONSTRAINTS YIELD name RETURN name")
        .await?;
    for row in &rows {
        let name: String = row.get("name").unwrap_or_default();
        if !name.is_empty() {
            let drop_stmt = format!("DROP CONSTRAINT {name} IF EXISTS");
            db.execute(&drop_stmt).await?;
        }
    }

    // List all indexes (skip built-in lookup indexes)
    let rows = db
        .execute("SHOW INDEXES YIELD name, type WHERE type <> 'LOOKUP' RETURN name")
        .await?;
    for row in &rows {
        let name: String = row.get("name").unwrap_or_default();
        if !name.is_empty() {
            let drop_stmt = format!("DROP INDEX {name} IF EXISTS");
            db.execute(&drop_stmt).await?;
        }
    }

    Ok(())
}

/// Delete all nodes and relationships in batches (handles large graphs).
async fn delete_all_data(db: &Db) -> crate::Result<()> {
    // Batch delete to avoid OOM on large graphs
    loop {
        let rows = db
            .execute("MATCH (n) WITH n LIMIT 10000 DETACH DELETE n RETURN count(*) AS deleted")
            .await?;

        let deleted: i64 = rows
            .first()
            .and_then(|r| r.get("deleted").ok())
            .unwrap_or(0);

        if deleted == 0 {
            break;
        }
        eprintln!("    deleted {deleted} node(s)...");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_empty() {
        assert!(split_cypher_statements("").is_empty());
        assert!(split_cypher_statements("   \n\n  ").is_empty());
    }

    #[test]
    fn split_single_statement() {
        let stmts = split_cypher_statements("MATCH (n) RETURN n;");
        assert_eq!(stmts.len(), 1);
        assert_eq!(stmts[0], "MATCH (n) RETURN n");
    }

    #[test]
    fn split_multiple_statements() {
        let input = "CREATE (a:Node);\nMATCH (n) RETURN n;\nMERGE (x:Foo);";
        let stmts = split_cypher_statements(input);
        assert_eq!(stmts.len(), 3);
        assert_eq!(stmts[0], "CREATE (a:Node)");
        assert_eq!(stmts[1], "MATCH (n) RETURN n");
        assert_eq!(stmts[2], "MERGE (x:Foo)");
    }

    #[test]
    fn split_strips_comments() {
        let input = "// This is a comment\nMATCH (n) RETURN n;";
        let stmts = split_cypher_statements(input);
        assert_eq!(stmts.len(), 1);
        assert_eq!(stmts[0], "MATCH (n) RETURN n");
    }

    #[test]
    fn split_preserves_semicolons_in_strings() {
        let input = "CREATE (n {name: 'hello;world'});";
        let stmts = split_cypher_statements(input);
        assert_eq!(stmts.len(), 1);
        assert!(stmts[0].contains("'hello;world'"));
    }

    #[test]
    fn split_handles_escaped_quotes() {
        let input = "CREATE (n {name: 'it''s a test'});";
        let stmts = split_cypher_statements(input);
        assert_eq!(stmts.len(), 1);
        assert!(stmts[0].contains("'it''s a test'"));
    }

    #[test]
    fn split_no_trailing_semicolon() {
        let input = "MATCH (n) RETURN n";
        let stmts = split_cypher_statements(input);
        assert_eq!(stmts.len(), 1);
        assert_eq!(stmts[0], "MATCH (n) RETURN n");
    }

    #[test]
    fn split_skips_empty_statements() {
        let input = ";;;\nMATCH (n) RETURN n;\n;;;";
        let stmts = split_cypher_statements(input);
        assert_eq!(stmts.len(), 1);
    }

    #[test]
    fn split_real_world_constraints() {
        let input = "\
// NovaNet Constraints
CREATE CONSTRAINT locale_key IF NOT EXISTS FOR (l:Locale) REQUIRE l.key IS UNIQUE;
CREATE INDEX locale_language IF NOT EXISTS FOR (l:Locale) ON (l.language_code);
// Another comment
CREATE CONSTRAINT project_key IF NOT EXISTS FOR (p:Project) REQUIRE p.key IS UNIQUE;
";
        let stmts = split_cypher_statements(input);
        assert_eq!(stmts.len(), 3);
        assert!(stmts[0].contains("locale_key"));
        assert!(stmts[1].contains("locale_language"));
        assert!(stmts[2].contains("project_key"));
    }

    #[test]
    fn collect_cypher_files_nonexistent_dir() {
        let files = collect_cypher_files(Path::new("/nonexistent")).unwrap();
        assert!(files.is_empty());
    }
}
