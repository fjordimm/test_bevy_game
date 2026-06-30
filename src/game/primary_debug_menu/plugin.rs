use bevy::prelude::*;

use crate::game::{
    core::resources::GlobalGuiRoot,
    gui::{
        gui_children,
        widgets::{floating_panel::gui_floating_panel, text::gui_text_p},
    },
};

pub struct PrimaryDebugMenuPlugin;

impl Plugin for PrimaryDebugMenuPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update,
                spawn_debug_menu
                    .run_if(run_once)
            )
        ;
    }
}

#[derive(Component)]
struct PrimaryDebugMenuTag;

fn spawn_debug_menu(mut commands: Commands, gui_root: Res<GlobalGuiRoot>) {
    let debug_menu = commands
        .spawn(gui_floating_panel("Primary Debug Menu", default()))
        .insert(gui_children(|p| {
            p.spawn(gui_text_p("what's up buddy"));
        }))
        .insert(PrimaryDebugMenuTag)
        .insert(ZIndex(4000))
        .id();

    commands.entity(gui_root.0).add_child(debug_menu);
}
