use bevy::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_egui::{EguiContext, EguiMultipassSchedule, egui};

use crate::game::{
    core::{global_resources::KeyBindings, states::OverallState},
    playing_state::{
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
            .add_systems(Update, toggle_pause.run_if(in_state(OverallState::Playing)))
            .add_systems(PlayingStateCameraForEgui, pause_gui.run_if(in_state(OverallState::Playing)).run_if(in_state(PauseState::Paused)))
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

fn pause_gui(
    mut commands: Commands,
    mut egui_context: Single<&mut EguiContext, With<PlayingStateCameraForEgui>>,
) -> Result {
    let ctx = egui_context.get_mut();

    egui::Window::new("pausemenubaby").show(ctx, |ui| {
        ui.label("yeppers im paused");
    });
    // egui::CentralPanel::default().show(ctx, |ui| {
    //     if ui.button("Play").clicked() {
    //         commands.set_state(OverallState::Playing);
    //     }
    // });

    Ok(())
}
