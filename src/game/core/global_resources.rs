use bevy::input::keyboard::KeyCode;
use bevy::prelude::Font;
use bevy::prelude::Handle;
use bevy_ecs::resource::Resource;

#[derive(Resource, Debug)]
pub struct KeyBindings {
    pub pause: KeyCode,
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub move_up: KeyCode,
    pub move_down: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            pause: KeyCode::Escape,
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            move_up: KeyCode::Space,
            move_down: KeyCode::ShiftLeft,
        }
    }
}

#[derive(Resource, Debug)]
pub struct GlobalFonts {
    pub sans: Handle<Font>,
}
