# Omega Modernization Plan (Rust + Bevy + serde + ratatui)

> Deprecated as an active execution plan on 2026-02-07.
> Use `docs/migration/PLAYABLE_OMEGA_STRICT_PLAN.md` as the sole authoritative execution plan.
> This document remains as architecture and historical context only.


## 1. Purpose and Constraints

This plan defines a staged, parallelizable refactor of the current Omega C codebase into a multi-crate Rust workspace with:
- `omega-core` (deterministic simulation)
- `omega-bevy` (graphical frontend)
- `omega-tui` (terminal frontend via ratatui)
- `omega-save` (versioned serialization/migrations via serde)
- `omega-content` (assets/content parsing + validation)

Primary goals:
- Preserve game behavior while replacing risky legacy architecture.
- Enable multiple agents/teams to work in parallel with low merge conflict risk.
- Keep the system shippable at each major milestone.

Non-goals (initially):
- Full gameplay redesign.
- Asset/art overhaul.
- Network/multiplayer.

## 2. Current-State Summary (Why this plan)

Key traits in current code:
- Global mutable state spread across modules (`omega.c`, `glob.h`, `defs.h`).
- Single infinite command loop (`omega.c`) with environment-driven branching.
- Binary save/load tightly coupled to struct memory layout (`save.c`).
- Widespread unsafe string handling patterns (`strcpy`, `strcat`, `sprintf`).
- Legacy build and platform ifdef complexity (Makefiles, BSD/SYSV/MSDOS/AMIGA).

Implications:
- High regression risk if directly rewriting UI + core together.
- Save compatibility and determinism must be designed explicitly.
- Frontend must be decoupled from simulation from day one.

## 3. Target Architecture

## 3.1 Workspace Layout

```text
omega/
  Cargo.toml (workspace)
  crates/
    omega-core/
    omega-content/
    omega-save/
    omega-bevy/
    omega-tui/
    omega-tools/      # optional: converters, validators, replay tools
  docs/
    architecture/
    migration/
```

## 3.2 Ownership Boundaries

- `omega-core`
  - Pure domain model and turn simulation.
  - No terminal/UI/game-engine dependencies.
  - Input: abstract commands/events. Output: state transitions + render-friendly projections.

- `omega-content`
  - Load/validate static content (maps, entities, loot tables, dialogs).
  - Convert legacy map/content formats into typed domain structures.

- `omega-save`
  - serde models, schema versions, and migration pipeline.
  - Save/load of core state only (not renderer internals).

- `omega-bevy`
  - Rendering, camera, animation, audio, input mapping.
  - Adapts core state to ECS/UI views.

- `omega-tui`
  - ratatui rendering and keyboard handling.
  - Uses same core command/event API as Bevy frontend.

- `omega-tools`
  - Golden test tools, replay recorder/player, content linting, benchmarks.

## 3.3 Core Design Rules

- Domain logic must be deterministic and side-effect controlled.
- No direct random calls in gameplay logic; use injected RNG trait/service.
- No filesystem access from `omega-core`.
- Save format versioned from v1 onward.
- Frontends never mutate domain state directly; they issue commands.

## 4. Parallel Workstreams (Agent-Friendly)

Each workstream has independent deliverables and interfaces to reduce blocking.

## WS-A: Foundation and Governance

Scope:
- Create workspace, CI, linting, formatting, branch conventions, ADR templates.

Deliverables:
- Cargo workspace skeleton.
- CI matrix (`fmt`, `clippy`, tests, docs).
- Code owners per crate.

Dependencies:
- None.

Can run in parallel with:
- WS-B, WS-C, WS-D (after initial crate stubs exist).

## WS-B: Domain Core Extraction (`omega-core`)

Scope:
- Build canonical game state model and turn engine.
- Start with movement/combat/inventory/time subsystems.

Deliverables:
- `GameState`, `Command`, `Outcome` APIs.
- Deterministic simulation loop API (`step(state, command, rng)`).
- Unit tests for rules and invariants.

Dependencies:
- Interface alignment with WS-E (save schema) and WS-F/G (frontend adapter contracts).

Can run in parallel with:
- WS-C, WS-D, WS-E, WS-H.

## WS-C: Content Pipeline (`omega-content`)

Scope:
- Parse legacy maps/content and produce typed validated structures.

Deliverables:
- Legacy loader for map data.
- Content schema and validation report.
- Build-time content checks.

Dependencies:
- Minimal model contracts from WS-B.

Can run in parallel with:
- WS-B, WS-E, WS-F, WS-G.

## WS-D: Behavior Parity Harness

Scope:
- Build a black-box parity framework against legacy behavior snapshots.

Deliverables:
- Golden scenarios (movement, combat outcomes, map transitions, status effects).
- Replay fixture format + runner.
- Regression dashboard.

Dependencies:
- WS-B API stable enough for test harness integration.

Can run in parallel with:
- WS-C, WS-E, WS-H.

## WS-E: Save/Load Modernization (`omega-save`)

Scope:
- Replace raw binary dumps with serde-based schema + migration chain.

Deliverables:
- `SaveEnvelope { version, payload, metadata }`.
- v1 schema and migration scaffolding.
- Round-trip and compatibility tests.

Dependencies:
- Domain structs from WS-B.

Can run in parallel with:
- WS-C, WS-F, WS-G.

## WS-F: ratatui Frontend (`omega-tui`)

Scope:
- Deliver playable terminal client first for fast parity verification.

Deliverables:
- Event loop + input mapping.
- Main map/status/inventory panels.
- Command dispatch into `omega-core`.

Dependencies:
- WS-B stable command API.

Can run in parallel with:
- WS-G, WS-E, WS-H.

## WS-G: Bevy Frontend (`omega-bevy`)

Scope:
- Build graphical client adapter over same command/state APIs.

Deliverables:
- Bevy app states (Boot/Menu/InGame/Pause/GameOver).
- Map rendering and sprite/tile pipeline.
- Input mapping parity with TUI.

Dependencies:
- WS-B command/outcome API.
- Content contracts from WS-C.

Can run in parallel with:
- WS-F, WS-E, WS-H.

## WS-H: Quality, Security, and Performance

Scope:
- Test infra, property tests, fuzzing, profiling, release quality gates.

Deliverables:
- `nextest`, proptest, fuzz targets.
- Perf baselines and budget checks.
- Security checklist (save file hardening, path safety, panic policy).

Dependencies:
- Minimal compilation of WS-B/WS-E.

Can run in parallel with:
- All workstreams.

## WS-I: Documentation and Developer Experience

Scope:
- Keep architecture docs, migration notes, playbooks, onboarding current.

Deliverables:
- ADRs, module diagrams, coding standards.
- “How to add a new command/effect/content item.”

Dependencies:
- Continuous sync with WS-A..H.

Can run in parallel with:
- All workstreams.

## 5. Milestone Plan (with Parallel Execution)

## Milestone 0: Bootstrap (1-2 weeks)

Exit criteria:
- Workspace compiles.
- CI green.
- Stubs for all crates.

Parallel tasks:
- A1: Workspace/CI setup (WS-A).
- I1: ADR template + architecture docs seed (WS-I).
- H1: Test harness scaffolding (WS-H).

## Milestone 1: Core Skeleton + Content Ingest (2-4 weeks)

Exit criteria:
- `omega-core` can execute basic turns in tests.
- `omega-content` loads key maps/content types.

Parallel tasks:
- B1: State and command model v1.
- C1: Legacy map converter/loader.
- D1: Golden scenario definitions.
- E1: Save envelope prototype.

## Milestone 2: Playable TUI Vertical Slice (3-6 weeks)

Exit criteria:
- End-to-end playable loop in terminal.
- Save/load v1 works.

Parallel tasks:
- F1: ratatui game loop/UI.
- B2: Combat/inventory/time systems.
- E2: serde schema stabilization.
- D2: Parity test expansion.
- H2: Property tests + fuzz seeds.

## Milestone 3: Bevy Vertical Slice (4-8 weeks)

Exit criteria:
- Playable Bevy client using same core.
- Input parity with TUI for shared commands.

Parallel tasks:
- G1: Rendering and app state flow.
- C2: Asset pipeline integration.
- B3: Projection/query APIs for frontend.
- H3: Frame-time and memory profiling.

## Milestone 4: Feature Parity and Legacy Decommission (closed 2026-02-06)

Exit criteria:
- Parity thresholds in section `5.1` are met.
- Legacy C runtime decommission criteria in section `5.2` are all met.

Parallel tasks:
- B4/C3: Remaining mechanics/content.
- D3: Replay parity confidence target achieved.
- E3: Save migration tooling.
- I2: Migration guides and contributor docs.

Closure evidence:
- `docs/migration/MILESTONE4_CLOSURE_REVIEW.md`
- `docs/migration/MILESTONE4_DECOMMISSION_CHECKLIST.md`

## Milestone 5: Productization, Reliability, and Extensibility (active)

Exit criteria:
- Productization and runtime-maturity thresholds in section `5.3` are met.
- Release/support operations runbooks are updated for steady-state Rust-only releases.
- Milestone 4 regression guardrails (`m4-daily`) remain green through M5 stabilization.

Parallel tasks:
- F3/G3: Frontend runtime polish, UX flow completion, and input/accessibility settings.
- C4/E4: Content-pack and mod compatibility contracts (schema + save metadata).
- H5/A2: Observability, security hardening, and release automation upgrades.
- I3: Documentation and contributor DX upgrades for new extension points.

## 5.1 Milestone 4 Parity Targets (Formal)

Parity is measured from replay/golden scenarios in WS-D across both frontends (`omega-tui` and `omega-bevy`) against approved legacy baselines.

Required thresholds:
- Golden scenario pass rate: `>= 98.0%` for `14` consecutive daily CI runs.
- Critical path pass rate (save/load, map transitions, combat resolution, inventory mutations): `100%` for `14` consecutive daily CI runs.
- Determinism: `100%` match for repeated seeded runs (`N=20` per scenario family) with zero divergent hashes.
- Save compatibility: `100%` pass on migration fixtures for all supported legacy save versions in scope.
- Severity gate: `0` open parity defects labeled `P0` and `<= 3` open parity defects labeled `P1` (all with accepted mitigation notes).
- Frontend command parity (shared command set): `100%` mapping parity between TUI and Bevy for commands defined in `omega-core::Command`.

Measurement rules:
- Denominator for pass-rate thresholds must include at least `500` total replay scenarios per daily run.
- Any scenario marked flaky is excluded only with a linked issue and owner; flaky exclusions must remain `< 2%` of suite.
- Threshold window resets if any daily run falls below target.

## 5.2 Legacy C Runtime Decommission Criteria (Formal)

The legacy C runtime is considered decommissioned only when all criteria below are true:
- Release path cutover:
  - Rust frontends/core are the default production path for `2` consecutive release candidates.
  - Legacy C runtime is not invoked by default in packaging, installers, or launch scripts.
- Rollback safety:
  - A documented rollback runbook exists and is validated in one dry-run on staging.
  - Save migration rollback expectations are documented (what can/cannot be rolled back).
- Quality and operations:
  - Crash-free session rate is `>= 99.5%` over a rolling `14`-day pre-release window.
  - No open `P0` defects in core gameplay, save/load, or startup paths.
  - Performance budget check: median turn-processing latency regression `<= 10%` vs agreed baseline on the parity benchmark set.
- CI and repository state:
  - Legacy C binary is removed from required CI release jobs.
  - Legacy build remains optional/manual only during one grace release, then removed from release branch policy.
  - Ownership for remaining legacy files is explicitly assigned (archive or delete decision recorded).
- Documentation and support:
  - Migration guide for players/modders is published and versioned.
  - Release notes explicitly announce legacy runtime retirement date and support policy.
  - On-call/support playbook references only Rust runtime paths for active releases.

## 5.3 Milestone 5 Productization and Runtime-Maturity Targets (Formal)

Milestone 5 focuses on ship-grade product operations after parity/decommission closure.

Required thresholds:
- End-to-end flow reliability:
  - Scripted flow (`new game -> save -> load -> game over -> restart`) passes in both TUI and Bevy for `30` consecutive daily runs.
- Runtime stability:
  - Crash-free session rate is `>= 99.9%` over a rolling `30`-day pre-release window.
  - Boot-to-interactive success rate is `>= 99.95%` on the supported CI smoke matrix.
- Performance:
  - Median turn-processing latency regression is `<= 5%` versus frozen Milestone 4 baseline.
  - Bevy p99 frame time is `<= 16.7ms` on agreed benchmark scene/profile.
  - Cold-start time to interactive menu is `<= 3.0s` on agreed reference hardware.
- Quality and defect control:
  - `0` open `P0` defects.
  - `<= 2` open `P1` defects older than `7` days, each with mitigation owner.
  - Flaky exclusions remain `<= 1%` of suite and every exclusion has issue + owner + expiry.
- Security and supply chain:
  - Daily fuzz smoke passes and weekly deep-fuzz campaign report is published with no untriaged crashes.
  - Dependency audit reports `0` known high/critical vulnerabilities at RC cut.
- Release operations:
  - Cross-platform release artifacts are generated/verified for `2` consecutive RCs.
  - Install/update/rollback drill is executed once per RC cycle with documented outcome.
- Extensibility and DX:
  - Content/schema compatibility matrix for active mod/content packs is published and versioned.
  - "How to add command/effect/content" guide is validated by at least one onboarding dry-run contributor.

Measurement rules:
- A daily run counts only if all required artifacts for that day are present.
- Missing artifact equals failed day for window counting.
- Any manual override must include linked issue, owner, and expiry date.
- Consecutive window resets on any failed day.

## 6. Task Decomposition for Multi-Agent Assignment

Use this template for each ticket:
- ID
- Crate owner
- Interface contract (input/output types)
- Test requirements
- Blocking dependencies
- Reviewer from another stream

Example independent ticket clusters:
- Cluster 1 (`omega-core`): command parser, movement system, combat rules.
- Cluster 2 (`omega-content`): map parser, loot table parser, validation CLI.
- Cluster 3 (`omega-save`): schema v1, migration runner, corruption handling.
- Cluster 4 (`omega-tui`): layout widgets, input bindings, message log.
- Cluster 5 (`omega-bevy`): tile renderer, camera follow, HUD.
- Cluster 6 (`quality`): property tests, replay runner, fuzzing.

## 7. Interface Contracts (Stabilize Early)

Define these first to unlock parallel work:
- `Command` enum (frontend -> core).
- `Outcome/Event` enum (core -> frontend/log).
- `GameSnapshot` read model (core -> renderer).
- `ContentProvider` trait (core <- content).
- `SaveCodec` trait (`encode/decode/migrate`).

Rules:
- No frontend crate imports another frontend crate.
- Only frontends depend on `omega-core`; core never depends on frontends.
- Save and content crates may depend on core model crate/types only.

## 8. Testing Strategy

Testing pyramid:
- Unit tests in each crate.
- Contract tests for cross-crate interfaces.
- Golden replay tests for behavioral parity.
- Property tests for invariants.
- Fuzz tests for parsers and save decoding.

Minimum gates per PR:
- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- unit + contract tests
- if parser/save touched: fuzz corpus smoke
- if core logic touched: golden replay subset

## 9. Risk Register and Mitigations

Risk: Behavior drift during rewrite.
- Mitigation: replay/golden parity harness before large rewrites.

Risk: Bevy API churn.
- Mitigation: pin Bevy version per release cycle; isolate Bevy adapter layer.

Risk: Save incompatibility.
- Mitigation: versioned serde envelope + migration tests.

Risk: Content conversion bugs.
- Mitigation: deterministic validation reports + checksum checks.

Risk: Team merge conflicts.
- Mitigation: strict crate boundaries and interface freeze windows.

## 10. Definition of Done (Per Milestone)

A milestone is done only if:
- Functional criteria met.
- Test/quality gates pass.
- Docs/ADR updated.
- Performance does not regress beyond agreed budget.
- Release notes include migration implications.

## 11. Recommended Branching and Release Cadence

- Trunk-based with short-lived feature branches.
- Weekly integration branch for cross-stream compatibility checks.
- Monthly stabilization release candidate.
- Feature flags for incomplete subsystems.

## 12. Immediate Next 10 Tickets (Post-M4 / M5 Kickoff)

1. Publish Milestone 5 execution plan and scorecard baselines.
2. Freeze Milestone 4 performance baseline snapshot and wire M5 `<=5%` regression check.
3. Add M5 daily gate workflow with artifact aggregation and failure summary.
4. Implement scripted end-to-end flow tests for both frontends (new/save/load/gameover/restart).
5. Add boot-to-interactive smoke matrix across supported OS targets.
6. Add Bevy benchmark scene + p99 frame-time reporter for CI gating.
7. Extend save envelope metadata for content-pack/mod compatibility.
8. Add content schema/version validation CLI checks in `omega-content`/`omega-tools`.
9. Automate dependency/security audit reporting in CI and release checklists.
10. Publish updated contributor onboarding playbook and run first dry-run.

## 13. Success Metrics

- Parity: percentage of golden scenarios matched.
- Stability: crash-free sessions and test pass rate.
- Velocity: cycle time per feature after milestone 2.
- Maintainability: reduced unsafe patterns and lower bug density.
- Onboarding: time for new contributor to land first feature.

---

This plan is intentionally modular so multiple agents can execute in parallel with minimal coupling. The key enabler is early contract stabilization (`Command`, `Outcome`, `Snapshot`, content/save traits).
