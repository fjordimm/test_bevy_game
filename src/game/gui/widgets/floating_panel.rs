/*
    Note: if there are multiple floating panels, they will not order themselves
*/

use bevy::prelude::*;

use crate::game::gui::{resources::GuiThemeComputed, widgets::text::gui_text_h2};

#[allow(unused)]
pub struct GuiFloatingPanelProps {
    pub starts_active: bool,
    pub starts_minimized: bool,
    pub starting_pos_x: f32,
    pub starting_pos_y: f32,
}

impl Default for GuiFloatingPanelProps {
    fn default() -> Self {
        Self {
            starts_active: true,
            starts_minimized: false,
            starting_pos_x: 10.0,
            starting_pos_y: 10.0,
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
    pos_x: f32,
    pos_y: f32,
}

#[derive(Component)]
struct GuiFloatingPanelTitleBarTag;

#[allow(unused)]
pub fn gui_floating_panel(title: impl Into<String>, props: GuiFloatingPanelProps) -> impl Bundle {
    let title = title.into();

    (
        GuiFloatingPanelAttribs {
            title: title.clone(),
        },
        GuiFloatingPanelState {
            is_active: props.starts_active,
            is_minimized: props.starts_minimized,
            pos_x: props.starting_pos_x,
            pos_y: props.starting_pos_y,
        },
        Node::default(),
        children![(
            GuiFloatingPanelTitleBarTag,
            Node::default(),
            children![(gui_text_h2(title))] // TODO: change
        )],
    )
}

// TODOr
// title_bar
//   title_bar_main_part
//   title_bar_button_part
//     minimize_button
//     x_button
// main_content
//   main_content_inner
// corner_resizer

fn apply_style(
    _commands: &mut Commands,
    theme: &GuiThemeComputed,
    attribs: &GuiFloatingPanelAttribs,
    state: &GuiFloatingPanelState,
    root_node: &mut Node,
    title_bar_node: &mut Node,
    title_node: &mut Node,
) {
    root_node.position_type = PositionType::Absolute;
    root_node.left = px(state.pos_x);
    root_node.top = px(state.pos_y);
    root_node.display = what_display(&state);
    root_node.border_radius = BorderRadius::all(px(theme.0.border_radius));
    root_node.flex_direction = FlexDirection::Column;
    root_node.justify_content = JustifyContent::FlexStart;
    root_node.align_items = AlignItems::Stretch;

    title_bar_node.display = Display::Flex;
    title_bar_node.border_radius = BorderRadius::top(px(theme.0.border_radius));
    title_bar_node.flex_direction = FlexDirection::Row;
    title_bar_node.justify_content = JustifyContent::FlexStart;
    title_bar_node.align_items = AlignItems::Stretch;
    title_bar_node.padding = UiRect::all(px(theme.0.padding_minor));
    title_bar_node.column_gap = px(theme.0.padding_main);
}

fn what_display(state: &GuiFloatingPanelState) -> Display {
    match state.is_active {
        true => Display::Flex,
        false => Display::None,
    }
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
    mut title_bar_q: Query<
        (&ChildOf, Entity, &mut Node),
        (
            With<GuiFloatingPanelTitleBarTag>,
            Without<GuiFloatingPanelAttribs>,
        ),
    >,
) {
}

fn update_style_on_theme_change() {}

fn update_style_from_state_change() {}
