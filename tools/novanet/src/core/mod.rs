//! Core module for NovaNet CLI
//!
//! This module contains shared functionality used across the CLI:
//! - Backup/restore operations for the brain directory
//! - Incremental export checkpoint tracking
//! - Shared UX helpers for polished CLI output

pub mod backup;
pub mod checkpoint;
pub mod ux;
