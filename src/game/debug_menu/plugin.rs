use std::time::Duration;

use bevy::{diagnostic::DiagnosticsStore, prelude::*, time::common_conditions::on_timer};
use bevy_prng::{ChaCha8Rng, WyRand};
use bevy_rand::global::GlobalRng;
use rand_core::Rng;

use crate::game::{
    core::GlobalGuiRoot,
    gui::{GuiFloatingPanel, GuiNode, GuiText},
    util::warned_ok,
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

fn spawn_debug_menu(
    mut commands: Commands,
    gui_root: Res<GlobalGuiRoot>,
    window_query: Query<&Window>,
    mut rng_query: Query<&mut WyRand, With<GlobalRng>>,
) {
    if let Some(mut rng) = warned_ok!(rng_query.single_mut()) {
        debug!("um: {}", rng.next_u32());
    }

    let mut pos_x = 10.0;
    let mut pos_y = 10.0;
    if let Some(window) = warned_ok!(window_query.single()) {
        pos_x = window.width() / 2.0;
        pos_y = window.height() / 2.0;
    }

    let debug_menu = GuiFloatingPanel::new(
        FlexDirection::Column,
        pos_x,
        pos_y,
        (GuiText::small_mono("howdy ho"),),
    )
    .spawn(&mut commands, Some(gui_root.0));
    commands.entity(debug_menu).insert(ZIndex(2000));
}

fn update_debug_menu(_diag: Res<DiagnosticsStore>) {
    // if let Some(fps) = diag.get_measurement(&FrameTimeDiagnosticsPlugin::FPS) {
    //     debug!("{:?}", fps.value);
    // }
}
