use bevy::prelude::*;

use crate::game::gui::fonts::GameFont;

pub const MAIN_COLOR: Color = Color::hsv(185.0, 0.1, 0.15);
pub const PAUSE_MENU_BG_COLOR: Color = Color::hsva(0.0, 0.0, 0.0, 0.5);
pub const BORDER_RADIUS: i32 = 6;
pub const MAIN_PADDING: i32 = 10;
pub const MAIN_FONT: GameFont = GameFont::Sans;
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
pub const TEXT_SIZE_SMALL_MONO: f32 = 14.0;

pub const BUTTON_COLOR_MAIN: Color = Color::hsv(185.0, 0.1, 0.25);
pub const BUTTON_COLOR_HOVER: Color = Color::hsv(185.0, 0.1, 0.2);
pub const BUTTON_COLOR_PRESSED: Color = Color::hsv(185.0, 0.1, 0.15);
