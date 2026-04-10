use bevy::prelude::*;

use crate::game::gui::{GuiButton, GuiDiv, GuiNode, GuiScreenDiv, GuiText};

#[derive(Event)]
pub struct ClickMeButtonEv;

pub fn main_menu_gui(mut commands: &mut Commands) {
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
                        Box::new(GuiButton::new("clickme", Some(ClickMeButtonEv))),
                    ],
                )),
            ],
        ))],
    )
    .spawn(&mut commands);
}
