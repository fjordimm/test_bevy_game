use bevy::prelude::*;

// 'T' stands for 'time of day'.
#[derive(Resource)]
pub struct SkyRotationT(pub f32);

// 'S' stands for 'season'.
#[derive(Resource)]
pub struct SkyRotationS(pub f32);
