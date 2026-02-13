# Classic Omega Parity Scorecard

Superseded for closure decisions by `docs/migration/FULL_OMEGA_PARITY_RECOVERY_SCORECARD.md`.  
This document is retained as historical record only.

Date: 2026-02-07  
Plan reference: `docs/migration/CLASSIC_OMEGA_PARITY_EXECUTION_PLAN.md`

Status legend:
- `PASS`: track exit criteria met
- `FAIL`: track executed but exit criteria not yet met
- `PENDING`: track not yet executed

## Track Status

| Track | Requirement | Evidence | Status |
|---|---|---|---|
| `P0` | Parity manifest + gap ledger committed | `target/classic-parity-manifest.json`, `target/classic-command-parity-matrix.json`, `target/classic-content-cardinality-matrix.json`, `target/classic-gap-ledger.json` | PASS |
| `P1` | Core data model/rules expansion complete | `crates/omega-core/src/lib.rs`, `target/classic-core-model-parity.json` (`5/5`) | PASS |
| `P2` | Command surface parity complete | `target/classic-command-parity-matrix.json` (`missing=0`, `partial=0`, `key_conflict=0`), `target/frontend-command-parity.json` (`67/67`), `crates/omega-core/src/lib.rs` (fully modeled legacy command handlers) | PASS |
| `P3` | Combat/AI/encounter parity complete | `target/classic-combat-encounter-parity.json` (`4/4`) | PASS |
| `P4` | Magic/items/effects catalog parity complete | `target/classic-content-cardinality-matrix.json` (cardinality matched), `target/classic-magic-item-parity.json` (`4/4`) | PASS |
| `P5` | World/sites/economy/social parity complete | `target/classic-site-service-parity-matrix.json` (`4/4` pass), replay fixtures `p5_*` | PASS |
| `P6` | Progression/factions/quests/endings parity complete | `target/classic-progression-branch-matrix.json` (`5/5` pass), replay fixtures `p6_*` | PASS |
| `P7` | Save/options/wizard compatibility complete | `target/save-compat-report.json` (`3/3`), `target/classic-compatibility-matrix.json` (`5/5`) | PASS |
| `P8` | TUI + Bevy full parity workflows complete | `target/frontend-command-parity.json` (`67/67`), `target/classic-frontend-workflow-parity.json` (`3/3`) | PASS |
| `P9` | Full parity harness/burn-in/closure complete | `target/classic-parity-regression-dashboard.json`, `target/classic-burnin-window.json`, `target/classic-state-integrity.json`, `target/classic-parity-baseline-freeze.json` | PASS |

Overall: **10/10 PASS**

## Baseline Metrics (Current Artifacts)

- Legacy commands captured: `75`
- Command parity:
  - same key: `57`
  - different key: `18`
  - partial: `0`
  - missing: `0`
  - key conflicts: `0`
- Legacy cardinality baseline highlights:
  - spells: `42`
  - monsters: `151`
  - traps: `13`
  - city sites: `30`
  - map fixtures: legacy `20`, rust `20`
- Replay/verification:
  - replay scenarios: `651`, failed: `0`
  - determinism runs: `13020`, divergent: `0`
  - frontend key parity cases: `67`, mismatched: `0`
  - save compatibility cases: `3`, failed: `0`
  - site/service matrix checks: `4`, failed: `0`
  - progression matrix checks: `5`, failed: `0`
  - state integrity scenarios: `3`, failed: `0`
  - core model parity checks: `5`, failed: `0`
  - combat/encounter parity checks: `4`, failed: `0`
  - magic/item parity checks: `4`, failed: `0`
  - compatibility matrix checks: `5`, failed: `0`
  - frontend workflow scenarios: `3`, failed: `0`
- Open gap ledger items: `0`
