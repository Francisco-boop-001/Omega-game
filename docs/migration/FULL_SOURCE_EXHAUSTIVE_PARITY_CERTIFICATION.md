# FULL_SOURCE_EXHAUSTIVE_PARITY_CERTIFICATION

## Scope
- Legacy authority: `archive/legacy-c-runtime/2026-02-06`
- Certification lane: differential + coverage + adversarial black-box, isolated under `target/certification/*`
- Existing parity suites remain regression support; certification is gated independently

## Collision Guard
- Removed artifact collision writer in `crates/omega-tools/src/bin/classic_site_service_parity.rs` that previously overwrote `target/rust-site-branch-contract.json`.
- `parity_certify` now enforces:
  - `target/rust-site-branch-contract.json` must expose branch contract shape (`branches`) and not parity matrix shape (`checks`).

## Certification Artifacts (Current)
1. `target/certification/baseline.json` (`pass=true`)
2. `target/certification/contracts/legacy-mechanics-ledger.json` (`total=1313`)
3. `target/certification/contracts/rust-mechanics-ledger.json` (`total=1356`)
4. `target/certification/contracts/mechanics_mapping.json` (`pass=true`, `unresolved=0`, `unknown=0`)
5. `target/certification/diff/legacy-headless-replay.json` (`total=17`)
6. `target/certification/diff/rust-headless-replay.json` (`total=17`)
7. `target/certification/diff/mechanics-differential.json` (`pass=true`, `17/17`)
8. `target/certification/diff/service-branch-differential.json` (`pass=true`, `11/11`)
9. `target/certification/coverage/branch-coverage.json` (`pass=true`, `missing=0`)
10. `target/certification/smoke/blackbox-adversarial.json` (`pass=true`, `25/25`)
11. `target/certification/defect-board.json` (`open=0`)
12. `target/certification/parity-certify.json` (`pass=true`, `7/7`)

## Mandatory Hard Gates
1. `cargo run -p omega-tools --bin parity_certify`
2. `cargo run -p omega-tools --bin true_parity_refresh`
3. `cargo test --workspace`

## Current Verdict
- `parity_certify`: PASS
- `true_parity_refresh`: PASS (includes mandatory `parity_certify` component)
- Certification defect board: `open=0`

Final certification status: **PASS** for the implemented zero-trust lane and its enforced artifacts.
