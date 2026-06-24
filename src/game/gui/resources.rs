use bevy::prelude::*;

use crate::game::core::resources::FontHandles;

#[allow(unused)]
#[derive(Resource)]
pub struct GuiTheme {
    pub font_main: Handle<Font>,
    pub main_padding: f32,
    pub font_size_regular: f32,
    pub font_size_medium: f32,
    pub font_size_title: f32,
}

impl GuiTheme {
    pub fn make(font_handles: &FontHandles) -> Self {
        Self {
            font_main: font_handles.sorts_mill_goudy.clone(),
            main_padding: 10.0,
            font_size_regular: 10.0,
            font_size_medium: 14.0,
            font_size_title: 20.0,
        }
    }
}
