use bevy::{log::LogPlugin, prelude::*, window::PrimaryWindow};

use crate::game::{core, egui_setup, main_menu_state, playing_state};

const DEBUG_BACKGROUND_COLOR: Color = Color::linear_rgb(1.0, 1.0, 0.0);

pub fn build_bevy_app() -> App {
    let mut app = App::new();

    let default_plugins = DefaultPlugins.build();
    let default_plugins = default_plugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: String::from("Test Bevy Game"),
            ..default()
        }),
        ..default()
    });
    #[cfg(debug_assertions)]
    let default_plugins = default_plugins.set(LogPlugin {
        filter: "info,wgpu_core=warn,wgpu_hal=warn,test_bevy_game=trace".into(),
        level: bevy::log::Level::INFO,
        ..default()
    });

    #[rustfmt::skip]
    app
        .add_plugins(default_plugins)
        .insert_resource(ClearColor(DEBUG_BACKGROUND_COLOR))
        .add_systems(Startup, set_window_maximized)
        .add_plugins(core::CorePlugin)
        .add_plugins(egui_setup::EguiSetupPlugin)
        .add_plugins(main_menu_state::MainMenuStatePlugin)
        .add_plugins(playing_state::PlayingStatePlugin);

    app
}

fn set_window_maximized(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    for mut window in windows.iter_mut() {
        window.set_maximized(true);
    }
}
