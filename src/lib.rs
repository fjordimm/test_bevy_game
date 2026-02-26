use bevy::{log::LogPlugin, prelude::*};

pub mod core;
pub mod world;

pub fn build_app() -> App {
    let mut app = App::new();

    #[cfg(debug_assertions)]
    app.add_plugins(DefaultPlugins
        .set(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,test_bevy_game=trace".into(),
            level: bevy::log::Level::INFO,
            ..default()
        })
    );

    #[cfg(not(debug_assertions))]
    app.add_plugins(DefaultPlugins);

    #[rustfmt::skip]
    app
        .add_plugins(core::CorePlugin)
        // .add_plugins(player::PlayerPlugin)
        // .add_plugins(physics::PhysicsPlugin)
        .add_plugins(world::WorldPlugin);

    app
}
