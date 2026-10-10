use bevy::prelude::*;

use crate::game::core::states::OverallState;

#[derive(SubStates, Debug, Clone, Eq, PartialEq, Hash, Default)]
#[source(OverallState = OverallState::Playing)]
pub enum PauseState {
    #[default]
    Unpaused,
    Paused,
}

/// Not to be set by anything directly other than game_loading_inhibition
#[derive(SubStates, Debug, Clone, Eq, PartialEq, Hash, Default)]
#[source(OverallState = OverallState::Playing)]
pub enum GameLoadingState {
    #[default]
    NotLoading,
    Loading,
}
