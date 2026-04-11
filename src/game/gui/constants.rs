use bevy::prelude::*;

use crate::game::gui::fonts::GuiFont;

pub const MAIN_COLOR: Color = Color::srgb(0.1, 0.1, 0.125);
pub const BORDER_RADIUS: i32 = 6;
pub const MAIN_PADDING: i32 = 10;
pub const MAIN_FONT: GuiFont = GuiFont::Sans;
pub fn main_box_shadow() -> BoxShadow {
    BoxShadow::new(
        Color::srgba(0.0, 0.0, 0.0, 0.5),
        Val::ZERO,
        Val::ZERO,
        Val::ZERO,
        px(4),
    )
}

pub const TEXT_SIZE_REGULAR: f32 = 20.0;
pub const TEXT_SIZE_H1: f32 = 32.0;
pub const TEXT_SIZE_H2: f32 = 28.0;
pub const TEXT_SIZE_H3: f32 = 24.0;

pub const BUTTON_COLOR_MAIN: Color = Color::srgb(0.0, 0.0, 0.5);
pub const BUTTON_COLOR_HOVER: Color = Color::srgb(0.0, 0.0, 0.4);
pub const BUTTON_COLOR_PRESSED: Color = Color::srgb(0.0, 0.0, 0.3);
