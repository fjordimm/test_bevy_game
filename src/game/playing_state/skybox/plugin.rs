use std::f32::consts::PI;

use bevy::prelude::*;

use crate::game::{
    core::states::OverallState,
    graphics::{
        global_render_data::resources::{GlobalRenderData, GlobalRenderDataHandle},
        skybox_material::plugin::SkyboxMaterial,
    },
    playing_state::{
        environment_light::{SeasonOfYear, TimeOfDay},
        player::tags::CameraForPlayer,
        sets::{DuringPlayingUnpaused, OnEnterPlaying},
        tags::PlayingStateEntity,
    },
    util::{alrms, alrro},
};

pub struct SkyboxPlugin;

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
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
    mut global_render_data: ResMut<GlobalRenderData>,
    global_render_data_handle: Res<GlobalRenderDataHandle>,
) {
    commands.spawn((
        PlayingStateEntity,
        SkyboxTag,
        Mesh3d(meshes.add(alrro!(
            Mesh::from(Sphere::new(1_000_000.)).with_inverted_winding()
        ))),
        MeshMaterial3d(materials.add(SkyboxMaterial {
            global_render_data: global_render_data_handle.get_handle(),
        })),
        Transform::default(),
    ));

    compute_global_render_data_vals(time_of_day.0, season_of_year.0, &mut global_render_data);
}

fn update_skybox(
    camera_transf_q: Option<Single<&Transform, With<CameraForPlayer>>>,
    skybox_transf_q: Option<Single<&mut Transform, (With<SkyboxTag>, Without<CameraForPlayer>)>>,
    time_of_day: Res<TimeOfDay>,
    season_of_year: Res<SeasonOfYear>,
    mut global_render_data: ResMut<GlobalRenderData>,
) {
    // Move it to be the same position as the camera.

    if let (Some(camera_transf), Some(mut skybox_transf)) =
        (alrms!(camera_transf_q), alrms!(skybox_transf_q))
    {
        skybox_transf.translation = camera_transf.translation;
    }

    compute_global_render_data_vals(time_of_day.0, season_of_year.0, &mut global_render_data);
}

fn compute_global_render_data_vals(
    time_of_day: f32,
    season_of_year: f32,
    global_render_data: &mut ResMut<GlobalRenderData>,
) {
    global_render_data.sun_position = Vec3::NEG_Z.rotate_x(time_of_day * 2. * PI);
    global_render_data.sun_position = global_render_data
        .sun_position
        .rotate_z(season_of_year * 2. * PI);

    global_render_data.sky_rotation_inv = Mat3::from_rotation_x(-time_of_day * 2. * PI);
}
