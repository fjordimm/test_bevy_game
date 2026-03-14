use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use crate::game::{
    core::states::OverallState,
    playing_state::{player::resources::PlayerMovementSettings, states::PauseState},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .init_resource::<PlayerMovementSettings>()
            .add_systems(OnEnter(OverallState::Playing), on_enter)
            .add_systems(OnExit(OverallState::Playing), on_exit)
            .add_systems(OnEnter(PauseState::Unpaused), on_enter_unpaused.run_if(in_state(OverallState::Playing)))
            .add_systems(OnExit(PauseState::Unpaused), on_exit_unpaused.run_if(in_state(OverallState::Playing)));
    }
}

fn on_enter() {}

fn on_exit() {}

fn on_enter_unpaused(mut primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>) {
    primary_cursor_options.grab_mode = CursorGrabMode::Confined;
    primary_cursor_options.visible = false;
}

fn on_exit_unpaused(mut primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>) {
    primary_cursor_options.grab_mode = CursorGrabMode::None;
    primary_cursor_options.visible = true;
}
