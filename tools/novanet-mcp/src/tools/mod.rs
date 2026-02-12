//! MCP Tools module
//!
//! Phase 1 implements: novanet_query, novanet_describe
//! Future phases will add: search, traverse, assemble, atoms, generate

pub mod describe;
pub mod query;

// Re-export tool params and results
pub use describe::{DescribeParams, DescribeResult, DescribeTarget};
pub use query::{QueryParams, QueryResult};
