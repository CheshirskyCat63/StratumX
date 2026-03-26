$ErrorActionPreference = "Stop"

function Step($m) { Write-Host "[*] $m" -ForegroundColor Cyan }

Step "Running smoke checks"
"SMOKE: define project-specific smoke commands here." | Tee-Object -FilePath ".\artifacts\smoke\smoke.txt"

# Replace this placeholder with real commands, for example:
# cargo run -p engine_headless -- --smoke