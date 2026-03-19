use bevy::prelude::*;

use crate::gui::GuiParent;

pub struct GuiText {
    text: String,
    font: Handle<Font>,
}

impl GuiText {
    pub fn new(text: impl Into<String>, font: Handle<Font>) -> Self {
        Self {
            text: text.into(),
            font,
        }
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
                TextFont {
                    font: self.font.clone(),
                    ..default()
                },
            ))
            .id()
    }
}
