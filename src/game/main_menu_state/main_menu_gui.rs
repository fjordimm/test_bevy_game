use core::f32;

use bevy::prelude::*;
use bevy_egui::{
    EguiContext,
    egui::{self, Direction, Layout, Margin},
};

use crate::{
    game::{
        core::{quit_game, states::OverallState},
        egui_setup::tags::CameraForEgui,
    },
    gui::MenuRegion,
};

pub fn main_menu_gui(
    mut egui_contexts: Query<&mut EguiContext, With<CameraForEgui>>,
    mut next_overall_state: ResMut<NextState<OverallState>>,
) -> Result {
    for mut egui_context in &mut egui_contexts {
        let ctx = egui_context.get_mut();

        egui::Area::new("ummmm".into())
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .layout(Layout::centered_and_justified(Direction::TopDown))
            .show(ctx, |ui| {
                egui::Frame::NONE
                    .fill(egui::Color32::from_rgb(34, 58, 51))
                    .corner_radius(egui::CornerRadius::same(12))
                    .inner_margin(Margin::same(30))
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            if ui.button("Playyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy").clicked() {
                                next_overall_state.set(OverallState::Playing);
                            }
                            if ui.button("Quit").clicked() {
                                quit_game();
                            }
                        });
                    });
            });

        // egui::CentralPanel::default()
        //     .frame(
        //         egui::Frame::NONE
        //             .fill(egui::Color32::from_rgb(255, 0, 0))
        //             .corner_radius(egui::CornerRadius::same(12)),
        //     )
        //     .show(ctx, |ui| {
        //         egui::Area::new("yerrr".into())

        //             .layout(Layout::centered_and_justified(Direction::TopDown))
        //             .show(ctx, |ui| {
        //             egui::Frame::NONE
        //                 .fill(egui::Color32::from_rgb(34, 58, 51))
        //                 .corner_radius(egui::CornerRadius::same(12))
        //                 .outer_margin(Margin::same(100))
        //                 .show(ui, |ui| {
        //                     ui.vertical_centered(|ui| {
        //                         if ui.button("Play").clicked() {
        //                             next_overall_state.set(OverallState::Playing);
        //                         }
        //                         if ui.button("Quit").clicked() {
        //                             quit_game();
        //                         }
        //                     });
        //                 });
        //         });
        //     });

        // egui::CentralPanel::default()
        //     .frame(
        //         egui::Frame::NONE
        //             .fill(egui::Color32::from_rgb(255, 0, 0))
        //             .corner_radius(egui::CornerRadius::same(12)),
        //     )
        //     .show(ctx, |ui| {
        //         egui::Window::new("bluh")
        //             .title_bar(false)
        //             .resizable(false)
        //             .movable(false)
        //             .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        //             .min_width(f32::INFINITY)
        //             .min_height(f32::INFINITY)
        //             .frame(
        //                 egui::Frame::NONE
        //                     .fill(egui::Color32::from_rgb(34, 58, 51))
        //                     .corner_radius(egui::CornerRadius::same(12))
        //                     .outer_margin(Margin::same(100)),
        //             )
        //             .show(ctx, |ui| {
        //                 ui.vertical_centered(|ui| {
        //                     if ui.button("Play").clicked() {
        //                         next_overall_state.set(OverallState::Playing);
        //                     }
        //                     if ui.button("Quit").clicked() {
        //                         quit_game();
        //                     }
        //                 });
        //             });
        //     });

        // egui::CentralPanel::default()
        //     .frame(
        //         egui::Frame::NONE
        //             .fill(egui::Color32::from_rgb(255, 0, 0))
        //             .corner_radius(egui::CornerRadius::same(12)),
        //     )
        //     .show(ctx, |ui| {
        //         egui::Frame::new()
        //             .fill(egui::Color32::from_rgb(34, 58, 51))
        //             .corner_radius(egui::CornerRadius::same(12))
        //             .outer_margin(Margin::same(100))
        //             .show(ui, |ui| {
        //                 ui.vertical_centered(|ui| {
        //                     if ui.button("Play").clicked() {
        //                         next_overall_state.set(OverallState::Playing);
        //                     }
        //                     if ui.button("Quit").clicked() {
        //                         quit_game();
        //                     }
        //                 });
        //             });
        //     });

        // egui::Area::new("main_menu_gui_menu".into())
        //     .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        //     .order(egui::Order::Foreground)
        //     .show(ctx, |_ui| {
        //         egui::CentralPanel::default()
        //             .frame(
        //                 egui::Frame::NONE
        //                     .fill(egui::Color32::from_rgb(255, 0, 0))
        //                     .corner_radius(egui::CornerRadius::same(12)),
        //             )
        //             .show(ctx, |ui| {
        //                 egui::Frame::new()
        //                     .fill(egui::Color32::from_rgb(34, 58, 51))
        //                     .corner_radius(egui::CornerRadius::same(12))
        //                     .outer_margin(Margin::same(100))
        //                     .show(ui, |ui| {
        //                         ui.vertical_centered(|ui| {
        //                             if ui.button("Play").clicked() {
        //                                 next_overall_state.set(OverallState::Playing);
        //                             }
        //                             if ui.button("Quit").clicked() {
        //                                 quit_game();
        //                             }
        //                         });
        //                     });
        //             });
        //     });

        // egui::Area::new("main_menu_gui_menu".into())
        //     .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        //     .order(egui::Order::Foreground)
        //     .show(ctx, |_ui| {
        //         egui::CentralPanel::default()
        //             .frame(
        //                 egui::Frame::NONE
        //                     .fill(egui::Color32::from_rgb(255, 0, 0))
        //                     .corner_radius(egui::CornerRadius::same(12)),
        //             )
        //             .show(ctx, |ui| {
        //                 egui::Window::new("menu_region")
        //                     .title_bar(false)
        //                     .resizable(false)
        //                     .movable(false)
        //                     .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        //                     .order(egui::Order::Foreground)
        //                     .frame(
        //                         egui::Frame::NONE
        //                             .fill(egui::Color32::from_rgb(34, 58, 51))
        //                             .corner_radius(egui::CornerRadius::same(12))
        //                             .outer_margin(Margin::same(100)),
        //                     )
        //                     .show(ctx, |ui| {
        //                         ui.vertical_centered(|ui| {
        //                             if ui.button("Play").clicked() {
        //                                 next_overall_state.set(OverallState::Playing);
        //                             }
        //                             if ui.button("Quit").clicked() {
        //                                 quit_game();
        //                             }
        //                         });
        //                     });
        //             });
        //     });

        // egui::Area::new("main_menu_gui_menu".into())
        //     .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        //     .order(egui::Order::Foreground)
        //     .show(ctx, |ui| {
        //         MenuRegion.show(ctx, ui, |ui| {
        //             ui.vertical_centered(|ui| {
        //                 if ui.button("Play").clicked() {
        //                     next_overall_state.set(OverallState::Playing);
        //                 }
        //                 if ui.button("Quit").clicked() {
        //                     quit_game();
        //                 }
        //             });
        //         });
        //     });

        // MenuRegion.show(ctx, |ui| {
        //     ui.vertical_centered(|ui| {
        //         if ui.button("Play").clicked() {
        //             next_overall_state.set(OverallState::Playing);
        //         }
        //         if ui.button("Quit").clicked() {
        //             quit_game();
        //         }
        //     });
        // });
    }

    Ok(())
}
