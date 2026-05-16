use bevy::prelude::*;

mod gui_button;
mod gui_div;
mod gui_entity;
mod gui_floating_panel;
mod gui_icon;
mod gui_node;
mod gui_screen_div;
mod gui_text;
mod scrolling;

pub mod constants;
pub mod fonts;
pub mod images;
pub mod plugin;
pub use gui_button::*;
pub use gui_div::*;
pub use gui_entity::*;
pub use gui_floating_panel::*;
pub use gui_icon::*;
pub use gui_node::*;
pub use gui_screen_div::*;
pub use gui_text::*;

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
