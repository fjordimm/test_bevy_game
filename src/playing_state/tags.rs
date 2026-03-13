use bevy::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;

#[derive(Component)]
pub struct PlayingStateEntity;

#[derive(Component, ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PlayingStateCameraForEgui;
