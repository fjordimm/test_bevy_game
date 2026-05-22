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

const DAY_ZENITH_COLOR: Color = Color::hsv(210.0, 0.8, 0.97);
const DAY_HORIZON_COLOR: Color = Color::hsv(210.0, 0.5, 1.0);
const NIGHT_ZENITH_COLOR: Color = Color::hsv(210.0, 0.8, 0.1);
const NIGHT_HORIZON_COLOR: Color = Color::hsv(210.0, 0.5, 0.1);

#[derive(Component)]
pub struct SkyboxTag;

fn spawn_skybox(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SkyboxMaterial>>,
    sun_position: Res<SunPosition>,
) {
    let dzcol = DAY_ZENITH_COLOR.to_linear();
    let dhcol = DAY_HORIZON_COLOR.to_linear();
    let nzcol = NIGHT_ZENITH_COLOR.to_linear();
    let nhcol = NIGHT_HORIZON_COLOR.to_linear();

    commands.spawn((
        PlayingStateEntity,
        SkyboxTag,
        Mesh3d(meshes.add(alrro!(
            Mesh::from(Sphere::new(10_000.0)).with_inverted_winding()
        ))),
        MeshMaterial3d(materials.add(SkyboxMaterial {
            day_zenith_color: Vec3::new(dzcol.red, dzcol.green, dzcol.blue),
            day_horizon_color: Vec3::new(dhcol.red, dhcol.green, dhcol.blue),
            night_zenith_color: Vec3::new(nzcol.red, nzcol.green, nzcol.blue),
            night_horizon_color: Vec3::new(nhcol.red, nhcol.green, nhcol.blue),
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
    pub day_zenith_color: Vec3,
    #[uniform(1)]
    pub day_horizon_color: Vec3,
    #[uniform(2)]
    pub night_zenith_color: Vec3,
    #[uniform(3)]
    pub night_horizon_color: Vec3,
    #[uniform(4)]
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
