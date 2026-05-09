use bevy::prelude::*;

use crate::game::{
    core::{
        global_resources::GlobalGuiRoot, quit_game, sets::GlobalStartupOrdering,
        states::OverallState,
    },
    gui::{
        self, GuiButton, GuiDiv, GuiDivStyle, GuiNode, GuiScreenDiv, GuiScreenDivTag, GuiText,
        constants::*,
    },
    playing_state::sets::PlayingStateOrdering,
    util::warned_ok,
};

pub struct MainMenuGuiPlugin;

impl Plugin for MainMenuGuiPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Startup,
                spawn_main_menu_gui
                    .in_set(GlobalStartupOrdering::GuiSpawning)
            )
            .add_systems(OnEnter(OverallState::MainMenu),
                update_main_menu_gui_hiddenness
                    .in_set(PlayingStateOrdering::Ui)
            )
            .add_systems(OnExit(OverallState::MainMenu),
                update_main_menu_gui_hiddenness
                    .in_set(PlayingStateOrdering::Ui)
            )
            .add_observer(play_button_observer)
            .add_observer(quit_button_observer);
    }
}

#[derive(Component)]
struct MainMenuGuiTag;

fn spawn_main_menu_gui(mut commands: Commands, gui_root: Res<GlobalGuiRoot>) {
    let main_menu_gui = GuiScreenDiv::new(
        false,
        gui::constants::MAIN_BACKGROUND_COLOR,
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
                GuiText::new_h1("Main Menu", true),
                GuiButton::new_regular(|| interactions::PlayButtonEv, "Play"),
                GuiButton::new_regular(|| interactions::QuitButtonEv, "Quit"),
            ),
        ),),
    )
    .spawn(&mut commands, Some(gui_root.0));
    commands.entity(main_menu_gui).insert(ZIndex(3001));
    commands.entity(main_menu_gui).insert(MainMenuGuiTag);
}

fn update_main_menu_gui_hiddenness(
    mut main_menu_gui_q: Query<&mut GuiScreenDivTag, With<MainMenuGuiTag>>,
    overall_state: Res<State<OverallState>>,
) {
    if let Some(mut screen_div) = warned_ok!(main_menu_gui_q.single_mut()) {
        screen_div.is_active = match overall_state.get() {
            OverallState::MainMenu => true,
            _ => false,
        }
    }
}

mod interactions {
    use bevy_ecs::event::Event;

    #[derive(Event, Clone)]
    pub struct PlayButtonEv;

    #[derive(Event, Clone)]
    pub struct QuitButtonEv;
}

fn play_button_observer(_: On<interactions::PlayButtonEv>, mut commands: Commands) {
    commands.set_state(OverallState::Playing);
}

fn quit_button_observer(_: On<interactions::QuitButtonEv>) {
    quit_game();
}
