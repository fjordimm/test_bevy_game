// Used for when you want a custom gui element (an Entity that should have some GUI component, i.e. Node, Text, etc.).

use bevy::prelude::*;

use crate::game::gui::GuiNode;

#[allow(unused)]
pub struct GuiEntity<B: Bundle> {
    content: Box<B>,
}

impl<B: Bundle> GuiEntity<B> {
    #[allow(unused)]
    pub fn new(content: B) -> Self {
        Self {
            content: Box::new(content),
        }
    }
}

impl<B: Bundle> GuiNode for GuiEntity<B> {
    fn spawn(self, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        let entity = commands.spawn(*self.content).id();
        if let Some(par) = parent {
            commands.entity(par).add_child(entity);
        }

        entity
    }

    fn spawn_dyn(self: Box<Self>, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        self.spawn(commands, parent)
    }
}
