use bevy::prelude::*;

use crate::game::gui::{GuiNode, constants::*, plugin::CollectionOfGuiItems};

pub struct GuiDiv {
    flex_direction: FlexDirection,
    justify_content: JustifyContent,
    align_items: AlignItems,
    children: Vec<Box<dyn GuiNode>>,
}

impl GuiDiv {
    pub fn new<C: Into<CollectionOfGuiItems>>(
        flex_direction: FlexDirection,
        justify_content: JustifyContent,
        align_items: AlignItems,
        children: C,
    ) -> Self {
        Self {
            flex_direction: flex_direction,
            justify_content: justify_content,
            align_items: align_items,
            children: children.into().0,
        }
    }

    pub fn new_simple<C: Into<CollectionOfGuiItems>>(children: C) -> Self {
        Self {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            children: children.into().0,
        }
    }
}

impl GuiNode for GuiDiv {
    fn spawn(self, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        let entity = commands
            .spawn((
                Node {
                    border_radius: BorderRadius::all(px(BORDER_RADIUS)),
                    display: Display::Flex,
                    flex_direction: self.flex_direction,
                    justify_content: self.justify_content,
                    align_items: self.align_items,
                    row_gap: px(MAIN_PADDING),
                    padding: UiRect::all(px(MAIN_PADDING)),
                    ..default()
                },
                main_box_shadow(),
                BackgroundColor(MAIN_COLOR),
            ))
            .id();
        if let Some(par) = parent {
            commands.entity(par).add_child(entity);
        }

        for child in self.children {
            let child_entity = child.spawn_dyn(commands, None);
            commands.entity(entity).add_child(child_entity);
        }

        entity
    }

    fn spawn_dyn(self: Box<Self>, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        self.spawn(commands, parent)
    }
}
