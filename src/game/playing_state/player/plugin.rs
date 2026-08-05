use std::f32::consts::PI;

use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::game::{
    core::{
        resources::KeyBindings,
        states::{MouseMode, OverallState},
    },
    playing_state::{
        player::{
            resources::PlayerMovementSettings,
            tags::{CameraForPlayer, PlayerBody},
        },
        sets::{DuringPlaying, DuringPlayingUnpaused, OnEnterPlaying, OnExitPlaying},
        states::PauseState,
        tags::PlayingStateEntity,
    },
    util::alrms,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .init_resource::<PlayerMovementSettings>()
            .add_systems(OnEnter(PauseState::Unpaused),
                grab_cursor
                    .in_set(DuringPlaying)
            )
            .add_systems(OnExit(PauseState::Unpaused),
                free_cursor
            )
            .add_systems(OnExit(OverallState::Playing),
                free_cursor
                    .in_set(OnExitPlaying::General)
            )
            .add_systems(OnEnter(OverallState::Playing),
                reset_rot_o
                    .in_set(OnEnterPlaying::ResourceSetup)
            )
            .add_systems(Update,
                rotate_and_move
                    .in_set(DuringPlayingUnpaused::General)
            )
            .add_systems(OnEnter(OverallState::Playing),
                spawn_player_body
                    .in_set(OnEnterPlaying::General)
            )
        ;
    }
}

fn grab_cursor(mut next_mouse_mode: ResMut<NextState<MouseMode>>) {
    next_mouse_mode.set(MouseMode::Grabbed);
}

fn free_cursor(mut next_mouse_mode: ResMut<NextState<MouseMode>>) {
    next_mouse_mode.set(MouseMode::Free);
}

#[derive(Resource)]
struct RotO(Option<(f32, f32)>); // (yaw, pitch)

fn reset_rot_o(mut commands: Commands) {
    commands.insert_resource(RotO(None));
}

fn rotate_and_move(
    time: Res<Time>,
    movement_settings: Res<PlayerMovementSettings>,
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut mouse_motion: MessageReader<MouseMotion>,
    camera_transf_q: Option<Single<&mut Transform, With<CameraForPlayer>>>,
    mut rot_o: ResMut<RotO>,
) {
    if let Some(mut camera_transf) = alrms!(camera_transf_q) {
        if let None = rot_o.0 {
            let real_rot = camera_transf.rotation.to_euler(EulerRot::YXZ);
            rot_o.0 = Some((real_rot.0, real_rot.1));
        }

        if let Some(rot) = alrms!(&mut rot_o.0) {
            // Rotation

            mouse_motion.read().for_each(|ev| {
                rot.0 -= (movement_settings.look_sensitivity * ev.delta.x).to_radians();
                rot.1 -= (movement_settings.look_sensitivity * ev.delta.y).to_radians();

                rot.1 = rot.1.clamp(-0.5 * PI, 0.5 * PI);

                camera_transf.rotation =
                    Quat::from_axis_angle(Vec3::Y, rot.0) * Quat::from_axis_angle(Vec3::X, rot.1);
            });

            // Movement

            let forward = -Quat::from_euler(EulerRot::YXZ, rot.0, 0., 0.).mul_vec3(Vec3::Z);
            let right = forward.rotate_y(-0.5 * PI);

            let mut velocity = Vec3::ZERO;
            if keys.pressed(key_bindings.move_forward) {
                velocity += forward;
            }
            if keys.pressed(key_bindings.move_backward) {
                velocity -= forward;
            }
            if keys.pressed(key_bindings.move_right) {
                velocity += right;
            }
            if keys.pressed(key_bindings.move_left) {
                velocity -= right;
            }
            if keys.pressed(key_bindings.move_up) {
                velocity += Vec3::Y;
            }
            if keys.pressed(key_bindings.move_down) {
                velocity -= Vec3::Y;
            }

            velocity = velocity.normalize_or(Vec3::ZERO);

            camera_transf.translation += velocity * movement_settings.speed * time.delta_secs();
        }
    }
}

fn spawn_player_body(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PlayingStateEntity,
        PlayerBody,
        Mesh3d(meshes.add(Capsule3d::new(0.25, 1000.))),
        MeshMaterial3d(materials.add(Color::linear_rgb(1., 0., 1.))),
        Transform::from_xyz(0., 0., 0.),
    ));
}
