use bevy::prelude::*;

use crate::game::{
    core::states::{MouseMode, OverallState},
    main_menu_state::tags::MainMenuStateEntity,
};

pub struct MainMenuStatePlugin;

impl Plugin for MainMenuStatePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::MainMenu), on_enter)
            .add_systems(OnExit(OverallState::MainMenu), remove_all_relevant_entities);
    }
}

fn on_enter(mut commands: Commands, mut next_mouse_mode: ResMut<NextState<MouseMode>>) {
    next_mouse_mode.set(MouseMode::Free);

    commands.spawn((MainMenuStateEntity, Camera2d::default()));
}

fn remove_all_relevant_entities(
    mut commands: Commands,
    all_entities_q: Query<Entity, With<MainMenuStateEntity>>,
) {
    all_entities_q.iter().for_each(|entity| {
        commands.entity(entity).despawn();
    });
}
