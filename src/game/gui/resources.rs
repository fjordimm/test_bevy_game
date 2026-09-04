use bevy::{platform::collections::HashMap, prelude::*, window::SystemCursorIcon};

use crate::game::core::resources::FontHandles;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum CursorIconPriority {
    Regular,
}

#[derive(Resource)]
pub struct CursorIconHandler {
    pub(super) candidates: HashMap<(Entity, SystemCursorIcon), CursorIconPriority>,
}

impl CursorIconHandler {
    pub fn add_candidate(
        &mut self,
        entity: Entity,
        icon: SystemCursorIcon,
        priority: CursorIconPriority,
    ) {
        self.candidates.insert((entity, icon), priority);
    }

    pub fn remove_candidate(&mut self, entity: Entity, icon: SystemCursorIcon) {
        self.candidates.remove(&(entity, icon));
    }
}

// IMPORTANT: If you add a new field to GuiTheme,
//   remember to rescale it in `GuiThemeComputed::compute_from`
//   (if it's numeric and should change based on the gui scale).

#[allow(unused)]
#[derive(Clone)]
pub struct GuiTheme {
    pub font_primary: Handle<Font>,
    pub font_mono: Handle<Font>,
    pub bg_color_main: Color,
    pub content_color_main: Color,
    pub box_shadow: BoxShadow,
    pub padding_main: f32,
    pub padding_minor: f32,
    pub border_radius: f32,
    pub font_size_p: f32,
    pub font_size_h1: f32,
    pub font_size_h2: f32,
    pub button_color_normal: Color,
    pub button_color_hovered: Color,
    pub button_color_pressed: Color,
    pub title_bar_color: Color,
    pub title_bar_button_padding: f32,
    pub title_bar_button_icon_size: f32,
    pub corner_resizer_padding: f32,
    pub corner_resizer_size: f32,
    pub floating_panel_content_min_height: f32,
    pub primary_debug_menu_starting_height: f32,
    pub pause_menu_bg_color: Color,
}

impl GuiTheme {
    pub fn make(font_handles: &FontHandles) -> Self {
        Self {
            font_primary: font_handles.cabin.clone(),
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
            padding_main: 10.0,
            padding_minor: 5.0,
            border_radius: 5.0,
            font_size_p: 12.0,
            font_size_h1: 20.0,
            font_size_h2: 16.0,
            button_color_normal: Color::hsv(185.0, 0.3, 0.25),
            button_color_hovered: Color::hsv(185.0, 0.3, 0.2),
            button_color_pressed: Color::hsv(185.0, 0.3, 0.15),
            title_bar_color: Color::hsv(185.0, 0.3, 0.25),
            title_bar_button_padding: 2.0,
            title_bar_button_icon_size: 16.0,
            corner_resizer_padding: 3.0,
            corner_resizer_size: 10.0,
            floating_panel_content_min_height: 20.0,
            primary_debug_menu_starting_height: 200.0,
            pause_menu_bg_color: Color::hsva(0.0, 0.0, 0.0, 0.75),
        }
    }
}

#[derive(Resource)]
pub struct GuiThemeUncomputed(pub GuiTheme);

#[derive(Resource)]
pub struct GuiScale(pub f32);

impl Default for GuiScale {
    fn default() -> Self {
        Self(1.0)
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
        ret.title_bar_button_padding *= scale.0;
        ret.title_bar_button_icon_size *= scale.0;
        ret.corner_resizer_padding *= scale.0;
        ret.corner_resizer_size *= scale.0;
        ret.floating_panel_content_min_height *= scale.0;
        ret.primary_debug_menu_starting_height *= scale.0;

        Self(ret)
    }
}
