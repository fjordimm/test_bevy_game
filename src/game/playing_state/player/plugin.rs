use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::game::{
    core::{resources::KeyBindings, states::OverallState},
    geometry::cube::{CubeMeshColors, cube_mesh},
    graphics::primary_material::plugin::PrimaryMaterial,
    playing_state::{
        player::{
            resources::{FreecamEnabled, PlayerMovementSettings},
            tags::PlayerBody,
        },
        reusable_materials::ReusableMaterials,
        sets::{DuringPlaying, DuringPlayingUnpaused, OnEnterPlaying},
        tags::{PlayingStateEntity, PrimaryCamera},
    },
    util::{alrms, alrrs},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::Playing),
                on_enter
                    .in_set(OnEnterPlaying::ResourceSetup)
            )
            .add_systems(OnEnter(OverallState::Playing),
                spawn_player_body
                    .in_set(OnEnterPlaying::SpawnThings)
            )
            .add_systems(Update,
                rotate_and_move
                    .in_set(DuringPlaying::General)
                    .in_set(DuringPlayingUnpaused)
            )
        ;
    }
}

#[derive(Resource)]
struct RotO(Option<(f32, f32)>); // (yaw, pitch)

fn on_enter(mut commands: Commands) {
    commands.insert_resource(PlayerMovementSettings::default());
    commands.insert_resource(RotO(None));
    commands.insert_resource(FreecamEnabled(false));
}

fn spawn_player_body(mut commands: Commands, reusable_materials: Res<ReusableMaterials>) {
    commands.spawn_scene(bsn! {
        PlayingStateEntity
        Transform::from_xyz(0.0, 90.0, 0.0)
        Mesh3d(asset_value(cube_mesh(CubeMeshColors::All(Color::linear_rgb(
            1.0, 0.0, 0.0,
        )))))
        MeshMaterial3d::<PrimaryMaterial>({ reusable_materials.primary_plain.clone() })
        template_value(RigidBody::Dynamic)
        template_value(LockedAxes::new().lock_rotation_x().lock_rotation_z())
        Collider::cuboid(1.0, 1.0, 1.0)
        PlayerBody
    });

    // TODO: Make the camera a child of the player body.
}

fn rotate_and_move(
    time: Res<Time>,
    movement_settings: Res<PlayerMovementSettings>,
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut mouse_motion: MessageReader<MouseMotion>,
    camera_transf_q: Option<Single<&mut Transform, With<PrimaryCamera>>>,
    mut rot_o: ResMut<RotO>,
    freecam_enabled: Res<FreecamEnabled>,
    player_body_q: Option<
        Single<(&mut Transform, &RigidBody), (With<PlayerBody>, Without<PrimaryCamera>)>,
    >,
) {
    if let Some(mut camera_transf) = alrms!(camera_transf_q) {
        if let None = rot_o.0 {
            let real_rot = camera_transf.rotation.to_euler(EulerRot::YXZ);
            rot_o.0 = Some((real_rot.0, real_rot.1));
        }

        if let Some(rot) = alrms!(&mut rot_o.0) {
            let (mut player_body_transf, player_body_rb) = alrrs!(player_body_q).into_inner();

            // if freecam_enabled.0 {
            //     if
            // }

            // Rotation

            mouse_motion.read().for_each(|ev| {
                rot.0 -= (movement_settings.look_sensitivity * ev.delta.x).to_radians();
                rot.1 -= (movement_settings.look_sensitivity * ev.delta.y).to_radians();

                rot.1 = rot.1.clamp(-0.5 * PI, 0.5 * PI);

                let rotation =
                    Quat::from_axis_angle(Vec3::Y, rot.0) * Quat::from_axis_angle(Vec3::X, rot.1);

                if freecam_enabled.0 {
                    camera_transf.rotation = rotation;
                } else {
                    player_body_transf.rotation = rotation;
                    camera_transf.rotation = rotation;
                }
            });

            // Movement

            let forward = -Quat::from_euler(EulerRot::YXZ, rot.0, 0.0, 0.0).mul_vec3(Vec3::Z);
            let right = forward.rotate_y(-0.5 * PI);

            let mut movement_vector = Vec3::ZERO;
            if freecam_enabled.0 {
                if keys.pressed(key_bindings.move_forward) {
                    movement_vector += forward;
                }
                if keys.pressed(key_bindings.move_backward) {
                    movement_vector -= forward;
                }
                if keys.pressed(key_bindings.move_right) {
                    movement_vector += right;
                }
                if keys.pressed(key_bindings.move_left) {
                    movement_vector -= right;
                }
                if keys.pressed(key_bindings.move_up) {
                    movement_vector += Vec3::Y;
                }
                if keys.pressed(key_bindings.move_down) {
                    movement_vector -= Vec3::Y;
                }
            } else {
                if keys.pressed(key_bindings.move_forward) {
                    movement_vector += forward;
                }
                if keys.pressed(key_bindings.move_backward) {
                    movement_vector -= forward;
                }
                if keys.pressed(key_bindings.move_right) {
                    movement_vector += right;
                }
                if keys.pressed(key_bindings.move_left) {
                    movement_vector -= right;
                }
                if keys.pressed(key_bindings.move_up) {
                    movement_vector += Vec3::Y;
                }
                if keys.pressed(key_bindings.move_down) {
                    movement_vector -= Vec3::Y;
                }
            }

            movement_vector = movement_vector.normalize_or(Vec3::ZERO);

            let translation_offset =
                movement_vector * movement_settings.freecam_speed * time.delta_secs();

            if freecam_enabled.0 {
                camera_transf.translation += translation_offset;
            } else {
                player_body_transf.translation += translation_offset;
                camera_transf.translation = player_body_transf.translation;
            }
        }
    }
}
