use bevy::prelude::*;

use crate::game::core::resources::FontHandles;

#[allow(unused)]
#[derive(Clone)]
pub struct GuiTheme {
    pub font_main: Handle<Font>,
    pub font_mono: Handle<Font>,
    pub bg_color_main: Color,
    pub content_color_main: Color,
    pub box_shadow: BoxShadow,
    pub padding_main: f32,
    pub border_radius: f32,
    pub font_size_regular: f32,
    pub font_size_medium: f32,
    pub font_size_title: f32,
}

impl GuiTheme {
    pub fn make(font_handles: &FontHandles) -> Self {
        Self {
            font_main: font_handles.cabin.clone(),
            font_mono: font_handles.ibm_plex_mono.clone(),
            bg_color_main: Color::hsv(185.0, 0.3, 0.15),
            content_color_main: Color::hsv(185.0, 0.075, 0.9),
            box_shadow: BoxShadow::new(
                Color::hsva(0.0, 0.0, 0.0, 0.5),
                Val::ZERO,
                Val::ZERO,
                Val::ZERO,
                px(5),
            ),
            padding_main: 10.,
            border_radius: 5.,
            font_size_regular: 10.,
            font_size_medium: 14.,
            font_size_title: 20.,
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

        ret.padding_main *= scale.0;
        ret.border_radius *= scale.0;
        ret.font_size_regular *= scale.0;
        ret.font_size_medium *= scale.0;
        ret.font_size_title *= scale.0;

        Self(ret)
    }
}
