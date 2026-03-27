//! Backup and rollback functionality for memory files
//!
//! Creates .md.bak backups before each write, enabling rollback.

use std::path::{Path, PathBuf};
use std::fs;

/// Create a backup of a file before modifying it
pub fn create_backup<P: AsRef<Path>>(path: P) -> std::io::Result<PathBuf> {
    let path = path.as_ref();
    let backup_path = PathBuf::from(format!("{}.bak", path.display()));

    // If backup already exists, keep it (don't overwrite)
    if backup_path.exists() {
        // Create a numbered backup instead: .md.bak.1, .md.bak.2, etc.
        return create_numbered_backup(path);
    }

    fs::copy(path, &backup_path)?;
    Ok(backup_path)
}

/// Create a numbered backup when .bak already exists
fn create_numbered_backup<P: AsRef<Path>>(path: P) -> std::io::Result<PathBuf> {
    let path = path.as_ref();
    let mut counter = 1;

    loop {
        let backup_path = PathBuf::from(format!("{}.bak.{}", path.display(), counter));
        if !backup_path.exists() {
            return fs::copy(path, &backup_path).map(|_| backup_path);
        }
        counter += 1;

        // Safety limit
        if counter > 100 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Too many backup files",
            ));
        }
    }
}

/// Restore a file from its backup
pub fn restore_from_backup<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    let path = path.as_ref();
    let backup_path = PathBuf::from(format!("{}.bak", path.display()));

    if !backup_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No backup file found",
        ));
    }

    fs::copy(&backup_path, path)?;
    Ok(())
}

/// Restore from a numbered backup
pub fn restore_from_numbered_backup<P: AsRef<Path>>(path: P, number: u32) -> std::io::Result<()> {
    let path = path.as_ref();
    let backup_path = PathBuf::from(format!("{}.bak.{}", path.display(), number));

    if !backup_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Backup file {} not found", number),
        ));
    }

    fs::copy(&backup_path, path)?;
    Ok(())
}

/// Clean up old numbered backups, keeping only the most recent N
pub fn cleanup_old_backups<P: AsRef<Path>>(path: P, keep_count: usize) -> std::io::Result<()> {
    let path = path.as_ref();
    let prefix = format!("{}.bak.", path.display());

    // Find all numbered backups
    let mut backups: Vec<(u32, PathBuf)> = Vec::new();
    if let Some(parent) = path.parent() {
        if let Ok(entries) = fs::read_dir(parent) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if let Some(num_str) = name.strip_prefix(&prefix) {
                    if let Ok(num) = num_str.parse::<u32>() {
                        backups.push((num, entry.path()));
                    }
                }
            }
        }
    }

    // Sort by number (ascending - oldest first)
    backups.sort();

    // Delete all but the most recent N
    let to_delete = backups.len().saturating_sub(keep_count);
    for (_num, backup_path) in backups.into_iter().take(to_delete) {
        let _ = fs::remove_file(backup_path);
    }

    Ok(())
}

/// Get the path to the backup file for a given path
pub fn backup_path<P: AsRef<Path>>(path: P) -> PathBuf {
    PathBuf::from(format!("{}.bak", path.as_ref().display()))
}

/// Check if a backup exists for a given path
pub fn backup_exists<P: AsRef<Path>>(path: P) -> bool {
    backup_path(path).exists()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_backup_roundtrip() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.md");

        // Write initial content
        let mut file = fs::File::create(&file_path).unwrap();
        write!(file, "original content").unwrap();

        // Create backup
        let backup = create_backup(&file_path).unwrap();
        assert!(backup.exists());

        // Modify file
        let mut file = fs::File::create(&file_path).unwrap();
        write!(file, "modified content").unwrap();

        // Restore from backup
        restore_from_backup(&file_path).unwrap();

        // Verify content restored
        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "original content");
    }
}