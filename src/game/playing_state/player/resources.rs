use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerMovementSettings {
    pub look_sensitivity: f32,
    pub freecam_speed: f32,
}

impl Default for PlayerMovementSettings {
    fn default() -> Self {
        Self {
            look_sensitivity: 0.2,
            freecam_speed: 100.0,
        }
    }
}

#[derive(Resource)]
pub struct FreecamEnabled(pub bool);
