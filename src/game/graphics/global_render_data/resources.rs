use bevy::{
    prelude::*,
    render::{render_resource::ShaderType, storage::ShaderStorageBuffer},
};

#[derive(Resource, ShaderType, Clone)]
pub struct GlobalRenderData {
    pub time_elapsed: f32,
    pub sun_position: Vec3,
    pub sky_rotation_inv: Mat3,
    pub cam_is_underwater: u32,
    _padding: [u32; 2],
}

impl Default for GlobalRenderData {
    fn default() -> Self {
        Self {
            time_elapsed: 0.,
            sun_position: Vec3::Y,
            sky_rotation_inv: Mat3::IDENTITY,
            cam_is_underwater: 0,
            _padding: default(),
        }
    }
}

#[derive(Resource)]
pub struct GlobalRenderDataHandle(pub(super) Handle<ShaderStorageBuffer>);

impl GlobalRenderDataHandle {
    pub fn get_handle(&self) -> Handle<ShaderStorageBuffer> {
        self.0.clone()
    }
}
