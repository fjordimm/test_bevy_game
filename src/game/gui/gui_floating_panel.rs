/*
    Note: if there are multiple floating panels, they will not order themselves
*/

use bevy::{
    prelude::*,
    ui_widgets::{ControlOrientation, CoreScrollbarThumb, Scrollbar},
    window::{CursorIcon, PrimaryWindow, SystemCursorIcon},
};

use crate::game::{
    gui::{
        GuiButton, GuiIcon, GuiNode, GuiText, constants::*, gui_button::GuiButtonStyle,
        images::UiIconOption, plugin::CollectionOfGuiItems,
    },
    util::{TempOnCreation, warned_ok},
};

#[derive(Component)]
pub struct GuiFloatingPanelTag {
    pub is_active: bool,
    pub is_minimized: bool,
    title_bar_div: Entity,
    main_content_div: Entity,
    main_content_div_inner: Entity,
    resizer: Entity,
    h_scrollbar: Entity,
    v_scrollbar: Entity,
}

#[derive(Component)]
pub struct GuiFloatingPanelTitleBarTag;

#[derive(Component)]
pub struct GuiFloatingPanelMinimizeButtonTag;

#[derive(Component)]
pub struct GuiFloatingPanelXButtonTag;

#[derive(Component)]
pub struct GuiFloatingPanelMainContentTag;

#[derive(Component)]
pub struct GuiFloatingPanelMainContentInnerTag;

#[derive(Component)]
pub struct GuiFloatingPanelResizerTag;

#[derive(Component)]
pub struct GuiFloatingPanelHScrollbarTag;

#[derive(Component)]
pub struct GuiFloatingPanelVScrollbarTag;

pub struct GuiFloatingPanel {
    starts_active: bool,
    pos_x: f32,
    pos_y: f32,
    title: String,
    children: Vec<Box<dyn GuiNode>>,
}

impl GuiFloatingPanel {
    pub fn new<C: Into<CollectionOfGuiItems>>(
        starts_active: bool,
        pos_x: f32,
        pos_y: f32,
        title: impl Into<String>,
        children: C,
    ) -> Self {
        Self {
            starts_active: starts_active,
            pos_x: pos_x,
            pos_y: pos_y,
            title: title.into(),
            children: children.into().0,
        }
    }
}

#[derive(Component)]
pub struct WaitOneFrame;

impl GuiNode for GuiFloatingPanel {
    fn spawn(self, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        let entity = commands
            .spawn((
                GuiFloatingPanelTag {
                    is_active: true, // Only temporary
                    is_minimized: false,
                    title_bar_div: Entity::PLACEHOLDER, // Only temporary
                    main_content_div: Entity::PLACEHOLDER, // Only temporary
                    main_content_div_inner: Entity::PLACEHOLDER, // Only temporary
                    resizer: Entity::PLACEHOLDER,       // Only temporary
                    h_scrollbar: Entity::PLACEHOLDER,   // Only temporary
                    v_scrollbar: Entity::PLACEHOLDER,   // Only temporary
                },
                Node {
                    position_type: PositionType::Absolute,
                    left: px(self.pos_x),
                    top: px(self.pos_y),
                    border_radius: BorderRadius::all(px(BORDER_RADIUS)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Stretch,
                    ..default()
                },
                main_box_shadow(),
                BackgroundColor(MAIN_COLOR),
            ))
            .id();
        if let Some(par) = parent {
            commands.entity(par).add_child(entity);
        }

        let title_bar = commands
            .spawn((
                GuiFloatingPanelTitleBarTag,
                Button,
                Node {
                    border_radius: BorderRadius::top(px(BORDER_RADIUS)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Stretch,
                    padding: UiRect::all(px(MINOR_PADDING)),
                    column_gap: px(MAIN_PADDING),
                    ..default()
                },
                BackgroundColor(BUTTON_COLOR_MAIN),
                WaitOneFrame,
            ))
            .id();
        commands.entity(entity).add_child(title_bar);

        let main_content_div = commands
            .spawn((
                GuiFloatingPanelMainContentTag,
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    padding: UiRect::all(px(MAIN_PADDING)),
                    ..default()
                },
                BackgroundColor(Color::hsv(90.0, 1.0, 0.8)),
            ))
            .id();
        commands.entity(entity).add_child(main_content_div);

        let main_content_div_inner = commands
            .spawn((
                GuiFloatingPanelMainContentInnerTag,
                Interaction::default(),
                Node {
                    flex_grow: 1.0,
                    align_self: AlignSelf::Stretch,
                    overflow: Overflow::scroll(),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    row_gap: px(MAIN_PADDING),
                    ..default()
                },
                BackgroundColor(Color::hsv(270.0, 1.0, 0.7)),
                ScrollPosition::default(),
            ))
            .id();
        commands
            .entity(main_content_div)
            .add_child(main_content_div_inner);

        for child in self.children {
            let child_entity = child.spawn_dyn(commands, None);
            commands
                .entity(main_content_div_inner)
                .add_child(child_entity);
        }

        let h_scrollbar = commands
            .spawn((
                GuiFloatingPanelHScrollbarTag,
                Node {
                    position_type: PositionType::Absolute,
                    left: px(0),
                    right: px(MAIN_PADDING),
                    bottom: px(0),
                    height: px(SCROLLBAR_WIDTH),
                    display: Display::None,
                    ..default()
                },
                Scrollbar {
                    target: main_content_div_inner,
                    orientation: ControlOrientation::Horizontal,
                    min_thumb_length: SCROLLBAR_THUMB_MIN_HEIGHT,
                },
                BackgroundColor(Color::hsv(0.0, 0.5, 0.5)),
                Children::spawn(Spawn((
                    CoreScrollbarThumb,
                    Node {
                        position_type: PositionType::Absolute,
                        height: px(SCROLLBAR_WIDTH),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(px(
                            (SCROLLBAR_WIDTH - SCROLLBAR_THUMB_WIDTH) as f32 / 2.0
                        )),
                        ..default()
                    },
                    BackgroundColor(Color::hsv(180.0, 1.0, 0.5)),
                    Children::spawn(Spawn((
                        Node {
                            flex_grow: 1.0,
                            height: px(SCROLLBAR_THUMB_WIDTH),
                            border_radius: BorderRadius::all(px(SCROLLBAR_THUMB_WIDTH)),
                            ..default()
                        },
                        BackgroundColor(Color::hsv(180.0, 1.0, 0.1)),
                    ))),
                ))),
            ))
            .id();
        commands.entity(main_content_div).add_child(h_scrollbar);

        let v_scrollbar = commands
            .spawn((
                GuiFloatingPanelVScrollbarTag,
                Node {
                    position_type: PositionType::Absolute,
                    top: px(0),
                    bottom: px(MAIN_PADDING),
                    right: px(0),
                    width: px(SCROLLBAR_WIDTH),
                    display: Display::None,
                    ..default()
                },
                Scrollbar {
                    target: main_content_div_inner,
                    orientation: ControlOrientation::Vertical,
                    min_thumb_length: SCROLLBAR_THUMB_MIN_HEIGHT,
                },
                BackgroundColor(Color::hsv(0.0, 0.5, 0.5)),
                Children::spawn(Spawn((
                    CoreScrollbarThumb,
                    Node {
                        position_type: PositionType::Absolute,
                        width: px(SCROLLBAR_WIDTH),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(px(
                            (SCROLLBAR_WIDTH - SCROLLBAR_THUMB_WIDTH) as f32 / 2.0
                        )),
                        ..default()
                    },
                    BackgroundColor(Color::hsv(180.0, 1.0, 0.5)),
                    Children::spawn(Spawn((
                        Node {
                            flex_grow: 1.0,
                            width: px(SCROLLBAR_THUMB_WIDTH),
                            border_radius: BorderRadius::all(px(SCROLLBAR_THUMB_WIDTH)),
                            ..default()
                        },
                        BackgroundColor(Color::hsv(180.0, 1.0, 0.1)),
                    ))),
                ))),
            ))
            .id();
        commands.entity(main_content_div).add_child(v_scrollbar);

        let title_bar_main_part = commands
            .spawn((Node {
                flex_grow: 1.0,
                border_radius: BorderRadius::top_left(px(BORDER_RADIUS)),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                padding: UiRect::left(px(MINOR_PADDING)),
                column_gap: px(MINOR_PADDING),
                ..default()
            },))
            .id();
        commands.entity(title_bar).add_child(title_bar_main_part);

        GuiText::new_small(self.title, false).spawn(commands, Some(title_bar_main_part));

        let title_bar_button_part = commands
            .spawn((Node {
                border_radius: BorderRadius::top_right(px(BORDER_RADIUS)),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                column_gap: px(MINOR_PADDING),
                ..default()
            },))
            .id();
        commands.entity(title_bar).add_child(title_bar_button_part);

        let minimize_button = GuiButton::new(
            GuiButtonStyle::TitleBarButton,
            || interactions::MinimizeButtonEv { panel_div: entity },
            (GuiIcon::new(
                UiIconOption::Minimize,
                TITLE_BAR_BUTTON_ICON_SIZE,
                TITLE_BAR_BUTTON_ICON_SIZE,
            ),),
        )
        .spawn(commands, Some(title_bar_button_part));
        commands
            .entity(minimize_button)
            .insert(GuiFloatingPanelMinimizeButtonTag);

        let x_button = GuiButton::new(
            GuiButtonStyle::TitleBarButton,
            || interactions::XButtonEv { panel_div: entity },
            (GuiIcon::new(
                UiIconOption::X,
                TITLE_BAR_BUTTON_ICON_SIZE,
                TITLE_BAR_BUTTON_ICON_SIZE,
            ),),
        )
        .spawn(commands, Some(title_bar_button_part));
        commands.entity(x_button).insert(GuiFloatingPanelXButtonTag);

        let corner_resizer = commands
            .spawn((
                GuiFloatingPanelResizerTag,
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    right: px(CORNER_RESIZER_PADDING),
                    bottom: px(CORNER_RESIZER_PADDING),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ))
            .id();
        commands.entity(entity).add_child(corner_resizer);

        GuiIcon::new(
            UiIconOption::CornerResizer,
            CORNER_RESIZER_SIZE,
            CORNER_RESIZER_SIZE,
        )
        .spawn(commands, Some(corner_resizer));

        commands.entity(entity).observe(
            move |me: On<TempOnCreation>, mut query: Query<&mut GuiFloatingPanelTag>| {
                if let Ok(mut panel) = query.get_mut(me.0) {
                    panel.is_active = self.starts_active;
                    panel.title_bar_div = title_bar;
                    panel.main_content_div = main_content_div;
                    panel.main_content_div_inner = main_content_div_inner;
                    panel.resizer = corner_resizer;
                    panel.h_scrollbar = h_scrollbar;
                    panel.v_scrollbar = v_scrollbar;
                }
            },
        );
        commands.trigger(TempOnCreation(entity));

        entity
    }

    fn spawn_dyn(self: Box<Self>, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        self.spawn(commands, parent)
    }
}

mod interactions {
    use bevy_ecs::{entity::Entity, event::Event};

    #[derive(Event, Clone)]
    pub struct MinimizeButtonEv {
        pub panel_div: Entity,
    }

    #[derive(Event, Clone)]
    pub struct XButtonEv {
        pub panel_div: Entity,
    }
}

pub fn minimize_button_observer(
    ev: On<interactions::MinimizeButtonEv>,
    mut panel_q: Query<&mut GuiFloatingPanelTag>,
) {
    if let Ok(mut panel) = panel_q.get_mut(ev.panel_div) {
        panel.is_minimized = match panel.is_minimized {
            true => false,
            false => true,
        }
    }
}

pub fn x_button_observer(
    ev: On<interactions::XButtonEv>,
    mut panel_q: Query<&mut GuiFloatingPanelTag>,
) {
    if let Ok(mut panel) = panel_q.get_mut(ev.panel_div) {
        panel.is_active = match panel.is_active {
            true => false,
            false => true,
        }
    }
}

pub fn update_panel_from_is_active(
    mut panel_q: Query<(&GuiFloatingPanelTag, &mut Node), Changed<GuiFloatingPanelTag>>,
) {
    panel_q.iter_mut().for_each(|(panel, mut panel_node)| {
        panel_node.display = match panel.is_active {
            false => Display::None,
            true => Display::Flex,
        }
    });
}

pub fn update_main_content_from_is_minimized(
    panel_q: Query<&GuiFloatingPanelTag, Changed<GuiFloatingPanelTag>>,
    mut main_content_q: Query<&mut Node, With<GuiFloatingPanelMainContentTag>>,
) {
    panel_q.iter().for_each(|panel| {
        if let Some(mut main_content_node) =
            warned_ok!(main_content_q.get_mut(panel.main_content_div))
        {
            main_content_node.display = match panel.is_minimized {
                false => Display::Flex,
                true => Display::None,
            }
        }
    });
}

pub fn update_title_bar_from_is_minimized(
    panel_q: Query<&GuiFloatingPanelTag, Changed<GuiFloatingPanelTag>>,
    mut title_bar_q: Query<&mut Node, With<GuiFloatingPanelTitleBarTag>>,
) {
    panel_q.iter().for_each(|panel| {
        if let Some(mut title_bar_node) = warned_ok!(title_bar_q.get_mut(panel.title_bar_div)) {
            title_bar_node.border_radius = match panel.is_minimized {
                false => BorderRadius::top(px(BORDER_RADIUS)),
                true => BorderRadius::all(px(BORDER_RADIUS)),
            }
        }
    });
}

pub fn update_resizer_from_is_minimized(
    panel_q: Query<&GuiFloatingPanelTag, Changed<GuiFloatingPanelTag>>,
    mut resizer_q: Query<&mut Node, With<GuiFloatingPanelResizerTag>>,
) {
    panel_q.iter().for_each(|panel| {
        if let Some(mut resizer_node) = warned_ok!(resizer_q.get_mut(panel.resizer)) {
            resizer_node.display = match panel.is_minimized {
                false => Display::Flex,
                true => Display::None,
            }
        }
    });
}

pub fn update_panel_dragged(
    interaction_q: Query<
        (&Interaction, &ChildOf),
        (Changed<Interaction>, With<GuiFloatingPanelTitleBarTag>),
    >,
    mut panel_being_dragged: Local<Option<Entity>>,
    mut panel_q: Query<&mut Node, With<GuiFloatingPanelTag>>,
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
        let mut delta_x = 0.0;
        let mut delta_y = 0.0;
        mouse_motion.read().for_each(|msg| {
            delta_x += msg.delta.map(|d| d.x).unwrap_or(0.0);
            delta_y += msg.delta.map(|d| d.y).unwrap_or(0.0);
        });

        if let Ok(mut panel_node) = panel_q.get_mut(target_panel) {
            if !matches!(panel_node.left, Val::Px(_)) {
                panel_node.left = px(0);
            }

            if !matches!(panel_node.top, Val::Px(_)) {
                panel_node.top = px(0);
            }

            if let Val::Px(ref mut left) = panel_node.left {
                *left += delta_x;
            }

            if let Val::Px(ref mut top) = panel_node.top {
                *top += delta_y;
            }
        }
    }
}

pub fn update_panel_resized(
    interaction_q: Query<
        (&Interaction, &ChildOf),
        (Changed<Interaction>, With<GuiFloatingPanelResizerTag>),
    >,
    mut panel_being_resized: Local<Option<Entity>>,
    panel_q: Query<&GuiFloatingPanelTag>,
    mut main_content_q: Query<(&mut Node, &ComputedNode), With<GuiFloatingPanelMainContentTag>>,
    main_content_inner_q: Query<&ComputedNode, With<GuiFloatingPanelMainContentInnerTag>>,
    mut h_scrollbar_q: Query<
        &mut Node,
        (
            With<GuiFloatingPanelHScrollbarTag>,
            (
                Without<GuiFloatingPanelMainContentTag>, // to resolve query conflict
                Without<GuiFloatingPanelVScrollbarTag>,  // to resolve query conflict
            ),
        ),
    >,
    mut v_scrollbar_q: Query<
        &mut Node,
        (
            With<GuiFloatingPanelVScrollbarTag>,
            (
                Without<GuiFloatingPanelMainContentTag>, // to resolve query conflict
                Without<GuiFloatingPanelHScrollbarTag>,  // to resolve query conflict
            ),
        ),
    >,
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
        if let Ok(panel) = panel_q.get(target_panel) {
            let mut delta_x = 0.0;
            let mut delta_y = 0.0;
            mouse_motion.read().for_each(|msg| {
                delta_x += msg.delta.map(|d| d.x).unwrap_or(0.0);
                delta_y += msg.delta.map(|d| d.y).unwrap_or(0.0);
            });

            // Show/hide the scrollbars
            if let Some(computed_node) =
                warned_ok!(main_content_inner_q.get(panel.main_content_div_inner))
            {
                if let Some(mut h_scrollbar_node) =
                    warned_ok!(h_scrollbar_q.get_mut(panel.h_scrollbar))
                {
                    h_scrollbar_node.display =
                        match computed_node.content_size.x - computed_node.size.x > 0.0 {
                            false => Display::None,
                            true => Display::Flex,
                        }
                }

                if let Some(mut v_scrollbar_node) =
                    warned_ok!(v_scrollbar_q.get_mut(panel.v_scrollbar))
                {
                    v_scrollbar_node.display =
                        match computed_node.content_size.y - computed_node.size.y > 0.0 {
                            false => Display::None,
                            true => Display::Flex,
                        }
                }
            }

            // Resize the main content div
            if let Some((mut main_content_node, computed_node)) =
                warned_ok!(main_content_q.get_mut(panel.main_content_div))
            {
                if !matches!(main_content_node.width, Val::Px(_)) {
                    main_content_node.width =
                        px(computed_node.size.x * computed_node.inverse_scale_factor + 1.0);
                }

                if !matches!(main_content_node.height, Val::Px(_)) {
                    main_content_node.height =
                        px(computed_node.size.y * computed_node.inverse_scale_factor + 1.0);
                }

                if let Val::Px(ref mut width) = main_content_node.width {
                    *width += delta_x;
                }

                if let Val::Px(ref mut height) = main_content_node.height {
                    *height += delta_y;
                }
            }
        }
    }
}

pub fn update_panel_resized_enforce_min_width(
    panel_q: Query<&GuiFloatingPanelTag>,
    title_bar_q: Query<&ComputedNode, With<GuiFloatingPanelTitleBarTag>>,
    mut main_content_div_q: Query<(&ComputedNode, &mut Node), With<GuiFloatingPanelMainContentTag>>,
) {
    panel_q.iter().for_each(|panel| {
        if let Some(title_bar_computed_node) = warned_ok!(title_bar_q.get(panel.title_bar_div)) {
            if let Some((main_content_div_computed_node, mut main_content_div_node)) =
                warned_ok!(main_content_div_q.get_mut(panel.main_content_div))
            {
                // so that it doesn't do it when the panel is minimized
                if main_content_div_computed_node.size.x > 0.0 {
                    if main_content_div_computed_node.size.x < title_bar_computed_node.size.x {
                        // main_content_div_node.min_width = px(title_bar_computed_node.size.x
                        //     * title_bar_computed_node.inverse_scale_factor
                        //     - 1.0); // TODO: should I include this or not?
                        main_content_div_node.width = px(title_bar_computed_node.size.x
                            * title_bar_computed_node.inverse_scale_factor);
                    }
                }
            }
        }
    });
}

pub fn update_cursor_from_resizer_interaction(
    mut commands: Commands,
    interaction_q: Query<
        (&Interaction, Entity),
        (With<GuiFloatingPanelResizerTag>, Changed<Interaction>),
    >,
    mut button_last_interacted_with: Local<Option<Entity>>,
    window_q: Query<Entity, With<PrimaryWindow>>,
) {
    if let Some(window) = warned_ok!(window_q.single()) {
        interaction_q.iter().for_each(|(interaction, button_id)| {
            if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
                commands
                    .entity(window)
                    .insert(CursorIcon::from(SystemCursorIcon::SeResize));

                *button_last_interacted_with = Some(button_id);
            } else {
                if let Some(button_last_interacted_with) = *button_last_interacted_with {
                    if button_last_interacted_with == button_id {
                        commands
                            .entity(window)
                            .insert(CursorIcon::from(SystemCursorIcon::Default));
                    }
                }
            }
        });
    }
}
