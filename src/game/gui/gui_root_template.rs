use bevy::prelude::*;

pub fn gui_root_template() -> Node {
    Node {
        width: Val::Vw(100.0),
        height: Val::Vh(100.0),
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::FlexStart,
        align_items: AlignItems::FlexStart,
        ..default()
    }
}
