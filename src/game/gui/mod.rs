use bevy::prelude::*;

pub mod plugin;
pub mod resources;
pub mod sets;
pub mod widgets;

pub fn make_global_gui_root() -> Node {
    Node {
        width: vw(100),
        height: vh(100),
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::FlexStart,
        align_items: AlignItems::FlexStart,
        ..default()
    }
}

#[derive(Component)]
struct GuiChildren(Box<dyn FnOnce(&mut ChildSpawner) + Sync + Send>);

#[allow(unused)]
pub fn gui_children<F: 'static + FnOnce(&mut ChildSpawner) + Sync + Send>(func: F) -> impl Bundle {
    GuiChildren(Box::new(func))
}

#[allow(unused)]
pub fn gui_child(bundle: impl Bundle) -> impl Bundle {
    GuiChildren(Box::new(|p| {
        p.spawn(bundle);
    }))
}
