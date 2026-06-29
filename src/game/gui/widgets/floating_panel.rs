/*
    Note: if there are multiple floating panels, they will not order themselves
*/

use bevy::prelude::*;

use crate::game::{
    gui::{
        resources::GuiThemeComputed,
        sets::GuiSystemsOrdering,
        widgets::text::{GuiTextSetContent, gui_text_h2},
    },
    util::alrro,
};

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

#[derive(Component)]
struct GuiFloatingPanelTitleBarMainPartTag;

#[derive(Component)]
struct GuiFloatingPanelTitleTextTag;

#[derive(Component)]
struct GuiFloatingPanelTitleBarButtonPartTag;

#[derive(Component)]
struct GuiFloatingPanelMinimizeButton;

#[derive(Component)]
struct GuiFloatingPanelXButton;

#[derive(Component)]
struct GuiFloatingPanelMainContent;

#[derive(Component)]
struct GuiFloatingPanelMainContentInner;

#[derive(Component)]
struct GuiFloatingPanelCornerResizer;

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
            children![(GuiFloatingPanelTitleTextTag, gui_text_h2(String::new()))]
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
    commands: &mut Commands,
    theme: &GuiThemeComputed,
    attribs: &GuiFloatingPanelAttribs,
    state: &GuiFloatingPanelState,
    root_node: &mut Node,
    title_bar: &Entity,
    title_bar_node: &mut Node,
    title_text: &Entity,
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
    commands
        .entity(*title_bar)
        .insert(BackgroundColor(theme.0.floating_panel_title_bar_color));

    commands.entity(*title_text).trigger(|e| GuiTextSetContent {
        entity: e,
        content: attribs.title.clone(),
    });
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
            .add_systems(Update,
                setup_relations
                    .in_set(GuiSystemsOrdering::SetupRelations)
            )
            .add_systems(Update,
                handle_gui_children
                    .in_set(GuiSystemsOrdering::HandleGuiChildren)
            )
            .add_systems(Update,
                update_style_on_init_or_attrib_change
                    .in_set(GuiSystemsOrdering::UpdateStyle)
            )
            .add_systems(Update,
                update_style_on_theme_change
                    .run_if(resource_changed::<GuiThemeComputed>)
                    .in_set(GuiSystemsOrdering::UpdateStyle)
            )
            .add_systems(Update,
                update_style_on_state_change
                    .in_set(GuiSystemsOrdering::UpdateStyle)
            )
        ;
    }
}

#[derive(Component)]
struct GuiFloatingPanelRelations {
    title_bar: Entity,
    title_text: Entity,
}

fn setup_relations(
    mut commands: Commands,
    root_q: Query<
        (Entity, &Children),
        (
            With<GuiFloatingPanelAttribs>,
            Without<GuiFloatingPanelRelations>,
        ),
    >,
    title_bar_q: Query<
        (Entity, &Children),
        (
            With<GuiFloatingPanelTitleBarTag>,
            Without<GuiFloatingPanelAttribs>,
            Without<GuiFloatingPanelTitleTextTag>,
        ),
    >,
    title_text_q: Query<
        Entity,
        (
            With<GuiFloatingPanelTitleTextTag>,
            Without<GuiFloatingPanelAttribs>,
            Without<GuiFloatingPanelTitleBarTag>,
        ),
    >,
) {
    root_q.iter().for_each(|(root, root_children)| {
        let mut relations = GuiFloatingPanelRelations {
            title_bar: Entity::PLACEHOLDER,
            title_text: Entity::PLACEHOLDER,
        };

        root_children.iter().for_each(|child| {
            if let Ok((title_bar, title_bar_children)) = title_bar_q.get(child) {
                relations.title_bar = title_bar;

                title_bar_children.iter().for_each(|child| {
                    if let Ok(title_text) = title_text_q.get(child) {
                        relations.title_text = title_text;
                    }
                });
            }
        });

        commands.entity(root).insert(relations);
    });
}

fn handle_gui_children() {}

fn update_style_on_init_or_attrib_change(
    mut commands: Commands,
    theme: Res<GuiThemeComputed>,
    mut root_q: Query<
        (
            &GuiFloatingPanelRelations,
            &GuiFloatingPanelAttribs,
            &GuiFloatingPanelState,
            &mut Node,
        ),
        Or<(
            Added<GuiFloatingPanelRelations>,
            Changed<GuiFloatingPanelAttribs>,
        )>,
    >,
    mut title_bar_q: Query<
        (Entity, &mut Node),
        (
            With<GuiFloatingPanelTitleBarTag>,
            Without<GuiFloatingPanelAttribs>,
            Without<GuiFloatingPanelTitleTextTag>,
        ),
    >,
    title_text_q: Query<
        Entity,
        (
            With<GuiFloatingPanelTitleTextTag>,
            Without<GuiFloatingPanelAttribs>,
            Without<GuiFloatingPanelTitleBarTag>,
        ),
    >,
) {
    root_q
        .iter_mut()
        .for_each(|(relations, attribs, state, mut root_node)| {
            let (title_bar, mut title_bar_node) = alrro!(title_bar_q.get_mut(relations.title_bar));
            let title_text = alrro!(title_text_q.get(relations.title_text));

            apply_style(
                &mut commands,
                &theme,
                &attribs,
                &state,
                &mut root_node,
                &title_bar,
                &mut title_bar_node,
                &title_text,
            );
        });
}

fn update_style_on_theme_change() {}

fn update_style_on_state_change() {}
