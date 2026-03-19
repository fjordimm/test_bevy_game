use bevy::prelude::*;

use crate::gui::{self, GuiNode};

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
                    row_gap: px(gui::constants::MAIN_PADDING),
                    padding: UiRect::all(px(gui::constants::MAIN_PADDING)),
                    border: UiRect::all(px(gui::constants::DIV_BORDER_SIZE)),
                    ..default()
                },
                BackgroundColor(gui::constants::DIV_MAIN_COLOR),
                BorderColor::all(gui::constants::DIV_BORDER_COLOR),
            ))
            .id();

        for child in &self.children {
            let child_entity = child.spawn(commands);
            commands.entity(entity).add_child(child_entity);
        }

        entity
    }
}
