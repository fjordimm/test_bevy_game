use bevy::prelude::*;

pub trait GuiNode {
    fn spawn(self, commands: &mut Commands, parent: Option<Entity>) -> Entity;

    fn spawn_dyn(self: Box<Self>, commands: &mut Commands, parent: Option<Entity>) -> Entity;
}
