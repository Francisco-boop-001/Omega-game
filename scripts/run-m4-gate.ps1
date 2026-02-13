param(
  [string]$RepoRoot = (Split-Path -Parent $PSScriptRoot)
)

$ErrorActionPreference = 'Stop'
Set-Location $RepoRoot

$targetDir = Join-Path $RepoRoot 'target'
if (!(Test-Path $targetDir)) {
  New-Item -ItemType Directory -Path $targetDir | Out-Null
}

function Invoke-Step {
  param(
    [Parameter(Mandatory = $true)][string]$Name,
    [Parameter(Mandatory = $true)][string]$Command
  )

  $start = Get-Date
  $status = 'PASS'
  $message = ''
  try {
    (& { Invoke-Expression $Command }) | Out-Host
    if ($LASTEXITCODE -ne 0) {
      throw "command exited with code $LASTEXITCODE"
    }
  }
  catch {
    $status = 'FAIL'
    $message = $_.Exception.Message
  }
  $elapsed = [Math]::Round(((Get-Date) - $start).TotalSeconds, 2)
  return [pscustomobject]@{
    step = $Name
    status = $status
    seconds = $elapsed
    command = $Command
    message = $message
  }
}

$steps = @(
  @{ name = 'fmt'; command = 'cargo fmt --all -- --check' },
  @{ name = 'clippy'; command = 'cargo clippy --workspace --all-targets -- -D warnings' },
  @{ name = 'tests'; command = 'cargo test --workspace' },
  @{ name = 'replay-dashboard'; command = 'cargo run -p omega-tools --bin replay_tool -- --min-scenarios 500 --summary-only' },
  @{ name = 'determinism'; command = 'cargo run -p omega-tools --bin determinism_check -- --runs-per-fixture 20' },
  @{ name = 'frontend-parity'; command = 'cargo run -p omega-tools --bin frontend_parity' },
  @{ name = 'save-compat'; command = 'cargo run -p omega-tools --bin save_compat_report' },
  @{ name = 'fuzz-smoke'; command = 'cargo run -p omega-tools --bin fuzz_smoke' },
  @{ name = 'perf-budget'; command = 'cargo run -p omega-tools --bin perf_baseline -- --check' }
)

$results = @()
foreach ($step in $steps) {
  $result = Invoke-Step -Name $step.name -Command $step.command
  $results += $result
  if ($result.status -eq 'FAIL') {
    break
  }
}

$overall = if (($results | Where-Object { $_.status -eq 'FAIL' }).Count -eq 0) { 'PASS' } else { 'FAIL' }

$dashboardPath = Join-Path $targetDir 'ws-d-regression-dashboard.json'
$determinismPath = Join-Path $targetDir 'ws-d-determinism-report.json'
$parityPath = Join-Path $targetDir 'frontend-command-parity.json'
$saveCompatPath = Join-Path $targetDir 'save-compat-report.json'
$flakePath = Join-Path $RepoRoot 'docs\quality\flake_exclusions.json'
$defectPath = Join-Path $RepoRoot 'docs\migration\PARITY_DEFECT_BOARD.json'

$dashboard = if (Test-Path $dashboardPath) { Get-Content $dashboardPath -Raw | ConvertFrom-Json } else { $null }
$determinism = if (Test-Path $determinismPath) { Get-Content $determinismPath -Raw | ConvertFrom-Json } else { $null }
$parity = if (Test-Path $parityPath) { Get-Content $parityPath -Raw | ConvertFrom-Json } else { $null }
$saveCompat = if (Test-Path $saveCompatPath) { Get-Content $saveCompatPath -Raw | ConvertFrom-Json } else { $null }
$flake = if (Test-Path $flakePath) { Get-Content $flakePath -Raw | ConvertFrom-Json } else { $null }
$defects = if (Test-Path $defectPath) { Get-Content $defectPath -Raw | ConvertFrom-Json } else { $null }

$totalScenarios = if ($dashboard) { [int]$dashboard.total } else { 0 }
$passedScenarios = if ($dashboard) { [int]$dashboard.passed } else { 0 }
$failedScenarios = if ($dashboard) { [int]$dashboard.failed } else { 0 }
$criticalFailed = if ($dashboard) { [int]$dashboard.critical_path_failed } else { 0 }
$replayPassRate = if ($totalScenarios -gt 0) { [Math]::Round(($passedScenarios / $totalScenarios) * 100, 2) } else { 0.0 }

$denominatorPass = $totalScenarios -ge 500
$replayRatePass = $replayPassRate -ge 98.0
$criticalPass = ($criticalFailed -eq 0)
$determinismPass = ($determinism -ne $null -and [bool]$determinism.passed)
$frontendParityPass = ($parity -ne $null -and [int]$parity.mismatched_cases -eq 0)
$saveCompatPass = ($saveCompat -ne $null -and [int]$saveCompat.failed -eq 0)

$flakeMaxRatio = if ($flake -and $flake.policy -and $flake.policy.max_ratio -ne $null) { [double]$flake.policy.max_ratio } else { 0.02 }
$flakeExclusions = if ($flake -and $flake.exclusions) { @($flake.exclusions) } else { @() }
$flakeIssueOwnerPass = $true
foreach ($item in $flakeExclusions) {
  if ([string]::IsNullOrWhiteSpace([string]$item.issue) -or [string]::IsNullOrWhiteSpace([string]$item.owner)) {
    $flakeIssueOwnerPass = $false
    break
  }
}
$flakeRatio = if ($totalScenarios -gt 0) { $flakeExclusions.Count / $totalScenarios } else { 0.0 }
$flakePolicyPass = ($flakeRatio -lt $flakeMaxRatio) -and $flakeIssueOwnerPass

$openP0 = if ($defects -and $defects.summary -and $defects.summary.open_p0 -ne $null) { [int]$defects.summary.open_p0 } else { 0 }
$openP1 = if ($defects -and $defects.summary -and $defects.summary.open_p1 -ne $null) { [int]$defects.summary.open_p1 } else { 0 }
$severityPass = ($openP0 -eq 0 -and $openP1 -le 3)

$gatePass = $denominatorPass -and $replayRatePass -and $criticalPass -and $determinismPass -and $frontendParityPass -and $saveCompatPass -and $flakePolicyPass -and $severityPass
$overallStatus = if ($overall -eq 'PASS' -and $gatePass) { 'PASS' } else { 'FAIL' }

$summaryPath = Join-Path $targetDir 'm4-gate-check-summary.md'
$lines = New-Object System.Collections.Generic.List[string]
$lines.Add('# Milestone 4 Gate Check')
$lines.Add('')
$lines.Add("Timestamp: $(Get-Date -Format o)")
$lines.Add("Overall status: $overallStatus")
$lines.Add('')
$lines.Add('## Command Results')
$lines.Add('')
$lines.Add('| Step | Status | Seconds | Command |')
$lines.Add('|---|---|---:|---|')
foreach ($result in $results) {
  $lines.Add("| $($result.step) | $($result.status) | $($result.seconds) | ``$($result.command)`` |")
}
$lines.Add('')
$lines.Add('## Parity Snapshot')
$lines.Add('')
$lines.Add("- Replay scenarios: $totalScenarios")
$lines.Add("- Replay pass rate: $replayPassRate% ($passedScenarios/$totalScenarios)")
$lines.Add("- Critical-path failures: $criticalFailed")
$lines.Add("- Denominator gate (>=500 scenarios/day): $(if ($denominatorPass) { 'PASS' } else { 'FAIL' })")
$lines.Add("- Frontend command parity gate: $(if ($frontendParityPass) { 'PASS' } else { 'FAIL' })")
$lines.Add("- Determinism gate (N=20 repeats): $(if ($determinismPass) { 'PASS' } else { 'FAIL' })")
$lines.Add("- Save compatibility gate: $(if ($saveCompatPass) { 'PASS' } else { 'FAIL' })")
$lines.Add("- Flake policy gate: $(if ($flakePolicyPass) { 'PASS' } else { 'FAIL' })")
$lines.Add("- Severity gate (P0=0, P1<=3): $(if ($severityPass) { 'PASS' } else { 'FAIL' })")
$lines.Add('')
$lines.Add('## Gate Status')
$lines.Add('')
$lines.Add("| Gate | Status |")
$lines.Add("|---|---|")
$lines.Add("| Replay denominator >=500 | $(if ($denominatorPass) { 'PASS' } else { 'FAIL' }) |")
$lines.Add("| Replay pass rate >=98% | $(if ($replayRatePass) { 'PASS' } else { 'FAIL' }) |")
$lines.Add("| Critical path 100% | $(if ($criticalPass) { 'PASS' } else { 'FAIL' }) |")
$lines.Add("| Determinism N=20 | $(if ($determinismPass) { 'PASS' } else { 'FAIL' }) |")
$lines.Add("| Frontend parity 100% | $(if ($frontendParityPass) { 'PASS' } else { 'FAIL' }) |")
$lines.Add("| Save compatibility 100% | $(if ($saveCompatPass) { 'PASS' } else { 'FAIL' }) |")
$lines.Add("| Flake policy <2% + owner/issue | $(if ($flakePolicyPass) { 'PASS' } else { 'FAIL' }) |")
$lines.Add("| Severity gate (P0=0, P1<=3) | $(if ($severityPass) { 'PASS' } else { 'FAIL' }) |")
$lines.Add('')

$lines | Set-Content -Encoding UTF8 $summaryPath
Write-Output "Wrote $summaryPath"

if ($overallStatus -ne 'PASS') {
  exit 1
}
