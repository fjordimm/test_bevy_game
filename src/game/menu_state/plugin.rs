use bevy::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_egui::{EguiContext, EguiMultipassSchedule, egui};

use crate::{
    game::core::states::OverallState,
    game::menu_state::tags::{MenuStateCameraForEgui, MenuStateEntity},
};

pub struct MenuStatePlugin;

impl Plugin for MenuStatePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::Menu), on_enter_state)
            .add_systems(OnExit(OverallState::Menu), on_exit_state)
            .add_systems(MenuStateCameraForEgui, funny1menu);
    }
}

fn funny1menu(
    mut commands: Commands,
    mut egui_context: Single<&mut EguiContext, With<MenuStateCameraForEgui>>,
) -> Result {
    let ctx = egui_context.get_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button("Play").clicked() {
            commands.set_state(OverallState::Playing);
        }
    });

    Ok(())
}

fn on_enter_state(mut commands: Commands) {
    debug!("menu_state on_enter_state");

    commands.spawn((
        MenuStateEntity,
        MenuStateCameraForEgui,
        EguiMultipassSchedule(MenuStateCameraForEgui.intern()),
        Camera2d::default(),
    ));
}

fn on_exit_state(mut commands: Commands, query: Query<Entity, With<MenuStateEntity>>) {
    debug!("menu_state on_exit_state");

    for entity in &query {
        commands.entity(entity).despawn();
    }
}
