use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};
use bevy_prng::{ChaCha8Rng, WyRand};
use bevy_rand::plugin::EntropyPlugin;

use crate::game::{
    core::{
        global_resources::KeyBindings,
        states::{MouseMode, OverallState},
    },
    debug_menu::DebugMenuPlugin,
    gui::{self, gui_root_template, plugin::GuiPlugin},
    main_menu_state::MainMenuStatePlugin,
    playing_state::PlayingStatePlugin,
};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            // External Plugins
            .add_plugins(FrameTimeDiagnosticsPlugin::new(120))
            .add_plugins(EntropyPlugin::<WyRand>::default())
            // Relevant Stuff
            .init_resource::<KeyBindings>()
            .init_state::<MouseMode>()
            .init_state::<OverallState>()
            .add_systems(Startup, global_startup)
            .add_systems(Update, gui::fonts::apply_gui_fonts)
            .add_systems(OnEnter(MouseMode::Grabbed), on_enter_mouse_grabbed)
            .add_systems(OnExit(MouseMode::Grabbed), on_exit_mouse_grabbed)
            .add_plugins(GuiPlugin)
            .add_plugins(DebugMenuPlugin)
            .add_plugins(MainMenuStatePlugin)
            .add_plugins(PlayingStatePlugin);
    }
}

#[derive(Resource)]
pub struct GlobalGuiRoot(pub Entity);

fn global_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(gui::fonts::make_global_fonts_resource(asset_server));

    let gui_root = commands.spawn(gui_root_template()).id();
    commands.insert_resource(GlobalGuiRoot(gui_root));

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
