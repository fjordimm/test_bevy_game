mod plugins;
mod states;

use crate::plugins::*;
use crate::states::GameState;
use crate::states::PauseState;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Test Bevy Game".into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .init_resource::<PauseState>()
        // .add_plugins(AssetsPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        // .add_plugins(UiPlugin)
        // .add_plugins(AudioPlugin)
        // .add_plugins(DebugPlugin);
        .run();
}
