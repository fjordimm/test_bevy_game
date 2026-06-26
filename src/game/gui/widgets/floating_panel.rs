/*
    Note: if there are multiple floating panels, they will not order themselves
*/

use bevy::prelude::*;

use crate::game::gui::resources::GuiThemeComputed;

#[allow(unused)]
pub struct GuiFloatingPanelProps {
    pub starts_active: bool,
    pub starts_minimized: bool,
}

impl Default for GuiFloatingPanelProps {
    fn default() -> Self {
        Self {
            starts_active: true,
            starts_minimized: false,
        }
    }
}

#[derive(Component)]
struct GuiFloatingPanelAttribs {
    title: String,
}

#[derive(Component)]
struct GuiFloatingPanelState {
    is_active: bool,
    is_minimized: bool,
}

#[derive(Component)]
struct GuiFloatingPanelTitleBarTag;

#[allow(unused)]
pub fn gui_floating_panel(title: impl Into<String>, props: GuiFloatingPanelProps) -> impl Bundle {
    (
        GuiFloatingPanelAttribs {
            title: title.into(),
        },
        GuiFloatingPanelState {
            is_active: props.starts_active,
            is_minimized: props.starts_minimized,
        },
        Node::default(),
        children![(GuiFloatingPanelTitleBarTag, Node::default())],
    )
}

// title_bar
//   title_bar_main_part
//   title_bar_button_part
//     minimize_button
//     x_button
// main_content
//   main_content_inner
// corner_resizer

fn set_style(
    commands: &mut Commands,
    theme: &GuiThemeComputed,
    attribs: &GuiFloatingPanelAttribs,
    state: &GuiFloatingPanelState,
    root_node: &mut Node,
    title_bar_node: &mut Node,
) {
}

pub struct GuiFloatingPanelPlugin;

impl Plugin for GuiFloatingPanelPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update, handle_gui_children)
            .add_systems(Update, update_style_on_attrib_change)
            .add_systems(Update,
                update_style_on_theme_change
                    .run_if(resource_changed::<GuiThemeComputed>)
            )
            .add_systems(Update, update_style_from_state_change)
        ;
    }
}

fn handle_gui_children() {}

fn update_style_on_attrib_change(
    mut commands: Commands,
    theme: Res<GuiThemeComputed>,
    mut root_q: Query<(
        &GuiFloatingPanelAttribs,
        &GuiFloatingPanelState,
        Entity,
        &mut Node,
    )>,
    mut title_bar_q: Query<(Entity, &mut Node), With<GuiFloatingPanelTitleBarTag>>,
) {
}

fn update_style_on_theme_change() {}

fn update_style_from_state_change() {}
