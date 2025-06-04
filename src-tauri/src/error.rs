//! Error handling for the KadeIDE application

use thiserror::Error;
use std::fmt;

/// Main error type for the KadeIDE application
#[derive(Debug, Error)]
pub enum Error {
    /// I/O errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Tauri errors
    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),
    
    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),
    
    /// Plugin errors
    #[error("Plugin error: {0}")]
    Plugin(String),
    
    /// Custom error message
    #[error("{0}")]
    Custom(String),
}

/// Result type for the KadeIDE application
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Create a new custom error
    pub fn custom<M: Into<String>>(msg: M) -> Self {
        Self::Custom(msg.into())
    }
    
    /// Create a new configuration error
    pub fn config<M: Into<String>>(msg: M) -> Self {
        Self::Config(msg.into())
    }
    
    /// Create a new plugin error
    pub fn plugin<M: Into<String>>(msg: M) -> Self {
        Self::Plugin(msg.into())
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Self::Custom(s.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::Custom(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_creation() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err = Error::from(io_err);
        assert!(matches!(err, Error::Io(_)));
        
        let custom_err = Error::custom("test error");
        assert!(matches!(custom_err, Error::Custom(_)));
        
        let config_err = Error::config("invalid config");
        assert!(matches!(config_err, Error::Config(_)));
    }
}
