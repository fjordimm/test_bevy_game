use bevy::prelude::*;

use crate::game::gui::gui_button;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update, gui_button::update_style);
    }
}
