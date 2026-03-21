use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use crate::game::{
    core::{
        global_resources::KeyBindings,
        states::{MouseMode, OverallState},
    },
    gui,
    main_menu_state::MainMenuStatePlugin,
    playing_state::PlayingStatePlugin,
};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .init_resource::<KeyBindings>()
            .init_state::<MouseMode>()
            .init_state::<OverallState>()
            .add_systems(Startup, load_global_resources)
            .add_systems(Update, gui::fonts::apply_gui_fonts)
            .add_systems(OnEnter(MouseMode::Grabbed), on_enter_mouse_grabbed)
            .add_systems(OnExit(MouseMode::Grabbed), on_exit_mouse_grabbed)
            .add_plugins(MainMenuStatePlugin)
            .add_plugins(PlayingStatePlugin);
    }
}

fn load_global_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(gui::fonts::make_global_fonts_resource(asset_server));

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
