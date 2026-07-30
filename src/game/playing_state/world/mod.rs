use bevy::prelude::*;

pub mod plugin;
pub mod terrain;

#[derive(Resource)]
pub struct TimeOfDay(pub f32);

#[derive(Resource)]
pub struct SeasonOfYear(pub f32);
