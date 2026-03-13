use bevy::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_egui::{EguiContext, EguiMultipassSchedule, egui};

use crate::{
    game::core::states::OverallState,
    game::playing_state::{
        player::PlayerPlugin,
        sets::GameSet,
        states::PauseState,
        tags::{PlayingStateCameraForEgui, PlayingStateEntity},
        world::WorldPlugin,
    },
};

pub struct PlayingStatePlugin;

impl Plugin for PlayingStatePlugin {
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
            .add_systems(OnEnter(OverallState::Playing), on_enter_state)
            .add_systems(OnExit(OverallState::Playing), on_exit_state)
            // .add_systems(PlayingStateCameraForEgui, funny1playing)
            .add_plugins(WorldPlugin)
            .add_plugins(PlayerPlugin);
    }
}

fn on_enter_state(mut commands: Commands) {
    debug!("playing_state on_enter_state");

    commands.spawn((
        PlayingStateEntity,
        PlayingStateCameraForEgui,
        EguiMultipassSchedule(PlayingStateCameraForEgui.intern()),
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(-Vec3::Z, Vec3::Y),
    ));
}

fn on_exit_state(mut commands: Commands, query: Query<Entity, With<PlayingStateEntity>>) {
    debug!("playing_state on_exit_state");

    for entity in &query {
        commands.entity(entity).despawn();
    }
}
