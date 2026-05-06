use bevy::prelude::*;

use crate::game::gui::fonts::FontOption;

pub const MAIN_COLOR: Color = Color::hsv(185.0, 0.1, 0.15);
pub const PAUSE_MENU_BG_COLOR: Color = Color::hsva(0.0, 0.0, 0.0, 0.75);
pub const BORDER_RADIUS: i32 = 6;
pub const MAIN_PADDING: i32 = 10;
pub const MINOR_PADDING: i32 = 5;
pub const MAIN_FONT: FontOption = FontOption::Sans;
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
pub const TEXT_SIZE_SMALL: f32 = 14.0;
pub const TEXT_SIZE_SMALL_MONO: f32 = 14.0;

pub const BUTTON_COLOR_MAIN: Color = Color::hsv(185.0, 0.1, 0.25);
pub const BUTTON_COLOR_HOVER: Color = Color::hsv(185.0, 0.1, 0.2);
pub const BUTTON_COLOR_PRESSED: Color = Color::hsv(185.0, 0.1, 0.15);

pub const TITLE_BAR_BUTTON_COLOR_MAIN: Color = BUTTON_COLOR_PRESSED;
pub const TITLE_BAR_BUTTON_COLOR_HOVER: Color = BUTTON_COLOR_HOVER;
pub const TITLE_BAR_BUTTON_COLOR_PRESSED: Color = BUTTON_COLOR_MAIN;
pub const TITLE_BAR_BUTTON_SIZE: i32 = 20;
pub const TITLE_BAR_BUTTON_ICON_SIZE: i32 = 16;
