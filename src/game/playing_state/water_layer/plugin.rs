use std::{f32::consts::PI, time::Duration};

use bevy::{math::DVec3, prelude::*, time::common_conditions::on_timer};

use crate::game::{
    core::states::OverallState,
    geometry::water_layer::water_layer,
    graphics::{
        global_render_data::resources::{GlobalRenderData, GlobalRenderDataHandle},
        water_material::plugin::{WaterMaterial, water_material},
        water_underside_material::plugin::{WaterUndersideMaterial, water_underside_material},
    },
    playing_state::{
        coord_rebasing::{
            CoordRebasingOrigin, to_transf_space, to_world_space, world_space_transf,
        },
        player::tags::PlayerBody,
        sets::{DuringPlayingUnpaused, OnEnterPlaying},
        tags::{PlayingStateEntity, PrimaryCamera},
    },
    util::{alrms, alrrs},
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
                relocate_to_player_xz
                    .in_set(DuringPlayingUnpaused::General)
                    .run_if(on_timer(Duration::from_millis(RELOCATE_INTERVAL)))
            )
            .add_systems(Update,
                update_cam_is_underwater
                    .in_set(DuringPlayingUnpaused::General)
            )
        ;
    }
}

const WATER_LAYER_HEIGHT: f64 = 25.0;

const RELOCATE_INTERVAL: u64 = 5000;

#[derive(Component)]
struct WaterLayerTopside;

#[derive(Component)]
struct WaterLayerUnderside;

fn on_enter(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_w: ResMut<Assets<WaterMaterial>>,
    mut materials_wu: ResMut<Assets<WaterUndersideMaterial>>,
    global_render_data_handle: Res<GlobalRenderDataHandle>,
    coord_rebasing_origin: Res<CoordRebasingOrigin>,
) {
    commands.spawn((
        PlayingStateEntity,
        Mesh3d(meshes.add(water_layer())),
        MeshMaterial3d(materials_w.add(water_material(
            default(),
            global_render_data_handle.get_handle(),
        ))),
        world_space_transf(
            Transform::from_translation(to_transf_space(
                DVec3::new(0.0, WATER_LAYER_HEIGHT, 0.0),
                &coord_rebasing_origin,
            ))
            .with_scale(Vec3::splat(15_000.0)),
        ),
        WaterLayerTopside,
    ));
    commands.spawn((
        PlayingStateEntity,
        Mesh3d(meshes.add(water_layer())),
        MeshMaterial3d(materials_wu.add(water_underside_material(
            default(),
            global_render_data_handle.get_handle(),
        ))),
        world_space_transf(
            Transform::from_translation(to_transf_space(
                DVec3::new(0.0, WATER_LAYER_HEIGHT, 0.0),
                &coord_rebasing_origin,
            ))
            .with_scale(Vec3::splat(15_000.0).rotate_x(PI)),
        ),
        WaterLayerUnderside,
    ));
}

fn relocate_to_player_xz(
    player_body_q: Option<Single<&Transform, With<PlayerBody>>>,
    water_layer_topside_q: Option<
        Single<
            &mut Transform,
            (
                With<WaterLayerTopside>,
                Without<PlayerBody>,
                Without<WaterLayerUnderside>,
            ),
        >,
    >,
    water_layer_underside_q: Option<
        Single<
            &mut Transform,
            (
                With<WaterLayerUnderside>,
                Without<PlayerBody>,
                Without<WaterLayerTopside>,
            ),
        >,
    >,
) {
    if let Some(player_body) = alrms!(player_body_q) {
        let mut water_layer_topside = alrrs!(water_layer_topside_q);
        water_layer_topside.translation.x = player_body.translation.x;
        water_layer_topside.translation.z = player_body.translation.z;

        let mut water_layer_underside = alrrs!(water_layer_underside_q);
        water_layer_underside.translation.x = player_body.translation.x;
        water_layer_underside.translation.z = player_body.translation.z;
    }
}

fn update_cam_is_underwater(
    camera_q: Option<Single<&Transform, With<PrimaryCamera>>>,
    mut global_render_data: ResMut<GlobalRenderData>,
    coord_rebasing_origin: Res<CoordRebasingOrigin>,
) {
    if let Some(camera_transf) = alrms!(camera_q) {
        global_render_data.cam_is_underwater =
            (to_world_space(camera_transf.translation, &coord_rebasing_origin).y
                <= WATER_LAYER_HEIGHT) as u32;
    }
}
