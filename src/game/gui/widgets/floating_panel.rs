/*
    Note: if there are multiple floating panels, they will not order themselves
*/

use bevy::{prelude::*, window::SystemCursorIcon};
use bevy_ecs::query::QueryData;

use crate::game::{
    gui::{
        GuiChildren,
        resources::{CursorIconHandler, CursorIconPriority, GuiThemeComputed},
        sets::GuiSystemsOrdering,
        widgets::{
            button::{GuiButtonProps, GuiButtonStyle, gui_button},
            icon::{GuiIconIcon, GuiIconProps, gui_icon},
            text::{GuiTextProps, GuiTextSize, gui_text},
        },
    },
    util::{alrmo, alrro},
};

#[allow(unused)]
pub struct GuiFloatingPanelProps {
    pub starts_active: bool,
    pub starts_minimized: bool,
    pub starting_pos_x: f32,
    pub starting_pos_y: f32,
    pub starting_content_width: f32,
    pub starting_content_height: f32,
}

impl Default for GuiFloatingPanelProps {
    fn default() -> Self {
        Self {
            starts_active: true,
            starts_minimized: false,
            starting_pos_x: 10.0,
            starting_pos_y: 10.0,
            starting_content_width: 0.,
            starting_content_height: 0.,
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
    size_x: f32,
    size_y: f32,
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
            size_x: props.starting_content_width,
            size_y: props.starting_content_height,
        },
        Node::default(),
        children![
            (
                GuiFloatingPanelTitleBarTag,
                Button,
                Node::default(),
                children![
                    (GuiFloatingPanelTitleBarMainPartTag, Node::default()),
                    (
                        GuiFloatingPanelTitleBarButtonPartTag,
                        Node::default(),
                        children![
                            (
                                GuiFloatingPanelMinimizeButtonTag,
                                gui_button(GuiButtonProps {
                                    button_style: GuiButtonStyle::TitleBarButton
                                })
                            ),
                            (
                                GuiFloatingPanelXButtonTag,
                                gui_button(GuiButtonProps {
                                    button_style: GuiButtonStyle::TitleBarButton
                                })
                            )
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

fn apply_style(
    commands: &mut Commands,
    theme: &GuiThemeComputed,
    attribs: &GuiFloatingPanelAttribs,
    state: &GuiFloatingPanelState,
    root_entity: &Entity,
    root_node: &mut Node,
    title_bar_entity: &Entity,
    title_bar_node: &mut Node,
    title_bar_main_part_entity: &Entity,
    title_bar_main_part_node: &mut Node,
    title_bar_button_part_node: &mut Node,
    minimize_button_entity: &Entity,
    x_button_entity: &Entity,
    main_content_node: &mut Node,
    main_content_inner_entity: &Entity,
    main_content_inner_node: &mut Node,
    corner_resizer_entity: &Entity,
    corner_resizer_node: &mut Node,
) {
    root_node.position_type = PositionType::Absolute;
    root_node.left = px(state.pos_x);
    root_node.top = px(state.pos_y);
    root_node.display = what_display_for_root(&state);
    root_node.border_radius = BorderRadius::all(px(theme.0.border_radius));
    root_node.flex_direction = FlexDirection::Column;
    root_node.justify_content = JustifyContent::FlexStart;
    root_node.align_items = AlignItems::Stretch;
    commands
        .entity(*root_entity)
        .insert(BackgroundColor(theme.0.bg_color_main))
        .insert(theme.0.box_shadow.clone());

    title_bar_node.display = Display::Flex;
    title_bar_node.border_radius = BorderRadius::top(px(theme.0.border_radius));
    title_bar_node.flex_direction = FlexDirection::Row;
    title_bar_node.justify_content = JustifyContent::FlexStart;
    title_bar_node.align_items = AlignItems::Stretch;
    title_bar_node.padding = UiRect::all(px(theme.0.padding_minor));
    title_bar_node.column_gap = px(theme.0.padding_main);
    commands
        .entity(*title_bar_entity)
        .insert(BackgroundColor(theme.0.title_bar_color));

    title_bar_main_part_node.flex_grow = 1.;
    title_bar_main_part_node.display = Display::Flex;
    title_bar_node.border_radius = what_border_radius_for_title_bar(&theme, &state);
    title_bar_main_part_node.overflow = Overflow::hidden();
    title_bar_main_part_node.flex_direction = FlexDirection::Row;
    title_bar_main_part_node.justify_content = JustifyContent::FlexStart;
    title_bar_main_part_node.align_items = AlignItems::Center;
    title_bar_main_part_node.padding = UiRect::left(px(theme.0.padding_minor));
    title_bar_main_part_node.column_gap = px(theme.0.padding_minor);

    let title_text = commands
        .spawn(gui_text(
            attribs.title.clone(),
            GuiTextProps {
                size: GuiTextSize::H2,
                wraps: false,
                ..default()
            },
        ))
        .id();
    commands
        .entity(*title_bar_main_part_entity)
        .despawn_children()
        .add_child(title_text);

    title_bar_button_part_node.display = Display::Flex;
    title_bar_button_part_node.border_radius = BorderRadius::top_right(px(theme.0.border_radius));
    title_bar_button_part_node.flex_direction = FlexDirection::Row;
    title_bar_button_part_node.justify_content = JustifyContent::FlexStart;
    title_bar_button_part_node.align_items = AlignItems::Center;
    title_bar_button_part_node.column_gap = px(theme.0.padding_minor);

    let minimize_button_icon = commands
        .spawn(gui_icon(
            GuiIconIcon::Minimize,
            theme.0.title_bar_button_icon_size,
            theme.0.title_bar_button_icon_size,
            GuiIconProps::default(),
        ))
        .id();
    commands
        .entity(*minimize_button_entity)
        .despawn_children()
        .add_child(minimize_button_icon);

    let x_button_icon = commands
        .spawn(gui_icon(
            GuiIconIcon::X,
            theme.0.title_bar_button_icon_size,
            theme.0.title_bar_button_icon_size,
            GuiIconProps::default(),
        ))
        .id();
    commands
        .entity(*x_button_entity)
        .despawn_children()
        .add_child(x_button_icon);

    main_content_node.display = what_display_for_main_content(&state);
    main_content_node.width = what_width_for_main_content(&state);
    main_content_node.height = what_height_for_main_content(&state);
    main_content_node.border_radius = BorderRadius::bottom(px(theme.0.border_radius));
    main_content_node.flex_direction = FlexDirection::Column;
    main_content_node.justify_content = JustifyContent::FlexStart;
    main_content_node.align_items = AlignItems::FlexStart;

    main_content_inner_node.flex_grow = 1.;
    main_content_inner_node.align_self = AlignSelf::Stretch;
    main_content_inner_node.overflow = Overflow::hidden();
    main_content_inner_node.display = Display::Flex;
    main_content_inner_node.flex_direction = FlexDirection::Column;
    main_content_inner_node.justify_content = JustifyContent::FlexStart;
    main_content_inner_node.align_items = AlignItems::FlexStart;
    main_content_inner_node.padding = UiRect::all(px(theme.0.padding_main));
    main_content_inner_node.row_gap = px(theme.0.padding_main);
    commands
        .entity(*main_content_inner_entity)
        .insert(ScrollPosition::default());

    corner_resizer_node.position_type = PositionType::Absolute;
    corner_resizer_node.right = px(theme.0.corner_resizer_padding);
    corner_resizer_node.bottom = px(theme.0.corner_resizer_padding);
    corner_resizer_node.display = what_display_for_corner_resizer(&state);
    corner_resizer_node.flex_direction = FlexDirection::Column;
    corner_resizer_node.justify_content = JustifyContent::Center;
    corner_resizer_node.align_items = AlignItems::Center;

    let corner_resizer_icon = commands
        .spawn(gui_icon(
            GuiIconIcon::CornerResizer,
            theme.0.corner_resizer_size,
            theme.0.corner_resizer_size,
            GuiIconProps::default(),
        ))
        .id();
    commands
        .entity(*corner_resizer_entity)
        .despawn_children()
        .add_child(corner_resizer_icon);
}

fn modify_style_from_state(
    _commands: &mut Commands,
    theme: &GuiThemeComputed,
    _attribs: &GuiFloatingPanelAttribs,
    state: &GuiFloatingPanelState,
    root_node: &mut Node,
    title_bar_node: &mut Node,
    main_content_node: &mut Node,
    corner_resizer_node: &mut Node,
) {
    root_node.left = px(state.pos_x);
    root_node.top = px(state.pos_y);
    root_node.display = what_display_for_root(&state);

    title_bar_node.border_radius = what_border_radius_for_title_bar(&theme, &state);

    main_content_node.display = what_display_for_main_content(&state);
    main_content_node.width = what_width_for_main_content(&state);
    main_content_node.height = what_height_for_main_content(&state);

    corner_resizer_node.display = what_display_for_corner_resizer(&state);
}

fn what_width_for_main_content(state: &GuiFloatingPanelState) -> Val {
    match state.is_minimized {
        true => Val::Auto,
        false => px(state.size_x),
    }
}

fn what_height_for_main_content(state: &GuiFloatingPanelState) -> Val {
    match state.is_minimized {
        true => Val::Auto,
        false => px(state.size_y),
    }
}

fn what_display_for_root(state: &GuiFloatingPanelState) -> Display {
    match state.is_active {
        true => Display::Flex,
        false => Display::None,
    }
}

fn what_border_radius_for_title_bar(
    theme: &GuiThemeComputed,
    state: &GuiFloatingPanelState,
) -> BorderRadius {
    match state.is_minimized {
        true => BorderRadius::all(px(theme.0.border_radius)),
        false => BorderRadius::top(px(theme.0.border_radius)),
    }
}

fn what_display_for_main_content(state: &GuiFloatingPanelState) -> Display {
    match state.is_minimized {
        true => Display::None,
        false => Display::Flex,
    }
}

fn what_display_for_corner_resizer(state: &GuiFloatingPanelState) -> Display {
    match state.is_minimized {
        true => Display::None,
        false => Display::Flex,
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
            .add_systems(Update,
                update_panel_dragged
                    .in_set(GuiSystemsOrdering::UpdateState)
            )
            .add_systems(Update,
                update_panel_resized
                    .in_set(GuiSystemsOrdering::UpdateState)
            )
            .add_systems(Update,
                enforce_min_size
                    .in_set(GuiSystemsOrdering::PreUpdateState)
            )
            .add_systems(Update, update_cursor_icon)
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
    root_q: Query<
        (Entity, &Children),
        (
            With<GuiFloatingPanelAttribs>,
            Without<GuiFloatingPanelRelations>,
        ),
    >,
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

                                commands.entity(minimize_button).observe(
                                    move |_: On<Pointer<Click>>, mut root_state_q: Query<&mut GuiFloatingPanelState>| {
                                        if let Some(mut root_state) = alrmo!(root_state_q.get_mut(root)) {
                                            root_state.is_minimized = !root_state.is_minimized;
                                        }
                                    },
                                );
                            }

                            if let Ok(x_button) = x_button_q.get(child) {
                                relations.x_button = x_button;

                                commands.entity(x_button).observe(
                                    move |_: On<Pointer<Click>>, mut root_state_q: Query<&mut GuiFloatingPanelState>| {
                                        if let Some(mut root_state) = alrmo!(root_state_q.get_mut(root)) {
                                            root_state.is_active = false;
                                        }
                                    },
                                );
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

fn handle_gui_children(
    world: &mut World,
    mut root_q: Local<QueryState<Entity, (With<GuiFloatingPanelRelations>, With<GuiChildren>)>>,
    mut relations_q: Local<QueryState<&GuiFloatingPanelRelations>>,
    mut main_content_inner_q: Local<QueryState<Entity, With<GuiFloatingPanelMainContentInnerTag>>>,
) {
    let entities: Vec<_> = root_q.iter(world).map(|e| e).collect();

    entities.iter().for_each(|entity| {
        let mut entity_mut = world.entity_mut(*entity);
        if let Some(gui_children) = entity_mut.take::<GuiChildren>() {
            let relations = alrro!(relations_q.get(&world, *entity));
            let main_content_inner =
                alrro!(main_content_inner_q.get(&world, relations.main_content_inner));

            world
                .entity_mut(main_content_inner)
                .with_children(gui_children.0);
        }
    });
}

fn update_style_on_init_or_attrib_change(
    mut commands: Commands,
    theme: Res<GuiThemeComputed>,
    mut root_q: Query<
        (
            &GuiFloatingPanelRelations,
            &GuiFloatingPanelAttribs,
            &GuiFloatingPanelState,
            Entity,
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
        Entity,
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
        Entity,
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
        &mut Node,
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
        (Entity, &mut Node),
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
        .for_each(|(relations, attribs, state, root_entity, mut root_node)| {
            let (title_bar_entity, mut title_bar_node) =
                alrro!(title_bar_q.get_mut(relations.title_bar));
            let (title_bar_main_part_entity, mut title_bar_main_part_node) =
                alrro!(title_bar_main_part_q.get_mut(relations.title_bar_main_part));
            let mut title_bar_button_part_node =
                alrro!(title_bar_button_part_q.get_mut(relations.title_bar_button_part));
            let minimize_button_entity =
                alrro!(minimize_button_q.get_mut(relations.minimize_button));
            let x_button_entity = alrro!(x_button_q.get_mut(relations.x_button));
            let mut main_content_node = alrro!(main_content_q.get_mut(relations.main_content));
            let (main_content_inner_entity, mut main_content_inner_node) =
                alrro!(main_content_inner_q.get_mut(relations.main_content_inner));
            let (corner_resizer_entity, mut corner_resizer_node) =
                alrro!(corner_resizer_q.get_mut(relations.corner_resizer));

            apply_style(
                &mut commands,
                &theme,
                &attribs,
                &state,
                &root_entity,
                &mut root_node,
                &title_bar_entity,
                &mut title_bar_node,
                &title_bar_main_part_entity,
                &mut title_bar_main_part_node,
                &mut title_bar_button_part_node,
                &minimize_button_entity,
                &x_button_entity,
                &mut main_content_node,
                &main_content_inner_entity,
                &mut main_content_inner_node,
                &corner_resizer_entity,
                &mut corner_resizer_node,
            );
        });
}

fn update_style_on_theme_change(
    mut commands: Commands,
    theme: Res<GuiThemeComputed>,
    mut root_q: Query<(
        &GuiFloatingPanelRelations,
        &GuiFloatingPanelAttribs,
        &GuiFloatingPanelState,
        Entity,
        &mut Node,
    )>,
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
        Entity,
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
        Entity,
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
        &mut Node,
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
        (Entity, &mut Node),
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
        .for_each(|(relations, attribs, state, root_entity, mut root_node)| {
            let (title_bar_entity, mut title_bar_node) =
                alrro!(title_bar_q.get_mut(relations.title_bar));
            let (title_bar_main_part_entity, mut title_bar_main_part_node) =
                alrro!(title_bar_main_part_q.get_mut(relations.title_bar_main_part));
            let mut title_bar_button_part_node =
                alrro!(title_bar_button_part_q.get_mut(relations.title_bar_button_part));
            let minimize_button_entity =
                alrro!(minimize_button_q.get_mut(relations.minimize_button));
            let x_button_entity = alrro!(x_button_q.get_mut(relations.x_button));
            let mut main_content_node = alrro!(main_content_q.get_mut(relations.main_content));
            let (main_content_inner_entity, mut main_content_inner_node) =
                alrro!(main_content_inner_q.get_mut(relations.main_content_inner));
            let (corner_resizer_entity, mut corner_resizer_node) =
                alrro!(corner_resizer_q.get_mut(relations.corner_resizer));

            apply_style(
                &mut commands,
                &theme,
                &attribs,
                &state,
                &root_entity,
                &mut root_node,
                &title_bar_entity,
                &mut title_bar_node,
                &title_bar_main_part_entity,
                &mut title_bar_main_part_node,
                &mut title_bar_button_part_node,
                &minimize_button_entity,
                &x_button_entity,
                &mut main_content_node,
                &main_content_inner_entity,
                &mut main_content_inner_node,
                &corner_resizer_entity,
                &mut corner_resizer_node,
            );
        });
}

fn update_style_on_state_change(
    mut commands: Commands,
    theme: Res<GuiThemeComputed>,
    mut root_q: Query<
        (
            &GuiFloatingPanelRelations,
            &GuiFloatingPanelAttribs,
            &GuiFloatingPanelState,
            &mut Node,
        ),
        Changed<GuiFloatingPanelState>,
    >,
    mut title_bar_q: Query<
        &mut Node,
        (
            With<GuiFloatingPanelTitleBarTag>,
            Without<GuiFloatingPanelAttribs>,
            Without<GuiFloatingPanelMainContentTag>,
            Without<GuiFloatingPanelCornerResizerTag>,
        ),
    >,
    mut main_content_q: Query<
        &mut Node,
        (
            With<GuiFloatingPanelMainContentTag>,
            Without<GuiFloatingPanelAttribs>,
            Without<GuiFloatingPanelTitleBarTag>,
            Without<GuiFloatingPanelCornerResizerTag>,
        ),
    >,
    mut corner_resizer_q: Query<
        &mut Node,
        (
            With<GuiFloatingPanelCornerResizerTag>,
            Without<GuiFloatingPanelAttribs>,
            Without<GuiFloatingPanelTitleBarTag>,
            Without<GuiFloatingPanelMainContentTag>,
        ),
    >,
) {
    root_q
        .iter_mut()
        .for_each(|(relations, attribs, state, mut root_node)| {
            let mut title_bar_node = alrro!(title_bar_q.get_mut(relations.title_bar));
            let mut main_content_node = alrro!(main_content_q.get_mut(relations.main_content));
            let mut corner_resizer_node =
                alrro!(corner_resizer_q.get_mut(relations.corner_resizer));

            modify_style_from_state(
                &mut commands,
                &theme,
                &attribs,
                &state,
                &mut root_node,
                &mut title_bar_node,
                &mut main_content_node,
                &mut corner_resizer_node,
            );
        });
}

fn update_panel_dragged(
    interaction_q: Query<
        (&Interaction, &ChildOf),
        (Changed<Interaction>, With<GuiFloatingPanelTitleBarTag>),
    >,
    mut panel_being_dragged: Local<Option<Entity>>,
    mut panel_q: Query<&mut GuiFloatingPanelState>,
    mut mouse_motion: MessageReader<CursorMoved>,
) {
    interaction_q
        .iter()
        .for_each(|(interaction, childof)| match *interaction {
            Interaction::Pressed => {
                *panel_being_dragged = Some(childof.0);
            }
            _ => {
                *panel_being_dragged = None;
            }
        });

    if let Some(target_panel) = *panel_being_dragged {
        let mut delta_x = 0.;
        let mut delta_y = 0.;
        mouse_motion.read().for_each(|msg| {
            delta_x += msg.delta.map(|d| d.x).unwrap_or(0.);
            delta_y += msg.delta.map(|d| d.y).unwrap_or(0.);
        });

        if let Ok(mut panel_state) = panel_q.get_mut(target_panel) {
            panel_state.pos_x += delta_x;
            panel_state.pos_y += delta_y;
        }
    }
}

fn update_panel_resized(
    interaction_q: Query<
        (&Interaction, &ChildOf),
        (Changed<Interaction>, With<GuiFloatingPanelCornerResizerTag>),
    >,
    mut panel_being_resized: Local<Option<Entity>>,
    mut panel_q: Query<&mut GuiFloatingPanelState>,
    mut mouse_motion: MessageReader<CursorMoved>,
) {
    interaction_q
        .iter()
        .for_each(|(interaction, childof)| match *interaction {
            Interaction::Pressed => {
                *panel_being_resized = Some(childof.0);
            }
            _ => {
                *panel_being_resized = None;
            }
        });

    if let Some(target_panel) = *panel_being_resized {
        if let Ok(mut panel_state) = panel_q.get_mut(target_panel) {
            let mut delta_x = 0.0;
            let mut delta_y = 0.0;
            mouse_motion.read().for_each(|msg| {
                delta_x += msg.delta.map(|d| d.x).unwrap_or(0.0);
                delta_y += msg.delta.map(|d| d.y).unwrap_or(0.0);
            });

            panel_state.size_x += delta_x;
            panel_state.size_y += delta_y;
        }
    }
}

fn enforce_min_size(
    theme: Res<GuiThemeComputed>,
    mut root_q: Query<(
        &GuiFloatingPanelRelations,
        &mut GuiFloatingPanelState,
        &Node,
    ) /* TODO Changed<GuiFloatingPanelState>*/>,
    title_bar_q: Query<
        &ComputedNode,
        (
            With<GuiFloatingPanelTitleBarTag>,
            Without<GuiFloatingPanelAttribs>,
            Without<GuiFloatingPanelMainContentTag>,
        ),
    >,
    main_content_q: Query<
        (&ComputedNode, &Node),
        (
            With<GuiFloatingPanelMainContentTag>,
            Without<GuiFloatingPanelAttribs>,
            Without<GuiFloatingPanelTitleBarTag>,
        ),
    >,
) {
    root_q
        .iter_mut()
        .for_each(|(relations, mut state, root_node)| {
            let title_bar_computed_node = alrro!(title_bar_q.get(relations.title_bar));
            let (main_content_computed_node, main_content_node) =
                alrro!(main_content_q.get(relations.main_content));

            if root_node.display != Display::None && main_content_node.display != Display::None {
                if main_content_computed_node.size.x < title_bar_computed_node.size.x {
                    state.size_x = title_bar_computed_node.size.x
                        * title_bar_computed_node.inverse_scale_factor;
                }
                if state.size_y < theme.0.floating_panel_content_min_height {
                    state.size_y = theme.0.floating_panel_content_min_height;
                }
            }
        });
}

fn update_cursor_icon(
    mut cursor_icon_handler: ResMut<CursorIconHandler>,
    interaction_q: Query<
        (&Interaction, Entity),
        (With<GuiFloatingPanelCornerResizerTag>, Changed<Interaction>),
    >,
) {
    interaction_q.iter().for_each(|(interaction, entity)| {
        if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
            cursor_icon_handler.add_candidate(
                entity,
                SystemCursorIcon::SeResize,
                CursorIconPriority::Regular,
            );
        } else {
            cursor_icon_handler.remove_candidate(entity, SystemCursorIcon::SeResize);
        }
    });
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct GuiFloatingPanelInterface {
    state: &'static mut GuiFloatingPanelState,
}

impl<'w, 's> GuiFloatingPanelInterfaceItem<'w, 's> {
    pub fn is_active(&self) -> bool {
        self.state.is_active
    }

    pub fn set_is_active(&mut self, val: bool) {
        self.state.is_active = val;
    }
}
