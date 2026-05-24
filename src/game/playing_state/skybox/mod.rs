use bevy::prelude::*;

pub mod plugin;

#[derive(Resource)]
pub struct SunPosition(pub Vec3);

#[derive(Resource)]
pub struct SkyRotationInv(pub Mat3);
