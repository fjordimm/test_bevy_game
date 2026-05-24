use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::game::playing_state::{
    sets::DuringPlayingUnpaused,
    skybox::{SkyRotationInv, SunPosition},
};

pub struct QuickDevTestPlugin;

impl Plugin for QuickDevTestPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update,
                rotate_sun
                    .in_set(DuringPlayingUnpaused::General)
            );
    }
}

fn rotate_sun(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    mut sun_position: ResMut<SunPosition>,
    mut sky_rotation_inv: ResMut<SkyRotationInv>,
) {
    let mut rotation = None;

    if keys.just_pressed(KeyCode::Digit1) {
        rotation = Some(-0.0f32.to_radians());
    }
    if keys.just_pressed(KeyCode::Digit2) {
        rotation = Some(-45.0f32.to_radians());
    }
    if keys.just_pressed(KeyCode::Digit3) {
        rotation = Some(-60.0f32.to_radians());
    }
    if keys.just_pressed(KeyCode::Digit4) {
        rotation = Some(-75.0f32.to_radians());
    }
    if keys.just_pressed(KeyCode::Digit5) {
        rotation = Some(-85.0f32.to_radians());
    }
    if keys.just_pressed(KeyCode::Digit6) {
        rotation = Some(-90.0f32.to_radians());
    }
    if keys.just_pressed(KeyCode::Digit7) {
        rotation = Some(-105.0f32.to_radians());
    }
    if keys.just_pressed(KeyCode::Digit8) {
        rotation = Some(-135.0f32.to_radians());
    }
    if keys.just_pressed(KeyCode::Digit9) {
        rotation = Some(-180.0f32.to_radians());
    }

    if let Some(rotation) = rotation {
        sun_position.0 = Vec3::Y.rotate_x(rotation);
        sky_rotation_inv.0 = Mat3::from_rotation_x(-rotation);
    }

    for mouse_wheel in mouse_wheel_reader.read() {
        sun_position.0 = sun_position.0.rotate_x(-0.01 * mouse_wheel.y);
        sky_rotation_inv.0 *= Mat3::from_rotation_x(0.01 * mouse_wheel.y);
    }

    sun_position.0 = sun_position.0.rotate_x(-0.005 * time.delta_secs());
    sky_rotation_inv.0 *= Mat3::from_rotation_x(0.005 * time.delta_secs());
}
