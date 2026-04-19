use std::time::Duration;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    time::common_conditions::on_timer,
};

use crate::game::{
    core::GlobalGuiRoot,
    gui::{GuiFloatingPanel, GuiNode, GuiText},
};

pub struct DebugMenuPlugin;

impl Plugin for DebugMenuPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update,
                spawn_debug_menu
                    .run_if(run_once)
            )
            .add_systems(Update,
                update_debug_menu
                    .run_if(on_timer(Duration::from_secs(1)))
            );
    }
}

fn spawn_debug_menu(mut commands: Commands, gui_root: Res<GlobalGuiRoot>) {
    let debug_menu =
        GuiFloatingPanel::new(FlexDirection::Column, (GuiText::small_mono("howdy ho"),))
            .spawn(&mut commands, Some(gui_root.0));
    commands.entity(debug_menu).insert(ZIndex(2000));
}

fn update_debug_menu(_diag: Res<DiagnosticsStore>) {
    // if let Some(fps) = diag.get_measurement(&FrameTimeDiagnosticsPlugin::FPS) {
    //     debug!("{:?}", fps.value);
    // }
}
