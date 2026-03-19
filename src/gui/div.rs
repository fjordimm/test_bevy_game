use bevy::prelude::*;

use crate::gui::GuiNode;

pub struct Div {
    color: Color,
    children: Vec<Box<dyn GuiNode>>,
}

impl Div {
    pub fn new(color: Color, children: Vec<Box<dyn GuiNode>>) -> Self {
        Self {
            color: color,
            children: children,
        }
    }
}

impl GuiNode for Div {
    fn spawn(&self, commands: &mut Commands) -> Entity {
        let entity = commands
            .spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: px(10),
                    padding: UiRect::all(px(10)),
                    ..default()
                },
                BackgroundColor(self.color),
            ))
            .id();

        let text = commands.spawn(Text::new("Heeeee")).id();
        commands.entity(entity).add_child(text);

        for child in &self.children {
            let child_entity = child.spawn(commands);
            commands.entity(entity).add_child(child_entity);
        }

        entity
    }
}
