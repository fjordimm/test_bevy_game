use bevy::prelude::*;

use crate::core::states::OverallState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::Playing), on_enter_state)
            .add_systems(OnExit(OverallState::Playing), on_exit_state);
    }
}

fn on_enter_state() {}

fn on_exit_state() {}
