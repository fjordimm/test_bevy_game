use bevy::prelude::*;

use crate::game::core::resources::FontHandles;

#[derive(Resource)]
pub struct GuiThemeUncomputed {
    pub font_main: Handle<Font>,
    pub font_mono: Handle<Font>,
    pub main_padding: f32,
    pub font_size_regular: f32,
    pub font_size_medium: f32,
    pub font_size_title: f32,
}

impl GuiThemeUncomputed {
    pub fn make(font_handles: &FontHandles) -> Self {
        Self {
            font_main: font_handles.cabin.clone(),
            font_mono: font_handles.ibm_plex_mono.clone(),
            main_padding: 10.,
            font_size_regular: 10.,
            font_size_medium: 14.,
            font_size_title: 20.,
        }
    }
}

#[derive(Resource)]
pub struct GuiScale(pub f32);

impl Default for GuiScale {
    fn default() -> Self {
        Self(1.)
    }
}

#[derive(Resource, Default)]
pub struct GuiTheme {
    pub font_main: Handle<Font>,
    pub font_mono: Handle<Font>,
    pub main_padding: f32,
    pub font_size_regular: f32,
    pub font_size_medium: f32,
    pub font_size_title: f32,
}

pub fn compute_gui_theme(theme_uncomputed: &GuiThemeUncomputed, scale: &GuiScale) -> GuiTheme {
    GuiTheme {
        font_main: theme_uncomputed.font_main.clone(),
        font_mono: theme_uncomputed.font_mono.clone(),
        main_padding: scale.0 * theme_uncomputed.main_padding,
        font_size_regular: scale.0 * theme_uncomputed.font_size_regular,
        font_size_medium: scale.0 * theme_uncomputed.font_size_medium,
        font_size_title: scale.0 * theme_uncomputed.font_size_title,
    }
}
