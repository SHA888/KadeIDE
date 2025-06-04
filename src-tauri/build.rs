//! Build script for KadeIDE
//!
//! This script handles build-time configurations and validations

use std::env;
use std::path::PathBuf;

fn main() {
    // Enable build features based on environment variables
    let mut features = vec![];
    
    // Check for AI feature flag
    if env::var("CARGO_FEATURE_AI").is_ok() {
        println!("cargo:rustc-cfg=feature=\"ai\"");
        features.push("ai");
    }
    
    // Check for LSP feature flag
    if env::var("CARGO_FEATURE_LSP").is_ok() {
        println!("cargo:rustc-cfg=feature=\"lsp\"");
        features.push("lsp");
    }
    
    // Generate build info
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string());
    println!("cargo:rustc-env=KADEIDE_VERSION={}", version);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    
    // Generate Tauri build info
    if let Err(e) = tauri_build::try_build(tauri_build::Attributes::new()) {
        panic!("failed to run tauri-build: {}", e);
    }
    
    // Generate bindings if needed
    #[cfg(feature = "bindgen")]
    generate_bindings();
}

#[cfg(feature = "bindgen")]
fn generate_bindings() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Example: Generate bindings for native code
    // bindgen::Builder::default()
    //     .header("src/native.h")
    //     .generate()
    //     .expect("Unable to generate bindings")
    //     .write_to_file(out_dir.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");
}
