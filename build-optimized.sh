#!/bin/bash

# Build the application in release mode with optimizations
cargo build --release

# Analyze the binary size
echo "\n=== Binary Size Analysis ==="
du -h ./target/release/kade-ide

# List largest dependencies
echo "\n=== Largest Dependencies ==="
cargo bloat --release --crates

# Optimize wasm-opt if WebAssembly is used
if [ -d "./src-tauri/target/wasm32-unknown-unknown/release" ]; then
    echo "\n=== Optimizing WebAssembly ==="
    wasm-opt -Oz -o ./src-tauri/target/wasm32-unknown-unknown/release/optimized.wasm \
        ./src-tauri/target/wasm32-unknown-unknown/release/kade_ide.wasm
    
    echo "\n=== WASM Size Comparison ==="
    du -h ./src-tauri/target/wasm32-unknown-unknown/release/kade_ide.wasm
    du -h ./src-tauri/target/wasm32-unknown-unknown/release/optimized.wasm
fi

echo "\nBuild and optimization complete!"
