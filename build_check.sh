#!/bin/bash

# Build check script for nightly Rust compatibility
echo "=== UAV Swarm Nightly Rust Compatibility Check ==="

# Check if rust toolchain file exists
if [ -f "rust-toolchain.toml" ]; then
    echo "✓ rust-toolchain.toml found"
else
    echo "✗ rust-toolchain.toml missing"
    exit 1
fi

# Update to latest nightly
echo "Updating to latest nightly..."
rustup update nightly

# Clean build directory
echo "Cleaning build directory..."
cargo clean

# Check formatting
echo "Checking code formatting..."
cargo fmt --check || {
    echo "Code formatting issues found. Run 'cargo fmt' to fix."
}

# Run clippy with nightly lints
echo "Running clippy with nightly lints..."
cargo clippy -- -D warnings

# Build with nightly features
echo "Building with nightly..."
cargo build --release

# Run tests
echo "Running tests..."
cargo test

echo "=== Build check complete ==="