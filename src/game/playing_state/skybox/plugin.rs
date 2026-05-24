use bevy::{prelude::*, render::render_resource::*, shader::ShaderRef};

use crate::game::{
    core::states::OverallState,
    playing_state::{
        SunPosition, player::tags::CameraForPlayer, sets::DuringPlayingUnpaused,
        tags::PlayingStateEntity,
    },
    util::{alrms, alrro},
};

pub struct SkyboxPlugin;

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_plugins(MaterialPlugin::<SkyboxMaterial>::default())
            .add_systems(OnEnter(OverallState::Playing),
                spawn_skybox
            )
            .add_systems(Update,
                update_skybox
                    .in_set(DuringPlayingUnpaused::General)
            );
    }
}

#[derive(Component)]
pub struct SkyboxTag;

fn spawn_skybox(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SkyboxMaterial>>,
    sun_position: Res<SunPosition>,
) {
    commands.spawn((
        PlayingStateEntity,
        SkyboxTag,
        Mesh3d(meshes.add(alrro!(
            Mesh::from(Sphere::new(10_000.0)).with_inverted_winding()
        ))),
        MeshMaterial3d(materials.add(SkyboxMaterial {
            sun_position: sun_position.0,
        })),
        Transform::default(),
    ));
}

fn update_skybox(
    camera_transf_q: Option<Single<&Transform, With<CameraForPlayer>>>,
    skybox_transf_q: Option<Single<&mut Transform, (With<SkyboxTag>, Without<CameraForPlayer>)>>,
    mut materials: ResMut<Assets<SkyboxMaterial>>,
    sun_position: Res<SunPosition>,
) {
    // Move it to be the same position as the camera.

    if let (Some(camera_transf), Some(mut skybox_transf)) =
        (alrms!(camera_transf_q), alrms!(skybox_transf_q))
    {
        skybox_transf.translation = camera_transf.translation;
    }

    // Update the shader uniforms.

    materials.iter_mut().for_each(|(_, mat)| {
        mat.sun_position = sun_position.0;
    });
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SkyboxMaterial {
    #[uniform(0)]
    pub sun_position: Vec3,
}

impl Material for SkyboxMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/sky.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
}
