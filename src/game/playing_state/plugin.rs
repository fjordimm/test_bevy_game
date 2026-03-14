use bevy::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_egui::EguiMultipassSchedule;

use crate::game::{
    core::{global_resources::KeyBindings, states::OverallState},
    playing_state::{
        pause_gui::pause_gui,
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
            .add_systems(OnEnter(OverallState::Playing), on_enter)
            .add_systems(OnExit(OverallState::Playing), on_exit)
            .add_systems(Update, toggle_pause.run_if(in_state(OverallState::Playing)))
            .add_systems(PlayingStateCameraForEgui, pause_gui.run_if(in_state(OverallState::Playing)).run_if(in_state(PauseState::Paused)))
            .add_plugins(WorldPlugin)
            .add_plugins(PlayerPlugin);
    }
}

fn on_enter(mut commands: Commands, mut next_pause_state: ResMut<NextState<PauseState>>) {
    debug!("playing_state on_enter");

    next_pause_state.set(PauseState::Unpaused);

    commands.spawn((
        PlayingStateEntity,
        PlayingStateCameraForEgui,
        EguiMultipassSchedule(PlayingStateCameraForEgui.intern()),
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(-Vec3::Z, Vec3::Y),
    ));
}

fn on_exit(mut commands: Commands, query: Query<Entity, With<PlayingStateEntity>>) {
    debug!("playing_state on_exit");

    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    pause_state: Res<State<PauseState>>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
) {
    if keys.just_pressed(key_bindings.pause) {
        next_pause_state.set(match pause_state.get() {
            PauseState::Unpaused => PauseState::Paused,
            PauseState::Paused => PauseState::Unpaused,
        });
    }
}
