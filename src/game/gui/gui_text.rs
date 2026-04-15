use bevy::prelude::*;

use crate::game::gui::{GuiNode, constants::*, fonts::GameFont};

pub struct GuiText {
    text: String,
    font: GameFont,
    size: f32,
}

impl GuiText {
    pub fn regular(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font: MAIN_FONT,
            size: TEXT_SIZE_REGULAR,
        }
    }

    pub fn h1(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font: MAIN_FONT,
            size: TEXT_SIZE_H1,
        }
    }

    pub fn h2(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font: MAIN_FONT,
            size: TEXT_SIZE_H2,
        }
    }

    pub fn h3(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font: MAIN_FONT,
            size: TEXT_SIZE_H3,
        }
    }

    pub fn small_mono(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font: GameFont::Mono,
            size: TEXT_SIZE_SMALL_MONO,
        }
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
                TextFont {
                    font_size: self.size,
                    ..default()
                },
                self.font,
            ))
            .id()
    }
}
