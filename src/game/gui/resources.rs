use bevy::prelude::*;

#[derive(Resource)]
pub struct GuiTheme {
    pub main_padding: f32,
    pub font_size_regular: f32,
    pub font_size_medium: f32,
    pub font_size_title: f32,
}

impl Default for GuiTheme {
    fn default() -> Self {
        Self {
            main_padding: 10.0,
            font_size_regular: 10.0,
            font_size_medium: 14.0,
            font_size_title: 20.0,
        }
    }
}
