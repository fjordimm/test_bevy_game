use bevy::prelude::*;

use crate::game::gui::{GuiNode, constants::*, plugin::CollectionOfGuiItems};

pub struct GuiScreenDiv {
    color: Color,
    flex_direction: FlexDirection,
    children: Vec<Box<dyn GuiNode>>,
}

impl GuiScreenDiv {
    pub fn new<C: Into<CollectionOfGuiItems>>(
        color: Color,
        flex_direction: FlexDirection,
        children: C,
    ) -> Self {
        Self {
            color: color,
            flex_direction: flex_direction,
            children: children.into().0,
        }
    }
}

impl GuiNode for GuiScreenDiv {
    fn spawn(&self, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        let entity = commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    display: Display::Flex,
                    flex_direction: self.flex_direction,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: px(MAIN_PADDING),
                    ..default()
                },
                BackgroundColor(self.color),
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
