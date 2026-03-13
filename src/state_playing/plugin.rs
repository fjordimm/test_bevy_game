use bevy::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_egui::{EguiContext, EguiMultipassSchedule, egui};

use crate::{
    core::states::OverallState,
    state_playing::{
        player::PlayerPlugin,
        sets::GameSet,
        states::PauseState,
        tags::{StatePlayingCameraForEgui, StatePlayingEntity},
        world::WorldPlugin,
    },
};

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
            .add_systems(OnEnter(OverallState::Playing), on_enter_state)
            .add_systems(OnExit(OverallState::Playing), on_exit_state)
            .add_systems(StatePlayingCameraForEgui, funny1playing)
            .add_plugins(WorldPlugin)
            .add_plugins(PlayerPlugin);
    }
}

fn funny1playing(
    // mut commands: Commands,
    mut egui_context: Single<&mut EguiContext, With<StatePlayingCameraForEgui>>,
) -> Result {
    let ctx = egui_context.get_mut();

    egui::Window::new("Sup").show(ctx, |ui| {
        ui.label("ahahahahah");
    });

    Ok(())
}

fn on_enter_state(mut commands: Commands) {
    debug!("state_playing on_enter_state");

    commands.spawn((
        StatePlayingEntity,
        StatePlayingCameraForEgui,
        EguiMultipassSchedule(StatePlayingCameraForEgui.intern()),
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(-Vec3::Z, Vec3::Y),
    ));
}

fn on_exit_state(mut commands: Commands, query: Query<Entity, With<StatePlayingEntity>>) {
    debug!("state_playing on_exit_state");

    for entity in &query {
        commands.entity(entity).despawn();
    }
}
