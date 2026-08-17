use bevy::prelude::*;

use crate::game::graphics::{
    post_processor::plugin::PostProcessorPlugin, primary_material::plugin::PrimaryMaterialPlugin,
    terrain_material::plugin::TerrainMaterialPlugin,
};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_plugins(PrimaryMaterialPlugin)
            .add_plugins(TerrainMaterialPlugin)
            .add_plugins(PostProcessorPlugin)
        ;
    }
}
