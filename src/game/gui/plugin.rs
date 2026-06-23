use bevy::prelude::*;

use crate::game::gui::{
    resources::GuiTheme,
    sets::{GuiWidgetDuringAddFunctionalComponents, GuiWidgetDuringUpdateFunctionalComponents},
    widgets::text::GuiTextPlugin,
};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .configure_sets(Update,
                (GuiWidgetDuringAddFunctionalComponents, GuiWidgetDuringUpdateFunctionalComponents).chain()
            )
            .insert_resource(GuiTheme::default())
            .add_plugins(GuiTextPlugin)
        ;
    }
}
