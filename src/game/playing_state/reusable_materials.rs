use bevy::prelude::*;

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
        _materials_primary: &mut ResMut<Assets<PrimaryMaterial>>,
        materials_terrain: &mut ResMut<Assets<TerrainMaterial>>,
    ) -> Self {
        Self {
            terrain: materials_terrain.add(terrain_material(default())),
        }
    }
}
