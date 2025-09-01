#!/bin/bash
echo "Running cargo fmt..."
cargo fmt

echo "Running cargo clippy..."
cargo clippy -- -D warnings

echo "Running cargo test..."
cargo test

echo "Running example ..."
cargo run --example ideal_use_case
echo "All checks completed!"