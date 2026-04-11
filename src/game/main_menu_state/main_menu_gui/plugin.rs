use bevy::prelude::*;

use crate::game::{
    core::states::OverallState,
    gui::{self, GuiButton, GuiDiv, GuiNode, GuiScreenDiv, GuiText},
};

pub mod interactions {
    use bevy_ecs::event::Event;

    #[derive(Event)]
    pub struct ClickMeButtonEv;
}

pub struct MainMenuGuiPlugin;

impl Plugin for MainMenuGuiPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_observer(funny);
    }
}

fn funny(_: On<interactions::ClickMeButtonEv>, mut commands: Commands) {
    debug!("Button was clicked.");
    commands.set_state(OverallState::Playing);
}

pub fn make_main_menu_gui() -> GuiScreenDiv {
    GuiScreenDiv::new(
        gui::constants::MAIN_COLOR,
        FlexDirection::Column,
        vec![Box::new(GuiDiv::new(
            FlexDirection::Column,
            vec![
                Box::new(GuiText::h1("Main Menu")),
                Box::new(GuiDiv::new(
                    FlexDirection::Column,
                    vec![
                        Box::new(GuiText::regular("one")),
                        Box::new(GuiText::regular("two")),
                        Box::new(GuiButton::new(
                            "clickme",
                            Some(|| interactions::ClickMeButtonEv),
                        )),
                    ],
                )),
            ],
        ))],
    )
}
