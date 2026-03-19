use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use crate::game::{
    core::{
        global_resources::{GlobalFonts, KeyBindings},
        states::{MouseMode, OverallState},
    },
    main_menu_state::MainMenuStatePlugin,
    playing_state::{PlayingStatePlugin, sets::PlayingStateOrdering},
};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .init_resource::<KeyBindings>()
            .init_state::<MouseMode>()
            .init_state::<OverallState>()
            .add_systems(Startup, load_global_fonts)
            .add_systems(OnEnter(MouseMode::Grabbed),
                on_enter_mouse_grabbed
                    .in_set(PlayingStateOrdering::Ui)
            )
            .add_systems(OnExit(MouseMode::Grabbed),
                on_exit_mouse_grabbed
                    .in_set(PlayingStateOrdering::Ui)
            )
            .add_plugins(MainMenuStatePlugin)
            .add_plugins(PlayingStatePlugin);
    }
}

fn load_global_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bluh = asset_server.load("fonts/Oswald-VariableFont_wght.ttf");
    commands.insert_resource(GlobalFonts { sans: bluh });

    commands.set_state(OverallState::MainMenu);
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
