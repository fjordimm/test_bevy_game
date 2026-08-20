use std::f32::consts::PI;

use bevy::prelude::*;

use crate::game::{
    core::states::OverallState,
    geometry::water_layer::water_layer,
    graphics::{
        global_render_data::resources::{GlobalRenderData, GlobalRenderDataHandle},
        water_material::plugin::{WaterMaterial, water_material},
        water_underside_material::plugin::{WaterUndersideMaterial, water_underside_material},
    },
    playing_state::{
        player::tags::CameraForPlayer,
        sets::{DuringPlayingUnpaused, OnEnterPlaying},
        tags::PlayingStateEntity,
    },
    util::alrms,
};

pub struct WaterLayerPlugin;

impl Plugin for WaterLayerPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::Playing),
                on_enter
                    .in_set(OnEnterPlaying::General)
            )
            .add_systems(Update,
                update_cam_is_underwater
                    .in_set(DuringPlayingUnpaused::General)
            )
        ;
    }
}

const WATER_LAYER_HEIGHT: f32 = 25.;

fn on_enter(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_w: ResMut<Assets<WaterMaterial>>,
    mut materials_wu: ResMut<Assets<WaterUndersideMaterial>>,
    global_render_data_handle: Res<GlobalRenderDataHandle>,
) {
    commands.spawn((
        PlayingStateEntity,
        Mesh3d(meshes.add(water_layer())),
        MeshMaterial3d(materials_w.add(water_material(
            default(),
            global_render_data_handle.get_handle(),
        ))),
        Transform::from_xyz(0., WATER_LAYER_HEIGHT, 0.).with_scale(Vec3::splat(15_000.)),
    ));
    commands.spawn((
        PlayingStateEntity,
        Mesh3d(meshes.add(water_layer())),
        MeshMaterial3d(materials_wu.add(water_underside_material(
            default(),
            global_render_data_handle.get_handle(),
        ))),
        Transform::from_xyz(0., WATER_LAYER_HEIGHT, 0.)
            .with_scale(Vec3::splat(15_000.).rotate_x(PI)),
    ));
}

fn update_cam_is_underwater(
    camera_q: Option<Single<&Transform, With<CameraForPlayer>>>,
    mut global_render_data: ResMut<GlobalRenderData>,
) {
    if let Some(camera_transf) = alrms!(camera_q) {
        global_render_data.cam_is_underwater =
            (camera_transf.translation.y <= WATER_LAYER_HEIGHT) as u32;
    }
}
