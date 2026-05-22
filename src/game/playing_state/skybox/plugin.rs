use bevy::{prelude::*, render::render_resource::*, shader::ShaderRef};

use crate::game::{
    core::states::OverallState,
    playing_state::{
        player::tags::CameraForPlayer, sets::DuringPlayingUnpaused, tags::PlayingStateEntity,
    },
    util::{alrms, alrro},
};

pub struct SkyboxPlugin;

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_plugins(MaterialPlugin::<SkyMaterial>::default())
            .add_systems(OnEnter(OverallState::Playing),
                spawn_skybox
            )
            .add_systems(Update,
                move_skybox_with_camera
                    .in_set(DuringPlayingUnpaused::General)
            );
    }
}

#[derive(Component)]
pub struct SkyboxTag;

fn spawn_skybox(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials: ResMut<Assets<SkyMaterial>>,
) {
    commands.spawn((
        PlayingStateEntity,
        SkyboxTag,
        Mesh3d(meshes.add(alrro!(
            Mesh::from(Sphere::new(1000.0)).with_inverted_winding()
        ))),
        MeshMaterial3d(materials.add(SkyMaterial {
            sun_direction: Vec3::new(0.4, 1.0, 0.7).normalize(),
        })),
        Transform::default(),
    ));
}

fn move_skybox_with_camera(
    camera_transf_q: Option<Single<&Transform, With<CameraForPlayer>>>,
    skybox_transf_q: Option<Single<&mut Transform, (With<SkyboxTag>, Without<CameraForPlayer>)>>,
) {
    if let (Some(camera_transf), Some(mut skybox_transf)) =
        (alrms!(camera_transf_q), alrms!(skybox_transf_q))
    {
        skybox_transf.translation = camera_transf.translation;
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SkyMaterial {
    #[uniform(0)]
    pub sun_direction: Vec3,
}

impl Material for SkyMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/sky.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
}
