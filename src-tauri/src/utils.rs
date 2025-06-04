//! Utility functions and types for the KadeIDE application

use std::path::{Path, PathBuf};
use std::fs;
use log::{debug, error};
use dirs;
use crate::error::{Error, Result};

const APP_NAME: &str = "kade-ide";

/// Get the application's configuration directory
pub fn get_config_dir() -> Result<PathBuf> {
    dirs::config_dir()
        .map(|p| p.join(APP_NAME))
        .ok_or_else(|| Error::custom("Failed to get config directory"))
}

/// Get the application's data directory
pub fn get_data_dir() -> Result<PathBuf> {
    dirs::data_dir()
        .map(|p| p.join(APP_NAME))
        .ok_or_else(|| Error::custom("Failed to get data directory"))
}

/// Get the application's cache directory
pub fn get_cache_dir() -> Result<PathBuf> {
    dirs::cache_dir()
        .map(|p| p.join(APP_NAME))
        .ok_or_else(|| Error::custom("Failed to get cache directory"))
}

/// Get the application's local data directory (for logs, etc.)
pub fn get_local_data_dir() -> Result<PathBuf> {
    dirs::data_local_dir()
        .map(|p| p.join(APP_NAME))
        .ok_or_else(|| Error::custom("Failed to get local data directory"))
}

/// Ensure a directory exists, creating it if necessary
pub fn ensure_dir_exists<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    if !path.exists() {
        debug!("Creating directory: {}", path.display());
        fs::create_dir_all(path).map_err(Error::from)
    } else if !path.is_dir() {
        Err(Error::custom(format!("Path exists but is not a directory: {}", path.display())))
    } else {
        Ok(())
    }
}

/// Read a file to a string
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
    fs::read_to_string(&path)
        .map_err(|e| {
            error!("Failed to read file {}: {}", path.as_ref().display(), e);
            Error::from(e)
        })
}

/// Write a string to a file, creating parent directories if needed
pub fn write_file<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<()> {
    if let Some(parent) = path.as_ref().parent() {
        ensure_dir_exists(parent)?;
    }
    
    fs::write(&path, contents)
        .map_err(|e| {
            error!("Failed to write file {}: {}", path.as_ref().display(), e);
            Error::from(e)
        })
}

/// Get the application version
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_ensure_dir_exists() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("test_dir");
        
        // Test creating a new directory
        assert!(ensure_dir_exists(&test_dir).is_ok());
        assert!(test_dir.is_dir());
        
        // Test with existing directory
        assert!(ensure_dir_exists(&test_dir).is_ok());
        
        // Test with file path (should fail)
        let file_path = temp_dir.path().join("test_file.txt");
        std::fs::write(&file_path, "test").unwrap();
        assert!(ensure_dir_exists(&file_path).is_err());
    }
    
    #[test]
    fn test_read_write_file() {
        let temp_dir = tempdir().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        let test_content = "test content";
        
        // Test writing and reading a file
        assert!(write_file(&test_file, test_content).is_ok());
        assert_eq!(read_file(&test_file).unwrap(), test_content);
    }
}
