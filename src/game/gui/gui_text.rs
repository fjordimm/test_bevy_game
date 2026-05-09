use bevy::prelude::*;

use crate::game::gui::{GuiNode, constants::*, fonts::FontOption};

pub struct GuiText {
    text: String,
    font: FontOption,
    size: f32,
    wraps: bool,
}

impl GuiText {
    pub fn new(text: impl Into<String>, font: FontOption, size: f32, wraps: bool) -> Self {
        Self {
            text: text.into(),
            font: font,
            size: size,
            wraps: wraps,
        }
    }

    pub fn new_regular(text: impl Into<String>, wraps: bool) -> Self {
        Self {
            text: text.into(),
            font: MAIN_FONT,
            size: TEXT_SIZE_REGULAR,
            wraps: wraps,
        }
    }

    pub fn new_h1(text: impl Into<String>, wraps: bool) -> Self {
        Self {
            text: text.into(),
            font: MAIN_FONT,
            size: TEXT_SIZE_H1,
            wraps: wraps,
        }
    }

    pub fn new_h2(text: impl Into<String>, wraps: bool) -> Self {
        Self {
            text: text.into(),
            font: MAIN_FONT,
            size: TEXT_SIZE_H2,
            wraps: wraps,
        }
    }

    pub fn new_h3(text: impl Into<String>, wraps: bool) -> Self {
        Self {
            text: text.into(),
            font: MAIN_FONT,
            size: TEXT_SIZE_H3,
            wraps: wraps,
        }
    }

    pub fn new_small(text: impl Into<String>, wraps: bool) -> Self {
        Self {
            text: text.into(),
            font: MAIN_FONT,
            size: TEXT_SIZE_SMALL,
            wraps: wraps,
        }
    }

    pub fn new_small_mono(text: impl Into<String>, wraps: bool) -> Self {
        Self {
            text: text.into(),
            font: FontOption::Mono,
            size: TEXT_SIZE_SMALL_MONO,
            wraps: wraps,
        }
    }
}

impl GuiNode for GuiText {
    fn spawn(self, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        let entity = commands
            .spawn((
                Text::new(&self.text),
                TextColor(MAIN_CONTENT_COLOR),
                TextFont {
                    font_size: self.size,
                    ..default()
                },
                match self.wraps {
                    true => TextLayout::new(Justify::default(), LineBreak::WordBoundary),
                    false => TextLayout::new(Justify::default(), LineBreak::NoWrap),
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
