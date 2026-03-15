use bevy::prelude::*;

use crate::game::egui_setup::tags::CameraForEgui;

#[derive(Component)]
pub struct MainMenuStateEntity;

#[derive(Component, Clone, Debug, PartialEq, Eq, Hash)]
#[require(CameraForEgui)]
pub struct MainMenuStateCameraForEgui;
