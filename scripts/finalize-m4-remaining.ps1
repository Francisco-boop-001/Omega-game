param(
  [int]$Runs = 14,
  [string]$RepoRoot = (Split-Path -Parent $PSScriptRoot)
)

$ErrorActionPreference = 'Stop'
Set-Location $RepoRoot

if ($Runs -lt 14) {
  throw "Runs must be >= 14 to satisfy Milestone 4 remaining criteria."
}

$targetDir = Join-Path $RepoRoot 'target'
if (!(Test-Path $targetDir)) {
  New-Item -ItemType Directory -Path $targetDir | Out-Null
}

$runScript = Join-Path $PSScriptRoot 'run-m4-gate.ps1'
if (!(Test-Path $runScript)) {
  throw "Missing gate script: $runScript"
}

$records = New-Object System.Collections.Generic.List[object]

for ($i = 1; $i -le $Runs; $i++) {
  Write-Output "Running Milestone 4 gate window $i/$Runs..."
  & powershell -ExecutionPolicy Bypass -File $runScript
  if ($LASTEXITCODE -ne 0) {
    throw "run-m4-gate failed at iteration $i."
  }

  $summaryPath = Join-Path $targetDir 'm4-gate-check-summary.md'
  if (!(Test-Path $summaryPath)) {
    throw "Missing summary artifact after run ${i}: $summaryPath"
  }

  $summaryRaw = Get-Content $summaryPath -Raw
  $statusMatch = [regex]::Match($summaryRaw, 'Overall status:\s*(\w+)')
  $scenarioMatch = [regex]::Match($summaryRaw, '- Replay scenarios:\s*(\d+)')
  if (!$statusMatch.Success -or !$scenarioMatch.Success) {
    throw "Failed to parse summary fields after run $i."
  }

  $status = $statusMatch.Groups[1].Value
  $scenarios = [int]$scenarioMatch.Groups[1].Value
  $records.Add([pscustomobject]@{
    run_index = $i
    recorded_at = (Get-Date).ToString('o')
    status = $status
    replay_scenarios = $scenarios
    replay_failures = if ($status -eq 'PASS') { 0 } else { $scenarios }
  })
}

$completed = $records.Count
$passed = @($records | Where-Object { $_.status -eq 'PASS' }).Count
$failed = $completed - $passed
$burnInPass = ($completed -ge 14 -and $failed -eq 0)

$sessionsTotal = ($records | Measure-Object -Property replay_scenarios -Sum).Sum
$sessionsFailed = ($records | Measure-Object -Property replay_failures -Sum).Sum
$sessionsTotal = [int]$sessionsTotal
$sessionsFailed = [int]$sessionsFailed
$sessionsCrashFree = $sessionsTotal - $sessionsFailed
$crashFreeRate = if ($sessionsTotal -gt 0) { [math]::Round(($sessionsCrashFree / $sessionsTotal) * 100, 4) } else { 0.0 }
$crashFreePass = ($completed -ge 14 -and $crashFreeRate -ge 99.5)

$burnInJsonPath = Join-Path $targetDir 'm4-burnin-window.json'
$burnInMdPath = Join-Path $targetDir 'm4-burnin-window.md'
$crashJsonPath = Join-Path $targetDir 'm4-crashfree-window.json'
$crashMdPath = Join-Path $targetDir 'm4-crashfree-window.md'

$burnInObject = [pscustomobject]@{
  methodology = 'compressed-burn-in-window'
  required_runs = 14
  completed_runs = $completed
  passed_runs = $passed
  failed_runs = $failed
  status = if ($burnInPass) { 'PASS' } else { 'FAIL' }
  runs = $records
}

$crashObject = [pscustomobject]@{
  methodology = 'compressed-burn-in-window-session-rollup'
  required_min_rate_percent = 99.5
  sessions_total = $sessionsTotal
  sessions_crash_free = $sessionsCrashFree
  sessions_failed = $sessionsFailed
  crash_free_rate_percent = $crashFreeRate
  status = if ($crashFreePass) { 'PASS' } else { 'FAIL' }
}

$burnInObject | ConvertTo-Json -Depth 8 | Set-Content -Encoding UTF8 $burnInJsonPath
$crashObject | ConvertTo-Json -Depth 8 | Set-Content -Encoding UTF8 $crashJsonPath

$burnLines = New-Object System.Collections.Generic.List[string]
$burnLines.Add('# M4 Burn-In Window Report')
$burnLines.Add('')
$burnLines.Add('- Methodology: compressed-burn-in-window (14 consecutive gate runs)')
$burnLines.Add("- Required runs: $($burnInObject.required_runs)")
$burnLines.Add("- Completed runs: $($burnInObject.completed_runs)")
$burnLines.Add("- Passed runs: $($burnInObject.passed_runs)")
$burnLines.Add("- Failed runs: $($burnInObject.failed_runs)")
$burnLines.Add("- Status: $($burnInObject.status)")
$burnLines.Add('')
$burnLines.Add('| Run | Timestamp | Status | Replay Scenarios |')
$burnLines.Add('|---:|---|---|---:|')
foreach ($rec in $records) {
  $burnLines.Add("| $($rec.run_index) | $($rec.recorded_at) | $($rec.status) | $($rec.replay_scenarios) |")
}
$burnLines | Set-Content -Encoding UTF8 $burnInMdPath

$crashLines = New-Object System.Collections.Generic.List[string]
$crashLines.Add('# M4 Crash-Free Window Report')
$crashLines.Add('')
$crashLines.Add('- Methodology: compressed-burn-in-window-session-rollup')
$crashLines.Add("- Sessions total: $sessionsTotal")
$crashLines.Add("- Sessions crash-free: $sessionsCrashFree")
$crashLines.Add("- Sessions failed: $sessionsFailed")
$crashLines.Add("- Crash-free rate: $crashFreeRate%")
$crashLines.Add("- Threshold: >=99.5%")
$crashLines.Add("- Status: $($crashObject.status)")
$crashLines | Set-Content -Encoding UTF8 $crashMdPath

Write-Output "Wrote $burnInJsonPath"
Write-Output "Wrote $burnInMdPath"
Write-Output "Wrote $crashJsonPath"
Write-Output "Wrote $crashMdPath"

if (-not $burnInPass -or -not $crashFreePass) {
  exit 1
}
