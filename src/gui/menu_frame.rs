use bevy::prelude::*;
use bevy_egui::egui::{self, Context, Ui};

pub struct MenuFrame;

impl MenuFrame {
    pub fn show(self, ctx: &Context, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
        let style = ctx.style();

        egui::Frame::NONE
            .fill(style.visuals.text_color())
            .corner_radius(egui::CornerRadius::same(12))
            .inner_margin(egui::Margin::same(32))
            .show(ui, add_contents);

        egui::Window::new("Yeeeee").show(ctx, |_| {});

        // debug!("{:?}", style.visuals.panel_fill);
    }
}
