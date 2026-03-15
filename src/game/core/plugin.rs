use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use crate::game::{
    core::{
        global_resources::KeyBindings,
        states::{MouseMode, OverallState},
    },
    playing_state::sets::PlayingStateOrdering,
};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .init_resource::<KeyBindings>()
            .init_state::<OverallState>()
            .init_state::<MouseMode>()
            .add_systems(OnEnter(MouseMode::Grabbed),
                on_enter_mouse_grabbed
                    .in_set(PlayingStateOrdering::Ui)
            )
            .add_systems(OnExit(MouseMode::Grabbed),
                on_exit_mouse_grabbed
                    .in_set(PlayingStateOrdering::Ui)
            );
    }
}

fn on_enter_mouse_grabbed(mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>) {
    cursor_options.grab_mode = CursorGrabMode::Confined;
    cursor_options.visible = false;
}

fn on_exit_mouse_grabbed(
    mut primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    primary_cursor_options.grab_mode = CursorGrabMode::None;
    primary_cursor_options.visible = true;
}
