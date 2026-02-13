param(
  [string]$BaselineDate = (Get-Date -Format 'yyyy-MM-dd'),
  [int]$PerfIterations = 200
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Resolve-RepoRoot {
  if ($PSScriptRoot) {
    return (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
  }
  return (Get-Location).Path
}

function Ensure-File {
  param([string]$Path)
  if (-not (Test-Path -LiteralPath $Path)) {
    throw "Required file not found: $Path"
  }
}

function Get-Json {
  param([string]$Path)
  return Get-Content -LiteralPath $Path -Raw | ConvertFrom-Json
}

function New-FileDigestRecord {
  param([string]$FullPath, [string]$RootDir)

  $item = Get-Item -LiteralPath $FullPath
  $hash = Get-FileHash -Algorithm SHA256 -LiteralPath $FullPath
  $root = [System.IO.Path]::GetFullPath($RootDir).TrimEnd('\', '/')
  $full = [System.IO.Path]::GetFullPath($FullPath)
  $relativePath = $full
  if ($full.StartsWith($root, [System.StringComparison]::OrdinalIgnoreCase)) {
    $relativePath = $full.Substring($root.Length).TrimStart('\', '/')
  }
  $relativePath = $relativePath.Replace('\', '/')

  return [pscustomobject]@{
    relative_path = $relativePath
    size_bytes = [int64]$item.Length
    sha256 = $hash.Hash.ToLowerInvariant()
    last_write_utc = $item.LastWriteTimeUtc.ToString('o')
  }
}

$repoRoot = Resolve-RepoRoot
$targetDir = Join-Path $repoRoot 'target'
$snapshotDir = Join-Path $targetDir ("m5-baseline\" + $BaselineDate)
$freezeJsonPath = Join-Path $targetDir 'm5-m4-baseline-freeze.json'
$freezeMdPath = Join-Path $targetDir 'm5-m4-baseline-freeze.md'

if (-not (Test-Path -LiteralPath $targetDir)) {
  New-Item -ItemType Directory -Path $targetDir | Out-Null
}
if (-not (Test-Path -LiteralPath $snapshotDir)) {
  New-Item -ItemType Directory -Path $snapshotDir -Force | Out-Null
}

$runGateScript = Join-Path $repoRoot 'scripts\run-m4-gate.ps1'
Ensure-File -Path $runGateScript

Write-Output "Running M4 gate bundle..."
powershell -ExecutionPolicy Bypass -File $runGateScript
if ($LASTEXITCODE -ne 0) {
  throw "run-m4-gate failed with exit code $LASTEXITCODE."
}

Write-Output "Capturing performance snapshot (iterations=$PerfIterations)..."
$perfRaw = cargo run -q -p omega-tools --bin perf_baseline -- --iterations $PerfIterations
if ($LASTEXITCODE -ne 0) {
  throw "perf_baseline failed with exit code $LASTEXITCODE."
}
$perf = $perfRaw | ConvertFrom-Json

$sourceFiles = @(
  'm4-gate-check-summary.md',
  'ws-d-regression-dashboard.json',
  'ws-d-regression-dashboard.md',
  'ws-d-determinism-report.json',
  'ws-d-determinism-report.md',
  'frontend-command-parity.json',
  'frontend-command-parity.md',
  'save-compat-report.json',
  'save-compat-report.md',
  'm4-burnin-window.json',
  'm4-burnin-window.md',
  'm4-crashfree-window.json',
  'm4-crashfree-window.md'
)

foreach ($name in $sourceFiles) {
  $path = Join-Path $targetDir $name
  Ensure-File -Path $path
}

$dashboard = Get-Json -Path (Join-Path $targetDir 'ws-d-regression-dashboard.json')
$determinism = Get-Json -Path (Join-Path $targetDir 'ws-d-determinism-report.json')
$frontendParity = Get-Json -Path (Join-Path $targetDir 'frontend-command-parity.json')
$saveCompat = Get-Json -Path (Join-Path $targetDir 'save-compat-report.json')
$burnIn = Get-Json -Path (Join-Path $targetDir 'm4-burnin-window.json')
$crashFree = Get-Json -Path (Join-Path $targetDir 'm4-crashfree-window.json')

$summaryContent = Get-Content -LiteralPath (Join-Path $targetDir 'm4-gate-check-summary.md') -Raw
$overallStatus = 'UNKNOWN'
if ($summaryContent -match 'Overall status: (\w+)') {
  $overallStatus = $Matches[1]
}

$replayPassRatePercent = [double]$dashboard.pass_rate_percent_x100
if ($replayPassRatePercent -gt 100.0) {
  $replayPassRatePercent = $replayPassRatePercent / 100.0
}

$copiedFiles = New-Object System.Collections.Generic.List[string]
foreach ($name in $sourceFiles) {
  $src = Join-Path $targetDir $name
  $dst = Join-Path $snapshotDir $name
  Copy-Item -LiteralPath $src -Destination $dst -Force
  $copiedFiles.Add($dst)
}

$perfPath = Join-Path $snapshotDir ("perf-baseline-iterations-{0}.json" -f $PerfIterations)
$perf | ConvertTo-Json -Depth 10 | Set-Content -LiteralPath $perfPath -Encoding UTF8
$copiedFiles.Add($perfPath)

$budgetPath = Join-Path $repoRoot 'docs\quality\perf-budgets.json'
Ensure-File -Path $budgetPath
$budgetDst = Join-Path $snapshotDir 'perf-budgets.json'
Copy-Item -LiteralPath $budgetPath -Destination $budgetDst -Force
$copiedFiles.Add($budgetDst)

$filesManifest = @()
foreach ($file in $copiedFiles) {
  $filesManifest += New-FileDigestRecord -FullPath $file -RootDir $repoRoot
}
$filesManifest = $filesManifest | Sort-Object relative_path

$snapshotRelative = $snapshotDir
$rootNorm = [System.IO.Path]::GetFullPath($repoRoot).TrimEnd('\', '/')
$snapNorm = [System.IO.Path]::GetFullPath($snapshotDir)
if ($snapNorm.StartsWith($rootNorm, [System.StringComparison]::OrdinalIgnoreCase)) {
  $snapshotRelative = $snapNorm.Substring($rootNorm.Length).TrimStart('\', '/')
}
$snapshotRelative = $snapshotRelative.Replace('\', '/')

$baseline = [pscustomobject]@{
  baseline_id = "m4-freeze-$BaselineDate"
  created_at_utc = (Get-Date).ToUniversalTime().ToString('o')
  created_by_script = 'scripts/freeze-m4-baseline.ps1'
  source_gate_script = 'scripts/run-m4-gate.ps1'
  status = if ($overallStatus -eq 'PASS') { 'PASS' } else { 'FAIL' }
  m4_gate = [pscustomobject]@{
    overall_status = $overallStatus
    replay_scenarios = [int]$dashboard.total
    replay_passed = [int]$dashboard.passed
    replay_failed = [int]$dashboard.failed
    replay_pass_rate_percent = [double]$replayPassRatePercent
    critical_path_total = [int]$dashboard.critical_path_total
    critical_path_failed = [int]$dashboard.critical_path_failed
    determinism_divergent_runs = [int]$determinism.divergent_runs
    frontend_mismatched_cases = [int]$frontendParity.mismatched_cases
    save_compat_failed = [int]$saveCompat.failed
    burn_in_status = [string]$burnIn.status
    crash_free_status = [string]$crashFree.status
    crash_free_rate_percent = [double]$crashFree.crash_free_rate_percent
  }
  perf_baseline = [pscustomobject]@{
    iterations = [int]$perf.iterations
    scenarios_per_iteration = [int]$perf.scenarios_per_iteration
    avg_ms = [double]$perf.avg_ms
    p95_ms = [double]$perf.p95_ms
    max_ms = [double]$perf.max_ms
  }
  snapshot_directory = $snapshotRelative
  files = $filesManifest
}

$baseline | ConvertTo-Json -Depth 12 | Set-Content -LiteralPath $freezeJsonPath -Encoding UTF8

$lines = New-Object System.Collections.Generic.List[string]
$lines.Add('# M5 Baseline Freeze (M4 Reference)')
$lines.Add('')
$lines.Add("- Baseline ID: $($baseline.baseline_id)")
$lines.Add("- Created at (UTC): $($baseline.created_at_utc)")
$lines.Add("- Overall M4 gate status: $($baseline.m4_gate.overall_status)")
$lines.Add("- Snapshot directory: $($baseline.snapshot_directory)")
$lines.Add('')
$lines.Add('## Stability Snapshot')
$lines.Add('')
$lines.Add("- Replay pass rate: $($baseline.m4_gate.replay_pass_rate_percent)% ($($baseline.m4_gate.replay_passed)/$($baseline.m4_gate.replay_scenarios))")
$lines.Add("- Critical-path failures: $($baseline.m4_gate.critical_path_failed) of $($baseline.m4_gate.critical_path_total)")
$lines.Add("- Determinism divergent runs: $($baseline.m4_gate.determinism_divergent_runs)")
$lines.Add("- Frontend mismatched cases: $($baseline.m4_gate.frontend_mismatched_cases)")
$lines.Add("- Save compatibility failures: $($baseline.m4_gate.save_compat_failed)")
$lines.Add("- Burn-in status: $($baseline.m4_gate.burn_in_status)")
$lines.Add("- Crash-free status: $($baseline.m4_gate.crash_free_status) ($($baseline.m4_gate.crash_free_rate_percent)%)")
$lines.Add('')
$lines.Add('## Performance Snapshot')
$lines.Add('')
$lines.Add("- Iterations: $($baseline.perf_baseline.iterations)")
$lines.Add("- Scenarios per iteration: $($baseline.perf_baseline.scenarios_per_iteration)")
$lines.Add("- Avg ms: $([math]::Round($baseline.perf_baseline.avg_ms, 6))")
$lines.Add("- P95 ms: $([math]::Round($baseline.perf_baseline.p95_ms, 6))")
$lines.Add("- Max ms: $([math]::Round($baseline.perf_baseline.max_ms, 6))")
$lines.Add('')
$lines.Add('## Files')
$lines.Add('')
$lines.Add('| File | Size (bytes) | SHA256 |')
$lines.Add('|---|---:|---|')
foreach ($file in $baseline.files) {
  $lines.Add("| $($file.relative_path) | $($file.size_bytes) | $($file.sha256) |")
}

$lines | Set-Content -LiteralPath $freezeMdPath -Encoding UTF8

Write-Output "Wrote $freezeJsonPath"
Write-Output "Wrote $freezeMdPath"
Write-Output "Snapshot: $snapshotDir"
