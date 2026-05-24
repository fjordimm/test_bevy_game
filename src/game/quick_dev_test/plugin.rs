use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::game::playing_state::{SunPosition, sets::DuringPlayingUnpaused};

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
    _time: Res<Time>,
    mut sun_position: ResMut<SunPosition>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
) {
    if keys.just_pressed(KeyCode::Digit1) {
        sun_position.0 = Vec3::Y.rotate_x(-0.0f32.to_radians())
    }
    if keys.just_pressed(KeyCode::Digit2) {
        sun_position.0 = Vec3::Y.rotate_x(-45.0f32.to_radians())
    }
    if keys.just_pressed(KeyCode::Digit3) {
        sun_position.0 = Vec3::Y.rotate_x(-60.0f32.to_radians())
    }
    if keys.just_pressed(KeyCode::Digit4) {
        sun_position.0 = Vec3::Y.rotate_x(-75.0f32.to_radians())
    }
    if keys.just_pressed(KeyCode::Digit5) {
        sun_position.0 = Vec3::Y.rotate_x(-85.0f32.to_radians())
    }
    if keys.just_pressed(KeyCode::Digit6) {
        sun_position.0 = Vec3::Y.rotate_x(-90.0f32.to_radians())
    }
    if keys.just_pressed(KeyCode::Digit7) {
        sun_position.0 = Vec3::Y.rotate_x(-105.0f32.to_radians())
    }
    if keys.just_pressed(KeyCode::Digit8) {
        sun_position.0 = Vec3::Y.rotate_x(-135.0f32.to_radians())
    }
    if keys.just_pressed(KeyCode::Digit9) {
        sun_position.0 = Vec3::Y.rotate_x(-180.0f32.to_radians())
    }

    for mouse_wheel in mouse_wheel_reader.read() {
        sun_position.0 = sun_position.0.rotate_x(-0.01 * mouse_wheel.y);
    }
}
