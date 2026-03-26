$ErrorActionPreference = "Stop"

function Step($m) { Write-Host "[*] $m" -ForegroundColor Cyan }

$reportPath = ".\artifacts\reports\metrics-summary.md"
$buildTime = "N/A"
$startupMs = "N/A"
$fixedTickMean = "N/A"
$fixedTickP95 = "N/A"
$peakRss = "N/A"
$allocsPerTick = "N/A"
$binarySize = "N/A"

Step "Writing metrics summary"

@"
# Metrics Summary

- build time: $buildTime
- startup ms: $startupMs
- fixed tick mean ms: $fixedTickMean
- fixed tick p95 ms: $fixedTickP95
- peak rss mb: $peakRss
- allocs per tick: $allocsPerTick
- binary size: $binarySize

## Notes
Replace placeholder values with real metrics once smoke and benchmark commands are wired.
"@ | Set-Content -Path $reportPath -Encoding utf8