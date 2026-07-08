use bevy::prelude::*;

use crate::game::{
    core::states::{MouseMode, OverallState},
    main_menu_state::main_menu_gui::plugin::MainMenuGuiPlugin,
    util::alrms,
};

pub struct MainMenuStatePlugin;

impl Plugin for MainMenuStatePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::MainMenu), on_enter)
            .add_systems(OnExit(OverallState::MainMenu), despawn_camera)
            .add_plugins(MainMenuGuiPlugin)
        ;
    }
}

#[derive(Component)]
struct MainMenuStateCamera;

fn on_enter(mut commands: Commands, mut next_mouse_mode: ResMut<NextState<MouseMode>>) {
    next_mouse_mode.set(MouseMode::Free);

    commands.spawn((MainMenuStateCamera, Camera2d::default()));
}

fn despawn_camera(
    mut commands: Commands,
    camera_q: Option<Single<Entity, With<MainMenuStateCamera>>>,
) {
    if let Some(camera) = alrms!(camera_q) {
        commands.entity(*camera).despawn();
    }
}
