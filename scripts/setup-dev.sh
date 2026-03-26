#!/usr/bin/env bash
set -euo pipefail

echo "[*] Checking repository root"
test -d "./1.docs/canonical" || { echo "[!] Missing ./1.docs/canonical"; exit 1; }

echo "[*] Checking tools"
command -v git >/dev/null 2>&1 || { echo "[!] git not found"; exit 1; }

echo "[*] Preparing artifact folders"
mkdir -p \
  ./artifacts \
  ./artifacts/bench \
  ./artifacts/logs \
  ./artifacts/reports \
  ./artifacts/smoke \
  ./artifacts/test-results

if [ -f "./Cargo.toml" ]; then
  echo "[*] Rust workspace detected"
  command -v cargo >/dev/null 2>&1 || { echo "[!] cargo not found"; exit 1; }
  command -v rustc >/dev/null 2>&1 || { echo "[!] rustc not found"; exit 1; }

  echo "[*] Rust versions"
  rustc --version
  cargo --version

  echo "[*] Fetching dependencies"
  cargo fetch
else
  echo "[*] No Cargo.toml at repository root yet"
  echo "[*] Canonical/bootstrap phase only; skipping cargo fetch"
fi

echo "[*] Setup complete"