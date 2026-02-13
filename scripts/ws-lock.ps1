param(
  [Parameter(Mandatory = $true)]
  [ValidateSet('claim','release','status')]
  [string]$Action,

  [Parameter(Mandatory = $true)]
  [ValidateSet('WS-A','WS-B','WS-C','WS-D','WS-E','WS-F','WS-G','WS-H','WS-I')]
  [string]$Workstream,

  [string]$Owner = $env:USERNAME
)

$ErrorActionPreference = 'Stop'
$ownerPattern = '^[A-Za-z0-9._-]+$'
$repoRoot = Split-Path -Parent $PSScriptRoot
$locksDir = Join-Path $repoRoot 'swarm\locks'
$lockPath = Join-Path $locksDir ("$Workstream.lock")

if (!(Test-Path $locksDir)) {
  New-Item -ItemType Directory -Path $locksDir | Out-Null
}

switch ($Action) {
  'claim' {
    if ([string]::IsNullOrWhiteSpace($Owner) -or $Owner -notmatch $ownerPattern) {
      Write-Error "Owner '$Owner' is invalid. Use [A-Za-z0-9._-]+ (example: github-handle)."
    }

    if (Test-Path $lockPath) {
      $existing = Get-Content $lockPath -Raw
      Write-Error "Lock already held for $Workstream. Existing lock:`n$existing"
    }

    $payload = @{
      workstream = $Workstream
      owner = $Owner
      host = $env:COMPUTERNAME
      pid = $PID
      claimed_at_utc = (Get-Date).ToUniversalTime().ToString('o')
    } | ConvertTo-Json -Depth 4

    $payload | Set-Content -Encoding UTF8 $lockPath
    Write-Output "Claimed $Workstream by $Owner"
  }

  'release' {
    if (!(Test-Path $lockPath)) {
      Write-Output "No lock present for $Workstream"
      exit 0
    }
    Remove-Item $lockPath
    Write-Output "Released $Workstream"
  }

  'status' {
    if (Test-Path $lockPath) {
      Write-Output "LOCKED: $Workstream"
      Get-Content $lockPath
    }
    else {
      Write-Output "UNLOCKED: $Workstream"
    }
  }
}
