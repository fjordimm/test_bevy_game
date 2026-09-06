use bevy::{prelude::*, render::storage::ShaderStorageBuffer};

use crate::game::graphics::primary_material::plugin::PrimaryMaterial;

#[derive(Resource)]
pub struct ReusableMaterials {}

impl ReusableMaterials {
    pub fn new(
        _global_render_data_handle: Handle<ShaderStorageBuffer>,
        _materials_primary: &mut ResMut<Assets<PrimaryMaterial>>,
    ) -> Self {
        Self {}
    }
}
