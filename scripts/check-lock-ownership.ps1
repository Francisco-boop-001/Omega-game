param(
  [string]$Actor = $env:GITHUB_ACTOR
)

$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent $PSScriptRoot
$locksDir = Join-Path $repoRoot 'swarm\locks'
$ownerPattern = '^[A-Za-z0-9._-]+$'

function Get-WorkstreamsForPath([string]$path) {
  $normalized = $path.Replace('\\', '/').ToLowerInvariant()
  $set = New-Object System.Collections.Generic.HashSet[string]

  if ($normalized.StartsWith('.github/') -or $normalized -eq 'contributing.md' -or $normalized.StartsWith('docs/governance/') -or $normalized.StartsWith('swarm/')) {
    [void]$set.Add('WS-A')
  }
  if ($normalized.StartsWith('crates/omega-core/')) {
    [void]$set.Add('WS-B')
  }
  if ($normalized.StartsWith('crates/omega-content/')) {
    [void]$set.Add('WS-C')
  }
  if ($normalized.StartsWith('crates/omega-tools/')) {
    [void]$set.Add('WS-D')
  }
  if ($normalized.StartsWith('docs/migration/ws-d')) {
    [void]$set.Add('WS-D')
  }
  if ($normalized.StartsWith('crates/omega-save/')) {
    [void]$set.Add('WS-E')
  }
  if ($normalized.StartsWith('crates/omega-tui/')) {
    [void]$set.Add('WS-F')
  }
  if ($normalized.StartsWith('crates/omega-bevy/')) {
    [void]$set.Add('WS-G')
  }
  if ($normalized.StartsWith('docs/architecture/') -or $normalized.StartsWith('docs/migration/')) {
    [void]$set.Add('WS-I')
  }

  return $set
}

function Get-ChangedFiles([string]$eventName, [object]$eventJson) {
  $baseSha = $null
  $headSha = $null

  if ($eventName -eq 'pull_request' -or $eventName -eq 'pull_request_target') {
    $baseSha = $eventJson.pull_request.base.sha
    $headSha = $eventJson.pull_request.head.sha
  }
  elseif ($eventName -eq 'push') {
    $baseSha = $eventJson.before
    $headSha = $eventJson.after
  }

  if ([string]::IsNullOrWhiteSpace($baseSha) -or [string]::IsNullOrWhiteSpace($headSha)) {
    $baseSha = 'HEAD~1'
    $headSha = 'HEAD'
  }

  $diff = git diff --name-only $baseSha $headSha
  return @($diff | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
}

if ([string]::IsNullOrWhiteSpace($Actor)) {
  Write-Error 'Actor is empty. Set GITHUB_ACTOR or pass -Actor.'
}

if ($Actor -notmatch $ownerPattern) {
  Write-Error "Actor '$Actor' is invalid. Expected format [A-Za-z0-9._-]+."
}

if (!(Test-Path $locksDir)) {
  Write-Output 'No lock directory present. Skipping lock ownership check.'
  exit 0
}

$eventPath = $env:GITHUB_EVENT_PATH
$eventName = $env:GITHUB_EVENT_NAME
if ([string]::IsNullOrWhiteSpace($eventPath) -or !(Test-Path $eventPath)) {
  Write-Error 'GITHUB_EVENT_PATH not found. This script is intended for CI use.'
}

$eventJson = Get-Content $eventPath -Raw | ConvertFrom-Json
$changedFiles = Get-ChangedFiles -eventName $eventName -eventJson $eventJson

if ($changedFiles.Count -eq 0) {
  Write-Output 'No changed files detected. Skipping lock ownership check.'
  exit 0
}

$touched = New-Object System.Collections.Generic.HashSet[string]
foreach ($file in $changedFiles) {
  $ws = Get-WorkstreamsForPath $file
  foreach ($w in $ws) { [void]$touched.Add($w) }
}

if ($touched.Count -eq 0) {
  Write-Output 'No mapped workstreams touched by this change set. Skipping lock ownership check.'
  exit 0
}

$violations = @()
foreach ($workstream in $touched) {
  $lockFile = Join-Path $locksDir ("$workstream.lock")
  if (!(Test-Path $lockFile)) {
    continue
  }

  $lock = Get-Content $lockFile -Raw | ConvertFrom-Json
  $owner = [string]$lock.owner

  if ([string]::IsNullOrWhiteSpace($owner) -or $owner -notmatch $ownerPattern) {
    $violations += "Lock '$workstream' has invalid owner format: '$owner'"
    continue
  }

  if ($owner.ToLowerInvariant() -ne $Actor.ToLowerInvariant()) {
    $violations += "Lock '$workstream' is owned by '$owner' but CI actor is '$Actor'"
  }
}

if ($violations.Count -gt 0) {
  Write-Output 'Lock ownership check failed:'
  foreach ($v in $violations) {
    Write-Output "- $v"
  }
  exit 1
}

Write-Output "Lock ownership check passed for actor '$Actor'."
