use bevy::prelude::*;

use crate::game::{
    core::resources::{GlobalGuiRoot, KeyBindings},
    diagnosis::resources::LagSpikeDiag,
    gui::{
        gui_children,
        resources::GuiThemeComputed,
        widgets::{
            div::{GuiDivCustomStyle, GuiDivProps, GuiDivStyle, gui_div, gui_div_p},
            floating_panel::{
                GuiFloatingPanelInterface, GuiFloatingPanelProps, gui_floating_panel,
            },
            text::{GuiTextInterface, gui_text_h2, gui_text_m},
        },
    },
    playing_state::{coord_rebasing::WorldSpaceEntity, tags::PrimaryCamera},
};

pub struct CheatsMenuPlugin;

impl Plugin for CheatsMenuPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
        ;
    }
}
