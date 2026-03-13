use bevy::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;

#[derive(Component)]
pub struct MainMenuStateEntity;

#[derive(Component, ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MainMenuStateCameraForEgui;
