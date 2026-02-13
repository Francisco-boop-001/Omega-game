# FULL_MECHANICS_PARITY_AUDIT_V2

## Summary

- generated_at_utc: `unix:1770859361`
- total mechanics compared: `1313`
- pass: `true`
- unknown: `0`
- main_non_equivalent: `0`
- unresolved_gameplay: `0`
- gameplay_excluded: `0`

## Legacy Mechanics Inventory

- source: `legacy_omega_source_snapshot`
- total: `1313`
- ledger artifact: `target/legacy-mechanics-ledger.json`

| tier | count |
|---|---:|
| Main | 329 |
| Secondary | 474 |
| Tertiary | 62 |
| Rest | 448 |

## Rust Mechanics Inventory

- source: `rust_omega_runtime`
- total: `1410`
- ledger artifact: `target/rust-mechanics-ledger.json`

| tier | count |
|---|---:|
| Main | 125 |
| Secondary | 104 |
| Tertiary | 294 |
| Rest | 887 |

## Parity Verdict by Mechanic

| mechanic_id | status | tier | domain | notes |
|---|---|---|---|---|
| legacy.command.A | Exact | Main | magic_and_items | direct symbol match |
| legacy.command.C | Exact | Main | inventory_and_equipment | direct symbol match |
| legacy.command.D | Exact | Main | combat_and_interaction | direct symbol match |
| legacy.command.E | Exact | Rest | misc_commands | direct symbol match |
| legacy.command.F | Exact | Rest | misc_commands | direct symbol match |
| legacy.command.G | Exact | Main | combat_and_interaction | direct symbol match |
| legacy.command.H | Exact | Rest | misc_commands | direct symbol match |
| legacy.command.I | Exact | Main | inventory_and_equipment | direct symbol match |
| legacy.command.M | Exact | Rest | misc_commands | direct symbol match |
| legacy.command.O | Exact | Tertiary | ui_and_help | direct symbol match |
| legacy.command.P | Exact | Tertiary | ui_and_help | direct symbol match |
| legacy.command.Q | Exact | Main | session_and_victory | direct symbol match |
| legacy.command.R | Exact | Main | session_and_victory | direct symbol match |
| legacy.command.S | Exact | Main | session_and_victory | direct symbol match |
| legacy.command.T | Exact | Main | combat_and_interaction | direct symbol match |
| legacy.command.V | Exact | Tertiary | ui_and_help | direct symbol match |
| legacy.command.Z | Exact | Rest | misc_commands | direct symbol match |
| legacy.command.a | Exact | Main | magic_and_items | direct symbol match |
| legacy.command.at | Exact | Rest | misc_commands | direct symbol match |
| legacy.command.c | Exact | Rest | misc_commands | direct symbol match |
| legacy.command.comma | Exact | Main | movement_and_traversal | direct symbol match |
| legacy.command.ctrl_f | Exact | Tertiary | wizard_and_debug | direct symbol match |
| legacy.command.ctrl_g | Exact | Tertiary | wizard_and_debug | direct symbol match |
| legacy.command.ctrl_i | ExcludedNonGameplay | Rest | misc_commands | platform/presentation-only surface |
| legacy.command.ctrl_l | Exact | Tertiary | ui_and_help | direct symbol match |
| legacy.command.ctrl_o | Exact | Tertiary | ui_and_help | direct symbol match |
| legacy.command.ctrl_p | Exact | Tertiary | ui_and_help | direct symbol match |
| legacy.command.ctrl_r | Exact | Tertiary | ui_and_help | direct symbol match |
| legacy.command.ctrl_w | Exact | Tertiary | wizard_and_debug | direct symbol match |
| legacy.command.ctrl_x | Exact | Tertiary | wizard_and_debug | direct symbol match |
| legacy.command.d | Exact | Main | inventory_and_equipment | direct symbol match |
| legacy.command.dot | Exact | Main | movement_and_traversal | direct symbol match |
| legacy.command.e | Exact | Main | magic_and_items | direct symbol match |
| legacy.command.f | Exact | Main | combat_and_interaction | direct symbol match |
| legacy.command.g | Equivalent | Main | inventory_and_equipment | covered by equivalent command/data behavior |
| legacy.command.gt_ | Exact | Main | movement_and_traversal | direct symbol match |
| legacy.command.help | Exact | Tertiary | ui_and_help | direct symbol match |
| legacy.command.i | Exact | Main | inventory_and_equipment | direct symbol match |
| legacy.command.lt_ | Exact | Main | movement_and_traversal | direct symbol match |
| legacy.command.m | Exact | Main | magic_and_items | direct symbol match |
| legacy.command.o | Exact | Rest | misc_commands | direct symbol match |
| legacy.command.p | Exact | Main | combat_and_interaction | direct symbol match |
| legacy.command.q | Exact | Main | magic_and_items | direct symbol match |
| legacy.command.r | Exact | Main | magic_and_items | direct symbol match |
| legacy.command.s | Exact | Rest | misc_commands | direct symbol match |
| legacy.command.slash | Exact | Tertiary | ui_and_help | direct symbol match |
| legacy.command.t | Exact | Main | combat_and_interaction | direct symbol match |
| legacy.command.v | Exact | Main | movement_and_traversal | direct symbol match |
| legacy.command.x | Exact | Rest | misc_commands | direct symbol match |
| legacy.command.z | Exact | Main | magic_and_items | direct symbol match |
| legacy.define.L_ABYSS | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_ADEPT | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_AIR_STATION | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_ALCHEMIST | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_ALTAR | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_ARENA | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_ARENA_EXIT | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_ARMORER | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_BALANCESTONE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_BANK | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_BROTHEL | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_CARTOGRAPHER | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_CASINO | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_CASTLE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_CEMETARY | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_CHAOS | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_CHAOSTONE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_CHARITY | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_CIRCLE_LIBRARY | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_CLUB | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_COLLEGE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_COMMANDANT | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_COMMONS | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_CONDO | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_COUNTRYSIDE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_CRAP | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_DINER | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_DPW | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_DROP_EVERY_PORTCULLIS | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_DRUID | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_EARTH_STATION | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_ENTER_CIRCLE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_ENTER_COURT | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_ESCALATOR | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_FDEMON | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_FINAL_ABYSS | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_FIRE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_FIRE_STATION | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_GARDEN | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_GRANARY | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_GYM | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_HEALER | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_HEDGE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_HOUSE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_HOUSE_EXIT | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_HOVEL | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_JAIL | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_LAVA | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_LAWSTONE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_LIBRARY | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_LIFT | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_MAGIC_POOL | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_MANSION | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_MAZE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_MERC_GUILD | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_MINDSTONE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_MONASTERY | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_NO_OP | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_OCCUPIED_HOUSE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_OMEGA | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_ORACLE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_ORDER | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_PAWN_SHOP | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_PORTCULLIS | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_PORTCULLIS_TRAP | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_RAISE_PORTCULLIS | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_RUBBLE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_SACRIFICESTONE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_SAFE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_SEWER | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_SORCERORS | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_STABLES | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_STATUE_RANDOM | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_STATUE_WAKE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_TACTICAL_EXIT | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_TAVERN | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_TEMPLE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_TEMPLE_WARNING | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_THIEVES_GUILD | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_THRONE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_TOME1 | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_TOME2 | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_TOURIST | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_TRAP_ABYSS | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_TRAP_ACID | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_TRAP_BLADE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_TRAP_DART | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_TRAP_DISINTEGRATE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_TRAP_DOOR | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_TRAP_FIRE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_TRAP_MANADRAIN | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_TRAP_PIT | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_TRAP_SIREN | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_TRAP_SLEEP_GAS | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_TRAP_SNARE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_TRAP_TELEPORT | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_TRIFID | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_VAULT | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_VOICE1 | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_VOICE2 | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_VOICE3 | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_VOID | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_VOIDSTONE | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_VOID_STATION | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_WARNING | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_WATER | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_WATER_STATION | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.L_WHIRLWIND | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.MAXCONNECTIONS | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.MAXITEMS | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.MAXLENGTH | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.MAXLEVELS | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.MAXPACK | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.MAXROOMS | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.MAXWIDTH | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.M_INVISIBLE | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MELEE_COLD | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MELEE_DEATH | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MELEE_DISEASE | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MELEE_DRAGON | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MELEE_ELEC | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MELEE_FIRE | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MELEE_GRAPPLE | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MELEE_MASTER | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MELEE_MP | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MELEE_NG | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MELEE_NORMAL | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MELEE_POISON | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MELEE_SLEEP | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MELEE_SPIRIT | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MELEE_SUCCUBUS | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MOVE_ANIMAL | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MOVE_CONFUSED | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MOVE_FLUTTER | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MOVE_FOLLOW | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MOVE_LEASH | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MOVE_NORMAL | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MOVE_RANDOM | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MOVE_SCAREDY | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MOVE_SMART | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MOVE_SPIRIT | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_MOVE_TELEPORT | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_NO_OP | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_ACID_CLOUD | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_AGGRAVATE | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_ANGEL | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_AV | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_BLACKOUT | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_BOG | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_COURT | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_DE | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_DEATH | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_DEMON | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_DEMONLOVER | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_DRAGONLORD | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_EATER | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_ESCAPE | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_EXPLODE | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_FLUTTER | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_GHOST | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_HUGE | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_ILLUSION | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_LAIR | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_LAWBRINGER | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_LW | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_MASTER | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_MB | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_MERCHANT | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_MIRROR | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_MP | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_POISON_CLOUD | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_PRIME | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_RAISE | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_SEDUCTOR | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_SERVANT | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_SPELL | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_SUMMON | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_SURPRISE | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_SWARM | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_THIEF | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_WERE | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_WHIRL | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_WHISTLEBLOWER | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_SP_WYRM | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_STRIKE_BLIND | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_STRIKE_FBALL | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_STRIKE_FBOLT | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_STRIKE_LBALL | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_STRIKE_MASTER | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_STRIKE_MISSILE | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_STRIKE_SNOWBALL | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_STRIKE_SONIC | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_ANIMAL | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_ARCHMAGE | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_ASSASSIN | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_BEG | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_BURBLE | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_DEMONLOVER | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_DRUID | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_EF | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_EVIL | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_GF | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_GHOST | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_GREEDY | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_GUARD | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_HINT | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_HORSE | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_HUNGRY | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_HYENA | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_IM | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_LB | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_MAHARAJA | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_MAN | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_MERCHANT | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_MIMSY | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_MORGON | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_MP | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_NINJA | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_PARROT | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_PRIME | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_ROBOT | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_SCREAM | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_SEDUCTOR | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_SERVANT | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_SILENT | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_SLITHY | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_STUPID | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_THIEF | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.M_TALK_TITTER | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.NUMARMOR | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.NUMARTIFACTS | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.NUMBOOTS | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.NUMCARDS | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.NUMCITYSITES | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.NUMCLOAKS | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.NUMFOODS | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.NUMIMMUNITIES | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.NUMMONSTERS | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.define.NUMOPTIONS | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.NUMPOTIONS | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.NUMRANKS | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.NUMRINGS | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.NUMROOMNAMES | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.NUMSCROLLS | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.NUMSHIELDS | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.NUMSPELLS | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.NUMSTATI | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.NUMSTATS | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.NUMSTICKS | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.NUMTFOPTIONS | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.NUMTHINGS | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.NUMTRAPS | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.define.NUMWEAPONS | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.NUM_SAFE_TRAPS | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.O_ARMOR | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.O_BELT1 | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.O_BELT2 | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.O_BELT3 | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.O_BOOTS | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.O_CLOAK | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.O_LEFT_SHOULDER | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.O_READY_HAND | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.O_RIGHT_SHOULDER | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.O_RING1 | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.O_RING2 | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.O_RING3 | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.O_RING4 | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.O_SHIELD | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.O_UP_IN_AIR | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.O_WEAPON_HAND | Equivalent | Main | core_system_contract | covered by equivalent command/data behavior |
| legacy.define.S_ACCURACY | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_ALERT | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_APPORT | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_BLESS | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_BREATHE | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_CLAIRVOYANCE | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_CURE | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_DESECRATE | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_DISINTEGRATE | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_DISPEL | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_DISRUPT | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_DRAIN | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_ENCHANT | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_FEAR | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_FIREBOLT | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_HASTE | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_HEAL | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_HELLFIRE | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_HERO | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_IDENTIFY | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_INVISIBLE | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_KNOWLEDGE | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_LBALL | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_LEVITATE | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_MISSILE | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_MON_DET | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_NUTRITION | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_OBJ_DET | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_POLYMORPH | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_REGENERATE | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_RESTORE | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_RETURN | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_RITUAL | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_SANCTIFY | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_SANCTUARY | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_SHADOWFORM | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_SLEEP | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_SUMMON | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_TELEPORT | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_TRUESIGHT | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_WARP | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.define.S_WISH | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.definition.c_statusp | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.definition.find_and_remove_item | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.definition.loc_statusp | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.definition.m_statusp | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.definition.random_range | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.definition.true_item_value | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.abortshadowform | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.abyss_file | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.accuracy | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.acid_cloud | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.acquire | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.activate | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.add_to_pack | Equivalent | Main | inventory_and_equipment | covered by equivalent command/data behavior |
| legacy.function.adeptfile | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.aggravate | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.alert | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.alert_guards | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.amnesia | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.annihilate | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.answer_prayer | Equivalent | Secondary | quests_and_progression | covered by equivalent command/data behavior |
| legacy.function.apport | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.assign_city_function | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.assign_village_function | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.augment | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.baditem | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.badobject | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.ball | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.bank_create_account | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.bank_create_card | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.bank_index_number | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.bank_index_password | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.bank_init | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.bank_random_account_number | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.bash_item | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.bash_location | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.blankoutspot | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.bless | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.blotspot | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.bolt | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.break_weapon | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.breathe | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.bufferappend | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.buffercycle | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.bufferprint | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.build_room | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.build_square_room | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.buyfromstock | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.calc_melee | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.calc_points | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.calc_weight | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.calcmana | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.callitem | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.cast_spell | Exact | Main | magic_and_spells | direct symbol match |
| legacy.function.cavern_level | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.change_environment | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.change_level | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.change_to_game_perms | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.change_to_user_perms | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.charid | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.check_memory | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.check_sacrilege | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.checkclear | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.checkhigh | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.cinema_blank | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.cinema_confirm | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.cinema_getnum_line | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.cinema_hide | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.cinema_interact | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.cinema_interact_line | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.cinema_print_line | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.cinema_scene | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.cinema_ynq | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.cinema_ynq_line | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.city_move | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.cityguidefile | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.clairvoyance | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.cleanse | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.clear_if_necessary | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.clear_level | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.clear_screen | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.clearmsg | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.clearmsg1 | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.clearmsg3 | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.closedoor | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.colour_off | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.colour_on | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.combat_help | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.commanderror | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.commandlist | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.competence_check | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.comwinprint | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.confirmation | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.conform_lost_object | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.conform_lost_objects | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.conform_unused_object | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.copy_obj | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.copyfile | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.corridor_crawl | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.countrysearch | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.create_object | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.cryptkey | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.cure | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.cureforpay | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.cursed | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.damage_item | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.dataprint | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.day | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.deathprint | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.default_maneuvers | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.deflection | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.describe_player | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.destroy_order | Equivalent | Secondary | quests_and_progression | covered by equivalent command/data behavior |
| legacy.function.detach_money | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.determine_npc_behavior | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.disarm | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.disease | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.disintegrate | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.dismount_steed | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.dispel | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.displace | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.display_bigwin | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.display_death | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.display_inventory_slot | Equivalent | Main | inventory_and_equipment | covered by equivalent command/data behavior |
| legacy.function.display_option_slot | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.display_options | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.display_pack | Equivalent | Main | inventory_and_equipment | covered by equivalent command/data behavior |
| legacy.function.display_possessions | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.display_quit | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.display_stat_slot | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.display_stats | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.display_win | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.displaycryptfile | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.displayfile | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.dispose_lost_objects | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.disrupt | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.distance | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.do_inventory_control | Equivalent | Main | inventory_and_equipment | covered by equivalent command/data behavior |
| legacy.function.do_los | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.do_object_los | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.dobackspace | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.dodrawspot | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.downstairs | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.drain | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.drain_life | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.draw_explosion | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.drawmonsters | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.drawomega | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.drawplayer | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.drawscreen | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.drawspot | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.drawvision | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.drop | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.drop_at | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.drop_from_slot | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.drop_money | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.drop_weapon | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.eat | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.editstats | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.enchant | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.endgraf | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.enter_site | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.erase_level | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.erase_monster | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.examine | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.expval | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.extendlog | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.fball | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.fbolt | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.fight_monster | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.filecheck | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.find_and_remove_item | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.find_item | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.find_stairs | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.findlevel | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.findspace | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.fire | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.fix_phantom | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.fixnpc | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.fixpack | Equivalent | Main | inventory_and_equipment | covered by equivalent command/data behavior |
| legacy.function.floor_inv | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.flux | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.foodcheck | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.free_dungeon | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.free_level | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.free_mons_and_objs | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.free_obj | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.free_objlist | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.frobgamestatus | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.gain_experience | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.gain_item | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.gain_level | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.game_restore | Equivalent | Main | save_and_session | covered by equivalent command/data behavior |
| legacy.function.genrand | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.get_inventory_slot | Equivalent | Main | inventory_and_equipment | covered by equivalent command/data behavior |
| legacy.function.get_item_number | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.get_money | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.get_to_pack | Equivalent | Main | inventory_and_equipment | covered by equivalent command/data behavior |
| legacy.function.getdir | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.getitem | Equivalent | Main | inventory_and_equipment | covered by equivalent command/data behavior |
| legacy.function.getitem_prompt | Equivalent | Main | inventory_and_equipment | covered by equivalent command/data behavior |
| legacy.function.getlocation | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.getnumber | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.getspell | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.getspot | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.give | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.give_money | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.givemonster | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.goberserk | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.gymtrain | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.haste | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.heal | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.healforpay | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.hellfire | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.help | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.hero | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.hide | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.hide_line | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.hint | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.hitp | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.hostilemonstersnear | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.hour | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.hourly_check | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.hp_req_print | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.hp_req_test | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.hunt | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.i_accuracy | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_acquire | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_alert | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_antioch | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_apport | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_augment | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_azoth | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_bless | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_breathing | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_chaos | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_charge | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_clairvoyance | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_corpse | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_crystal | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_cure | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_death | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_defend | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_deflect | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_demonblade | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_desecrate | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_disintegrate | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_dispel | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_displace | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_disrupt | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_enchant | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_enchantment | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_fear | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_fear_resist | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_fireball | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_firebolt | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_flux | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_food | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_heal | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_helm | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_hero | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_hide | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_holding | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_id | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_illuminate | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_immune | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_invisible | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_jane_t | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_juggernaut | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_key | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_knowledge | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_kolwynia | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_law | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_lball | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_lbolt | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_lembas | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_levitate | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_life | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_lightsabre | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_mace_disrupt | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_missile | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_mondet | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_neutralize_poison | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_no_op | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_normal_armor | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_normal_shield | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_normal_weapon | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_nothing | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_objdet | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_orbair | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_orbdead | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_orbearth | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_orbfire | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_orbmastery | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_orbwater | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_pepper_food | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_perm_accuracy | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_perm_agility | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_perm_breathing | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_perm_burden | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_perm_deflect | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_perm_displace | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_perm_energy_resist | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_perm_fear_resist | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_perm_fire_resist | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_perm_gaze_immune | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_perm_hero | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_perm_illuminate | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_perm_invisible | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_perm_knowledge | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_perm_levitate | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_perm_negimmune | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_perm_poison_resist | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_perm_protection | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_perm_regenerate | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_perm_speed | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_perm_strength | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_perm_truesight | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_pick | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_planes | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_poison_food | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_polymorph | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_pow | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_raise_portcullis | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_regenerate | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_restore | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_sceptre | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_serenity | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_sleep_other | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_sleep_self | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_snowball | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_speed | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_spells | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_stargem | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_stim | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_summon | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_symbol | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_teleport | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_trap | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_truesight | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_victrix | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_warp | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.i_wish | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.icebolt | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.identify | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.illuminate | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.inbounds | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.increase_priest_rank | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.index_to_key | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.indoors_random_event | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.inflict_fear | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.init_perms | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.init_world | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.initdirs | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.initgraf | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.inititem | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.initplayer | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.initrand | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.initspells | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.initstats | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.install_specials | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.install_traps | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.inv_help | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.inventory_control | Equivalent | Main | inventory_and_equipment | covered by equivalent command/data behavior |
| legacy.function.inversedir | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.invisible | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.item_inventory | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.item_use | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.item_useable | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.item_value | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.itemblessing | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.itemcharge | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.itemlist | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.itemplus | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.key_to_index | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.kill_all_levels | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.kill_levels | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.knowledge | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.l_abyss | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_adept | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_air_station | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_alchemist | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_altar | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_arena | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_arena_exit | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_armorer | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_balancestone | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_bank | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_brothel | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_cartographer | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_casino | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_castle | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_chaos | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_chaostone | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_charity | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_circle_library | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_club | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_college | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_commandant | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_condo | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_countryside | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_crap | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_diner | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_dpw | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_drop_every_portcullis | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_earth_station | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_enter_circle | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_enter_court | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_escalator | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_fire | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_fire_station | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_gym | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_healer | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_hedge | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_house | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_house_exit | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_hovel | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_lava | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_lawstone | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_library | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_lift | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_magic_pool | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_mansion | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_merc_guild | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_mindstone | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_monastery | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_no_op | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_oracle | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_order | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_pawn_shop | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_portcullis_trap | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_raise_portcullis | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_rubble | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_sacrificestone | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_safe | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_sorcerors | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_statue_wake | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_tactical_exit | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_tavern | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_temple_warning | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_thieves_guild | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_throne | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_tome1 | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_tome2 | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_tourist | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_trap_abyss | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_trap_acid | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_trap_blade | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_trap_dart | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_trap_disintegrate | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_trap_door | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_trap_fire | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_trap_manadrain | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_trap_pit | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_trap_siren | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_trap_sleepgas | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_trap_snare | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_trap_teleport | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_trifid | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_vault | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_voice1 | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_voice2 | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_voice3 | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_void | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_void_station | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_voidstone | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_water | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_water_station | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.l_whirlwind | Equivalent | Secondary | locations_and_sites | covered by equivalent command/data behavior |
| legacy.function.lball | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.lbolt | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.learnclericalspells | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.learnspell | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.level_drain | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.level_return | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.levelrefresh | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.levitate | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.lgetc | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.lightspot | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.litroom | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.load_abyss | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.load_arena | Equivalent | Secondary | quests_and_progression | covered by equivalent command/data behavior |
| legacy.function.load_circle | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.load_city | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.load_country | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.load_court | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.load_dlair | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.load_house | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.load_misle | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.load_speak | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.load_temple | Equivalent | Secondary | quests_and_progression | covered by equivalent command/data behavior |
| legacy.function.load_village | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.lock_score_file | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.locprint | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.los_p | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.lose_all_items | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.m_abyss | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_aggravate | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_altar | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_blind_strike | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_confused_move | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_create | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_damage | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_death | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_dropstuff | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_fire | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_fireball | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_firebolt | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_flutter_move | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_follow_move | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_hit | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_huge_sounds | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_illusion | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_lava | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_lball | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_move_animal | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_move_leash | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_movefunction | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_nbolt | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_no_op | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_normal_move | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_pickup | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_pulse | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_random_move | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_remove | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_scaredy_move | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_simple_move | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_smart_move | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_snowball | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_acid_cloud | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_angel | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_av | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_blackout | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_bogthing | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_court | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_demon | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_demonlover | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_dragonlord | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_eater | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_escape | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_explode | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_ghost | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_lair | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_lw | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_mb | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_merchant | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_mirror | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_mp | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_ng | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_poison_cloud | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_prime | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_raise | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_seductor | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_servant | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_spell | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_surprise | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_swarm | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_were | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_sp_whistleblower | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_spirit_move | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_strike_sonic | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_summon | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_animal | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_archmage | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_assassin | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_beg | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_burble | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_demonlover | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_druid | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_ef | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_evil | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_gf | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_greedy | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_guard | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_hint | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_horse | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_hungry | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_hyena | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_im | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_maharaja | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_man | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_merchant | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_mimsy | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_mp | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_ninja | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_parrot | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_prime | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_robot | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_scream | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_seductor | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_servant | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_silent | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_slithy | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_stupid | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_thief | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_talk_titter | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_teleport | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_thief_f | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_trap_abyss | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_trap_acid | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_trap_blade | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_trap_dart | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_trap_disintegrate | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_trap_door | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_trap_fire | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_trap_manadrain | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_trap_pit | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_trap_sleepgas | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_trap_snare | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_trap_teleport | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_unblocked | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_vanish | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.m_water | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.maddch | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.magic | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.magic_resist | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.main | Exact | Rest | misc_runtime | direct symbol match |
| legacy.function.make_archmage | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_armor | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_artifact | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_boots | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_cash | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_cloak | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_corpse | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_country_monsters | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_country_screen | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.make_creature | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_food | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_food_bin | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_forest | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_general_map | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.make_guard | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_high_priest | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_hiscore_npc | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_horse | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_house_npc | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_hp | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_jungle | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_justiciar | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_log_npc | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_major_undead | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_mansion_npc | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_merchant | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_minor_undead | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_mountains | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_plains | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_potion | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_prime | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_ring | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_river | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_road | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_scroll | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_sheep | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_shield | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_site_monster | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_site_treasure | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_specific_treasure | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_stairs | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_stick | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_swamp | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_thing | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.make_weapon | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.makedoor | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.manastorm | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.maneuvers | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.map_close | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.map_getDepth | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.map_getLength | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.map_getSiteChar | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.map_getWidth | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.map_setLevel | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.maze_corridor | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.maze_level | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.mcigetc | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.menuaddch | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.menuclear | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.menugetc | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.menulongprint | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.menunumprint | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.menuprint | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.menuspellprint | Equivalent | Tertiary | ui_and_logging | covered by equivalent command/data behavior |
| legacy.function.merge_item | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.mgetc | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.minute_status_check | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.mlongprint | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.mnumprint | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.mondet | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.monster_action | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.monster_hit | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.monster_melee | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.monster_move | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.monster_special | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.monster_strike | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.monster_talk | Equivalent | Secondary | monster_ai_and_behaviors | covered by equivalent command/data behavior |
| legacy.function.monsterlist | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.moon_check | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.morewait | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.move_slot | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.movecursor | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.movemonster | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.movepincountry | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.moveplayer | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.mprint | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.msdos_changelevel | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.msdos_init | Equivalent | Secondary | world_and_generation | covered by equivalent command/data behavior |
| legacy.function.nap | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.nbolt | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.nighttime | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.nprint1 | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.nprint2 | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.nprint3 | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.objdet | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.objequal | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.offscreen | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.ok_outdated | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.ok_to_free | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.omega_title | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.omegan_character_stats | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.omshowcursor | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.opendoor | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.orbcheck | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.outdoors_random_event | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.p_country_moveable | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.p_country_process | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.p_damage | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.p_death | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.p_drop_at | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.p_drown | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.p_fumble | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.p_hit | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.p_immune | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.p_moveable | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.p_movefunction | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.p_poison | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.p_process | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.p_teleport | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.p_win | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.pacify_guards | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.pack_extra_items | Equivalent | Main | inventory_and_equipment | covered by equivalent command/data behavior |
| legacy.function.parsecitysite | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.parsenum | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.peruse | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.phaseprint | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.pickpocket | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.pickup | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.pickup_at | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.player_dump | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.player_hit | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.player_miss | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.player_on_sanctuary | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.plotchar | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.plotmon | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.plotspot | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.polymorph | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.populate_level | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.print1 | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.print2 | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.print3 | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.put_to_pack | Equivalent | Main | inventory_and_equipment | covered by equivalent command/data behavior |
| legacy.function.putspot | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.quaff | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.quit | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.random_item | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.random_loc | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.random_range | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.random_temple_site | Equivalent | Secondary | quests_and_progression | covered by equivalent command/data behavior |
| legacy.function.recover_stat | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.redraw | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.regenerate | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.rename_player | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.repair_jail | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.rest | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.restore_country | Equivalent | Main | save_and_session | covered by equivalent command/data behavior |
| legacy.function.restore_game | Equivalent | Main | save_and_session | covered by equivalent command/data behavior |
| legacy.function.restore_hiscore_npc | Equivalent | Main | save_and_session | covered by equivalent command/data behavior |
| legacy.function.restore_item | Equivalent | Main | save_and_session | covered by equivalent command/data behavior |
| legacy.function.restore_itemlist | Equivalent | Main | save_and_session | covered by equivalent command/data behavior |
| legacy.function.restore_level | Equivalent | Main | save_and_session | covered by equivalent command/data behavior |
| legacy.function.restore_monsters | Equivalent | Main | save_and_session | covered by equivalent command/data behavior |
| legacy.function.restore_player | Equivalent | Main | save_and_session | covered by equivalent command/data behavior |
| legacy.function.resurrect_guards | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.room_corridor | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.room_level | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.roomcheck | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.s_accuracy | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_alert | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_apport | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_bless | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_breathe | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_clairvoyance | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_cure | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_desecrate | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_disintegrate | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_dispel | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_disrupt | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_drain | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_enchant | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_fear | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_firebolt | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_haste | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_heal | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_hellfire | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_hero | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_identify | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_invisible | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_knowledge | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_lball | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_levitate | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_missile | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_mondet | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_objdet | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_polymorph | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_regenerate | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_restore | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_return | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_ritual | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_sanctify | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_sanctuary | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_shadowform | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_sleep | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_summon | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_teleport | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_truesight | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_warp | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.s_wish | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.sanctify | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.sanctuary | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.save | Equivalent | Main | save_and_session | covered by equivalent command/data behavior |
| legacy.function.save_country | Equivalent | Main | save_and_session | covered by equivalent command/data behavior |
| legacy.function.save_game | Equivalent | Main | save_and_session | covered by equivalent command/data behavior |
| legacy.function.save_hiscore_npc | Equivalent | Main | save_and_session | covered by equivalent command/data behavior |
| legacy.function.save_item | Equivalent | Main | save_and_session | covered by equivalent command/data behavior |
| legacy.function.save_itemlist | Equivalent | Main | save_and_session | covered by equivalent command/data behavior |
| legacy.function.save_level | Equivalent | Main | save_and_session | covered by equivalent command/data behavior |
| legacy.function.save_monsters | Equivalent | Main | save_and_session | covered by equivalent command/data behavior |
| legacy.function.save_omegarc | Equivalent | Main | save_and_session | covered by equivalent command/data behavior |
| legacy.function.save_player | Equivalent | Main | save_and_session | covered by equivalent command/data behavior |
| legacy.function.screencheck | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.screenmodx | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.screenmody | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.search | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.searchat | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.send_to_jail | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.setPlayerXY | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.setchargestr | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.setlastxy | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.setnumstr | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.setoptions | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.setplustr | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.setspot | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.sewer_corridor | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.sewer_level | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.sgenrand | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.shadowform | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.show_inventory_slot | Equivalent | Main | inventory_and_equipment | covered by equivalent command/data behavior |
| legacy.function.show_license | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.show_screen | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.showflags | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.showhour | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.showknownsites | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.showknownspells | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.showmenu | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.showminute | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.showmotd | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.showroom | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.showscores | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.shuffle | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.signalexit | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.signalsave | Equivalent | Main | save_and_session | covered by equivalent command/data behavior |
| legacy.function.sleep_monster | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.sleep_player | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.slottable | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.snowball | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.special_village_site | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.spellparse | Equivalent | Main | magic_and_spells | covered by equivalent command/data behavior |
| legacy.function.split_item | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.spreadroomdark | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.spreadroomlight | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.stationcheck | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.statmod | Exact | Rest | misc_runtime | direct symbol match |
| legacy.function.statue_random | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.stillonblock | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.stock_level | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.stolen_item | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.stonecheck | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.straggle_corridor | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.strategic_teleport | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.strengthen_death | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.strmem | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.strprefix | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.summon | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.surrender | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.switch_to_slot | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.tacmonster | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.tacoptions | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.tacplayer | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.take_from_pack | Equivalent | Main | inventory_and_equipment | covered by equivalent command/data behavior |
| legacy.function.talk | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.tenminute_check | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.tenminute_status_check | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.terrain_check | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.test_file_access | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.theologyfile | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.threaten | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.time_clock | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.timeprint | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.toggle_item_use | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.top_inventory_control | Equivalent | Main | inventory_and_equipment | covered by equivalent command/data behavior |
| legacy.function.torch_check | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.transcribe_monster_actions | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.true_item_value | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.truesight | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.tunnel | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.tunnelcheck | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.twohandedp | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.unblocked | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.unlock_score_file | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.upstairs | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.user_character_stats | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.user_intro | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.usleep | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.vault | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.version | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.view_los_p | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.view_unblocked | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.wake_statue | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.wandercheck | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.warp | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.weapon_acidwhip | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.weapon_arrow | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.weapon_bare_hands | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.weapon_bolt | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.weapon_defend | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.weapon_demonblade | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.weapon_desecrate | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.weapon_firestar | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.weapon_lightsabre | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.weapon_mace_disrupt | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.weapon_normal_hit | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.weapon_scythe | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.weapon_tangle | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.weapon_use | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.weapon_victrix | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.weapon_vorpal | Equivalent | Main | items_and_equipment | covered by equivalent command/data behavior |
| legacy.function.wish | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.wishfile | ExcludedNonGameplay | Rest | misc_runtime | platform/presentation-only surface |
| legacy.function.wizard | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.xredraw | ExcludedNonGameplay | Tertiary | ui_and_logging | platform/presentation-only surface |
| legacy.function.ynq | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.ynq1 | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.ynq2 | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |
| legacy.function.zapwand | Equivalent | Rest | misc_runtime | covered by equivalent command/data behavior |

## Missing/Partial Mechanics

- defect board artifact: `target/mechanics-missing-defect-board.json`
- mapping contract: `docs/migration/MECHANICS_PARITY_MAPPING.yaml`
