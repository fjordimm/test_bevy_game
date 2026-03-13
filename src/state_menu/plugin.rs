use bevy::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_egui::{EguiContext, EguiMultipassSchedule, egui};

use crate::{
    core::states::OverallState,
    state_menu::tags::{StateMenuCameraForEgui, StateMenuEntity},
};

pub struct StateMenuPlugin;

impl Plugin for StateMenuPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::Menu), on_enter_state)
            .add_systems(OnExit(OverallState::Menu), on_exit_state)
            .add_systems(StateMenuCameraForEgui, funny1menu);
    }
}

fn funny1menu(
    mut commands: Commands,
    mut egui_context: Single<&mut EguiContext, With<StateMenuCameraForEgui>>,
) -> Result {
    let ctx = egui_context.get_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("This is the menu????");
        ui.label("Yep it sure is!");
        if ui.button("Play").clicked() {
            debug!("CLIKEDDDDDDDDD");
            commands.set_state(OverallState::Playing);
        }
    });

    Ok(())
}

fn on_enter_state(mut commands: Commands) {
    debug!("state_menu on_enter_state");

    commands.spawn((
        StateMenuEntity,
        StateMenuCameraForEgui,
        EguiMultipassSchedule(StateMenuCameraForEgui.intern()),
        Camera2d::default(),
    ));
}

fn on_exit_state(mut commands: Commands, query: Query<Entity, With<StateMenuEntity>>) {
    debug!("state_menu on_exit_state");

    for entity in &query {
        commands.entity(entity).despawn();
    }
}
