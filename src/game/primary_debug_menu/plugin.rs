use bevy::prelude::*;

use crate::game::{
    core::resources::{GlobalGuiRoot, KeyBindings},
    gui::{
        gui_children,
        resources::GuiThemeComputed,
        widgets::{
            floating_panel::{
                GuiFloatingPanelInterface, GuiFloatingPanelProps, gui_floating_panel,
            },
            text::gui_text_p,
        },
    },
};

pub struct PrimaryDebugMenuPlugin;

impl Plugin for PrimaryDebugMenuPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update,
                spawn_primary_debug_menu
                    .run_if(run_once)
            )
            .add_systems(Update, toggle_debug_menu)
        ;
    }
}

#[derive(Component)]
struct PrimaryDebugMenuTag;

fn spawn_primary_debug_menu(
    mut commands: Commands,
    gui_root: Res<GlobalGuiRoot>,
    theme: Res<GuiThemeComputed>,
) {
    let debug_menu = commands
        .spawn(gui_floating_panel("Primary Debug Menu", GuiFloatingPanelProps {
            starts_active: true,
            starting_content_height: theme.0.primary_debug_menu_starting_height,
            ..default()
        }))
        .insert(gui_children(|p| {
            p.spawn(gui_text_p("what's up buddyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy"));
            p.spawn(gui_text_p("kowabunga yayyyy"));
        }))
        .insert(PrimaryDebugMenuTag)
        .insert(ZIndex(4000))
        .id();

    commands.entity(gui_root.0).add_child(debug_menu);
}

fn toggle_debug_menu(
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut debug_menu_q: Query<GuiFloatingPanelInterface, With<PrimaryDebugMenuTag>>,
) {
    if keys.just_pressed(key_bindings.toggle_debug_menu) {
        debug_menu_q.iter_mut().for_each(|mut debug_menu| {
            debug_menu.set_is_active(!debug_menu.is_active());
        });
    }
}
