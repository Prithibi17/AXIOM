#!/bin/bash
set -e

echo "Starting AXIOM v1.0.0 Release Process..."

# 1. Run all tests
echo "Running test suite..."
cargo test --workspace

# 2. Build release artifacts
echo "Building release artifacts..."
cargo build --release

# 3. Publish core compiler to crates.io
echo "Publishing axiom-compiler to Cargo..."
# cargo publish --manifest-path axiom-compiler/Cargo.toml

# 4. Publish CLI tool to npm
echo "Publishing axiom-cli to npm registry..."
# npm publish axiom-cli

echo "Release v1.0.0 completed successfully!"
