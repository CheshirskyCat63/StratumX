#!/usr/bin/env bash
set -euo pipefail

echo "[*] cargo fmt --all --check"
cargo fmt --all --check 2>&1 | tee ./artifacts/test-results/fmt.txt

echo "[*] cargo clippy --workspace --all-targets --all-features -- -D warnings"
cargo clippy --workspace --all-targets --all-features -- -D warnings 2>&1 | tee ./artifacts/test-results/clippy.txt

echo "[*] cargo test --workspace --all-features"
cargo test --workspace --all-features 2>&1 | tee ./artifacts/test-results/cargo-test.txt