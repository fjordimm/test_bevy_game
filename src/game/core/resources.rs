use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use bevy_ecs::resource::Resource;

#[derive(Resource, Debug)]
pub struct Fonts {
    pub sans: Handle<Font>,
    pub serif: Handle<Font>,
    pub mono: Handle<Font>,
}

#[derive(Resource, Debug)]
pub struct UiIcons {
    pub x: Handle<Image>,
    pub minimize: Handle<Image>,
    pub corner_resizer: Handle<Image>,
}

#[allow(unused)]
#[derive(Resource, Debug)]
pub struct KeyBindings {
    pub pause: KeyCode,
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub move_up: KeyCode,
    pub move_down: KeyCode,
    pub toggle_debug_menu: KeyCode,
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
            toggle_debug_menu: KeyCode::KeyP,
        }
    }
}

#[derive(Resource)]
pub struct GlobalGuiRoot(pub Entity);
