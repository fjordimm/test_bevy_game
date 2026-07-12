use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline},
    prelude::*,
    render::render_resource::{
        AsBindGroup, Face, RenderPipelineDescriptor, SpecializedMeshPipelineError,
    },
    shader::ShaderRef,
};
use bevy_mesh::MeshVertexBufferLayoutRef;

pub struct PrimaryShaderPlugin;

impl Plugin for PrimaryShaderPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_plugins(MaterialPlugin::<PrimaryShaderMaterial>::default())
        ;
    }
}

pub struct PrimaryShaderMaterialProps {}

impl Default for PrimaryShaderMaterialProps {
    fn default() -> Self {
        Self {}
    }
}

pub type PrimaryShaderMaterial = ExtendedMaterial<StandardMaterial, __PrimaryShaderExtension>;

pub fn primary_shader_material(_props: PrimaryShaderMaterialProps) -> PrimaryShaderMaterial {
    PrimaryShaderMaterial {
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
        extension: __PrimaryShaderExtension {},
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct __PrimaryShaderExtension {
    // TODOr
    // #[uniform(100)]
    // edge_color: Vec4,
}

// TODOr
// pub const ATTRIBUTE_POLYGONITY0: MeshVertexAttribute =
//     MeshVertexAttribute::new("Polygonity0", 54784352, VertexFormat::Float32x4);

impl MaterialExtension for __PrimaryShaderExtension {
    fn vertex_shader() -> ShaderRef {
        "shaders/primary.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/primary.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_COLOR.at_shader_location(1),
            // ATTRIBUTE_POLYGONITY0.at_shader_location(2), // TODOr
        ])?;

        descriptor.vertex.buffers = vec![vertex_layout];

        Ok(())
    }
}
