# Placeholder Eradication Checklist

Date: 2026-02-08  
Status: Active (authoritative until all items are checked)

This checklist is the source of truth for removing placeholder behavior and false-positive parity PASS states.

## Rules

- [x] `Rule-001` No parity status may be called "complete" while any item in this file is unchecked.
- [x] `Rule-002` Old green artifacts are non-authoritative if they are built from proxy inputs instead of runtime parity assertions.
- [x] `Rule-003` Every checked item must include a linked code/test change and a reproducible command result.

## P0 Runtime Placeholder Removal

- [x] `P0-001` Remove city placeholder assignment plan and bind all city service tiles from real map semantics.
Files: `crates/omega-content/src/lib.rs:377`, `crates/omega-content/src/lib.rs:415`, `crates/omega-content/src/lib.rs:1423`  
Done when: `city_placeholder_assignment` and placeholder-index routing are removed; tests assert real site mapping instead of "dense placeholder business coverage".

- [x] `P0-002` Eliminate generic legacy command fallbacks that silently succeed without behavior.
Files: `crates/omega-core/src/lib.rs:1317`, `crates/omega-core/src/lib.rs:1731`  
Done when: unknown legacy commands hard-fail with explicit unsupported status; no `"legacy command resolved with no additional world effect"` path remains.

- [x] `P0-003` Replace command acknowledgements that defer core behavior to frontend stubs.
Files: `crates/omega-core/src/lib.rs:1695`, `crates/omega-core/src/lib.rs:1697`  
Done when: command effects are modeled in core state transitions (or rejected explicitly) rather than "frontend handles" notes.

- [x] `P0-004` Replace simplified city-site service menus/effects with parity behavior per site type.
Files: `crates/omega-core/src/lib.rs:2341`, `crates/omega-core/src/lib.rs:2482`, `crates/omega-core/src/lib.rs:2964`  
Done when: shops, guilds, bank, temple, order, castle, arena, and altars execute legacy-equivalent rules and branching outcomes.
Command: `cargo test -p omega-core` (36 passed, 0 failed on 2026-02-08)

- [x] `P0-005` Remove bootstrap fallback-to-default runtime path in playable frontends.
Files: `crates/omega-tui/src/bin/omega-tui-app.rs:158`, `crates/omega-bevy/src/bin/omega-bevy-app.rs:152`, `crates/omega-tui/src/lib.rs:248`, `crates/omega-bevy/src/lib.rs:344`  
Done when: bootstrap failure is surfaced as hard error; no playable session starts from `GameState::default()` fallback.

- [x] `P0-006` Remove placeholder-named runtime artifacts from production gameplay loops.
Files: `crates/omega-core/src/lib.rs:1424`, `crates/omega-core/src/lib.rs:1444`, `crates/omega-core/src/lib.rs:2075`, `crates/omega-core/src/lib.rs:2508`  
Done when: temporary names like `foraged-food-*`, `hidden-cache-*`, `shop-ration`, `shop-potion`, `named-*`, `artifact-wish` are replaced by catalog-backed items and typed effects.
Command: `cargo run -p omega-tools --bin classic_magic_item_parity` (total=4, passed=4, failed=0 on 2026-02-08)

- [x] `P0-007` Lock encounter spawning so city/village safety and parity populations cannot regress.
Files: `crates/omega-core/src/lib.rs:3226`, `crates/omega-core/src/lib.rs:3292`  
Done when: no countryside random encounter can appear in city contexts; encounter tables are terrain- and context-appropriate and parity-validated.
Command: `cargo test -p omega-core` (includes countryside encounter guards; 36 passed, 0 failed on 2026-02-08)

## P1 Verification Hardening (No More False PASS)

- [x] `P1-001` Stop building "true" parity artifacts from classic/proxy report files.
Files: `crates/omega-tools/src/bin/true_parity_refresh.rs:177`, `crates/omega-tools/src/bin/true_parity_refresh.rs:183`, `crates/omega-tools/src/bin/true_parity_refresh.rs:287`, `crates/omega-tools/src/bin/true_parity_refresh.rs:322`  
Done when: true parity reports are generated from direct runtime parity fixtures and end-to-end traces, not reused classic outputs.
Command: `cargo run -p omega-tools --bin true_parity_refresh` (status PASS; dashboard `target/true-parity-regression-dashboard.json` on 2026-02-08)

- [x] `P1-002` Expand startup parity checks from spawn/context only to full interactive startup readiness.
Files: `crates/omega-tools/src/bin/true_startup_parity.rs:57`  
Done when: startup matrix verifies actionable doors, at least one functioning service, and valid exit to country map in both frontends.
Command: `cargo run -p omega-tools --bin true_startup_parity` (total=14, passed=14, failed=0 on 2026-02-08)

- [x] `P1-003` Add dedicated full-city interaction parity matrix.
Files: `crates/omega-tools/src/bin/classic_site_service_parity.rs`  
Done when: matrix includes per-site assertions for Rampart and all villages (store/guild/temple/palace/order/arena) with state delta checks.
Command: `cargo run -p omega-tools --bin classic_site_service_parity` (total=33, passed=33, failed=0 on 2026-02-08)

- [x] `P1-004` Add questionnaire character-creation parity checks in shipped launch flows.
Files: `crates/omega-tui/src/bin/omega-tui-app.rs`, `crates/omega-bevy/src/bin/omega-bevy-app.rs`, `crates/omega-core/src/lib.rs`  
Done when: questionnaire path follows legacy `char.c` question flow/scoring and is test-covered in all primary user launch paths.
Command: `cargo test -p omega-core` + `cargo test -p omega-tui --bin omega-tui-app` + `cargo test -p omega-bevy --bin omega-bevy-app` (all tests passed on 2026-02-08)

- [x] `P1-005` Add inventory/log UX parity checks.
Files: `crates/omega-tui/src/lib.rs:453`, `crates/omega-tui/src/lib.rs:486`, `crates/omega-bevy/src/lib.rs:464`  
Done when: inventory actions and log/event ordering are deterministic, chronological, and human-readable across frontends.
Command: `cargo test -p omega-tui --lib` and `cargo test -p omega-bevy --lib` (all tests passed on 2026-02-08)

## Full-Game Closure Gates (User-Visible Parity)

- [x] `G-001` Character creation offers questionnaire mode and applies resulting archetype/alignment/stats.
Files: `crates/omega-tui/src/bin/omega-tui-app.rs`, `crates/omega-bevy/src/bin/omega-bevy-app.rs`  
Command: `cargo test -p omega-core` + `cargo test -p omega-tui --bin omega-tui-app` + `cargo test -p omega-bevy --bin omega-bevy-app` (legacy questionnaire parity tests pass on 2026-02-08)
- [x] `G-002` New game starts in Rampart at parity spawn on full Rampart map.
Files: `crates/omega-tools/src/bin/true_startup_parity.rs`  
Command: `cargo run -p omega-tools --bin true_startup_parity` (`spawn=(62,20)`, `city_map=3`, status PASS on 2026-02-08)
- [x] `G-003` Stepping into doors/services triggers interactions; bump/open/close door semantics work.
Files: `crates/omega-core/src/lib.rs`, `crates/omega-tools/src/bin/true_startup_parity.rs`  
Command: `cargo run -p omega-tools --bin true_startup_parity` (door bump-open and service actionable checks PASS for content/tui/bevy on 2026-02-08)
- [x] `G-004` Exiting Rampart transitions to full country map; entering city/village/temple/special sites loads distinct maps.
Files: `crates/omega-tools/src/bin/true_startup_parity.rs`, `crates/omega-tools/src/bin/classic_site_service_parity.rs`  
Command: `cargo run -p omega-tools --bin true_startup_parity` + `cargo run -p omega-tools --bin classic_site_service_parity` (exit-to-country + all six villages distinct map IDs PASS on 2026-02-08)
- [x] `G-005` Stores, guilds, factions, palace/castle, temple/altar, bank, arena are all functional and useful.
Files: `crates/omega-core/src/lib.rs`, `crates/omega-tools/src/bin/classic_site_service_parity.rs`  
Command: `cargo run -p omega-tools --bin classic_site_service_parity` (33/33 PASS with per-site state deltas on 2026-02-08)
- [x] `G-006` Quest/progression/ending branches work through to completion paths.
Files: `crates/omega-tools/src/bin/classic_progression_branch_matrix.rs`  
Command: `cargo run -p omega-tools --bin classic_progression_branch_matrix` (total=5, passed=5, failed=0 on 2026-02-08)
- [x] `G-007` Magic system and spell catalog behavior are operational (including cast flow and effects).
Files: `crates/omega-core/src/lib.rs`, `crates/omega-tools/src/bin/classic_magic_item_parity.rs`  
Command: `cargo run -p omega-tools --bin classic_magic_item_parity` (total=4, passed=4, failed=0 on 2026-02-08)
- [x] `G-008` Inventory is fully functional (pickup/drop/use/equip/constraints/persistence).
Files: `crates/omega-core/src/lib.rs`, `crates/omega-tui/src/lib.rs`, `crates/omega-tools/src/bin/m5_e2e_journey.rs`  
Command: `cargo test -p omega-core` + `cargo run -p omega-tools --bin m5_e2e_journey` (pickup/drop/capacity plus save/load/restart loops PASS on 2026-02-08)
- [x] `G-009` Monsters/items/traps/combat are parity-aligned and context-appropriate.
Files: `crates/omega-core/src/lib.rs`, `crates/omega-tools/src/bin/classic_combat_encounter_parity.rs`  
Command: `cargo run -p omega-tools --bin classic_combat_encounter_parity` (total=4, passed=4, failed=0 on 2026-02-08)
- [x] `G-010` No placeholder text, proxy behavior, or no-op command path remains in gameplay-critical flows.
Files: `crates/omega-tools/src/bin/strict_placeholder_audit.rs`, `crates/omega-tools/src/bin/true_parity_refresh.rs`  
Command: `cargo run -p omega-tools --bin strict_placeholder_audit` + `cargo run -p omega-tools --bin true_parity_gate` (PASS on 2026-02-08)

## Mandatory Verification Commands

- [x] `V-001` `cargo test --workspace`
- [x] `V-002` `powershell -ExecutionPolicy Bypass -File scripts/run-recovery-gate.ps1`
- [x] `V-003` Add and run a strict placeholder audit tool that fails on:
`city_placeholder_assignment` usage, default bootstrap fallback activation in playable modes, generic no-op legacy command resolutions, and proxy-fed true parity generation.
Command: `cargo run -p omega-tools --bin strict_placeholder_audit` (status PASS, passed=4/4 on 2026-02-08)

## Completion Rule

- [x] `Done` All checklist items above are checked, verification commands pass, and a manual playthrough confirms:
questionnaire -> Rampart -> services -> overworld -> return/side sites -> quest/magic/inventory/combat loops all function without placeholders.
Command: `cargo test --workspace` + `powershell -ExecutionPolicy Bypass -File scripts/run-recovery-gate.ps1` + `cargo run -p omega-tools --bin m5_e2e_journey` + `cargo run -p omega-tools --bin true_parity_gate` (all PASS on 2026-02-08)
