use bevy::prelude::*;

pub trait GuiNode {
    fn spawn(&self, commands: &mut Commands, parent: Option<Entity>) -> Entity;
}
