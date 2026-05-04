use bevy::prelude::*;

use crate::game::gui::{GuiNode, constants::*, plugin::CollectionOfGuiItems};

pub enum GuiDivStyle {
    None,
    Regular,
}

pub struct GuiDiv {
    style: GuiDivStyle,
    expand: bool,
    padding: UiRect,
    gap: i32,
    flex_direction: FlexDirection,
    justify_content: JustifyContent,
    align_items: AlignItems,
    children: Vec<Box<dyn GuiNode>>,
}

impl GuiDiv {
    pub fn new<C: Into<CollectionOfGuiItems>>(
        style: GuiDivStyle,
        expand: bool,
        padding: UiRect,
        gap: i32,
        flex_direction: FlexDirection,
        justify_content: JustifyContent,
        align_items: AlignItems,
        children: C,
    ) -> Self {
        Self {
            style: style,
            expand: expand,
            padding: padding,
            gap: gap,
            flex_direction: flex_direction,
            justify_content: justify_content,
            align_items: align_items,
            children: children.into().0,
        }
    }
}

impl GuiNode for GuiDiv {
    fn spawn(self, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        let entity = match self.style {
            GuiDivStyle::None => commands.spawn((Node {
                width: if self.expand { percent(100) } else { Val::Auto },
                height: if self.expand { percent(100) } else { Val::Auto },
                display: Display::Flex,
                flex_direction: self.flex_direction,
                justify_content: self.justify_content,
                align_items: self.align_items,
                padding: self.padding,
                row_gap: px(self.gap),
                ..default()
            },)),
            GuiDivStyle::Regular => commands.spawn((
                Node {
                    border_radius: BorderRadius::all(px(BORDER_RADIUS)),
                    width: if self.expand { percent(100) } else { Val::Auto },
                    height: if self.expand { percent(100) } else { Val::Auto },
                    display: Display::Flex,
                    flex_direction: self.flex_direction,
                    justify_content: self.justify_content,
                    align_items: self.align_items,
                    padding: self.padding,
                    row_gap: px(self.gap),
                    ..default()
                },
                main_box_shadow(),
                BackgroundColor(MAIN_COLOR),
            )),
        }
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
