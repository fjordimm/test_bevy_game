use bevy::{diagnostic::DiagnosticsStore, prelude::*, time::common_conditions::on_timer};
use std::time::Duration;

use crate::game::{
    core::{
        resources::{GlobalGuiRoot, KeyBindings},
        sets::GlobalStartupOrdering,
    },
    gui::{GuiButton, GuiFloatingPanel, GuiFloatingPanelTag, GuiNode, GuiText},
};

pub struct DebugMenuPlugin;

impl Plugin for DebugMenuPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Startup,
                spawn_debug_menu
                    .in_set(GlobalStartupOrdering::GuiSpawning)
            )
            .add_systems(Update,
                update_debug_menu
                    .run_if(on_timer(Duration::from_secs(1)))
            )
            .add_systems(Update, toggle_debug_menu);
    }
}

#[derive(Component)]
struct DebugMenuTag;

fn spawn_debug_menu(mut commands: Commands, gui_root: Res<GlobalGuiRoot>) {
    let debug_menu = GuiFloatingPanel::new(
            false,
            30.0,
            30.0,
            "Debug Menu",
            (
                GuiText::new_small_mono("mmmmmmmmmmmmmmmmmmmmmmmmmmmmmm\nmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm\nmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm\nmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm\nmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm\nmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm", false),
                GuiText::new_small_mono("I'm some more text", false),
                GuiButton::new_regular_eventless("nothin"),
            )
        )
    .spawn(&mut commands, Some(gui_root.0));
    commands.entity(debug_menu).insert(ZIndex(4000));
    commands.entity(debug_menu).insert(DebugMenuTag);
}

fn update_debug_menu(_diag: Res<DiagnosticsStore>) {
    // if let Some(fps) = diag.get_measurement(&FrameTimeDiagnosticsPlugin::FPS) {
    //     debug!("{:?}", fps.value);
    // }
}

fn toggle_debug_menu(
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut debug_menu_q: Query<&mut GuiFloatingPanelTag, With<DebugMenuTag>>,
) {
    if keys.just_pressed(key_bindings.toggle_debug_menu) {
        debug_menu_q.iter_mut().for_each(|mut panel| {
            let is_active = panel.get_is_active();
            panel.set_is_active(!is_active);
        });
    }
}
