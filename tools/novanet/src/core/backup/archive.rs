//! Archive operations for backup files
//!
//! This module provides functions for creating and extracting tar.gz archives:
//! - Create archives with manifest and checksums
//! - Extract archives to destination
//! - Read manifest from existing archives

use super::types::{BackupContents, BackupError, BackupManifest, Result};
use chrono::Utc;
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use tar::{Archive, Builder};
use tracing::instrument;

/// Manifest filename inside the archive
const MANIFEST_FILENAME: &str = "manifest.json";

/// Current backup format version
const BACKUP_VERSION: &str = "1.0.0";

/// Create a tar.gz archive of the brain directory
///
/// The archive contains:
/// - manifest.json with metadata and checksums
/// - All files from the brain directory
#[instrument(skip_all)]
pub async fn create_archive(
    brain_dir: &Path,
    backup_path: &Path,
    description: Option<String>,
) -> Result<()> {
    // Collect files and compute checksums
    let (contents, checksums) = collect_files_with_checksums(brain_dir)?;

    // Create manifest
    let manifest = BackupManifest {
        version: BACKUP_VERSION.to_string(),
        created_at: Utc::now(),
        description,
        contents,
        checksums,
    };

    // Create the archive
    let file = File::create(backup_path)?;
    let encoder = GzEncoder::new(file, Compression::default());
    let mut builder = Builder::new(encoder);

    // Add manifest first
    let manifest_json = serde_json::to_string_pretty(&manifest)?;
    let manifest_bytes = manifest_json.as_bytes();
    let mut header = tar::Header::new_gnu();
    header.set_size(manifest_bytes.len() as u64);
    header.set_mode(0o644);
    header.set_mtime(Utc::now().timestamp() as u64);
    header.set_cksum();
    builder.append_data(&mut header, MANIFEST_FILENAME, manifest_bytes)?;

    // Add all files from brain directory
    add_directory_to_archive(&mut builder, brain_dir, Path::new("brain"))?;

    // Finish the archive
    let encoder = builder.into_inner()?;
    encoder.finish()?;

    Ok(())
}

/// Extract a tar.gz archive to the brain directory
///
/// This will overwrite existing files in the destination.
#[instrument(skip_all)]
pub async fn extract_archive(backup_path: &Path, brain_dir: &Path) -> Result<()> {
    let file = File::open(backup_path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;

        // Skip the manifest file (we don't need to extract it)
        if path.to_string_lossy() == MANIFEST_FILENAME {
            continue;
        }

        // Remove "brain/" prefix and extract to destination
        let relative_path = path.strip_prefix("brain").unwrap_or(&path);
        let dest_path = brain_dir.join(relative_path);

        // Create parent directories if needed
        if let Some(parent) = dest_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Extract the file
        if entry.header().entry_type().is_file() {
            let mut file = File::create(&dest_path)?;
            std::io::copy(&mut entry, &mut file)?;
        } else if entry.header().entry_type().is_dir() {
            std::fs::create_dir_all(&dest_path)?;
        }
    }

    Ok(())
}

/// Read the manifest from a backup archive
#[instrument(skip_all)]
pub fn read_manifest_from_archive(backup_path: &Path) -> Result<BackupManifest> {
    let file = File::open(backup_path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;

        if path.to_string_lossy() == MANIFEST_FILENAME {
            let mut content = String::new();
            entry.read_to_string(&mut content)?;
            let manifest: BackupManifest = serde_json::from_str(&content)?;
            return Ok(manifest);
        }
    }

    Err(BackupError::ManifestCorrupted(
        "Manifest not found in archive".to_string(),
    ))
}

/// Collect files from a directory and compute SHA256 checksums
fn collect_files_with_checksums(dir: &Path) -> Result<(BackupContents, HashMap<String, String>)> {
    let mut file_count = 0u64;
    let mut total_size = 0u64;
    let mut checksums = HashMap::new();
    let mut directories = Vec::new();

    collect_files_recursive(
        dir,
        dir,
        &mut file_count,
        &mut total_size,
        &mut checksums,
        &mut directories,
    )?;

    let contents = BackupContents {
        file_count,
        total_size,
        directories,
    };

    Ok((contents, checksums))
}

/// Recursively collect files and compute checksums
fn collect_files_recursive(
    base_dir: &Path,
    current_dir: &Path,
    file_count: &mut u64,
    total_size: &mut u64,
    checksums: &mut HashMap<String, String>,
    directories: &mut Vec<String>,
) -> Result<()> {
    for entry in std::fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path
            .strip_prefix(base_dir)
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();

        if path.is_dir() {
            directories.push(relative_path.clone());
            collect_files_recursive(
                base_dir,
                &path,
                file_count,
                total_size,
                checksums,
                directories,
            )?;
        } else if path.is_file() {
            *file_count += 1;
            let metadata = std::fs::metadata(&path)?;
            *total_size += metadata.len();

            // Compute SHA256 checksum
            let checksum = compute_file_checksum(&path)?;
            checksums.insert(relative_path, checksum);
        }
    }

    Ok(())
}

/// Compute SHA256 checksum of a file
fn compute_file_checksum(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

/// Add a directory and its contents to a tar archive
fn add_directory_to_archive<W: Write>(
    builder: &mut Builder<W>,
    source_dir: &Path,
    archive_prefix: &Path,
) -> Result<()> {
    for entry in std::fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(source_dir).unwrap_or(&path);
        let archive_path = archive_prefix.join(relative_path);

        if path.is_dir() {
            add_directory_to_archive(builder, &path, &archive_path)?;
        } else if path.is_file() {
            let mut file = File::open(&path)?;
            builder.append_file(&archive_path, &mut file)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_create_and_read_manifest() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let brain_dir = temp_dir.path().join("brain");
        let backup_path = temp_dir.path().join("test-backup.tar.gz");

        // Create a simple brain directory
        std::fs::create_dir_all(&brain_dir).expect("Failed to create brain dir");
        std::fs::write(brain_dir.join("test.txt"), "Hello, World!")
            .expect("Failed to write test file");

        // Create archive
        create_archive(&brain_dir, &backup_path, Some("Test backup".to_string()))
            .await
            .expect("Failed to create archive");

        // Read manifest
        let manifest = read_manifest_from_archive(&backup_path).expect("Failed to read manifest");

        assert_eq!(manifest.version, "1.0.0");
        assert_eq!(manifest.description, Some("Test backup".to_string()));
        assert_eq!(manifest.contents.file_count, 1);
        assert!(!manifest.checksums.is_empty());
    }

    #[tokio::test]
    async fn test_create_and_extract_archive() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let brain_dir = temp_dir.path().join("brain");
        let backup_path = temp_dir.path().join("test-backup.tar.gz");
        let restore_dir = temp_dir.path().join("restored");

        // Create a brain directory with some files
        std::fs::create_dir_all(brain_dir.join("subdir")).expect("Failed to create dirs");
        std::fs::write(brain_dir.join("test.txt"), "Hello, World!")
            .expect("Failed to write test file");
        std::fs::write(brain_dir.join("subdir/nested.txt"), "Nested content")
            .expect("Failed to write nested file");

        // Create archive
        create_archive(&brain_dir, &backup_path, None)
            .await
            .expect("Failed to create archive");

        // Extract archive
        std::fs::create_dir_all(&restore_dir).expect("Failed to create restore dir");
        extract_archive(&backup_path, &restore_dir)
            .await
            .expect("Failed to extract archive");

        // Verify restored files
        let restored_content = std::fs::read_to_string(restore_dir.join("test.txt"))
            .expect("Failed to read restored file");
        assert_eq!(restored_content, "Hello, World!");

        let nested_content = std::fs::read_to_string(restore_dir.join("subdir/nested.txt"))
            .expect("Failed to read nested file");
        assert_eq!(nested_content, "Nested content");
    }

    #[test]
    fn test_compute_file_checksum() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("test.txt");
        std::fs::write(&file_path, "Hello, World!").expect("Failed to write file");

        let checksum = compute_file_checksum(&file_path).expect("Failed to compute checksum");

        // SHA256 of "Hello, World!" is known
        assert_eq!(
            checksum,
            "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"
        );
    }

    #[test]
    fn test_read_manifest_not_found() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let empty_archive_path = temp_dir.path().join("empty.tar.gz");

        // Create an empty tar.gz
        let file = File::create(&empty_archive_path).expect("Failed to create file");
        let encoder = GzEncoder::new(file, Compression::default());
        let builder = Builder::new(encoder);
        let encoder = builder.into_inner().expect("Failed to finish builder");
        encoder.finish().expect("Failed to finish encoder");

        let result = read_manifest_from_archive(&empty_archive_path);
        assert!(matches!(result, Err(BackupError::ManifestCorrupted(_))));
    }
}
