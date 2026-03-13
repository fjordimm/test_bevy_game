use bevy::prelude::*;
use bevy_egui::{EguiContext, egui};

use crate::game::{
    core::states::OverallState,
    playing_state::{states::PauseState, tags::PlayingStateCameraForEgui},
};

pub fn pause_gui(
    mut egui_context: Single<&mut EguiContext, With<PlayingStateCameraForEgui>>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
    mut next_overall_state: ResMut<NextState<OverallState>>,
) -> Result {
    let ctx = egui_context.get_mut();

    egui::Area::new("pause_gui_background".into())
        .fixed_pos(egui::pos2(0.0, 0.0))
        .order(egui::Order::Background)
        .show(ctx, |ui| {
            ui.painter().rect_filled(
                ctx.viewport_rect(),
                0.0,
                egui::Color32::from_black_alpha(180),
            );
        });

    egui::Area::new("pause_gui_menu".into())
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            egui::Frame::NONE
                .fill(egui::Color32::from_rgb(34, 58, 51))
                .corner_radius(egui::CornerRadius::same(12))
                .inner_margin(egui::Margin::same(32))
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Paused");
                        if ui.button("Resume").clicked() {
                            next_pause_state.set(PauseState::Unpaused);
                        }
                        if ui.button("Exit to Main Menu").clicked() {
                            next_overall_state.set(OverallState::MainMenu)
                        }
                    });
                });
        });

    Ok(())
}
