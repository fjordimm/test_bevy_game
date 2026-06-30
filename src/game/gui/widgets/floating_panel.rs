/*
    Note: if there are multiple floating panels, they will not order themselves
*/

use bevy::prelude::*;

use crate::game::{
    gui::{
        resources::GuiThemeComputed,
        sets::GuiSystemsOrdering,
        widgets::{button::gui_button, text::gui_text_h2},
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
struct GuiFloatingPanelTitleBarButtonPartTag;

#[derive(Component)]
struct GuiFloatingPanelMinimizeButtonTag;

#[derive(Component)]
struct GuiFloatingPanelXButtonTag;

#[derive(Component)]
struct GuiFloatingPanelMainContentTag;

#[derive(Component)]
struct GuiFloatingPanelMainContentInnerTag;

#[derive(Component)]
struct GuiFloatingPanelCornerResizerTag;

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
        children![
            (
                GuiFloatingPanelTitleBarTag,
                Node::default(),
                children![
                    (GuiFloatingPanelTitleBarMainPartTag, Node::default()),
                    (
                        GuiFloatingPanelTitleBarButtonPartTag,
                        Node::default(),
                        children![
                            (GuiFloatingPanelMinimizeButtonTag, gui_button(default())),
                            (GuiFloatingPanelXButtonTag, gui_button(default()))
                        ]
                    )
                ]
            ),
            (
                GuiFloatingPanelMainContentTag,
                Node::default(),
                children![(GuiFloatingPanelMainContentInnerTag, Node::default())]
            ),
            (GuiFloatingPanelCornerResizerTag, Button, Node::default())
        ],
    )
}

// TODOr
// title_bar
//   title_bar_main_part
//     title_text
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
    title_bar_entity: &Entity,
    title_bar_node: &mut Node,
    title_bar_main_part_entity: &Entity,
    title_bar_main_part_node: &mut Node,
    _title_bar_button_part_node: &mut Node,
    _minimize_button_entity: &Entity,
    _minimize_button_node: &mut Node,
    _x_button_entity: &Entity,
    _x_button_node: &mut Node,
    _main_content_entity: &Entity,
    _main_content_node: &mut Node,
    _main_content_inner_node: &mut Node,
    _corner_resizer_entity: &Entity,
    _corner_resizer_node: &mut Node,
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
        .entity(*title_bar_entity)
        .insert(BackgroundColor(theme.0.floating_panel_title_bar_color));

    title_bar_main_part_node.flex_grow = 1.;
    title_bar_main_part_node.display = Display::Flex;
    title_bar_main_part_node.border_radius = BorderRadius::top_left(px(theme.0.border_radius));
    title_bar_main_part_node.flex_direction = FlexDirection::Row;
    title_bar_main_part_node.justify_content = JustifyContent::FlexStart;
    title_bar_main_part_node.align_items = AlignItems::Center;
    title_bar_main_part_node.padding = UiRect::left(px(theme.0.padding_minor));
    title_bar_main_part_node.column_gap = px(theme.0.padding_minor);

    let title_text = commands.spawn(gui_text_h2(attribs.title.clone())).id();
    commands
        .entity(*title_bar_main_part_entity)
        .add_child(title_text);
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
    title_bar_main_part: Entity,
    title_bar_button_part: Entity,
    minimize_button: Entity,
    x_button: Entity,
    main_content: Entity,
    main_content_inner: Entity,
    corner_resizer: Entity,
}

fn setup_relations(
    mut commands: Commands,
    root_q: Query<(Entity, &Children), With<GuiFloatingPanelAttribs>>,
    title_bar_q: Query<(Entity, &Children), With<GuiFloatingPanelTitleBarTag>>,
    title_bar_main_part_q: Query<Entity, With<GuiFloatingPanelTitleBarMainPartTag>>,
    title_bar_button_part_q: Query<
        (Entity, &Children),
        With<GuiFloatingPanelTitleBarButtonPartTag>,
    >,
    minimize_button_q: Query<Entity, With<GuiFloatingPanelMinimizeButtonTag>>,
    x_button_q: Query<Entity, With<GuiFloatingPanelXButtonTag>>,
    main_content_q: Query<(Entity, &Children), With<GuiFloatingPanelMainContentTag>>,
    main_content_inner_q: Query<Entity, With<GuiFloatingPanelMainContentInnerTag>>,
    corner_resizer_q: Query<Entity, With<GuiFloatingPanelCornerResizerTag>>,
) {
    root_q.iter().for_each(|(root, root_children)| {
        let mut relations = GuiFloatingPanelRelations {
            title_bar: Entity::PLACEHOLDER,
            title_bar_main_part: Entity::PLACEHOLDER,
            title_bar_button_part: Entity::PLACEHOLDER,
            minimize_button: Entity::PLACEHOLDER,
            x_button: Entity::PLACEHOLDER,
            main_content: Entity::PLACEHOLDER,
            main_content_inner: Entity::PLACEHOLDER,
            corner_resizer: Entity::PLACEHOLDER,
        };

        root_children.iter().for_each(|child| {
            if let Ok((title_bar, title_bar_children)) = title_bar_q.get(child) {
                relations.title_bar = title_bar;

                title_bar_children.iter().for_each(|child| {
                    if let Ok(title_bar_main_part) = title_bar_main_part_q.get(child) {
                        relations.title_bar_main_part = title_bar_main_part;
                    }

                    if let Ok((title_bar_button_part, title_bar_button_part_children)) =
                        title_bar_button_part_q.get(child)
                    {
                        relations.title_bar_button_part = title_bar_button_part;

                        title_bar_button_part_children.iter().for_each(|child| {
                            if let Ok(minimize_button) = minimize_button_q.get(child) {
                                relations.minimize_button = minimize_button;
                            }

                            if let Ok(x_button) = x_button_q.get(child) {
                                relations.x_button = x_button;
                            }
                        });
                    }
                });
            }

            if let Ok((main_content, main_content_children)) = main_content_q.get(child) {
                relations.main_content = main_content;

                main_content_children.iter().for_each(|child| {
                    if let Ok(main_content_inner) = main_content_inner_q.get(child) {
                        relations.main_content_inner = main_content_inner;
                    }
                });
            }

            if let Ok(corner_resizer) = corner_resizer_q.get(child) {
                relations.corner_resizer = corner_resizer;
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
            Without<GuiFloatingPanelTitleBarMainPartTag>,
            Without<GuiFloatingPanelTitleBarButtonPartTag>,
            Without<GuiFloatingPanelMinimizeButtonTag>,
            Without<GuiFloatingPanelXButtonTag>,
            Without<GuiFloatingPanelMainContentTag>,
            Without<GuiFloatingPanelMainContentInnerTag>,
            Without<GuiFloatingPanelCornerResizerTag>,
        ),
    >,
    mut title_bar_main_part_q: Query<
        (Entity, &mut Node),
        (
            With<GuiFloatingPanelTitleBarMainPartTag>,
            Without<GuiFloatingPanelAttribs>,
            Without<GuiFloatingPanelTitleBarTag>,
            Without<GuiFloatingPanelTitleBarButtonPartTag>,
            Without<GuiFloatingPanelMinimizeButtonTag>,
            Without<GuiFloatingPanelXButtonTag>,
            Without<GuiFloatingPanelMainContentTag>,
            Without<GuiFloatingPanelMainContentInnerTag>,
            Without<GuiFloatingPanelCornerResizerTag>,
        ),
    >,
    mut title_bar_button_part_q: Query<
        &mut Node,
        (
            With<GuiFloatingPanelTitleBarButtonPartTag>,
            Without<GuiFloatingPanelAttribs>,
            Without<GuiFloatingPanelTitleBarTag>,
            Without<GuiFloatingPanelTitleBarMainPartTag>,
            Without<GuiFloatingPanelMinimizeButtonTag>,
            Without<GuiFloatingPanelXButtonTag>,
            Without<GuiFloatingPanelMainContentTag>,
            Without<GuiFloatingPanelMainContentInnerTag>,
            Without<GuiFloatingPanelCornerResizerTag>,
        ),
    >,
    mut minimize_button_q: Query<
        (Entity, &mut Node),
        (
            With<GuiFloatingPanelMinimizeButtonTag>,
            Without<GuiFloatingPanelAttribs>,
            Without<GuiFloatingPanelTitleBarTag>,
            Without<GuiFloatingPanelTitleBarMainPartTag>,
            Without<GuiFloatingPanelTitleBarButtonPartTag>,
            Without<GuiFloatingPanelXButtonTag>,
            Without<GuiFloatingPanelMainContentTag>,
            Without<GuiFloatingPanelMainContentInnerTag>,
            Without<GuiFloatingPanelCornerResizerTag>,
        ),
    >,
    mut x_button_q: Query<
        (Entity, &mut Node),
        (
            With<GuiFloatingPanelXButtonTag>,
            Without<GuiFloatingPanelAttribs>,
            Without<GuiFloatingPanelTitleBarTag>,
            Without<GuiFloatingPanelTitleBarMainPartTag>,
            Without<GuiFloatingPanelTitleBarButtonPartTag>,
            Without<GuiFloatingPanelMinimizeButtonTag>,
            Without<GuiFloatingPanelMainContentTag>,
            Without<GuiFloatingPanelMainContentInnerTag>,
            Without<GuiFloatingPanelCornerResizerTag>,
        ),
    >,
    mut main_content_q: Query<
        (Entity, &mut Node),
        (
            With<GuiFloatingPanelMainContentTag>,
            Without<GuiFloatingPanelAttribs>,
            Without<GuiFloatingPanelTitleBarTag>,
            Without<GuiFloatingPanelTitleBarMainPartTag>,
            Without<GuiFloatingPanelTitleBarButtonPartTag>,
            Without<GuiFloatingPanelMinimizeButtonTag>,
            Without<GuiFloatingPanelXButtonTag>,
            Without<GuiFloatingPanelMainContentInnerTag>,
            Without<GuiFloatingPanelCornerResizerTag>,
        ),
    >,
    mut main_content_inner_q: Query<
        &mut Node,
        (
            With<GuiFloatingPanelMainContentInnerTag>,
            Without<GuiFloatingPanelAttribs>,
            Without<GuiFloatingPanelTitleBarTag>,
            Without<GuiFloatingPanelTitleBarMainPartTag>,
            Without<GuiFloatingPanelTitleBarButtonPartTag>,
            Without<GuiFloatingPanelMinimizeButtonTag>,
            Without<GuiFloatingPanelXButtonTag>,
            Without<GuiFloatingPanelMainContentTag>,
            Without<GuiFloatingPanelCornerResizerTag>,
        ),
    >,
    mut corner_resizer_q: Query<
        (Entity, &mut Node),
        (
            With<GuiFloatingPanelCornerResizerTag>,
            Without<GuiFloatingPanelAttribs>,
            Without<GuiFloatingPanelTitleBarTag>,
            Without<GuiFloatingPanelTitleBarMainPartTag>,
            Without<GuiFloatingPanelTitleBarButtonPartTag>,
            Without<GuiFloatingPanelMinimizeButtonTag>,
            Without<GuiFloatingPanelXButtonTag>,
            Without<GuiFloatingPanelMainContentTag>,
            Without<GuiFloatingPanelMainContentInnerTag>,
        ),
    >,
) {
    root_q
        .iter_mut()
        .for_each(|(relations, attribs, state, mut root_node)| {
            let (title_bar_entity, mut title_bar_node) =
                alrro!(title_bar_q.get_mut(relations.title_bar));
            let (title_bar_main_part_entity, mut title_bar_main_part_node) =
                alrro!(title_bar_main_part_q.get_mut(relations.title_bar_main_part));
            let mut title_bar_button_part_node =
                alrro!(title_bar_button_part_q.get_mut(relations.title_bar_button_part));
            let (minimize_button_entity, mut minimize_button_node) =
                alrro!(minimize_button_q.get_mut(relations.minimize_button));
            let (x_button_entity, mut x_button_node) =
                alrro!(x_button_q.get_mut(relations.x_button));
            let (main_content_entity, mut main_content_node) =
                alrro!(main_content_q.get_mut(relations.main_content));
            let mut main_content_inner_node =
                alrro!(main_content_inner_q.get_mut(relations.main_content_inner));
            let (corner_resizer_entity, mut corner_resizer_node) =
                alrro!(corner_resizer_q.get_mut(relations.corner_resizer));

            apply_style(
                &mut commands,
                &theme,
                &attribs,
                &state,
                &mut root_node,
                &title_bar_entity,
                &mut title_bar_node,
                &title_bar_main_part_entity,
                &mut title_bar_main_part_node,
                &mut title_bar_button_part_node,
                &minimize_button_entity,
                &mut minimize_button_node,
                &x_button_entity,
                &mut x_button_node,
                &main_content_entity,
                &mut main_content_node,
                &mut main_content_inner_node,
                &corner_resizer_entity,
                &mut corner_resizer_node,
            );
        });
}

fn update_style_on_theme_change() {}

fn update_style_on_state_change() {}
