//! MCP Tools module (v0.20.0)
//!
//! 8 tools after The Great Cleanup (was 14):
//!   novanet_query, novanet_describe, novanet_search, novanet_introspect,
//!   novanet_context, novanet_write, novanet_audit, novanet_batch
//!
//! Removed tools (absorbed or deleted):
//!   - novanet_traverse → merged into novanet_search (mode=walk)
//!   - novanet_assemble, novanet_atoms, novanet_generate → merged into novanet_context
//!   - novanet_cache_stats, novanet_cache_invalidate → deleted (D7)
//!   - novanet_check → absorbed into novanet_write (dry_run param, D6)

pub mod auditor;
pub mod batch;
pub mod context;
pub mod describe;
pub mod introspect;
pub mod query;
pub mod search;
pub mod write;

// Re-export tool params and results
pub use auditor::{
    AuditIssue, AuditParams, AuditResult, AuditSeverity, AuditSummary, AuditTarget,
    OntologyInsights,
};
pub use batch::{BatchParams, BatchResult};
pub use context::{ContextMode, ContextParams, ContextResult};
pub use describe::{DescribeParams, DescribeResult, DescribeTarget};
pub use introspect::{IntrospectParams, IntrospectResult, IntrospectTarget};
pub use query::{QueryParams, QueryResult};
pub use search::{SearchMode, SearchParams, SearchResult};
pub use write::{WriteParams, WriteResult};
