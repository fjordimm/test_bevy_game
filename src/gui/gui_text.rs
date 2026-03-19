use bevy::prelude::*;

use crate::gui::GuiNode;

pub struct GuiText {
    text: String,
}

impl GuiText {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl GuiNode for GuiText {
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
            ))
            .id()
    }
}
