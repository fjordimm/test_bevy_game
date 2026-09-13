use bevy::{prelude::*, render::storage::ShaderBuffer};

use crate::game::graphics::primary_material::plugin::{PrimaryMaterial, primary_material};

#[derive(Resource)]
pub struct ReusableMaterials {
    pub primary: Handle<PrimaryMaterial>,
}

impl ReusableMaterials {
    pub fn new(
        global_render_data_handle: Handle<ShaderBuffer>,
        primary_mat: &mut ResMut<Assets<PrimaryMaterial>>,
    ) -> Self {
        Self {
            primary: primary_mat.add(primary_material(default(), global_render_data_handle)),
        }
    }
}
