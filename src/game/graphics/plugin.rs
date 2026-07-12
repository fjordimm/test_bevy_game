use bevy::prelude::*;

use crate::game::graphics::{
    post_processor::plugin::PostProcessorPlugin, primary_shader::plugin::PrimaryShaderPlugin,
};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_plugins(PrimaryShaderPlugin)
            .add_plugins(PostProcessorPlugin)
        ;
    }
}
