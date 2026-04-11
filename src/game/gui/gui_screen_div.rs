use bevy::prelude::*;

use crate::game::gui::{GuiNode, constants::*};

pub struct GuiScreenDiv {
    color: Color,
    flex_direction: FlexDirection,
    children: Vec<Box<dyn GuiNode>>,
}

impl GuiScreenDiv {
    pub fn new(
        color: Color,
        flex_direction: FlexDirection,
        children: Vec<Box<dyn GuiNode>>,
    ) -> Self {
        Self {
            color: color,
            flex_direction: flex_direction,
            children: children,
        }
    }
}

impl GuiNode for GuiScreenDiv {
    fn spawn(&self, commands: &mut Commands) -> Entity {
        let entity = commands
            .spawn((
                Node {
                    width: percent(100),
                    height: percent(100),
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

        for child in &self.children {
            let child_entity = child.spawn(commands);
            commands.entity(entity).add_child(child_entity);
        }

        entity
    }
}
