use bevy::prelude::*;
use bevy_egui::{EguiContext, egui};

use crate::game::{
    core::{
        quit_game,
        states::{MouseMode, OverallState},
    },
    main_menu_state::tags::{MainMenuStateCameraForEgui, MainMenuStateEntity},
    egui_setup::tags::CameraForEgui,
};

pub struct MainMenuStatePlugin;

impl Plugin for MainMenuStatePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::MainMenu), on_enter)
            .add_systems(OnExit(OverallState::MainMenu), on_exit)
            .add_systems(CameraForEgui,
                main_menu_gui
                    .run_if(in_state(OverallState::MainMenu))
            );
    }
}

fn on_enter(mut commands: Commands, mut next_mouse_mode: ResMut<NextState<MouseMode>>) {
    next_mouse_mode.set(MouseMode::Free);

    commands.spawn((
        MainMenuStateEntity,
        MainMenuStateCameraForEgui,
        Camera2d::default(),
    ));
}

fn on_exit(mut commands: Commands, query: Query<Entity, With<MainMenuStateEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn main_menu_gui(
    mut egui_contexts: Query<&mut EguiContext, With<CameraForEgui>>,
    mut next_overall_state: ResMut<NextState<OverallState>>,
) -> Result {
    for mut egui_context in &mut egui_contexts {
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
                                next_overall_state.set(OverallState::Playing);
                            }
                            if ui.button("Quit").clicked() {
                                quit_game();
                            }
                        });
                    });
            });
    }

    Ok(())
}
