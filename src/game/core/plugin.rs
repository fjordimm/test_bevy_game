use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use crate::game::{
    core::{
        resources::{FontHandles, GlobalGuiRoot, KeyBindings, UiIconHandles},
        sets::{GLOBAL_STARTUP_ORDERING_ORDER, GlobalStartupOrdering},
        states::{MouseMode, OverallState},
    },
    gui::make_global_gui_root,
    util::alrms,
};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
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
        ;
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(FontHandles::make(&asset_server));
    commands.insert_resource(UiIconHandles::make(&asset_server));

    let gui_root = commands.spawn(make_global_gui_root()).id();
    commands.insert_resource(GlobalGuiRoot(gui_root));
}

fn start_game(mut next_overall_state: ResMut<NextState<OverallState>>) {
    next_overall_state.set(OverallState::MainMenu);
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
