use bevy::prelude::*;

use crate::game::core::resources::FontHandles;

// IMPORTANT: If you add a new field to GuiTheme,
//   remember to rescale it in `GuiThemeComputed::compute_from`
//   (if it's numeric and should change based on the gui scale).

#[allow(unused)]
#[derive(Clone)]
pub struct GuiTheme {
    pub font_main: Handle<Font>,
    pub font_mono: Handle<Font>,
    pub bg_color_main: Color,
    pub content_color_main: Color,
    pub box_shadow: BoxShadow,
    pub button_color_normal: Color,
    pub button_color_hovered: Color,
    pub button_color_pressed: Color,
    pub floating_panel_title_bar_color: Color,
    pub pause_menu_bg_color: Color,
    pub padding_main: f32,
    pub padding_minor: f32,
    pub border_radius: f32,
    pub font_size_p: f32,
    pub font_size_h1: f32,
    pub font_size_h2: f32,
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
            button_color_normal: Color::hsv(185.0, 0.3, 0.25),
            button_color_hovered: Color::hsv(185.0, 0.3, 0.2),
            button_color_pressed: Color::hsv(185.0, 0.3, 0.15),
            floating_panel_title_bar_color: Color::hsv(185.0, 0.3, 0.25),
            pause_menu_bg_color: Color::hsva(0.0, 0.0, 0.0, 0.75),
            padding_main: 10.,
            padding_minor: 5.,
            border_radius: 5.,
            font_size_p: 12.,
            font_size_h1: 20.,
            font_size_h2: 16.,
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
        ret.font_size_p *= scale.0;
        ret.font_size_h1 *= scale.0;
        ret.font_size_h2 *= scale.0;

        Self(ret)
    }
}
