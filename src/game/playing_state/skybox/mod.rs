use bevy::prelude::*;

pub mod plugin;

// Only to be modified by skybox::plugin.
#[derive(Resource)]
pub struct ComputedSkyboxValues {
    pub sun_position: Vec3,
    pub sky_rotation_inv: Mat3,
}
