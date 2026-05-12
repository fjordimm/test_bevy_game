use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GlobalStartupOrdering {
    RandomnessUseOnly,
    CoreUseOnly,
    GuiSpawning,
}

pub const GLOBAL_STARTUP_ORDERING_ORDER: (
    GlobalStartupOrdering,
    GlobalStartupOrdering,
    GlobalStartupOrdering,
) = (
    GlobalStartupOrdering::RandomnessUseOnly,
    GlobalStartupOrdering::CoreUseOnly,
    GlobalStartupOrdering::GuiSpawning,
);
