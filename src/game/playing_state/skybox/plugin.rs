use std::f32::consts::PI;

use bevy::prelude::*;

use crate::game::{
    core::states::OverallState,
    geometry::cube::cube_mesh,
    graphics::{
        global_render_data::resources::{GlobalRenderData, GlobalRenderDataHandle},
        skybox_material::plugin::SkyboxMaterial,
    },
    playing_state::{
        environment_light::resources::{SkyRotationS, SkyRotationT},
        sets::{DuringPlaying, DuringPlayingUnpaused, OnEnterPlaying},
        tags::{PlayingStateEntity, PrimaryCamera},
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
                    .in_set(OnEnterPlaying::SpawnThings)
            )
            .add_systems(Update,
                update_skybox
                    .in_set(DuringPlaying::General)
                    .in_set(DuringPlayingUnpaused)
            )
        ;
    }
}

#[derive(Component, FromTemplate)]
pub struct SkyboxTag;

fn spawn_skybox(
    mut commands: Commands,
    time_of_day: Res<SkyRotationT>,
    season_of_year: Res<SkyRotationS>,
    mut global_render_data: ResMut<GlobalRenderData>,
    global_render_data_handle: Res<GlobalRenderDataHandle>,
) {
    commands.spawn_scene(bsn! {
        PlayingStateEntity
        Mesh3d(asset_value(alrro!(cube_mesh(default()).with_inverted_winding())))
        MeshMaterial3d::<SkyboxMaterial>(asset_value(SkyboxMaterial {
            global_render_data_handle: global_render_data_handle.get_handle(),
        }))
        Transform::from_scale(Vec3::splat(1_000_000.0))
        SkyboxTag
    });

    compute_global_render_data_vals(time_of_day.0, season_of_year.0, &mut global_render_data);
}

fn update_skybox(
    camera_transf_q: Option<Single<&Transform, With<PrimaryCamera>>>,
    skybox_transf_q: Option<Single<&mut Transform, (With<SkyboxTag>, Without<PrimaryCamera>)>>,
    time_of_day: Res<SkyRotationT>,
    season_of_year: Res<SkyRotationS>,
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
    global_render_data.sun_position = Vec3::NEG_Z
        .rotate_x(time_of_day * 2.0 * PI)
        .rotate_z(season_of_year * 2.0 * PI);

    global_render_data.sky_rotation_inv = Mat3::from_rotation_x(-time_of_day * 2.0 * PI)
        * Mat3::from_rotation_z(-season_of_year * 2.0 * PI);
}
