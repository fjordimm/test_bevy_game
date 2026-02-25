use crate::states::{GameState, PauseState};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PauseState>()
            .add_systems(OnEnter(GameState::Playing), grab_cursor)
            .add_systems(OnExit(GameState::Playing), release_cursor)
            .add_systems(
                Update,
                (player_movement, toggle_pause).run_if(in_state(GameState::Playing)),
            );
    }
}
fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    mut pause: ResMut<PauseState>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        pause.paused = !pause.paused;
        cursor_options.visible = pause.paused;
        cursor_options.grab_mode = if pause.paused {
            CursorGrabMode::None
        } else {
            CursorGrabMode::Locked
        };
    }
}

fn grab_cursor(mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>) {
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Locked;
}

fn release_cursor(mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>) {
    cursor_options.visible = true;
    cursor_options.grab_mode = CursorGrabMode::None;
}

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    pause: Res<PauseState>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
) {
    if pause.paused {
        return;
    }

    let mut direction = Vec3::ZERO;
    let forward = camera.forward().as_vec3();
    let right = camera.right().as_vec3();

    let forward_flat = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let right_flat = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

    if keys.pressed(KeyCode::KeyW) {
        direction += forward_flat;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction -= forward_flat;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction -= right_flat;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction += right_flat;
    }

    let speed = 5.0;
    camera.translation += direction.normalize_or_zero() * speed * time.delta_secs();
}
