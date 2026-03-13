use bevy::{log::LogPlugin, prelude::*};

pub mod core;
pub mod state_menu;
pub mod state_playing;
pub mod ui;

pub fn build_app() -> App {
    let mut app = App::new();

    #[cfg(debug_assertions)]
    app.add_plugins(DefaultPlugins.set(LogPlugin {
        filter: "info,wgpu_core=warn,wgpu_hal=warn,test_bevy_game=trace".into(),
        level: bevy::log::Level::INFO,
        ..default()
    }));

    #[cfg(not(debug_assertions))]
    app.add_plugins(DefaultPlugins);

    #[rustfmt::skip]
    app
        .insert_resource(ClearColor(Color::linear_rgb(0.7, 0.7, 0.0)))
        .add_plugins(core::CorePlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(state_menu::StateMenuPlugin)
        .add_plugins(state_playing::StatePlayingPlugin);

    app
}
