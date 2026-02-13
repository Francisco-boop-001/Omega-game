param(
  [string]$RepoRoot = (Split-Path -Parent $PSScriptRoot),
  [switch]$StrictArtifactMode
)

$ErrorActionPreference = 'Stop'
Set-Location $RepoRoot

$targetDir = Join-Path $RepoRoot 'target'
if (!(Test-Path -LiteralPath $targetDir)) {
  New-Item -ItemType Directory -Path $targetDir | Out-Null
}

function Read-OverallStatusFromSummary {
  param([string]$SummaryPath)
  if (!(Test-Path -LiteralPath $SummaryPath)) {
    return 'MISSING'
  }
  $raw = Get-Content -LiteralPath $SummaryPath -Raw
  if ($raw -match 'Overall status: (\w+)') {
    return $Matches[1]
  }
  return 'UNKNOWN'
}

function Get-ArtifactRecord {
  param(
    [Parameter(Mandatory = $true)][string]$Path,
    [Parameter(Mandatory = $true)][string]$Category
  )

  $rootNorm = [System.IO.Path]::GetFullPath($RepoRoot).TrimEnd('\', '/')
  $pathNorm = [System.IO.Path]::GetFullPath($Path)
  $displayPath = $pathNorm
  if ($pathNorm.StartsWith($rootNorm, [System.StringComparison]::OrdinalIgnoreCase)) {
    $displayPath = $pathNorm.Substring($rootNorm.Length).TrimStart('\', '/')
  }
  $displayPath = $displayPath.Replace('\', '/')

  if (Test-Path -LiteralPath $Path) {
    $item = Get-Item -LiteralPath $Path
    return [pscustomobject]@{
      path = $displayPath
      category = $Category
      status = 'PRESENT'
      size_bytes = [int64]$item.Length
      last_write_utc = $item.LastWriteTimeUtc.ToString('o')
    }
  }

  return [pscustomobject]@{
    path = $displayPath
    category = $Category
    status = 'MISSING'
    size_bytes = 0
    last_write_utc = $null
  }
}

$runM4Gate = Join-Path $RepoRoot 'scripts\run-m4-gate.ps1'
if (!(Test-Path -LiteralPath $runM4Gate)) {
  throw "Required script not found: $runM4Gate"
}

Write-Output "Running foundational M4 gate bundle..."
& $runM4Gate
if ($LASTEXITCODE -ne 0) {
  throw "run-m4-gate failed with exit code $LASTEXITCODE"
}

Write-Output "Running M5 frontend E2E journey automation..."
cargo run -p omega-tools --bin m5_e2e_journey
if ($LASTEXITCODE -ne 0) {
  throw "m5_e2e_journey failed with exit code $LASTEXITCODE"
}

Write-Output "Running M5 lifecycle parity report..."
cargo run -p omega-tools --bin m5_lifecycle_parity
if ($LASTEXITCODE -ne 0) {
  throw "m5_lifecycle_parity failed with exit code $LASTEXITCODE"
}

Write-Output "Running M5 boot reliability report..."
cargo run -p omega-tools --bin m5_boot_reliability
if ($LASTEXITCODE -ne 0) {
  throw "m5_boot_reliability failed with exit code $LASTEXITCODE"
}

Write-Output "Running M5 perf budget report..."
cargo run -p omega-tools --bin m5_perf_budget_report
if ($LASTEXITCODE -ne 0) {
  throw "m5_perf_budget_report failed with exit code $LASTEXITCODE"
}

Write-Output "Running M5 frame-time report..."
cargo run -p omega-tools --bin m5_frame_time_report
if ($LASTEXITCODE -ne 0) {
  throw "m5_frame_time_report failed with exit code $LASTEXITCODE"
}

Write-Output "Running M5 security audit..."
cargo run -p omega-tools --bin m5_security_audit
if ($LASTEXITCODE -ne 0) {
  throw "m5_security_audit failed with exit code $LASTEXITCODE"
}

Write-Output "Running M5 weekly fuzz report..."
cargo run -p omega-tools --bin m5_fuzz_weekly_report
if ($LASTEXITCODE -ne 0) {
  throw "m5_fuzz_weekly_report failed with exit code $LASTEXITCODE"
}

Write-Output "Running M5 release operations checklist..."
cargo run -p omega-tools --bin m5_release_operations_checklist
if ($LASTEXITCODE -ne 0) {
  throw "m5_release_operations_checklist failed with exit code $LASTEXITCODE"
}

$m4SummaryPath = Join-Path $targetDir 'm4-gate-check-summary.md'
$m4OverallStatus = Read-OverallStatusFromSummary -SummaryPath $m4SummaryPath

$baselineFreezePath = Join-Path $targetDir 'm5-m4-baseline-freeze.json'
$baselineStatus = 'MISSING'
if (Test-Path -LiteralPath $baselineFreezePath) {
  try {
    $baseline = Get-Content -LiteralPath $baselineFreezePath -Raw | ConvertFrom-Json
    if ($baseline.status) {
      $baselineStatus = [string]$baseline.status
    } else {
      $baselineStatus = 'UNKNOWN'
    }
  }
  catch {
    $baselineStatus = 'INVALID'
  }
}

$artifactSpecs = @(
  @{ path = (Join-Path $targetDir 'm5-e2e-journey-report.json'); category = 'productization' },
  @{ path = (Join-Path $targetDir 'm5-e2e-journey-report.md'); category = 'productization' },
  @{ path = (Join-Path $targetDir 'm5-lifecycle-parity-report.json'); category = 'productization' },
  @{ path = (Join-Path $targetDir 'm5-lifecycle-parity-report.md'); category = 'productization' },
  @{ path = (Join-Path $targetDir 'm5-boot-reliability.json'); category = 'reliability' },
  @{ path = (Join-Path $targetDir 'm5-boot-reliability.md'); category = 'reliability' },
  @{ path = (Join-Path $targetDir 'm5-perf-budget-report.json'); category = 'performance' },
  @{ path = (Join-Path $targetDir 'm5-perf-budget-report.md'); category = 'performance' },
  @{ path = (Join-Path $targetDir 'm5-frame-time-report.json'); category = 'performance' },
  @{ path = (Join-Path $targetDir 'm5-frame-time-report.md'); category = 'performance' },
  @{ path = (Join-Path $targetDir 'm5-security-audit.json'); category = 'security' },
  @{ path = (Join-Path $targetDir 'm5-fuzz-weekly-report.md'); category = 'security' },
  @{ path = (Join-Path $targetDir 'm5-release-operations-checklist.md'); category = 'release' },
  @{ path = (Join-Path $targetDir 'm5-m4-baseline-freeze.json'); category = 'baseline' },
  @{ path = (Join-Path $targetDir 'm5-m4-baseline-freeze.md'); category = 'baseline' }
)

$artifactRecords = @()
foreach ($spec in $artifactSpecs) {
  $artifactRecords += Get-ArtifactRecord -Path $spec.path -Category $spec.category
}

$presentCount = ($artifactRecords | Where-Object { $_.status -eq 'PRESENT' }).Count
$missingCount = ($artifactRecords | Where-Object { $_.status -eq 'MISSING' }).Count
$totalCount = $artifactRecords.Count
$coveragePercent = if ($totalCount -gt 0) { [Math]::Round(($presentCount / $totalCount) * 100, 2) } else { 0.0 }

$pendingPaths = @($artifactRecords | Where-Object { $_.status -eq 'MISSING' } | Select-Object -ExpandProperty path)

$strictArtifactsPass = if ($StrictArtifactMode.IsPresent) { $missingCount -eq 0 } else { $true }
$overallStatus = if ($m4OverallStatus -eq 'PASS' -and $baselineStatus -eq 'PASS' -and $strictArtifactsPass) { 'PASS' } else { 'FAIL' }
$skeletonMode = if ($StrictArtifactMode.IsPresent) { 'STRICT' } else { 'SKELETON' }

$artifactSummaryPathJson = Join-Path $targetDir 'm5-artifact-summary.json'
$artifactSummaryPathMd = Join-Path $targetDir 'm5-artifact-summary.md'
$gateSummaryPath = Join-Path $targetDir 'm5-gate-check-summary.md'

$artifactSummary = [pscustomobject]@{
  timestamp_utc = (Get-Date).ToUniversalTime().ToString('o')
  mode = $skeletonMode
  totals = [pscustomobject]@{
    present = $presentCount
    missing = $missingCount
    total = $totalCount
    coverage_percent = $coveragePercent
  }
  artifacts = $artifactRecords
}

$artifactSummary | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $artifactSummaryPathJson -Encoding UTF8

$artifactLines = New-Object System.Collections.Generic.List[string]
$artifactLines.Add('# M5 Artifact Coverage Summary')
$artifactLines.Add('')
$artifactLines.Add("Timestamp: $($artifactSummary.timestamp_utc)")
$artifactLines.Add("Mode: $skeletonMode")
$artifactLines.Add("Coverage: $presentCount/$totalCount ($coveragePercent%)")
$artifactLines.Add('')
$artifactLines.Add('| Category | Path | Status | Size (bytes) | Last write (UTC) |')
$artifactLines.Add('|---|---|---|---:|---|')
foreach ($record in $artifactRecords) {
  $artifactLines.Add("| $($record.category) | $($record.path) | $($record.status) | $($record.size_bytes) | $($record.last_write_utc) |")
}
$artifactLines | Set-Content -LiteralPath $artifactSummaryPathMd -Encoding UTF8

$gateLines = New-Object System.Collections.Generic.List[string]
$gateLines.Add('# Milestone 5 Gate Check')
$gateLines.Add('')
$gateLines.Add("Timestamp: $(Get-Date -Format o)")
$gateLines.Add("Mode: $skeletonMode")
$gateLines.Add("Overall status: $overallStatus")
$gateLines.Add('')
$gateLines.Add('## Foundation Status')
$gateLines.Add('')
$gateLines.Add("- M4 foundational gate: $m4OverallStatus")
$gateLines.Add("- Baseline freeze package status: $baselineStatus")
$gateLines.Add("- Artifact coverage: $presentCount/$totalCount ($coveragePercent%)")
$gateLines.Add('')
$gateLines.Add('## Pending M5 Artifacts')
$gateLines.Add('')
if ($pendingPaths.Count -eq 0) {
  $gateLines.Add('- None')
} else {
  foreach ($pending in $pendingPaths) {
    $gateLines.Add("- $pending")
  }
}
$gateLines.Add('')
$gateLines.Add('## Notes')
$gateLines.Add('')
if ($StrictArtifactMode.IsPresent) {
  $gateLines.Add('- Strict mode is enabled: missing M5 deliverable artifacts fail the workflow.')
} else {
  $gateLines.Add('- Skeleton mode is enabled: missing M5 deliverable artifacts are reported but do not fail the workflow.')
  $gateLines.Add('- Use `-StrictArtifactMode` after M5 artifact-producing jobs are implemented.')
}
$gateLines.Add('')

$gateLines | Set-Content -LiteralPath $gateSummaryPath -Encoding UTF8

Write-Output "Wrote $gateSummaryPath"
Write-Output "Wrote $artifactSummaryPathJson"
Write-Output "Wrote $artifactSummaryPathMd"

if ($overallStatus -ne 'PASS') {
  exit 1
}
