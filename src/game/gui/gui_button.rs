use bevy::prelude::*;

use crate::game::gui::{
    GuiParent, GuiText,
    constants::{DIV_BORDER_COLOR, DIV_BORDER_SIZE, DIV_MAIN_COLOR, MAIN_PADDING},
};

pub struct GuiButton {
    text: String,
}

impl GuiButton {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl GuiParent for GuiButton {
    fn spawn(&self, commands: &mut Commands) -> Entity {
        let entity = commands
            .spawn((
                Node {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(px(MAIN_PADDING)),
                    border: UiRect::all(px(DIV_BORDER_SIZE)),
                    ..default()
                },
                BackgroundColor(DIV_MAIN_COLOR),
                BorderColor::all(DIV_BORDER_COLOR),
            ))
            .id();

        {
            let child_entity = GuiText::new(&self.text).spawn(commands);
            commands.entity(entity).add_child(child_entity);
        }

        entity
    }
}
