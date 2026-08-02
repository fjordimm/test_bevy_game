use std::f32::consts::PI;

use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::render_resource::*,
    shader::ShaderRef,
};
use bevy_mesh::MeshVertexBufferLayoutRef;

use crate::game::{
    core::states::OverallState,
    playing_state::{
        player::tags::CameraForPlayer,
        sets::{DuringPlayingUnpaused, OnEnterPlaying},
        skybox::ComputedSkyboxValues,
        tags::PlayingStateEntity,
        world::{SeasonOfYear, TimeOfDay},
    },
    util::{alrms, alrro},
};

pub struct SkyboxPlugin;

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_plugins(MaterialPlugin::<SkyboxMaterial>::default())
            .insert_resource(ComputedSkyboxValues { sun_position: Vec3::NEG_Z, sky_rotation_inv: Mat3::IDENTITY })
            .add_systems(OnEnter(OverallState::Playing),
                spawn_skybox
                    .in_set(OnEnterPlaying::General)
            )
            .add_systems(Update,
                update_skybox
                    .in_set(DuringPlayingUnpaused::General)
            )
        ;
    }
}

#[derive(Component)]
pub struct SkyboxTag;

fn spawn_skybox(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SkyboxMaterial>>,
    time_of_day: Res<TimeOfDay>,
    season_of_year: Res<SeasonOfYear>,
    mut computed_skybox_values: ResMut<ComputedSkyboxValues>,
) {
    compute_skybox_values(time_of_day.0, season_of_year.0, &mut computed_skybox_values);

    commands.spawn((
        PlayingStateEntity,
        SkyboxTag,
        Mesh3d(meshes.add(alrro!(
            Mesh::from(Sphere::new(1_000_000.)).with_inverted_winding()
        ))),
        MeshMaterial3d(materials.add(SkyboxMaterial {
            sun_position: computed_skybox_values.sun_position,
            sky_rotation_inv: computed_skybox_values.sky_rotation_inv,
        })),
        Transform::default(),
    ));
}

fn update_skybox(
    camera_transf_q: Option<Single<&Transform, With<CameraForPlayer>>>,
    skybox_transf_q: Option<Single<&mut Transform, (With<SkyboxTag>, Without<CameraForPlayer>)>>,
    mut materials: ResMut<Assets<SkyboxMaterial>>,
    time_of_day: Res<TimeOfDay>,
    season_of_year: Res<SeasonOfYear>,
    mut computed_skybox_values: ResMut<ComputedSkyboxValues>,
) {
    // Move it to be the same position as the camera.

    if let (Some(camera_transf), Some(mut skybox_transf)) =
        (alrms!(camera_transf_q), alrms!(skybox_transf_q))
    {
        skybox_transf.translation = camera_transf.translation;
    }

    // Update the shader uniforms.

    compute_skybox_values(time_of_day.0, season_of_year.0, &mut computed_skybox_values);

    materials.iter_mut().for_each(|(_, mat)| {
        mat.sun_position = computed_skybox_values.sun_position;
        mat.sky_rotation_inv = computed_skybox_values.sky_rotation_inv;
    });
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SkyboxMaterial {
    #[uniform(0)]
    pub sun_position: Vec3,
    #[uniform(1)]
    pub sky_rotation_inv: Mat3,
}

impl Material for SkyboxMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/sky.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/sky.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }

    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout
            .0
            .get_layout(&[Mesh::ATTRIBUTE_POSITION.at_shader_location(0)])?;

        descriptor.vertex.buffers = vec![vertex_layout];

        Ok(())
    }
}

fn compute_skybox_values(
    time_of_day: f32,
    season_of_year: f32,
    computed_skybox_values: &mut ComputedSkyboxValues,
) {
    computed_skybox_values.sun_position = Vec3::NEG_Z.rotate_x(time_of_day * PI);
    computed_skybox_values.sun_position = computed_skybox_values
        .sun_position
        .rotate_z(season_of_year * PI);

    computed_skybox_values.sky_rotation_inv = Mat3::from_rotation_x(-time_of_day * PI);
}
