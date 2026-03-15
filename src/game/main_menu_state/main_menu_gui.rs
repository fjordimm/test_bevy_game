use bevy::prelude::*;
use bevy_egui::{EguiContext, egui};

use crate::game::{
    core::{quit_game, states::OverallState},
    egui_setup::tags::CameraForEgui,
};

pub fn main_menu_gui(
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
