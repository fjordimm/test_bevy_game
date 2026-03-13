use bevy::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;

#[derive(Component)]
pub struct MenuStateEntity;

#[derive(Component, ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MenuStateCameraForEgui;
