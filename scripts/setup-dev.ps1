#!/usr/bin/env bash
set -euo pipefail

echo "[*] Checking repository root"
test -d "./1.docs/canonical" || { echo "[!] Missing ./1.docs/canonical"; exit 1; }

echo "[*] Checking tools"
command -v git >/dev/null 2>&1 || { echo "[!] git not found"; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo "[!] cargo not found"; exit 1; }
command -v rustc >/dev/null 2>&1 || { echo "[!] rustc not found"; exit 1; }

echo "[*] Rust versions"
rustc --version
cargo --version

echo "[*] Fetching dependencies"
cargo fetch

echo "[*] Preparing artifact folders"
mkdir -p \
  ./artifacts \
  ./artifacts/bench \
  ./artifacts/logs \
  ./artifacts/reports \
  ./artifacts/smoke \
  ./artifacts/test-results

echo "[*] Setup complete"