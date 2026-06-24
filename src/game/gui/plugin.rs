use bevy::prelude::*;

use crate::game::{
    core::{resources::FontHandles, sets::GlobalStartupOrdering},
    gui::{
        resources::{GuiScale, GuiTheme, GuiThemeComputed, GuiThemeUncomputed},
        widgets::{
            button::GuiButtonPlugin, div::GuiDivPlugin, screen_div::GuiScreenDivPlugin,
            text::GuiTextPlugin,
        },
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
                update_gui_theme_computed
                    .run_if(resource_changed::<GuiScale>.or(resource_changed::<GuiThemeUncomputed>))
            )
            .add_plugins(GuiDivPlugin)
            .add_plugins(GuiScreenDivPlugin)
            .add_plugins(GuiTextPlugin)
            .add_plugins(GuiButtonPlugin)
        ;
    }
}

fn startup(mut commands: Commands, font_handles: Res<FontHandles>) {
    let gui_theme_uncomputed = GuiThemeUncomputed(GuiTheme::make(&font_handles));
    let gui_scale = GuiScale::default();

    commands.insert_resource(GuiThemeComputed::compute_from(
        &gui_theme_uncomputed.0,
        &gui_scale,
    ));
    commands.insert_resource(gui_theme_uncomputed);
    commands.insert_resource(gui_scale);
}

fn update_gui_theme_computed(
    theme_uncomputed: Res<GuiThemeUncomputed>,
    scale: Res<GuiScale>,
    mut theme: ResMut<GuiThemeComputed>,
) {
    *theme = GuiThemeComputed::compute_from(&theme_uncomputed.0, &scale);
}
