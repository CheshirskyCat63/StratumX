#!/usr/bin/env bash
set -euo pipefail

mkdir -p ./artifacts/test-results

if [ ! -f "./Cargo.toml" ]; then
  echo "[*] No Cargo.toml at repository root yet" | tee ./artifacts/test-results/test-status.txt
  echo "[*] Skipping cargo fmt/clippy/test during pre-workspace phase" | tee -a ./artifacts/test-results/test-status.txt
  exit 0
fi

echo "[*] cargo fmt --all --check"
cargo fmt --all --check 2>&1 | tee ./artifacts/test-results/fmt.txt

echo "[*] cargo clippy --workspace --all-targets --all-features -- -D warnings"
cargo clippy --workspace --all-targets --all-features -- -D warnings 2>&1 | tee ./artifacts/test-results/clippy.txt

echo "[*] cargo test --workspace --all-features"
cargo test --workspace --all-features 2>&1 | tee ./artifacts/test-results/cargo-test.txt