use crate::game::core::global_resources::GlobalFonts;
use bevy::prelude::*;

#[derive(Component)]
pub enum GuiFont {
    Sans,
    Serif,
}

pub fn make_global_fonts_resource(asset_server: Res<AssetServer>) -> GlobalFonts {
    GlobalFonts {
        sans: asset_server.load("fonts/Cabin-VariableFont_wdth,wght.ttf"),
        serif: asset_server.load("fonts/SortsMillGoudy-Regular.ttf"),
    }
}

pub fn apply_gui_fonts(
    global_fonts: Res<GlobalFonts>,
    mut query: Query<(&mut TextFont, &GuiFont), (Added<Text>, With<GuiFont>)>,
) {
    for (mut text_font, gui_font) in &mut query {
        text_font.font = match gui_font {
            GuiFont::Sans => global_fonts.sans.clone(),
            GuiFont::Serif => global_fonts.serif.clone(),
        };
    }
}
