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

pub struct WaterUndersideMaterialPlugin;

impl Plugin for WaterUndersideMaterialPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_plugins(MaterialPlugin::<WaterUndersideMaterial>::default())
            .add_systems(Update,
                fix_storage_buffer_bug
            )
        ;
    }
}

pub struct WaterUndersideMaterialProps {}

impl Default for WaterUndersideMaterialProps {
    fn default() -> Self {
        Self {}
    }
}

pub type WaterUndersideMaterial = ExtendedMaterial<StandardMaterial, __WaterUndersideMaterialExtension>;

pub fn water_underside_material(
    _props: WaterUndersideMaterialProps,
    global_render_data_handle: Handle<ShaderStorageBuffer>,
) -> WaterUndersideMaterial {
    WaterUndersideMaterial {
        base: StandardMaterial {
            base_color: Color::WHITE.with_alpha(0.9),
            alpha_mode: AlphaMode::Blend,
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
            fog_enabled: false,
            ..default()
        },
        extension: __WaterUndersideMaterialExtension {
            global_render_data_handle: global_render_data_handle,
            texturing_scale: 1.,
        },
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct __WaterUndersideMaterialExtension {
    #[storage(100, read_only)]
    pub global_render_data_handle: Handle<ShaderStorageBuffer>,
    #[uniform(101)]
    texturing_scale: f32,
}

impl MaterialExtension for __WaterUndersideMaterialExtension {
    fn vertex_shader() -> ShaderRef {
        "shaders/water_underside_material.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/water_underside_material.wgsl".into()
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

fn fix_storage_buffer_bug(mut materials: ResMut<Assets<WaterUndersideMaterial>>) {
    materials.iter_mut().for_each(|_| {});
}
