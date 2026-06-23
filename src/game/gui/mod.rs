use bevy::prelude::*;

pub mod widgets;
pub mod plugin;
pub mod resources;

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
