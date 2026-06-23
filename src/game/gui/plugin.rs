use bevy::prelude::*;

use crate::game::gui::{resources::GuiTheme, widgets::text::GuiTextPlugin};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .insert_resource(GuiTheme::default())
            .add_plugins(GuiTextPlugin)
        ;
    }
}
