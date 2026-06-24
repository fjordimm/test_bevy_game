use bevy::{log::LogPlugin, prelude::*, window::PrimaryWindow};

use crate::game::core::{plugin::CorePlugin, sets::GlobalStartupOrdering};

const DEBUG_BACKGROUND_COLOR: Color = Color::linear_rgb(1., 1., 0.);

pub fn build_bevy_app() -> App {
    let mut app = App::new();

    let default_plugins = DefaultPlugins.build();
    let default_plugins = default_plugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: String::from("Test Bevy Game"),
            present_mode: bevy::window::PresentMode::Immediate,
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
        .add_systems(Startup,
            set_window_maximized
                .in_set(GlobalStartupOrdering::BuildBevyAppUseOnly)
        )
        .add_plugins(CorePlugin)
    ;

    app
}

fn set_window_maximized(mut window_q: Query<&mut Window, With<PrimaryWindow>>) {
    window_q.iter_mut().for_each(|mut window| {
        window.set_maximized(true);
    });
}
