use std::time::Duration;

use avian3d::{collision::collider::Collider, dynamics::rigid_body::RigidBody};
use bevy::{input::mouse::MouseWheel, prelude::*, time::common_conditions::on_timer};
use rand_distr::num_traits::Pow;

use crate::game::{
    core::states::OverallState,
    geometry::{cube::cube_mesh, dodec::dodec_mesh},
    graphics::{
        global_render_data::resources::GlobalRenderDataHandle,
        primary_material::plugin::{PrimaryMaterial, primary_material},
    },
    playing_state::{
        coord_rebasing::world_space_transf,
        environment_light::resources::{SkyRotationS, SkyRotationT},
        player::{
            resources::PlayerMovementSettings,
            tags::{CameraForPlayer, PlayerTransf},
        },
        sets::{DuringPlayingUnpaused, OnEnterPlaying},
        tags::PlayingStateEntity,
    },
    util::alrms,
};

pub struct QuickDevTestPlugin;

impl Plugin for QuickDevTestPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update,
                after_a_sec
                    .run_if(on_timer(Duration::from_secs(1)).and(run_once))
            )
            .add_systems(Update,
                scrolling
                    .in_set(DuringPlayingUnpaused::General)
            )
            .add_systems(Update,
                move_player_body_to_cam
                    .in_set(DuringPlayingUnpaused::General)
            )
            .add_systems(OnEnter(OverallState::Playing),
                spawn_some_stuff
                    .in_set(OnEnterPlaying::General)
            )
        ;
    }
}

fn after_a_sec(/* mut gui_scale: ResMut<GuiScale> */) {
    // gui_scale.0 = 5.0;
}

fn scrolling(
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    mut time_of_day: ResMut<SkyRotationT>,
    mut season_of_year: ResMut<SkyRotationS>,
    mut movement_settings: ResMut<PlayerMovementSettings>,
) {
    for mouse_wheel_msg in mouse_wheel_reader.read() {
        if keys.pressed(KeyCode::ControlLeft) {
            // Move sun.

            if keys.pressed(KeyCode::AltLeft) {
                season_of_year.0 += 0.03 * mouse_wheel_msg.y;
            } else {
                time_of_day.0 += -0.03 * mouse_wheel_msg.y;
            }
        } else {
            // Change movement speed.

            movement_settings.speed = movement_settings.speed.pow(1.0 - 0.01 * mouse_wheel_msg.y);

            if movement_settings.speed < 0.05 {
                movement_settings.speed = 0.05;
            }
            if movement_settings.speed > 10_000.0 {
                movement_settings.speed = 10_000.0;
            }
        }
    }
}

fn move_player_body_to_cam(
    keys: Res<ButtonInput<KeyCode>>,
    player_body_q: Option<Single<&mut Transform, With<PlayerTransf>>>,
    camera_q: Option<Single<&Transform, (With<CameraForPlayer>, Without<PlayerTransf>)>>,
) {
    if keys.pressed(KeyCode::BracketLeft) {
        if let Some(mut player_body) = alrms!(player_body_q) {
            if let Some(camera) = alrms!(camera_q) {
                player_body.translation.x = camera.translation.x;
                player_body.translation.z = camera.translation.z;
            }
        }
    }
}

fn spawn_some_stuff(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PrimaryMaterial>>,
    global_render_data_handle: Res<GlobalRenderDataHandle>,
) {
    commands.spawn((
        PlayingStateEntity,
        Mesh3d(meshes.add(dodec_mesh())),
        MeshMaterial3d(materials.add(primary_material(
            default(),
            global_render_data_handle.get_handle(),
        ))),
        world_space_transf(Transform::from_xyz(3.0, 0.0, -9.0)),
    ));

    commands.spawn((
        PlayingStateEntity,
        Mesh3d(meshes.add(cube_mesh())),
        MeshMaterial3d(materials.add(primary_material(
            default(),
            global_render_data_handle.get_handle(),
        ))),
        world_space_transf(
            Transform::from_xyz(0.0, 50.0, -20.0).with_scale(Vec3::new(100.0, 1.0, 100.0)),
        ),
        RigidBody::Static,
        Collider::cuboid(100.0, 1.0, 100.0),
    ));

    commands.spawn((
        PlayingStateEntity,
        Mesh3d(meshes.add(cube_mesh())),
        MeshMaterial3d(materials.add(primary_material(
            default(),
            global_render_data_handle.get_handle(),
        ))),
        world_space_transf({
            let mut transf = Transform::from_xyz(0.0, 70.0, -20.0);
            transf.rotate_z(0.1);
            transf.rotate_x(0.15);

            transf
        }),
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
    ));
}
