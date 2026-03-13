//! Core module for NovaNet CLI
//!
//! This module contains shared functionality used across the CLI:
//! - Backup/restore operations for the brain directory
//! - Incremental export checkpoint tracking

pub mod backup;
pub mod checkpoint;
