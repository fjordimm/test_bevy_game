use bevy::prelude::*;

use crate::game::{
    core::{resources::FontHandles, sets::GlobalStartupOrdering},
    gui::{
        resources::{GuiScale, GuiTheme, GuiThemeUncomputed, compute_gui_theme},
        widgets::{div::GuiDivPlugin, text::GuiTextPlugin},
    },
};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Startup,
                startup
                    .in_set(GlobalStartupOrdering::Regular)
            )
            .add_systems(Update,
                update_gui_globals
                    .run_if(resource_changed::<GuiScale>)
            )
            .add_plugins(GuiTextPlugin)
            .add_plugins(GuiDivPlugin)
        ;
    }
}

fn startup(mut commands: Commands, font_handles: Res<FontHandles>) {
    commands.insert_resource(GuiThemeUncomputed::make(&font_handles));
    commands.insert_resource(GuiScale::default());
    commands.insert_resource(GuiTheme::default());
}

fn update_gui_globals(
    theme_uncomputed: Res<GuiThemeUncomputed>,
    scale: Res<GuiScale>,
    mut theme: ResMut<GuiTheme>,
) {
    *theme = compute_gui_theme(&theme_uncomputed, &scale);
}
