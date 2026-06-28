use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};
use bevy_rand::plugin::EntropyPlugin;

use crate::game::{
    core::{
        resources::{FontHandles, GlobalGuiRoot, KeyBindings, UiIconHandles},
        sets::{GLOBAL_STARTUP_ORDERING_ORDER, GlobalStartupOrdering},
        states::{MouseMode, OverallState},
    },
    gui::{make_global_gui_root, plugin::GuiPlugin},
    main_menu_state::plugin::MainMenuStatePlugin,
    playing_state::plugin::PlayingStatePlugin,
    primary_debug_menu::plugin::PrimaryDebugMenuPlugin,
    quick_dev_test::plugin::QuickDevTestPlugin,
    random::{Prng, plugin::RandomPlugin},
    util::alrms,
};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            // External Plugins
            .add_plugins(EntropyPlugin::<Prng>::default())
            .add_plugins(FrameTimeDiagnosticsPlugin::new(120))
            // Relevant Stuff
            .init_resource::<KeyBindings>()
            .init_state::<MouseMode>()
            .init_state::<OverallState>()
            .configure_sets(Startup, GLOBAL_STARTUP_ORDERING_ORDER.chain())
            .add_systems(Startup,
                startup
                    .in_set(GlobalStartupOrdering::CoreUseOnly)
            )
            .add_systems(Update,
                start_game
                    .run_if(run_once)
            )
            .add_systems(OnEnter(MouseMode::Grabbed), on_enter_mouse_grabbed)
            .add_systems(OnExit(MouseMode::Grabbed), on_exit_mouse_grabbed)
            .add_plugins(RandomPlugin)
            .add_plugins(GuiPlugin)
            .add_plugins(MainMenuStatePlugin)
            .add_plugins(PlayingStatePlugin)
            .add_plugins(QuickDevTestPlugin)
            .add_plugins(PrimaryDebugMenuPlugin)
        ;
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(FontHandles::make(&asset_server));
    commands.insert_resource(UiIconHandles::make(&asset_server));

    let gui_root = commands.spawn(make_global_gui_root()).id();
    commands.insert_resource(GlobalGuiRoot(gui_root));
}

fn start_game(mut commands: Commands) {
    commands.set_state(OverallState::MainMenu);
}

fn on_enter_mouse_grabbed(
    cursor_options_q: Option<Single<&mut CursorOptions, With<PrimaryWindow>>>,
) {
    if let Some(mut cursor_options) = alrms!(cursor_options_q) {
        cursor_options.grab_mode = CursorGrabMode::Confined;
        cursor_options.visible = false;
    }
}

fn on_exit_mouse_grabbed(
    cursor_options_q: Option<Single<&mut CursorOptions, With<PrimaryWindow>>>,
) {
    if let Some(mut cursor_options) = alrms!(cursor_options_q) {
        cursor_options.grab_mode = CursorGrabMode::None;
        cursor_options.visible = true;
    }
}
