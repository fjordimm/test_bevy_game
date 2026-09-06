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

pub struct WaterMaterialPlugin;

impl Plugin for WaterMaterialPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_plugins(MaterialPlugin::<WaterMaterial>::default())
            .add_systems(Update,
                fix_storage_buffer_bug
            )
        ;
    }
}

const SHADER_ASSET_PATH: &str = "shaders/materials/water_material.wgsl";

pub const WATER_OPACITY: f32 = 0.4;

pub struct WaterMaterialProps {}

impl Default for WaterMaterialProps {
    fn default() -> Self {
        Self {}
    }
}

pub type WaterMaterial = ExtendedMaterial<StandardMaterial, __WaterMaterialExtension>;

pub fn water_material(
    _props: WaterMaterialProps,
    global_render_data_handle: Handle<ShaderStorageBuffer>,
) -> WaterMaterial {
    WaterMaterial {
        base: StandardMaterial {
            base_color: Color::WHITE.with_alpha(WATER_OPACITY),
            alpha_mode: AlphaMode::Blend,
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
        extension: __WaterMaterialExtension {
            global_render_data_handle: global_render_data_handle,
            texturing_scale: 1.0,
        },
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct __WaterMaterialExtension {
    #[storage(100, read_only)]
    pub global_render_data_handle: Handle<ShaderStorageBuffer>,
    #[uniform(101)]
    texturing_scale: f32,
}

impl MaterialExtension for __WaterMaterialExtension {
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
        let vertex_layout = layout
            .0
            .get_layout(&[Mesh::ATTRIBUTE_POSITION.at_shader_location(0)])?;

        descriptor.vertex.buffers = vec![vertex_layout];

        Ok(())
    }
}

fn fix_storage_buffer_bug(mut materials: ResMut<Assets<WaterMaterial>>) {
    materials.iter_mut().for_each(|_| {});
}
