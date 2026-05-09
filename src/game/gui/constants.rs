use bevy::prelude::*;

use crate::game::gui::fonts::FontOption;

pub const MAIN_BACKGROUND_COLOR: Color = Color::hsv(185.0, 0.3, 0.15);
pub const MAIN_CONTENT_COLOR: Color = Color::hsv(185.0, 0.075, 0.9);
pub const PAUSE_MENU_BG_COLOR: Color = Color::hsva(0.0, 0.0, 0.0, 0.75);
pub const BORDER_RADIUS: f32 = 5.0;
pub const MAIN_PADDING: f32 = 10.0;
pub const MINOR_PADDING: f32 = 5.0;
pub const MAIN_FONT: FontOption = FontOption::Sans;
pub fn main_box_shadow() -> BoxShadow {
    BoxShadow::new(
        Color::hsva(0.0, 0.0, 0.0, 0.5),
        Val::ZERO,
        Val::ZERO,
        Val::ZERO,
        px(5),
    )
}

pub const TEXT_SIZE_REGULAR: f32 = 20.0;
pub const TEXT_SIZE_H1: f32 = 32.0;
pub const TEXT_SIZE_H2: f32 = 28.0;
pub const TEXT_SIZE_H3: f32 = 24.0;
pub const TEXT_SIZE_SMALL: f32 = 14.0;
pub const TEXT_SIZE_SMALL_MONO: f32 = 14.0;

pub const BUTTON_COLOR_MAIN: Color = Color::hsv(185.0, 0.3, 0.25);
pub const BUTTON_COLOR_HOVER: Color = Color::hsv(185.0, 0.3, 0.2);
pub const BUTTON_COLOR_PRESSED: Color = Color::hsv(185.0, 0.3, 0.15);

pub const SCROLLBAR_COLOR: Color = MAIN_CONTENT_COLOR;
pub const SCROLLBAR_WIDTH: f32 = 10.0;
pub const SCROLLBAR_THUMB_WIDTH: f32 = 2.0;
pub const SCROLLBAR_THUMB_MIN_HEIGHT: f32 = 20.0;
pub const SCROLL_INTERVAL: f32 = 10.0;

pub const TITLE_BAR_BUTTON_COLOR_MAIN: Color = BUTTON_COLOR_PRESSED;
pub const TITLE_BAR_BUTTON_COLOR_HOVER: Color = BUTTON_COLOR_HOVER;
pub const TITLE_BAR_BUTTON_COLOR_PRESSED: Color = BUTTON_COLOR_MAIN;
pub const TITLE_BAR_BUTTON_SIZE: f32 = 20.0;
pub const TITLE_BAR_BUTTON_ICON_SIZE: f32 = 16.0;
pub const CORNER_RESIZER_SIZE: f32 = 10.0;
pub const CORNER_RESIZER_PADDING: f32 = 3.0;
