use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum PauseState {
    #[default]
    Limbo,
    Unpaused,
    Paused,
}

/// Not to be set by anything directly other than game_loading_inhibition
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameLoadingState {
    #[default]
    Limbo,
    NotLoading,
    Loading,
}
