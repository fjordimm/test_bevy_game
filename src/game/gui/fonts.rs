use crate::game::core::global_resources::GlobalFonts;
use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub enum GameFont {
    Sans,
    Serif,
    Mono,
}

pub fn make_global_fonts_resource(asset_server: Res<AssetServer>) -> GlobalFonts {
    GlobalFonts {
        sans: asset_server.load("fonts/Cabin-VariableFont_wdth,wght.ttf"),
        serif: asset_server.load("fonts/SortsMillGoudy-Regular.ttf"),
        mono: asset_server.load("fonts/IBMPlexMono-Regular.ttf"),
    }
}

pub fn apply_gui_fonts(
    global_fonts: Res<GlobalFonts>,
    mut query: Query<(&mut TextFont, &GameFont), (Added<Text>, With<GameFont>)>,
) {
    for (mut text_font, gui_font) in &mut query {
        text_font.font = match gui_font {
            GameFont::Sans => global_fonts.sans.clone(),
            GameFont::Serif => global_fonts.serif.clone(),
            GameFont::Mono => global_fonts.mono.clone(),
        };
    }
}
