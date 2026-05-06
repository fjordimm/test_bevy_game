use bevy::prelude::*;

use crate::game::{
    core::{global_resources::GlobalGuiRoot, sets::GlobalStartupOrdering, states::OverallState},
    gui::{
        self, GuiButton, GuiDiv, GuiDivStyle, GuiNode, GuiScreenDiv, GuiScreenDivTag, GuiText,
        constants::MAIN_PADDING,
    },
    playing_state::states::PauseState,
    util::warned_ok,
};

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Startup,
                spawn_pause_menu
                    .in_set(GlobalStartupOrdering::GuiSpawning)
            )
            .add_systems(OnEnter(OverallState::Playing), update_pause_menu_hiddenness)
            .add_systems(OnExit(OverallState::Playing), update_pause_menu_hiddenness)
            .add_systems(OnEnter(PauseState::Paused), update_pause_menu_hiddenness)
            .add_systems(OnExit(PauseState::Paused), update_pause_menu_hiddenness)
            .add_observer(exit_button_observer)
            .add_observer(continue_button_observer);
    }
}

#[derive(Component)]
struct PauseMenuTag;

fn spawn_pause_menu(mut commands: Commands, gui_root: Res<GlobalGuiRoot>) {
    let pause_menu = GuiScreenDiv::new(
        false,
        gui::constants::PAUSE_MENU_BG_COLOR,
        FlexDirection::Column,
        (GuiDiv::new(
            GuiDivStyle::Regular,
            false,
            UiRect::all(px(MAIN_PADDING)),
            MAIN_PADDING,
            FlexDirection::Column,
            JustifyContent::Center,
            AlignItems::Stretch,
            (
                GuiText::new_h1("Pause Menu"),
                GuiButton::new_regular(|| interactions::ContinueButtonEv, "Continue"),
                GuiButton::new_regular(|| interactions::ExitButtonEv, "Exit"),
            ),
        ),),
    )
    .spawn(&mut commands, Some(gui_root.0));
    commands.entity(pause_menu).insert(ZIndex(3000));
    commands.entity(pause_menu).insert(PauseMenuTag);
}

fn update_pause_menu_hiddenness(
    mut pause_menu_q: Query<&mut GuiScreenDivTag, With<PauseMenuTag>>,
    overall_state: Res<State<OverallState>>,
    pause_state: Res<State<PauseState>>,
) {
    if let Some(ref mut screen_div) = warned_ok!(pause_menu_q.single_mut()) {
        screen_div.is_active = match overall_state.get() {
            OverallState::Playing => match pause_state.get() {
                PauseState::Paused => true,
                _ => false,
            },
            _ => false,
        }
    }
}

mod interactions {
    use bevy_ecs::event::Event;

    #[derive(Event, Clone)]
    pub struct ContinueButtonEv;

    #[derive(Event, Clone)]
    pub struct ExitButtonEv;
}

fn continue_button_observer(_: On<interactions::ContinueButtonEv>, mut commands: Commands) {
    commands.set_state(PauseState::Unpaused);
}

fn exit_button_observer(_: On<interactions::ExitButtonEv>, mut commands: Commands) {
    commands.set_state(OverallState::MainMenu);
}
