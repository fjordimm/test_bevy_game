use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_plugins(EguiPlugin::default());
    }
}
