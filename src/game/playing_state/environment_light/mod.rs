use bevy::prelude::*;

pub mod plugin;

// TODO: put these in a new file 'resources.rs' in this module.

#[derive(Resource)]
pub struct TimeOfDay(pub f32);

#[derive(Resource)]
pub struct SeasonOfYear(pub f32);
