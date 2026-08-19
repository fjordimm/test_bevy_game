use bevy::{prelude::*, render::storage::ShaderStorageBuffer};

use crate::game::graphics::{
    primary_material::plugin::PrimaryMaterial,
    terrain_material::plugin::{TerrainMaterial, terrain_material},
};

#[derive(Resource)]
pub struct ReusableMaterials {
    pub terrain: Handle<TerrainMaterial>,
}

impl ReusableMaterials {
    pub fn new(
        global_render_data_handle: Handle<ShaderStorageBuffer>,
        _materials_primary: &mut ResMut<Assets<PrimaryMaterial>>,
        materials_terrain: &mut ResMut<Assets<TerrainMaterial>>,
    ) -> Self {
        Self {
            terrain: materials_terrain.add(terrain_material(default(), global_render_data_handle)),
        }
    }
}
