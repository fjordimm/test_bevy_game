use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use bevy_ecs::resource::Resource;

#[derive(Resource)]
pub struct FontHandles {
    #[allow(unused)]
    pub cabin: Handle<Font>,
    #[allow(unused)]
    pub sorts_mill_goudy: Handle<Font>,
    #[allow(unused)]
    pub ibm_plex_mono: Handle<Font>,
}

impl FontHandles {
    pub fn make(asset_server: &AssetServer) -> Self {
        Self {
            cabin: asset_server.load("fonts/Cabin-VariableFont_wdth,wght.ttf"),
            sorts_mill_goudy: asset_server.load("fonts/SortsMillGoudy-Regular.ttf"),
            ibm_plex_mono: asset_server.load("fonts/IBMPlexMono-Regular.ttf"),
        }
    }
}

#[derive(Resource, Debug)]
pub struct UiIconHandles {
    #[allow(unused)]
    pub x: Handle<Image>,
    #[allow(unused)]
    pub minimize: Handle<Image>,
    #[allow(unused)]
    pub corner_resizer: Handle<Image>,
}

impl UiIconHandles {
    pub fn make(asset_server: &AssetServer) -> Self {
        Self {
            x: asset_server.load("gui/icons/material-symbols--close.png"),
            minimize: asset_server.load("gui/icons/material-symbols--minimize.png"),
            corner_resizer: asset_server.load("gui/icons/material-symbols--resize-window.png"),
        }
    }
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
