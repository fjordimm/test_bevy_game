use bevy::prelude::*;

use crate::core::states::OverallState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::Playing), on_enter)
            .add_systems(OnExit(OverallState::Playing), on_exit);
    }
}

fn on_enter() {
    debug!("player enter");
}

fn on_exit() {
    debug!("player exit");
}
