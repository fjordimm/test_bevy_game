use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        render_resource::{AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError},
        storage::ShaderStorageBuffer,
    },
    shader::ShaderRef,
};
use bevy_mesh::MeshVertexBufferLayoutRef;

pub struct SkyboxMaterialPlugin;

impl Plugin for SkyboxMaterialPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_plugins(MaterialPlugin::<SkyboxMaterial>::default())
            .add_systems(Update,
                fix_storage_buffer_bug
            )
        ;
    }
}

const SHADER_ASSET_PATH: &str = "shaders/materials/skybox_material.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SkyboxMaterial {
    #[storage(0, read_only)]
    pub global_render_data_handle: Handle<ShaderStorageBuffer>,
}

impl Material for SkyboxMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }

    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout
            .0
            .get_layout(&[Mesh::ATTRIBUTE_POSITION.at_shader_location(0)])?;

        descriptor.vertex.buffers = vec![vertex_layout];

        Ok(())
    }
}

fn fix_storage_buffer_bug(mut materials: ResMut<Assets<SkyboxMaterial>>) {
    materials.iter_mut().for_each(|_| {});
}
