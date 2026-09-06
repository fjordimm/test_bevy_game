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

#[cfg(feature = "terrain_debug_cols")]
use crate::game::util::alrmo;

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

const SHADER_ASSET_PATH: &str = "shaders/materials/terrain_material.wgsl";

pub struct TerrainMaterialProps {}

impl Default for TerrainMaterialProps {
    fn default() -> Self {
        Self {}
    }
}

pub type TerrainMaterial = ExtendedMaterial<StandardMaterial, __TerrainMaterialExtension>;

pub fn terrain_material(
    _props: TerrainMaterialProps,
    texture: Handle<Image>,
    global_render_data_handle: Handle<ShaderStorageBuffer>,
) -> TerrainMaterial {
    TerrainMaterial {
        base: StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            metallic: 0.0,
            reflectance: 0.0,
            diffuse_transmission: 0.0,
            specular_transmission: 0.0,
            thickness: 0.0,
            ior: 1.5,
            attenuation_distance: 1.0,
            double_sided: false,
            cull_mode: Some(Face::Back),
            fog_enabled: false,
            ..default()
        },
        extension: __TerrainMaterialExtension {
            global_render_data_handle: global_render_data_handle,
            texture: texture,
        },
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct __TerrainMaterialExtension {
    #[storage(100, read_only)]
    pub global_render_data_handle: Handle<ShaderStorageBuffer>,
    #[texture(101)]
    #[sampler(102)]
    texture: Handle<Image>,
}

impl MaterialExtension for __TerrainMaterialExtension {
    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
            #[cfg(feature = "terrain_debug_cols")]
            Mesh::ATTRIBUTE_COLOR.at_shader_location(2),
        ])?;

        descriptor.vertex.buffers = vec![vertex_layout];

        #[cfg(feature = "terrain_debug_cols")]
        {
            descriptor
                .vertex
                .shader_defs
                .push("FEATURE_TERRAIN_DEBUG_COLS".into());
            if let Some(fragment) = alrmo!(descriptor.fragment_mut()) {
                fragment
                    .shader_defs
                    .push("FEATURE_TERRAIN_DEBUG_COLS".into());
            }
        }

        Ok(())
    }
}

fn fix_storage_buffer_bug(mut materials: ResMut<Assets<TerrainMaterial>>) {
    materials.iter_mut().for_each(|_| {});
}
