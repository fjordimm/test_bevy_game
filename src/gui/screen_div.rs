use bevy::prelude::*;

use crate::gui::GuiNode;

pub struct ScreenDiv {
    color: Color,
    children: Vec<Box<dyn GuiNode>>,
}

impl ScreenDiv {
    pub fn new(color: Color, children: Vec<Box<dyn GuiNode>>) -> Self {
        Self {
            color: color,
            children: children,
        }
    }
}

impl GuiNode for ScreenDiv {
    fn spawn(&self, commands: &mut Commands) -> Entity {
        let entity = commands
            .spawn((
                Node {
                    width: percent(100),
                    height: percent(100),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: px(10),
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
