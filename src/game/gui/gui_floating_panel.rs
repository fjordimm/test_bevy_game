use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::game::gui::{GuiNode, constants::*, plugin::CollectionOfGuiItems};

pub struct GuiFloatingPanel {
    flex_direction: FlexDirection,
    children: Vec<Box<dyn GuiNode>>,
}

impl GuiFloatingPanel {
    pub fn new<C: Into<CollectionOfGuiItems>>(flex_direction: FlexDirection, children: C) -> Self {
        Self {
            flex_direction: flex_direction,
            children: children.into().0,
        }
    }
}

impl GuiNode for GuiFloatingPanel {
    fn spawn(&self, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        let entity = commands
            .spawn((
                GuiFloatingPanelTag,
                Node {
                    position_type: PositionType::Absolute,
                    overflow: Overflow::hidden(),
                    left: px(100),
                    top: px(100),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(px(BORDER_RADIUS)),
                    ..default()
                },
                main_box_shadow(),
                BackgroundColor(MAIN_COLOR),
            ))
            .id();
        if let Some(par) = parent {
            commands.entity(par).add_child(entity);
        }

        let handle = commands
            .spawn((
                GuiFloatingPanelHandleTag { parent: entity },
                Button,
                Node {
                    width: Val::Percent(100.0),
                    height: px(15),
                    border_radius: BorderRadius::top(px(BORDER_RADIUS)),
                    ..default()
                },
                BackgroundColor(BUTTON_COLOR_MAIN),
            ))
            .id();
        commands.entity(entity).add_child(handle);

        let main_content_div = commands
            .spawn(Node {
                display: Display::Flex,
                flex_direction: self.flex_direction,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: px(MAIN_PADDING),
                padding: UiRect::all(px(MAIN_PADDING)),
                ..default()
            })
            .id();
        commands.entity(entity).add_child(main_content_div);

        for child in &self.children {
            let child_entity = child.spawn(commands, None);
            commands.entity(main_content_div).add_child(child_entity);
        }

        entity
    }
}

#[derive(Component)]
pub struct GuiFloatingPanelTag;

#[derive(Component)]
pub struct GuiFloatingPanelHandleTag {
    pub parent: Entity,
}

pub fn update(
    mut interaction_query: Query<(&Interaction, &GuiFloatingPanelHandleTag), Changed<Interaction>>,
    mut window_being_dragged: Local<Option<Entity>>,
    mut window_query: Query<&mut Node, With<GuiFloatingPanelTag>>,
    mut mouse_motion: MessageReader<CursorMoved>,
) {
    for (interaction, handle) in &interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *window_being_dragged = Some(handle.parent);
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

        if let Ok(mut target_node) = window_query.get_mut(target_window) {
            if let Val::Px(ref mut x) = target_node.left {
                *x += delta_x;
            }
            if let Val::Px(ref mut y) = target_node.top {
                *y += delta_y;
            }
        }
    }
}
