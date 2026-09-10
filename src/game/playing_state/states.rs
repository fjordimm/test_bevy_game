use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum PauseState {
    #[default]
    Limbo,
    Unpaused,
    Paused,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameLoadingState {
    #[default]
    NotLoading,
    #[allow(unused)]
    Loading,
}
