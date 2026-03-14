use bevy::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_egui::{EguiContext, EguiMultipassSchedule, egui};

use crate::game::{
    core::{quit_game, states::OverallState},
    main_menu_state::tags::{MainMenuStateCameraForEgui, MainMenuStateEntity},
};

pub struct MainMenuStatePlugin;

impl Plugin for MainMenuStatePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::MainMenu), on_enter)
            .add_systems(OnExit(OverallState::MainMenu), on_exit)
            .add_systems(MainMenuStateCameraForEgui, main_menu_gui);
    }
}

fn on_enter(mut commands: Commands) {
    debug!("main_menu_state on_enter");

    commands.spawn((
        MainMenuStateEntity,
        MainMenuStateCameraForEgui,
        EguiMultipassSchedule(MainMenuStateCameraForEgui.intern()),
        Camera2d::default(),
    ));
}

fn on_exit(mut commands: Commands, query: Query<Entity, With<MainMenuStateEntity>>) {
    debug!("main_menu_state on_exit");

    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn main_menu_gui(
    mut commands: Commands,
    mut egui_context: Single<&mut EguiContext, With<MainMenuStateCameraForEgui>>,
) -> Result {
    let ctx = egui_context.get_mut();

    egui::Area::new("main_menu_gui_menu".into())
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            egui::Frame::NONE
                .fill(egui::Color32::from_rgb(34, 58, 51))
                .corner_radius(egui::CornerRadius::same(12))
                .inner_margin(egui::Margin::same(32))
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        if ui.button("Play").clicked() {
                            commands.set_state(OverallState::Playing);
                        }
                        if ui.button("Quit").clicked() {
                            quit_game();
                        }
                    });
                });
        });

    Ok(())
}
