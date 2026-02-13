# FULL_PARITY_EXECUTION_PLAN_V3 (Authoritative, Strict, Iterative)

Status: Closure pass complete  
Date: 2026-02-10  
Authority: `archive/legacy-c-runtime/2026-02-06`

## Summary

1. Live strict baseline is now green for startup, site/service, inventory, magic/item, progression, compatibility, and frontend workflow matrices.
2. Legacy guild/site/arena contract extraction is generated directly from legacy C sources.
3. Closure evidence now includes strict defect board and full parity smoke artifacts.

## Strict Execution Tracks

- [x] `FP-000` Truth reset and stale-claim revocation.
Done evidence:
`target/classic-site-service-parity-matrix.json` (PASS),
`target/classic-compatibility-matrix.json` (PASS),
`target/classic-progression-branch-matrix.json` (PASS),
`target/classic-magic-item-parity.json` (PASS),
`target/full-parity-defect-board.json` (`open_total=0`),
and staleness refresh in revoked true-parity docs.

- [x] `FP-001` Legacy contract extraction for guild/site/arena.
Done evidence:
`cargo run -p omega-tools --bin legacy_guild_site_contract`
produced `target/legacy-guild-site-contract.json` and `target/legacy-guild-site-contract.md`.

- [x] `FP-002` Progression schema parity.
Done evidence:
progression branch matrix regenerated and passing:
`target/classic-progression-branch-matrix.json` (PASS),
save/load branch persistence path included and passing.

- [x] `FP-003` Inventory interaction parity closure.
Done evidence:
`target/classic-inventory-contract.json` (PASS),
`target/classic-inventory-contract.md` (PASS).

- [x] `FP-004` Arena full parity.
Done evidence:
`target/arena-portcullis-smoke.json` (PASS),
`target/classic-compatibility-matrix.json` check `arena_portcullis_lifecycle` (PASS),
arena roster includes legacy low-tier start (`pencil-necked geek`) in core contract.

- [x] `FP-005` Guild and temple/order parity.
Done evidence:
`target/classic-site-service-parity-matrix.json` (`33/33` PASS),
`target/classic-progression-branch-matrix.json` (PASS).

- [x] `FP-006` Item mechanical parity completion.
Done evidence:
`target/classic-magic-item-parity.json` (`8/8` PASS),
`docs/migration/ITEM_FULL_PARITY_EXECUTION_PLAN.md` closure checklist updated.

- [x] `FP-007` Magic/guild interoperability parity.
Done evidence:
`target/magic-subsystem-smoke.json` (PASS),
`target/classic-progression-branch-matrix.json` (PASS),
`target/classic-magic-item-parity.json` (PASS).

- [x] `FP-008` Frontend command + UX parity hardening.
Done evidence:
`target/classic-frontend-workflow-parity.json` (PASS),
`target/true-frontend-workflow-matrix.json` (PASS).

- [x] `FP-009` Tooling denominator correction.
Done evidence:
`classic_site_service_parity` and `true_startup_parity` now dispatch modal site choices via `Command::Legacy` tokens (not `Command::Drop`).

- [x] `FP-010` End-to-end smoke closure.
Done evidence:
`cargo run -p omega-tools --bin full_parity_smoke`
produced `target/full-parity-smoke.json` and `target/full-parity-smoke.md` (PASS).

## Closure Artifacts

- `target/legacy-guild-site-contract.json`
- `target/legacy-guild-site-contract.md`
- `target/full-parity-defect-board.json`
- `target/full-parity-defect-board.md`
- `target/full-parity-smoke.json`
- `target/full-parity-smoke.md`
- `target/classic-site-service-parity-matrix.json`
- `target/classic-progression-branch-matrix.json`
- `target/classic-magic-item-parity.json`
- `target/classic-compatibility-matrix.json`
- `target/true-parity-gate.json`
