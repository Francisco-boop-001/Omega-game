# Full Omega True Parity Plan (Strict)

Status: Revoked (non-authoritative)  
Date: 2026-02-07  
Scope: Full classic Omega gameplay and content parity on the new stack.

Revoked by: `docs/migration/FULL_OMEGA_PARITY_RECOVERY_PLAN.md` (2026-02-07)
Last reviewed for staleness gate: 2026-02-10

## Mission

Deliver a full playable Omega implementation with legacy-equivalent startup, world systems, content, mechanics, progression, and frontends.

Success means the player can start in Rampart city and complete real full-game runs, not a reduced simulation.

## Hard Rules

1. No synthetic placeholders for core catalogs in parity mode (spells, monsters, items, city systems).
2. No declaration-only parity claims; behavior must be validated by fixtures/scenarios.
3. No closure while any deviation remains open.
4. Every completed item must be checked `[x]` in this file immediately.
5. Scorecard and closure review must be updated in the same change as checklist updates.

## Legacy Authority

- `archive/legacy-c-runtime/2026-02-06/omega.c`
- `archive/legacy-c-runtime/2026-02-06/defs.h`
- `archive/legacy-c-runtime/2026-02-06/command1.c`
- `archive/legacy-c-runtime/2026-02-06/command2.c`
- `archive/legacy-c-runtime/2026-02-06/command3.c`
- `archive/legacy-c-runtime/2026-02-06/city.c`
- `archive/legacy-c-runtime/2026-02-06/country.c`
- `archive/legacy-c-runtime/2026-02-06/site1.c`
- `archive/legacy-c-runtime/2026-02-06/site2.c`
- `archive/legacy-c-runtime/2026-02-06/spell.c`
- `archive/legacy-c-runtime/2026-02-06/item.c`
- `archive/legacy-c-runtime/2026-02-06/itemf1.c`
- `archive/legacy-c-runtime/2026-02-06/itemf2.c`
- `archive/legacy-c-runtime/2026-02-06/itemf3.c`
- `archive/legacy-c-runtime/2026-02-06/mon.c`
- `archive/legacy-c-runtime/2026-02-06/mspec.c`
- `archive/legacy-c-runtime/2026-02-06/mtalk.c`
- `archive/legacy-c-runtime/2026-02-06/trap.c`
- `archive/legacy-c-runtime/2026-02-06/time.c`
- `archive/legacy-c-runtime/2026-02-06/save.c`
- `lib/help11.txt`
- `lib/help12.txt`
- `lib/help13.txt`
- `tools/libsrc/*.map`

## Global Done Definition

Closure is allowed only when all conditions are true:

1. Every track `T0..T12` is fully checked `[x]`.
2. `cargo test --workspace` passes.
3. Full artifact package exists and all required statuses are PASS.
4. Startup parity proves canonical Rampart city start.
5. Scripted full-play campaigns reach valid major endings.
6. TUI and Bevy both support full-session parity workflows.
7. Deviation ledger is empty.

## Dependency Graph

1. `T0` -> all tracks
2. `T1` -> `T2`, `T3`, `T7`, `T10`, `T11`
3. `T2` -> `T3`, `T6`, `T7`, `T8`, `T9`
4. `T3` -> `T4`, `T5`, `T6`, `T7`, `T10`
5. `T4`,`T5`,`T6` -> `T8`, `T10`, `T11`
6. `T7`,`T8`,`T9` -> `T10`, `T11`
7. `T10`,`T11` -> `T12`

## Execution Waves

### Wave A (Truth Reset + Startup)
- `T0`, `T1`

### Wave B (Runtime Foundations)
- `T2`, `T3`

### Wave C (Core Gameplay Parity)
- `T4`, `T5`, `T6`

### Wave D (World + Progression + Compatibility)
- `T7`, `T8`, `T9`

### Wave E (Frontends + Verification + Closure)
- `T10`, `T11`, `T12`

## Track Checklist

## Track T0: Governance Reset and Truth Baseline

- [x] `T0-001` Mark previous parity closure docs as superseded/non-authoritative.
- [x] `T0-002` Publish this plan as the closure contract and link from migration index.
- [x] `T0-003` Create `target/true-parity-deviations.json` schema and initialize with known gaps.
- [x] `T0-004` Add gating rule in tooling: no track completion without artifact evidence.

Exit:
- governance and truth baseline enforceable.

## Track T1: Startup and World Context Parity

- [x] `T1-001` Implement startup flow equivalent to legacy city-first `init_world`.
- [x] `T1-002` Remove first-sorted-map bootstrap dependency for new game startup.
- [x] `T1-003` Implement correct initial city location/context/welcome flow.
- [x] `T1-004` Add startup parity fixture: environment, coordinates, city context invariants.
- [x] `T1-005` Add launcher smoke test proving startup parity on both frontends.

Exit:
- new session always starts in Rampart city semantics.

## Track T2: Environment and Map Model Parity

- [x] `T2-001` Replace simplified world mode model with parity-capable environment model.
- [x] `T2-002` Implement environment transition logic (city, countryside, dungeons, special sites).
- [x] `T2-003` Implement level persistence policy (country/city/current dungeon behavior).
- [x] `T2-004` Implement map binding to environment IDs rather than generic first-map usage.
- [x] `T2-005` Add environment transition matrix fixtures.

Exit:
- environment transition matrix PASS.

## Track T3: Command Behavioral Parity

- [x] `T3-001` Build command behavior matrix from `help12/help13/help11` with expected effects.
- [x] `T3-002` Implement context-sensitive command routing and restrictions.
- [x] `T3-003` Implement timing/action cost parity behavior.
- [x] `T3-004` Implement command safety/confirmation semantics.
- [x] `T3-005` Add fixture denominator proving behavior, not just key mapping.

Exit:
- command behavior matrix `missing=0`, `partial=0`, `deviation=0`.

## Track T4: Spell System Parity (42)

- [x] `T4-001` Replace synthetic spell catalog with legacy-derived spell definitions.
- [x] `T4-002` Implement distinct per-spell effects and resource behavior.
- [x] `T4-003` Implement environment/target/state conditional spell semantics.
- [x] `T4-004` Implement spell learning/known/forgetting semantics.
- [x] `T4-005` Add deterministic fixture suite for all 42 spells.

Exit:
- spell matrix PASS `42/42`.

## Track T5: Items and Inventory Parity

- [x] `T5-001` Replace synthetic item catalogs with legacy-derived definitions.
- [x] `T5-002` Implement item generation/rarity/uniqueness behavior.
- [x] `T5-003` Implement use/equip semantics for all major item families.
- [x] `T5-004` Implement identify/name/blessing/curse interactions.
- [x] `T5-005` Implement burden and capacity semantics with fixture evidence.
- [x] `T5-006` Add family-by-family interaction matrix fixtures.

Exit:
- item parity matrix PASS with zero deviations.

## Track T6: Monster, Combat, and Trap Parity

- [x] `T6-001` Replace synthetic monster catalog with legacy-derived monster definitions.
- [x] `T6-002` Implement movement/melee/special/talk behavior families.
- [x] `T6-003` Implement tactical combat sequencing and line/maneuver semantics.
- [x] `T6-004` Implement faction/alignment/hostility consequences.
- [x] `T6-005` Implement trap and hazard semantics.
- [x] `T6-006` Add encounter matrix fixtures for all key families.

Exit:
- combat/encounter matrix PASS.

## Track T7: City, Site, Economy, and Social Parity

- [x] `T7-001` Implement city-site system behavior by site role, not random service roll.
- [x] `T7-002` Implement economy loops (banking/services/training/legal outcomes).
- [x] `T7-003` Implement NPC dialogue/talk flows for social and quest hooks.
- [x] `T7-004` Implement countryside and special-site interaction semantics.
- [x] `T7-005` Add site-by-site behavior fixtures.

Exit:
- site/economy/social matrix PASS with full denominator.

## Track T8: Progression, Quest, and Ending Parity

- [x] `T8-001` Implement rank progression and capability gates.
- [x] `T8-002` Implement quest state machine and branch persistence.
- [x] `T8-003` Implement ending path logic and eligibility rules.
- [x] `T8-004` Implement score policy parity.
- [x] `T8-005` Add progression and ending branch matrix fixtures.

Exit:
- progression/ending matrix PASS.

## Track T9: Save, Options, and Wizard Parity

- [x] `T9-001` Extend save schema for parity-required world and progression state.
- [x] `T9-002` Validate migration compatibility for supported versions.
- [x] `T9-003` Implement options behavior parity (`help11`) with runtime effects.
- [x] `T9-004` Implement wizard-mode compatibility and score policy.
- [x] `T9-005` Add save/load loops across environments and active branches.

Exit:
- compatibility matrix PASS.

## Track T10: Frontend Full-Session Parity

- [x] `T10-001` Ensure TUI can drive full parity sessions end-to-end.
- [x] `T10-002` Ensure Bevy can drive the same full sessions.
- [x] `T10-003` Enforce shared input contract across both frontends.
- [x] `T10-004` Implement diagnostics/help/identify parity workflows.
- [x] `T10-005` Add cross-frontend full-session equivalence fixtures.

Exit:
- frontend workflow matrix PASS.

## Track T11: Verification Harness Hardening

- [x] `T11-001` Replace declaration-based checks with behavior-grounded checks.
- [x] `T11-002` Add startup location oracle checks.
- [x] `T11-003` Add differential legacy-vs-rust comparable traces where feasible.
- [x] `T11-004` Add burn-in determinism gate (`N>=20`) for parity denominator.
- [x] `T11-005` Add stale-artifact detection and hard failure on missing artifacts.

Exit:
- verification dashboard PASS with zero blockers.

## Track T12: Full-Game Playable Closure

- [x] `T12-001` Execute canonical full-play scripts to valid ending branches.
- [x] `T12-002` Run manual smoke sessions on TUI and Bevy from fresh start.
- [x] `T12-003` Publish closure review confirming full-game playability.
- [x] `T12-004` Freeze baseline and finalize docs.

Exit:
- closure review approved.

## Required Artifacts (Closure Package)

- `target/true-parity-deviations.json`
- `target/true-startup-parity.json`
- `target/true-environment-transition-matrix.json`
- `target/true-command-behavior-matrix.json`
- `target/true-spell-parity-matrix.json`
- `target/true-item-parity-matrix.json`
- `target/true-combat-encounter-matrix.json`
- `target/true-site-economy-social-matrix.json`
- `target/true-progression-ending-matrix.json`
- `target/true-compatibility-matrix.json`
- `target/true-frontend-workflow-matrix.json`
- `target/true-parity-regression-dashboard.json`
- `target/true-burnin-window.json`
- `target/true-parity-baseline-freeze.json`
- `docs/migration/FULL_OMEGA_TRUE_PARITY_PLAN.md`
- `docs/migration/FULL_OMEGA_TRUE_PARITY_SCORECARD.md`
- `docs/migration/FULL_OMEGA_TRUE_PARITY_CLOSURE_REVIEW.md`

## Runbook (Mandatory per Closure Refresh)

1. `cargo test --workspace`
2. Run track artifacts in dependency order (T1->T11).
3. Regenerate regression dashboard and burn-in window.
4. Regenerate baseline freeze.
5. Update scorecard and closure review.

## Immediate Ordered Backlog (Execution Queue)

1. `Q-001` Mark old parity closure docs as superseded (`T0-001`).
2. `Q-002` Add `target/true-parity-deviations.json` generator and schema (`T0-003`).
3. `Q-003` Implement startup oracle tool `true_startup_parity` (`T1-004`).
4. `Q-004` Refactor bootstrap to explicit city-start selection, remove first-map startup (`T1-001`,`T1-002`).
5. `Q-005` Add startup invariants to TUI and Bevy launch paths (`T1-005`).
6. `Q-006` Introduce expanded environment enum and migration-safe serialization (`T2-001`).
7. `Q-007` Implement environment transition table and tests (`T2-002`,`T2-005`).
8. `Q-008` Implement level persistence model for city/country/current dungeon (`T2-003`).
9. `Q-009` Build command expectation matrix from help docs (`T3-001`).
10. `Q-010` Implement command context restrictions and safety semantics (`T3-002`,`T3-004`).
11. `Q-011` Replace declaration-only command report with behavior matrix (`T3-005`).
12. `Q-012` Import/define real 42-spell catalog and IDs (`T4-001`).
13. `Q-013` Implement spell semantics in grouped passes with fixtures (`T4-002..T4-005`).
14. `Q-014` Import/define real item catalogs and rarity/uniqueness flags (`T5-001`,`T5-002`).
15. `Q-015` Implement item use/equip/identify/burden parity fixtures (`T5-003..T5-006`).
16. `Q-016` Import/define monster catalog and behavior metadata (`T6-001`,`T6-002`).
17. `Q-017` Implement combat/trap parity suites (`T6-003..T6-006`).
18. `Q-018` Replace random city service roll with site-true logic (`T7-001`,`T7-002`).
19. `Q-019` Implement social/dialogue hooks and site fixtures (`T7-003..T7-005`).
20. `Q-020` Implement progression/quest/ending matrices and score rules (`T8-001..T8-005`).
21. `Q-021` Expand save/options/wizard compatibility suites (`T9-001..T9-005`).
22. `Q-022` Expand TUI full-session parity cases (`T10-001`).
23. `Q-023` Expand Bevy full-session parity cases (`T10-002`).
24. `Q-024` Enforce shared input parity and diagnostics workflows (`T10-003..T10-005`).
25. `Q-025` Final verification hardening, burn-in, closure freeze (`T11`,`T12`).

## PR Gate Policy (Strict)

1. Any PR that marks checklist items must include updated artifacts proving those items.
2. Any regression in startup/context/spell/item/monster/city behavior reopens impacted checklist items.
3. No PR may mark a track complete if one sub-item remains unchecked.
4. Any behavior change without fixture updates is blocked.
5. Deviation ledger must be updated for temporary gaps, and emptied before closure.

## Progress Snapshot

- [x] `PLAN-001` Master strict plan authored and activated.
- [x] `PLAN-002` Execution completed under this plan.
- [x] `PLAN-003` Closure artifacts generated and approved.
