use bevy::prelude::*;

use crate::{core::states::OverallState, state_playing::{
    player::PlayerPlugin, sets::GameSet, states::PauseState, tags::StatePlayingEntity, world::WorldPlugin
}};

pub struct StatePlayingPlugin;

impl Plugin for StatePlayingPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .configure_sets(
                Update, (
                    GameSet::Input,
                    GameSet::Movement,
                    GameSet::Physics,
                    GameSet::World,
                ).chain(),
            )
            .init_state::<PauseState>()
            .add_systems(OnEnter(OverallState::Playing), on_enter)
            .add_systems(OnExit(OverallState::Playing), on_exit)
            .add_plugins(WorldPlugin)
            .add_plugins(PlayerPlugin);
    }
}

fn on_enter(mut commands: Commands) {
    debug!("state_playing on_enter");
    
    commands.spawn((
        StatePlayingEntity,
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(-Vec3::Z, Vec3::Y),
    ));
}

fn on_exit(mut commands: Commands, query: Query<Entity, With<StatePlayingEntity>>) {
    debug!("state_playing on_exit");

    for entity in &query {
        commands.entity(entity).despawn();
    }
}
