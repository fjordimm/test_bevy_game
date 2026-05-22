use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};

use crate::game::{
    core::{
        resources::KeyBindings,
        states::{MouseMode, OverallState},
    },
    playing_state::{
        player::{resources::PlayerMovementSettings, tags::CameraForPlayer},
        sets::{DuringPlaying, DuringPlayingUnpaused},
        states::PauseState,
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
                    .in_set(DuringPlaying)
            )
            .add_systems(OnExit(OverallState::Playing),
                free_cursor
            )
            .add_systems(Update,
                (camera_look, movement)
                    .chain()
                    .in_set(DuringPlayingUnpaused::General)
            );
    }
}

fn grab_cursor(mut next_mouse_mode: ResMut<NextState<MouseMode>>) {
    next_mouse_mode.set(MouseMode::Grabbed);
}

fn free_cursor(mut next_mouse_mode: ResMut<NextState<MouseMode>>) {
    next_mouse_mode.set(MouseMode::Free);
}

fn camera_look(
    movement_settings: Res<PlayerMovementSettings>,
    mut mouse_motion: MessageReader<MouseMotion>,
    camera_transf_q: Option<Single<&mut Transform, With<CameraForPlayer>>>,
) {
    if let Some(mut camera_transf) = alrms!(camera_transf_q) {
        mouse_motion.read().for_each(|ev| {
            let (mut yaw, mut pitch, _) = camera_transf.rotation.to_euler(EulerRot::YXZ);

            pitch -= (movement_settings.look_sensitivity * ev.delta.y).to_radians();
            yaw -= (movement_settings.look_sensitivity * ev.delta.x).to_radians();

            pitch = pitch.clamp(-1.54, 1.54);

            camera_transf.rotation =
                Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        });
    }
}

fn movement(
    time: Res<Time>,
    movement_settings: Res<PlayerMovementSettings>,
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    camera_transf_q: Option<Single<&mut Transform, With<CameraForPlayer>>>,
) {
    if let Some(mut camera_transf) = alrms!(camera_transf_q) {
        let local_forward_dir = camera_transf.local_z();
        let forward = -Vec3::new(local_forward_dir.x, 0.0, local_forward_dir.z);
        let right = Vec3::new(local_forward_dir.z, 0.0, -local_forward_dir.x);

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

        // camera_trans.translation += Vec3::new(0.0, 0.0, -1.0 * time.delta_secs());
    }
}
