use bevy::prelude::*;

use crate::game::gui::{GuiParent, constants::MAIN_FONT};

pub struct GuiText {
    text: String,
}

impl GuiText {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl GuiParent for GuiText {
    fn spawn(&self, commands: &mut Commands) -> Entity {
        commands
            .spawn((
                Node {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Text::new(&self.text),
                TextColor(Color::WHITE),
                TextFont::default(),
                MAIN_FONT,
            ))
            .id()
    }
}
