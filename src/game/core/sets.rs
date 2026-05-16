use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GlobalStartupOrdering {
    RandomUseOnly,
    CoreUseOnly,
    GuiSpawning,
}

pub const GLOBAL_STARTUP_ORDERING_ORDER: (
    GlobalStartupOrdering,
    GlobalStartupOrdering,
    GlobalStartupOrdering,
) = (
    GlobalStartupOrdering::RandomUseOnly,
    GlobalStartupOrdering::CoreUseOnly,
    GlobalStartupOrdering::GuiSpawning,
);
