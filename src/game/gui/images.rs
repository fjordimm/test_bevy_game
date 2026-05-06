use bevy::prelude::*;

use crate::game::core::global_resources::UiIcons;

#[derive(Component, Debug, Clone, Copy)]
pub enum UiIconOption {
    X,
    Minimize,
    CornerResize,
}

pub fn make_ui_icons_resource(asset_server: &Res<AssetServer>) -> UiIcons {
    UiIcons {
        x: asset_server.load("gui/icons/material-symbols--close.png"),
        minimize: asset_server.load("gui/icons/material-symbols--minimize.png"),
        corner_resize: asset_server.load("gui/icons/material-symbols--resize-window.png"),
    }
}

pub fn apply_ui_icons(
    icons_res: Res<UiIcons>,
    mut image_q: Query<(&mut ImageNode, &UiIconOption), (Added<ImageNode>, With<UiIconOption>)>,
) {
    image_q.iter_mut().for_each(|(mut image_node, icon)| {
        image_node.image = match icon {
            UiIconOption::X => icons_res.x.clone(),
            UiIconOption::Minimize => icons_res.minimize.clone(),
            UiIconOption::CornerResize => icons_res.corner_resize.clone(),
        }
    });
}
