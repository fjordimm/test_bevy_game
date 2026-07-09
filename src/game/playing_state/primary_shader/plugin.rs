use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline},
    prelude::*,
    render::render_resource::{
        AsBindGroup, Face, RenderPipelineDescriptor, SpecializedMeshPipelineError,
    },
    shader::ShaderRef,
};
use bevy_mesh::{MeshVertexAttribute, MeshVertexBufferLayoutRef, VertexFormat};

pub struct PrimaryShaderPlugin;

impl Plugin for PrimaryShaderPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_plugins(MaterialPlugin::<PrimaryShaderMaterial>::default())
        ;
    }
}

pub struct PrimaryShaderMaterialProps {
    pub base_color: Color,
    pub edge_color: Color,
    pub texture: Option<Handle<Image>>,
}

impl Default for PrimaryShaderMaterialProps {
    fn default() -> Self {
        Self {
            base_color: Color::WHITE,
            edge_color: Color::WHITE,
            texture: None,
        }
    }
}

pub type PrimaryShaderMaterial = ExtendedMaterial<StandardMaterial, __PrimaryShaderExtension>;

pub fn primary_shader_material(props: PrimaryShaderMaterialProps) -> PrimaryShaderMaterial {
    let edge_color = props.edge_color.to_linear();

    PrimaryShaderMaterial {
        base: StandardMaterial {
            base_color: props.base_color,
            base_color_texture: props.texture,
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
        extension: __PrimaryShaderExtension {
            edge_color: vec4(edge_color.red, edge_color.green, edge_color.blue, 1.),
        },
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct __PrimaryShaderExtension {
    #[uniform(100)]
    edge_color: Vec4,
}

pub const ATTRIBUTE_EDGE_NEARNESS_UV: MeshVertexAttribute =
    MeshVertexAttribute::new("EdgeNearnessUV", 54784352, VertexFormat::Float32x2);

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
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
            ATTRIBUTE_EDGE_NEARNESS_UV.at_shader_location(3),
        ])?;

        descriptor.vertex.buffers = vec![vertex_layout];

        Ok(())
    }
}
