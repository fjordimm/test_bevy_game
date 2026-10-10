use avian3d::prelude::*;
use bevy::prelude::*;

use crate::game::playing_state::{
    sets::DuringPlaying,
    states::{GameLoadingState, PauseState},
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_message::<TogglePhysics>()
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
            .add_systems(Update,
                handle_toggle_physics
                    .in_set(DuringPlaying::Final)
            )
        ;
    }
}

#[derive(Message)]
struct TogglePhysics(bool);

fn unpause_physics(
    pause_state: Res<State<PauseState>>,
    game_loading_state: Res<State<GameLoadingState>>,
    mut toggle_physics_msg: MessageWriter<TogglePhysics>,
) {
    if let PauseState::Unpaused = pause_state.get()
        && let GameLoadingState::NotLoading = game_loading_state.get()
    {
        debug!("starting physics");
        toggle_physics_msg.write(TogglePhysics(true));
    }
}

fn pause_physics(mut toggle_physics_msg: MessageWriter<TogglePhysics>) {
    debug!("stopping physics");
    toggle_physics_msg.write(TogglePhysics(false));
}

fn handle_toggle_physics(
    mut toggle_physics_msg: MessageReader<TogglePhysics>,
    mut physics: ResMut<Time<Physics>>,
) {
    toggle_physics_msg.read().for_each(|msg| match msg.0 {
        true => {
            physics.unpause();
        }
        false => {
            physics.pause();
        }
    });
}
