use bevy::prelude::*;

pub mod core;
pub mod world;

pub fn build_app() -> App {
    let mut app = App::new();

    #[rustfmt::skip]
    app
        .add_plugins(DefaultPlugins)
        .add_plugins(core::CorePlugin)
        // .add_plugins(player::PlayerPlugin)
        // .add_plugins(physics::PhysicsPlugin)
        .add_plugins(world::WorldPlugin);

    app
}
