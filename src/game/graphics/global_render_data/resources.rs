use bevy::{
    prelude::*,
    render::{render_resource::ShaderType, storage::ShaderStorageBuffer},
};

#[derive(Resource, Default, ShaderType, Clone)]
pub struct GlobalRenderData {
    pub sun_position: Vec3,
    pub sky_rotation_inv: Mat3,
}

#[derive(Resource)]
pub struct GlobalRenderDataHandle(pub(super) Handle<ShaderStorageBuffer>);

impl GlobalRenderDataHandle {
    pub fn get_handle(&self) -> Handle<ShaderStorageBuffer> {
        self.0.clone()
    }
}
