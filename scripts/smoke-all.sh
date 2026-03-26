#!/usr/bin/env bash
set -euo pipefail

echo "[*] Running smoke checks"
echo "SMOKE: define project-specific smoke commands here." | tee ./artifacts/smoke/smoke.txt