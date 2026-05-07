/*
    Note: if there are multiple floating panels, they will not order themselves
*/

use bevy::prelude::*;

use crate::game::{
    gui::{
        GuiButton, GuiDiv, GuiDivStyle, GuiIcon, GuiNode, GuiText, constants::*,
        gui_button::GuiButtonStyle, images::UiIconOption, plugin::CollectionOfGuiItems,
    },
    util::TempOnCreation,
};

#[derive(Component)]
pub struct GuiFloatingPanelTag {
    pub is_active: bool,
    pub is_minimized: bool,
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
pub struct GuiFloatingPanelResizerTag;

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

impl GuiNode for GuiFloatingPanel {
    fn spawn(self, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        let entity = commands
            .spawn((
                GuiFloatingPanelTag {
                    is_active: true,
                    is_minimized: false,
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
            ))
            .id();
        commands.entity(entity).add_child(title_bar);

        let main_content_div = GuiDiv::new(
            GuiDivStyle::None,
            false,
            UiRect::all(px(MAIN_PADDING)),
            MAIN_PADDING,
            FlexDirection::Column,
            JustifyContent::FlexStart,
            AlignItems::FlexStart,
            (),
        )
        .spawn(commands, Some(entity));
        commands
            .entity(main_content_div)
            .insert(GuiFloatingPanelMainContentTag);

        for child in self.children {
            let child_entity = child.spawn_dyn(commands, None);
            commands.entity(main_content_div).add_child(child_entity);
        }

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

        GuiText::new_small(self.title).spawn(commands, Some(title_bar_main_part));

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

        let corner_resizer_div = commands
            .spawn((Node {
                position_type: PositionType::Absolute,
                right: px(CORNER_RESIZER_PADDING),
                bottom: px(CORNER_RESIZER_PADDING),
                width: px(CORNER_RESIZER_SIZE),
                height: px(CORNER_RESIZER_SIZE),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },))
            .id();
        commands.entity(entity).add_child(corner_resizer_div);

        let corner_resizer_button = GuiButton::new(
            GuiButtonStyle::TitleBarButton,
            || interactions::CornerResizerEv { panel_div: entity },
            (GuiIcon::new(
                UiIconOption::CornerResizer,
                CORNER_RESIZER_SIZE,
                CORNER_RESIZER_SIZE,
            ),),
        )
        .spawn(commands, Some(corner_resizer_div));
        commands
            .entity(corner_resizer_button)
            .insert(GuiFloatingPanelResizerTag);

        commands.add_observer(
            move |ent: On<TempOnCreation>, mut query: Query<&mut GuiFloatingPanelTag>| {
                if let Ok(mut screen_div) = query.get_mut(ent.0) {
                    screen_div.is_active = self.starts_active;
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

    #[derive(Event, Clone)]
    pub struct CornerResizerEv {
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

        if let Ok(mut panel) = panel_q.get_mut(target_panel) {
            if let Val::Px(ref mut x) = panel.left {
                *x += delta_x;
            } else {
                panel.left = px(0.0);
            }
            if let Val::Px(ref mut y) = panel.top {
                *y += delta_y;
            } else {
                panel.top = px(0.0);
            }
        }
    }
}

pub fn update_content_from_is_minimized(
    mut main_content_q: Query<(&ChildOf, &mut Node), With<GuiFloatingPanelMainContentTag>>,
    panel_q: Query<&GuiFloatingPanelTag, Changed<GuiFloatingPanelTag>>,
) {
    main_content_q
        .iter_mut()
        .for_each(|(childof, mut main_content_node)| {
            if let Ok(panel) = panel_q.get(childof.0) {
                main_content_node.display = match panel.is_minimized {
                    false => Display::Flex,
                    true => Display::None,
                }
            }
        });
}

pub fn update_title_bar_from_is_minimized(
    mut title_bar_q: Query<(&ChildOf, &mut Node), With<GuiFloatingPanelTitleBarTag>>,
    panel_q: Query<&GuiFloatingPanelTag, Changed<GuiFloatingPanelTag>>,
) {
    title_bar_q
        .iter_mut()
        .for_each(|(childof, mut title_bar_node)| {
            if let Ok(panel) = panel_q.get(childof.0) {
                title_bar_node.border_radius = match panel.is_minimized {
                    false => BorderRadius::top(px(BORDER_RADIUS)),
                    true => BorderRadius::all(px(BORDER_RADIUS)),
                }
            }
        });
}

// pub fn update_

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
