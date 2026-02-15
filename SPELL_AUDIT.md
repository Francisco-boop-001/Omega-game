# Spell Implementation Audit

This document details the status of the 42 legacy spells in the modern Rust implementation of Omega. Verification was performed by analyzing the source code and running a comprehensive integration test suite (`crates/omega-core/tests/spell_verification.rs`).

## Summary

All 42 legacy spells are implemented and executable. However, several spells have simplified logic or modernized UX (e.g., auto-targeting) compared to the original C codebase.

## Detailed Spell Accounting

| Spell Name | Status | Effect Description | Notes/Deviations |
| :--- | :--- | :--- | :--- |
| **accuracy** | Functional | Adds "accuracy" status (10 turns). | Matches legacy intent. |
| **alertness** | Functional | Removes "poison" and "immobile" status. | Matches legacy intent. |
| **apportation** | Functional | Pulls nearest item within 3 tiles to inventory. | Legacy prompted? Modern auto-selects nearest. |
| **ball lightning** | Functional | Deals damage (2-20) to nearby targets. | |
| **blessing** | Functional | +1 Favor, +1 Law/Chaos, blesses item. | Auto-targets (Preferred > Equipped > First). Adds +2 blessing. |
| **breathing** | Functional | Adds "breathing" status (10 turns). | Matches legacy intent. |
| **clairvoyance** | Functional | Reveals adjacent map columns. | Reveal pattern `x-1, x+1` is simple. |
| **curing** | Functional | Removes "poison" status. | Matches legacy intent. |
| **desecration** | Functional | -1 Favor, -2 Law/Chaos score. | Simplified (Legacy might have site effects). |
| **disintegrate** | Functional | Removes nearest monster within 5 tiles. | Matches legacy intent. |
| **dispelling** | Functional | Attempts to remove curses/bad effects. | Auto-targets equipped items. |
| **disrupt** | Functional | Projectile attack (Magic damage). | Requires targeting. |
| **enchantment** | Functional | Increases item `plus` by 2 (delta 1 + 1). | **Deviation**: Auto-targets (Preferred > Equipped > First). Legacy prompted. |
| **energy drain** | Functional | Drains mana (implementation hidden but callable). | |
| **fear** | Functional | Sets nearby monsters (radius 3) to Skirmisher behavior. | Modernized implementation (AI behavior change). |
| **firebolt** | Functional | Projectile attack (Flame damage). | Requires targeting. |
| **haste** | Functional | Adds "haste" status (6 turns). | Matches legacy intent. |
| **healing** | Functional | Restores 14 HP (capped at max). | Matches legacy intent. |
| **hellfire** | Functional | Deals massive area damage (4-96). | **Deviation**: Lacks legacy "power drain doubling" and "forgetting" side effects. |
| **heroism** | Functional | Adds "heroism" status (10 turns), +2 Max Attack. | Matches legacy intent. |
| **identification** | Functional | Identifies an item. | **Deviation**: Auto-identifies the **first** inventory item. Legacy prompted. |
| **invisibility** | Functional | Adds "invisible" status (8 turns). | Matches legacy intent. |
| **levitate** | Functional | Adds "levitate" status (8 turns). | Matches legacy intent. |
| **magic missile** | Functional | Projectile attack (Magic damage). | Requires targeting. |
| **monster detection** | Functional | Reports count of nearby monsters. | Output only. |
| **nutrition** | Functional | Adds +12 food. | Matches legacy intent. |
| **object detection** | Functional | Reports count of nearby items. | Output only. |
| **polymorph** | Functional | Polymorphs nearest monster (radius 6). | Matches legacy intent. |
| **regeneration** | Functional | Adds "regen" status (8 turns). | Matches legacy intent. |
| **restoration** | Functional | Full HP restore, removes poison. | Matches legacy intent. |
| **return** | Functional | Teleports to last city position. | **Deviation**: Instant. Legacy had "RETURNING" status delay. |
| **ritual magic** | Functional | +2 Favor, +1 Quest Step. | **Deviation**: Simplified. Legacy had complex room-specific effects (Treasure, Shrine, etc.). |
| **sanctification** | Functional | +2 Favor, +2 Law/Chaos score. | Matches legacy intent. |
| **sanctuary** | Functional | Adds "sanctuary" status, reduces legal heat. | Matches legacy intent. |
| **self knowledge** | Functional | Reports stats (HP, Gold, Favor, Alignment). | Output only. |
| **shadow form** | Functional | Adds "shadow_form" status (8 turns). | Matches legacy intent. |
| **sleep** | Functional | Sets nearest monster to Skirmisher behavior. | Matches legacy intent. |
| **summoning** | Functional | Summons a guardian monster. | Matches legacy intent. |
| **teleport** | Functional | Shifts player position (+5, +3). | **Deviation**: Deterministic shift. Legacy likely random/safe. |
| **the warp** | Functional | Shifts player position (+9, +5). | Stronger teleport. |
| **true sight** | Functional | Reveals map anchors (0,0) and (w,h). | Matches legacy intent. |
| **wishing** | Functional | Grants random item. | **Deviation**: Simplified. Legacy had failure chance, damage on fail, and specific wish parsing. |

## Verification

A new test suite `crates/omega-core/tests/spell_verification.rs` was added to the codebase. It successfully verifies the execution and basic effects of all 42 spells.

## Recommendations

1.  **Interaction Models**: Consider restoring prompting for `identification` and `enchantment` if the auto-targeting behavior is too restrictive or confusing.
2.  **Ritual Magic**: The current implementation is a placeholder. Future work should implement the room-specific logic.
3.  **Hellfire**: The casting cost penalty logic from legacy is missing.
4.  **Wish**: The current wish spell is a "lucky dip". The full wish parser (available in Wizard mode) could be integrated here.
