use bevy::prelude::*;

pub fn gui_root_template() -> Node {
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
