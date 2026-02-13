# Full Omega Parity Reality Audit (2026-02-07)

Status: Authoritative audit
Date: 2026-02-07
Scope: Validate actual gameplay parity against legacy Omega runtime sources.

## Objective Under Audit

Target objective (user-defined):

1. New game starts in Rampart city.
2. Rampart has full city map and functioning city systems (stores, guilds, palace, etc.).
3. Exiting Rampart reaches real overworld map.
4. Quests, items, and magic are legacy-parity functional.

## Method

1. Inspected Rust runtime behavior in `crates/omega-core`, `crates/omega-content`, `crates/omega-tui`, `crates/omega-bevy`.
2. Inspected legacy authority in `archive/legacy-c-runtime/2026-02-06/*.c` and `defs.h`.
3. Compared functional systems, not report artifacts.

## Critical Findings

## F-001 (P0): Runtime has no persistent tile/site map model

Evidence:
- `crates/omega-core/src/lib.rs:358` uses only bounds + entities, not level tile/site arrays.
- `crates/omega-core/src/lib.rs:640` movement only blocks on bounds/monster occupancy.
- Legacy uses full site grids with per-tile location functions in `archive/legacy-c-runtime/2026-02-06/city.c:13`.

Impact:
- Rampart cannot be represented with true walls, doors, sites, city exits, and special tiles.

## F-002 (P0): Frontends render synthetic void/floor, not loaded map tiles

Evidence:
- TUI renders only `@`, `m`, `*`, `.` in `crates/omega-tui/src/lib.rs:369` and `crates/omega-tui/src/lib.rs:392`.
- Bevy projects every map cell as floor in `crates/omega-bevy/src/lib.rs:393` and `crates/omega-bevy/src/lib.rs:397`.

Impact:
- User sees a void-like map even when `city.map` is loaded for bootstrap diagnostics.

## F-003 (P0): Bootstrap injects synthetic entities and generic labels

Evidence:
- Bootstrap selects `city.map` (`crates/omega-content/src/lib.rs:276`) but spawns `legacy-monster-*` with fixed stats (`crates/omega-content/src/lib.rs:335`) and `legacy-item-*` (`crates/omega-content/src/lib.rs:365`).

Impact:
- Immediate attacks by placeholder monsters and non-parity city population.

## F-004 (P0): City systems are replaced by simplified service roll

Evidence:
- `>` transition in `crates/omega-core/src/lib.rs:827` uses `service_roll` with four abstract outcomes.
- Legacy city maps and site function assignments are explicit and numerous (`L_BANK`, `L_TEMPLE`, `L_ARENA`, `L_COLLEGE`, etc.) in `archive/legacy-c-runtime/2026-02-06/city.c:85`, `archive/legacy-c-runtime/2026-02-06/city.c:112`, `archive/legacy-c-runtime/2026-02-06/city.c:193`, `archive/legacy-c-runtime/2026-02-06/city.c:200`.
- Legacy site behavior bodies live in `archive/legacy-c-runtime/2026-02-06/site1.c:11` and `archive/legacy-c-runtime/2026-02-06/site2.c:682`.

Impact:
- Stores/guilds/palace/economy/social loops are not actually implemented.

## F-005 (P0): Overworld is not implemented as legacy terrain/site grid

Evidence:
- Rust world mode is only two states (`DungeonCity`, `Countryside`) in `crates/omega-core/src/lib.rs:64`.
- Legacy defines full environment set in `archive/legacy-c-runtime/2026-02-06/defs.h:300` and loads country terrain/site map in `archive/legacy-c-runtime/2026-02-06/country.c:9` and `archive/legacy-c-runtime/2026-02-06/country.c:21`.

Impact:
- Exiting Rampart cannot lead to authentic overworld traversal and site entry behavior.

## F-006 (P0): Spell system is synthetic, not 42 distinct legacy spell semantics

Evidence:
- Rust casting cycles by `next_spell_index % 42` but dispatches only `spell_index % 6` behavior templates in `crates/omega-core/src/lib.rs:1282` and `crates/omega-core/src/lib.rs:1285`.
- Legacy initializes explicit per-spell properties and semantics in `archive/legacy-c-runtime/2026-02-06/spell.c:516`, `archive/legacy-c-runtime/2026-02-06/spell.c:532`, `archive/legacy-c-runtime/2026-02-06/spell.c:610`, `archive/legacy-c-runtime/2026-02-06/spell.c:643`.

Impact:
- Magic parity is not met.

## F-007 (P0): Monster and item catalogs are placeholders

Evidence:
- Placeholder catalog generation via numbered names in `crates/omega-content/src/lib.rs:96`.
- Legacy uses full static object and monster tables (`Objects[]`, `Monsters[]`) in `archive/legacy-c-runtime/2026-02-06/item.c:8`, `archive/legacy-c-runtime/2026-02-06/mon.c:12`, `archive/legacy-c-runtime/2026-02-06/omega.c:37`.

Impact:
- Item, monster, and encounter parity are not met.

## F-008 (P1): Current parity reports can pass without full-game parity

Evidence:
- `true_parity_refresh` builds true artifacts from existing `classic-*` report files (`crates/omega-tools/src/bin/true_parity_refresh.rs:177`, `crates/omega-tools/src/bin/true_parity_refresh.rs:183`, `crates/omega-tools/src/bin/true_parity_refresh.rs:322`).

Impact:
- Artifact PASS does not guarantee the user-facing objective.

## Conclusion

Parity is NOT complete for the required gameplay objective.

Current implementation is a reduced simulation with partial command/economy/combat behaviors and legacy-themed artifacts, not full classic Omega gameplay/content parity.

## Required Next Contract

A new execution contract must require:

1. Real tile/site map runtime for city + overworld + dungeon/special environments.
2. Site-function parity for Rampart and major location systems.
3. Real catalog ingestion for spells/items/monsters/traps.
4. Differential behavior checks against legacy runtime scenarios.
5. User-visible playability proof: full Rampart -> overworld -> quest progression sessions.
