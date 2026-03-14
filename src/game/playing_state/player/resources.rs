use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerMovementSettings {
    pub look_sensitivity: f32,
    pub speed: f32,
}

impl Default for PlayerMovementSettings {
    fn default() -> Self {
        Self {
            look_sensitivity: 0.00012,
            speed: 12.0,
        }
    }
}
