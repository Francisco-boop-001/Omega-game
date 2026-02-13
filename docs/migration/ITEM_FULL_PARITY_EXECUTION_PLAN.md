# ITEM_FULL_PARITY_EXECUTION_PLAN (Authoritative, No Name-Only Shortcuts)

## Summary
Implement full legacy Omega item parity in the Rust stack using the legacy C source as the executable spec.
Parity target is mechanical and data-complete: item definitions, inventory/equipment semantics, generation, use-effects, combat integration, UI flows, save/load behavior, and verification tooling.

## Execution Tracks (Strict Checklist)
- [x] `IP-001` Build authoritative legacy item spec extractor.
Done evidence:
`cargo run -p omega-tools --bin legacy_item_spec_extract`
Result: `total_items=223`, `unmatched_usef=0`, `parse_drops=0`.

- [x] `IP-002` Introduce full runtime item/equipment data model in core.
Done evidence:
`Item` now carries legacy fields (`usef`, `dmg`, `hit`, `plus`, `fragility`, `blessing`, etc.).
`EquipmentSlots` and typed `ItemFamily` are active runtime state in `crates/omega-core/src/lib.rs`.

- [x] `IP-003` Implement inventory and slot parity semantics.
Done evidence:
`target/classic-inventory-contract.json` PASS with runtime checks for modal inventory flow,
look-vs-show semantics, item prompt selection/lock, and legacy token site choice routing.

- [x] `IP-004` Implement item identification and naming semantics.
Done evidence:
typed identity/name fields (`objstr`, `truename`, `cursestr`, `known`, `used`, `blessing`)
are present in runtime state and validated through strict item parity checks and command-flow smoke.

- [x] `IP-005` Implement family generators (`make_*`) with legacy randomization.
Done evidence:
`target/classic-magic-item-parity.json` PASS (`prototype_cardinality`, `prototype_fields`,
`runtime_typed_instantiation`, `wizard_wish_typed_output`) with legacy catalog-backed instantiation.

- [x] `IP-006` Implement exhaustive `item_use` parity by `usef`.
Done evidence:
All legacy `I_*` tokens are present in core dispatch surface and covered by test:
`tests::item_usef_dispatch_covers_legacy_catalog_without_fallbacks`.
No `modeled fallback` signature remains in item dispatch.

- [x] `IP-007` Integrate combat/effect semantics for weapons and worn gear.
Done evidence:
`target/classic-magic-item-parity.json` PASS (`typed_command_flow`, `weighted_burden`)
and compatibility smoke confirm typed combat/equipment behavior is active in runtime.

- [x] `IP-008` Implement acquisition/wish/shop/drop with full item instances.
Done evidence:
Wizard acquisition and runtime pickup/drop paths instantiate typed catalog-backed items; placeholder grant names are blocked by tests/tooling.

- [x] `IP-009` Save/load migration and backward compatibility.
Done evidence:
`cargo test -p omega-save` PASS; legacy-like payload decode and roundtrip tests pass with typed fields/defaults.

- [x] `IP-010` Frontend interaction parity for inventory/equipment UX.
Done evidence:
`target/classic-frontend-workflow-parity.json` PASS and inventory modal contract PASS
across TUI/Bevy interaction routing surfaces.

- [x] `IP-011` Replace shallow parity tooling with strict matrices.
Done evidence:
`classic_magic_item_parity` now includes `usef` runtime token coverage and fallback-signature guard.
`true_parity_refresh` item matrix includes `legacy_usef_tokens_covered`.

- [x] `IP-012` Closure evidence and contract correction.
Done evidence:
`target/full-parity-defect-board.json` (`open_total=0`) and
`target/full-parity-smoke.json` (PASS) generated in strict closure pass.

## Validation Snapshot
- `cargo test -p omega-core` PASS (`61` tests)
- `cargo test -p omega-content` PASS
- `cargo test -p omega-save` PASS
- `cargo test -p omega-tui` PASS
- `cargo test -p omega-bevy` PASS
- `cargo run -p omega-tools --bin classic_magic_item_parity` PASS (`8/8`)
- `cargo run -p omega-tools --bin true_parity_refresh` PASS
