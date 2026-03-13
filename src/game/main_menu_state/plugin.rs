use bevy::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_egui::{EguiContext, EguiMultipassSchedule, egui};

use crate::{
    game::core::states::OverallState,
    game::main_menu_state::tags::{MainMenuStateCameraForEgui, MainMenuStateEntity},
};

pub struct MainMenuStatePlugin;

impl Plugin for MainMenuStatePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::MainMenu), on_enter_state)
            .add_systems(OnExit(OverallState::MainMenu), on_exit_state)
            .add_systems(MainMenuStateCameraForEgui, gui);
    }
}

fn on_enter_state(mut commands: Commands) {
    debug!("main_menu_state on_enter_state");

    commands.spawn((
        MainMenuStateEntity,
        MainMenuStateCameraForEgui,
        EguiMultipassSchedule(MainMenuStateCameraForEgui.intern()),
        Camera2d::default(),
    ));
}

fn on_exit_state(mut commands: Commands, query: Query<Entity, With<MainMenuStateEntity>>) {
    debug!("main_menu_state on_exit_state");

    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn gui(
    mut commands: Commands,
    mut egui_context: Single<&mut EguiContext, With<MainMenuStateCameraForEgui>>,
) -> Result {
    let ctx = egui_context.get_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button("Play").clicked() {
            commands.set_state(OverallState::Playing);
        }
    });

    Ok(())
}
