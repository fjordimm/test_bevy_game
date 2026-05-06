use bevy::prelude::*;

use crate::game::gui::{GuiNode, constants::*, fonts::FontOption};

pub struct GuiText {
    text: String,
    font: FontOption,
    size: f32,
}

impl GuiText {
    pub fn new(text: impl Into<String>, font: FontOption, size: f32) -> Self {
        Self {
            text: text.into(),
            font: font,
            size: size,
        }
    }

    pub fn new_regular(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font: MAIN_FONT,
            size: TEXT_SIZE_REGULAR,
        }
    }

    pub fn new_h1(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font: MAIN_FONT,
            size: TEXT_SIZE_H1,
        }
    }

    pub fn new_h2(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font: MAIN_FONT,
            size: TEXT_SIZE_H2,
        }
    }

    pub fn new_h3(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font: MAIN_FONT,
            size: TEXT_SIZE_H3,
        }
    }

    pub fn new_small(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font: MAIN_FONT,
            size: TEXT_SIZE_SMALL,
        }
    }

    pub fn new_small_mono(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font: FontOption::Mono,
            size: TEXT_SIZE_SMALL_MONO,
        }
    }
}

impl GuiNode for GuiText {
    fn spawn(self, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        let entity = commands
            .spawn((
                Text::new(&self.text),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: self.size,
                    ..default()
                },
                self.font,
            ))
            .id();
        if let Some(par) = parent {
            commands.entity(par).add_child(entity);
        }

        entity
    }

    fn spawn_dyn(self: Box<Self>, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        self.spawn(commands, parent)
    }
}
