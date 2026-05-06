use crate::game::core::global_resources::Fonts;
use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub enum FontOption {
    Sans,
    Serif,
    Mono,
}

pub fn make_fonts_resource(asset_server: &Res<AssetServer>) -> Fonts {
    Fonts {
        sans: asset_server.load("fonts/Cabin-VariableFont_wdth,wght.ttf"),
        serif: asset_server.load("fonts/SortsMillGoudy-Regular.ttf"),
        mono: asset_server.load("fonts/IBMPlexMono-Regular.ttf"),
    }
}

pub fn apply_fonts(
    fonts_res: Res<Fonts>,
    mut font_q: Query<(&mut TextFont, &FontOption), (Added<Text>, With<FontOption>)>,
) {
    font_q.iter_mut().for_each(|(mut text_font, gui_font)| {
        text_font.font = match gui_font {
            FontOption::Sans => fonts_res.sans.clone(),
            FontOption::Serif => fonts_res.serif.clone(),
            FontOption::Mono => fonts_res.mono.clone(),
        };
    });
}
