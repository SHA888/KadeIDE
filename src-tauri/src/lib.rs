//! KadeIDE - A lightweight, high-performance code editor built with Tauri
//!
//! This module contains the core backend implementation for KadeIDE,
//! including the main application entry point and core functionality.

// Re-export the main application entry point
pub use crate::app::run;

// Core modules
mod app;
mod error;
mod utils;

/// Pre-export commonly used types and traits
pub mod prelude {
    pub use crate::error::{Error, Result};
    pub use tauri::Manager;
    pub use log::{debug, error, info, trace, warn};
}

/// Re-export for use in tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_app_initialization() {
        // Basic test to verify the module structure
        assert!(true);
    }
}
