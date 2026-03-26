$ErrorActionPreference = "Stop"

function Step($m) { Write-Host "[*] $m" -ForegroundColor Cyan }
function Fail($m) { Write-Host "[!] $m" -ForegroundColor Red; exit 1 }

Step "Checking repository root"
if (-not (Test-Path ".\1.docs\canonical")) { Fail "Missing .\1.docs\canonical" }

Step "Checking tools"
$null = Get-Command git -ErrorAction Stop
$null = Get-Command cargo -ErrorAction Stop
$null = Get-Command rustc -ErrorAction Stop

Step "Checking Rust toolchain"
rustc --version
cargo --version

Step "Fetching dependencies"
cargo fetch

Step "Preparing artifact folders"
@(
    ".\artifacts",
    ".\artifacts\bench",
    ".\artifacts\logs",
    ".\artifacts\reports",
    ".\artifacts\smoke",
    ".\artifacts\test-results"
) | ForEach-Object {
    if (-not (Test-Path $_)) { New-Item -ItemType Directory -Path $_ | Out-Null }
}

Step "Setup complete"