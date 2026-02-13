# Full Omega Parity Recovery Plan (Authoritative)

Status: Active (authoritative)
Date: 2026-02-07
Scope: Deliver full classic Omega gameplay/content parity on Rust stack with user-visible correctness.

Supersedes:
- `docs/migration/FULL_OMEGA_TRUE_PARITY_PLAN.md` (revoked)
- `docs/migration/FULL_OMEGA_TRUE_PARITY_SCORECARD.md` (revoked)
- `docs/migration/FULL_OMEGA_TRUE_PARITY_CLOSURE_REVIEW.md` (revoked)

Ground truth audit:
- `docs/migration/FULL_OMEGA_PARITY_REALITY_AUDIT_2026-02-07.md`

## Objective

When a new game starts:

1. Player starts in Rampart city on the real Rampart map.
2. Rampart city systems are fully functional (stores, guilds, palace, temple, bank, arena, etc.).
3. Exiting Rampart leads to real overworld traversal.
4. Quests, items, monsters, magic, progression, and endings behave at legacy parity.

## Non-Negotiable Rules

1. No placeholder catalogs in parity mode.
2. No synthetic map rendering in parity mode.
3. No closure while any P0 gap is open.
4. Checklist updates are mandatory and immediate after completion.
5. Scorecard and closure review must be updated in the same change as checklist updates.

## Legacy Authority

- `archive/legacy-c-runtime/2026-02-06/omega.c`
- `archive/legacy-c-runtime/2026-02-06/defs.h`
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

## Done Definition

Closure is allowed only when all are true:

1. Every track below is fully checked `[x]`.
2. New game starts in Rampart with visible full map and working city sites.
3. Player can exit Rampart and traverse real overworld sites.
4. Spells/items/monsters/traps/quests/progression/endings pass parity matrices.
5. TUI and Bevy both run full parity sessions with shared outcome expectations.
6. Differential checks against legacy traces pass for selected canonical scenarios.
7. `cargo test --workspace` passes.
8. Recovery scorecard is all PASS and closure review is APPROVED.

## Track Checklist

## Track R0: Governance Reset and Contract Replacement

- [x] `R0-001` Publish reality audit and mark previous true parity closure as revoked.
- [x] `R0-002` Publish this recovery plan as new authoritative contract.
- [x] `R0-003` Publish recovery scorecard and closure review templates.
- [x] `R0-004` Add hard tooling guard preventing PASS if map rendering is synthetic (`.` fill) in parity mode.
- [x] `R0-005` Add hard tooling guard preventing PASS if placeholder catalog entries (`spell-###`, `monster-###`, `legacy-monster-*`) are used in parity mode.

Exit:
- governance state matches reality and blocks false closure.
- guard implementation: `crates/omega-tools/src/bin/recovery_contract_guard.rs`
- guard entrypoint: `scripts/run-recovery-gate.ps1`
- current guard baseline (2026-02-07): `target/recovery-contract-guard.json` => `PASS` (`4/4` pass)

## Track R1: World/Map Runtime Model Parity

- [x] `R1-001` Introduce tile/site level model equivalent to legacy `Level->site[x][y]` semantics.
- [x] `R1-002` Introduce full environment enum aligned to legacy environment set (`E_CITY`, `E_COUNTRYSIDE`, etc.).
- [x] `R1-003` Implement country grid model (`Country[i][j]` terrain + aux/status).
- [x] `R1-004` Implement map binding by semantic identity (city/country/dungeon/site), not just bounds.
- [x] `R1-005` Add fixtures validating tile/site loading from `city.map` and `country.map`.

Exit:
- runtime can represent and persist real map geometry + site functions.

## Track R2: Rampart City Playable Parity

- [x] `R2-001` Render real Rampart tilemap in TUI and Bevy (walls/doors/features/maze/guards).
- [x] `R2-002` Set canonical start location and safe initial state equivalent to legacy startup behavior.
- [x] `R2-003` Replace bootstrap synthetic city spawns with map-driven/legacy-driven population.
- [x] `R2-004` Implement city movement blocking rules (`NOCITYMOVE`, portcullis, etc.) where applicable.
- [x] `R2-005` Add screenshot/text fixtures proving non-void Rampart at startup.

Exit:
- user visibly starts in functioning Rampart, not synthetic void.

## Track R3: City Site Function Parity

- [x] `R3-001` Implement deterministic site mapping from city tiles to `L_*` city functions.
- [x] `R3-002` Implement core sites: bank, temple, arena, merc guild, college, sorcerors, castle, thieves guild, DPW, armorer, tourism, charity.
- [x] `R3-003` Implement economy/legal/social side effects per site workflows.
- [x] `R3-004` Implement NPC interactions/talk hooks required by city progression.
- [x] `R3-005` Add site-by-site fixture matrix with effect assertions.

Exit:
- Rampart sites are real gameplay systems, not random service roll.

## Track R4: Overworld/Country Parity

- [x] `R4-001` Load and represent full `country.map` terrain + site IDs.
- [x] `R4-002` Implement countryside movement/time scaling and encounter semantics.
- [x] `R4-003` Implement site entry from overworld into city/village/temple/special dungeons.
- [x] `R4-004` Implement return transitions back to overworld with state persistence.
- [x] `R4-005` Add overworld traversal fixture suite (Rampart exit -> travel -> site entry -> return).

Exit:
- player can leave Rampart and play the real overworld loop.

### 2026-02-08 Strict Revalidation

- [x] `R3-004` Revalidated NPC/site interaction hooks with interactive menu flow and altar patron/sacrilege behavior tests in `crates/omega-core/src/lib.rs`.
- [x] `R3-005` Revalidated site fixture matrix through `classic_site_service_parity` (`12/12 PASS`) in `scripts/run-recovery-gate.ps1`.
- [x] `R4-001` Revalidated full map loading with arena map semantic binding (`map 1`) and tile-site semantics in `crates/omega-content/src/lib.rs`.
- [x] `R4-002` Revalidated world transition semantics and movement/travel behavior through `recovery_refresh` and `recovery-overworld-transition-matrix.json`.

## Track R5: Catalog Data Parity (Spells/Items/Monsters/Traps/Sites)

- [x] `R5-001` Replace numbered placeholder catalog generation with legacy-derived definitions.
- [x] `R5-002` Import/encode spell definitions (42), including costs/flags/knowledge behavior.
- [x] `R5-003` Import/encode item/object families and uniqueness/rarity semantics.
- [x] `R5-004` Import/encode monster definitions and behavior metadata.
- [x] `R5-005` Import/encode trap and city-site catalogs.

Exit:
- runtime uses real catalog data instead of synthetic labels.

## Track R6: Magic System Parity

- [x] `R6-001` Implement 42 spell dispatch with distinct semantics.
- [x] `R6-002` Implement spell learning/known/forgetting and knowledge gating.
- [x] `R6-003` Implement context-sensitive spell behavior by environment/target/state.
- [x] `R6-004` Implement power drain and edge-case behavior (e.g., wish/hellfire-type semantics).
- [x] `R6-005` Add deterministic 42-spell parity matrix fixtures.

Exit:
- magic behaves as legacy-equivalent system, not modulo template.

## Track R7: Items, Monsters, Combat, Traps Parity

- [x] `R7-001` Implement item use/equip/family effects and identify/bless/curse interactions.
- [x] `R7-002` Implement generation/drop/uniqueness behavior parity.
- [x] `R7-003` Implement monster AI families and special abilities from legacy behavior families.
- [x] `R7-004` Implement tactical combat line/maneuver semantics and faction consequences.
- [x] `R7-005` Implement trap/hazard behavior parity.
- [x] `R7-006` Add encounter matrix covering key monster/item/trap families.

Exit:
- core tactical gameplay loop aligns with legacy systems.

## Track R8: Quests, Progression, Endings Parity

- [x] `R8-001` Implement full quest state machine and branch persistence.
- [x] `R8-002` Implement rank/alignment/deity progression gates and side effects.
- [x] `R8-003` Implement ending eligibility and branch logic.
- [x] `R8-004` Implement score policy parity including wizard interactions.
- [x] `R8-005` Add progression/ending branch matrix fixtures.

Exit:
- real quest-capable full run paths exist.

## Track R9: Save/Options/Wizard Parity

- [x] `R9-001` Expand save schema to persist new map/site/world/quest state fully.
- [x] `R9-002` Preserve migration compatibility and add parity fields defaults/migrations.
- [x] `R9-003` Implement options behavior parity from `help11` with runtime effects.
- [x] `R9-004` Implement wizard-mode command compatibility and score disqualification behavior.
- [x] `R9-005` Add save/load loops across city, country, and active quest branches.

Exit:
- long sessions are persistent and behavior-stable.

## Track R10: Frontend Playability Parity (TUI + Bevy)

- [x] `R10-001` TUI renders true map tiles/sites and supports full-session workflows.
- [x] `R10-002` Bevy renders true map tiles/sites and supports same workflows.
- [x] `R10-003` Enforce shared gameplay input contract over parity-complete command surface.
- [x] `R10-004` Implement diagnostics/help/identify workflows with parity behavior.
- [x] `R10-005` Add cross-frontend equivalence fixtures for canonical sessions.

Exit:
- both frontends are usable for authentic play, not simulation harnesses.

## Track R11: Verification Hardening and Differential Parity

- [x] `R11-001` Build objective gap ledger from runtime behavior checks (not declaration rollups).
- [x] `R11-002` Add differential trace harness against legacy runtime for canonical scripts.
- [x] `R11-003` Add determinism burn-in (`N>=20`) on full parity fixture denominator.
- [x] `R11-004` Add stale artifact and synthetic-behavior detection hard failures.
- [x] `R11-005` Add visual parity checks for startup/city/overworld rendering.

Exit:
- evidence chain reflects real gameplay equivalence.

## Track R12: Playable Full-Game Closure

- [x] `R12-001` Execute canonical scripted sessions: Rampart start -> city services -> overworld travel -> site -> quest progression.
- [x] `R12-002` Execute manual smoke sessions on TUI and Bevy with recorded outcomes.
- [x] `R12-003` Produce approved closure review with zero open P0/P1 parity gaps.
- [x] `R12-004` Freeze parity baseline artifacts and docs.

Exit:
- playable full game with legacy-equivalent systems and verified evidence.

## Mandatory Artifacts (Recovery Contract)

- `target/recovery-gap-ledger.json`
- `target/recovery-contract-guard.json`
- `target/recovery-rampart-startup-visual.json`
- `target/recovery-city-site-matrix.json`
- `target/recovery-overworld-transition-matrix.json`
- `target/recovery-spell-parity-matrix.json`
- `target/recovery-item-parity-matrix.json`
- `target/recovery-monster-combat-trap-matrix.json`
- `target/recovery-progression-ending-matrix.json`
- `target/recovery-save-options-wizard-matrix.json`
- `target/recovery-frontend-equivalence-matrix.json`
- `target/recovery-differential-trace-report.json`
- `target/recovery-burnin-window.json`
- `target/recovery-baseline-freeze.json`
- `docs/migration/FULL_OMEGA_PARITY_RECOVERY_SCORECARD.md`
- `docs/migration/FULL_OMEGA_PARITY_RECOVERY_CLOSURE_REVIEW.md`

## Immediate Execution Queue

1. `RQ-001` Implement map/tile/site runtime model (`R1`).
2. `RQ-002` Wire frontends to true map rendering (`R2`,`R10`).
3. `RQ-003` Implement real city site logic (`R3`).
4. `RQ-004` Implement overworld travel and site transitions (`R4`).
5. `RQ-005` Replace synthetic catalogs and spell/monster/item systems (`R5`,`R6`,`R7`).
6. `RQ-006` Complete quests/progression/endings/save/options/wizard parity (`R8`,`R9`).
7. `RQ-007` Run differential verification and closure package (`R11`,`R12`).
