use bevy::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;

#[derive(Component, ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct CameraForEgui;
