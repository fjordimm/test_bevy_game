use bevy::{prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef};

pub struct PhongPlugin;

impl Plugin for PhongPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_plugins(MaterialPlugin::<PhongMaterial>::default())
        ;
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PhongMaterial {
    #[uniform(0)]
    pub base_color: Vec4,
}

impl PhongMaterial {
    pub fn new(color: Color) -> Self {
        let col = color.to_linear();

        Self {
            base_color: vec4(col.red, col.green, col.blue, col.alpha),
        }
    }
}

impl Material for PhongMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/phong.frag.wgsl".into()
    }

    // TODO: do I need this?
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
}
