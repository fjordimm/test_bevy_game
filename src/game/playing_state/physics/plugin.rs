use avian3d::prelude::*;
use bevy::prelude::*;

use crate::game::playing_state::states::{GameLoadingState, PauseState};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(PauseState::Unpaused),
                unpause_physics
            )
            .add_systems(OnEnter(GameLoadingState::NotLoading),
                unpause_physics
            )
            .add_systems(OnExit(PauseState::Unpaused),
                pause_physics
            )
            .add_systems(OnExit(GameLoadingState::NotLoading),
                pause_physics
            )
        ;
    }
}

fn unpause_physics(
    pause_state: Res<State<PauseState>>,
    game_loading_state: Res<State<GameLoadingState>>,
    mut physics: ResMut<Time<Physics>>,
) {
    if let PauseState::Unpaused = pause_state.get()
        && let GameLoadingState::NotLoading = game_loading_state.get()
    {
        physics.unpause();
    }
}

fn pause_physics(mut physics: ResMut<Time<Physics>>) {
    physics.pause();
}
