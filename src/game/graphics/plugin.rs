use bevy::prelude::*;

use crate::game::graphics::{
    global_render_data::plugin::GlobalRenderDataPlugin,
    post_processor::plugin::PostProcessorPlugin, primary_material::plugin::PrimaryMaterialPlugin,
    skybox_material::plugin::SkyboxMaterialPlugin, terrain_material::plugin::TerrainMaterialPlugin,
};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_plugins(GlobalRenderDataPlugin)
            .add_plugins(SkyboxMaterialPlugin)
            .add_plugins(PrimaryMaterialPlugin)
            .add_plugins(TerrainMaterialPlugin)
            .add_plugins(PostProcessorPlugin)
        ;
    }
}
