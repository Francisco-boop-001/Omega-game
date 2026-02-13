$ErrorActionPreference = "Stop"

$summaryPath = "target/m4-gate-check-summary.md"
$dashboardPath = "target/ws-d-regression-dashboard.json"

function Run-Step {
    param(
        [string]$Name,
        [string]$Command
    )

    Write-Host "==> $Name"
    $start = Get-Date
    & powershell -NoProfile -Command $Command | Out-Host
    $exitCode = $LASTEXITCODE
    $end = Get-Date
    $elapsed = [math]::Round(($end - $start).TotalSeconds, 2)

    return [PSCustomObject]@{
        Name = $Name
        Command = $Command
        ExitCode = $exitCode
        ElapsedSeconds = $elapsed
        Passed = ($exitCode -eq 0)
    }
}

$steps = @(
    @{ Name = "fmt"; Command = "cargo fmt --all -- --check" },
    @{ Name = "clippy"; Command = "cargo clippy --workspace --all-targets -- -D warnings" },
    @{ Name = "tests"; Command = "cargo test --workspace" },
    @{ Name = "replay-dashboard"; Command = "cargo run -p omega-tools --bin replay_tool" },
    @{ Name = "fuzz-smoke"; Command = "cargo run -p omega-tools --bin fuzz_smoke" },
    @{ Name = "perf-budget"; Command = "cargo run -p omega-tools --bin perf_baseline -- --check" }
)

$results = @()
foreach ($step in $steps) {
    $result = Run-Step -Name $step.Name -Command $step.Command
    $results += $result
    if (-not $result.Passed) {
        break
    }
}

$timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ssK"
$overallPass = ($results | Where-Object { -not $_.Passed }).Count -eq 0

$parityTotal = 0
$parityPassed = 0
$parityRate = 0.0
$denominatorGate = $false

if (Test-Path $dashboardPath) {
    $dashboard = Get-Content $dashboardPath -Raw | ConvertFrom-Json
    $parityTotal = [int]$dashboard.total
    $parityPassed = [int]$dashboard.passed
    if ($parityTotal -gt 0) {
        $parityRate = [math]::Round((100.0 * $parityPassed / $parityTotal), 2)
    }
    $denominatorGate = $parityTotal -ge 500
}

$lines = @()
$lines += "# Milestone 4 Gate Check"
$lines += ""
$lines += "Timestamp: $timestamp"
$lines += "Overall status: $(if ($overallPass) { 'PASS' } else { 'FAIL' })"
$lines += ""
$lines += "## Command Results"
$lines += ""
$lines += "| Step | Status | Seconds | Command |"
$lines += "|---|---|---:|---|"
foreach ($r in $results) {
    $lines += "| $($r.Name) | $(if ($r.Passed) { 'PASS' } else { 'FAIL' }) | $($r.ElapsedSeconds) | ``$($r.Command)`` |"
}
$lines += ""
$lines += "## Parity Snapshot"
$lines += ""
$lines += "- Replay scenarios: $parityTotal"
$lines += "- Replay pass rate: $parityRate% ($parityPassed/$parityTotal)"
$lines += "- Denominator gate (>=500 scenarios/day): $(if ($denominatorGate) { 'PASS' } else { 'FAIL' })"

if (-not (Test-Path "target")) {
    New-Item -ItemType Directory -Path "target" | Out-Null
}

Set-Content -Path $summaryPath -Value ($lines -join "`n")
Write-Host "Wrote $summaryPath"

if (-not $overallPass) {
    exit 1
}
