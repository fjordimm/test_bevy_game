use bevy::prelude::*;

pub mod plugin;

#[derive(Resource)]
pub struct TimeOfDay(pub f32);
