use bevy::prelude::*;

use crate::game::{
    core::{resources::FontHandles, sets::GlobalStartupOrdering},
    gui::{
        resources::GuiTheme,
        sets::{GuiWidgetDuringAddFunctionalComponents, GuiWidgetDuringUpdateFunctionalComponents},
        widgets::text::GuiTextPlugin,
    },
};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .configure_sets(Update,
                (
                    GuiWidgetDuringAddFunctionalComponents,
                    GuiWidgetDuringUpdateFunctionalComponents
                ).chain()
            )
            .add_systems(Startup,
                startup
                    .in_set(GlobalStartupOrdering::Regular)
            )
            .add_plugins(GuiTextPlugin)
        ;
    }
}

fn startup(mut commands: Commands, font_handles: Res<FontHandles>) {
    commands.insert_resource(GuiTheme::make(&font_handles));
}
