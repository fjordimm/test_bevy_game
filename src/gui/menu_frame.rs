use bevy::prelude::*;
use bevy_egui::egui::{self, Context, Ui};

pub struct MenuRegion;

impl MenuRegion {
    pub fn show(self, ctx: &Context, add_contents: impl FnOnce(&mut Ui)) {
        // let style = ctx.style();

        egui::Window::new("menu_region")
            .title_bar(false)
            .resizable(false)
            .movable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .order(egui::Order::Foreground)
            .frame(
                egui::Frame::NONE
                    .fill(egui::Color32::from_rgb(34, 58, 51))
                    .corner_radius(egui::CornerRadius::same(12)),
            )
            .show(ctx, add_contents);
    }
}
