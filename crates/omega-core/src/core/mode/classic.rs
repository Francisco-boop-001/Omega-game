use super::contracts::{
    CombatPolicy, EconomyPolicy, ItemPolicy, MagicPolicy, ModePolicySet, QuestPolicy,
    ServicePolicy, TraversalPolicy, VictoryPolicy,
};

struct ClassicPolicy;

impl CombatPolicy for ClassicPolicy {}
impl MagicPolicy for ClassicPolicy {}
impl ItemPolicy for ClassicPolicy {}
impl ServicePolicy for ClassicPolicy {}
impl QuestPolicy for ClassicPolicy {}
impl TraversalPolicy for ClassicPolicy {}
impl VictoryPolicy for ClassicPolicy {}
impl EconomyPolicy for ClassicPolicy {}

static CLASSIC_POLICY: ClassicPolicy = ClassicPolicy;
static CLASSIC_POLICY_SET: ModePolicySet = ModePolicySet {
    combat: &CLASSIC_POLICY,
    magic: &CLASSIC_POLICY,
    item: &CLASSIC_POLICY,
    service: &CLASSIC_POLICY,
    quest: &CLASSIC_POLICY,
    traversal: &CLASSIC_POLICY,
    victory: &CLASSIC_POLICY,
    economy: &CLASSIC_POLICY,
};

pub fn policy_set() -> &'static ModePolicySet {
    &CLASSIC_POLICY_SET
}
