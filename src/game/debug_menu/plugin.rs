use std::time::Duration;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    time::common_conditions::on_timer,
};

use crate::game::gui::{GuiDiv, GuiText};

pub struct DebugMenuPlugin;

impl Plugin for DebugMenuPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update,
                add_debug_menu_gui
                    .run_if(run_once)
            )
            .add_systems(Update,
                update_debug_menu
                    .run_if(on_timer(Duration::from_secs(1)))
            );
    }
}

fn add_debug_menu_gui() {
    GuiDiv::new(
        FlexDirection::Column,
        vec![Box::new(GuiText::h1("Debug Menu"))],
    );
}

fn update_debug_menu(diag: Res<DiagnosticsStore>) {
    if let Some(fps) = diag.get_measurement(&FrameTimeDiagnosticsPlugin::FPS) {
        debug!("{:?}", fps.value);
    }
}
