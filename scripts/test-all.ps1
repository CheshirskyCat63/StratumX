$ErrorActionPreference = "Stop"

function Step($m) { Write-Host "[*] $m" -ForegroundColor Cyan }

Step "cargo fmt --all --check"
cargo fmt --all --check 2>&1 | Tee-Object -FilePath ".\artifacts\test-results\fmt.txt"

Step "cargo clippy --workspace --all-targets --all-features -- -D warnings"
cargo clippy --workspace --all-targets --all-features -- -D warnings 2>&1 | Tee-Object -FilePath ".\artifacts\test-results\clippy.txt"

Step "cargo test --workspace --all-features"
cargo test --workspace --all-features 2>&1 | Tee-Object -FilePath ".\artifacts\test-results\cargo-test.txt"