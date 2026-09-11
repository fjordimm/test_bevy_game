use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GlobalStartupOrdering {
    BuildBevyAppUseOnly,
    RandomUseOnly,
    GuiUseOnly,
    Regular,
}

pub const GLOBAL_STARTUP_ORDERING_ORDER: (
    GlobalStartupOrdering,
    GlobalStartupOrdering,
    GlobalStartupOrdering,
    GlobalStartupOrdering,
) = (
    GlobalStartupOrdering::BuildBevyAppUseOnly,
    GlobalStartupOrdering::RandomUseOnly,
    GlobalStartupOrdering::GuiUseOnly,
    GlobalStartupOrdering::Regular,
);
