# Classic Omega Full Parity Execution Plan

Superseded for closure decisions by `docs/migration/FULL_OMEGA_PARITY_RECOVERY_PLAN.md`.  
This document is retained as historical execution record only.

Status: Active (planning baseline approved)  
Date: 2026-02-07  
Scope: achieve feature-complete gameplay/content parity with legacy Omega `0.90` behavior, using the Rust stack.

## Purpose

Move from "minimum playable" to "classic Omega feature-complete parity" against the archived legacy runtime source:
- `archive/legacy-c-runtime/2026-02-06`

Foundation already complete:
- `docs/migration/PLAYABLE_OMEGA_STRICT_PLAN.md` (closed)

## Legacy Feature Baseline (Parity Contract Seed)

Parity is measured against legacy documented and code-defined surfaces.

Primary references:
- `lib/help1.txt` .. `lib/help13.txt`
- `archive/legacy-c-runtime/2026-02-06/defs.h`
- legacy modules in `archive/legacy-c-runtime/2026-02-06/*.c`

Known legacy cardinalities from `defs.h`:
- spells: `NUMSPELLS=42`
- monsters: `NUMMONSTERS=151`
- traps: `NUMTRAPS=13` (`NUM_SAFE_TRAPS=7`)
- city sites: `NUMCITYSITES=30`
- items:
  - scrolls: `NUMSCROLLS=24`
  - potions: `NUMPOTIONS=18`
  - foods: `NUMFOODS=16`
  - weapons: `NUMWEAPONS=41`
  - armor: `NUMARMOR=17`
  - shields: `NUMSHIELDS=8`
  - cloaks: `NUMCLOAKS=7`
  - boots: `NUMBOOTS=7`
  - rings: `NUMRINGS=9`
  - sticks: `NUMSTICKS=17`
  - artifacts: `NUMARTIFACTS=26`
- map fixtures currently present under `tools/libsrc`: `20` `.map` files

Command surfaces to preserve:
- dungeon/city command list (`lib/help12.txt`)
- countryside command list (`lib/help13.txt`)

## Non-Negotiable Full-Parity Exit Criteria

1. Full command parity across dungeon/city/countryside help-listed commands.
2. Full content catalog parity for monsters/spells/traps/item families and map sites.
3. Full progression parity for guilds/priesthood/alignment/quest and win-state branches (including Total Winner path).
4. Full runtime parity for core systems: combat maneuvers, magic effects, statuses, NPC interaction, economy, travel.
5. Save/options/wizard/high-score parity policy implemented and validated.
6. TUI and Bevy both expose parity-complete gameplay workflows (not just core-slice workflows).
7. Parity harness has approved denominator and burn-in confidence window.

## Track Map (Strict)

Only items mapped to these tracks are in-scope until full parity closure.

## Track P0: Parity Spec and Gap Ledger

- [x] `P0-001` Build machine-readable parity manifest from legacy docs/code.
- [x] `P0-002` Produce command-level parity matrix (`legacy -> rust`) with status per command.
- [x] `P0-003` Produce content cardinality matrix with source-of-truth counts and rust counts.
- [x] `P0-004` Publish approved gap ledger (missing/partial/deviated behavior with owner).

Exit:
- signed parity manifest and gap ledger committed.

## Track P1: Core Data Model and Rules Expansion

- [x] `P1-001` Expand player model to legacy-equivalent stats, derived combat fields, resistances/immunities, status flags.
- [x] `P1-002` Expand world model for environments (city/country/dungeon/special sites), level persistence rules, transitions.
- [x] `P1-003` Implement legacy-like turn scheduler (player, monsters, environment, timed effects).
- [x] `P1-004` Implement legacy-complete status/effect stack semantics and cleanup rules.

Exit:
- core model can represent all legacy parity manifest fields without ad-hoc shims.

## Track P2: Commands and Interaction Semantics

- [x] `P2-001` Implement dungeon/city command semantics from `help12`.
- [x] `P2-002` Implement countryside command semantics from `help13`.
- [x] `P2-003` Implement command timing costs and action points (including combat maneuver sequencing).
- [x] `P2-004` Implement command-side safety/confirmation behaviors and option gating.

### P2 Closure Checklist (Mandatory Check-Off)

- [x] `P2-BL-001` Command manifest baseline established (`missing=0`, `partial>0`, `key_conflict>0`).
- [x] `P2-A-001` Resolve key conflicts to zero (`q`, `P`, ctrl-command routing policy) with explicit frontend mapping parity.
- [x] `P2-A-002` Replace placeholder handlers for dungeon/city commands with behavior-complete implementations.
- [x] `P2-A-003` Replace placeholder handlers for countryside commands with behavior-complete implementations.
- [x] `P2-A-004` Implement per-command legacy time-cost model and action-point scheduling semantics.
- [x] `P2-A-005` Implement confirmations/safety gates/options-driven command variants.
- [x] `P2-A-006` Regenerate `classic-command-parity-matrix` and require `partial=0`, `missing=0`, `key_conflict=0`.
- [x] `P2-A-007` Add replay fixtures for every formerly-partial command and pass all of them in CI.

Exit:
- command matrix reaches 100% implemented (or approved explicit incompatibility list = empty at closure).

## Track P3: Combat, AI, and Encounter Parity

- [x] `P3-001` Implement full combat maneuver system (`attack/block/riposte/lunge`, lines high/center/low).
- [x] `P3-002` Implement legacy monster movement/melee/special attack/talk behavior families.
- [x] `P3-003` Implement trap/hazard interaction parity and damage/status application.
- [x] `P3-004` Implement faction/alignment-sensitive hostility and law/chaos consequences.

Exit:
- combat and encounter replay suite passes against parity fixtures for all core families.

## Track P4: Magic, Items, and Effects Parity

- [x] `P4-001` Implement full spell catalog (`42`) with power/mana/stacking semantics.
- [x] `P4-002` Implement item family behavior parity (scroll/potion/weapon/armor/shield/cloak/boots/ring/stick/artifact).
- [x] `P4-003` Implement artifact activation and unique side effects parity.
- [x] `P4-004` Implement identification, naming, inventory-slot behavior, burden/capacity semantics.

Exit:
- item/spell/effect parity matrix complete with 100% covered interactions.

## Track P5: World, Sites, Economy, and Social Systems

- [x] `P5-001` Implement city services/guild halls/training/economy behaviors (bank, houses, shops, legal consequences).
- [x] `P5-002` Implement countryside exploration, discovery/search/hunting/terrain time costs.
- [x] `P5-003` Implement villages/special sites and environment-specific event logic.
- [x] `P5-004` Implement dialogue/NPC interactions and quest hooks.

### P5 Closure Checklist (Mandatory Check-Off)

- [x] `P5-BL-001` World-mode/state scaffolding exists (dungeon/city/countryside mode + basic travel hooks).
- [x] `P5-A-001` Implement city economy loop: shops, bank, services, training costs, and legal penalties.
- [x] `P5-A-002` Implement countryside exploration/search/hunting with terrain-scaled travel time.
- [x] `P5-A-003` Implement village/special-site entry logic and site-specific event handlers.
- [x] `P5-A-004` Implement NPC dialogue trees and quest-entry hooks tied to world state.
- [x] `P5-A-005` Create `target/classic-site-service-parity-matrix.json` and require full PASS.
- [x] `P5-A-006` Add replay/integration scenarios covering economy, travel, and social interactions.
- [x] `P5-A-007` Close `G-003`/`G-004` world-system gap items in `classic-gap-ledger`.

Exit:
- site/service parity matrix complete for all legacy city/country/location systems.

## Track P6: Progression, Factions, Quests, and Endings

- [x] `P6-001` Implement guild rank progression and rank-dependent capabilities.
- [x] `P6-002` Implement priesthood/deity progression and alignment interactions.
- [x] `P6-003` Implement quest/state machine parity (including key narrative pivots).
- [x] `P6-004` Implement full ending/high-score eligibility paths, including Total Winner.

### P6 Closure Checklist (Mandatory Check-Off)

- [x] `P6-BL-001` Progression model scaffolding exists (guild/priest/alignment/quest/total-winner fields).
- [x] `P6-A-001` Implement guild rank gates, unlock rules, and capability deltas per rank.
- [x] `P6-A-002` Implement deity/priesthood progression and law-chaos alignment consequences.
- [x] `P6-A-003` Implement quest graph state machine with narrative pivot persistence.
- [x] `P6-A-004` Implement ending branches and score eligibility (normal vs wizard) including Total Winner.
- [x] `P6-A-005` Add progression/quest replay suites and persistence loops (save/load mid-branch).
- [x] `P6-A-006` Create `target/classic-progression-branch-matrix.json` and require full PASS.
- [x] `P6-A-007` Close progression-related open gap items and clear `classic-gap-ledger`.

Exit:
- all legacy win/score branches represented and testable in rust runtime.

## P2/P5/P6 Integrated Closure Sequence

1. `SEQ-1`: Finish `P2-A-001..P2-A-007` until command parity has zero conflicts and zero partials.
2. `SEQ-2`: Finish `P5-A-001..P5-A-007` and land world/site/economy/social parity matrix.
3. `SEQ-3`: Finish `P6-A-001..P6-A-007` and land progression/ending parity matrix.
4. `SEQ-4`: Re-run `P9` burn-in and close only when blockers list is empty.

## Checklist Discipline (Required)

1. Every completed work item must be immediately flipped from `[ ]` to `[x]` in this file in the same change.
2. No track may be declared complete while any item under its closure checklist remains unchecked.
3. Scorecard and closure review must be updated in the same commit that checks off a track item.

## Track P7: Save/Restore/Options/Wizard Compatibility

- [x] `P7-001` Extend save schema to parity-required state while preserving migration guarantees.
- [x] `P7-002` Implement legacy options behavior parity (`help11` options and runtime effects).
- [x] `P7-003` Implement wizard-mode compatibility policy (supported subset + explicit exclusions if any).
- [x] `P7-004` Implement score/log policy parity for normal vs wizard sessions.

Exit:
- compatibility test matrix passes across all supported save/options permutations.

## Track P8: Frontend Parity Completion (TUI + Bevy)

- [x] `P8-001` TUI full command/UI workflow parity (inventory modes, message history, status flags, map semantics).
- [x] `P8-002` Bevy workflow parity for the same command and state surfaces.
- [x] `P8-003` Shared input contract parity for all relevant commands/modes.
- [x] `P8-004` UX diagnostics parity (`x`, `/`, `?`, contextual feedback flows).

Exit:
- both frontends can drive full-parity sessions, not just core slices.

## Track P9: Verification, Burn-In, and Closure

- [x] `P9-001` Expand replay/parity harness to full feature denominator.
- [x] `P9-002` Add long-run state integrity tests (save/load loops, multi-dungeon progression, quest branch persistence).
- [x] `P9-003` Define and pass parity burn-in window.
- [x] `P9-004` Publish full parity closure review and freeze baseline.

Exit:
- full parity scorecard shows all PASS and closure package approved.

## Required Artifacts for Full-Parity Closure

- `target/classic-parity-manifest.json`
- `target/classic-command-parity-matrix.json`
- `target/classic-content-cardinality-matrix.json`
- `target/classic-gap-ledger.json`
- `target/classic-parity-regression-dashboard.json`
- `target/classic-parity-regression-dashboard.md`
- `target/classic-burnin-window.json`
- `target/classic-burnin-window.md`
- `target/classic-site-service-parity-matrix.json`
- `target/classic-site-service-parity-matrix.md`
- `target/classic-progression-branch-matrix.json`
- `target/classic-progression-branch-matrix.md`
- `target/classic-state-integrity.json`
- `target/classic-state-integrity.md`
- `target/classic-core-model-parity.json`
- `target/classic-core-model-parity.md`
- `target/classic-combat-encounter-parity.json`
- `target/classic-combat-encounter-parity.md`
- `target/classic-magic-item-parity.json`
- `target/classic-magic-item-parity.md`
- `target/classic-compatibility-matrix.json`
- `target/classic-compatibility-matrix.md`
- `target/classic-frontend-workflow-parity.json`
- `target/classic-frontend-workflow-parity.md`
- `target/classic-parity-baseline-freeze.json`
- `target/classic-parity-baseline-freeze.md`
- `docs/migration/CLASSIC_OMEGA_PARITY_SCORECARD.md`
- `docs/migration/CLASSIC_OMEGA_PARITY_CLOSURE_REVIEW.md`

## Governance Rules (Anti-Scope Drift)

1. No new roadmap branches outside tracks `P0..P9`.
2. Any ticket must map to one and only one track item.
3. Behavior changes that intentionally differ from legacy must be logged as explicit deviations and must be zero at closure.
4. Performance, refactors, and visual polish are secondary until parity scorecard is fully PASS.

## Current Progress Snapshot

- Completed foundation: playable-minimum stack and strict artifact gate from prior plan.
- Full-classic parity tracks (`P0..P9`): all tracks complete.
- Overall full-classic parity completion: `10/10 tracks complete`.
- Active closure focus: none (closure package complete).
