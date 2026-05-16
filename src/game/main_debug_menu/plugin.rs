use bevy::{diagnostic::DiagnosticsStore, prelude::*, time::common_conditions::on_timer};
use rand::{Rng, RngExt};
use rand_distr::{Distribution, Normal, StandardNormal};
use std::time::Duration;

use crate::game::{
    core::{
        global_resources::{GlobalGuiRoot, KeyBindings},
        sets::GlobalStartupOrdering,
    },
    gui::{GuiFloatingPanel, GuiFloatingPanelTag, GuiNode, GuiText},
    playing_state::sets::PlayingStateOrdering,
    random::{Prng, rands::GeneralRand},
    util::{alrmo, alrro},
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
    mut general_rand: Single<&mut Prng, With<GeneralRand>>,
) {
    for _ in 0..50 {
        debug!(
            "Random Number: {}",
            general_rand.sample(alrro!(Normal::new(0.0, f64::NAN)))
        );
    }

    // let nd = rand_distr::Normal::from;

    let mut pos_x = 10.0;
    let mut pos_y = 10.0;
    if let Some(_window) = alrmo!(window_q.single()) {
        pos_x = 100.0;
        pos_y = 100.0;
    }

    let main_debug_menu = GuiFloatingPanel::new(
        true,
        pos_x,
        pos_y,
        "Debug Menu",
        (
            GuiText::new_small_mono("mmmmmmmmmmmmmmmmmmmmmmmmmmmmmm\nmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm\nmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm\nmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm\nmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm\nmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm", false),
            GuiText::new_small_mono("I'm some more text", false),
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
            panel.is_active = match panel.is_active {
                false => true,
                true => false,
            }
        });
    }
}
