param()

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $repoRoot

function Invoke-CargoTool {
  param(
    [Parameter(Mandatory = $true)][string]$ToolName
  )

  Write-Output "Running $ToolName ..."
  cargo run -p omega-tools --bin $ToolName
  if ($LASTEXITCODE -ne 0) {
    throw "$ToolName failed with exit code $LASTEXITCODE"
  }
}

Invoke-CargoTool -ToolName "classic_parity_manifest"
Invoke-CargoTool -ToolName "classic_core_model_parity"
Invoke-CargoTool -ToolName "classic_magic_item_parity"
Invoke-CargoTool -ToolName "classic_combat_encounter_parity"
Invoke-CargoTool -ToolName "classic_site_service_parity"
Invoke-CargoTool -ToolName "classic_progression_branch_matrix"
Invoke-CargoTool -ToolName "classic_frontend_workflow_parity"
Invoke-CargoTool -ToolName "classic_compatibility_matrix"
Invoke-CargoTool -ToolName "classic_state_integrity"
Invoke-CargoTool -ToolName "determinism_check"
Invoke-CargoTool -ToolName "classic_parity_reports"
Invoke-CargoTool -ToolName "classic_parity_baseline_freeze"
Invoke-CargoTool -ToolName "true_startup_parity"
Invoke-CargoTool -ToolName "true_parity_refresh"
Invoke-CargoTool -ToolName "strict_placeholder_audit"
Invoke-CargoTool -ToolName "recovery_rampart_startup_visual"
Invoke-CargoTool -ToolName "recovery_contract_guard"
Invoke-CargoTool -ToolName "recovery_refresh"

Write-Output "Recovery gate PASS."
