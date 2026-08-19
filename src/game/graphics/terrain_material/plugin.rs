use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline},
    prelude::*,
    render::{
        render_resource::{
            AsBindGroup, Face, RenderPipelineDescriptor, SpecializedMeshPipelineError,
        },
        storage::ShaderStorageBuffer,
    },
    shader::ShaderRef,
};
use bevy_mesh::MeshVertexBufferLayoutRef;

pub struct TerrainMaterialPlugin;

impl Plugin for TerrainMaterialPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_plugins(MaterialPlugin::<TerrainMaterial>::default())
            .add_systems(Update,
                fix_storage_buffer_bug
            )
        ;
    }
}

pub struct TerrainMaterialProps {
    pub texturing_scale: f32,
}

impl Default for TerrainMaterialProps {
    fn default() -> Self {
        Self {
            texturing_scale: 1.,
        }
    }
}

pub type TerrainMaterial = ExtendedMaterial<StandardMaterial, __TerrainMaterialExtension>;

pub fn terrain_material(
    props: TerrainMaterialProps,
    global_render_data_handle: Handle<ShaderStorageBuffer>,
) -> TerrainMaterial {
    TerrainMaterial {
        base: StandardMaterial {
            perceptual_roughness: 1.,
            metallic: 0.,
            reflectance: 0.,
            diffuse_transmission: 0.,
            specular_transmission: 0.,
            thickness: 0.,
            ior: 1.5,
            attenuation_distance: 1.0,
            double_sided: false,
            cull_mode: Some(Face::Back),
            fog_enabled: true,
            ..default()
        },
        extension: __TerrainMaterialExtension {
            global_render_data_handle: global_render_data_handle,
            texturing_scale: props.texturing_scale,
        },
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct __TerrainMaterialExtension {
    #[storage(100, read_only)]
    pub global_render_data_handle: Handle<ShaderStorageBuffer>,
    #[uniform(101)]
    texturing_scale: f32,
}

impl MaterialExtension for __TerrainMaterialExtension {
    fn vertex_shader() -> ShaderRef {
        "shaders/terrain_material.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/terrain_material.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout
            .0
            .get_layout(&[Mesh::ATTRIBUTE_POSITION.at_shader_location(0)])?;

        descriptor.vertex.buffers = vec![vertex_layout];

        Ok(())
    }
}

fn fix_storage_buffer_bug(mut materials: ResMut<Assets<TerrainMaterial>>) {
    materials.iter_mut().for_each(|_| {});
}
