use bevy::{diagnostic::DiagnosticsStore, prelude::*, time::common_conditions::on_timer};
use std::time::Duration;

use crate::game::{
    core::{
        resources::{GlobalGuiRoot, KeyBindings},
        sets::GlobalStartupOrdering,
    },
    gui::{GuiButton, GuiFloatingPanel, GuiFloatingPanelTag, GuiNode, GuiText},
    playing_state::sets::PlayingStateOrdering,
};

pub struct MainDebugMenuPlugin;

impl Plugin for MainDebugMenuPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Startup,
                spawn_main_debug_menu
                    .in_set(GlobalStartupOrdering::GuiSpawning)
            )
            .add_systems(Update,
                update_main_debug_menu
                    .in_set(PlayingStateOrdering::Ui)
                    .run_if(on_timer(Duration::from_secs(1)))
            )
            .add_systems(Update,
                toggle_main_debug_menu
                    .in_set(PlayingStateOrdering::Ui)
            );
    }
}

#[derive(Component)]
struct MainDebugMenuTag;

fn spawn_main_debug_menu(mut commands: Commands, gui_root: Res<GlobalGuiRoot>) {
    let main_debug_menu = GuiFloatingPanel::new(
            true,
            30.0,
            30.0,
            "Main Debug Menu",
            (
                GuiText::new_small_mono("mmmmmmmmmmmmmmmmmmmmmmmmmmmmmm\nmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm\nmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm\nmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm\nmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm\nmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm", false),
                GuiText::new_small_mono("I'm some more text", false),
                GuiButton::new_regular_eventless("nothin"),
            )
        )
    .spawn(&mut commands, Some(gui_root.0));
    commands.entity(main_debug_menu).insert(ZIndex(4000));
    commands.entity(main_debug_menu).insert(MainDebugMenuTag);
}

fn update_main_debug_menu(_diag: Res<DiagnosticsStore>) {
    // if let Some(fps) = diag.get_measurement(&FrameTimeDiagnosticsPlugin::FPS) {
    //     debug!("{:?}", fps.value);
    // }
}

fn toggle_main_debug_menu(
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut main_debug_menu_q: Query<&mut GuiFloatingPanelTag, With<MainDebugMenuTag>>,
) {
    if keys.just_pressed(key_bindings.open_main_debug_menu) {
        main_debug_menu_q.iter_mut().for_each(|mut panel| {
            let is_active = panel.get_is_active();
            panel.set_is_active(!is_active);
        });
    }
}
