use bevy::prelude::*;

pub trait GuiParent {
    fn spawn(&self, commands: &mut Commands) -> Entity;
}
