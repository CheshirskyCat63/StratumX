$ErrorActionPreference = "Stop"

function Step($m) { Write-Host "[*] $m" -ForegroundColor Cyan }

Step "Running benchmarks"
"BENCH: define project-specific benchmark commands here." | Tee-Object -FilePath ".\artifacts\bench\bench.txt"

# Replace this placeholder with real commands, for example:
# cargo bench --workspace