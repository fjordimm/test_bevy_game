use bevy::prelude::*;

use crate::game::gui::{
    GuiNode,
    constants::{DIV_BORDER_COLOR, DIV_BORDER_SIZE, DIV_MAIN_COLOR, MAIN_PADDING},
};

pub struct GuiDiv {
    flex_direction: FlexDirection,
    children: Vec<Box<dyn GuiNode>>,
}

impl GuiDiv {
    pub fn new(flex_direction: FlexDirection, children: Vec<Box<dyn GuiNode>>) -> Self {
        Self {
            flex_direction: flex_direction,
            children: children,
        }
    }
}

impl GuiNode for GuiDiv {
    fn spawn(&self, commands: &mut Commands) -> Entity {
        let entity = commands
            .spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: self.flex_direction,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: px(MAIN_PADDING),
                    padding: UiRect::all(px(MAIN_PADDING)),
                    border: UiRect::all(px(DIV_BORDER_SIZE)),
                    ..default()
                },
                BackgroundColor(DIV_MAIN_COLOR),
                BorderColor::all(DIV_BORDER_COLOR),
            ))
            .id();

        for child in &self.children {
            let child_entity = child.spawn(commands);
            commands.entity(entity).add_child(child_entity);
        }

        entity
    }
}
