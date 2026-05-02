use bevy::prelude::*;

use crate::game::gui::{GuiButton, GuiEntity, GuiNode, constants::*, plugin::CollectionOfGuiItems};

// Note: if there are multiple floating panels, they will not order themselves
pub struct GuiFloatingPanel {
    pos_x: f32,
    pos_y: f32,
    title_bar_children: Vec<Box<dyn GuiNode>>,
    children: Vec<Box<dyn GuiNode>>,
}

impl GuiFloatingPanel {
    pub fn new<C1: Into<CollectionOfGuiItems>, C2: Into<CollectionOfGuiItems>>(
        pos_x: f32,
        pos_y: f32,
        title_bar_children: C1,
        children: C2,
    ) -> Self {
        Self {
            pos_x: pos_x,
            pos_y: pos_y,
            title_bar_children: title_bar_children.into().0,
            children: children.into().0,
        }
    }
}

impl GuiNode for GuiFloatingPanel {
    fn spawn(self, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        let entity = commands
            .spawn((
                GuiFloatingPanelTag,
                Node {
                    position_type: PositionType::Absolute,
                    border_radius: BorderRadius::all(px(BORDER_RADIUS)),
                    overflow: Overflow::hidden(),
                    left: px(self.pos_x),
                    top: px(self.pos_y),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
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
                GuiFloatingPanelTitleBarTag { parent: entity },
                Button,
                Node {
                    border_radius: BorderRadius::top(px(BORDER_RADIUS)),
                    width: percent(100),
                    min_height: px(MAIN_PADDING),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(BUTTON_COLOR_MAIN),
            ))
            .id();
        commands.entity(entity).add_child(title_bar);

        let title_bar_main_part = commands
            .spawn((
                Node {
                    justify_self: JustifySelf::Stretch,
                    border_radius: BorderRadius::top_left(px(BORDER_RADIUS)),
                    min_height: percent(100),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    row_gap: px(MINOR_PADDING),
                    padding: UiRect::all(px(MINOR_PADDING)),
                    ..default()
                },
                BackgroundColor(Color::hsv(90.0, 1.0, 1.0)),
            ))
            .id();
        commands.entity(title_bar).add_child(title_bar_main_part);

        for child in self.title_bar_children {
            let child_entity = child.spawn_dyn(commands, None);
            commands.entity(title_bar_main_part).add_child(child_entity);
        }

        let title_bar_button_part = commands
            .spawn((
                Node {
                    justify_self: JustifySelf::End,
                    border_radius: BorderRadius::top_right(px(BORDER_RADIUS)),
                    min_height: percent(100),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    row_gap: px(MINOR_PADDING),
                    padding: UiRect::all(px(MINOR_PADDING)),
                    ..default()
                },
                BackgroundColor(Color::hsv(270.0, 1.0, 1.0)),
            ))
            .id();
        commands.entity(title_bar).add_child(title_bar_button_part);

        let x_button = GuiButton::new_unstyled_eventless((GuiEntity::new((
            Node {
                border_radius: BorderRadius::all(px(BORDER_RADIUS)),
                width: px(30),
                height: px(30),
                ..default()
            },
            BackgroundColor(Color::hsv(0.0, 1.0, 1.0)),
        )),))
        .spawn(commands, None);
        commands.entity(title_bar_button_part).add_child(x_button);

        let main_content_div = commands
            .spawn(Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: px(MAIN_PADDING),
                padding: UiRect::all(px(MAIN_PADDING)),
                ..default()
            })
            .id();
        commands.entity(entity).add_child(main_content_div);

        for child in self.children {
            let child_entity = child.spawn_dyn(commands, None);
            commands.entity(main_content_div).add_child(child_entity);
        }

        entity
    }

    fn spawn_dyn(self: Box<Self>, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        self.spawn(commands, parent)
    }
}

#[derive(Component)]
pub struct GuiFloatingPanelTag;

#[derive(Component)]
pub struct GuiFloatingPanelTitleBarTag {
    pub parent: Entity,
}

pub fn update(
    interaction_q: Query<(&Interaction, &GuiFloatingPanelTitleBarTag), Changed<Interaction>>,
    mut window_being_dragged: Local<Option<Entity>>,
    mut window_q: Query<&mut Node, With<GuiFloatingPanelTag>>,
    mut mouse_motion: MessageReader<CursorMoved>,
) {
    for (interaction, title_bar) in &interaction_q {
        match *interaction {
            Interaction::Pressed => {
                *window_being_dragged = Some(title_bar.parent);
            }
            _ => {
                *window_being_dragged = None;
            }
        }
    }

    if let Some(target_window) = *window_being_dragged {
        let mut delta_x = 0.0;
        let mut delta_y = 0.0;
        for msg in mouse_motion.read() {
            delta_x += msg.delta.map(|d| d.x).unwrap_or(0.0);
            delta_y += msg.delta.map(|d| d.y).unwrap_or(0.0);
        }

        if let Ok(mut target_node) = window_q.get_mut(target_window) {
            if let Val::Px(ref mut x) = target_node.left {
                *x += delta_x;
            } else {
                target_node.left = px(0.0);
            }
            if let Val::Px(ref mut y) = target_node.top {
                *y += delta_y;
            } else {
                target_node.top = px(0.0);
            }
        }
    }
}
