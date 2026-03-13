use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};

use crate::core::states::OverallState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_plugins(EguiPlugin::default())
            .add_systems(EguiPrimaryContextPass, funny1);
    }
}

fn funny1(mut commands: Commands, mut contexts: EguiContexts) -> Result {
    let ctx = contexts.ctx_mut()?;

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
