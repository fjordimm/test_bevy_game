use bevy::prelude::*;

use crate::game::core::resources::FontHandles;

#[allow(unused)]
#[derive(Clone)]
pub struct GuiTheme {
    pub font_main: Handle<Font>,
    pub font_mono: Handle<Font>,
    pub main_padding: f32,
    pub font_size_regular: f32,
    pub font_size_medium: f32,
    pub font_size_title: f32,
    pub main_bg_color: Color,
    pub main_content_color: Color,
    pub main_box_shadow: BoxShadow,
}

impl GuiTheme {
    pub fn make(font_handles: &FontHandles) -> Self {
        Self {
            font_main: font_handles.cabin.clone(),
            font_mono: font_handles.ibm_plex_mono.clone(),
            main_padding: 10.,
            font_size_regular: 10.,
            font_size_medium: 14.,
            font_size_title: 20.,
            main_bg_color: Color::hsv(185.0, 0.3, 0.15),
            main_content_color: Color::hsv(185.0, 0.075, 0.9),
            main_box_shadow: BoxShadow::new(
                Color::hsva(0.0, 0.0, 0.0, 0.5),
                Val::ZERO,
                Val::ZERO,
                Val::ZERO,
                px(5),
            ),
        }
    }
}

#[derive(Resource)]
pub struct GuiThemeUncomputed(pub GuiTheme);

#[derive(Resource)]
pub struct GuiScale(pub f32);

impl Default for GuiScale {
    fn default() -> Self {
        Self(1.)
    }
}

#[derive(Resource)]
pub struct GuiThemeComputed(pub GuiTheme);

impl GuiThemeComputed {
    pub fn compute_from(theme_uncomputed: &GuiTheme, scale: &GuiScale) -> Self {
        let mut ret = theme_uncomputed.clone();

        ret.main_padding *= scale.0;
        ret.font_size_regular *= scale.0;
        ret.font_size_medium *= scale.0;
        ret.font_size_title *= scale.0;

        Self(ret)
    }
}
