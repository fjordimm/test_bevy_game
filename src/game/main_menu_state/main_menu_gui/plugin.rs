use bevy::prelude::*;

use crate::game::{
    core::{quit_game, states::OverallState},
    gui::{self, GuiButton, GuiDiv, GuiScreenDiv, GuiText},
};

pub struct MainMenuGuiPlugin;

impl Plugin for MainMenuGuiPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_observer(play_button_observer)
            .add_observer(quit_button_observer);
    }
}

pub mod interactions {
    use bevy_ecs::event::Event;

    #[derive(Event)]
    pub struct PlayButtonEv;

    #[derive(Event)]
    pub struct QuitButtonEv;
}

fn play_button_observer(_: On<interactions::PlayButtonEv>, mut commands: Commands) {
    commands.set_state(OverallState::Playing);
}

fn quit_button_observer(_: On<interactions::QuitButtonEv>) {
    quit_game();
}

macro_rules! children {
    () => {
        21
    };
}

pub fn make_main_menu_gui() -> GuiScreenDiv {
    debug!("{:?}", children!());

    GuiScreenDiv::new(
        gui::constants::MAIN_COLOR,
        FlexDirection::Column,
        (GuiDiv::new(
            FlexDirection::Column,
            (
                GuiText::h1("Main Menu"),
                GuiButton::plain(Some(|| interactions::PlayButtonEv), "Play"),
                GuiButton::plain(Some(|| interactions::QuitButtonEv), "Quit"),
            ),
        ),),
    )
}
