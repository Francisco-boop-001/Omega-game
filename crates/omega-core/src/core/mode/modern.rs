use super::contracts::{
    CombatPolicy, EconomyPolicy, ItemPolicy, MagicPolicy, ModePolicySet, QuestPolicy,
    ServicePolicy, TraversalPolicy, VictoryPolicy,
};

struct ModernPolicy;

impl CombatPolicy for ModernPolicy {}
impl MagicPolicy for ModernPolicy {}
impl ItemPolicy for ModernPolicy {}
impl ServicePolicy for ModernPolicy {}
impl QuestPolicy for ModernPolicy {}
impl TraversalPolicy for ModernPolicy {}
impl VictoryPolicy for ModernPolicy {}
impl EconomyPolicy for ModernPolicy {}

static MODERN_POLICY: ModernPolicy = ModernPolicy;
static MODERN_POLICY_SET: ModePolicySet = ModePolicySet {
    combat: &MODERN_POLICY,
    magic: &MODERN_POLICY,
    item: &MODERN_POLICY,
    service: &MODERN_POLICY,
    quest: &MODERN_POLICY,
    traversal: &MODERN_POLICY,
    victory: &MODERN_POLICY,
    economy: &MODERN_POLICY,
};

pub fn policy_set() -> &'static ModePolicySet {
    &MODERN_POLICY_SET
}
