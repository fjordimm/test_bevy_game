use bevy::prelude::*;

use crate::game::gui::{GuiNode, images::UiIconOption};

pub struct GuiIcon {
    icon: UiIconOption,
    width: i32,
    height: i32,
}

impl GuiIcon {
    pub fn new(icon: UiIconOption, width: i32, height: i32) -> Self {
        Self {
            icon: icon,
            width: width,
            height: height,
        }
    }
}

impl GuiNode for GuiIcon {
    fn spawn(self, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        let entity = commands
            .spawn((
                Node {
                    width: px(self.width),
                    height: px(self.height),
                    ..default()
                },
                ImageNode {
                    image_mode: NodeImageMode::Stretch,
                    ..default()
                },
                self.icon,
            ))
            .id();
        if let Some(par) = parent {
            commands.entity(par).add_child(entity);
        }

        entity
    }

    fn spawn_dyn(self: Box<Self>, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        self.spawn(commands, parent)
    }
}
