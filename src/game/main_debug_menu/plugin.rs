use std::time::Duration;

use bevy::{diagnostic::DiagnosticsStore, prelude::*, time::common_conditions::on_timer};
use bevy_prng::WyRand;
use bevy_rand::global::GlobalRng;
use rand_core::Rng;

use crate::game::{
    core::{
        global_resources::{GlobalGuiRoot, KeyBindings},
        sets::GlobalStartupOrdering,
    },
    gui::{
        GuiDiv, GuiDivStyle, GuiFloatingPanel, GuiFloatingPanelTag, GuiNode, GuiText,
        constants::MAIN_PADDING,
    },
    playing_state::sets::PlayingStateOrdering,
    util::warned_ok,
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

fn spawn_main_debug_menu(
    mut commands: Commands,
    gui_root: Res<GlobalGuiRoot>,
    window_q: Query<&Window>,
    mut rng_q: Query<&mut WyRand, With<GlobalRng>>,
) {
    if let Some(mut rng) = warned_ok!(rng_q.single_mut()) {
        debug!("um: {}", rng.next_u32());
    }

    let mut pos_x = 10.0;
    let mut pos_y = 10.0;
    if let Some(window) = warned_ok!(window_q.single()) {
        // pos_x = window.width() / 4.0;
        // pos_y = window.height() / 4.0;
        pos_x = 100.0;
        pos_y = 100.0;
    }

    let main_debug_menu = GuiFloatingPanel::new(
        true,
        pos_x,
        pos_y,
        "Debug Menu",
        (GuiDiv::new(
            GuiDivStyle::None,
            true,
            UiRect::ZERO,
            MAIN_PADDING,
            FlexDirection::Column,
            JustifyContent::FlexStart,
            AlignItems::FlexStart,
            (GuiText::new_small_mono("hello"),),
        ),),
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
        for mut panel in &mut main_debug_menu_q {
            panel.is_active = match panel.is_active {
                false => true,
                true => false,
            }
        }
    }
}
