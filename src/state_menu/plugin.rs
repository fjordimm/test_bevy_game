use bevy::prelude::*;

use crate::{core::states::OverallState, state_menu::tags::StateMenuEntity};

pub struct StateMenuPlugin;

impl Plugin for StateMenuPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::Menu), on_enter)
            .add_systems(OnExit(OverallState::Menu), on_exit);
    }
}

fn on_enter(mut commands: Commands) {
    debug!("state_menu on_enter");
    
    commands.spawn((StateMenuEntity, Camera2d::default()));
}

fn on_exit(mut commands: Commands, query: Query<Entity, With<StateMenuEntity>>) {
    debug!("state_menu on_exit");

    for entity in &query {
        commands.entity(entity).despawn();
    }
}
