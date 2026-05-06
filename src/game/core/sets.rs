use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GlobalStartupOrdering {
    CoreUseOnly,
    GuiSpawning,
}

pub const GLOBAL_STARTUP_ORDERING_ORDER: (GlobalStartupOrdering, GlobalStartupOrdering) = (
    GlobalStartupOrdering::CoreUseOnly,
    GlobalStartupOrdering::GuiSpawning,
);
