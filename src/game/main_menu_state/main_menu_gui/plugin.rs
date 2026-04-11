use bevy::prelude::*;

use crate::game::gui::{GuiButton, GuiDiv, GuiNode, GuiScreenDiv, GuiText};

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

fn funny(_: On<interactions::ClickMeButtonEv>) {
    debug!("Ahheyeyeeee");
}

pub fn make_main_menu_gui(mut commands: &mut Commands) {
    GuiScreenDiv::new(
        Color::srgb(0.0, 0.0, 0.1),
        FlexDirection::Column,
        vec![Box::new(GuiDiv::new(
            FlexDirection::Column,
            vec![
                Box::new(GuiText::new("thingy")),
                Box::new(GuiDiv::new(
                    FlexDirection::Column,
                    vec![
                        Box::new(GuiText::new("one")),
                        Box::new(GuiText::new("two")),
                        Box::new(GuiButton::new("clickme", Some(|| interactions::ClickMeButtonEv))),
                    ],
                )),
            ],
        ))],
    )
    .spawn(&mut commands);
}
