use crate::states::GameState;
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};
use std::f32::consts::FRAC_PI_2;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_camera)
            .add_systems(Update, mouse_look.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
struct PlayerCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        PlayerCamera,
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.7, 0.0), // eye height
    ));
}

fn mouse_look(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut camera: Single<&mut Transform, With<PlayerCamera>>,
) {
    let delta = accumulated_mouse_motion.delta;

    if delta == Vec2::ZERO {
        return;
    }

    let sensitivity = 0.001;
    let delta_yaw = -delta.x * sensitivity;
    let delta_pitch = -delta.y * sensitivity;

    let (yaw, pitch, _roll) = camera.rotation.to_euler(EulerRot::YXZ);
    let yaw = yaw + delta_yaw;

    const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
    let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
}
