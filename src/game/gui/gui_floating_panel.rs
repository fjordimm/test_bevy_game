use bevy::prelude::*;

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
                Node {
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    flex_direction: self.flex_direction,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: px(MAIN_PADDING),
                    padding: UiRect::all(px(MAIN_PADDING)),
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

        for child in &self.children {
            let child_entity = child.spawn(commands, None);
            commands.entity(entity).add_child(child_entity);
        }

        entity
    }
}
