use bevy::prelude::*;

use crate::game::graphics::primary_shader::plugin::{
    PrimaryShaderMaterial, PrimaryShaderMaterialProps, primary_shader_material,
};

#[derive(Resource)]
pub struct ReusableMaterials {
    pub terrain: Handle<PrimaryShaderMaterial>,
}

impl ReusableMaterials {
    pub fn new(materials: &mut ResMut<Assets<PrimaryShaderMaterial>>) -> Self {
        Self {
            terrain: materials.add(primary_shader_material(PrimaryShaderMaterialProps {
                texturing_scale: 1.,
            })),
        }
    }
}
